use std::ops::Range;

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
use crate::proxy::SRAMProxy;

impl MemoryConfiguration for SRAMMemory {
    const IS_WRITABLE: bool = true;
    const ADDRESS_SPACE: Range<Address> = soc::SRAM::ADDR_SPACE;
    const BUS_WIDTH: Size = Size::Word;
    const WAIT_STATES: u8 = 0;
}

type SRAMMemory = Memory<MemorySC>;

#[derive(
    MainComponent, SkippableClockTreeNode, TickComponent, TickComponentExtra, DisableableComponent,
)]
#[skippable_if_disableable]
pub(crate) struct SRAMComponent {
    #[subcomponent(MemorySC)]
    memory: SRAMMemory,
}

#[component_impl(sram)]
impl SRAMComponent {
    pub(crate) fn new() -> Self {
        Self {
            memory: SRAMMemory::new_zeroed(),
        }
    }

    pub(crate) fn tick(&mut self, ctx: &mut Context) {
        SRAMMemory::run_driver(self, ctx);
    }

    pub(crate) fn tock(&mut self, ctx: &mut Context) {
        SRAMMemory::tock(self, ctx);
    }

    #[handler]
    pub fn on_new_ahb_slave_input(
        &mut self,
        ctx: &mut Context,
        msg: MasterToSlaveWires<<SRAMComponent as AHBPortConfig>::Data>,
    ) {
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

bridge_ports!(@slave SRAMComponent => @auto_configured @slave SRAMMemory);

#[component_impl(sram)]
impl AHBPortConfig for SRAMComponent {
    type Data = DataBus;
    type Component = Self;
    const TAG: &'static str = "SRAM";
}

#[component_impl(sram)]
impl AHBSlavePortProxiedInput for SRAMComponent {
    fn proxy_ahb_input(ctx: &mut Context, msg: MasterToSlaveWires<Self::Data>) {
        SRAMProxy.on_new_ahb_slave_input(ctx, msg);
    }
}
