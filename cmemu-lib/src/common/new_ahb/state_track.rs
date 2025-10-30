//! Helper for tracking address-data phase transitions
use crate::common::new_ahb::signals::{AhbResponseControl, MasterToSlaveAddrPhase};
#[cfg_attr(not(debug_assertions), allow(unused_imports))]
use crate::utils::IfExpr;

/// Simple and unoptimized helper to track the state of AHB buses for a "through" component.
///
/// In AHB ("full") or inside interconnects, an address phase may:
/// - proceed to a data phase,
/// - `Idle` proceeds to a data phase considered non-existent,
/// - stay in address phase due to low `HREADY` signal,
/// - by denied lines by e.g., arbiters (in this case, the existing data phase finishes).
#[derive(Default, Debug)]
pub(crate) struct AHBStateTrack {
    // Note: this is not Flop-based, as the whole struct is Flop-like
    // write-only
    last_addr: Option<MasterToSlaveAddrPhase>,
    last_reply: Option<AhbResponseControl>,
    last_deny: Option<bool>,

    // This is readable
    data_address: Option<MasterToSlaveAddrPhase>,
}

/// Get information about the result of address and data phases after the edge.
#[derive(Debug)]
pub(crate) struct TransitionInfo {
    /// This means that addr-phase advanced to data phase
    pub advanced: bool,
    /// This means that the last data-phase is done.
    ///
    /// This may be different from `advanced`, if we had active data phase and addr phase,
    /// but the arbiter did not uphold our access to the bus.
    /// It is true if an `Idle` transfer was in the data phase.
    pub finished: bool,
    /// True if a data phase is active (either previous or a new one).
    ///
    /// This excludes `Idle` and other `advanced` states with inactive address lines.
    pub has_data_ph: bool,
}

impl AHBStateTrack {
    pub(crate) fn set_last_addr(&mut self, addr_phase: MasterToSlaveAddrPhase) {
        assert!(!self.is_last_addr_set());
        self.last_addr = Some(addr_phase);
    }
    pub(crate) fn is_last_addr_set(&self) -> bool {
        self.last_addr.is_some()
    }

    pub(crate) fn set_last_reply(&mut self, reply: AhbResponseControl) {
        assert!(!self.is_last_reply_set());
        self.last_reply = Some(reply);
    }
    pub(crate) fn is_last_reply_set(&self) -> bool {
        self.last_reply.is_some()
    }

    pub(crate) fn set_last_deny(&mut self, deny: bool) {
        assert!(!self.is_last_deny_set());
        self.last_deny = Some(deny);
    }
    pub(crate) fn is_last_deny_set(&self) -> bool {
        self.last_deny.is_some()
    }

    /// Get address wires for the current data-phase
    ///
    /// This is either from the `last_addr`, previous `data_address`, or None if not a valid one.
    pub(crate) fn data_address(&self) -> Option<&MasterToSlaveAddrPhase> {
        self.data_address.as_ref()
    }

    /// Update the state of the tracker.
    ///
    /// Consumes input values and produces transition information and
    /// address information for the data-phase in [`Self::data_address`].
    pub(crate) fn update(&mut self) -> TransitionInfo {
        let last_addr = self.last_addr.as_ref();
        let trans_valid = last_addr.is_some_and(|a| a.meta.is_address_valid());
        // HREADY from a slave: no response means Success
        let hready_out_s = self.last_reply.is_none_or(|r| r.HREADYOUT());
        // combine our input HREADY with slave HREADY -- we send this in a reflecting component
        let hready_in_s = hready_out_s && last_addr.is_none_or(|a| a.HREADYIN());

        let trans_valid_rdy = trans_valid && hready_in_s;
        // Missing deny is an acceptance
        let advanced = hready_in_s && !self.last_deny.unwrap_or(false);
        let finished = hready_in_s;
        if advanced && trans_valid_rdy {
            self.data_address = self.last_addr.take();
        } else if finished {
            self.data_address = None;
        }
        self.last_reply = None;
        self.last_addr = None;
        self.last_deny = None;
        TransitionInfo {
            advanced,
            finished,
            has_data_ph: self.data_address.is_some(),
        }
    }

    /// Verify that if we got a low input HREADY, we also sent a low output HREADY.
    #[cfg(debug_assertions)]
    pub(crate) fn assert_hready_is_reflected(&self) {
        let hready_out_s = self.last_reply.is_none_or(|r| r.HREADYOUT());
        let hready_in_s = self.last_addr.as_ref().is_none_or(|a| a.HREADYIN());
        assert!(
            (!hready_out_s).implies(!hready_in_s),
            "HREADYOUT was not properly reflected as HREADYIN wire: {:?} got {:?}",
            self.last_reply,
            self.last_addr
        );
    }

    pub(crate) fn seems_active(&self) -> bool {
        self.last_addr
            .as_ref()
            .is_some_and(|l| l.meta.is_address_valid())
            || self.data_address.is_some()
    }
}
