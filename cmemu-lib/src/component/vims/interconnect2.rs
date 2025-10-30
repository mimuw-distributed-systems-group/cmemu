use super::cache::Mode;
use crate::common::new_ahb::arbiter::{FixedArbiter, NoArbiter};
use crate::common::new_ahb::databus::DataBus;
use crate::common::new_ahb::decoder::{AhbDecode, AhbPort as DPort, Decoder};
use crate::common::new_ahb::input_stage::InputStage;
use crate::common::new_ahb::interconnect::lite_wrapper::LiteWrapperCfg;
use crate::common::new_ahb::output_stage::{AhbPort as OPort, OutputStage};
use crate::common::new_ahb::ports::{
    AHBMasterPortInput, AHBMasterPortOutput, AHBPortConfig, AHBSlavePortInput, AHBSlavePortOutput,
};
use crate::common::new_ahb::signals::{
    MasterToSlaveWires, SlaveToMasterWires, TrackedBool, TransferMeta,
};
use crate::common::new_ahb::vlan::{
    AHBSoftVlanSlavePortInput, AhbDecoderTag, AhbMultiMasterConfig, AhbSlaveOutputDispatcher, Unit,
};
use crate::common::utils::{FromMarker, SubcomponentProxyMut, iter_enum};
use crate::component::vims::VIMSComponent;
use crate::engine::{
    CombFlop, Context, DisableableComponent, Subcomponent, TickComponent, TickComponentExtra,
};
use crate::utils::IfExpr;
use crate::{
    bridge_ports, build_interconnect, codegen_line_wrapper_for_interconnect,
    decoder_tags_and_markers, make_concrete_dispatcher, make_port_struct,
};
use cc2650_constants as soc;
use cmemu_common::address::RangeUnion;
use cmemu_common::{Address, address::EMPTY_RANGE, address_match_range_exhaustive};
use enum_map::{EnumMap, enum_map};
use std::fmt::Debug;
use std::ops::{Range, RangeFull};

decoder_tags_and_markers!(@with_markers
pub(crate) enum SlavePorts {
    Cache,
    IDCode,
    Sysbus,
});

const FLASH_ALIAS_RANGES: RangeUnion<Range<Address>, Range<Address>> = RangeUnion(
    // System bus has a line buffer somewhere, but it is not shared with the below.
    &soc::FLASHMEM::SYSTEM_ALIAS_ADDR_SPACE,
    // This has a line buffer, but diverts cache
    #[cfg(not(feature = "soc-stm32f100rbt6"))]
    &soc::FLASHMEM::UNCACHED_ADDR_SPACE,
    #[cfg(feature = "soc-stm32f100rbt6")]
    &soc::FLASHMEM::FLASH_OR_SYSTEM_ALIAS_ADDR_SPACE,
);

// Apparently, system may use the basic address as well
const FLASH_IDCODE_RANGE: Range<Address> = soc::FLASHMEM::ADDR_SPACE;
const FLASH_RANGES: RangeUnion<RangeUnion<Range<Address>, Range<Address>>, Range<Address>> =
    RangeUnion(&FLASH_ALIAS_RANGES, &FLASH_IDCODE_RANGE);

decoder_tags_and_markers!(@with_dispatcher
pub(crate) enum MasterPorts {
    Flash = FLASH_RANGES,
    GPRAM = soc::GPRAM::ADDR_SPACE,
    ROM = soc::BROM::ADDR_SPACE,
    Internal = soc::VIMS::ADDR_SPACE,
    CacheS = EMPTY_RANGE,
});

build_interconnect!(
    VimsInterconnect
    masters SlavePorts => [IDCode, Sysbus, Cache]
    slaves MasterPorts => [Flash, GPRAM, ROM, Internal, CacheS]
    using InputStage as input, Decoder=>DPort as decoder, and OPort=>OutputStage as output
);
impl AhbDecoderTag for Decoder<IDCodeDecoderSC> {
    type Enum = Option<MasterPorts>;

    fn dynamic_decode(comp: &VIMSComponent, _ctx: &mut Context, meta: &TransferMeta) -> Self::Enum {
        let gpram_mode = comp.cache.get_target_mode_for_addr_routing();
        address_match_range_exhaustive! {meta.addr,
            soc::FLASHMEM::ADDR_SPACE :if gpram_mode == Mode::Cache => Some(MasterPorts::CacheS),
            FLASH_RANGES => Some(MasterPorts::Flash),
            soc::GPRAM::ADDR_SPACE :if gpram_mode == Mode::GPRAM => Some(MasterPorts::GPRAM),
            soc::GPRAM::ADDR_SPACE => None,
            RangeFull => Self::decode(meta.addr)
        }
    }
}
impl AhbDecoderTag for Decoder<SysbusDecoderSC> {
    type Enum = Option<MasterPorts>;

    fn dynamic_decode(comp: &VIMSComponent, _ctx: &mut Context, meta: &TransferMeta) -> Self::Enum {
        let gpram_mode = comp.cache.get_target_mode_for_addr_routing();
        address_match_range_exhaustive! {meta.addr,
            FLASH_RANGES => Some(MasterPorts::Flash),
            soc::GPRAM::ADDR_SPACE :if gpram_mode == Mode::GPRAM => Some(MasterPorts::GPRAM),
            soc::GPRAM::ADDR_SPACE => None,
            RangeFull => Self::decode(meta.addr)
        }
    }
}
impl AhbDecoderTag for Decoder<CacheDecoderSC> {
    type Enum = Option<MasterPorts>;
    const REFLECTS_HREADY: bool = false;
    const GRANTER: Option<fn(&mut Self::Component, &mut Context, TrackedBool)> =
        Some(|_, _, granted| {
            if *granted {
                return;
            }
            unimplemented!(
                "Arbitration between Cache and Sysbus is not implemented! \
            The constants will need research."
            )
        });

    fn decode(addr: Address) -> Self::Enum {
        let res = Self::Enum::decode(addr);
        debug_assert_eq!(res, Some(MasterPorts::Flash));
        res
    }
}

impl AhbMultiMasterConfig for OutputStage<FlashOutputSC> {
    type MastersEnum = SlavePorts;
    type Arbiter = FixedArbiter<SlavePorts>;
}

impl VIMSComponent {
    // TODO: we currently don't support unknown waitstates inside cache
    // We also have many constants in the cache implementation.
    pub(crate) fn cache_arbitration_hax(comp: &mut VIMSComponent, enabled: bool) {
        let this = FlashOutputSC::component_to_member_mut(comp);
        this.get_arbiter_unchecked()
            .force_req_hax(enabled.ife(Some(SlavePorts::Cache), None));
    }
}

// TODO: The arbiters need some research: Fixed is generally faster.
impl AhbMultiMasterConfig for OutputStage<GPRAMOutputSC> {
    type MastersEnum = SlavePorts;
    type Arbiter = FixedArbiter<SlavePorts, IDCode>;
}
impl AhbMultiMasterConfig for OutputStage<CacheSOutputSC> {
    type MastersEnum = SlavePorts;
    type Arbiter = NoArbiter<SlavePorts, IDCode>;
}
impl AhbMultiMasterConfig for OutputStage<ROMOutputSC> {
    type MastersEnum = SlavePorts;
    type Arbiter = NoArbiter<SlavePorts, IDCode>;
}
impl AhbMultiMasterConfig for OutputStage<InternalOutputSC> {
    type MastersEnum = SlavePorts;
    type Arbiter = FixedArbiter<SlavePorts, IDCode>;
}

codegen_line_wrapper_for_interconnect!(VimsInterconnect; pub(super));

impl LiteWrapperCfg for LiteWrapper {
    type Data = DataBus;
    type InputTag = SlavePorts;
    type OutputTag = MasterPorts;
}
