use super::cache;
use crate::common::new_ahb;
use crate::common::new_ahb::databus::DataBus;
use crate::common::new_ahb::line_buffer::LineBufferCfg;
use crate::common::new_ahb::ports::{AHBPortConfig, AHBSlavePortInput};
use crate::common::new_ahb::signals::{MasterToSlaveWires, Size, TransferMeta};
use crate::common::new_ahb::vlan::AHBSoftVlanSlavePortInput;
use crate::component::vims::{
    CacheRAMComponent, CodeCacheLineBufferSubcomponent, CodeFlashLineBufferSubcomponent,
    SysbusSPort, VIMSComponent, VIMSRegistersDriverSubcomponent,
};
use crate::engine::Context;
use crate::{bridge_ports, make_concrete_dispatcher};
// use super::flash_mux;
use super::idcode_mux::IDCodeSPort;
use super::interconnect2::{
    Cache, CacheS, Flash, GPRAM, IDCode, Internal, LiteInput as Input, LiteOutput as Output,
    LiteWrapper as Interconnect, ROM, SlavePorts, Sysbus,
};
use super::{FlashMPort, GpramMPort, RomMPort};

pub(super) type CodeFlashLineBuffer =
    new_ahb::line_buffer::LineBuffer<CodeFlashLineBufferSubcomponent>;
pub(super) type CodeCacheLineBuffer =
    new_ahb::line_buffer::LineBuffer<CodeCacheLineBufferSubcomponent>;

pub(super) type VIMSRegistersDriver = new_ahb::slave_driver::SimpleSynchronousSlaveInterface<
    VIMSRegistersDriverSubcomponent,
    RegistersPort,
>;

pub(crate) struct RegistersPort;
bridge_ports!(@auto_configured Output<Internal> => RegistersPort);
bridge_ports!(@slave RegistersPort => @slave VIMSRegistersDriver);

impl LineBufferCfg for CodeFlashLineBuffer {
    const UPSIZED: Size = if cfg!(feature = "soc-cc2652") {
        Size::FourWord
    } else if cfg!(feature = "soc-stm32f100rbt6") {
        Size::Word
    } else {
        Size::Doubleword
    };

    fn extract_upstream_from_upsized(
        addr: &TransferMeta,
        data: &<Self as AHBPortConfig>::Data,
    ) -> <Self as AHBPortConfig>::Data {
        data.clone().extract_from_aligned(addr.addr, addr.size)
    }
}

impl AHBPortConfig for CodeFlashLineBuffer {
    type Data = DataBus;
    type Component = VIMSComponent;
    const TAG: &'static str = "CodeFlashLB";
}

impl LineBufferCfg for CodeCacheLineBuffer {
    const UPSIZED: Size = Size::Doubleword;
    const ENABLED_BY_DEFAULT: bool = false;

    fn extract_upstream_from_upsized(
        addr: &TransferMeta,
        data: &<Self as AHBPortConfig>::Data,
    ) -> <Self as AHBPortConfig>::Data {
        data.clone().extract_from_aligned(addr.addr, addr.size)
    }
}

impl AHBPortConfig for CodeCacheLineBuffer {
    type Data = DataBus;
    type Component = VIMSComponent;
    const TAG: &'static str = "CodeCacheLB";
}

make_concrete_dispatcher!(
    _rt_dispatch_input on AHBSlavePortInput::on_ahb_input<MasterToSlaveWires>:
    SlavePorts for IDCode, Sysbus, Cache
);
// TODO: rework vlan a bit if that works
impl AHBPortConfig for Interconnect {
    type Data = DataBus;
    type Component = VIMSComponent;
    const TAG: &'static str = "VimsIC";
}

impl AHBSoftVlanSlavePortInput<SlavePorts> for Interconnect {
    fn on_ahb_soft_tagged_input(
        comp: &mut Self::Component,
        ctx: &mut Context,
        tag: SlavePorts,
        msg: MasterToSlaveWires<Self::Data>,
    ) {
        _rt_dispatch_input(tag)(comp, ctx, msg);
    }
}

bridge_ports!(@auto_configured Output<Flash> => CodeFlashLineBuffer);
bridge_ports!(@master CodeFlashLineBuffer => @master FlashMPort);

bridge_ports!(@auto_configured @master Output<GPRAM> => @master GpramMPort);

// bridge_ports!(@auto_configured @master flash_mux::FlashMultiMaster => @master FlashMPort);
bridge_ports!(@auto_configured @master Output<ROM> => @master RomMPort);

bridge_ports!(@slave IDCodeSPort => @auto_configured @slave Input<IDCode>);
bridge_ports!(@slave SysbusSPort => @auto_configured  @slave Input<Sysbus>);

impl AHBPortConfig for RegistersPort {
    type Data = DataBus;
    type Component = VIMSComponent;
    const TAG: &'static str = "VimsRegisters";
}

// CACHE
bridge_ports!(@auto_configured Output<CacheS> => CodeCacheLineBuffer);
bridge_ports!(@master CodeCacheLineBuffer => @slave cache::CacheComponent);
bridge_ports!(@master cache::FlashPort => @auto_configured @slave Input<Cache>);
bridge_ports!(@master cache::CacheRAMPort => @slave CacheRAMComponent);
