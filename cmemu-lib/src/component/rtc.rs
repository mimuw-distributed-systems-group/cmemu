pub const RTC_ROUTE_INJECTION: Range<Address> = RTC::ADDR_SPACE;

use crate::bridge_ports;
#[proxy_use]
use crate::common::Address;
#[proxy_use]
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
use crate::common::utils::MaybeMut;
use crate::component::aon_event::AonEvent;
use crate::engine::{CombRegister, SeqRegister};
#[proxy_use]
use crate::engine::{
    Context, DisableableComponent, MainComponent, SkippableClockTreeNode, TickComponent,
    TickComponentExtra,
};
use crate::proxy::{AONEventProxy, RTCBypassProxy, RTCProxy};
use cc2650_constants::AON_RTC as RTC;
use cmemu_common::HwRegister;
use cmemu_proc_macros::{component_impl, handler, proxy_use};
use log::{debug, log_enabled, trace, warn};
use std::cmp::min;
use std::ops::Range;

// TODO: implement correctness and timing tests
// Some adjustment of timing is possible using AON_WUC, not implemented yet.

// [TI-TRM] 14.2.2 - how the counter works
// A counter is 70-bit wide with custom increment, runs on the slow clock.
// Events come from 3 channels, may be delayed and combined here.
//
// The RTC has an MCU-AON interface: the MCU side is clocked with HF, but the AON side with LF.
// The RTC registers are clocked with LF and interact with the MCU-shadow-interface on LF's tick.
// That means, writes by the MCU side are effectively delayed to be consumed later.
// The same goes for reads: MCU has copies which are updated on LF ticks.
// AON_RTC::SYNC addr is used to wait for the propagation (docs say 1-2 LF cycles)
// TODO: implement the MCU-AON interface (rtc_bypass is part of a trick right now)
// The interface should operate with the AHB clock and have effects on the slow clock.
// Maybe we don't even need shadow registers: just using the 'next' part of flops may be enough.

#[derive(MainComponent, TickComponent, TickComponentExtra, DisableableComponent)]
pub(crate) struct RTCComponent {
    #[subcomponent(DriverSC)]
    driver: BusDriver,

    counter: RtcCounter,

    latched_subsec: Option<u32>,

    #[flop]
    subsecinc: SeqRegister<u32>,

    #[flop]
    ctl: SeqRegister<RTC::CTL::Register>,

    #[flop]
    chctl: SeqRegister<RTC::CHCTL::Register>,

    // CombRegister so we can possibly mutate it both in Write and in Update
    #[flop]
    evflags: CombRegister<RTC::EVFLAGS::Register>,

    #[flop]
    ch0cmp: SeqRegister<RTC::CH0CMP::Register>,
    #[flop]
    ch1cmp: SeqRegister<RTC::CH1CMP::Register>,
    #[flop]
    ch2cmp: SeqRegister<RTC::CH2CMP::Register>,

    #[flop]
    ch2cmpinc: SeqRegister<RTC::CH2CMPINC::Register>,
    #[flop]
    ch1capt: SeqRegister<RTC::CH1CAPT::Register>,

    // Right now, we have a mismatch in timings for the registers and need to fully implement
    // the MCU-AON interface as described in [TI-TRM] 14.3 RTC Registers
    any_write_waiting: bool,
}
type BusDriver = SimpleSynchronousSlaveInterface<DriverSC, RTCComponent>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum RtcChannel {
    Ch0,
    Ch1,
    Ch2,
}

#[component_impl(rtc)]
impl RTCComponent {
    pub fn new() -> Self {
        Self {
            driver: Default::default(),

            counter: RtcCounter::new(),
            latched_subsec: None,

            subsecinc: SeqRegister::new(RTC::SUBSECINC::RESET_VALUE),

            ctl: SeqRegister::new(RTC::CTL::Register::new()),
            chctl: SeqRegister::new(RTC::CHCTL::Register::new()),
            evflags: CombRegister::new(RTC::EVFLAGS::Register::new()),
            ch0cmp: SeqRegister::new(RTC::CH0CMP::Register::new()),
            ch1cmp: SeqRegister::new(RTC::CH1CMP::Register::new()),
            ch2cmp: SeqRegister::new(RTC::CH2CMP::Register::new()),

            ch2cmpinc: SeqRegister::new(RTC::CH2CMPINC::Register::new()),
            ch1capt: SeqRegister::new(RTC::CH1CAPT::Register::new()),

            any_write_waiting: false,
        }
    }

    pub fn tick(&mut self, ctx: &mut Context) {
        self.any_write_waiting = false;
        BusDriver::run_driver(self, ctx);

        self.tick_counter(ctx, u128::from(*self.subsecinc));
    }

    pub fn tock(&mut self, ctx: &mut Context) {
        BusDriver::tock(self, ctx);
    }

    // Returns true iff this call caused an interrupt to be raised.
    #[allow(clippy::similar_names)]
    fn tick_counter(&mut self, ctx: &mut Context, counter_delta: u128) -> bool {
        let ctl = self.ctl.bitfields();
        if ctl.EN() == 0 {
            return false;
        }
        let chctl = self.chctl.bitfields();
        let compare_before = self.counter.get_compare_value();
        self.counter.tick(counter_delta);
        let compare_after = self.counter.get_compare_value();
        let compare_not_wrapped = compare_before < compare_after;

        let ch0_event = if chctl.CH0_EN() != 0 {
            let ch0cmp = self.ch0cmp.read();
            if compare_not_wrapped {
                compare_before < ch0cmp && ch0cmp <= compare_after
            } else {
                compare_before < ch0cmp || ch0cmp <= compare_after
            }
        } else {
            false
        };
        let ch1_event = if chctl.CH1_EN() != 0 && chctl.CH1_CAPT_EN() == 0 {
            let ch1cmp = self.ch1cmp.read();
            if compare_not_wrapped {
                compare_before < ch1cmp && ch1cmp <= compare_after
            } else {
                compare_before < ch1cmp || ch1cmp <= compare_after
            }
        } else {
            false
        };
        let ch2_event = if chctl.CH2_EN() != 0 {
            let ch2cmp = self.ch2cmp.read();
            if compare_not_wrapped {
                compare_before < ch2cmp && ch2cmp <= compare_after
            } else {
                compare_before < ch2cmp || ch2cmp <= compare_after
            }
        } else {
            false
        };
        if ch2_event && chctl.CH2_CONT_EN() != 0 {
            self.ch2cmp
                .set_next_mutated_reg(self.ch2cmp.read().wrapping_add(self.ch2cmpinc.read()));
        }

        if ch1_event && self.evflags.bitfields().CH1() != 0 {
            warn!(
                "We allowed RTC interrupt despite bit being set in EVFLAGS.\
                 Write a test for that. Contiki seems to rely on this!"
            );
        }
        self.trigger_events(ctx, (ch0_event, ch1_event, ch2_event))
    }

    // TODO: Should not be called more than once per cycle I guess. This impl allows for that!
    #[allow(clippy::similar_names)]
    fn trigger_events(&mut self, ctx: &mut Context, ch_events: (bool, bool, bool)) -> bool {
        let (ch0_event, ch1_event, ch2_event) = ch_events;
        let ctl = self.ctl.bitfields();

        if (ch0_event || ch1_event || ch2_event) && ctl.EV_DELAY() != 0 {
            unimplemented!("delaying RTC events is unimplemented");
        }

        if ch0_event {
            self.evflags.next_builder().mut_bitfields().set_CH0(1);
            AONEventProxy.notify(ctx, AonEvent::RTC_CH0);
        }
        if ch1_event {
            self.evflags.next_builder().mut_bitfields().set_CH1(1);
            AONEventProxy.notify(ctx, AonEvent::RTC_CH1);
        }
        if ch2_event {
            self.evflags.next_builder().mut_bitfields().set_CH2(1);
            AONEventProxy.notify(ctx, AonEvent::RTC_CH2);
        }

        let combined_event = {
            let comb_ev_mask = u32::from(ctl.COMB_EV_MASK());
            let mut combined_event = false;
            if comb_ev_mask & RTC::CTL::COMB_EV_MASK::E::CH0 != 0 {
                combined_event |= ch0_event;
            }
            if comb_ev_mask & RTC::CTL::COMB_EV_MASK::E::CH1 != 0 {
                combined_event |= ch1_event;
            }
            if comb_ev_mask & RTC::CTL::COMB_EV_MASK::E::CH2 != 0 {
                combined_event |= ch2_event;
            }
            combined_event
        };

        if combined_event {
            trace!("RTC Combined Event: ch0:{ch0_event:?}, ch1:{ch1_event:?}, ch2:{ch2_event:?}");
            AONEventProxy.notify(ctx, AonEvent::RTC_COMB_DLY);
        }
        ch0_event | ch1_event | ch2_event
    }

    #[handler]
    pub fn on_ch1capt_event(&mut self, ctx: &mut Context) {
        // FIXME: this should be delayed to a tick, so it is correctly handled while skipping cycles

        // [TI-TRM] 14.4.1.11 CH1CAPT Register
        // NOTE: This is captured only on pos-edge (matters if the event is held up)
        let chctl = self.chctl.bitfields();
        if chctl.CH1_EN() == 1 && chctl.CH1_CAPT_EN() == 1 {
            // TODO: write tests
            self.ch1capt
                .set_next_mutated_reg(self.counter.get_compare_value());

            // Trigger event of CH1
            self.trigger_events(ctx, (false, true, false));
            if self.ctl.bitfields().EV_DELAY() != 0 {
                unimplemented!("delaying RTC events is unimplemented");
            }
        }
    }

    #[handler]
    pub fn bypass_write(&mut self, ctx: &mut Context, address: Address, mut data: DataBus) {
        let aligned_address = <Self as AlignedHandler>::ALIGN.align_addr(address);
        if data.size() != <Self as AlignedHandler>::ALIGN {
            let fill: DataBus =
                Self::get_data_for_address(MaybeMut::Ref(self), ctx, aligned_address)
                    .unwrap()
                    .into();
            data = fill.emplace_in_aligned(address, data);
        }
        self.set_data_for_address(
            ctx,
            aligned_address,
            <Self as AlignedHandler>::Native::from(data),
        );
    }

    #[handler]
    pub fn request_bypass_read(&mut self, ctx: &mut Context, address: Address, size: Size) {
        let data = Self::get_data_for_address(
            MaybeMut::Mut(self),
            ctx,
            <Self as AlignedHandler>::ALIGN.align_addr(address),
        )
        .unwrap();
        RTCBypassProxy.bypass_read(
            ctx,
            address,
            DataBus::from(data).extract_from_aligned(address, size),
        );
    }

    fn ch1_is_cmp_mode(&self) -> bool {
        let chctl = self.chctl.bitfields();
        chctl.CH1_EN() == 1 && chctl.CH1_CAPT_EN() == 0
    }

    /// How many cycles till the RTC generates any new event? `None` if never/unbounded.
    fn cycles_until_event(&self) -> Option<u64> {
        let chctl = self.chctl.bitfields();
        let ctl = self.ctl.bitfields();

        if ctl.EN() == 0 || (chctl.CH0_EN() == 0 && !self.ch1_is_cmp_mode() && chctl.CH2_EN() == 0)
        {
            return None;
        }

        if log_enabled!(log::Level::Debug) {
            // Gate against side effects
            debug!(
                "cycles_until_interrupt(): CH1CMP value: {:} next: {:?}, current cmp: {:}",
                self.ch1cmp.read(),
                self.ch1cmp.peek_next(),
                self.counter.get_compare_value()
            );
        }

        if self.evflags.bitfields().CH1() != 0 {
            warn!(
                "evflags.CH1 not cleared. Should we generate an event? But Contiki seems to rely on it."
            );
        }

        if self.ctl.bitfields().EV_DELAY() != 0 {
            unimplemented!("EV delay may give additional results in RTC::cycles_until_event");
        }

        // Use next value if written but not yet flopped
        let mut result = u64::MAX;
        if chctl.CH0_EN() == 1 {
            result = min(
                result,
                self.counter.ticks_until(
                    *self.subsecinc,
                    self.ch0cmp.peek_next().unwrap_or(&*self.ch0cmp).read(),
                ),
            );
        }
        if self.ch1_is_cmp_mode() {
            result = min(
                result,
                self.counter.ticks_until(
                    *self.subsecinc,
                    self.ch1cmp.peek_next().unwrap_or(&*self.ch1cmp).read(),
                ),
            );
        }
        if chctl.CH2_EN() == 1 {
            result = min(
                result,
                self.counter.ticks_until(
                    *self.subsecinc,
                    self.ch2cmp.peek_next().unwrap_or(&*self.ch2cmp).read(),
                ),
            );
        }
        Some(result)
    }

    fn check_for_immediate_event(&mut self, ctx: &mut Context, ch: RtcChannel, compare_value: u32) {
        // [TI-TRM] 14.2.3.1 Capture and Compare:
        // "If a compare value is set so that the compare value minus current value is larger than the
        // seconds wrap-around time minus one second (2^32 × SCLK_LFperiod – 1), an immediate
        // compare event is set to avoid losing the event."
        // [TI-TRM] 14.4.1.8 CH1CMP Register
        // "Writing to this register can trigger an immediate*) event in case the
        // new compare value matches a Real Time Clock value from 1 second
        // in the past up till current Real Time Clock value."
        // The event is triggered on the AON slow-clock side:
        // "It can take up to 2 SCLK_LF clock cycles before event occurs due
        // to synchronization."

        // TODO: check for exactness
        if self.counter.get_compare_value().wrapping_sub(compare_value)
            <= RtcCounter::COMPARE_VALUE_ONE_SECOND
        {
            self.trigger_events(
                ctx,
                (
                    ch == RtcChannel::Ch0,
                    ch == RtcChannel::Ch1,
                    ch == RtcChannel::Ch2,
                ),
            );
        }
    }

    #[handler]
    pub fn on_new_ahb_slave_input(
        &mut self,
        ctx: &mut Context,
        msg: MasterToSlaveWires<<RTCComponent as AHBPortConfig>::Data>,
    ) {
        <Self as AHBSlavePortInput>::on_ahb_input(self, ctx, msg);
    }

    fn set_data_for_address(
        &mut self,
        ctx: &mut Context,
        addr: Address,
        data: <RTCComponent as AlignedHandler>::Native,
    ) {
        trace!(
            "RTC write {addr:?}(\"{}\") = {data:x?} [t={:?}, cmp={:?}]",
            ctx.display_named_address(addr),
            ctx.event_queue().get_current_time(),
            self.counter.get_compare_value(),
        );
        match addr {
            RTC::CTL::ADDR => {
                let mut next = *self.ctl;
                next.mutate(data);

                if next.bitfields().RESET() != 0 {
                    self.counter = RtcCounter::new();
                    next.mut_bitfields().set_RESET(0);
                }
                if next.bitfields().RTC_UPD_EN() == 0 {
                    // paranoid! as Contiki sets this up for some reason (it is also needed by the RFC)
                    paranoid!(
                        warn,
                        "RTC_UPD_EN is unimplemented (would kill skipping over LF clock)"
                    );
                }
                debug_assert!(
                    next.bitfields().RTC_4KHZ_EN() == 0,
                    "RTC_4KHZ unimplemented"
                );
                self.ctl.set_next(next);
            }
            RTC::EVFLAGS::ADDR => {
                // EVFlag is Write-1-Clears
                let reg = RTC::EVFLAGS::Register::from(data).bitfields();

                let next = self.evflags.next_builder().mut_bitfields();
                // "Channel 0 event flag, set when CHCTL.CH0_EN = 1 and the RTC
                // value matches or passes the CH0CMP value.
                // Writing 1 clears this flag."
                // TODO: "Note that a new event can not occur on this
                //        channel in first 2 SCLK_LF cycles after a clearance."
                // Whatever it means!
                if reg.CH0() != 0 {
                    next.set_CH0(0);
                }
                if reg.CH1() != 0 {
                    next.set_CH1(0);
                }
                if reg.CH2() != 0 {
                    next.set_CH2(0);
                }
            }
            RTC::SEC::ADDR => {
                self.counter.set_sec(data);
            }
            RTC::SUBSEC::ADDR => {
                self.counter.set_subsec(data);
            }
            RTC::CHCTL::ADDR => {
                self.chctl.set_next_mutated_reg(data);
            }
            RTC::CH0CMP::ADDR => {
                let next = self.ch0cmp.set_next_mutated_reg(data);
                self.check_for_immediate_event(ctx, RtcChannel::Ch0, next.read());
            }
            RTC::CH1CMP::ADDR => {
                let next = self.ch1cmp.set_next_mutated_reg(data);
                self.check_for_immediate_event(ctx, RtcChannel::Ch1, next.read());
            }
            RTC::CH2CMP::ADDR => {
                let next = self.ch2cmp.set_next_mutated_reg(data);
                self.check_for_immediate_event(ctx, RtcChannel::Ch2, next.read());
            }
            RTC::CH2CMPINC::ADDR => {
                self.ch2cmpinc.set_next_mutated_reg(data);
            }
            RTC::CH1CAPT::ADDR => {
                paranoid!(
                    warn,
                    "Writing to a read-only RTC::CH1CAPT register: {data:}"
                );
            }
            RTC::SYNC::ADDR => {
                // This just sets the "any write pending" flag
                // TODO: implement MCU-AON interface
            }
            a => unimplemented!("Requested RTC data write {:x?} for address {:?}", data, a),
        }
        self.any_write_waiting = true;
    }

    // Mutability of MaybeMut may be confusing, but it indicates if side effects are allowed.
    // We need a mut smart pointer to get a mut reference from it.
    // Returns None of pending
    fn get_data_for_address(
        mut this: MaybeMut<Self>,
        ctx: &Context,
        addr: Address,
    ) -> Option<<RTCComponent as AlignedHandler>::Native> {
        let res = match addr {
            RTC::CTL::ADDR => Some(this.ctl.read()),
            RTC::EVFLAGS::ADDR => Some(this.evflags.read()),
            RTC::SEC::ADDR => {
                let sec = this.counter.get_sec();
                let subsec = this.counter.get_subsec();
                if let Some(mut_self) = this.get_mut() {
                    mut_self.latched_subsec = Some(subsec);
                }
                Some(sec)
            }
            RTC::SUBSEC::ADDR => this.get_mut().and_then(|t| t.latched_subsec.take()).or_else(
                || {
                    paranoid!(
                        warn,
                        "You should've read SEC register first. Real hardware's output may be unreliable."
                    );
                    Some(this.counter.get_subsec())
                }
            ),
            RTC::SUBSECINC::ADDR => Some(*this.subsecinc),
            RTC::CHCTL::ADDR => Some(this.chctl.read()),
            RTC::CH0CMP::ADDR => Some(this.ch0cmp.read()),
            RTC::CH1CMP::ADDR => Some(this.ch1cmp.read()),
            RTC::CH2CMP::ADDR => Some(this.ch2cmp.read()),
            RTC::CH2CMPINC::ADDR => Some(this.ch2cmpinc.read()),
            RTC::CH1CAPT::ADDR => Some(this.ch1capt.read()),
            RTC::SYNC::ADDR if this.any_write_waiting && this.get_mut().is_some() => {
                // TODO: wait until there're no outstanding write requests
                None
            }
            // The register always returns 0
            RTC::SYNC::ADDR => Some(0),
            a => unimplemented!(
                "Requested RTC data read for address {:?}: {}",
                a,
                ctx.display_named_address(a)
            ),
        };
        trace!(
            "RTC read {addr:?}(\"{}\") = {res:x?} [t={:?}, cmp={:?}]",
            ctx.display_named_address(addr),
            ctx.event_queue().get_current_time(),
            this.counter.get_compare_value(),
        );
        res
    }
}

bridge_ports!(@slave RTCComponent => @slave BusDriver);

#[component_impl(rtc)]
impl AHBPortConfig for RTCComponent {
    type Data = DataBus;
    type Component = Self;
    const TAG: &'static str = "RTC";
}
#[component_impl(rtc)]
impl AHBSlavePortProxiedInput for RTCComponent {
    fn proxy_ahb_input(ctx: &mut Context, msg: MasterToSlaveWires<Self::Data>) {
        RTCProxy.on_new_ahb_slave_input(ctx, msg);
    }
}

#[component_impl(rtc)]
impl AlignedHandler for RTCComponent {
    const WRITE_MODE: WriteMode = WriteMode::Combinatorial;
    const ALIGN: Size = Size::Word;
    type Native = u32;

    fn read_for_write_filler(
        slave: &Self::Component,
        ctx: &Context,
        address: Address,
    ) -> Self::Native {
        Self::get_data_for_address(MaybeMut::Ref(slave), ctx, address).unwrap()
    }

    fn pre_write(
        _slave: &mut Self::Component,
        _ctx: &mut Context,
        _address: Address,
    ) -> SimpleWriteResponse {
        // TODO: why SUCCESS causes an error?
        SimpleWriteResponse::Pending
    }

    fn read_data(
        slave: &mut Self::Component,
        ctx: &mut Context,
        address: Address,
    ) -> SimpleResponse<Self::Native> {
        let data = Self::get_data_for_address(MaybeMut::Mut(slave), ctx, address);
        if let Some(data) = data {
            SimpleResponse::Success(data)
        } else {
            SimpleResponse::Pending
        }
    }

    fn write_data(
        slave: &mut Self::Component,
        ctx: &mut Context,
        address: Address,
        data: Self::Native,
        post_success: bool,
    ) -> SimpleWriteResponse {
        if post_success {
            slave.set_data_for_address(ctx, address, data);
        }
        SimpleWriteResponse::SUCCESS
    }
}

#[component_impl(rtc)]
impl SkippableClockTreeNode for RTCComponent {
    fn max_cycles_to_skip(
        comp: &mut Self::Component,
        _ctx: &mut Context,
        _parent: Self::IdSpace,
        _extra: &mut Self::Extra,
    ) -> u64 {
        if !comp.can_be_disabled_now() {
            0
        } else if let Some(cycs) = comp.cycles_until_event() {
            // -1 because we don't want to raise the interrupt in skip
            cycs.saturating_sub(1)
        } else {
            u64::MAX
        }
    }

    fn emulate_skipped_cycles(
        comp: &mut Self::Component,
        ctx: &mut Context,
        _parent: Self::IdSpace,
        _extra: &mut Self::Extra,
        skipped_cycles: u64,
    ) {
        let this = comp;
        let pre_compare_value = this.counter.get_compare_value();
        let caused_interrupt = this.tick_counter(
            ctx,
            u128::from(*this.subsecinc) * u128::from(skipped_cycles),
        );
        let post_compare_value = this.counter.get_compare_value();
        trace!(
            "RTC t={:#?} cnt={:?} sleepcycles={skipped_cycles} pre={pre_compare_value} post={post_compare_value} ch1cmp={} caused={caused_interrupt}",
            ctx.event_queue().get_current_time(),
            this.counter,
            this.ch1cmp.read()
        );

        assert!(
            !caused_interrupt,
            "Emulate_skipped_cycles caused an interrupt from RTC! \
            pre_compare_value: {pre_compare_value:x}, CH1CMP: {:x} \
            post_compare_value: {post_compare_value:x}",
            this.ch1cmp.read()
        );
    }
}

/// A 70-bit counter with programmable increment.
///
/// The meaning of the bits is scattered across the chapter [TI-TRM] 14 Real-Time Clock.
/// Essentially, it has a resolution of 2^(-38) seconds ~ 4 picos:
/// - `reg[5:0]` is not readable and only influenced by the programmable increment SUBSECINC
/// - `reg[37:6]` is readable as SUBSEC
/// - `reg[69:38]` is readable as SEC
/// - `reg[53:22]` is used for comparators to trigger events
#[derive(Copy, Clone, Debug)]
struct RtcCounter {
    counter: u128,
}

impl RtcCounter {
    const SEC_SIZE: u8 = 32;
    const SUBSEC_SIZE: u8 = 32;
    const HIDDEN_SUBSEC_SIZE: u8 = 6;
    const COUNTER_SIZE: u8 = Self::SEC_SIZE + Self::SUBSEC_SIZE + Self::HIDDEN_SUBSEC_SIZE;

    const COMPARE_VALUE_HIGH_SEC_MASK: u32 = 0xFFFF_0000;
    const COMPARE_VALUE_SUBSEC_SIZE: u8 = 16;
    const COMPARE_VALUE_SEC_SIZE: u8 = 16;
    const COMPARE_VALUE_ONE_SECOND: u32 = 1 << Self::COMPARE_VALUE_SUBSEC_SIZE;
    const COMPARE_VALUE_SUBSEC_SHIFT: u8 = 16;

    fn new() -> Self {
        Self {
            counter: (u128::from(RTC::SEC::RESET_VALUE)
                << (Self::SUBSEC_SIZE + Self::HIDDEN_SUBSEC_SIZE))
                | (u128::from(RTC::SUBSEC::RESET_VALUE) << Self::HIDDEN_SUBSEC_SIZE),
        }
    }

    #[allow(clippy::cast_possible_truncation)]
    fn get_sec(&self) -> u32 {
        (self.counter >> (Self::SUBSEC_SIZE + Self::HIDDEN_SUBSEC_SIZE)) as u32
    }

    #[allow(clippy::cast_possible_truncation)]
    fn get_subsec(&self) -> u32 {
        (self.counter >> Self::HIDDEN_SUBSEC_SIZE) as u32
    }

    fn get_hidden_subsec(&self) -> u8 {
        (self.counter % (1 << Self::HIDDEN_SUBSEC_SIZE)) as u8
    }

    #[allow(clippy::cast_lossless)]
    fn set(&mut self, sec: u32, subsec: u32, hidden_subsec: u8) {
        self.counter = ((sec as u128) << (Self::SUBSEC_SIZE + Self::HIDDEN_SUBSEC_SIZE))
            | (u128::from(subsec) << Self::HIDDEN_SUBSEC_SIZE)
            | ((hidden_subsec as u128) % (1 << Self::HIDDEN_SUBSEC_SIZE));
    }

    fn set_sec(&mut self, sec: u32) {
        self.set(sec, self.get_subsec(), self.get_hidden_subsec());
    }

    fn set_subsec(&mut self, subsec: u32) {
        self.set(self.get_sec(), subsec, self.get_hidden_subsec());
    }

    // [TI-TRM] 14.4.1.7 CH0CMP Register - mapping from the compare to the counter bits.
    // Note: Special case is when the value is up to 1 second in the past.
    fn get_compare_value(&self) -> u32 {
        let sec = self.get_sec() % (1 << Self::COMPARE_VALUE_SEC_SIZE);
        let subsec = self.get_subsec() >> Self::COMPARE_VALUE_SUBSEC_SHIFT;
        (sec << Self::COMPARE_VALUE_SUBSEC_SIZE) | subsec
    }

    fn tick(&mut self, delta: u128) {
        self.counter += delta;
        // For all frowning at the modulo... the compiler is smart about this, and it is cleaner.
        self.counter %= 1 << Self::COUNTER_SIZE;
    }

    fn ticks_until(&self, delta: u32, compare_value: u32) -> u64 {
        let delta = u128::from(delta);
        let target_counter = self.counter_at_next_compare_value(compare_value);

        let diff = target_counter - self.counter;
        let result = diff.div_ceil(delta);
        result.try_into().unwrap()
    }

    fn counter_at_next_compare_value(&self, compare_value: u32) -> u128 {
        let mut counter: RtcCounter = RtcCounter::new();
        counter.set(
            self.get_sec() & Self::COMPARE_VALUE_HIGH_SEC_MASK
                | (compare_value >> Self::COMPARE_VALUE_SUBSEC_SIZE),
            (compare_value % (1 << Self::COMPARE_VALUE_SUBSEC_SIZE))
                << Self::COMPARE_VALUE_SUBSEC_SHIFT,
            0,
        );
        if counter.counter < self.counter {
            counter.set_sec(
                counter
                    .get_sec()
                    .checked_add(1)
                    .expect("Wrapping RTC is not implemented"),
            );
        }
        counter.counter
    }
}
