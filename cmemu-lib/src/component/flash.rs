use std::ops::Range;

use log::debug;

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
use crate::common::new_memory::InvalidAddressError;
use crate::common::new_memory::{Memory, MemoryConfiguration};
#[proxy_use]
use crate::component::memory_bypass::MemoryBypassReceiver;
#[proxy_use]
use crate::engine::Context;
use crate::engine::{
    DisableableComponent, MainComponent, SkippableClockTreeNode, TickComponent, TickComponentExtra,
};
use crate::proxy::FlashProxy;

impl MemoryConfiguration for FlashMemory {
    const IS_WRITABLE: bool = false;
    const ADDRESS_SPACE: Range<Address> = soc::FLASHMEM::ADDR_SPACE;
    const BUS_WIDTH: Size = if cfg!(feature = "soc-cc2652") {
        Size::FourWord
    } else if cfg!(feature = "soc-stm32f100rbt6") {
        Size::Word
    } else {
        Size::Doubleword
    };
    const WAIT_STATES: u8 = if cfg!(feature = "soc-stm32f100rbt6") {
        0
    } else {
        2
    };
}

type FlashMemory = Memory<MemorySC>;

#[derive(MainComponent, SkippableClockTreeNode, TickComponent, TickComponentExtra)]
#[skippable_if_disableable]
pub(crate) struct FlashComponent {
    #[subcomponent(MemorySC)]
    memory: FlashMemory,
}

#[component_impl(flash)]
impl FlashComponent {
    pub(crate) fn new(memory: &[u8]) -> Self {
        debug!("Creating memory flash, {} bytes", memory.len());
        Self {
            memory: Memory::new_with_contents(memory),
        }
    }

    pub(crate) fn tick(&mut self, ctx: &mut Context) {
        FlashMemory::run_driver(self, ctx);
    }

    pub(crate) fn tock(&mut self, ctx: &mut Context) {
        FlashMemory::tock(self, ctx);
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

    #[handler]
    pub fn on_new_ahb_slave_input(
        &mut self,
        ctx: &mut Context,
        msg: MasterToSlaveWires<<FlashComponent as AHBPortConfig>::Data>,
    ) {
        <Self as AHBSlavePortInput>::on_ahb_input(self, ctx, msg);
    }

    #[handler]
    #[allow(dead_code)] // debug assertions
    pub(crate) fn assert_addr_content(&mut self, _ctx: &mut Context, addr: Address, data: [u8; 8]) {
        let mut correct_data = [0_u8; 8];
        assert!(
            self.read_memory(addr, &mut correct_data).is_ok(),
            "Error reading memory from {addr:?}"
        );
        assert_eq!(
            &data, &correct_data,
            "Cache race condition: read incorrect value from {addr:?}"
        );
    }
}

bridge_ports!(@no_m2s @slave FlashComponent => @auto_configured @slave FlashMemory);

#[component_impl(flash)]
impl AHBSlavePortInput for FlashComponent {
    fn on_ahb_input(
        comp: &mut Self::Component,
        ctx: &mut Context,
        mut msg: MasterToSlaveWires<Self::Data>,
    ) {
        // We have alias address spaces, that seem to be resolved this far (possibly by ignoring high addr bits).
        if let Some(meta) = msg.addr_phase.meta.meta_mut() {
            // FIXME: specify aliases explicitly?
            meta.addr = meta
                .addr
                .masked(0x00ff_ffff)
                .offset(soc::FLASHMEM::ADDR.into());
        }
        <FlashMemory as AHBSlavePortInput>::on_ahb_input(comp, ctx, msg);
    }
}

#[component_impl(flash)]
impl AHBPortConfig for FlashComponent {
    type Data = DataBus;
    type Component = Self;
    const TAG: &'static str = "Flash";
}

#[component_impl(flash)]
impl AHBSlavePortProxiedInput for FlashComponent {
    fn proxy_ahb_input(ctx: &mut Context, msg: MasterToSlaveWires<Self::Data>) {
        FlashProxy.on_new_ahb_slave_input(ctx, msg);
    }
}

#[component_impl(flash)]
impl DisableableComponent for FlashComponent {
    fn can_be_disabled_now(&self) -> bool {
        true
    }
}
