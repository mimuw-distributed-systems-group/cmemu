// IMPORTANT: keep all RNG code replicable (no OS seed in impls!)
use crate::engine::{Context, Duration, PICOS_IN_SECOND, Timepoint};
use fastrand::Rng;
use std::ops::RangeInclusive;

// Oscillators:
// - Perfect has const delay
// - Base_freg (const?) + drift (per node) + jitter
// - PLL: divider + trained with ticks
// Oscillators convert time-based events to tick-based events
/// The Oscillator API
pub trait Oscillator {
    /// Reset of the oscillator (can get something from the ctx).
    fn power_on_reset(&mut self, ctx: &mut Context);
    /// Normal operation: at asked duration, return next duration.
    fn pulse(&mut self, ctx: &Context) -> Duration;
    /// Fast-forward of time (from last aligned tick): how many ticks happened, remaining time.
    ///
    /// Must be consistent with [`Self::cycles_to_time`].
    fn fast_forward(&mut self, ctx: &Context, delta: Duration) -> (u64, Duration);

    /// Return the duration that it would take to skip `cycles` amount of cycles.
    fn cycles_to_time(&mut self, ctx: &Context, cycles: u64) -> Duration;
    /// Return the current frequency (in Hertz) for debugging.
    #[cfg_attr(not(test), allow(unused))]
    fn get_instantaneous_frequency(&self) -> u64;
}

/// A perfect oscillator.
#[derive(Debug, Clone, Default)]
pub(crate) struct ConstOsc<const N: u64>;

impl<const N: u64> Oscillator for ConstOsc<N> {
    fn power_on_reset(&mut self, _ctx: &mut Context) {}

    fn pulse(&mut self, _ctx: &Context) -> Duration {
        Duration::from_picos(N)
    }

    fn fast_forward(&mut self, _ctx: &Context, delta: Duration) -> (u64, Duration) {
        // TODO: on an aligned time, should we return an immediate pulse or the next one?
        let duration = Duration::from_picos(N);
        (
            delta / duration,
            Duration::from_picos(N - delta.as_picos() % N),
        )
    }

    fn cycles_to_time(&mut self, ctx: &Context, cycles: u64) -> Duration {
        self.pulse(ctx).saturating_mul_u64(cycles)
    }

    fn get_instantaneous_frequency(&self) -> u64 {
        PICOS_IN_SECOND / N
    }
}

// This should be easy to expand to a temperature-based skew
/// An oscillator with first-order skew and random jitter.
///
/// The oscillator is characterized by a `central_frequency` (in picos/tick) and `drift`.
/// On reset, we skew the `central_frequency` by `rand(-drift..=drift)` based on the `node_id`,
/// so the runs should be replicable.
/// Additionally, each pulse delay is modified by `rand(-jitter..=jitter)`.
#[cfg_attr(not(test), allow(unused))] // Not yet employed
#[derive(Debug, Clone)]
pub(crate) struct JitteryOsc {
    // Day-to-day data
    base_delay: u64,
    jitter: u32,
    rng: Rng,
    // Configuration. Maybe make the config a trait-consts.
    central_frequency: u64,
    drift: u32,
}

impl Oscillator for JitteryOsc {
    fn power_on_reset(&mut self, ctx: &mut Context) {
        const JITTERY_OSC_RNG_SALT: u64 = 0x0013_77e2;
        self.rng
            .seed(ctx.node_id() ^ JITTERY_OSC_RNG_SALT ^ self.base_delay);
        self.update_cache();
    }

    fn pulse(&mut self, _ctx: &Context) -> Duration {
        let mut ps = self.base_delay;
        if self.jitter != 0 {
            ps = ps.saturating_add_signed(self.random_jitter());
        }
        Duration::from_picos(ps)
    }

    fn fast_forward(&mut self, ctx: &Context, delta: Duration) -> (u64, Duration) {
        // TODO: support sub-ps drift (48MHz * 10ppm is < 1 ps per tick)
        // Note: this impl is not a true distribution as if ticked n times
        let extrapolated = self.pulse(ctx);
        let next = self.pulse(ctx);
        let mut full_cycles = delta / extrapolated;
        let mut rem = delta - extrapolated * full_cycles;
        if rem > next {
            // Rare case possible if next was smaller than the extrapolated cycle
            full_cycles += 1;
            rem -= next;
        }
        (full_cycles, next - rem)
    }

    fn cycles_to_time(&mut self, ctx: &Context, cycles: u64) -> Duration {
        debug_assert!(cycles > 0);
        // Not the greatest impl?
        let mut cloned = self.clone();
        let extrapolated = cloned.pulse(ctx);
        let next = cloned.pulse(ctx);
        debug_assert!(
            next * 2 > extrapolated && extrapolated * 2 > next,
            "Assumptions of JitteryOsc need jitter to be an order of magnitude smaller than the frequency."
        );
        // worst case: full = (e*(c-1)+n)/e = (c-1) + [n/e]; rem = n % e
        extrapolated
            .saturating_mul_u64(cycles.saturating_sub(1))
            .saturating_add_duration(next)
    }

    fn get_instantaneous_frequency(&self) -> u64 {
        PICOS_IN_SECOND / self.base_delay
    }
}

#[cfg_attr(not(test), allow(dead_code))]
impl JitteryOsc {
    pub(crate) fn new(cycles_per_pulse: u64, drift_range: u32, jitter_range: u32) -> Self {
        Self {
            base_delay: cycles_per_pulse,
            jitter: jitter_range,
            rng: Rng::with_seed(cycles_per_pulse ^ u64::from(drift_range)),
            central_frequency: cycles_per_pulse,
            drift: drift_range,
        }
    }

    fn drift_range(&self) -> RangeInclusive<i32> {
        (-self.drift.cast_signed())..=self.drift.cast_signed()
    }

    fn jitter_range(&self) -> RangeInclusive<i32> {
        (-self.jitter.cast_signed())..=self.jitter.cast_signed()
    }

    fn random_jitter(&mut self) -> i64 {
        i64::from(self.rng.i32(self.jitter_range()))
    }

    fn update_cache(&mut self) {
        self.base_delay = self
            .central_frequency
            .saturating_add_signed(i64::from(self.rng.i32(self.drift_range())));
    }

    /// Update the dift and jitter components (e.g. by training an RC osc with a crystal one)
    pub(crate) fn train(&mut self, new_drift: u32, new_jitter: u32) {
        self.drift = new_drift;
        self.jitter = new_jitter;
        // Or maybe it should cap the drift instead of reroll?
        self.update_cache();
    }
}

/// PLL (Phase Locked Loop) oscillator, that allows any ratio over a parent oscillator.
///
/// The oscillator implements a simple PLL with a central `delay`,
/// and a random walk with `rand(-decay..=decay)` on each pulse.
///
/// The parent oscillator should call [`Self::tick`] to train this oscillator (fix `delay`)..
/// This is a rudimentary implementation to verify the `CMEmu` design allows for placing PLLs.
#[cfg_attr(not(test), allow(unused))] // Will be used one day
#[derive(Debug, Clone)]
pub(crate) struct PhaseLockedLoop {
    /// Picos to next tick
    delay: u64,
    /// Lock decay per tick a random int from `(-decay, decay)` is added to `delay`.
    decay: u32,
    rng: Rng,
    last_tick: Option<Timepoint>,
    /// `(n, m)`: #n PLL Pulses per #m Ticks
    ratio: (u8, u8),
    /// Balance is positive if PLL is ticking too fast
    balance: i16,
}

impl Oscillator for PhaseLockedLoop {
    fn power_on_reset(&mut self, ctx: &mut Context) {
        const PLL_OSC_SALT: u64 = 0x1102c;
        self.rng.seed(ctx.node_id() ^ PLL_OSC_SALT);
    }

    fn pulse(&mut self, _ctx: &Context) -> Duration {
        debug_assert!(
            self.is_locked(),
            "PLL {self:?} cannot be pulsed without ticks (no ticks or lock was lost: freq change?)"
        );
        if self.decay != 0 {
            self.delay = self
                .delay
                .saturating_add_signed(i64::from(self.rng.i32(self.decay_range())));
        }
        self.balance = self.balance.saturating_add(self.ratio.1.into());
        Duration::from_picos(self.delay)
    }

    fn fast_forward(&mut self, _ctx: &Context, delta: Duration) -> (u64, Duration) {
        let dpicos = delta.as_picos();
        // Invalidate last tick, as it may be way off
        self.last_tick = None;
        // Assume lock is kept
        if self.is_locked() {
            (
                dpicos / self.delay,
                Duration::from_picos(self.delay - dpicos % self.delay),
            )
        } else {
            todo!("Implement non-locked PLL decaying")
        }
    }

    fn cycles_to_time(&mut self, _ctx: &Context, cycles: u64) -> Duration {
        // Assume lock is kept
        if self.is_locked() {
            Duration::from_picos(self.delay).saturating_mul_u64(cycles)
        } else {
            todo!("Implement non-locked PLL decaying")
        }
    }

    fn get_instantaneous_frequency(&self) -> u64 {
        PICOS_IN_SECOND / self.delay
    }
}

#[cfg_attr(not(test), allow(dead_code))]
impl PhaseLockedLoop {
    pub(crate) fn new(ratio: (u8, u8), decay: u32) -> Self {
        Self {
            delay: 0, // invalid?
            decay,
            rng: Rng::with_seed(u64::from(decay)),
            last_tick: None,
            ratio,
            balance: i16::MAX, // start invalid
        }
    }

    fn is_locked(&self) -> bool {
        const LOCK_LOSS_EPOCHS: i16 = 4;
        self.balance < i16::from(self.ratio.0).saturating_mul(LOCK_LOSS_EPOCHS)
    }

    fn decay_range(&self) -> RangeInclusive<i32> {
        (-self.decay.cast_signed())..=self.decay.cast_signed()
    }

    /// External ticks from the parent oscillator
    pub(crate) fn tick(&mut self, ctx: &Context) {
        let time = ctx.event_queue().get_current_time();
        if let Some(last_tick) = self.last_tick.take() {
            let delta = time.wrapping_sub_timepoint(last_tick).as_picos();

            // Consider implementing full stabilization, e.g. with midpoint
            self.delay = mul_u64_rational(delta, (self.ratio.1, self.ratio.0));
            // Balance would be updated like this:
            // self.balance = self.balance.saturating_sub(self.ratio.0.into());
            self.balance = 0;
        }
        self.last_tick = Some(time);
    }
}

#[cfg_attr(not(test), allow(dead_code))]
fn mul_u64_rational(x: u64, r: (impl Into<u64>, impl Into<u64>)) -> u64 {
    let mult = r.0.into();
    let div = r.1.into();
    match x.checked_mul(mult) {
        Some(xm) => xm / div,
        None => {
            u64::try_from(u128::from(x) * u128::from(mult) / u128::from(div)).unwrap_or(u64::MAX)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};
    use std::any::Any;

    #[fixture]
    fn ctx() -> Context {
        Context::new_for_test()
    }

    fn advance_time(ctx: &mut Context, delta: Duration) {
        let now = ctx.event_queue().get_current_time();
        dbg!(delta);
        ctx.event_queue_mut()
            .fake_time(now.wrapping_add_duration(delta));
    }

    #[rstest]
    #[case::const_12(ConstOsc::<12>)]
    #[case::const_1(ConstOsc::<1>)]
    #[case::jittery(JitteryOsc::new(123, 10, 1))]
    #[case::pll(PhaseLockedLoop::new((7, 2), 2))]
    fn test_oscillators(
        #[case] mut osc: impl Oscillator + Any,
        #[values(0, 1)] node_id: u64,
        mut ctx: Context,
    ) {
        let ctx = &mut ctx;
        ctx.set_node_id_for_test(node_id);
        let mut rng = Rng::new();
        osc.power_on_reset(ctx);

        if let Some(osc) = (&mut osc as &mut dyn Any).downcast_mut::<PhaseLockedLoop>() {
            for _ in 0..5 {
                osc.tick(ctx);
                advance_time(ctx, Duration::from_picos(123));
            }
        }

        // Regular pulsing
        for _ in 0..5 {
            let delay = osc.pulse(ctx);
            assert!(delay > Duration::ZERO);
            // divided by 2 for a broad range (sanity check)
            assert!(osc.get_instantaneous_frequency() > PICOS_IN_SECOND / delay.as_picos() / 2);
            advance_time(ctx, delay);
        }
        // Typical sleep with possible early wakeup
        for _ in 0..6 {
            let skippable_cycles = rng.u64(1..7);
            let delay = osc.cycles_to_time(ctx, skippable_cycles);
            let actual_skip = Duration::from_picos(rng.u64(0..=delay.as_picos()));
            advance_time(ctx, actual_skip);
            let (cycles, remainder) = osc.fast_forward(ctx, actual_skip);
            assert!(cycles <= skippable_cycles);
            advance_time(ctx, remainder);
            if rng.bool() {
                advance_time(ctx, osc.pulse(ctx));
            }
        }
        // Exact wakeup sleep
        for i in 1..10 {
            let fixed_cycles = i;
            let delay = osc.cycles_to_time(ctx, fixed_cycles);
            advance_time(ctx, delay);
            let (cycles, remainder) = osc.fast_forward(ctx, delay);
            assert!(
                cycles == fixed_cycles || cycles == fixed_cycles.saturating_sub(1),
                "{cycles} != {fixed_cycles}+-1"
            );
            advance_time(ctx, remainder);
        }
        // Saturating time / cycles, exact wakeup
        for i in 0..5 {
            let skippable_cycles = u64::MAX - i;
            let delay = osc.cycles_to_time(ctx, skippable_cycles);
            advance_time(ctx, delay);
            let (cycles, remainder) = osc.fast_forward(ctx, delay);
            assert!(cycles <= skippable_cycles);
            // divided by 2 for a broad range (sanity check)
            assert!(cycles >= u64::MAX / osc.get_instantaneous_frequency() / 2);
            advance_time(ctx, remainder);
        }
    }

    #[rstest]
    fn trait_jittery_osc(#[values(0, 1)] node_id: u64, mut ctx: Context) {
        let mut osc = JitteryOsc::new(1_000_000, 1_000, 100);
        let ctx = &mut ctx;
        ctx.set_node_id_for_test(node_id);
        osc.power_on_reset(ctx);

        assert!((998_000..=1_002_000).contains(&osc.get_instantaneous_frequency()));
        osc.train(10, 1);
        assert!((999_980..=1_000_020).contains(&osc.get_instantaneous_frequency()));
        // Make it a perfect clock
        osc.train(0, 0);
        assert_eq!(osc.get_instantaneous_frequency(), 1_000_000);
        for _ in 0..30 {
            let delay = osc.pulse(ctx);
            // No jitter
            assert_eq!(delay.as_picos(), 1_000_000);
            advance_time(ctx, delay);
        }
    }

    #[rstest]
    #[case::keep(true)]
    #[should_panic(expected = "lock was lost")]
    #[case::loss(false)]
    fn test_pll_lock(
        #[case] keep_lock: bool,
        #[values(0, 1)] node_id: u64,
        #[values((3, 2), (7, 1), (2, 6))] ratio: (u8, u8),
        mut ctx: Context,
    ) {
        let mut ref_osc = ConstOsc::<21524>;
        let mut osc = PhaseLockedLoop::new(ratio, 24);
        let ctx = &mut ctx;
        ctx.set_node_id_for_test(node_id);
        osc.power_on_reset(ctx);

        // startup
        for _ in 0..5 {
            advance_time(ctx, ref_osc.pulse(ctx));
            osc.tick(ctx);
        }
        // manual event-queue impl
        let now = ctx.event_queue().get_current_time();
        let mut next_ref = now + ref_osc.pulse(ctx);
        let mut next_osc = now + osc.pulse(ctx);
        for _ in 0..40 {
            if next_ref < next_osc && keep_lock {
                advance_time(ctx, next_ref - ctx.event_queue().get_current_time());
                next_ref += dbg!(ref_osc.pulse(ctx));
                osc.tick(ctx);
            } else {
                advance_time(ctx, next_osc - ctx.event_queue().get_current_time());
                next_osc += dbg!(osc.pulse(ctx));
            }
        }
    }
}
