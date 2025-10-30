use super::*;
use crate::component::clock_tree::oscillators::ConstOsc;
use std::{iter, mem};
use test_log::test;

const OSC_PS: u64 = 1000;
// Unit tests for graph nodes: each node output is linked to the mock Log struct.
// The tests know some details to set a state of a node without going through the commit sequence.
// All inputs are simulated (we can pass any `parent` we want).
#[derive(Subcomponent)]
struct TestG {
    #[subcomponent(GSC)]
    g: GateNode<GSC>,
    #[subcomponent(DSC)]
    d: DividerNode<DSC>,
    #[subcomponent(SSC)]
    s: SwitchNode<SSC>,
    #[subcomponent(OSC)]
    o: OscillatorNode<OSC, ConstOsc<OSC_PS>>,
    #[subcomponent(Log)]
    log: Log,
    root_power: PowerMode,
}
type G = GateNode<GSC>;
type D = DividerNode<DSC>;
type S = SwitchNode<SSC>;
type O = OscillatorNode<OSC, ConstOsc<OSC_PS>>;

#[derive(Subcomponent)]
#[subcomponent_1to1]
struct Log {
    l: Vec<Ev>,
    allow_mode: PowerMode,
    max_skip: u64,
}
impl Log {
    fn event(comp: &mut <Self as Subcomponent>::Component, ev: Ev) {
        Self::component_to_member_mut(comp).l.push(ev);
    }
}

/// Event with logged parent and argument
#[derive(Debug, Eq, PartialEq)]
enum Ev {
    Tick(Id),
    Tock(Id),
    Prepare(Id, PowerMode),
    SetPower(Id, PowerMode),
    MaxSkip(Id),
    Emulate(Id, u64),
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy, enum_map::Enum)]
enum Id {
    G,
    D,
    S,
    O,
    R, // Root
}

impl EnergyNode for TestG {
    type Extra = ();
    type IdSpace = Id;
    const NAME: &'static str = "<Root>";

    fn id() -> Self::IdSpace {
        Id::R
    }
}

impl EnergyNode for G {
    type Extra = ();
    type IdSpace = Id;
    const NAME: &'static str = "G";

    fn id() -> Self::IdSpace {
        Id::G
    }
}

impl EnergyNode for D {
    type Extra = ();
    type IdSpace = Id;
    const NAME: &'static str = "D";

    fn id() -> Self::IdSpace {
        Id::D
    }
}

impl EnergyNode for S {
    type Extra = ();
    type IdSpace = Id;
    const NAME: &'static str = "S";

    fn id() -> Self::IdSpace {
        Id::S
    }
}

impl SwitchConf for S {
    fn is_valid_parent(_parent: &Self::IdSpace) -> bool {
        true
    }
}

impl EnergyNode for O {
    type Extra = ();
    type IdSpace = Id;
    const NAME: &'static str = "O";

    fn id() -> Self::IdSpace {
        Id::O
    }
}

macro_rules! rpitit_to_log {
    {$ty:ty} => {
impl RPITITNode<TickMapper, ()> for $ty {
    fn map_children(
        comp: &mut Self::Component,
        _ctx: &mut Context,
        _extra: &mut Self::Extra,
        _param: TickMapper,
    ) -> impl Iterator<Item = ()> {
        iter::once(Log::event(comp, Ev::Tick(Self::id())))
    }
}
impl RPITITNode<TockMapper, ()> for $ty {
    fn map_children(
        comp: &mut Self::Component,
        _ctx: &mut Context,
        _extra: &mut Self::Extra,
        _param: TockMapper,
    ) -> impl Iterator<Item = ()> {
        iter::once(Log::event(comp, Ev::Tock(Self::id())))
    }
}
impl RPITITNode<PrepareMapper, PowerMode> for $ty {
    fn map_children(
        comp: &mut Self::Component,
        _ctx: &mut Context,
        _extra: &mut Self::Extra,
        param: PrepareMapper,
    ) -> impl Iterator<Item = PowerMode> {
        iter::once({
            Log::event(comp, Ev::Prepare(Self::id(), param.0));
            Log::get_proxy(comp).allow_mode
        })
    }
}
impl RPITITNode<SetPowerMapper, ()> for $ty {
    fn map_children(
        comp: &mut Self::Component,
        _ctx: &mut Context,
        _extra: &mut Self::Extra,
        param: SetPowerMapper,
    ) -> impl Iterator<Item = ()> {
        iter::once(Log::event(comp, Ev::SetPower(Self::id(), param.0)))
    }
}
impl RPITITNode<MaxSkipMapper, u64> for $ty {
    fn map_children(
        comp: &mut Self::Component,
        _ctx: &mut Context,
        _extra: &mut Self::Extra,
        _param: MaxSkipMapper,
    ) -> impl Iterator<Item = u64> {
        iter::once({
            Log::event(comp, Ev::MaxSkip(Self::id()));
            Log::get_proxy(comp).max_skip
        })
    }
}
impl RPITITNode<EmulateMapper, ()> for $ty {
    fn map_children(
        comp: &mut Self::Component,
        _ctx: &mut Context,
        _extra: &mut Self::Extra,
        param: EmulateMapper,
    ) -> impl Iterator<Item = ()> {
        iter::once(Log::event(comp, Ev::Emulate(Self::id(), param.0)))
    }
}
    };
}
rpitit_to_log!(G);
rpitit_to_log!(D);
rpitit_to_log!(S);
rpitit_to_log!(O);

impl TestG {
    fn new() -> Self {
        Self {
            g: G::new(false),
            d: D::new(1),
            s: S::new(Id::G),
            o: O::new(Default::default(), PowerMode::Active),
            log: Log {
                l: vec![],
                allow_mode: PowerMode::Active,
                max_skip: 0,
            },
            root_power: PowerMode::Active,
        }
    }
}

impl DispatchGetPower for TestG {
    fn dispatch_get_power_state(comp: &TestG, ctx: &Context, node: Id) -> PowerMode {
        let res = match node {
            Id::G => <G as PowerNode>::get_power_state(comp, ctx),
            Id::D => <D as PowerNode>::get_power_state(comp, ctx),
            Id::S => <S as PowerNode>::get_power_state(comp, ctx),
            Id::O => <O as PowerNode>::get_power_state(comp, ctx),
            Id::R => comp.root_power,
        };
        trace!("Someone checks the state of {node:?} -> it is {res:?}");
        res
    }
}

#[test]
fn gate_node() {
    let mut test = TestG::new();
    let mut ctx = Context::new_for_test();
    let unit = &mut ();
    test.g.in_mode = PowerMode::Active;
    assert_eq!(test.g.effective_power_mode(), PowerMode::ClockGated);

    <G as ClockTreeNode>::tick(&mut test, &mut ctx, Id::R, unit);
    <G as ClockTreeNode>::tock(&mut test, &mut ctx, Id::R, unit);
    assert_eq!(test.log.l, vec![]);

    test.g.gate.relays_ticks = true;
    assert_eq!(test.g.effective_power_mode(), PowerMode::Active);
    <G as ClockTreeNode>::tick(&mut test, &mut ctx, Id::R, unit);
    <G as ClockTreeNode>::tock(&mut test, &mut ctx, Id::R, unit);
    assert_eq!(
        mem::take(&mut test.log.l),
        vec![Ev::Tick(Id::G), Ev::Tock(Id::G)]
    );

    let to_skip =
        <G as SkippableClockTreeNode>::max_cycles_to_skip(&mut test, &mut ctx, Id::R, unit);
    assert_eq!(to_skip, test.log.max_skip);
    <G as SkippableClockTreeNode>::emulate_skipped_cycles(
        &mut test,
        &mut ctx,
        Id::R,
        unit,
        to_skip,
    );
    assert_eq!(
        mem::take(&mut test.log.l),
        vec![Ev::MaxSkip(Id::G), Ev::Emulate(Id::G, test.log.max_skip),]
    );

    // Just fake disabling
    assert_eq!(
        <G as PowerNode>::get_power_state(&test, &ctx),
        PowerMode::Active
    );
    assert_eq!(
        <G as PowerNode>::prepare_to_disable(&mut test, &mut ctx, Id::R, unit, PowerMode::Off),
        PowerMode::Active
    );
    assert_eq!(
        mem::take(&mut test.log.l),
        vec![Ev::Prepare(Id::G, PowerMode::Off)]
    );

    // Disable
    <G as ClockTreeNode>::tick(&mut test, &mut ctx, Id::R, unit);
    G::set_next(&mut test, false);
    // No dice until Tock
    assert!(!G::try_apply_next(&mut test, &mut ctx, unit));
    test.log.allow_mode = PowerMode::ClockGated;
    assert!(!G::try_apply_next(&mut test, &mut ctx, unit));
    <G as ClockTreeNode>::tock(&mut test, &mut ctx, Id::R, unit);
    test.log.allow_mode = PowerMode::Active;
    assert!(!G::try_apply_next(&mut test, &mut ctx, unit));
    test.log.allow_mode = PowerMode::ClockGated;
    assert!(G::try_apply_next(&mut test, &mut ctx, unit));
    assert_eq!(
        <G as PowerNode>::get_power_state(&test, &ctx),
        PowerMode::ClockGated
    );
    <G as ClockTreeNode>::tick(&mut test, &mut ctx, Id::R, unit);
    <G as ClockTreeNode>::tock(&mut test, &mut ctx, Id::R, unit);
    assert_eq!(
        mem::take(&mut test.log.l),
        vec![
            Ev::Tick(Id::G),
            Ev::Tock(Id::G),
            Ev::Prepare(Id::G, PowerMode::ClockGated),
            Ev::Prepare(Id::G, PowerMode::ClockGated),
            Ev::SetPower(Id::G, PowerMode::ClockGated)
        ]
    );

    assert_eq!(
        <G as SkippableClockTreeNode>::max_cycles_to_skip(&mut test, &mut ctx, Id::R, unit),
        u64::MAX
    );
    <G as SkippableClockTreeNode>::emulate_skipped_cycles(&mut test, &mut ctx, Id::R, unit, 132);
    assert_eq!(test.log.l, vec![]);
}

#[test]
fn divider_node() {
    let mut test = TestG::new();
    let mut ctx = Context::new_for_test();
    let unit = &mut ();
    test.d.in_mode = PowerMode::Active;
    test.log.max_skip = 13;

    test.d.divider.divider = 1;
    <D as ClockTreeNode>::tick(&mut test, &mut ctx, Id::R, unit);
    <D as ClockTreeNode>::tock(&mut test, &mut ctx, Id::R, unit);
    assert_eq!(
        mem::take(&mut test.log.l),
        vec![Ev::Tick(Id::D), Ev::Tock(Id::D)]
    );

    D::set_next(&mut test, 3);
    <D as ClockTreeNode>::tick(&mut test, &mut ctx, Id::R, unit);
    assert!(!D::try_apply_next(&mut test, &mut ctx, unit));
    <D as ClockTreeNode>::tock(&mut test, &mut ctx, Id::R, unit);
    assert!(D::try_apply_next(&mut test, &mut ctx, unit));
    assert_eq!(
        <D as PowerNode>::get_power_state(&test, &ctx),
        PowerMode::Active
    );
    assert_eq!(
        mem::take(&mut test.log.l),
        vec![Ev::Tick(Id::D), Ev::Tock(Id::D)]
    );

    let to_skip =
        <D as SkippableClockTreeNode>::max_cycles_to_skip(&mut test, &mut ctx, Id::R, unit);
    // 2 till next tick/tock
    assert_eq!(to_skip, 2 + test.log.max_skip * 3);
    <D as SkippableClockTreeNode>::emulate_skipped_cycles(
        &mut test,
        &mut ctx,
        Id::R,
        unit,
        to_skip,
    );
    test.log.max_skip = 0;
    assert_eq!(
        <D as SkippableClockTreeNode>::max_cycles_to_skip(&mut test, &mut ctx, Id::R, unit),
        0
    );
    assert_eq!(
        mem::take(&mut test.log.l),
        vec![
            Ev::MaxSkip(Id::D),
            Ev::Emulate(Id::D, 13),
            Ev::MaxSkip(Id::D),
        ]
    );

    // Disabling
    test.log.allow_mode = PowerMode::Retention;
    assert_eq!(
        <D as PowerNode>::get_power_state(&test, &ctx),
        PowerMode::Active
    );
    <D as ClockTreeNode>::tick(&mut test, &mut ctx, Id::R, unit);
    assert_eq!(
        <D as PowerNode>::prepare_to_disable(&mut test, &mut ctx, Id::R, unit, PowerMode::Off),
        PowerMode::Active
    );
    <D as ClockTreeNode>::tock(&mut test, &mut ctx, Id::R, unit);
    assert_eq!(
        <D as PowerNode>::prepare_to_disable(&mut test, &mut ctx, Id::R, unit, PowerMode::Off),
        PowerMode::Retention
    );
    <D as PowerNode>::set_power_state(&mut test, &mut ctx, Id::R, unit, PowerMode::Retention);
    assert_eq!(
        mem::take(&mut test.log.l),
        vec![
            // Max allowed sleep means next Tick propagates!
            Ev::Tick(Id::D),
            // Prepare is blocked by the divider
            Ev::Tock(Id::D),
            Ev::Prepare(Id::D, PowerMode::Off),
            Ev::SetPower(Id::D, PowerMode::Retention)
        ]
    );

    // Just test the case of skipping a few cycles till ticking
    <D as PowerNode>::set_power_state(&mut test, &mut ctx, Id::R, unit, PowerMode::Active);
    let to_skip =
        <D as SkippableClockTreeNode>::max_cycles_to_skip(&mut test, &mut ctx, Id::R, unit);
    assert_eq!(to_skip, 2);
    <D as SkippableClockTreeNode>::emulate_skipped_cycles(
        &mut test,
        &mut ctx,
        Id::R,
        unit,
        to_skip,
    );
    assert_eq!(
        mem::take(&mut test.log.l),
        vec![Ev::SetPower(Id::D, PowerMode::Active), Ev::MaxSkip(Id::D),]
    );
}

#[test]
fn switch_node() {
    let mut test = TestG::new();
    let mut ctx = Context::new_for_test();
    let unit = &mut ();
    test.s.switch.parent = Id::R;
    test.root_power = PowerMode::Active;

    assert_eq!(
        S::find_effective_pm_of(&test, &ctx, Id::R),
        PowerMode::Active
    );
    assert_eq!(
        S::find_effective_pm_of(&test, &ctx, Id::S),
        PowerMode::Active
    );

    // Tick from a non-parent
    <S as ClockTreeNode>::tick(&mut test, &mut ctx, Id::G, unit);
    <S as ClockTreeNode>::tock(&mut test, &mut ctx, Id::G, unit);
    assert_eq!(
        <S as SkippableClockTreeNode>::max_cycles_to_skip(&mut test, &mut ctx, Id::G, unit),
        u64::MAX
    );
    <S as SkippableClockTreeNode>::emulate_skipped_cycles(
        &mut test,
        &mut ctx,
        Id::G,
        unit,
        u64::MAX,
    );
    assert_eq!(test.log.l, vec![]);

    // Ticks from parent
    <S as ClockTreeNode>::tick(&mut test, &mut ctx, Id::R, unit);
    <S as ClockTreeNode>::tock(&mut test, &mut ctx, Id::R, unit);
    test.log.max_skip = 1;
    assert_eq!(
        <S as SkippableClockTreeNode>::max_cycles_to_skip(&mut test, &mut ctx, Id::R, unit),
        1
    );
    <S as SkippableClockTreeNode>::emulate_skipped_cycles(&mut test, &mut ctx, Id::R, unit, 1);
    assert_eq!(
        mem::take(&mut test.log.l),
        vec![
            Ev::Tick(Id::S),
            Ev::Tock(Id::S),
            Ev::MaxSkip(Id::S),
            Ev::Emulate(Id::S, 1)
        ]
    );

    test.log.allow_mode = PowerMode::Active;
    // State from non-parent
    assert!(<S as PowerNode>::is_active(&test, &ctx));
    assert_eq!(
        <S as PowerNode>::prepare_to_disable(
            &mut test,
            &mut ctx,
            Id::G,
            unit,
            PowerMode::ClockGated
        ),
        PowerMode::ClockGated
    );
    <S as PowerNode>::set_power_state(&mut test, &mut ctx, Id::G, unit, PowerMode::ClockGated);
    assert_eq!(test.log.l, vec![]);

    // State from Parent
    test.log.allow_mode = PowerMode::Off;
    assert_eq!(
        <S as PowerNode>::prepare_to_disable(&mut test, &mut ctx, Id::R, unit, PowerMode::Off),
        PowerMode::Off
    );
    test.root_power = PowerMode::Off;
    <S as PowerNode>::set_power_state(&mut test, &mut ctx, Id::R, unit, PowerMode::Off);
    assert_eq!(
        mem::take(&mut test.log.l),
        vec![
            Ev::Prepare(Id::S, PowerMode::Off),
            Ev::SetPower(Id::S, PowerMode::Off),
        ]
    );

    // no-op
    assert!(S::try_apply_next(&mut test, &mut ctx, unit));
    assert_eq!(test.log.l, vec![]);
    // Switch from inactive to inactive (but other)
    test.g.in_mode = PowerMode::Active;
    test.g.gate.relays_ticks = false;
    assert_eq!(
        S::find_effective_pm_of(&test, &ctx, Id::G),
        PowerMode::ClockGated
    );
    assert!(!<S as PowerNode>::is_active(&test, &ctx));
    S::set_next(&mut test, Id::G);
    assert!(S::try_apply_next(&mut test, &mut ctx, unit));
    assert_eq!(
        mem::take(&mut test.log.l),
        vec![
            Ev::Prepare(Id::S, PowerMode::ClockGated),
            Ev::SetPower(Id::S, PowerMode::ClockGated),
        ]
    );

    // Inactive to active
    test.root_power = PowerMode::Active;
    S::set_next(&mut test, Id::R);
    // FIXME: the logic of prepare_to_disable is mixed, as it wants "closest to wanted" not "minimum"
    test.log.allow_mode = PowerMode::Active;
    assert!(S::try_apply_next(&mut test, &mut ctx, unit));
    assert_eq!(
        mem::take(&mut test.log.l),
        vec![
            Ev::Prepare(Id::S, PowerMode::Active),
            Ev::SetPower(Id::S, PowerMode::Active),
        ]
    );

    // Active to inactive
    test.log.allow_mode = PowerMode::Active;
    S::set_next(&mut test, Id::G);
    assert!(!S::try_apply_next(&mut test, &mut ctx, unit));
    // Now ready to sleep
    test.log.allow_mode = PowerMode::ClockGated;
    assert!(S::try_apply_next(&mut test, &mut ctx, unit));
    assert_eq!(
        mem::take(&mut test.log.l),
        vec![
            Ev::Prepare(Id::S, PowerMode::ClockGated),
            // second attempt
            Ev::Prepare(Id::S, PowerMode::ClockGated),
            Ev::SetPower(Id::S, PowerMode::ClockGated),
        ]
    );

    // Inactive to active, which is between Tick and Tock
    <S as ClockTreeNode>::tick(&mut test, &mut ctx, Id::R, unit);
    test.log.allow_mode = PowerMode::Active;
    S::set_next(&mut test, Id::R);
    assert!(S::try_apply_next(&mut test, &mut ctx, unit));
    // This Tock should be ignored
    <S as ClockTreeNode>::tock(&mut test, &mut ctx, Id::R, unit);
    <S as ClockTreeNode>::tick(&mut test, &mut ctx, Id::R, unit);
    assert_eq!(
        mem::take(&mut test.log.l),
        vec![
            Ev::Prepare(Id::S, PowerMode::Active),
            Ev::SetPower(Id::S, PowerMode::Active),
            Ev::Tick(Id::S),
        ]
    );
    // Active to active in a middle of a Tick
    test.g.in_mode = PowerMode::Active;
    test.g.gate.relays_ticks = true;
    assert_eq!(
        S::find_effective_pm_of(&test, &ctx, Id::G),
        PowerMode::Active
    );
    S::set_next(&mut test, Id::G);
    // not yet paired Tocks from R
    assert!(!S::try_apply_next(&mut test, &mut ctx, unit));
    <S as ClockTreeNode>::tick(&mut test, &mut ctx, Id::G, unit);
    assert!(!S::try_apply_next(&mut test, &mut ctx, unit));
    <S as ClockTreeNode>::tock(&mut test, &mut ctx, Id::G, unit);
    assert!(!S::try_apply_next(&mut test, &mut ctx, unit));
    <S as ClockTreeNode>::tock(&mut test, &mut ctx, Id::R, unit);
    assert!(S::try_apply_next(&mut test, &mut ctx, unit));
    // No power switch in active-active
    assert_eq!(mem::take(&mut test.log.l), vec![Ev::Tock(Id::S),]);
    // but paired Tick-Tock from G
    <S as ClockTreeNode>::tick(&mut test, &mut ctx, Id::G, unit);
    <S as ClockTreeNode>::tock(&mut test, &mut ctx, Id::G, unit);
    assert_eq!(
        mem::take(&mut test.log.l),
        vec![Ev::Tick(Id::S), Ev::Tock(Id::S),]
    );
}

#[test]
fn oscillator_node() {
    let mut test = TestG::new();
    let comp = &mut test;
    // Note: we don't manipulate the time, because the code doesn't need it at the time of writing!
    let mut ctx = Context::new_for_test();
    let ctx = &mut ctx;
    let unit = &mut ();
    comp.o.mode = PowerMode::Active;
    assert_eq!(
        <O as PowerNode>::get_power_state(comp, ctx),
        PowerMode::Active
    );

    // Tick-tocking
    assert_eq!(O::pulse(comp, ctx, unit), Duration::from_picos(1));
    assert_eq!(O::pulse(comp, ctx, unit), Duration::from_picos(OSC_PS - 1));
    assert_eq!(
        mem::take(&mut comp.log.l),
        vec![Ev::Tick(Id::O), Ev::Tock(Id::O)]
    );

    // Skipping
    comp.log.max_skip = 0;
    let to_skip = O::max_time_to_skip(comp, ctx, unit);
    assert_eq!(to_skip, None);

    let c = 3;
    comp.log.max_skip = c;
    let to_skip = O::max_time_to_skip(comp, ctx, unit);
    assert_eq!(to_skip, Some(Duration::from_picos(OSC_PS * c))); // Should be more?

    let next_aligned = O::fast_forward_time(comp, ctx, unit, to_skip.unwrap());
    assert_eq!(next_aligned, Duration::from_picos(OSC_PS)); // Or should be 1 or 0?
    assert_eq!(
        mem::take(&mut comp.log.l),
        vec![
            Ev::MaxSkip(Id::O),
            Ev::MaxSkip(Id::O),
            Ev::Emulate(Id::O, c),
        ]
    );

    // Trying to skip before tock doesn't propagate
    assert_eq!(O::pulse(comp, ctx, unit), Duration::from_picos(1));
    assert_eq!(O::max_time_to_skip(comp, ctx, unit), None);
    assert_eq!(O::pulse(comp, ctx, unit), Duration::from_picos(OSC_PS - 1));
    assert!(O::max_time_to_skip(comp, ctx, unit).is_some());
    // But let's ignore possible skipping
    assert_eq!(O::pulse(comp, ctx, unit), Duration::from_picos(1));
    assert_eq!(O::max_time_to_skip(comp, ctx, unit), None);
    assert_eq!(O::pulse(comp, ctx, unit), Duration::from_picos(OSC_PS - 1));
    assert_eq!(
        mem::take(&mut comp.log.l),
        vec![
            Ev::Tick(Id::O),
            // no MaxSkip here!
            Ev::Tock(Id::O),
            Ev::MaxSkip(Id::O),
            // second time
            Ev::Tick(Id::O),
            // no MaxSkip here!
            Ev::Tock(Id::O),
        ]
    );

    // Just disabling - we still need to specify if prepare-to-disable is propagated on inactive!
    comp.log.allow_mode = PowerMode::Retention;
    assert_eq!(
        <O as PowerNode>::get_power_state(comp, ctx),
        PowerMode::Active
    );
    assert_eq!(
        <O as PowerNode>::prepare_to_disable(comp, ctx, Id::R, unit, PowerMode::Off),
        PowerMode::Retention
    );
    <O as PowerNode>::set_power_state(comp, ctx, Id::R, unit, PowerMode::Retention);
    assert_eq!(
        mem::take(&mut comp.log.l),
        vec![
            Ev::Prepare(Id::O, PowerMode::Off),
            Ev::SetPower(Id::O, PowerMode::Retention)
        ]
    );
}
