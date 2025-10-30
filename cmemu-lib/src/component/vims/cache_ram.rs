use std::ops::Range;

use cmemu_proc_macros::proxy_use;

use crate::bridge_ports;
#[proxy_use]
use crate::common::Address;
#[proxy_use]
use crate::common::new_ahb::databus::DataBus;
#[proxy_use]
use crate::common::new_ahb::ports::AHBPortConfig;
#[proxy_use]
use crate::common::new_ahb::signals::Size;
use crate::common::new_memory::InvalidAddressError;
use crate::common::new_memory::{Memory, MemoryConfiguration};
#[proxy_use]
use crate::engine::Context;
use crate::engine::{
    DisableableComponent, SeqFlop, Subcomponent, TickComponent, TickComponentExtra,
};
use cc2650_constants as soc;

impl MemoryConfiguration for CacheRAMMemory {
    const IS_WRITABLE: bool = true;
    const ADDRESS_SPACE: Range<Address> = soc::GPRAM::ADDR_SPACE;
    const BUS_WIDTH: Size = Size::Doubleword;
    const WAIT_STATES: u8 = 0;
}
type CacheRAMMemory = Memory<MemorySC>;

#[derive(Subcomponent, TickComponent, DisableableComponent)]
#[subcomponent_1to1]
pub(crate) struct CacheRAMComponent {
    #[flop]
    pending_memory_update: SeqFlop<(u8, Address, [u8; 8])>,

    #[subcomponent(MemorySC)]
    memory: CacheRAMMemory,
}

impl TickComponentExtra for CacheRAMComponent {
    fn tick_extra(&mut self) {
        if self.pending_memory_update.is_set() {
            let (cycles_since_pended, address, data) = *self.pending_memory_update;
            if cycles_since_pended == 2 {
                self.write_memory(address, &data).map_err(|_| ()).unwrap();
            } else {
                self.pending_memory_update
                    .set_next((cycles_since_pended + 1, address, data));
            }
        }
    }
}

impl CacheRAMComponent {
    pub(crate) fn new() -> Self {
        Self {
            pending_memory_update: SeqFlop::new(),

            memory: CacheRAMMemory::new_zeroed(),
        }
    }

    pub(crate) fn tick(comp: &mut <Self as Subcomponent>::Component, ctx: &mut Context) {
        CacheRAMMemory::run_driver(comp, ctx);
    }

    pub(crate) fn tock(comp: &mut <Self as Subcomponent>::Component, ctx: &mut Context) {
        CacheRAMMemory::tock(comp, ctx);
    }

    pub fn pend_memory_update(
        comp: &mut <Self as Subcomponent>::Component,
        address: Address,
        data: [u8; 8],
    ) {
        Self::get_proxy(comp)
            .pending_memory_update
            .set_next((1, address, data));
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

    #[allow(dead_code)]
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

bridge_ports!(@slave CacheRAMComponent => @auto_configured @slave CacheRAMMemory);

impl AHBPortConfig for CacheRAMComponent {
    type Data = DataBus;
    type Component = <Self as Subcomponent>::Component;
    const TAG: &'static str = "CacheRAM";
}
