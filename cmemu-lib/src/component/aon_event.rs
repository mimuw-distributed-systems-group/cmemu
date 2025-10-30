pub const AON_EVENT_ROUTE_INJECTION: Range<Address> = AON_EVENT::ADDR_SPACE;
use crate::bridge_ports;
use crate::common::new_ahb::databus::DataBus;
use crate::common::new_ahb::ports::AHBSlavePortProxiedInput;
#[proxy_use]
use crate::common::new_ahb::ports::{AHBPortConfig, AHBSlavePortInput};
#[proxy_use]
use crate::common::new_ahb::signals::{MasterToSlaveWires, Size};
use crate::common::new_ahb::slave_driver::stateless_simplifiers::AlignedHandler;
use crate::common::new_ahb::slave_driver::{
    SimpleResponse, SimpleSynchronousSlaveInterface, SimpleWriteResponse, WriteMode,
};
#[proxy_use(proxy_only)]
use crate::component::aon_event::AonEvent;
use crate::component::event_fabric::EventFabricEvent;
use crate::component::wuc::WUCWakeupEvent;
use crate::engine::SeqRegister;
#[proxy_use]
use crate::engine::{
    Context, DisableableComponent, MainComponent, SkippableClockTreeNode, Subcomponent,
    TickComponent, TickComponentExtra,
};
use crate::proxy::{AONEventProxy, EventFabricProxy, RTCProxy, WUCProxy};
use cc2650_constants::AON_EVENT;
use cmemu_common::{Address, HwRegister};
use cmemu_proc_macros::{component_impl, handler, proxy_use};
use log::debug;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::ops::Range;

#[derive(
    MainComponent, SkippableClockTreeNode, TickComponent, TickComponentExtra, DisableableComponent,
)]
#[skippable_if_disableable]
pub(crate) struct AONEventComponent {
    #[subcomponent(DriverSC)]
    driver: BusDriver,

    #[flop]
    mcuwusel: SeqRegister<AON_EVENT::MCUWUSEL::Register>,
    #[flop]
    auxwusel: SeqRegister<AON_EVENT::AUXWUSEL::Register>,
    #[flop]
    evtomcusel: SeqRegister<AON_EVENT::EVTOMCUSEL::Register>,
    #[flop]
    rtcsel: SeqRegister<AON_EVENT::RTCSEL::Register>,
}
type BusDriver = SimpleSynchronousSlaveInterface<DriverSC, AONEventComponent>;

#[component_impl(aon_event)]
impl AONEventComponent {
    pub fn new() -> Self {
        Self {
            driver: Default::default(),
            mcuwusel: SeqRegister::new(AON_EVENT::MCUWUSEL::Register::new()),
            auxwusel: SeqRegister::new(AON_EVENT::AUXWUSEL::Register::new()),
            evtomcusel: SeqRegister::new(AON_EVENT::EVTOMCUSEL::Register::new()),
            rtcsel: SeqRegister::new(AON_EVENT::RTCSEL::Register::new()),
        }
    }

    pub fn tick(&mut self, ctx: &mut Context) {
        BusDriver::run_driver(self, ctx);
    }

    pub fn tock(&mut self, ctx: &mut Context) {
        BusDriver::tock(self, ctx);
    }

    #[handler]
    pub fn on_new_ahb_slave_input(
        &mut self,
        ctx: &mut Context,
        msg: MasterToSlaveWires<<AONEventComponent as AHBPortConfig>::Data>,
    ) {
        <Self as AHBSlavePortInput>::on_ahb_input(self, ctx, msg);
    }

    fn set_data_for_address(
        &mut self,
        addr: Address,
        data: <AONEventComponent as AlignedHandler>::Native,
        ctx: &mut Context,
    ) {
        fn valid_event(raw: u8) -> bool {
            AonEvent::try_from(raw).is_ok()
        }

        debug!("aonevent write: {:?} {:?}", addr, data);
        match addr {
            AON_EVENT::MCUWUSEL::ADDR => {
                let next = self.mcuwusel.set_next_mutated_reg(data).bitfields();
                debug_assert!(
                    valid_event(next.WU0_EV())
                        && valid_event(next.WU1_EV())
                        && valid_event(next.WU2_EV())
                        && valid_event(next.WU3_EV())
                );
            }
            AON_EVENT::AUXWUSEL::ADDR => {
                let next = self.auxwusel.set_next_mutated_reg(data).bitfields();
                debug_assert!(
                    valid_event(next.WU0_EV())
                        && valid_event(next.WU1_EV())
                        && valid_event(next.WU2_EV())
                );
            }
            AON_EVENT::EVTOMCUSEL::ADDR => {
                let next = self.evtomcusel.set_next_mutated_reg(data).bitfields();
                debug_assert!(
                    valid_event(next.AON_PROG0_EV())
                        && valid_event(next.AON_PROG1_EV())
                        && valid_event(next.AON_PROG2_EV())
                );
            }
            AON_EVENT::RTCSEL::ADDR => {
                let next = self.rtcsel.set_next_mutated_reg(data).bitfields();
                debug_assert!(valid_event(next.RTC_CH1_CAPT_EV()));
            }
            a => unimplemented!(
                "Requested AON_EVENT data write {:?} for address {:?}: {}",
                data,
                a,
                ctx.display_named_address(a),
            ),
        }
    }

    fn get_data_for_address(
        &self,
        addr: Address,
        ctx: &Context,
    ) -> <AONEventComponent as AlignedHandler>::Native {
        debug!("aonevent read: {:?}", addr);
        match addr {
            AON_EVENT::MCUWUSEL::ADDR => self.mcuwusel.read(),
            AON_EVENT::AUXWUSEL::ADDR => self.auxwusel.read(),
            AON_EVENT::EVTOMCUSEL::ADDR => self.evtomcusel.read(),
            AON_EVENT::RTCSEL::ADDR => self.rtcsel.read(),
            a => unimplemented!(
                "Requested AON_EVENT data read for address {:?}: {}",
                a,
                ctx.display_named_address(a)
            ),
        }
    }
}

bridge_ports!(@slave AONEventComponent => @slave BusDriver);

#[component_impl(aon_event)]
impl AHBPortConfig for AONEventComponent {
    type Data = DataBus;
    type Component = Self;
    const TAG: &'static str = "AON_EVENT";
}
#[component_impl(aon_event)]
impl AHBSlavePortProxiedInput for AONEventComponent {
    fn proxy_ahb_input(ctx: &mut Context, msg: MasterToSlaveWires<Self::Data>) {
        AONEventProxy.on_new_ahb_slave_input(ctx, msg);
    }
}

#[component_impl(aon_event)]
impl AlignedHandler for AONEventComponent {
    const WRITE_MODE: WriteMode = WriteMode::Combinatorial;
    const ALIGN: Size = Size::Word;
    type Native = u32;

    fn read_for_write_filler(
        slave: &Self::Component,
        ctx: &Context,
        address: Address,
    ) -> Self::Native {
        slave.get_data_for_address(address, ctx)
    }

    fn read_data(
        slave: &mut Self::Component,
        ctx: &mut Context,
        address: Address,
    ) -> SimpleResponse<Self::Native> {
        let this = Self::component_to_member_mut(slave);
        SimpleResponse::Success(this.get_data_for_address(address, ctx))
    }

    fn pre_write(
        _slave: &mut Self::Component,
        _ctx: &mut Context,
        _address: Address,
    ) -> SimpleWriteResponse {
        SimpleWriteResponse::SUCCESS
    }

    fn write_data(
        slave: &mut Self::Component,
        ctx: &mut Context,
        address: Address,
        data: Self::Native,
        post_success: bool,
    ) -> SimpleWriteResponse {
        if post_success {
            slave.set_data_for_address(address, data, ctx);
        }
        SimpleWriteResponse::SUCCESS
    }
}

#[component_impl(aon_event)]
impl AONEventComponent {
    #[handler]
    pub(crate) fn notify(&mut self, ctx: &mut Context, event: AonEvent) {
        debug!("AON_EVENT: {event:?}");
        // It is simpler to compare raw values
        let raw: u8 = event.into();
        let mcuwusel = self.mcuwusel.bitfields();
        if mcuwusel.WU0_EV() == raw
            || mcuwusel.WU1_EV() == raw
            || mcuwusel.WU2_EV() == raw
            || mcuwusel.WU3_EV() == raw
        {
            WUCProxy.on_wake_up_event(ctx, WUCWakeupEvent::MCU);
        }
        let auxwusel = self.auxwusel.bitfields();
        if auxwusel.WU0_EV() == raw || auxwusel.WU1_EV() == raw || auxwusel.WU2_EV() == raw {
            WUCProxy.on_wake_up_event(ctx, WUCWakeupEvent::AUX);
        }
        // Constant not-configured (see [TI-TRM] 4.4.2.1 Wake-Up Controller (WUC))
        if event == AonEvent::JTAG {
            WUCProxy.on_wake_up_event(ctx, WUCWakeupEvent::JTAG);
        }

        // AON to EventFabric
        // [TI-TRM] 4.4.2.3 MCU Event Fabric
        // "Seven output events from the AON event fabric are routed as inputs to the MCU event fabric."
        let evtomcusel = self.evtomcusel.bitfields();
        if evtomcusel.AON_PROG0_EV() == raw {
            EventFabricProxy.notify(ctx, EventFabricEvent::AON_PROG0);
        } // note: no else
        if evtomcusel.AON_PROG1_EV() == raw {
            EventFabricProxy.notify(ctx, EventFabricEvent::AON_PROG1);
        } // note: no else
        if evtomcusel.AON_PROG2_EV() == raw {
            EventFabricProxy.notify(ctx, EventFabricEvent::AON_PROG2);
        }
        match event {
            AonEvent::PAD => EventFabricProxy.notify(ctx, EventFabricEvent::AON_GPIO_EDGE),
            AonEvent::AUX_SWEV0 => EventFabricProxy.notify(ctx, EventFabricEvent::AON_AUX_SWEV0),
            AonEvent::RTC_COMB_DLY => EventFabricProxy.notify(ctx, EventFabricEvent::AON_RTC_COMB),
            // This is wroooooong to support this way!
            AonEvent::RTC_UPD => EventFabricProxy.notify(ctx, EventFabricEvent::AON_RTC_UPD),
            _ => None,
        };

        let rtcsel = self.rtcsel.bitfields();
        if rtcsel.RTC_CH1_CAPT_EV() == raw {
            RTCProxy.on_ch1capt_event(ctx);
        }
    }
}

#[allow(non_camel_case_types, dead_code)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub(crate) enum AonEvent {
    /// No event, always low
    NONE = 63,
    /// Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to `AUX_COMPB` which is synchronized in AUX
    AUX_COMPB_ASYNC_N = 56,
    /// Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to `AUX_COMPB` which is synchronized in AUX
    AUX_COMPB_ASYNC = 55,
    /// BATMON voltage update event
    BATMON_VOLT = 54,
    /// BATMON temperature update event
    BATMON_TEMP = 53,
    /// AUX Timer 1 Event
    AUX_TIMER1_EV = 52,
    /// AUX Timer 0 Event
    AUX_TIMER0_EV = 51,
    /// TDC completed or timed out
    AUX_TDC_DONE = 50,
    /// ADC conversion completed
    AUX_ADC_DONE = 49,
    /// Comparator B triggered
    AUX_COMPB = 48,
    /// Comparator A triggered
    AUX_COMPA = 47,
    /// AUX Software triggered event #2. Triggered by `AUX_EVCTL:SWEVSET.SWEV2`
    AUX_SWEV2 = 46,
    /// AUX Software triggered event #1. Triggered by `AUX_EVCTL:SWEVSET.SWEV1`
    AUX_SWEV1 = 45,
    /// AUX Software triggered event #0. Triggered by `AUX_EVCTL:SWEVSET.SWEV0`
    AUX_SWEV0 = 44,
    /// JTAG generated event
    JTAG = 43,
    /// RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)
    RTC_UPD = 42,
    /// RTC combined delayed event
    RTC_COMB_DLY = 41,
    /// RTC channel 2 - delayed event
    RTC_CH2_DLY = 40,
    /// RTC channel 1 - delayed event
    RTC_CH1_DLY = 39,
    /// RTC channel 0 - delayed event
    RTC_CH0_DLY = 38,
    /// RTC channel 2 event
    RTC_CH2 = 37,
    /// RTC channel 1 event
    RTC_CH1 = 36,
    /// RTC channel 0 event
    RTC_CH0 = 35,
    /// Edge detect on any PAD
    PAD = 32,
    /// Edge detect on PAD31
    PAD31 = 31,
    /// Edge detect on PAD30
    PAD30 = 30,
    /// Edge detect on PAD29
    PAD29 = 29,
    /// Edge detect on PAD28
    PAD28 = 28,
    /// Edge detect on PAD27
    PAD27 = 27,
    /// Edge detect on PAD26
    PAD26 = 26,
    /// Edge detect on PAD25
    PAD25 = 25,
    /// Edge detect on PAD24
    PAD24 = 24,
    /// Edge detect on PAD23
    PAD23 = 23,
    /// Edge detect on PAD22
    PAD22 = 22,
    /// Edge detect on PAD21
    PAD21 = 21,
    /// Edge detect on PAD20
    PAD20 = 20,
    /// Edge detect on PAD19
    PAD19 = 19,
    /// Edge detect on PAD18
    PAD18 = 18,
    /// Edge detect on PAD17
    PAD17 = 17,
    /// Edge detect on PAD16
    PAD16 = 16,
    /// Edge detect on PAD15
    PAD15 = 15,
    /// Edge detect on PAD14
    PAD14 = 14,
    /// Edge detect on PAD13
    PAD13 = 13,
    /// Edge detect on PAD12
    PAD12 = 12,
    /// Edge detect on PAD11
    PAD11 = 11,
    /// Edge detect on PAD10
    PAD10 = 10,
    /// Edge detect on PAD9
    PAD9 = 9,
    /// Edge detect on PAD8
    PAD8 = 8,
    /// Edge detect on PAD7
    PAD7 = 7,
    /// Edge detect on PAD6
    PAD6 = 6,
    /// Edge detect on PAD5
    PAD5 = 5,
    /// Edge detect on PAD4
    PAD4 = 4,
    /// Edge detect on PAD3
    PAD3 = 3,
    /// Edge detect on PAD2
    PAD2 = 2,
    /// Edge detect on PAD1
    PAD1 = 1,
    /// Edge detect on PAD0
    PAD0 = 0,
}
