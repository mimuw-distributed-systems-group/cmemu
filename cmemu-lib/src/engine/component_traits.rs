use crate::common::utils::SubcomponentProxyMut;
use crate::engine::Context;
pub(crate) use cmemu_proc_macros::{
    DisableableComponent, MainComponent, SkippableClockTreeNode, Subcomponent, TickComponent,
    TickComponentExtra,
};
use std::fmt::Debug;
use std::hash::Hash;
// export both derive macros and the traits

/// Trait meant to be derived automatically. For traversing component tree, ticking all the flops,
/// checking assertions, etc.
///
/// The order is:
/// - traverse (sub)components tree in post-order (leaves first), calling for each one:
///   - `TickComponentExtra::tick_assertions`
/// - traverse (sub)components tree in post-order (leaves first), calling for each one (in this order):
///   - `TickComponent::tick_flops`
///   - `TickComponentExtra::tick_extra`
/// - then `Component::tick()` body runs. Only of the main component.
///   It should call `tick`, `run_*`, or other methods of subcomponents if needed.
pub(crate) trait TickComponent: TickComponentExtra {
    /// Traverses component tree in post-order (leaves first) calling for each node:
    /// `TickComponentExtra::tick_assertions`.
    #[cfg(debug_assertions)]
    fn tick_assertions_traverse(&self);
    /// Traverses component tree in post-order (leaves first) calling for each node:
    /// `TickComponent::tick_flops`, then `TickComponentExtra::tick_extra`.
    fn tick_flops_and_extra_traverse(&mut self);
    /// Ticks flops that are direct fields on `Self`
    fn tick_flops(&mut self);
}

/// Trait for providing extra behaviour that can be manually implemented.
/// If there's no need for it, it can be derived.
pub(crate) trait TickComponentExtra {
    /// Check additional assertions at the end of the tick - right before actual next tick.
    ///
    /// You can access subcomponents of `self`. Assertions had been already run on them.
    #[cfg(debug_assertions)]
    fn tick_assertions(&self) {}
    /// Perform extra actions right after ticking the flops.
    ///
    /// Meant to manage state of direct fields and depend only on them, without assuming anything else.
    /// Can tamper with subcomponents of `self`, as `tick_extra` had been already run on them.
    /// Actual logic should be put in `run_*` and other methods that will be called by the main component `tick()`
    fn tick_extra(&mut self) {}
}

/// A marker trait to be derived on the main components.
///
/// **Note:** the proc macro generates multiple other implementations:
/// - includes derivation of [`Subcomponent`] (**don't** derive it too).
/// - connecting to the clock-and-power subsystem by generating [`EnergyNode`],
///   [`ClockTreeNode`], and [`PowerNode`]
///
/// Most importantly, `ClockTreeNode::tick` is implemented by calling [`TickComponent`],
/// then the `tick(&mut self, ctx: &mut Context)` method of the component.
/// Similarly, `ClockTreeNode::tock` is forwarded to the `tick` method.
///
/// The `PowerNode` derivation delegates switching decision to [`DisableableComponent`],
/// and uses the [`Context`] for state storage.
// TODO: Make the `tick`/`tock` methods a proper trait, not calling-by-convention.
// TODO: reset() method to handle off power mode
#[allow(unused)] // This is mostly to mark derive
pub(crate) trait MainComponent: Subcomponent
where
    Self: Subcomponent<Member = Self>,
{
}

/// Allows calling component's methods from subcomponent code
///
/// # General overview
///
/// Sometimes we want to split a big component into smaller parts (we call them "subcomponents")
/// that have direct access to each other. We could simply create object for each part
/// of component, but the borrow-checker forbids to pass reference to component and reference
/// to member-of-the-component at the same time.
///
/// We parametrize subcomponent type with [`Subcomponent`] trait implementation that says
/// how to get reference to subcomponent from reference to whole component.
///
/// ## Inter-subcomponent-communication
///
/// Communication between components is safe, because all messages goes through event queue
/// and no event can arrive between ticking components.
///
/// In communication inside single component (also: between subcomponents) we do not have this
/// guarantee. You need to specify some invariants and make sure they are respected.
///
/// # Proc macro
///
/// - `#[subcomponent(TargetStruct)]` generates atom `struct TargetStruct;` implementing `trait Subcomponent`.
///   Should be used always if possible.
/// - `#[subcomponent]` doesn't generate the aforementioned struct, so it can be implemented explicitly.
///   It is meant for cases when automatic generation doesn't work (yet), i.e. generics.
pub(crate) trait Subcomponent {
    type Component;
    type Member;

    fn component_to_member(component: &Self::Component) -> &Self::Member;
    fn component_to_member_mut(component: &mut Self::Component) -> &mut Self::Member;

    //noinspection RsNeedlessLifetimes
    /// Get a smart-pointer
    fn get_proxy(component: &mut Self::Component) -> SubcomponentProxyMut<'_, Self>
    where
        Self: Sized,
    {
        SubcomponentProxyMut::from(component)
    }
}

/// This trait is to be implemented by the <SC> auto-generated markers,
/// so they may be distinguished by the trait resolver from derived Subcomponent implementations.
pub(crate) trait PureSubcomponentMarker {}

/// Indicate whether the (sub)component may be safely disabled in this state.
///
/// The disabling means that lowering the power state (e.g., gating the clocks)
/// will not break any invariants.
///
/// This trait can be derived. The generated code requires that all the:
///
/// - `#[flop]`-s are clear (`is_clear`): that is ephemeral flops have neither current nor next value,
///   and register/memory banks have no next value.
/// - `#[subcomponent]`-s implement `DisableableComponent` and can be disabled,
/// - the same goes for any field marked with `#[disableable]` attribute,
///
/// except any fields marked as `#[disableable_ignore]`.
pub(crate) trait DisableableComponent {
    /// This method gets only `&self`, as it should depend only on the local state of
    /// the component (and its subcomponents).
    fn can_be_disabled_now(&self) -> bool;
}

////////////////////////////////
// Clock and power subsystem  //
////////////////////////////////

// Most of the methods will receive
// `comp: Self::Component` – because it allows putting the nodes in a graph
// `ctx: Context` - because the context has global information
// `parent: Self::IdSpace` - because switches need it
// `extra: Self::Extra` - because the clock code needs references to components

/// A core descriptor trait for the clock-and-power subsystem
pub(crate) trait EnergyNode {
    /// (Almost) all methods in this subsystem receive an extra value of this type.
    ///
    /// It is an abstraction leakage. The components use the unit here.
    /// The managing code need to get hold of references to `Components`.
    type Extra;
    /// An identifier space for nodes in the clock-and-power subsystem.
    ///
    /// These values come as `parent` in most methods, which are mostly useful for switches,
    /// but also for debugging purposes.
    type IdSpace: Debug + Clone + Copy + PartialEq + Eq + Hash + enum_map::Enum;
    /// A human-understandable name of this node (used only for logs).
    const NAME: &'static str;

    /// Return the identifier of the current clock-and-power graph node.
    fn id() -> Self::IdSpace;
}

/// The primary clock subsystem trait for `tick` and `tock`.
///
/// This is similar to methods called by the `MainComponent`, which are a special case,
/// but handle information needed for various nodes of the graph.
/// In fact, `#[derive(MainComponent)]` generates an implementation of the `ClockTreeNode`.
pub(crate) trait ClockTreeNode: EnergyNode + Subcomponent {
    fn tick(
        comp: &mut Self::Component,
        ctx: &mut Context,
        parent: Self::IdSpace,
        extra: &mut Self::Extra,
    );
    fn tock(
        comp: &mut Self::Component,
        ctx: &mut Context,
        parent: Self::IdSpace,
        extra: &mut Self::Extra,
    );
}

/// A generic power state of a digital component.
///
/// Not all values may be applicable to all components/nodes:
/// for instance, `ClockGated` is meaningless to nodes without a clock line,
/// but ordering of the enum variants provide a unified interface over homogenous power-consuming nodes.
///
/// This enum is abstracted from states described in [TI-TRM] 6. Power, Reset, and Clock Management
/// as well as sample docs for other uControllers.
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, enum_map::Enum)]
pub enum PowerMode {
    /// The node is cut off from any voltage lines.
    /// It is going through the reset sequence when powered on again.
    ///
    /// For some components without a full clean reset, the state of the component may depend
    /// on the time the node was off.
    /// For instance, SRAM bits are typically not zeroed on reset, but the slow discharge
    /// of the lines will result in a more-or-less unpredictable state.
    Off,
    /// The node is in a low-power retention state.
    ///
    /// It typically means, that the clock is not ticking and the node cannot output power.
    /// In other words, it can retain state of registers with only leakage current.
    /// As an example, consider an SRAM bit, which only requires voltage to the four transistors
    /// to preserve its state.
    Retention,
    /// The node is powered, so it holds all the state and outputs,
    /// but the clock is not ticking, so no sequential logic can proceed.
    ClockGated,
    /// The node has power and expect the clock lines to be ticking (if applicable).
    Active,
}

impl PowerMode {
    pub fn is_active(self) -> bool {
        self == PowerMode::Active
    }
}

/// The Cortex-M3 (CPU) power modes [ARM-TRM-G] 7. Power Management.
///
/// The documentation distinguishes HCLK from FCLK (Free-running CLocK), which is used by
/// parts of NVIC, DWT and ITM blocks.
/// FCLK is active in `Sleep` mode and may have its frequency reduced then.
///
/// The system always goes to `DeepSleep` through the shallow `Sleep` mode.
///
/// See also:
/// - [ARM-TRM-G] Figure 7-4 Power down timing sequence
/// - [ARM-TRM-G] A.5 Low power interface
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, enum_map::Enum)]
pub(crate) enum CpuMode {
    /// Indicates that all the clock to the Cortex-M3 may be gated.
    /// For systems with WIC (such as cc2650), FCLK is gated as well,
    /// and all the power to the processor.
    /// The `SLEEPHOLDREQn` signal is used to prevent the processor from waking up during
    /// power-down sequence.
    /// See the *SLEEPDEEP* signal in [ARM-TRM-G] 7.2 and 7.2.2
    DeepSleep,
    /// Indicates that clock of the processor may be stopped (HCLK),
    /// but other system components such as NVIC remain active (FCLK).
    /// See the *SLEEPING* signal in [ARM-TRM-G] 7.2 and 7.2.1
    Sleep,
    Run,
}

/// A trait for nodes which can emulate passage of time (ticks).
///
/// The high-level rules are:
/// - Emulation fast-forwards state from post-tock to a post-tock N full cycles later.
/// - Emulation cannot generate events. Reminder: the events should go in tock based on tick state.
/// - A node reports how many cycles it could skip (emulate) without generating events.
/// - A parent node may skip fewer or no cycles at all, but never more than returned.
///
/// This trait may be derived.
/// The default derivation disallows skipping cycles (always returns 0).
/// You can use the `#[skippable_if_disableable]` attribute, to call [`DisableableComponent`]
/// and return infinity when the component is disableable.
/// The emulation would just do nothing.
// TODO: there is a potential for static optimization, which is hard to represent right now.
//       Not-implementing this trait doesn't propagate too well: it should effectively kill evaluation
//       for a whole subtree, but the compiler cannot do that (as it needs to run side effects).
pub(crate) trait SkippableClockTreeNode: ClockTreeNode {
    /// Return the maximum number of cycles of this clock node would not generate any event
    /// without receiving one first.
    ///
    /// This method is not guaranteed to be called if we don't plan to skip.
    fn max_cycles_to_skip(
        _comp: &mut Self::Component,
        _ctx: &mut Context,
        _parent: Self::IdSpace,
        _extra: &mut Self::Extra,
    ) -> u64 {
        0 // No skipping allowed
    }

    /// Move from post-Tock to a post-Tock state `skipped_cycles` later.
    ///
    /// Must NOT generate an event invalidating cycle skipping invariants.
    /// Might not be called if no cycle was skipped.
    fn emulate_skipped_cycles(
        _comp: &mut Self::Component,
        _ctx: &mut Context,
        _parent: Self::IdSpace,
        _extra: &mut Self::Extra,
        _skipped_cycles: u64,
    ) {
        panic!(
            "Emulation is not supported for {}!",
            <Self as EnergyNode>::NAME
        );
    }
}

/// Represents power and clock-gating state changes.
pub(crate) trait PowerNode: EnergyNode + Subcomponent {
    /// Get the ground-truth state of the node
    fn get_power_state(comp: &Self::Component, ctx: &Context) -> PowerMode;
    fn is_active(comp: &Self::Component, ctx: &Context) -> bool {
        Self::get_power_state(comp, ctx).is_active()
    }

    /// Return the closest (to the requested) acceptable power mode.
    ///
    /// Facility for doing "ask-commit" procedures regarding the power state.
    /// The method is called on a clock/power subsystem node to indicate an intention
    /// to switch the power state of that node (usually as a consequence of a switch of another node).
    ///
    /// Example use-case: a gate should not switch its state between `tick` and `tock`.
    /// On a clock line, turning off a gate would represent transitioning from an `Active` power mode
    /// to a `ClockGated` mode.
    ///
    /// The implementation typically calls to [`DisableableComponent`].
    fn prepare_to_disable(
        comp: &mut Self::Component,
        ctx: &mut Context,
        parent: Self::IdSpace,
        extra: &mut Self::Extra,
        mode: PowerMode,
    ) -> PowerMode;

    /// Actually set the power mode
    fn set_power_state(
        comp: &mut Self::Component,
        ctx: &mut Context,
        parent: Self::IdSpace,
        extra: &mut Self::Extra,
        mode: PowerMode,
    );
}
