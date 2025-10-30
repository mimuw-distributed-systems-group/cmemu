#![allow(clippy::trivially_copy_pass_by_ref)]

use crate::component::clock_tree::oscillators::{Oscillator, PhaseLockedLoop};
use crate::engine::{
    ClockTreeNode, Context, Duration, EnergyNode, PowerMode, PowerNode, PureSubcomponentMarker,
    SkippableClockTreeNode, Subcomponent,
};
use crate::move_state_machine;
use crate::utils::IfExpr;
use enum_map::Enum;
use log::{debug, trace};
use owo_colors::OwoColorize;
use std::any::Any;
use std::cmp::min;
use std::marker::PhantomData;
use strum::IntoStaticStr;

/// Dynamic dispatch to `get_power_state` based on the node `IdSpace`
pub(crate) trait DispatchGetPower: EnergyNode + Subcomponent {
    fn dispatch_get_power_state(
        comp: &<Self as Subcomponent>::Component,
        ctx: &Context,
        node: <Self as EnergyNode>::IdSpace,
    ) -> PowerMode;
}

/// Facility to delegate dispatching methods on children to the generated code.
///
/// *RPITIT* stands for *Return Position Implement Trait In Trait* Rust concept,
/// which allows us to make a catch-all trait yielding an iterator,
/// so we don't need to know how to aggregate individual results in this trait.
///
/// Therefore, this trait allows us to represent the idea of "call this method on all children"
/// with decoupled implementors (here: generated from a built-script).
/// The generic parameters encode information about what should be done in the generated code.
pub(super) trait RPITITNode<Sub, R>: Subcomponent + EnergyNode {
    fn map_children(
        comp: &mut Self::Component,
        ctx: &mut Context,
        extra: &mut Self::Extra,
        param: Sub,
    ) -> impl Iterator<Item = R>;
}
pub(super) struct TickMapper;
pub(super) struct TockMapper;
pub(super) struct MaxSkipMapper;
pub(super) struct EmulateMapper(pub u64);
pub(super) struct PrepareMapper(pub PowerMode);
pub(super) struct SetPowerMapper(pub PowerMode);

#[derive(Debug, PartialEq, Clone, Copy, IntoStaticStr)]
#[strum(serialize_all = "UPPERCASE")]
enum TickKind {
    Tick,
    Tock,
}

#[derive(Debug, Clone)]
pub(crate) struct Divider {
    tick_counter: u32,
    tock_counter: u32,
    divider: u32,
    next_divider: Option<u32>,
    last_tick: TickKind,
}

/// Base divider node.
///
/// Keep in mind, `tick` and `tock` events are primary,
/// so this component is not making one from the another.
/// As a matter of choice is, which events are passed.
/// For better interaction with the overall system, we're passing the neighbouring ones,
/// thus a divider simply selects "for which cycle, the tick-tock pair will pass through".
/// This makes the "#tick == #tocks" invariant **composable**.
///
/// So, for a divider of 2, it is:
/// ```text
/// TICK | TOCK | -    | -    | TICK | ...
/// rather than making Tock from Tick
/// TICK | -    | TOCK | -    | TICK | ...
/// ```
/// and, for a divider of 3:
/// ```text
/// TICK | TOCK | -    | -    | -    | -    | TICK | ...
/// rather than equally-length phases
/// TICK | -    | -    | TOCK | -    | -    | TICK | ...
/// ```
impl Divider {
    pub fn new(divider: u32) -> Self {
        Self {
            divider,
            next_divider: None,
            tick_counter: 0,
            tock_counter: 0,
            last_tick: TickKind::Tock,
        }
    }

    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.tick_counter = 0;
        self.tock_counter = 0;
        self.last_tick = TickKind::Tock;
    }

    pub fn apply_next(&mut self) {
        debug_assert!(
            self.tick_counter == self.tock_counter,
            "Changing divider when number of ticks and tocks isn't equal"
        );

        if let Some(next_divider) = self.next_divider.take() {
            self.divider = next_divider;
            self.tick_counter %= self.divider;
            self.tock_counter %= self.divider;
        }
    }

    pub fn has_next(&self) -> bool {
        self.next_divider.is_some()
    }

    pub fn set_next(&mut self, next_divider: u32) {
        debug_assert!(self.next_divider.is_none());
        self.next_divider = Some(next_divider);
    }

    pub fn is_ready_for_switch(&self) -> bool {
        self.tick_counter == self.tock_counter && self.last_tick == TickKind::Tock
    }

    pub fn tick(&mut self) {
        move_state_machine!(self.last_tick => TickKind::Tock => TickKind::Tick);
        debug_assert!(self.tick_counter == self.tock_counter);
        self.tick_counter = if self.tick_counter + 1 == self.divider {
            0
        } else {
            self.tick_counter.wrapping_add(1)
        };
        self.last_tick = TickKind::Tick;
    }

    pub fn tock(&mut self) {
        move_state_machine!(self.last_tick => TickKind::Tick => TickKind::Tock);
        self.tock_counter = if self.tock_counter + 1 == self.divider {
            0
        } else {
            self.tock_counter.wrapping_add(1)
        };
        debug_assert!(self.tick_counter == self.tock_counter);
        self.last_tick = TickKind::Tock;
    }

    /// Tick till we will generate a sub-tick
    pub fn tick_to_event(&self) -> u32 {
        self.divider - self.tick_counter
    }

    /// Fast forward by `ticks`. Return number of skipped sub-tick-tock pairs.
    #[allow(clippy::cast_possible_truncation, reason = "not possible")]
    pub fn fast_forward_ticks(&mut self, ticks: u64) -> u64 {
        debug_assert!(self.tick_counter == self.tock_counter);
        debug_assert!(self.last_tick == TickKind::Tock);

        let missing = u64::from(self.tick_to_event());
        if ticks >= missing {
            let rem_ticks = ticks - missing;
            let wraps = 1 + rem_ticks / u64::from(self.divider);
            self.tick_counter = (rem_ticks % u64::from(self.divider)) as u32;
            self.tock_counter = self.tick_counter;
            wraps
        } else {
            self.tick_counter += ticks as u32;
            self.tock_counter += ticks as u32;
            0
        }
    }

    pub fn should_tick(&self) -> bool {
        self.tick_counter == 0 && self.last_tick == TickKind::Tick
    }

    pub fn should_tock(&self) -> bool {
        self.tock_counter == 0 && self.last_tick == TickKind::Tock
    }

    #[cfg(test)]
    pub fn should_tick_next_cycle(&self) -> bool {
        debug_assert!(self.last_tick == TickKind::Tick);
        let mut copy = self.clone();
        copy.tock();
        copy.tick();
        copy.should_tick()
    }
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct Gate {
    relays_ticks: bool,
    next_relays_ticks: Option<bool>,
    last_tick: TickKind,
}

impl Gate {
    pub fn new(relays_ticks: bool) -> Self {
        Self {
            relays_ticks,
            next_relays_ticks: None,
            last_tick: TickKind::Tock,
        }
    }

    pub fn set_next(&mut self, next_relays_ticks: bool) {
        // debug_assert!(self.next_relays_ticks.is_none() || self.next_relays_ticks.unwrap() == next_relays_ticks);
        self.next_relays_ticks = Some(next_relays_ticks);
    }

    pub fn apply_next(&mut self) {
        self.relays_ticks = self.next_relays_ticks.take().unwrap_or(self.relays_ticks);
    }

    pub fn has_next(&self) -> bool {
        self.next_relays_ticks.is_some()
    }

    pub fn should_relay(&self) -> bool {
        self.relays_ticks
    }

    pub fn is_ready_for_switch(&self) -> bool {
        self.last_tick == TickKind::Tock
    }

    pub fn tick(&mut self) {
        move_state_machine!(self.last_tick => TickKind::Tock => TickKind::Tick);
    }

    pub fn tock(&mut self) {
        move_state_machine!(self.last_tick => TickKind::Tick => TickKind::Tock);
    }
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct Switch<P: Enum + PartialEq> {
    parent: P,
    next_parent: Option<P>,
    // when switching parents, we need to make sure #ticks == #tocks
    should_ignore_first_tock: bool,
    last_tick: TickKind,
}

impl<P: Enum + PartialEq> Switch<P> {
    pub fn new(parent: P) -> Self {
        Self {
            parent,
            next_parent: None,
            should_ignore_first_tock: false,
            last_tick: TickKind::Tock,
        }
    }

    pub fn set_next(&mut self, next_parent: P) {
        self.next_parent = Some(next_parent);
    }

    pub fn apply_next(&mut self) {
        self.parent = self.next_parent.take().unwrap();
        // Ignore unaligned tock
        self.should_ignore_first_tock = true;
    }

    #[allow(unused)]
    pub fn has_next(&self) -> bool {
        self.next_parent.is_some()
    }

    pub fn is_ready_for_switch(&self) -> bool {
        self.last_tick == TickKind::Tock
    }

    #[inline(always)]
    pub fn check_parent(&self, input: P) -> bool {
        input == self.parent
    }

    #[inline]
    pub fn should_tick(&self, input: P) -> bool {
        self.check_parent(input)
    }

    #[inline]
    pub fn tick(&mut self) {
        move_state_machine!(self.last_tick => TickKind::Tock => TickKind::Tick);
        self.should_ignore_first_tock = false;
    }

    #[inline]
    pub fn should_tock(&self, input: P) -> bool {
        self.check_parent(input) && !self.should_ignore_first_tock
    }

    #[inline]
    pub fn tock(&mut self) {
        move_state_machine!(self.last_tick => TickKind::Tick => TickKind::Tock);
    }
}

#[derive(Subcomponent, Debug)]
pub(super) struct OscillatorNode<SC, Osc>
where
    SC: PureSubcomponentMarker + Subcomponent<Member = Self>,
    Osc: Oscillator + Any,
{
    osc: Osc,
    mode: PowerMode,
    last_tick: TickKind,
    tick_count: u64,
    _phantom_data: PhantomData<SC>,
}
impl<SC, Osc> OscillatorNode<SC, Osc>
where
    SC: PureSubcomponentMarker + Subcomponent<Member = Self>,
    Osc: Oscillator + Any,
{
    pub(crate) fn new(osc: Osc, mode: PowerMode) -> Self {
        Self {
            osc,
            mode,
            last_tick: TickKind::Tock,
            tick_count: 0,
            _phantom_data: PhantomData,
        }
    }

    pub(crate) fn get_ticks(comp: &SC::Component) -> u64 {
        let this = Self::component_to_member(comp);
        this.tick_count
    }

    pub(crate) fn power_on_reset(comp: &mut SC::Component, ctx: &mut Context) {
        let mut this = Self::get_proxy(comp);
        this.osc.power_on_reset(ctx);
    }
}

impl<SC, Osc> OscillatorNode<SC, Osc>
where
    Self: Subcomponent<Member = Self, Component = SC::Component>,
    SC: PureSubcomponentMarker + Subcomponent<Member = Self>,
    Osc: Oscillator + Any,
    Self: EnergyNode,
    Self: RPITITNode<TickMapper, ()> + RPITITNode<TockMapper, ()>,
{
    pub(super) fn pulse(
        comp: &mut SC::Component,
        ctx: &mut Context,
        extra: &mut <Self as EnergyNode>::Extra,
    ) -> Duration {
        let mut this = Self::get_proxy(comp);
        debug_assert!(this.mode.is_active(), "Pulsing a sleeping oscillator");
        let (next_tick, time) = if this.last_tick == TickKind::Tock {
            this.tick_count = this.tick_count.wrapping_add(1);
            (TickKind::Tick, Duration::MIN_DELAY)
        } else {
            (TickKind::Tock, this.osc.pulse(ctx) - Duration::MIN_DELAY)
        };
        this.last_tick = next_tick;

        debug!(target: "cmemu_lib::clock",
            "============ {} {} #{} ================================",
            <&'static str>::from(next_tick), <Self as EnergyNode>::NAME, this.tick_count
        );
        match next_tick {
            TickKind::Tick => {
                <Self as RPITITNode<_, _>>::map_children(comp, ctx, extra, TickMapper)
                    .for_each(drop);
            }
            TickKind::Tock => {
                <Self as RPITITNode<_, _>>::map_children(comp, ctx, extra, TockMapper)
                    .for_each(drop);
            }
        }
        time
    }
}

impl<SC, Osc> OscillatorNode<SC, Osc>
where
    Self: Subcomponent<Member = Self, Component = SC::Component>,
    SC: PureSubcomponentMarker + Subcomponent<Member = Self>,
    Osc: Oscillator + Any,
    Self: EnergyNode,
    Self: RPITITNode<MaxSkipMapper, u64> + RPITITNode<EmulateMapper, ()>,
{
    /// Maximum time to skip. `None` if cannot emulate cycles / need regular pulse.
    pub(super) fn max_time_to_skip(
        comp: &mut SC::Component,
        ctx: &mut Context,
        extra: &mut <Self as EnergyNode>::Extra,
    ) -> Option<Duration> {
        let mut this = Self::get_proxy(comp);
        debug_assert!(this.mode.is_active(), "Asking an off oscillator to skip ");
        if this.last_tick == TickKind::Tick {
            return None;
        }
        let cycles_to_skip = <Self as RPITITNode<_, _>>::map_children(
            this.component_mut(),
            ctx,
            extra,
            MaxSkipMapper,
        )
        .min()
        .unwrap_or(u64::MAX);
        // TODO: add +1 cycle, -1 pico
        (cycles_to_skip > 0).then(|| this.osc.cycles_to_time(ctx, cycles_to_skip))
    }

    pub(super) fn fast_forward_time(
        comp: &mut SC::Component,
        ctx: &mut Context,
        extra: &mut <Self as EnergyNode>::Extra,
        skipped_time: Duration,
    ) -> Duration {
        let mut this = Self::get_proxy(comp);
        debug_assert!(this.mode.is_active(), "Fast-forwarding an off oscillator");
        let (skipped_cycles, next_tick) = this.osc.fast_forward(ctx, skipped_time);
        trace!(
            "OSC {:?} slept {skipped_time:?} -> {skipped_cycles} cyc + {next_tick:?}",
            Self::id().blue()
        );

        this.tick_count = this.tick_count.wrapping_add(skipped_cycles);
        <Self as RPITITNode<_, _>>::map_children(comp, ctx, extra, EmulateMapper(skipped_cycles))
            .for_each(drop);
        next_tick
    }
}

impl<SC> ClockTreeNode for OscillatorNode<SC, PhaseLockedLoop>
where
    Self: Subcomponent<Member = Self, Component = SC::Component>,
    SC: PureSubcomponentMarker + Subcomponent<Member = Self>,
    Self: EnergyNode,
{
    fn tick(
        comp: &mut Self::Component,
        ctx: &mut Context,
        _parent: Self::IdSpace,
        _extra: &mut Self::Extra,
    ) {
        Self::get_proxy(comp).osc.tick(ctx);
    }

    fn tock(
        _comp: &mut Self::Component,
        _ctx: &mut Context,
        _parent: Self::IdSpace,
        _extra: &mut Self::Extra,
    ) {
    }
}

impl<SC, Osc> PowerNode for OscillatorNode<SC, Osc>
where
    Self: Subcomponent<Member = Self, Component = SC::Component>,
    SC: PureSubcomponentMarker + Subcomponent<Member = Self>,
    Osc: Oscillator + Any,
    Self: RPITITNode<PrepareMapper, PowerMode>,
    Self: RPITITNode<SetPowerMapper, ()>,
{
    fn get_power_state(comp: &SC::Component, _ctx: &Context) -> PowerMode {
        Self::component_to_member(comp).mode
    }

    fn prepare_to_disable(
        comp: &mut SC::Component,
        ctx: &mut Context,
        _parent: <Self as EnergyNode>::IdSpace,
        extra: &mut <Self as EnergyNode>::Extra,
        mode: PowerMode,
    ) -> PowerMode {
        if !Self::get_proxy(comp).mode.is_active() {
            return mode;
        }
        <Self as RPITITNode<_, _>>::map_children(comp, ctx, extra, PrepareMapper(mode))
            .max()
            .unwrap_or(mode)
    }

    fn set_power_state(
        comp: &mut SC::Component,
        ctx: &mut Context,
        _parent: <Self as EnergyNode>::IdSpace,
        extra: &mut <Self as EnergyNode>::Extra,
        mode: PowerMode,
    ) {
        let mut this = Self::get_proxy(comp);
        this.mode = mode;
        <Self as RPITITNode<_, _>>::map_children(comp, ctx, extra, SetPowerMapper(mode))
            .for_each(drop);
    }
}

#[derive(Subcomponent, Debug)]
pub(super) struct GateNode<SC>
where
    SC: PureSubcomponentMarker + Subcomponent<Member = Self>,
{
    gate: Gate,
    // do we need that, or is it only debug?
    in_mode: PowerMode,
    _phantom_data: PhantomData<SC>,
}

impl<SC> GateNode<SC>
where
    SC: PureSubcomponentMarker + Subcomponent<Member = Self>,
{
    pub(crate) fn new(initial_passes: bool) -> Self {
        Self {
            gate: Gate::new(initial_passes),
            // TODO: default to Off
            in_mode: PowerMode::Active,
            _phantom_data: PhantomData,
        }
    }

    fn effective_power_mode(&self) -> PowerMode {
        min(self.in_mode, self.gate_as_pm())
    }

    fn gate_as_pm(&self) -> PowerMode {
        self.gate
            .should_relay()
            .ife(PowerMode::Active, PowerMode::ClockGated)
    }

    fn effective_pm_will_change(&self, new_in: PowerMode, new_gate: PowerMode) -> bool {
        self.effective_power_mode() != min(new_in, new_gate)
    }
}

impl<SC> ClockTreeNode for GateNode<SC>
where
    Self: Subcomponent<Member = Self, Component = SC::Component>,
    SC: PureSubcomponentMarker + Subcomponent<Member = Self>,
    Self: EnergyNode,
    Self: RPITITNode<TickMapper, ()> + RPITITNode<TockMapper, ()>,
{
    fn tick(
        comp: &mut Self::Component,
        ctx: &mut Context,
        _parent: Self::IdSpace,
        extra: &mut Self::Extra,
    ) {
        let mut this = Self::get_proxy(comp);
        debug_assert!(this.effective_power_mode().is_active() || !this.gate.should_relay());
        this.gate.tick();
        if this.gate.should_relay() {
            <Self as RPITITNode<_, _>>::map_children(comp, ctx, extra, TickMapper).for_each(drop);
        }
    }

    fn tock(
        comp: &mut Self::Component,
        ctx: &mut Context,
        _parent: Self::IdSpace,
        extra: &mut Self::Extra,
    ) {
        let mut this = Self::get_proxy(comp);
        debug_assert!(this.effective_power_mode().is_active() || !this.gate.should_relay());
        this.gate.tock();
        if this.gate.should_relay() {
            <Self as RPITITNode<_, _>>::map_children(comp, ctx, extra, TockMapper).for_each(drop);
        }
    }
}

impl<SC> SkippableClockTreeNode for GateNode<SC>
where
    Self: Subcomponent<Member = Self, Component = SC::Component>,
    SC: PureSubcomponentMarker + Subcomponent<Member = Self>,
    Self: ClockTreeNode,
    Self: RPITITNode<MaxSkipMapper, u64> + RPITITNode<EmulateMapper, ()>,
{
    fn max_cycles_to_skip(
        comp: &mut Self::Component,
        ctx: &mut Context,
        _parent: Self::IdSpace,
        extra: &mut Self::Extra,
    ) -> u64 {
        let this = Self::get_proxy(comp);
        if this.gate.has_next() {
            0
        } else if this.gate.should_relay() {
            <Self as RPITITNode<_, _>>::map_children(comp, ctx, extra, MaxSkipMapper)
                .min()
                .unwrap_or(u64::MAX)
        } else {
            u64::MAX
        }
    }

    fn emulate_skipped_cycles(
        comp: &mut Self::Component,
        ctx: &mut Context,
        _parent: Self::IdSpace,
        extra: &mut Self::Extra,
        skipped_cycles: u64,
    ) {
        if Self::component_to_member(comp).gate.should_relay() {
            <Self as RPITITNode<_, _>>::map_children(
                comp,
                ctx,
                extra,
                EmulateMapper(skipped_cycles),
            )
            .for_each(drop);
        }
    }
}
impl<SC> GateNode<SC>
where
    Self: Subcomponent<Member = Self, Component = SC::Component>,
    SC: PureSubcomponentMarker + Subcomponent<Member = Self>,
    Self: RPITITNode<PrepareMapper, PowerMode>,
    Self: RPITITNode<SetPowerMapper, ()>,
    Self: PowerNode,
{
    pub(super) fn set_next(comp: &mut <Self as Subcomponent>::Component, pass: bool) {
        let mut this = Self::get_proxy(comp);
        this.gate.set_next(pass);
    }

    pub(super) fn try_apply_next(
        comp: &mut <Self as Subcomponent>::Component,
        ctx: &mut Context,
        extra: &mut <Self as EnergyNode>::Extra,
    ) -> bool {
        let mut this = Self::get_proxy(comp);
        if !this.gate.is_ready_for_switch() {
            return false;
        }
        if let Some(next) = this.gate.next_relays_ticks {
            let next_state = next.ife(PowerMode::Active, PowerMode::ClockGated);
            if this.effective_pm_will_change(this.in_mode, next_state) {
                let possible_next = <Self as RPITITNode<_, _>>::map_children(
                    this.component_mut(),
                    ctx,
                    extra,
                    PrepareMapper(next_state),
                )
                .max()
                .unwrap_or(next_state);
                trace!(
                    "PM:{} tries -> {next:?}: now: {:?}, possible: {possible_next:?}",
                    Self::NAME.blue(),
                    this.effective_power_mode()
                );
                if next.ife(next_state != possible_next, possible_next > next_state) {
                    return false;
                } else {
                    this.gate.apply_next();
                    <Self as RPITITNode<_, _>>::map_children(
                        this.component_mut(),
                        ctx,
                        extra,
                        SetPowerMapper(next_state),
                    )
                    .for_each(drop);
                }
            }
        }
        this.gate.apply_next();
        true
    }
}

impl<SC> PowerNode for GateNode<SC>
where
    Self: Subcomponent<Member = Self, Component = SC::Component>,
    SC: PureSubcomponentMarker + Subcomponent<Member = Self>,
    Self: RPITITNode<PrepareMapper, PowerMode>,
    Self: RPITITNode<SetPowerMapper, ()>,
{
    fn get_power_state(comp: &Self::Component, _ctx: &Context) -> PowerMode {
        Self::component_to_member(comp).effective_power_mode()
    }

    fn prepare_to_disable(
        comp: &mut Self::Component,
        ctx: &mut Context,
        _parent: Self::IdSpace,
        extra: &mut Self::Extra,
        mode: PowerMode,
    ) -> PowerMode {
        if !Self::get_proxy(comp).gate.should_relay() {
            return mode;
        }
        <Self as RPITITNode<_, _>>::map_children(comp, ctx, extra, PrepareMapper(mode))
            .max()
            .unwrap_or(mode)
    }

    fn set_power_state(
        comp: &mut Self::Component,
        ctx: &mut Context,
        _parent: Self::IdSpace,
        extra: &mut Self::Extra,
        mode: PowerMode,
    ) {
        let mut this = Self::get_proxy(comp);
        let run = this.effective_pm_will_change(mode, this.gate_as_pm());
        this.in_mode = mode;
        if run {
            <Self as RPITITNode<_, _>>::map_children(comp, ctx, extra, SetPowerMapper(mode))
                .for_each(drop);
        }
    }
}

#[derive(Subcomponent, Debug)]
pub(super) struct DividerNode<SC>
where
    SC: PureSubcomponentMarker + Subcomponent<Member = Self>,
{
    divider: Divider,
    in_mode: PowerMode,
    _phantom_data: PhantomData<SC>,
}

impl<SC> DividerNode<SC>
where
    Self: Subcomponent<Member = Self, Component = SC::Component>,
    SC: PureSubcomponentMarker + Subcomponent<Member = Self>,
    Self: EnergyNode,
{
    pub(crate) fn new(initial_divider: u32) -> Self {
        Self {
            divider: Divider::new(initial_divider),
            in_mode: PowerMode::Active,
            _phantom_data: PhantomData,
        }
    }

    pub(crate) fn set_next(comp: &mut <Self as Subcomponent>::Component, divider: u32) {
        trace!("DividerNode {}.set_next({})", Self::NAME.blue(), divider);
        Self::get_proxy(comp).divider.set_next(divider);
    }

    pub(super) fn try_apply_next(
        comp: &mut <Self as Subcomponent>::Component,
        _ctx: &mut Context,
        _extra: &mut <Self as EnergyNode>::Extra,
    ) -> bool {
        let mut this = Self::get_proxy(comp);
        let div = &mut this.divider;
        trace!("DIV{} r?{:?}", Self::NAME.blue(), div.is_ready_for_switch());
        if div.is_ready_for_switch() {
            div.apply_next();
            true
        } else {
            div.next_divider.is_none()
        }
    }
}

impl<SC> ClockTreeNode for DividerNode<SC>
where
    Self: Subcomponent<Member = Self, Component = SC::Component>,
    SC: PureSubcomponentMarker + Subcomponent<Member = Self>,
    Self: EnergyNode,
    Self: RPITITNode<TickMapper, ()> + RPITITNode<TockMapper, ()> + Subcomponent,
{
    fn tick(
        comp: &mut Self::Component,
        ctx: &mut Context,
        _parent: Self::IdSpace,
        extra: &mut Self::Extra,
    ) {
        let mut this = Self::get_proxy(comp);
        this.divider.tick();
        if this.divider.should_tick() {
            // trace!("DIV{} GO!", Self::NAME.blue());
            <Self as RPITITNode<_, _>>::map_children(comp, ctx, extra, TickMapper).for_each(drop);
        }
    }

    fn tock(
        comp: &mut Self::Component,
        ctx: &mut Context,
        _parent: Self::IdSpace,
        extra: &mut Self::Extra,
    ) {
        let mut this = Self::get_proxy(comp);
        this.divider.tock();
        if this.divider.should_tock() {
            <Self as RPITITNode<_, _>>::map_children(comp, ctx, extra, TockMapper).for_each(drop);
        }
    }
}

impl<SC> SkippableClockTreeNode for DividerNode<SC>
where
    Self: Subcomponent<Member = Self, Component = SC::Component>,
    SC: PureSubcomponentMarker + Subcomponent<Member = Self>,
    Self: ClockTreeNode,
    Self: RPITITNode<MaxSkipMapper, u64> + RPITITNode<EmulateMapper, ()>,
{
    // sub-calls can go in the middle of stuff, or we're going to be limited, let's see
    fn max_cycles_to_skip(
        comp: &mut Self::Component,
        ctx: &mut Context,
        _parent: Self::IdSpace,
        extra: &mut Self::Extra,
    ) -> u64 {
        let this = Self::get_proxy(comp);
        let div = u64::from(this.divider.divider);
        /* Just after the tock -  */
        let major_skip = if !this.divider.should_tick() && !this.divider.has_next() {
            // FIXME: cache this value
            let min_children =
                <Self as RPITITNode<_, _>>::map_children(comp, ctx, extra, MaxSkipMapper)
                    .min()
                    .unwrap_or(u64::MAX);
            min_children.saturating_mul(div)
        } else {
            0
        };
        let our_ticks_to_skip = {
            let this = Self::get_proxy(comp);
            // Now, we either call this in a random cycle phase, or skip till alignment,
            // but alignment may be hard for multiple co-prime numbers...
            u64::from(this.divider.tick_to_event().saturating_sub(1))
        };
        our_ticks_to_skip.saturating_add(major_skip)
    }

    fn emulate_skipped_cycles(
        comp: &mut Self::Component,
        ctx: &mut Context,
        _parent: Self::IdSpace,
        extra: &mut Self::Extra,
        skipped_cycles: u64,
    ) {
        // Either we did a major skip, or only a partial one.
        // We do tocks in the same cycle as ticks, so we are safe against partial skips.
        let sub = Self::get_proxy(comp)
            .divider
            .fast_forward_ticks(skipped_cycles);
        if sub > 0 {
            <Self as RPITITNode<_, _>>::map_children(comp, ctx, extra, EmulateMapper(sub))
                .for_each(drop);
        }
    }
}

impl<SC> PowerNode for DividerNode<SC>
where
    Self: Subcomponent<Member = Self, Component = SC::Component>,
    SC: PureSubcomponentMarker + Subcomponent<Member = Self>,
    Self: RPITITNode<PrepareMapper, PowerMode>,
    Self: RPITITNode<SetPowerMapper, ()>,
{
    fn get_power_state(comp: &Self::Component, _ctx: &Context) -> PowerMode {
        Self::component_to_member(comp).in_mode
    }

    fn prepare_to_disable(
        comp: &mut Self::Component,
        ctx: &mut Context,
        _parent: Self::IdSpace,
        extra: &mut Self::Extra,
        mode: PowerMode,
    ) -> PowerMode {
        let div = &mut Self::get_proxy(comp).divider;
        trace!(
            "PM:{} -> {mode:?} r?{:?}",
            Self::NAME.blue(),
            div.is_ready_for_switch()
        );
        if !div.is_ready_for_switch() {
            return PowerMode::Active;
        }
        <Self as RPITITNode<_, _>>::map_children(comp, ctx, extra, PrepareMapper(mode))
            .max()
            .unwrap_or(mode)
    }

    fn set_power_state(
        comp: &mut Self::Component,
        ctx: &mut Context,
        _parent: Self::IdSpace,
        extra: &mut Self::Extra,
        mode: PowerMode,
    ) {
        Self::component_to_member_mut(comp).in_mode = mode;
        <Self as RPITITNode<_, _>>::map_children(comp, ctx, extra, SetPowerMapper(mode))
            .for_each(drop);
    }
}

pub(super) trait SwitchConf: EnergyNode {
    fn is_valid_parent(parent: &Self::IdSpace) -> bool;
}

#[derive(Subcomponent)]
pub(super) struct SwitchNode<SC>
where
    SC: PureSubcomponentMarker + Subcomponent<Member = Self>,
    Self: EnergyNode,
{
    switch: Switch<<Self as EnergyNode>::IdSpace>,
    _phantom_data: PhantomData<SC>,
}

impl<SC> SwitchNode<SC>
where
    SC: PureSubcomponentMarker + Subcomponent<Member = Self>,
    Self: EnergyNode,
{
    pub(crate) fn new(initial_parent: <SwitchNode<SC> as EnergyNode>::IdSpace) -> Self {
        Self {
            switch: Switch::new(initial_parent),
            _phantom_data: PhantomData,
        }
    }

    pub(crate) fn get_parent(
        comp: &<Self as Subcomponent>::Component,
    ) -> <Self as EnergyNode>::IdSpace {
        Self::component_to_member(comp).switch.parent
    }
}

impl<SC> SwitchNode<SC>
where
    Self: Subcomponent<Member = Self, Component = SC::Component>,
    SC: PureSubcomponentMarker + Subcomponent<Member = Self>,
    Self: EnergyNode,
    SC::Component: DispatchGetPower,
    SC::Component: EnergyNode<IdSpace = <Self as EnergyNode>::IdSpace>,
    SC::Component: Subcomponent<Component = SC::Component>,
{
    fn find_effective_pm_of(
        comp: &<Self as Subcomponent>::Component,
        ctx: &Context,
        new_route: <Self as EnergyNode>::IdSpace,
    ) -> PowerMode {
        <SC::Component as DispatchGetPower>::dispatch_get_power_state(comp, ctx, new_route)
    }
}

impl<SC> SwitchNode<SC>
where
    Self: Subcomponent<Member = Self, Component = SC::Component>,
    SC: PureSubcomponentMarker + Subcomponent<Member = Self>,
    Self: RPITITNode<PrepareMapper, PowerMode>,
    Self: RPITITNode<SetPowerMapper, ()>,
    Self: SwitchConf,
    SC::Component: DispatchGetPower,
    SC::Component: EnergyNode<IdSpace = <Self as EnergyNode>::IdSpace>,
    SC::Component: Subcomponent<Component = SC::Component>,
{
    pub(crate) fn set_next(
        comp: &mut <Self as Subcomponent>::Component,
        new_route: <Self as EnergyNode>::IdSpace,
    ) {
        trace!("Switch {}.set_next({:?})", Self::NAME.blue(), new_route);
        assert!(
            Self::is_valid_parent(&new_route),
            "{new_route:?} is not valid parent of {:?}",
            <Self as EnergyNode>::NAME
        );
        let mut this = Self::get_proxy(comp);
        if new_route != this.switch.parent {
            this.switch.set_next(new_route);
        }
    }

    pub(super) fn try_apply_next(
        comp: &mut <Self as Subcomponent>::Component,
        ctx: &mut Context,
        extra: &mut <Self as EnergyNode>::Extra,
    ) -> bool {
        let mut this = Self::get_proxy(comp);
        if !this.switch.is_ready_for_switch() {
            // Makes sure #ticks == #tocks
            return false;
        }
        let Some(new_route) = this.switch.next_parent else {
            return true;
        };
        let parent = this.switch.parent;
        let new_route_pm = Self::find_effective_pm_of(this.component(), ctx, new_route);
        let old_route_pm = Self::find_effective_pm_of(this.component(), ctx, parent);
        if new_route_pm == old_route_pm {
            this.switch.apply_next();
            return true;
        }
        let possible_next = <Self as RPITITNode<_, _>>::map_children(
            this.component_mut(),
            ctx,
            extra,
            PrepareMapper(new_route_pm),
        )
        .max()
        .unwrap_or(new_route_pm);

        trace!(
            "PM:{} switch {parent:?} -> {new_route:?}: now: {old_route_pm:?}, new: {new_route_pm:?}, possible: {possible_next:?}",
            Self::NAME.blue(),
        );
        if new_route_pm
            .is_active()
            .ife(new_route_pm != possible_next, possible_next > new_route_pm)
        {
            false
        } else {
            this.switch.apply_next();
            <Self as RPITITNode<_, _>>::map_children(
                this.component_mut(),
                ctx,
                extra,
                SetPowerMapper(new_route_pm),
            )
            .for_each(drop);
            true
        }
    }
}

impl<SC> ClockTreeNode for SwitchNode<SC>
where
    Self: Subcomponent<Member = Self, Component = SC::Component>,
    SC: PureSubcomponentMarker + Subcomponent<Member = Self>,
    Self: EnergyNode,
    Self: RPITITNode<TickMapper, ()> + RPITITNode<TockMapper, ()> + Subcomponent,
{
    fn tick(
        comp: &mut Self::Component,
        ctx: &mut Context,
        parent: Self::IdSpace,
        extra: &mut Self::Extra,
    ) {
        let mut this = Self::get_proxy(comp);
        if this.switch.should_tick(parent) {
            this.switch.tick();
            <Self as RPITITNode<_, _>>::map_children(comp, ctx, extra, TickMapper).for_each(drop);
        }
    }

    fn tock(
        comp: &mut Self::Component,
        ctx: &mut Context,
        parent: Self::IdSpace,
        extra: &mut Self::Extra,
    ) {
        let mut this = Self::get_proxy(comp);
        if this.switch.should_tock(parent) {
            this.switch.tock();
            <Self as RPITITNode<_, _>>::map_children(comp, ctx, extra, TockMapper).for_each(drop);
        }
    }
}

impl<SC> SkippableClockTreeNode for SwitchNode<SC>
where
    Self: Subcomponent<Member = Self, Component = SC::Component>,
    SC: PureSubcomponentMarker + Subcomponent<Member = Self>,
    Self: ClockTreeNode,
    Self: RPITITNode<MaxSkipMapper, u64> + RPITITNode<EmulateMapper, ()>,
{
    fn max_cycles_to_skip(
        comp: &mut Self::Component,
        ctx: &mut Context,
        parent: Self::IdSpace,
        extra: &mut Self::Extra,
    ) -> u64 {
        let this = Self::get_proxy(comp);

        if this
            .switch
            .next_parent
            .is_some_and(|n| n == parent || this.switch.parent == parent)
        {
            0
        } else if this.switch.check_parent(parent) {
            <Self as RPITITNode<_, _>>::map_children(comp, ctx, extra, MaxSkipMapper)
                .min()
                .unwrap_or(u64::MAX)
        } else {
            u64::MAX
        }
    }

    fn emulate_skipped_cycles(
        comp: &mut Self::Component,
        ctx: &mut Context,
        parent: Self::IdSpace,
        extra: &mut Self::Extra,
        skipped_cycles: u64,
    ) {
        let this = Self::get_proxy(comp);
        if this.switch.check_parent(parent) {
            <Self as RPITITNode<_, _>>::map_children(
                comp,
                ctx,
                extra,
                EmulateMapper(skipped_cycles),
            )
            .for_each(drop);
        }
    }
}

impl<SC> PowerNode for SwitchNode<SC>
where
    Self: Subcomponent<Member = Self, Component = SC::Component>,
    SC: PureSubcomponentMarker + Subcomponent<Member = Self>,
    Self: RPITITNode<PrepareMapper, PowerMode>,
    Self: RPITITNode<SetPowerMapper, ()>,
    SC::Component: DispatchGetPower,
    SC::Component: EnergyNode<IdSpace = <Self as EnergyNode>::IdSpace>,
    SC::Component: Subcomponent<Component = SC::Component>,
{
    fn get_power_state(comp: &Self::Component, ctx: &Context) -> PowerMode {
        let parent = Self::component_to_member(comp).switch.parent;
        Self::find_effective_pm_of(comp, ctx, parent)
    }

    fn prepare_to_disable(
        comp: &mut Self::Component,
        ctx: &mut Context,
        parent: Self::IdSpace,
        extra: &mut Self::Extra,
        mode: PowerMode,
    ) -> PowerMode {
        if !Self::get_proxy(comp).switch.check_parent(parent) {
            return mode;
        }
        <Self as RPITITNode<_, _>>::map_children(comp, ctx, extra, PrepareMapper(mode))
            .max()
            .unwrap_or(mode)
    }
    fn set_power_state(
        comp: &mut Self::Component,
        ctx: &mut Context,
        parent: Self::IdSpace,
        extra: &mut Self::Extra,
        mode: PowerMode,
    ) {
        if Self::get_proxy(comp).switch.check_parent(parent) {
            <Self as RPITITNode<_, _>>::map_children(comp, ctx, extra, SetPowerMapper(mode))
                .for_each(drop);
        }
    }
}

#[cfg(test)]
mod graph_tests;

#[cfg(test)]
mod tests {

    use super::Divider;

    // TODO: test for case 1 => 2 => 1 => 2 and/or set() after tick() but not tock().

    #[test]
    #[allow(clippy::bool_assert_comparison)]
    fn should_tick_and_should_tock() {
        let mut divider = Divider::new(2);

        for _ in 0..2 {
            // It's ignored - we first tick()/tock() and only then check should_tick()/should_tock().
            assert_eq!(divider.should_tick(), false);
            assert_eq!(divider.should_tock(), true);

            divider.tick();
            assert_eq!(divider.should_tick(), false);
            assert_eq!(divider.should_tock(), false);
            divider.tock();
            assert_eq!(divider.should_tick(), false);
            assert_eq!(divider.should_tock(), false);
            // Thus we don't tick on the whole first cycle and both tick and tock on the second.
            divider.tick();
            assert_eq!(divider.should_tick(), true);
            assert_eq!(divider.should_tock(), false);
            divider.tock();
            assert_eq!(divider.should_tick(), false);
            assert_eq!(divider.should_tock(), true);
        }
    }

    #[test]
    #[allow(clippy::similar_names)]
    fn divider_ticks_in_nth_cycle() {
        const TEST_CASES: usize = 6;

        let mut divider = Divider::new(3);

        let expected_next_ticks: [bool; TEST_CASES] = [false, true, false, false, true, false];
        let expected_this_ticks: [bool; TEST_CASES] = [false, false, true, false, false, true];
        let expected_this_tocks: [bool; TEST_CASES] = [false, false, true, false, false, true];

        for i in 0..TEST_CASES {
            divider.tick();
            let next_tick = divider.should_tick_next_cycle();
            let this_tick = divider.should_tick();
            divider.tock();
            let this_tock = divider.should_tock();

            assert_eq!(next_tick, expected_next_ticks[i]);
            assert_eq!(this_tick, expected_this_ticks[i]);
            assert_eq!(this_tock, expected_this_tocks[i]);
        }
    }
}
