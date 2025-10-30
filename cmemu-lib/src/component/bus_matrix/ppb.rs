//! For general overview of busses in ARM Cortex-M3 see [ARM-TDG] 6.2

use log::warn;

use cc2650_constants as soc;
use cc2650_constants::CoreMap::PPBRegion;
use cmemu_proc_macros::proxy_use;

use crate::common::Address;
use crate::common::new_ahb::databus::DataBus;
use crate::common::new_ahb::decoder::{AhbDecode, AhbPort, Decoder, DefaultSlave};
use crate::common::new_ahb::ports::AHBMasterPortOutput;
#[proxy_use]
use crate::common::new_ahb::ports::{AHBMasterPortInput, AHBPortConfig};
#[proxy_use]
use crate::common::new_ahb::signals::{
    MasterToSlaveAddrPhase, MasterToSlaveWires, SlaveToMasterWires,
};
use crate::common::new_ahb::vlan::AhbMasterOutputDispatcher;
#[proxy_use]
use crate::common::new_ahb::vlan::{
    AHBMasterPortTaggedInput, AHBSoftVlanMasterPortInput, AhbDecoderTag,
};
use crate::common::utils::FromMarker;
use crate::component::bus_matrix::{BusMatrixComponent, DwtM, NvicM, PPBSubcomponent, PpbMasters};
#[proxy_use]
use crate::engine::Context;
use crate::utils::IfExpr;

// XXX: Do we need to reflect HREADYOUT?
pub(super) type PrivatePeripheralBus = Decoder<PPBSubcomponent>;

impl AHBPortConfig for PrivatePeripheralBus {
    type Data = DataBus;
    type Component = BusMatrixComponent;
    const TAG: &'static str = "PPB";
}

#[allow(clippy::absolute_paths)]
const MOCKED_ADDRESSES: &[Address] = if cfg!(feature = "frankentrace") {
    &[
        soc::CPU_ITM::LAR::ADDR,
        soc::CPU_SCS::ACTLR::ADDR,
        soc::CPU_SCS::DEMCR::ADDR,
        // PC tracing - ignore for now
        soc::CPU_ITM::STIM0::ADDR,
        soc::CPU_ITM::STIM1::ADDR,
        soc::CPU_ITM::STIM2::ADDR,
        soc::CPU_ITM::STIM3::ADDR,
        soc::CPU_ITM::STIM4::ADDR,
        soc::CPU_ITM::STIM5::ADDR,
        soc::CPU_ITM::STIM6::ADDR,
        soc::CPU_ITM::STIM7::ADDR,
        soc::CPU_ITM::STIM8::ADDR,
        soc::CPU_ITM::STIM9::ADDR,
        soc::CPU_ITM::STIM10::ADDR,
        soc::CPU_ITM::STIM11::ADDR,
        soc::CPU_ITM::TER::ADDR,
        soc::CPU_ITM::TCR::ADDR,
        soc::CPU_TPIU::FFCR::ADDR,
        soc::CPU_TPIU::SPPR::ADDR,
        soc::CPU_TPIU::ACPR::ADDR,
    ]
} else {
    &[
        soc::CPU_ITM::LAR::ADDR,
        soc::CPU_SCS::ACTLR::ADDR,
        soc::CPU_SCS::DEMCR::ADDR,
    ]
};

impl AhbDecode for Option<PPBRegion> {
    fn decode(addr: Address) -> Self {
        Some(PPBRegion::from(addr))
    }
}

impl FromMarker<DefaultSlave> for Option<PPBRegion> {
    fn from_marker() -> Self {
        None
    }
}

impl AhbDecoderTag for PrivatePeripheralBus {
    type Enum = Option<PPBRegion>;

    fn decode(addr: Address) -> Self::Enum {
        if MOCKED_ADDRESSES.contains(&addr.aligned_down_to_4_bytes()) {
            return None;
        }

        Self::Enum::decode(addr)
    }

    fn stateless_mock(msg: &MasterToSlaveAddrPhase) -> Option<Self::Data> {
        // Mocked writes used in CM bootstrap && Test preparation sequences.
        // To be replaced with correct accesses to registers.
        // Note: only requests for which `decode` function returned `None` are handled here.

        // Note: ACTRL Contains `DISDEFWBUF` bit field, which is used to disable write buffer.

        warn!("Mock access to ppb on message: {:?}", msg);

        let meta = msg.meta.meta().unwrap();
        assert!(
            MOCKED_ADDRESSES.contains(&meta.addr),
            "Unsupported address in mock {msg:?}"
        );
        Some(meta.is_writing().ife(
            DataBus::HighZ,
            DataBus::Word(0).extract_from_aligned(meta.addr, meta.size),
        ))
    }
}

impl AhbMasterOutputDispatcher<Option<PPBRegion>> for PrivatePeripheralBus {
    fn dispatch_ahb_output(
        comp: &mut Self::Component,
        ctx: &mut Context,
        tag: Self::Enum,
        msg: MasterToSlaveWires<Self::Data>,
    ) {
        // TODO: think of a better place for this
        // [ARM-ARM] A3.2.1 Alignment behavior
        // [ARM-ARM] B3.1.1 General rules for PPB register accesses
        // Unaligned requests to PPB (which is "strongly ordered" memory) are UNPREDICTABLE.
        if let Some(meta) = msg.addr_phase.meta.meta() {
            assert!(
                meta.size.is_addr_aligned(meta.addr),
                "UNPREDICTABLE: unaligned access to PPB at request: {:?}",
                msg.addr_phase
            );
        }

        match tag {
            Some(PPBRegion::SCS) => <NvicM as AHBMasterPortOutput>::send_ahb_output(comp, ctx, msg),
            Some(PPBRegion::DWT) => <DwtM as AHBMasterPortOutput>::send_ahb_output(comp, ctx, msg),
            _ => <AhbPort<PPBSubcomponent, DefaultSlave> as AHBMasterPortOutput>::send_ahb_output(
                comp, ctx, msg,
            ),
        }
    }
}

impl From<PpbMasters> for PPBRegion {
    fn from(p: PpbMasters) -> Self {
        match p {
            PpbMasters::NvicM => PPBRegion::SCS,
            PpbMasters::DwtM => PPBRegion::DWT,
        }
    }
}

impl AHBSoftVlanMasterPortInput<PpbMasters> for PrivatePeripheralBus {
    fn on_ahb_soft_tagged_input(
        comp: &mut Self::Component,
        ctx: &mut Context,
        tag: PpbMasters,
        msg: SlaveToMasterWires<Self::Data>,
    ) {
        <PrivatePeripheralBus as AHBMasterPortTaggedInput>::on_ahb_tagged_input(
            comp,
            ctx,
            Some(tag.into()),
            msg,
        );
    }
}

// Todo: those are unneeeded
impl AHBMasterPortInput for NvicM {
    fn on_ahb_input(
        comp: &mut Self::Component,
        ctx: &mut Context,
        msg: SlaveToMasterWires<Self::Data>,
    ) {
        <PrivatePeripheralBus as AHBSoftVlanMasterPortInput<PpbMasters>>::on_ahb_soft_tagged_input(
            comp,
            ctx,
            NvicM.into(),
            msg,
        );
    }
}

impl AHBMasterPortInput for DwtM {
    fn on_ahb_input(
        comp: &mut Self::Component,
        ctx: &mut Context,
        msg: SlaveToMasterWires<Self::Data>,
    ) {
        <PrivatePeripheralBus as AHBSoftVlanMasterPortInput<PpbMasters>>::on_ahb_soft_tagged_input(
            comp,
            ctx,
            NvicM.into(),
            msg,
        );
    }
}
