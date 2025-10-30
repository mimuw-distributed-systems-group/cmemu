use cmemu_proc_macros::{component_impl, handler, proxy_use};

use crate::common::new_ahb::databus::DataBus;
#[proxy_use]
use crate::common::new_ahb::ports::{AHBPortConfig, AHBSlavePortInput};
#[proxy_use]
use crate::common::new_ahb::signals::{MasterToSlaveWires, SlaveToMasterWires};
use crate::common::new_ahb::vlan::AHBSoftVlanMasterPortInput;
use crate::component::bitband::Bitband;
use crate::component::semi_hosting::SemiHosting;
#[proxy_use(proxy_only)]
use crate::component::sysbus::PublicMasters;
#[proxy_use]
use crate::engine::Context;
use crate::engine::{
    DisableableComponent, MainComponent, PowerNode, SkippableClockTreeNode, TickComponent,
    TickComponentExtra,
};
use crate::proxy::SystemBusProxy;
use crate::{expose_ports, make_port_struct};

#[derive(
    MainComponent, SkippableClockTreeNode, TickComponent, TickComponentExtra, DisableableComponent,
)]
#[skippable_if_disableable]
pub(crate) struct SystemBusComponent {
    #[subcomponent(Interconnect)]
    interconnect: Interconnect,

    // TODO: find better place for this
    #[subcomponent(SemiHosting)]
    pub(crate) semi_hosting: SemiHosting,

    #[subcomponent(Bitband)]
    bitband: Bitband,
}

type Interconnect = interconnect::LiteWrapper;

#[component_impl(sysbus)]
impl SystemBusComponent {
    pub(crate) fn new() -> Self {
        Self {
            interconnect: Interconnect::new(),
            semi_hosting: SemiHosting::new(),
            bitband: Bitband::new(),
        }
    }

    pub(crate) fn tick(&mut self, ctx: &mut Context) {
        Interconnect::tick(self, ctx);
        SemiHosting::tick(self, ctx);
        Bitband::tick(self, ctx);
    }

    pub(crate) fn tock(&mut self, ctx: &mut Context) {
        Interconnect::tock(self, ctx);
        SemiHosting::tock(self, ctx);
        Bitband::tock(self, ctx);
    }

    #[handler]
    pub fn on_new_ahb_slave_input(
        &mut self,
        ctx: &mut Context,
        msg: MasterToSlaveWires<<SystemBusComponent as AHBPortConfig>::Data>,
    ) {
        // TODO: reduce copy-paste factor of this part
        if !<Self as PowerNode>::is_active(self, ctx) {
            debug_assert!(msg.is_inert());
            return;
        }
        <CoreSPort as AHBSlavePortInput>::on_ahb_input(self, ctx, msg);
    }

    #[handler]
    pub fn on_new_ahb_master_tagged_input(
        &mut self,
        ctx: &mut Context,
        tag: PublicMasters,
        msg: SlaveToMasterWires<<SystemBusComponent as AHBPortConfig>::Data>,
    ) {
        if !<Self as PowerNode>::is_active(self, ctx) {
            debug_assert!(msg.is_inert());
            return;
        }
        <Self as AHBSoftVlanMasterPortInput<PublicMasters>>::on_ahb_soft_tagged_input(
            self, ctx, tag, msg,
        );
    }
}

#[component_impl(sysbus)]
impl AHBPortConfig for SystemBusComponent {
    type Data = DataBus;
    type Component = Self;
    const TAG: &'static str = "SysBus";
}

expose_ports! {
    SystemBusComponent data = DataBus,
    Master PublicMasters [proxy=SystemBusProxy.on_new_ahb_master_tagged_input] {
        SramMPort,
        VimsMPort,
        PrcmMPort,
        RTCBypassMPort,
        AonBusMPort,
        MemMockMPort,
        UartLiteMPort,
        GpioMPort,
        RfcMPort,
        EventFabricMPort,
    }
}
// Public ports
make_port_struct!(pub(crate) CoreSPort);

mod interconnect {
    //! For general overview of busses in ARM Cortex-M3 see [ARM-TDG] 6.2
    use std::fmt::Debug;
    use std::ops::{Range, RangeFull};

    use enum_map::{EnumMap, enum_map};

    use crate::common::Address;
    use crate::common::new_ahb::arbiter::{FixedArbiter, NoArbiter};
    use crate::common::new_ahb::databus::DataBus;
    use crate::common::new_ahb::decoder::{AhbDecode, AhbPort as DPort, Decoder};
    use crate::common::new_ahb::input_stage::InputStage;
    use crate::common::new_ahb::interconnect::lite_wrapper::LiteWrapperCfg;
    use crate::common::new_ahb::output_stage::{AhbPort as OPort, OutputStage};
    use crate::common::new_ahb::ports::{
        AHBMasterPortInput, AHBMasterPortOutput, AHBPortConfig, AHBSlavePortInput,
        AHBSlavePortOutput, AHBSlavePortProxiedInput, AhbSlavePortOutputWithGranting,
    };
    use crate::common::new_ahb::signals::{
        MasterToSlaveAddrPhase, MasterToSlaveWires, SlaveToMasterWires, TrackedBool,
    };
    use crate::common::new_ahb::vlan::{
        AHBSoftVlanSlavePortInput, AhbDecoderTag, AhbMultiMasterConfig, AhbSlaveOutputDispatcher,
        Unit,
    };
    use crate::common::utils::{FromMarker, SubcomponentProxyMut, iter_enum};
    use crate::component::semi_hosting;
    use crate::component::sysbus::CoreSPort;
    use crate::component::{aon_bus, bitband, gpio, prcm, rtc_bypass};
    use crate::engine::{
        CombFlop, Context, DisableableComponent, Subcomponent, TickComponent, TickComponentExtra,
    };
    use crate::proxy::SystemBusProxy;
    use crate::utils::IfExpr;
    use crate::{bridge_ports, make_concrete_dispatcher, make_port_struct};
    use crate::{
        build_interconnect, codegen_line_wrapper_for_interconnect, decoder_tags_and_markers,
    };
    use cc2650_constants as soc;
    use cmemu_common::address::RangeUnion;
    use cmemu_common::address_match_range;

    use super::{
        AonBusMPort, EventFabricMPort, GpioMPort, MemMockMPort, PrcmMPort, RTCBypassMPort,
        RfcMPort, SramMPort, SystemBusComponent, UartLiteMPort, VimsMPort,
    };

    decoder_tags_and_markers!(@with_markers
    pub(crate) enum SlavePorts {
        Core,
        BitbandS,
    });

    // Bitband memory region is not continuous. The true region is used in Decoder<CoreDecoderSC>::decode().
    const BITBAND_REGION: RangeUnion<Range<Address>, Range<Address>> = RangeUnion(
        &bitband::SRAM_BITBAND_ALIAS_REGION,
        &bitband::PERIPH_BITBAND_ALIAS_REGION,
    );

    decoder_tags_and_markers!(@with_dispatcher
    pub(crate) enum MasterPorts {
        VIMS = soc::VIMS::ADDR_SPACE,
        SRAM = soc::SRAM::ADDR_SPACE,
        // TODO: this would be better as a decoder in semihosting, but I'm not sure if we have it working
        SemiOsData = semi_hosting::OS_DATA_RANGE,
        Semi = semi_hosting::SEMI_HOSTING_ADDR_SPACE,

        // Routing is enabled only if they are implemented on this branch.
        PRCM = prcm::PRCM_ROUTE_INJECTION,
        RTCBypass = rtc_bypass::ROUTE_INJECTION,
        AonBus = aon_bus::AON_BUS_ROUTE_INJECTION,
        // TODO: UartLite should be under AonBus
        UartLite = soc::AUX_RAM::ADDR_SPACE,
        BitbandM = BITBAND_REGION,
        GPIO = gpio::GPIO_ROUTE_INJECTION,
        RFC = soc::RFC::ADDR_SPACE,
        EventFabric = soc::EVENT::ADDR_SPACE,
        MemMock = RangeFull,
    });

    build_interconnect!(
        SysbusInterconnect
        masters SlavePorts => [Core, BitbandS]
        slaves MasterPorts => [VIMS, SRAM, SemiOsData, Semi, PRCM, RTCBypass, AonBus, UartLite, BitbandM, GPIO, MemMock, RFC, EventFabric]
        using InputStage as input, Decoder=>DPort as decoder, and OPort=>OutputStage as output
    );

    // TODO: one should not use bit-banding to set bits in SCS memory region
    // because no bit-banding for SCS exists. The use of nonexisting bit-banding
    // was caused by a bug in TI driverlib which assumed there exists one.
    // We will temporarily support the bit-band address as our tests
    // are driverlib-based and access the bit-band address.
    // In the future we probably should warn about access caused by bug in driverlib.
    // NOTE(matrach): This is VENDOR space routed back to PPB and TI could put here anything
    const WRITE_BUFFER_DISABLE_BIT_BAND_ADDRESS: Address = Address::from_const(0xE21C_0104);

    impl AhbDecoderTag for Decoder<CoreDecoderSC> {
        type Enum = Option<MasterPorts>;
        // PROOF: a test with pipelined ldr GPIO; ldr SRAM
        const REFLECTS_HREADY: bool = false;
        const GRANTER: Option<fn(&mut Self::Component, &mut Context, TrackedBool)> =
            Some(<Self as AhbSlavePortOutputWithGranting>::send_grant_wire);

        fn decode(addr: Address) -> Self::Enum {
            address_match_range! {addr,
                WRITE_BUFFER_DISABLE_BIT_BAND_ADDRESS => None,
                _ => Self::Enum::decode(addr)
            }
        }

        fn stateless_mock(msg: &MasterToSlaveAddrPhase) -> Option<Self::Data> {
            let meta = msg.meta.meta().unwrap();
            if meta.addr == WRITE_BUFFER_DISABLE_BIT_BAND_ADDRESS {
                return Some(meta.is_writing().ife(
                    DataBus::HighZ,
                    DataBus::Word(0).extract_from_aligned(meta.addr, meta.size),
                ));
            }
            paranoid!(warn, "Unsupported request to a default slave: {:?}", msg);
            None
        }
    }

    impl AhbDecoderTag for Decoder<BitbandSDecoderSC> {
        type Enum = Option<MasterPorts>;
        const REFLECTS_HREADY: bool = true;
    }

    impl AhbMultiMasterConfig for OutputStage<VIMSOutputSC> {
        type MastersEnum = SlavePorts;
        type Arbiter = FixedArbiter<SlavePorts>;
    }

    impl AhbMultiMasterConfig for OutputStage<SRAMOutputSC> {
        type MastersEnum = SlavePorts;
        type Arbiter = FixedArbiter<SlavePorts>;
    }
    impl AhbMultiMasterConfig for OutputStage<BitbandMOutputSC> {
        type MastersEnum = SlavePorts;
        type Arbiter = NoArbiter<SlavePorts, Core>;
    }
    impl AhbMultiMasterConfig for OutputStage<PRCMOutputSC> {
        type MastersEnum = SlavePorts;
        type Arbiter = FixedArbiter<SlavePorts>;
    }
    impl AhbMultiMasterConfig for OutputStage<RTCBypassOutputSC> {
        type MastersEnum = SlavePorts;
        type Arbiter = FixedArbiter<SlavePorts>;
    }
    impl AhbMultiMasterConfig for OutputStage<AonBusOutputSC> {
        type MastersEnum = SlavePorts;
        type Arbiter = FixedArbiter<SlavePorts>;
    }
    impl AhbMultiMasterConfig for OutputStage<SemiOsDataOutputSC> {
        type MastersEnum = SlavePorts;
        type Arbiter = NoArbiter<SlavePorts, Core>;
    }
    impl AhbMultiMasterConfig for OutputStage<SemiOutputSC> {
        type MastersEnum = SlavePorts;
        type Arbiter = NoArbiter<SlavePorts, Core>;
    }
    impl AhbMultiMasterConfig for OutputStage<UartLiteOutputSC> {
        type MastersEnum = SlavePorts;
        type Arbiter = FixedArbiter<SlavePorts>;
    }
    impl AhbMultiMasterConfig for OutputStage<RFCOutputSC> {
        type MastersEnum = SlavePorts;
        type Arbiter = FixedArbiter<SlavePorts>;
    }
    impl AhbMultiMasterConfig for OutputStage<EventFabricOutputSC> {
        type MastersEnum = SlavePorts;
        type Arbiter = FixedArbiter<SlavePorts>;
    }

    impl AhbMultiMasterConfig for OutputStage<MemMockOutputSC> {
        type MastersEnum = SlavePorts;
        type Arbiter = FixedArbiter<SlavePorts>;
    }
    impl AhbMultiMasterConfig for OutputStage<GPIOOutputSC> {
        type MastersEnum = SlavePorts;
        type Arbiter = FixedArbiter<SlavePorts>;
    }

    codegen_line_wrapper_for_interconnect!(SysbusInterconnect; pub(super));
    impl LiteWrapperCfg for LiteWrapper {
        type Data = DataBus;
        type InputTag = SlavePorts;
        type OutputTag = MasterPorts;
    }

    impl AHBPortConfig for CoreSPort {
        type Data = DataBus;
        type Component = SystemBusComponent;
        const TAG: &'static str = "CoreSPort";
    }

    impl AHBSlavePortProxiedInput for CoreSPort {
        fn proxy_ahb_input(ctx: &mut Context, msg: MasterToSlaveWires<Self::Data>) {
            SystemBusProxy.on_new_ahb_slave_input(ctx, msg);
        }
    }

    pub(crate) struct StatelessTap;

    bridge_ports!(@auto_configured @master LiteOutput<VIMS> => @master VimsMPort);
    bridge_ports!(@auto_configured @master LiteOutput<SRAM> => @master SramMPort);
    bridge_ports!(@auto_configured @master LiteOutput<Semi> => @slave semi_hosting::SemiHosting);
    bridge_ports!(@auto_configured @master LiteOutput<SemiOsData> => @slave semi_hosting::OsDataMemory);
    bridge_ports!(@auto_configured @master LiteOutput<PRCM> => @master PrcmMPort);
    bridge_ports!(@auto_configured @master LiteOutput<RTCBypass> => @master RTCBypassMPort);
    bridge_ports!(@auto_configured @master LiteOutput<AonBus> => @master AonBusMPort);
    bridge_ports!(@auto_configured @master LiteOutput<MemMock> => @master MemMockMPort);
    bridge_ports!(@auto_configured @master LiteOutput<UartLite> => @master UartLiteMPort);
    bridge_ports!(@auto_configured @master LiteOutput<RFC> => @master RfcMPort);
    bridge_ports!(@auto_configured @master LiteOutput<EventFabric> => @master EventFabricMPort);
    bridge_ports!(@auto_configured @master LiteOutput<GPIO> => @master GpioMPort);
    bridge_ports!(@slave CoreSPort => @auto_configured @slave StatelessTap);
    bridge_ports!(@master StatelessTap => @auto_configured @slave LiteInput<Core>);

    bridge_ports!(@auto_configured @master LiteOutput<BitbandM> => @slave bitband::Bitband);
    bridge_ports!(@master bitband::Bitband => @auto_configured @slave LiteInput<BitbandS>);

    bridge_ports!(@no_m2s @slave StatelessTap => @master StatelessTap);
    impl AHBSlavePortInput for StatelessTap {
        fn on_ahb_input(
            comp: &mut Self::Component,
            ctx: &mut Context,
            mut msg: MasterToSlaveWires<Self::Data>,
        ) {
            // Implement TI's small combinatorial module that makes 0x6000_0000 an unbufferable
            // alias to 0x4000_000.
            if let Some(ref mut meta) = msg.addr_phase.meta.meta_mut()
                && let Some(addr) = soc::is_unbuffered_alias(meta.addr)
            {
                meta.addr = addr;
                meta.prot.is_bufferable = false;
            }
            <Self as AHBMasterPortOutput>::send_ahb_output(comp, ctx, msg);
        }
    }
}
