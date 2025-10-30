use std::ops::Range;

use log::trace;

use cc2650_constants as soc;
use cmemu_proc_macros::{component_impl, handler, proxy_use};

use crate::bridge_ports;
#[proxy_use]
use crate::common::Address;
#[proxy_use]
use crate::common::new_ahb::databus::DataBus;
#[proxy_use]
use crate::common::new_ahb::ports::{AHBPortConfig, AHBSlavePortInput, AHBSlavePortProxiedInput};
#[proxy_use]
use crate::common::new_ahb::signals::{MasterToSlaveWires, Size};
#[proxy_use]
use crate::common::new_memory::{InvalidAddressError, Memory, MemoryConfiguration};
#[proxy_use]
use crate::component::memory_bypass::MemoryBypassReceiver;
#[proxy_use]
use crate::engine::Context;
use crate::engine::{
    DisableableComponent, MainComponent, SkippableClockTreeNode, TickComponent, TickComponentExtra,
};
use crate::proxy::ROMProxy;

impl MemoryConfiguration for ROMMemory {
    const IS_WRITABLE: bool = false;
    const ADDRESS_SPACE: Range<Address> = soc::BROM::ADDR_SPACE;
    const BUS_WIDTH: Size = Size::Word;
    const WAIT_STATES: u8 = 0;
}
type ROMMemory = Memory<MemorySC>;

#[derive(MainComponent, SkippableClockTreeNode, TickComponent, TickComponentExtra)]
#[skippable_if_disableable]
pub(crate) struct ROMComponent {
    #[subcomponent(MemorySC)]
    memory: ROMMemory,

    memory_provided: bool,
}

#[component_impl(rom)]
impl ROMComponent {
    pub(crate) fn new(memory_opt: Option<&[u8]>) -> Self {
        let memory = if let Some(mem) = memory_opt {
            assert!(
                // TODO: magic 0x44 from driverlib.elf, and 20480 from flattened driverlib.bin
                mem.len() == ROMMemory::memory_size() || mem.len() == 20480 || mem.len() == 0x44,
                "Invalid ROM dump size, doesn't fit exactly in address space",
            );

            trace!(
                "Creating memory ROM with contents: {} bytes",
                ROMMemory::memory_size()
            );

            ROMMemory::new_with_contents(mem)
        } else {
            trace!(
                "Creating non-readable empty ROM memory: {} bytes",
                ROMMemory::memory_size()
            );
            ROMMemory::new_zeroed()
        };

        Self {
            memory,
            memory_provided: memory_opt.is_some(),
        }
    }

    pub(crate) fn tick(&mut self, ctx: &mut Context) {
        ROMMemory::run_driver(self, ctx);
    }

    pub(crate) fn tock(&mut self, ctx: &mut Context) {
        ROMMemory::tock(self, ctx);
    }

    #[handler]
    pub fn on_new_ahb_slave_input(
        &mut self,
        ctx: &mut Context,
        msg: MasterToSlaveWires<<ROMComponent as AHBPortConfig>::Data>,
    ) {
        assert!(
            self.memory_provided,
            "ROM wasn't provided on startup, reads aren't supported"
        );
        <Self as AHBSlavePortInput>::on_ahb_input(self, ctx, msg);
    }

    #[handler]
    pub(crate) fn request_memory_write_bypass(
        &mut self,
        ctx: &mut Context,
        start_address: Address,
        data: DataBus,
    ) {
        self.memory
            .request_memory_write_bypass(ctx, start_address, data);
    }

    pub(crate) fn write_memory(
        &mut self,
        start_address: Address,
        memory: &[u8],
    ) -> Result<(), InvalidAddressError> {
        // XXX: change this iface later?
        self.memory
            .write_memory(start_address, memory)
            .or(Err(InvalidAddressError))
    }

    #[handler]
    pub(crate) fn request_memory_read_bypass(
        &self,
        ctx: &mut Context,
        start_address: Address,
        buffer: Vec<u8>,
        size: usize,
        receiver: MemoryBypassReceiver,
    ) {
        self.memory
            .request_memory_read_bypass(ctx, start_address, buffer, size, receiver);
    }

    pub(crate) fn read_memory(
        &self,
        start_address: Address,
        memory: &mut [u8],
    ) -> Result<(), InvalidAddressError> {
        self.memory
            .read_memory(start_address, memory)
            .or(Err(InvalidAddressError))
    }
}

bridge_ports!(@slave ROMComponent => @auto_configured @slave ROMMemory);

#[component_impl(rom)]
impl AHBPortConfig for ROMComponent {
    type Data = DataBus;
    type Component = Self;
    const TAG: &'static str = "ROM";
}

#[component_impl(rom)]
impl AHBSlavePortProxiedInput for ROMComponent {
    fn proxy_ahb_input(ctx: &mut Context, msg: MasterToSlaveWires<Self::Data>) {
        ROMProxy.on_new_ahb_slave_input(ctx, msg);
    }
}

#[component_impl(rom)]
impl DisableableComponent for ROMComponent {
    fn can_be_disabled_now(&self) -> bool {
        true
    }
}
