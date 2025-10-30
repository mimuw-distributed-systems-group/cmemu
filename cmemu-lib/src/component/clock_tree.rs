#![allow(clippy::items_after_statements, clippy::wildcard_imports)]
use crate::build_data::{
    self, ClockTreeNodes, EnergyEntity, Oscillators, collect, dispatch, for_each,
};
use crate::common::utils::{SubcomponentProxyMut, iter_enum};
use crate::component::Components;
use crate::engine::{
    Context, Duration, EnergyNode, EventRevokeToken, PowerMode, PowerNode, Subcomponent, Timepoint,
};
use crate::proxy::{ClockTreeProxy, OSCProxy, PRCMProxy};
use crate::utils::{IfExpr, dife};
use enum_map::{EnumMap, enum_map};
use log::{debug, info, trace, warn};
use owo_colors::OwoColorize;
use std::ops::RangeInclusive;

pub mod nodes;
use nodes::DispatchGetPower;
mod oscillators;

mod graph {
    #![allow(non_camel_case_types)]
    #![allow(dead_code, unused_braces, unused_qualifications)]
    #![allow(clippy::absolute_paths, clippy::pedantic)]
    // A generated graph of nodes by the build script. There should be no special logic there.
    include!(concat!(env!("OUT_DIR"), "/clock_graph.rs"));
}

use graph::Nodes;

#[cfg(test)]
#[test]
fn macros_example() {
    macro_rules! generate {
        ($path:path) => {
            dbg!(stringify!($path));
        };
        ({ $(#[$attr:meta])? $comp:path }) => {
            dbg!(stringify!($path));
        };
    }
    for_each! {build_data::components call generate;}
    let ex_osc = PowerClockManager::default_clock();
    use crate::build_data::Oscillators::*;
    dispatch!(build_data::oscillators => osc = ex_osc => EnergyEntity::Oscillator(osc));
}

#[derive(Subcomponent)]
pub(crate) struct PowerClockManager {
    #[subcomponent(Nodes)]
    nodes: Nodes,

    nodes_wait_change: EnumMap<ClockTreeNodes, bool>,
    // A cached version of doing `any(nodes_wait_change)`
    any_node_waits_change: bool,

    /// Manages when something should happen
    #[subcomponent(TicksScheduler)]
    scheduler: TicksScheduler,
}

impl PowerClockManager {
    pub(crate) fn new() -> Self {
        Self {
            nodes: Nodes::new(),
            nodes_wait_change: enum_map!(_ => false),
            any_node_waits_change: false,

            scheduler: TicksScheduler::new(),
        }
    }

    // FIXME: this should come from the config file!
    fn default_clock() -> Oscillators {
        Oscillators::RC48M
    }

    // #[handler]
    pub(crate) fn power_on_reset(&mut self, ctx: &mut Context, components: &mut Components) {
        self.start_oscillator(ctx, components, Self::default_clock());
        self.power_on_reset_inner(ctx, components);
    }
}

///////////////////////////////////////////////////////
// External API: handlers code                       //
///////////////////////////////////////////////////////

#[derive(Clone, Debug)]
pub(crate) struct ClockTreeState {
    pub(crate) fast_clock_source: Oscillators,
    pub(crate) slow_clock_source: Oscillators,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub(crate) enum ClockTreeStateQuerent {
    OSC,
}

// NOTE: All the handlers have a commented #[handler] here, because this file is not automatically
// parsed right now.
// The handlers are manually listed in cmemu-codegen under `produce_clock_tree_component_desc`,
// as they require some hacks to pass special arguments (variable delay or Components).
impl PowerClockManager {
    // #[handler]
    pub(crate) fn want_node_state(
        &mut self,
        ctx: &mut Context,
        node: ClockTreeNodes,
        power: PowerMode,
    ) {
        self.want_gate_node_state(ctx, node, power.is_active());
    }

    // #[handler]
    pub(crate) fn query_state(&mut self, ctx: &mut Context, querent: ClockTreeStateQuerent) {
        // TODO: last bastion of non-generic code! (referencing SclkHf)
        info!("query_state({:?})", querent);
        // Right now, we have a fixed divider linked to SCLK_HF - we need to support
        // multiple clocks per component for the SyncDownBridge
        let EnergyEntity::Oscillator(div_parent) = graph::SclkHf::get_parent(self) else {
            panic!("invalid sclk_hf parent!")
        };
        let response = ClockTreeState {
            fast_clock_source: div_parent,
            slow_clock_source: div_parent,
        };

        match querent {
            ClockTreeStateQuerent::OSC => {
                OSCProxy.on_clock_tree_state_response(ctx, response);
            }
        }
    }

    // #[handler]
    pub(crate) fn want_switch_parent(
        &mut self,
        ctx: &mut Context,
        node: ClockTreeNodes,
        parent: EnergyEntity,
    ) {
        self.want_switch_node_parent(ctx, node, parent);
    }

    // #[handler]
    pub(crate) fn want_divider_scale(
        &mut self,
        ctx: &mut Context,
        node: ClockTreeNodes,
        divisor: u32,
    ) {
        self.want_divider_node_value(ctx, node, divisor);
    }

    // #[handler]
    pub(crate) fn start_oscillator(
        &mut self,
        ctx: &mut Context,
        components: &mut Components,
        osc: Oscillators,
    ) {
        info!("start_oscillator({:?})", osc);
        if self
            .get_direct_energy_state(ctx, EnergyEntity::Oscillator(osc))
            .is_active()
        {
            warn!(
                "Oscillator {osc:?} is already active, it should not be requested again by the managing component!"
            );
            return;
        }
        TicksScheduler::request_enable_osc(self, ctx, components, osc);
    }

    // #[handler]
    pub(crate) fn stop_oscillator(
        &mut self,
        ctx: &mut Context,
        components: &mut Components,
        osc: Oscillators,
    ) {
        info!("stop_oscillator({:?})", osc);
        if !self
            .get_direct_energy_state(ctx, EnergyEntity::Oscillator(osc))
            .is_active()
        {
            warn!(
                "Oscillator {osc:?} is already inactive, it should not be requested again by the managing component!"
            );
            return;
        }
        TicksScheduler::request_disable_osc(self, ctx, components, osc);
    }

    // #[handler]
    pub(crate) fn start_sleep(&mut self, ctx: &mut Context) {
        info!(
            "start_sleep: PCM will start asking for skipping cycles (cycle {})",
            ctx.cycle_no()
        );

        self.scheduler.sleeping_enabled = true;
    }

    // #[handler]
    pub(crate) fn stop_sleep(&mut self, ctx: &mut Context) {
        info!(
            "stop sleep: CT will no longer ask for skipping cycles (cycle {})",
            ctx.cycle_no()
        );

        self.scheduler.sleeping_enabled = false;
    }

    // Handlers for internal use (do not call them from the external code!)

    // #[handler]
    pub(crate) fn external_wake_up(&mut self, ctx: &mut Context, components: &mut Components) {
        info!("WAKE UP CALLED at cycle {}", ctx.cycle_no());
        TicksScheduler::wakeup_skipping_oscillators(
            &mut Self::get_proxy(self).as_proxy(),
            ctx,
            components,
        );
    }

    // #[handler]
    pub(crate) fn wakeup_skipping_osc(
        &mut self,
        ctx: &mut Context,
        components: &mut Components,
        osc: Oscillators,
    ) {
        TicksScheduler::wakeup_skipping_osc(self, ctx, components, osc);
    }

    // #[handler]
    pub(crate) fn pulse(
        &mut self,
        ctx: &mut Context,
        components: &mut Components,
        osc: Oscillators,
    ) {
        {
            // This block contains all the actions we need to delay such as changing state of gates or dividers.
            // We don't delay setting new state of gates, we make it two-phase - the commit is now,
            // We need to make sure #ticks == #tocks, but since it is not exposed to this part of the framework,
            // (as it per-oscillator anyway), all nodes make sure they implement the switch correctly.
            self.nodes_try_apply_next(ctx, components);
        }
        {
            // FIXME: this should be probably managed by a component, not here! and not hardcoded!
            if osc == Self::default_clock() {
                ctx.set_cycle_no(graph::RC48M::get_ticks(self).wrapping_add(1));
            }
        }

        TicksScheduler::pulse_oscillator(self, ctx, components, osc);
    }
}

///////////////////////////////////////////////////////
// Internal code for managing the state of the graph //
///////////////////////////////////////////////////////

impl PowerClockManager {
    fn power_on_reset_inner(&mut self, ctx: &mut Context, components: &mut Components) {
        // TODO: reset all components, propagate energy state

        // Initial state for oscillators
        macro_rules! generator {
            ($osc:ident) => {
                graph::$osc::power_on_reset(self, ctx);
                ctx.set_energy_state_of(
                    EnergyEntity::Oscillator(Oscillators::$osc),
                    <graph::$osc as PowerNode>::get_power_state(self, ctx),
                );
            };
        }
        for_each!(build_data::oscillators call generator;);

        // Publish initial power state of internal nodes
        // FIXME: propagate state!
        macro_rules! generator {
            ($node:ident) => {
                ctx.set_energy_state_of(
                    EnergyEntity::ClockTree(ClockTreeNodes::$node),
                    <graph::$node as PowerNode>::get_power_state(self, ctx),
                );
            };
        }
        for_each!(build_data::clock_tree_nodes call generator;);

        // Initial state of components
        // TODO: call actual reset, etc.
        macro_rules! generator {
            ({ $(#[$attr:meta])? $comp:path }) => {
                $(#[$attr])?
                {
                let comp = <Components as AsMut<$comp>>::as_mut(components);
                // This is not needed, but shows the macro structure is complete
                ctx.set_energy_state_of(
                    <$comp as EnergyNode>::id(),
                    <$comp as PowerNode>::get_power_state(comp, ctx),
                );
                }
            };
        }
        for_each!(build_data::components call generator;);
    }

    fn notify_change_is_live(&self, ctx: &mut Context, node: ClockTreeNodes) {
        // TODO: There should be some info about actual change to prevent loops
        PRCMProxy.on_clock_gate_states_loaded(ctx, node);
    }

    fn nodes_try_apply_next(&mut self, ctx: &mut Context, comps: &mut Components) -> bool {
        if !self.any_node_waits_change {
            return true;
        }

        // Try to visit nodes in post-order, as it is more likely to accept disabling changes.
        // We iterate over all nodes here, but this should not be called too often
        // (hence the noisy `info!` log).
        let mut all_done = true;
        macro_rules! generator {
            ($node:ident) => {
                if self.nodes_wait_change[ClockTreeNodes::$node] {
                    if graph::$node::try_apply_next(self, ctx, comps) {
                        self.nodes_wait_change[ClockTreeNodes::$node] = false;
                        // Make sure power state is up to date
                        ctx.set_energy_state_of(
                            EnergyEntity::ClockTree(ClockTreeNodes::$node),
                            <graph::$node as PowerNode>::get_power_state(self, ctx),
                        );
                        self.notify_change_is_live(ctx, ClockTreeNodes::$node);
                    } else {
                        all_done = false;
                    }
                }
            };
        }
        for_each!(build_data::clock_tree_nodes_postorder call generator;);

        info!(
            "PM: All nodes state applied? {}",
            dife(all_done, "yes".green(), "no".red())
        );
        // TODO: can set_apply_next create a new waiting change? then this will be incorrect
        self.any_node_waits_change = !all_done;
        all_done
    }

    fn want_gate_node_state(&mut self, ctx: &mut Context, node: ClockTreeNodes, pass: bool) {
        debug!("PM Want gate node state {node:?} to pass: {pass:?}");

        // Publish lower state to Context as it should be pessimistic
        let entity = EnergyEntity::ClockTree(node);
        let cur_power = <Self as DispatchGetPower>::dispatch_get_power_state(self, ctx, entity);
        if cur_power > PowerMode::ClockGated && !pass {
            ctx.set_energy_state_of(entity, PowerMode::ClockGated);
        }

        // TODO: we may want to notify that no actual change was done
        // Template by calling a generator macro from a macro-passed list
        macro_rules! generator {
            ($id:ident, $path:ident) => {{
                graph::$path::set_next(self, pass);
                self.nodes_wait_change[$id] = true;
            }};
        }
        use crate::build_data::ClockTreeNodes::*;
        // Just Gate types: we should consider a better naming scheme
        dispatch!(build_data::clock_tree_nodes_Gate => n = node => @generator; _ => panic!("not a gate"));
        self.any_node_waits_change = true;
    }

    fn want_switch_node_parent(
        &mut self,
        _ctx: &mut Context,
        node: ClockTreeNodes,
        parent: EnergyEntity,
    ) {
        debug!("CT Want switch node state {node:?} to have parent: {parent:?}");

        macro_rules! generator {
            ($id:ident, $path:ident) => {{
                graph::$path::set_next(self, parent);
                self.nodes_wait_change[$id] = true;
            }};
        }
        use crate::build_data::ClockTreeNodes::*;
        dispatch!(build_data::clock_tree_nodes_Switch => n = node => @generator; _ => panic!("not a switch"));
        self.any_node_waits_change = true;
    }

    fn want_divider_node_value(&mut self, _ctx: &mut Context, node: ClockTreeNodes, divisor: u32) {
        debug!("CT Want divider node state {node:?} to have divisor: {divisor:?}");

        macro_rules! generator {
            ($id:ident, $path:ident) => {{
                graph::$path::set_next(self, divisor);
                self.nodes_wait_change[$id] = true;
            }};
        }
        use crate::build_data::ClockTreeNodes::*;
        dispatch!(build_data::clock_tree_nodes_Divider => n = node => @generator; _ => panic!("not a divider"));
        self.any_node_waits_change = true;
    }

    fn get_direct_energy_state(&self, ctx: &Context, node: EnergyEntity) -> PowerMode {
        <Self as DispatchGetPower>::dispatch_get_power_state(self, ctx, node)
    }
}

impl EnergyNode for PowerClockManager {
    type Extra = ();
    type IdSpace = EnergyEntity;
    const NAME: &'static str = "<PM root>";

    fn id() -> Self::IdSpace {
        EnergyEntity::Component(build_data::Components::PowerClockManager)
    }
}

impl DispatchGetPower for PowerClockManager {
    fn dispatch_get_power_state(comp: &Self, ctx: &Context, node: EnergyEntity) -> PowerMode {
        // Template by calling a generator macro from a macro-passed list
        macro_rules! generator {
            ($id:ident, $path:ident) => {
                <graph::$path as PowerNode>::get_power_state(comp, ctx)
            };
        }

        match node {
            // Well, the true value is in CTX
            EnergyEntity::Component(_) => ctx.get_energy_state_of(node),
            EnergyEntity::ClockTree(node) => {
                use crate::build_data::ClockTreeNodes::*;
                dispatch!(build_data::clock_tree_nodes => _n = node => @generator)
            }
            EnergyEntity::Oscillator(node) => {
                use crate::build_data::Oscillators::*;
                dispatch!(build_data::oscillators => _n = node => @generator)
            }
        }
    }
}

///////////////////////////////////////////////////////
// Management of the oscillators                     //
///////////////////////////////////////////////////////

/// A scheduler managing when oscillators need to be pulsed/skipped.
///
/// Each oscillator can be either of two modes:
/// - *Normal*: meaning it has an exact next pulse time
/// - *Skipping*: meaning it has specified an upper bound for the next *Normal* pulse
///
/// Oscillators in *Skipping* mode act as if thy were waiting on a conditional variable,
/// checking if the managed components need any processing.
/// Right now, we make any regular pulse event resulting in full node processing act as a `wake_all` call.
#[derive(Subcomponent)]
#[subcomponent_1to1]
struct TicksScheduler {
    skipping_oscs: EnumMap<Oscillators, Option<(RangeInclusive<Timepoint>, EventRevokeToken)>>,
    should_be_disabled: EnumMap<Oscillators, bool>,
    /// If sleeping is enabled, we will try calling code in `SkippableClockTreeNode`.
    /// In theory, it should be always valid, but this should save a lot of time when the
    /// main CPU is enabled.
    sleeping_enabled: bool,
}

impl TicksScheduler {
    fn new() -> Self {
        Self {
            skipping_oscs: enum_map! {_ => None},
            should_be_disabled: enum_map! {_ => false},
            sleeping_enabled: false,
        }
    }
}

// Main impl of the scheduling logic
impl TicksScheduler {
    // TODO: consider pre-dispatching by moving `osc` into the type

    /// An event aligned to the oscillator pulse time. Main logic lies here.
    pub(super) fn pulse_oscillator(
        comp: &mut <Self as Subcomponent>::Component,
        ctx: &mut Context,
        components: &mut Components,
        osc: Oscillators,
    ) {
        let mut this = Self::get_proxy(comp);
        trace!(
            "{} pulsing {osc:?} at {:?}",
            "Oscillator".blue(),
            ctx.event_queue().get_current_time()
        );
        debug_assert!(this.skipping_oscs[osc].is_none());
        if let Some(upper_bound) = Self::can_skip(&mut this, ctx, components, osc) {
            Self::put_on_wakelist(&mut this, ctx, components, osc, upper_bound);
        } else {
            let delay = Self::make_osc_tick(&mut this, ctx, components, osc);
            // TODO: what should be the other? should we fast-forward here?
            //       NOTE: future event_queue is not stable (preserving insertion order)!
            Self::wakeup_skipping_oscillators(&mut this, ctx, components);
            // Put it later in case wee land on the same timestamp, so waked-up oscs have time
            // to process flops.
            ClockTreeProxy.pulse(ctx, delay, osc);
            // trace!("Pulsing {osc:?} done, next: {delay:?}");
        }
    }

    /// An event generated to make sure upper-bound skip is seen
    pub(super) fn wakeup_skipping_osc(
        comp: &mut <Self as Subcomponent>::Component,
        ctx: &mut Context,
        components: &mut Components,
        osc: Oscillators,
    ) {
        Self::wakeup_osc(&mut Self::get_proxy(comp), ctx, components, osc, true);
    }

    /// Nodes will return ZERO (None) when then cannot fast-forward time
    fn can_skip(
        this: &mut SubcomponentProxyMut<'_, Self>,
        ctx: &mut Context,
        components: &mut Components,
        osc: Oscillators,
    ) -> Option<Duration> {
        if !this.sleeping_enabled && !this.should_be_disabled[osc] {
            return None;
        }

        macro_rules! generator {
            ($id:ident, $path:ident) => {
                graph::$path::max_time_to_skip(this.component_mut(), ctx, components)
            };
        }
        use crate::build_data::Oscillators::*;
        let max_skip = dispatch!(build_data::oscillators => _o = osc => @generator);
        // TODO: should we have a minimum skip time?
        debug_assert!(
            this.should_be_disabled[osc]
                .implies(max_skip.is_some_and(|x| x > Duration::LOOKS_SATURATED))
        );
        max_skip
    }

    /// Normal mode: get next tick time
    fn make_osc_tick(
        this: &mut SubcomponentProxyMut<'_, Self>,
        ctx: &mut Context,
        components: &mut Components,
        osc: Oscillators,
    ) -> Duration {
        macro_rules! generator {
            ($id:ident, $path:ident) => {
                graph::$path::pulse(this.component_mut(), ctx, components)
            };
        }
        use crate::build_data::Oscillators::*;
        dispatch!(build_data::oscillators => _o = osc => @generator)
    }

    /// Skipping mode wakeup - call fast-forward on the oscillator!
    fn wakeup_osc(
        this: &mut SubcomponentProxyMut<'_, Self>,
        ctx: &mut Context,
        components: &mut Components,
        osc: Oscillators,
        // Use to ignore old scheduled wakeups
        from_event: bool,
    ) {
        let now = ctx.event_queue().get_current_time();
        trace!(
            "Attempt to wake {osc:?} at {now:?} ev:{from_event:?}, skip: {:?}",
            this.skipping_oscs[osc]
        );
        let Some((range, token)) = this.skipping_oscs[osc].take() else {
            return;
        };
        if !from_event {
            token.revoke(ctx);
        } else {
            debug_assert_eq!(now, *range.end());
        }
        let skipped_time = now.wrapping_sub_timepoint(*range.start());

        macro_rules! generator {
            ($id:ident, $path:ident) => {
                graph::$path::fast_forward_time(this.component_mut(), ctx, components, skipped_time)
            };
        }
        use crate::build_data::Oscillators::*;
        let next_normal = dispatch!(build_data::oscillators => _o = osc => @generator);
        debug!(
            "{} {osc:?} wake at {now:#?} post {skipped_time:#?}, tick in {next_normal:#?}",
            "Oscillator".blue()
        );
        ClockTreeProxy.pulse(ctx, next_normal, osc);
    }

    fn wakeup_skipping_oscillators(
        this: &mut SubcomponentProxyMut<'_, Self>,
        ctx: &mut Context,
        components: &mut Components,
    ) {
        for osc in iter_enum::<Oscillators>() {
            if this.skipping_oscs[osc].is_some() {
                Self::wakeup_osc(this, ctx, components, osc, false);
            }
        }
    }

    /// Skipping mode wakeup - call fast-forward on the oscillator!
    fn put_on_wakelist(
        this: &mut SubcomponentProxyMut<'_, Self>,
        ctx: &mut Context,
        components: &mut Components,
        osc: Oscillators,
        upper_bound: Duration,
    ) {
        let now = ctx.event_queue().get_current_time();
        if this.should_be_disabled[osc] {
            debug!(
                "{} {osc:?} is disabled at {now:#?} and stop participating in scheduling",
                "Oscillator".blue()
            );
            Self::inner_set_osc_power_state(
                this.component_mut(),
                ctx,
                components,
                osc,
                PowerMode::ClockGated,
            );
            return;
        }

        let wakeup_time = now.wrapping_add_duration(upper_bound);
        debug!(
            "{} {osc:?} will sleep up to {upper_bound:?} (until {wakeup_time:#?})",
            "Oscillator".blue()
        );
        let token = ClockTreeProxy
            .wakeup_skipping_osc(ctx, upper_bound, osc)
            .unwrap();
        this.skipping_oscs[osc] = Some((now..=wakeup_time, token));
    }

    fn inner_set_osc_power_state(
        comp: &mut <Self as Subcomponent>::Component,
        ctx: &mut Context,
        components: &mut Components,
        osc: Oscillators,
        mode: PowerMode,
    ) {
        let mut this = Self::get_proxy(comp);
        let parent = <PowerClockManager as EnergyNode>::id();
        macro_rules! generator {
            ($id:ident, $path:ident) => {{
                assert!(
                    <graph::$path as PowerNode>::prepare_to_disable(
                        this.component_mut(),
                        ctx,
                        parent,
                        components,
                        mode
                    ) == mode
                );
                <graph::$path as PowerNode>::set_power_state(
                    this.component_mut(),
                    ctx,
                    parent,
                    components,
                    mode,
                );
                ctx.set_energy_state_of(EnergyEntity::Oscillator($id), mode);
            }};
        }
        use crate::build_data::Oscillators::*;
        dispatch!(build_data::oscillators => o = osc => @generator);
    }

    fn request_disable_osc(
        comp: &mut <Self as Subcomponent>::Component,
        ctx: &mut Context,
        components: &mut Components,
        osc: Oscillators,
    ) {
        let mut this = Self::get_proxy(comp);

        this.should_be_disabled[osc] = true;
        // It is already sleeping, our work is done!
        if let Some((_sleep, wakeup)) = this.skipping_oscs[osc].take() {
            wakeup.revoke(ctx);
            let mode = PowerMode::ClockGated;
            TicksScheduler::inner_set_osc_power_state(comp, ctx, components, osc, mode);
        }
    }

    fn request_enable_osc(
        comp: &mut <Self as Subcomponent>::Component,
        ctx: &mut Context,
        components: &mut Components,
        osc: Oscillators,
    ) {
        let mut this = Self::get_proxy(comp);
        this.should_be_disabled[osc] = false;
        // Not sleeping, just we should delay the startup TODO: delay startup
        let startup_latency = Duration::MIN_DELAY;
        ClockTreeProxy.pulse(ctx, startup_latency, osc);
        let mode = PowerMode::Active;
        TicksScheduler::inner_set_osc_power_state(comp, ctx, components, osc, mode);
    }
}
