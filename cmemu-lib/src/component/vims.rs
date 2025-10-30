use cmemu_proc_macros::{component_impl, handler, proxy_use};

use crate::common::new_ahb::databus::DataBus;
#[proxy_use]
use crate::common::new_ahb::ports::AHBPortConfig;
#[proxy_use]
use crate::common::new_ahb::signals::{MasterToSlaveWires, SlaveToMasterWires};
use crate::common::new_ahb::vlan::{AHBSoftVlanMasterPortInput, AHBSoftVlanSlavePortInput};
use crate::component::vims::cache::CacheComponent;
use crate::component::vims::internal_routing::{
    CodeCacheLineBuffer, CodeFlashLineBuffer, VIMSRegistersDriver,
};
#[proxy_use(proxy_only)]
pub(crate) use crate::component::vims::{PublicMasters, PublicSlaves};
#[proxy_use]
use crate::engine::Context;
use crate::engine::{
    DisableableComponent, MainComponent, PowerNode, SkippableClockTreeNode, TickComponent,
    TickComponentExtra,
};
use crate::expose_ports;
use crate::proxy::VIMSProxy;

use self::cache_ram::CacheRAMComponent;
use self::interconnect2::LiteWrapper as Interconnect;

mod cache;
mod cache_ram;
mod idcode_mux;
mod interconnect2;
mod internal_routing;
mod registers;

// ===========================================================================
// VIMSComponent
// ===========================================================================

#[derive(MainComponent, SkippableClockTreeNode, TickComponent)]
#[skippable_if_disableable]
pub(crate) struct VIMSComponent {
    #[subcomponent(Interconnect)]
    interconnect: Interconnect,

    #[subcomponent(CacheComponent)]
    cache: CacheComponent,

    #[subcomponent(CodeFlashLineBufferSubcomponent)]
    code_flash_line_buffer: CodeFlashLineBuffer,

    #[subcomponent(CodeCacheLineBufferSubcomponent)]
    code_cache_line_buffer: CodeCacheLineBuffer,

    #[subcomponent(VIMSRegistersDriverSubcomponent)]
    registers: VIMSRegistersDriver,

    /// Temporary separate RAM component to simplify routing inside cache
    #[subcomponent(CacheRAMComponent)]
    cache_ram: CacheRAMComponent,
}

#[component_impl(vims)]
impl DisableableComponent for VIMSComponent {
    fn can_be_disabled_now(&self) -> bool {
        // IC is the entrypoint to us – let's trust it
        self.interconnect.can_be_disabled_now() && self.registers.can_be_disabled_now()
    }
}

#[component_impl(vims)]
impl TickComponentExtra for VIMSComponent {
    fn tick_extra(&mut self) {}
}

#[component_impl(vims)]
impl VIMSComponent {
    pub(crate) fn new() -> Self {
        Self {
            interconnect: Interconnect::new(),
            cache: CacheComponent::new(),
            code_flash_line_buffer: CodeFlashLineBuffer::new(),
            code_cache_line_buffer: CodeCacheLineBuffer::new(),
            registers: VIMSRegistersDriver::new(),

            cache_ram: CacheRAMComponent::new(),
        }
    }

    pub(crate) fn tick(&mut self, ctx: &mut Context) {
        Interconnect::tick(self, ctx);
        CacheComponent::tick(self, ctx);
        CodeFlashLineBuffer::tick(self, ctx);
        CodeCacheLineBuffer::tick(self, ctx);
        VIMSRegistersDriver::run_driver(self, ctx);

        CacheRAMComponent::tick(self, ctx);
    }

    pub(crate) fn tock(&mut self, ctx: &mut Context) {
        Interconnect::tock(self, ctx);
        CacheComponent::tock(self, ctx);
        CodeFlashLineBuffer::tock(self, ctx);
        CodeCacheLineBuffer::tock(self, ctx);
        VIMSRegistersDriver::tock(self, ctx);

        CacheRAMComponent::tock(self, ctx);
    }

    #[handler]
    pub fn on_new_ahb_master_tagged_input(
        &mut self,
        ctx: &mut Context,
        tag: PublicMasters,
        msg: SlaveToMasterWires<<VIMSComponent as AHBPortConfig>::Data>,
    ) {
        if !<Self as PowerNode>::is_active(self, ctx) {
            debug_assert!(msg.is_inert());
            return;
        }
        <Self as AHBSoftVlanMasterPortInput<PublicMasters>>::on_ahb_soft_tagged_input(
            self, ctx, tag, msg,
        );
    }

    #[handler]
    pub fn on_new_ahb_slave_tagged_input(
        &mut self,
        ctx: &mut Context,
        tag: PublicSlaves,
        msg: MasterToSlaveWires<<VIMSComponent as AHBPortConfig>::Data>,
    ) {
        if !<Self as PowerNode>::is_active(self, ctx) {
            debug_assert!(msg.is_inert());
            return;
        }
        <Self as AHBSoftVlanSlavePortInput<PublicSlaves>>::on_ahb_soft_tagged_input(
            self, ctx, tag, msg,
        );
    }
}

#[component_impl(vims)]
impl AHBPortConfig for VIMSComponent {
    type Data = DataBus;
    type Component = VIMSComponent;
    const TAG: &'static str = "Vims";
}

expose_ports! {
    VIMSComponent data = DataBus,
    Master PublicMasters [proxy=VIMSProxy.on_new_ahb_master_tagged_input] {
        FlashMPort,
        GpramMPort,
        RomMPort,
    }
}

expose_ports! {
    VIMSComponent data = DataBus,
    Slave PublicSlaves [proxy=VIMSProxy.on_new_ahb_slave_tagged_input] {
        ICodeSPort,
        DCodeSPort,
        SysbusSPort,
    }
}
