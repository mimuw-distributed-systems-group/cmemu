//! The main Bus Matrix Interconnect
//!
//! We initially assumed the Bux Matrix could act as an AHB-Lite interconnect.
//! However, this was refuted by our measurements, which have shown that a memory transfer
//! may be stalled in the address phases, even when no data phase is present.
//! Therefore, we simulate a subset of the full AHB protocol, by adding a combinatorial GRANT wire.
//!
//! For a general overview of busses in ARM Cortex-M3 see [ARM-TDG] 6.2

use std::fmt::Debug;

use cc2650_constants::CoreMap::{Bus, CoreMemoryMap};
use cc2650_constants::MPU;

use crate::common::Address;
use crate::common::new_ahb::arbiter::{
    CombinatorialFixedArbiter, FixedArbiter, NullArbiter, ReversedCombFixedArbiter,
};
use crate::common::new_ahb::decoder::{self, AhbDecode};
use crate::common::new_ahb::input_stage::transparent::{
    TransparentInputStage, TransparentInputStageCfg,
};
use crate::common::new_ahb::output_stage::combinatorial_os;
use crate::common::new_ahb::ports::{
    AHBSlavePortInput, AHBSlavePortOutput, AhbMasterPortInputWithGranting,
    AhbSlavePortOutputWithGranting,
};
use crate::common::new_ahb::signals::{MasterToSlaveWires, TrackedBool};
use crate::common::new_ahb::vlan::{
    AHBSoftVlanSlavePortInput, AhbDecoderTag, AhbMultiMasterConfig, AhbSlaveOutputDispatcher,
};
use crate::component::bus_matrix::registration_buffer::IBusRegistrationBuffer;
use crate::component::bus_matrix::{BusMatrixComponent, DBusAligner};
use crate::engine::{
    Context, DisableableComponent, Subcomponent, TickComponent, TickComponentExtra,
};
use crate::{bridge_ports, make_concrete_dispatcher, make_port_struct};
use crate::{build_interconnect, decoder_tags_and_markers};

decoder_tags_and_markers!(@with_markers
pub(crate) enum SlavePorts {
    Data,
    Instruction,
    Debugger,
});

// See [ARM-TRM-G] 12.1 About bus interfaces
// There is a clear distinction between Internal and External Private Peripheral Bus
// Vendor range is not very clear, since it is defined as `0xE010_0000..` on system bus, as well
// as on External PPB.
decoder_tags_and_markers!(@with_dispatcher
pub(crate) enum MasterPorts {
    ICode,
    DCode,
    System,
    PPB,
});

impl From<Bus> for MasterPorts {
    fn from(bus: Bus) -> Self {
        match bus {
            Bus::DCode => MasterPorts::DCode,
            Bus::ICode => MasterPorts::ICode,
            Bus::System | Bus::BitbandAlias => MasterPorts::System,
            Bus::ExternalPPB | Bus::InternalPPB => MasterPorts::PPB,
            _ => unimplemented!(),
        }
    }
}

// Assuming that we decode for DATA
impl AhbDecode for Option<MasterPorts> {
    fn decode(addr: Address) -> Self {
        Some(Bus::from_map(addr.into(), true).into())
    }
}

build_interconnect!(
    pub(super) BusMatrixInterconnect
    masters SlavePorts => [Data, Debugger, Instruction]
    slaves MasterPorts => [ICode, DCode, System, PPB]
    using TransparentInputStage as input, decoder::Decoder=>decoder::AhbPort as decoder,
          and combinatorial_os::AhbPort=>combinatorial_os::OutputStage as output
);

impl TransparentInputStageCfg for TransparentInputStage<DataInputSC> {
    // TODO: scrap manual granter routing and use bridge-ports!
    const GRANTER: fn(&mut Self::Component, &mut Context, TrackedBool) =
        <DBusAligner as AhbMasterPortInputWithGranting>::on_grant_wire;
}

impl TransparentInputStageCfg for TransparentInputStage<InstructionInputSC> {
    // It seems that it is the register buffer that terminates the transfer after all
    const GRANTER: fn(&mut Self::Component, &mut Context, TrackedBool) =
        <IBusRegistrationBuffer as AhbMasterPortInputWithGranting>::on_grant_wire;
}

impl TransparentInputStageCfg for TransparentInputStage<DebuggerInputSC> {
    const GRANTER: fn(&mut Self::Component, &mut Context, TrackedBool) = |_, _, _| unimplemented!();
}

impl AhbDecoderTag for decoder::Decoder<InstructionDecoderSC> {
    type Enum = Option<MasterPorts>;

    fn decode(addr: Address) -> Self::Enum {
        // Vendor is permanently XN
        // [ARM-ARM] B3.5.2 Behavior when the MPU is disabled
        let region = CoreMemoryMap::from(addr);
        if MPU::XN::from(region) == MPU::XN::InstructionFetchDisabled {
            todo!("We should generate MemManage fault")
        }
        let bus = Bus::from_map(region, false);

        // XXX: For now we route to DCode to do the arbitration -- but it may require separation
        if bus == Bus::ICode {
            return Some(Bus::DCode.into());
        }
        Some(bus.into())
    }
    const REFLECTS_HREADY: bool = false;
    const GRANTER: Option<fn(&mut Self::Component, &mut Context, TrackedBool)> =
        Some(<Self as AhbSlavePortOutputWithGranting>::send_grant_wire);
}
impl AhbDecoderTag for decoder::Decoder<DataDecoderSC> {
    type Enum = Option<MasterPorts>;
    const REFLECTS_HREADY: bool = false;
    const GRANTER: Option<fn(&mut Self::Component, &mut Context, TrackedBool)> =
        Some(<Self as AhbSlavePortOutputWithGranting>::send_grant_wire);
}
impl AhbDecoderTag for decoder::Decoder<DebuggerDecoderSC> {
    type Enum = Option<MasterPorts>;
    const REFLECTS_HREADY: bool = false;
    const GRANTER: Option<fn(&mut Self::Component, &mut Context, TrackedBool)> =
        Some(<Self as AhbSlavePortOutputWithGranting>::send_grant_wire);
}

impl AhbMultiMasterConfig for combinatorial_os::OutputStage<ICodeOutputSC> {
    type MastersEnum = SlavePorts;
    // Nothing is connected
    // type Arbiter = NoArbiter<SlavePorts, Instruction>;
    type Arbiter = NullArbiter<SlavePorts>;
}

// It seems that both on IDCode and System output, ldr cannot pipeline after str,
// But it is fine, if the ldr goes to a different memory.
// For proof, see: `ldr_str_timing.asm` - you may just focus on LSU transfers non-coliding with Fetch
//      E.g., with code:sram,load/store:gpram, you will see that LSUCNT increases linearly
//      with number of repetitions, even for narrow instructions.
// To cleanly see a LSU-fetch collision, see `pipelining_str_cares_about_dcode` with sram/sram/narrow
// configuration.
// LDR to different memory than STR: in `ldr_str_timing.asm`

impl AhbMultiMasterConfig for combinatorial_os::OutputStage<DCodeOutputSC> {
    type MastersEnum = SlavePorts;
    type Arbiter = CombinatorialFixedArbiter<SlavePorts>;
    // type Arbiter = AMBANonCompliantDNOTITRANS;
    // type Arbiter = FixedArbiter<SlavePorts>;
}
impl AhbMultiMasterConfig for combinatorial_os::OutputStage<SystemOutputSC> {
    type MastersEnum = SlavePorts;
    // type Arbiter = RoundRobinArbiter<SlavePorts>;
    // type Arbiter = CombinatorialFixedArbiter<SlavePorts>;
    // Minimal-proof test: .align 3; isb.w; ldr.w cyccnt; ldr.n sram; ldr.n sram; ldr.w cyccnt
    // The second ldr.n will be stalled, what is confirmed by tracing side-effects in interrupted state
    // This Arbiter probably detects bursts (LDM/STM)
    type Arbiter = ReversedCombFixedArbiter<SlavePorts>;
}

impl AhbMultiMasterConfig for combinatorial_os::OutputStage<PPBOutputSC> {
    type MastersEnum = SlavePorts;
    type Arbiter = FixedArbiter<SlavePorts>;
}

pub(super) type Interconnect = BusMatrixInterconnect;

/// Decides if requesting specified address by Fetch (on Instruction Bus)
/// needs registration of the transfer request (i.e. delaying signal
/// to the beginning of the next cycle).
///
/// It seems that only Fetch transfer requests to the System bus needs registration.
/// See: [ARM-TRM-G] 12.5.6.
pub(in crate::component) fn fetch_needs_registration(addr: Address) -> bool {
    Bus::from_map(addr.into(), false) == Bus::System
}

pub(super) fn end_combinatorial_changes(comp: &mut BusMatrixComponent, ctx: &mut Context) {
    combinatorial_os::OutputStage::<ICodeOutputSC>::send_output(comp, ctx);
    combinatorial_os::OutputStage::<DCodeOutputSC>::send_output(comp, ctx);
    combinatorial_os::OutputStage::<SystemOutputSC>::send_output(comp, ctx);
    combinatorial_os::OutputStage::<PPBOutputSC>::send_output(comp, ctx);
}

// // TODO: This may come useful when porting to other chips
// //       It may come in handy when porting to other SoCs
// #[derive(Subcomponent, TickComponent, DisableableComponent, Debug)]
// pub(crate) struct AMBANonCompliantDNOTITRANS {
//     current_addr_in_port: Option<Option<SlavePorts>>,
// }
//
// impl Default for AMBANonCompliantDNOTITRANS {
//     fn default() -> Self {
//         Self {
//             current_addr_in_port: Some(None),
//         }
//     }
// }
//
// impl TickComponentExtra for AMBANonCompliantDNOTITRANS {
//     fn tick_extra(&mut self) {
//         self.current_addr_in_port = None;
//     }
// }
//
// impl Arbiter<SlavePorts> for AMBANonCompliantDNOTITRANS {
//     fn arbitrate(
//         &mut self,
//         reqs: EnumMap<SlavePorts, bool>,
//         _last: MasterToSlaveAddrPhase,
//     ) -> Option<SlavePorts> {
//         // [ARM-TRM-G] 12.6 Unifying the code buses
//         // Suggests that using DNOTITRANS will mask HTRANS on ICode when HTRANS is active on DCode
//         // --> Thus again possibly changing transaction state on IDCode on waitstates?
//         // If DCode cannot mask ICode after it has presented HTRANS on the bus, then it would
//         // be equivalent to `CombinatorialFixedArbiter`, that is AMBA-compliant.
//         let visible_htrans_src = if reqs[SlavePorts::Data] {
//             Some(SlavePorts::Data)
//         } else if reqs[SlavePorts::Instruction] {
//             Some(SlavePorts::Instruction)
//         } else {
//             None
//         };
//         debug_assert!(!reqs[SlavePorts::Debugger] && self.current_addr_in_port.is_none());
//
//         trace!("DNOTITRANS from {:?} chosen {:?}", reqs, visible_htrans_src,);
//         self.current_addr_in_port = Some(visible_htrans_src);
//         visible_htrans_src
//     }
//
//     fn get_addr_in_port(&self) -> Option<SlavePorts> {
//         self.current_addr_in_port.unwrap()
//     }
// }
