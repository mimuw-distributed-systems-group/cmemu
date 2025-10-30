use crate::bridge_ports;
use crate::common::new_ahb::ports::AHBSlavePortInput;
use crate::common::new_ahb::signals::{MasterToSlaveWires, TrackedBool};
use crate::common::new_ahb::write_buffer::WriteBufferCfg;
#[cfg(not(feature = "nm-unstable"))]
use crate::component::bus_matrix::BusMatrixComponent;
use crate::component::bus_matrix::aligner::{AddrHandlingMode, AlignerCfg};
use crate::component::bus_matrix::registration_buffer::IBusRegistrationBuffer;
use crate::engine::Context;
use crate::make_concrete_dispatcher;
use crate::proxy::CoreProxy;
use cc2650_constants::CoreMap::CoreMemoryMap;
use cmemu_common::Address;

use super::interconnect::{
    DCode, Data, Debugger, ICode, Input, Instruction, Output, PPB as PPBMarker, SlavePorts, System,
};
use super::{DBusAligner, DBusS, DCodeM, DCodeWB, DebugS, IBusS, ICodeM, SysbusM, SystemWB};

// Function from SlavePorts enum to the mentioned function
make_concrete_dispatcher!(
    _rt_dispatch_input on AHBSlavePortInput::on_ahb_input<MasterToSlaveWires>:
    SlavePorts for Instruction, Data, Debugger
);

//////////
// Output
//////////

bridge_ports!(@auto_configured @master Output<ICode> => @master ICodeM);

#[cfg(feature = "nm-unstable")]
bridge_ports!(@auto_configured @master Output<DCode> => @slave DCodeWB);
#[cfg(feature = "nm-unstable")]
bridge_ports!(@auto_configured @master DCodeWB => @master DCodeM);

// TODO: Write buffer has a bug on unbuffered addresses. We need more tests of this component.
#[cfg(not(feature = "nm-unstable"))]
bridge_ports!(@auto_configured @master Output<DCode> => @master DCodeM);
#[cfg(not(feature = "nm-unstable"))]
use crate::terminate_port;
#[cfg(not(feature = "nm-unstable"))]
terminate_port!(@unconfigured_twosided DCodeWB where component=BusMatrixComponent);

impl WriteBufferCfg for DCodeWB {
    const IS_BUF_TO_LOAD_FAST: bool = false;
}

impl WriteBufferCfg for SystemWB {
    const IS_BUF_TO_LOAD_FAST: bool = false;
}
#[cfg(feature = "nm-unstable")]
bridge_ports!(@auto_configured @master Output<System> => @slave SystemWB);
#[cfg(feature = "nm-unstable")]
bridge_ports!(@auto_configured @master SystemWB => @master SysbusM);
#[cfg(not(feature = "nm-unstable"))]
bridge_ports!(@auto_configured @master Output<System> => @master SysbusM);
#[cfg(not(feature = "nm-unstable"))]
terminate_port!(@unconfigured_twosided SystemWB where component=BusMatrixComponent);

// There is no write buffer on this output. For proof see tests in misc/dwt/*.tzst (no increment seen)
// TODO: this test checks only if IS_BUF_TO_LOAD_FAST -> need a test with multiple stalled stores
//       Possible with TPIU.
bridge_ports!(@auto_configured @master Output<PPBMarker> => @slave super::PPB);

//////////
// Input
//////////

bridge_ports!(@slave IBusS => @auto_configured @slave IBusRegistrationBuffer);
bridge_ports!(@master IBusRegistrationBuffer => @auto_configured @slave Input<Instruction>);

bridge_ports!(@slave DebugS => @auto_configured @slave Input<Debugger>);
bridge_ports!(@slave DBusS => @auto_configured @slave DBusAligner);
bridge_ports!(@master DBusAligner => @auto_configured @slave Input<Data>);
impl AlignerCfg for DBusAligner {
    const GRANTER: Option<fn(&mut Self::Component, &mut Context, TrackedBool)> =
        Some(|_, ctx, granted| {
            CoreProxy.on_grant_data(ctx, granted);
        });

    fn how_to_handle_unaligned(addr: Address) -> AddrHandlingMode {
        // TODO: read the docs about unaligned, there is something that some regions are always LE
        match CoreMemoryMap::from(addr) {
            CoreMemoryMap::PPB(_) => AddrHandlingMode::AddrTruncatedReinterpret,
            _ => AddrHandlingMode::Aligner,
        }
    }
}
