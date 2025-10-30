//! The Bus Matrix component (and some adjacent boxes) on the Cortex-M package
//!
//! Since it lives in the package, we found that a combinatorial interface with the Core is required.
//!
//! To the external world, the component exposes these public AHB port types:
//!
//! - From the Core side, we have buses (nameable through the [`PublicSlaves`] enum or units):
//!   - [`IBusS`] - the instruction bus from the core (has GRANT wire)
//!   - [`DBusS`] - the data bus from the core (has GRANT wire)
//!   - [`DebugS`] - the debug (DAP) port (has GRANT wire)
//!
//! From the output side, we have two namespaces: internal and external, all using AHB-Lite:
//! - Ports external to the Cortex-M are parts of the [`PublicMasters`] enum:
//!   - [`ICodeM`]
//!   - [`DCodeM`]
//!   - [`SysbusM`]
//! - Ports internal to the package go through the PPB (Private Peripheral Bus),
//!   whose decoding is inlined in this component, thus we export the [`PpbMasters`] enum with:
//!   - [`NvicM`],
//!   - [`DwtM`],
//!   - add new PPB components here once implemented!
//!
//! As the `bus_matrix` module contains some adjacent blocks, these are roughly split as so:
//! - [`interconnect`] implements the main Bus Matrix logic of routing and arbitration,
//! - [`internal_routing`] is in a separate module, to route the publicly exposed ports internally,
//! - [`aligner`], `bitband`, [`mpu`], [`registration_buffer`] implement the "adjacent blocks",
//! - [`ppb`] hosts the PPB bus implementation (dispatch to mentioned [`PpbMasters`]).
// Bibliography:
//  [AHB] AMBA 3 AHB-Lite Protocol
//    https://static.docs.arm.com/ihi0033/a/IHI0033A_new_eula.pdf
//  Also see CMSDK.

use cmemu_proc_macros::{component_impl, handler, proxy_use};

use crate::common::new_ahb::databus::DataBus;
#[proxy_use]
use crate::common::new_ahb::ports::AHBPortConfig;
#[proxy_use]
use crate::common::new_ahb::signals::{MasterToSlaveWires, SlaveToMasterWires};
use crate::common::new_ahb::vlan::{AHBSoftVlanMasterPortInput, AHBSoftVlanSlavePortInput};
use crate::common::new_ahb::write_buffer::WriteBuffer;
use crate::component::bus_matrix::aligner::Aligner;
use crate::component::bus_matrix::registration_buffer::IBusRegistrationBuffer;
#[proxy_use(proxy_only)]
use crate::component::bus_matrix::{PpbMasters, PublicMasters, PublicSlaves};
#[proxy_use]
use crate::engine::{Context, PowerNode};
use crate::engine::{
    DisableableComponent, MainComponent, SkippableClockTreeNode, TickComponent, TickComponentExtra,
};
use crate::expose_ports;
use crate::proxy::BusMatrixProxy;

use self::interconnect::Interconnect;

// Curently mocked by sysbus
// mod bitband;
// TODO: route through Register and MPU
mod aligner;
mod interconnect;
mod internal_routing;
mod mpu;
mod ppb;
mod registration_buffer;

// ===========================================================================
// BusMatrixComponent
// ===========================================================================

#[derive(MainComponent, SkippableClockTreeNode, TickComponent, TickComponentExtra)]
#[skippable_if_disableable]
pub(crate) struct BusMatrixComponent {
    #[subcomponent(Interconnect)]
    interconnect: Interconnect,
    #[subcomponent(IBusRegistrationBuffer)]
    registration_buffer: IBusRegistrationBuffer,
    #[subcomponent(PPBSubcomponent)]
    ppb: PPB,

    #[subcomponent(SystemWBSC)]
    sys_wb: SystemWB,
    #[subcomponent(DCodeAlignerSC)]
    dcode_aligner: DBusAligner,
    #[subcomponent(DCodeWBSC)]
    dcode_wb: DCodeWB,
    // #[subcomponent(Bitband)]
    // bitband: Bitband,
}

type PPB = ppb::PrivatePeripheralBus;
type SystemWB = WriteBuffer<SystemWBSC>;
type DCodeWB = WriteBuffer<DCodeWBSC>;
type DBusAligner = Aligner<DCodeAlignerSC>;
// type Bitband = bitband::Bitband;

#[component_impl(bus_matrix)]
impl BusMatrixComponent {
    pub(crate) fn new() -> Self {
        Self {
            interconnect: Interconnect::new(),
            registration_buffer: IBusRegistrationBuffer::new(),
            ppb: PPB::new(),
            sys_wb: WriteBuffer::new(),
            dcode_wb: WriteBuffer::new(),
            dcode_aligner: Aligner::new(),
        }
    }

    pub(crate) fn tick(&mut self, ctx: &mut Context) {
        IBusRegistrationBuffer::tick(self, ctx);
        // Bitband::run_driver(self, ctx);
        PPB::sub_tick(self, ctx);
        Interconnect::tick(self, ctx);
        SystemWB::tick(self, ctx);
        DBusAligner::tick(self, ctx);
        DCodeWB::tick(self, ctx);
    }

    pub(crate) fn tock(&mut self, ctx: &mut Context) {
        IBusRegistrationBuffer::tock(self, ctx);
        // Bitband::tock(self, ctx);
        Interconnect::tock(self, ctx);
    }

    // TODO: This needs to be rethought since we have combinatorial dependence on Core,
    //   which is another component: maybe core should send ahb in tick? or BM be subcomp of Core?
    #[handler]
    pub fn on_core_tock_done(&mut self, ctx: &mut Context) {
        interconnect::end_combinatorial_changes(self, ctx);

        SystemWB::tock(self, ctx);
        DBusAligner::tock(self, ctx);
        DCodeWB::tock(self, ctx);
        PPB::tock(self, ctx);
    }

    #[handler]
    pub fn on_new_ahb_master_tagged_input(
        &mut self,
        ctx: &mut Context,
        tag: PublicMasters,
        msg: SlaveToMasterWires<<BusMatrixComponent as AHBPortConfig>::Data>,
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
        msg: MasterToSlaveWires<<BusMatrixComponent as AHBPortConfig>::Data>,
    ) {
        if !<Self as PowerNode>::is_active(self, ctx) {
            debug_assert!(msg.is_inert());
            return;
        }
        <Self as AHBSoftVlanSlavePortInput<PublicSlaves>>::on_ahb_soft_tagged_input(
            self, ctx, tag, msg,
        );
    }

    #[handler]
    pub fn on_new_ahb_ppb_master_tagged_input(
        &mut self,
        ctx: &mut Context,
        tag: PpbMasters,
        msg: SlaveToMasterWires<<BusMatrixComponent as AHBPortConfig>::Data>,
    ) {
        if !<Self as PowerNode>::is_active(self, ctx) {
            debug_assert!(msg.is_inert());
            return;
        }
        <PPB as AHBSoftVlanMasterPortInput<PpbMasters>>::on_ahb_soft_tagged_input(
            self, ctx, tag, msg,
        );
    }
}

#[component_impl(bus_matrix)]
impl AHBPortConfig for BusMatrixComponent {
    type Data = DataBus;
    type Component = Self;
    const TAG: &'static str = "BusMatrix";
}

expose_ports! {
    BusMatrixComponent data = DataBus,
    Master PublicMasters [proxy=BusMatrixProxy.on_new_ahb_master_tagged_input] {
        ICodeM,
        DCodeM,
        SysbusM,
    }
    Master PpbMasters [proxy=BusMatrixProxy.on_new_ahb_ppb_master_tagged_input] {
        NvicM,
        DwtM,
    }
}

expose_ports! {
    BusMatrixComponent data = DataBus,
    Slave PublicSlaves [proxy=BusMatrixProxy.on_new_ahb_slave_tagged_input] {
        IBusS,
        DBusS,
        DebugS,
    }
}

#[component_impl(bus_matrix)]
impl DisableableComponent for BusMatrixComponent {
    fn can_be_disabled_now(&self) -> bool {
        true
    }
}
