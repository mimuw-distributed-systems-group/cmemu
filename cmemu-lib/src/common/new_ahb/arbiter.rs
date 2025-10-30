use crate::common::new_ahb::decoder::DefaultSlave;
use crate::common::new_ahb::signals::wire::LOW;
use crate::common::new_ahb::signals::{Burst, MasterToSlaveAddrPhase};
use crate::common::utils::FromMarker;
use crate::common::utils::iter_enum_t;
use crate::engine::Subcomponent;
use crate::engine::{BufferFlop, DisableableComponent, TickComponent, TickComponentExtra};
use enum_map::{EnumArray, EnumMap};
use log::trace;
use std::fmt::Debug;
use std::marker::PhantomData;

// Arbiter has no access to outer state by design
pub(crate) trait Arbiter<TAG>: TickComponent
where
    TAG: EnumArray<bool> + Copy + PartialEq + Debug,
{
    /// None -> `no_port`, Some -> `addr_in_port`
    /// Called after tick edge for simplicity!
    fn arbitrate(&mut self, reqs: EnumMap<TAG, bool>, last: MasterToSlaveAddrPhase) -> Option<TAG>;
    // Returns tha same as arbitrate, but may be called earlier
    fn get_addr_in_port(&self) -> Option<TAG>;
}

#[derive(Subcomponent, TickComponent, TickComponentExtra, DisableableComponent, Debug)]
pub(crate) struct NoArbiter<TAG, ONLYPM>
where
    TAG: PartialEq + Copy + Debug + FromMarker<ONLYPM>,
{
    phantom_tag: PhantomData<TAG>,
    phantom_pm: PhantomData<ONLYPM>,
}
impl<TAG, ONLYPM> Default for NoArbiter<TAG, ONLYPM>
where
    TAG: PartialEq + Copy + Debug + FromMarker<ONLYPM>,
{
    fn default() -> Self {
        Self {
            phantom_tag: PhantomData,
            phantom_pm: PhantomData,
        }
    }
}

impl<TAG, ONLYPM> Arbiter<TAG> for NoArbiter<TAG, ONLYPM>
where
    TAG: EnumArray<bool> + Copy + PartialEq + Debug + FromMarker<ONLYPM>,
{
    fn arbitrate(
        &mut self,
        #[cfg_attr(not(debug_assertions), allow(unused))] reqs: EnumMap<TAG, bool>,
        _last: MasterToSlaveAddrPhase,
    ) -> Option<TAG> {
        #[cfg(debug_assertions)]
        {
            let sole_marker = FromMarker::<ONLYPM>::from_marker();
            assert!(
                !reqs.iter().any(|(t, &v)| v && t != sole_marker),
                "NoArbiter got requests from non-sole marker {}: {:?}",
                <TAG as FromMarker<ONLYPM>>::MARKER_NAME,
                reqs,
            );
        }
        Some(FromMarker::<ONLYPM>::from_marker())
    }

    fn get_addr_in_port(&self) -> Option<TAG> {
        Some(FromMarker::<ONLYPM>::from_marker())
    }
}

/// Always returns None, panics at any request
/// Or should we call it Unimplemented Arbiter?
#[derive(Subcomponent, TickComponent, TickComponentExtra, DisableableComponent, Debug)]
pub(crate) struct NullArbiter<TAG>
where
    TAG: PartialEq + Copy + Debug,
{
    phantom_tag: PhantomData<TAG>,
}
impl<TAG> Default for NullArbiter<TAG>
where
    TAG: PartialEq + Copy + Debug,
{
    fn default() -> Self {
        Self {
            phantom_tag: PhantomData,
        }
    }
}

impl<TAG> Arbiter<TAG> for NullArbiter<TAG>
where
    TAG: EnumArray<bool> + Copy + PartialEq + Debug,
{
    fn arbitrate(
        &mut self,
        #[cfg_attr(not(debug_assertions), allow(unused))] reqs: EnumMap<TAG, bool>,
        _last: MasterToSlaveAddrPhase,
    ) -> Option<TAG> {
        #[cfg(debug_assertions)]
        {
            assert!(
                !reqs.iter().any(|(_t, &v)| v),
                "NullArbiter got a request: {reqs:?}",
            );
        }
        None
    }

    fn get_addr_in_port(&self) -> Option<TAG> {
        None
    }
}

// Note: the default marker is implemented for all generated tags enum-struct-magic
type AlwaysNoneMarker = DefaultSlave;
#[allow(dead_code)]
#[derive(Subcomponent, TickComponent, TickComponentExtra, DisableableComponent, Debug)]
pub(crate) struct RoundRobinArbiter<TAG, DEFAULT = AlwaysNoneMarker>
where
    TAG: PartialEq + Copy + EnumArray<bool> + Debug,
    Option<TAG>: FromMarker<DEFAULT>,
{
    // Arbiter does all the work on thick, so those values are stale
    #[flop]
    addr_in_port: BufferFlop<Option<TAG>>,

    phantom_tag: PhantomData<DEFAULT>,
}

impl<TAG, DEFAULT> Default for RoundRobinArbiter<TAG, DEFAULT>
where
    TAG: PartialEq + Copy + EnumArray<bool> + Debug,
    Option<TAG>: FromMarker<DEFAULT>,
{
    fn default() -> Self {
        RoundRobinArbiter {
            addr_in_port: BufferFlop::new_from(FromMarker::<DEFAULT>::from_marker()),
            phantom_tag: PhantomData,
        }
    }
}

impl<TAG, DEFAULT> Arbiter<TAG> for RoundRobinArbiter<TAG, DEFAULT>
where
    TAG: PartialEq + Copy + EnumArray<bool> + Debug,
    Option<TAG>: FromMarker<DEFAULT>,
{
    fn arbitrate(&mut self, reqs: EnumMap<TAG, bool>, last: MasterToSlaveAddrPhase) -> Option<TAG> {
        if !last.ready {
            self.addr_in_port.keep_current_as_next();
            return *self.addr_in_port.get_prev_cycle();
        }

        debug_assert!(
            !last.meta.is_address_valid_and(|m| m.burst != Burst::Single),
            "Burst transfers need research in round-robin arbiter!"
        );

        // Round-robin arbitration scheme -> we start from the current input port
        // and find first next (wrapping to first) that requests access.
        // In case nobody requests, an IDLE transfer will keep the current port,
        // otherwise it will go to /dev/null
        let next_port = if last.lock {
            *self.addr_in_port.get_prev_cycle()
        } else {
            // Find next port requesting
            // None is same as last
            let start = self
                .addr_in_port
                .unwrap_or_else(|| reqs.iter().next_back().unwrap().0);
            // We should first iterate [next(start)+1:] then [:next(start)]
            // first after the current one
            let proposal = reqs
                .iter()
                .skip_while(|&(t, _)| t != start)
                .skip(1)
                .find(|&(_, &v)| v)
                .map(|(t, _)| t);
            // if not found, find until the current one
            let proposal = proposal.or_else(|| {
                reqs.iter()
                    // Commented to allow visiting start!
                    // .take_while(|&(t, _)| t != start)
                    .find(|&(_, &v)| v)
                    .map(|(t, _)| t)
            });
            // Still haven't visited self
            let keep_current = self.addr_in_port.filter(|_| last.meta.is_idle());
            proposal.or(keep_current)
        };

        self.addr_in_port.set_this_cycle(next_port);
        // Output is lagged!
        trace!(
            "RR Arbitration of {:?} while {:?}: next_port: {:?}",
            reqs, last, self.addr_in_port
        );

        *self.addr_in_port.get_this_cycle()
    }

    fn get_addr_in_port(&self) -> Option<TAG> {
        *self.addr_in_port.get_this_cycle()
    }
}

#[derive(Subcomponent, TickComponent, TickComponentExtra, DisableableComponent, Debug)]
pub(crate) struct FixedArbiter<TAG, DEFAULT = AlwaysNoneMarker>
where
    TAG: PartialEq + Copy + EnumArray<bool>,
    Option<TAG>: FromMarker<DEFAULT>,
{
    #[flop]
    addr_in_port: BufferFlop<Option<TAG>>,
    forced: Option<TAG>,
    phantom_tag: PhantomData<DEFAULT>,
}

impl<TAG, DEFAULT> Default for FixedArbiter<TAG, DEFAULT>
where
    TAG: PartialEq + Copy + EnumArray<bool>,
    Option<TAG>: FromMarker<DEFAULT>,
{
    fn default() -> Self {
        Self {
            // TODO: Currently we start with the highest priority -> needs research for generality
            addr_in_port: BufferFlop::new_from(
                <Option<TAG> as FromMarker<DEFAULT>>::from_marker()
                    .or_else(|| iter_enum_t::<TAG, _>().next()),
            ),
            forced: None,
            phantom_tag: PhantomData,
        }
    }
}
impl<TAG, DEFAULT> FixedArbiter<TAG, DEFAULT>
where
    TAG: PartialEq + Copy + EnumArray<bool>,
    Option<TAG>: FromMarker<DEFAULT>,
{
    pub(crate) fn force_req_hax(&mut self, forced: Option<TAG>) {
        self.forced = forced;
    }
}

impl<TAG, DEFAULT> Arbiter<TAG> for FixedArbiter<TAG, DEFAULT>
where
    TAG: PartialEq + Copy + EnumArray<bool> + Debug,
    Option<TAG>: FromMarker<DEFAULT>,
{
    fn arbitrate(
        &mut self,
        mut reqs: EnumMap<TAG, bool>,
        last: MasterToSlaveAddrPhase,
    ) -> Option<TAG> {
        if let Some(tag) = self.forced {
            reqs[tag] = true;
        }
        // TODO: burst transfers?
        if last.ready && last.lock == LOW {
            let keep_current = self.addr_in_port.filter(|_| last.meta.is_idle());

            // Top-priority or keep if transfer is active
            self.addr_in_port.set_this_cycle(
                reqs.iter()
                    .find(|&(t, &v)| v || (Some(t) == *self.addr_in_port && !last.meta.is_idle()))
                    .map(|(t, _)| t)
                    .or(keep_current),
            );
        } else {
            self.addr_in_port.keep_current_as_next();
        }
        // Output is lagged!
        trace!(
            "Fixed Arbitration of {:?} while {:?}: next_port: {:?}",
            reqs, last, self.addr_in_port,
        );
        self.forced = None;
        *self.addr_in_port.get_this_cycle()
    }

    fn get_addr_in_port(&self) -> Option<TAG> {
        *self.addr_in_port.get_this_cycle()
    }
}

/// Performs GRANTs in the same cycle
/// It is expected to be called AFTER all input in known for the current cycle
#[derive(Subcomponent, TickComponent, TickComponentExtra, DisableableComponent, Debug)]
pub(crate) struct CombinatorialFixedArbiter<TAG>
where
    TAG: PartialEq + Copy + EnumArray<bool>,
{
    #[flop]
    addr_in_port: BufferFlop<Option<TAG>>,
}

impl<TAG> Default for CombinatorialFixedArbiter<TAG>
where
    TAG: PartialEq + Copy + EnumArray<bool>,
{
    fn default() -> Self {
        Self {
            addr_in_port: BufferFlop::new_from(None),
        }
    }
}

impl<TAG> Arbiter<TAG> for CombinatorialFixedArbiter<TAG>
where
    TAG: PartialEq + Copy + EnumArray<bool> + Debug,
{
    // Designed to answer in the same cycle (combinatorially)
    fn arbitrate(&mut self, reqs: EnumMap<TAG, bool>, last: MasterToSlaveAddrPhase) -> Option<TAG> {
        if (last.ready || !last.meta.is_address_valid()) && last.lock == LOW {
            // if last.HMASTLOCK() == LOW {
            let keep_current = self.addr_in_port.filter(|_| last.meta.is_idle());
            // (last.HREADY || !last.meta.is_address_valid())
            // Top-priority or keep if transfer is active
            self.addr_in_port.set_this_cycle(
                reqs.iter()
                    // Allow fast transition
                    // TODO: this may put unstable data on AddrPhase wires
                    .find(|&(_, &v)| v)
                    .map(|(t, _)| t)
                    .or(keep_current),
            );
        } else {
            self.addr_in_port.keep_current_as_next();
        }
        // Output is lagged!
        trace!(
            "Combinatorial Fixed Arbitration of {:?} while {:?}: next_port: {:?}",
            reqs, last, self.addr_in_port,
        );
        *self.addr_in_port.get_this_cycle()
    }

    fn get_addr_in_port(&self) -> Option<TAG> {
        *self.addr_in_port.get_this_cycle()
    }
}

// XXX: TODO: it only reverses order
#[derive(Subcomponent, TickComponent, TickComponentExtra, DisableableComponent, Debug)]
pub(crate) struct ReversedCombFixedArbiter<TAG>
where
    TAG: PartialEq + Copy + EnumArray<bool>,
{
    #[flop]
    addr_in_port: BufferFlop<Option<TAG>>,
}

impl<TAG> Default for ReversedCombFixedArbiter<TAG>
where
    TAG: PartialEq + Copy + EnumArray<bool>,
{
    fn default() -> Self {
        Self {
            addr_in_port: BufferFlop::new_from(None),
        }
    }
}

impl<TAG> Arbiter<TAG> for ReversedCombFixedArbiter<TAG>
where
    TAG: PartialEq + Copy + EnumArray<bool> + Debug,
{
    // Designed to answer in the same cycle (combinatorially)
    fn arbitrate(&mut self, reqs: EnumMap<TAG, bool>, last: MasterToSlaveAddrPhase) -> Option<TAG> {
        if (last.ready || !last.meta.is_address_valid()) && last.lock == LOW {
            // if last.HMASTLOCK() == LOW {
            let keep_current = self.addr_in_port.filter(|_| last.meta.is_idle());
            // (last.HREADY || !last.meta.is_address_valid())
            // Top-priority or keep if transfer is active
            self.addr_in_port.set_this_cycle(
                reqs.iter()
                    .rev() // XXX: only this change
                    .find(|&(_, &v)| v)
                    .map(|(t, _)| t)
                    .or(keep_current),
            );
        } else {
            self.addr_in_port.keep_current_as_next();
        }

        // Output is lagged!
        trace!(
            "ReversedComb Fixed Arbitration of {:?} while {:?}: next_port: {:?}",
            reqs, last, self.addr_in_port,
        );
        *self.addr_in_port.get_this_cycle()
    }

    fn get_addr_in_port(&self) -> Option<TAG> {
        *self.addr_in_port.get_this_cycle()
    }
}
