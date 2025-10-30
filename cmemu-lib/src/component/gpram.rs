use std::ops::Range;

use cc2650_constants as soc;
use cmemu_proc_macros::{component_impl, handler, proxy_use};

use crate::bridge_ports;
#[proxy_use]
use crate::common::Address;
#[proxy_use]
use crate::common::new_ahb::databus::DataBus;
use crate::common::new_ahb::ports::AHBSlavePortProxiedInput;
#[proxy_use]
use crate::common::new_ahb::ports::{AHBPortConfig, AHBSlavePortInput};
#[proxy_use]
use crate::common::new_ahb::signals::{MasterToSlaveWires, Size};
use crate::common::new_memory::{InvalidAddressError, Memory, MemoryConfiguration};
#[proxy_use]
use crate::component::memory_bypass::MemoryBypassReceiver;
#[proxy_use]
use crate::engine::Context;
use crate::engine::{
    DisableableComponent, MainComponent, SkippableClockTreeNode, TickComponent, TickComponentExtra,
};
use crate::proxy::GPRAMProxy;

impl MemoryConfiguration for GPRAMMemory {
    const IS_WRITABLE: bool = true;
    const ADDRESS_SPACE: Range<Address> = soc::GPRAM::ADDR_SPACE;
    const BUS_WIDTH: Size = Size::Doubleword;
    const WAIT_STATES: u8 = 0;
}
type GPRAMMemory = Memory<MemorySC>;

#[derive(MainComponent, SkippableClockTreeNode, TickComponent, TickComponentExtra)]
#[skippable_if_disableable]
pub(crate) struct GPRAMComponent {
    #[subcomponent(MemorySC)]
    memory: GPRAMMemory,
}

#[component_impl(gpram)]
impl GPRAMComponent {
    pub(crate) fn new() -> Self {
        Self {
            memory: GPRAMMemory::new_zeroed(),
        }
    }

    pub(crate) fn tick(&mut self, ctx: &mut Context) {
        GPRAMMemory::run_driver(self, ctx);
    }

    pub(crate) fn tock(&mut self, ctx: &mut Context) {
        GPRAMMemory::tock(self, ctx);
    }

    #[handler]
    pub fn on_new_ahb_slave_input(
        &mut self,
        ctx: &mut Context,
        msg: MasterToSlaveWires<<GPRAMComponent as AHBPortConfig>::Data>,
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

bridge_ports!(@slave GPRAMComponent => @auto_configured @slave GPRAMMemory);

#[component_impl(gpram)]
impl AHBPortConfig for GPRAMComponent {
    type Data = DataBus;
    type Component = Self;
    const TAG: &'static str = "GPRAM";
}

#[component_impl(gpram)]
impl AHBSlavePortProxiedInput for GPRAMComponent {
    fn proxy_ahb_input(ctx: &mut Context, msg: MasterToSlaveWires<Self::Data>) {
        GPRAMProxy.on_new_ahb_slave_input(ctx, msg);
    }
}

#[component_impl(gpram)]
impl DisableableComponent for GPRAMComponent {
    fn can_be_disabled_now(&self) -> bool {
        true
    }
}
