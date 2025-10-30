//! Currently we don't have a full reference of the bus connections mapping in the SoC.
//! AON* and AUX* address spaces lie inside `AON_VD` (Voltage Domain), see \[TI-TRM] 6.4 Digital Power Partitioning
//! Therefore it is likely that there is a single bridge interface from `MCU_VD`.
#[proxy_use]
use crate::common::new_ahb::MasterToSlaveWires;
use crate::common::new_ahb::decoder::{AhbDecode, AhbPort, Decoder, DefaultSlave};
#[proxy_use]
use crate::common::new_ahb::ports::AHBSlavePortInput;
use crate::common::new_ahb::ports::AhbSlavePortOutputWithGranting;
use crate::common::new_ahb::ports::{AHBMasterPortOutput, AHBSlavePortProxiedInput};
use crate::common::new_ahb::signals::TrackedBool;
#[proxy_use]
use crate::common::new_ahb::vlan::AHBSoftVlanMasterPortInput;
use crate::common::new_ahb::vlan::{
    AHBMasterPortTaggedInput, AhbDecoderTag, AhbMasterOutputDispatcher,
};
#[proxy_use]
use crate::common::new_ahb::{AHBPortConfig, DataBus, SlaveToMasterWires};
#[proxy_use(proxy_only)]
use crate::component::aon_bus::PublicMasters;
use crate::component::aon_event::AON_EVENT_ROUTE_INJECTION;
use crate::component::osc::OSC_ROUTE_INJECTION;
use crate::component::rtc::RTC_ROUTE_INJECTION;
use crate::component::sync_down_bridge::SyncDownBridge;
use crate::component::wuc::WUC_ROUTE_INJECTION;
#[proxy_use]
use crate::engine::{Context, Subcomponent};
use crate::engine::{
    DisableableComponent, MainComponent, SkippableClockTreeNode, TickComponent, TickComponentExtra,
};
use crate::proxy::AonBusProxy;
use crate::{bridge_ports, expose_ports};
use cc2650_constants as soc;
use cmemu_common::address::RangeUnion;
use cmemu_common::{Address, address_match_range};
use cmemu_proc_macros::{component_impl, handler, proxy_use};
use log::trace;
use std::ops::Range;

// pub const AON_BUS_ROUTE_INJECTION: Range<Address> =
//     soc::AON::ADDR_SPACE.start..soc::AUX::ADDR_SPACE.end;
#[allow(clippy::type_complexity)]
pub const AON_BUS_ROUTE_INJECTION: RangeUnion<
    RangeUnion<Range<Address>, Range<Address>>,
    RangeUnion<Range<Address>, Range<Address>>,
> = RangeUnion(
    &RangeUnion(&OSC_ROUTE_INJECTION, &RTC_ROUTE_INJECTION),
    &RangeUnion(&AON_EVENT_ROUTE_INJECTION, &WUC_ROUTE_INJECTION),
);

pub(crate) type SlowDecoder = Decoder<SlowDecoderSC>;

pub(crate) struct SlowDecoderSC {}

#[derive(MainComponent, TickComponent, TickComponentExtra, DisableableComponent)]
pub(crate) struct AonBusComponent {
    #[subcomponent(SyncDownBridge)]
    sync_down_bridge: SyncDownBridge,
}

#[component_impl(aon_bus)]
impl AonBusComponent {
    pub(crate) fn new() -> Self {
        Self {
            sync_down_bridge: SyncDownBridge::new(),
        }
    }

    pub(crate) fn tick(&mut self, ctx: &mut Context) {
        // SlowDecoder::sub_tick(self, ctx);
        SyncDownBridge::sub_tick(self, ctx);
    }

    pub(crate) fn tock(&mut self, ctx: &mut Context) {
        // SlowDecoder::tock(self, ctx);
        SyncDownBridge::tock(self, ctx);
    }

    #[handler]
    pub fn on_new_ahb_slave_input(
        &mut self,
        ctx: &mut Context,
        msg: MasterToSlaveWires<<AonBusComponent as AHBPortConfig>::Data>,
    ) {
        <Self as AHBSlavePortInput>::on_ahb_input(self, ctx, msg);
    }

    #[handler]
    pub fn on_new_ahb_master_tagged_input(
        &mut self,
        ctx: &mut Context,
        tag: PublicMasters,
        msg: SlaveToMasterWires<<AonBusComponent as AHBPortConfig>::Data>,
    ) {
        <SlowDecoder as AHBSoftVlanMasterPortInput<PublicMasters>>::on_ahb_soft_tagged_input(
            self, ctx, tag, msg,
        );
    }
}

#[component_impl(aon_bus)]
impl SkippableClockTreeNode for AonBusComponent {
    fn max_cycles_to_skip(
        comp: &mut Self::Component,
        _ctx: &mut Context,
        _parent: Self::IdSpace,
        _extra: &mut Self::Extra,
    ) -> u64 {
        let this = comp;
        if this.can_be_disabled_now() {
            // this.sync_down_bridge
            //     .divider()
            //     .tick_to_event()
            //     .saturating_sub(1) as u64
            u64::MAX
        } else {
            0
        }
    }

    fn emulate_skipped_cycles(
        comp: &mut Self::Component,
        _ctx: &mut Context,
        _parent: Self::IdSpace,
        _extra: &mut Self::Extra,
        skipped_cycles: u64,
    ) {
        let this = comp;
        trace!("AonBus::emulate_skipped_cycles({skipped_cycles:?})");
        this.sync_down_bridge
            .divider_mut()
            .fast_forward_ticks(skipped_cycles);
    }
}

bridge_ports!(@slave AonBusComponent => @auto_configured @slave SyncDownBridge);
bridge_ports!(@with_granting @master SyncDownBridge => @auto_configured @slave SlowDecoder);

#[component_impl(aon_bus)]
impl AHBPortConfig for AonBusComponent {
    type Data = DataBus;
    type Component = <Self as Subcomponent>::Component;
    const TAG: &'static str = "AonBus";
}
#[component_impl(aon_bus)]
impl AHBSlavePortProxiedInput for AonBusComponent {
    fn proxy_ahb_input(ctx: &mut Context, msg: MasterToSlaveWires<Self::Data>) {
        AonBusProxy.on_new_ahb_slave_input(ctx, msg);
    }
}

impl AhbDecoderTag for SlowDecoder {
    const REFLECTS_HREADY: bool = false;
    const GRANTER: Option<fn(&mut Self::Component, &mut Context, TrackedBool)> =
        Some(<Self as AhbSlavePortOutputWithGranting>::send_grant_wire);
    type Enum = Option<PublicMasters>;
}

expose_ports! {
    AonBusComponent data = DataBus,
    Master PublicMasters [proxy=AonBusProxy.on_new_ahb_master_tagged_input] {
        RtcMPort, // part of AON
        AonEventMPort, // part of AON
        WucMPort, // part of AON
        OscMPort, // part of AUX
    }
}

// TODO: remove this boilerplace and implement it under expore_ports! with some flag
impl AhbDecode for Option<PublicMasters> {
    fn decode(addr: Address) -> Self {
        address_match_range!(addr,
            soc::AON_RTC::ADDR_SPACE => Some(PublicMasters::RtcMPort),
            soc::AON_EVENT::ADDR_SPACE => Some(PublicMasters::AonEventMPort),
            soc::AON_WUC::ADDR_SPACE => Some(PublicMasters::WucMPort),
            soc::AUX_DDI0_OSC::ADDR_SPACE => Some(PublicMasters::OscMPort),
            _ => None,
        )
    }
}

impl AHBSoftVlanMasterPortInput<PublicMasters> for SlowDecoder {
    fn on_ahb_soft_tagged_input(
        comp: &mut Self::Component,
        ctx: &mut Context,
        tag: PublicMasters,
        msg: SlaveToMasterWires<Self::Data>,
    ) {
        <SlowDecoder as AHBMasterPortTaggedInput>::on_ahb_tagged_input(comp, ctx, Some(tag), msg);
    }
}

impl AhbMasterOutputDispatcher<Option<PublicMasters>> for SlowDecoder {
    fn dispatch_ahb_output(
        comp: &mut Self::Component,
        ctx: &mut Context,
        tag: Self::Enum,
        msg: MasterToSlaveWires<Self::Data>,
    ) {
        match tag {
            Some(PublicMasters::RtcMPort) => {
                <RtcMPort as AHBMasterPortOutput>::send_ahb_output(comp, ctx, msg);
            }
            Some(PublicMasters::AonEventMPort) => {
                <AonEventMPort as AHBMasterPortOutput>::send_ahb_output(comp, ctx, msg);
            }
            Some(PublicMasters::WucMPort) => {
                <WucMPort as AHBMasterPortOutput>::send_ahb_output(comp, ctx, msg);
            }
            Some(PublicMasters::OscMPort) => {
                <OscMPort as AHBMasterPortOutput>::send_ahb_output(comp, ctx, msg);
            }
            _ => <AhbPort<SlowDecoderSC, DefaultSlave> as AHBMasterPortOutput>::send_ahb_output(
                comp, ctx, msg,
            ),
        }
    }
}
// This is second-level boilerplate (since expose_ports! require hard-vlan level routing)
bridge_ports!(@no_m2s @master AhbPort<SlowDecoderSC, OscMPort> => @master OscMPort);
bridge_ports!(@no_m2s @master AhbPort<SlowDecoderSC, RtcMPort> => @master RtcMPort);
bridge_ports!(@no_m2s @master AhbPort<SlowDecoderSC, AonEventMPort> => @master AonEventMPort);
bridge_ports!(@no_m2s @master AhbPort<SlowDecoderSC, WucMPort> => @master WucMPort);

impl crate::engine::Subcomponent for SlowDecoderSC {
    type Component = <AonBusComponent as crate::engine::Subcomponent>::Component;
    type Member = SlowDecoder;
    fn component_to_member(component: &Self::Component) -> &Self::Member {
        &<AonBusComponent as crate::engine::Subcomponent>::component_to_member(component)
            .sync_down_bridge
            .decoder
    }
    fn component_to_member_mut(component: &mut Self::Component) -> &mut Self::Member {
        &mut <AonBusComponent as crate::engine::Subcomponent>::component_to_member_mut(component)
            .sync_down_bridge
            .decoder
    }
}
