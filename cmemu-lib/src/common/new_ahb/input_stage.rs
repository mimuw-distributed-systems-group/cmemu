//! Input Stage of an AHB-Lite interconnect may need to store the addr-phase.
//!
//! In AHB-Lite, when the current transfer (possibly `Idle`) has HREADY data-phase response,
//! the addr-phase request will advance to the data-phase.
//! But, there may be contention on the output with another master, so the transfer
//! cannot actually proceed.
//! The role of the [`InputStage`] is to remember the addr-phase request,
//! and reply to the master with waitstates, while retrying the request on the slave side.
use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;

use log::trace;

use crate::common::new_ahb::ports::{
    AHBMasterPortInput, AHBMasterPortOutput, AHBPortConfig, AHBSlavePortInput, AHBSlavePortOutput,
    AhbMasterPortInputWithGranting,
};
use crate::common::new_ahb::signals::{
    AhbResponseControl, MasterToSlaveWires, SlaveToMasterWires, TrackedBool, TransferType,
};
use crate::common::new_ahb::state_track::{AHBStateTrack, TransitionInfo};
use crate::common::utils::SubcomponentProxyMut;
use crate::engine::{
    Context, DisableableComponent, StateMachine, Subcomponent, TickComponent, TickComponentExtra,
};
#[cfg(feature = "cycle-debug-logger")]
use crate::proxy::CycleDebugLoggerProxy;
#[cfg_attr(not(debug_assertions), allow(unused_imports))]
use crate::utils::IfExpr;

pub(crate) mod transparent;

/// The AHB-Lite component to possibly remember transfer rejected by inner interconnect
///
/// Main invariants upheld:
/// - promises upholding main CMEmu AHB invariants
/// - if the upstream transfer advanced, we inject waitstates
#[derive(Subcomponent, TickComponent, DisableableComponent)]
pub(crate) struct InputStage<SC>
where
    SC: Subcomponent<Member = InputStage<SC>>,
    InputStage<SC>: AhbMasterPortInputWithGranting,
    InputStage<SC>: AHBSlavePortOutput<Component = SC::Component>,
{
    /// Status of the interconnect deciding the operation in this cycle
    state: InputState,
    /// Captured addr-phase that has advanced from the AHB-Lite side,
    /// but on from the interconnect side.
    upstream_track: AHBStateTrack,
    /// `Some` if GRANT wire was received during the cycle
    addr_phase_rejected: Option<bool>,

    phantom_sc: PhantomData<SC>,
}

#[derive(Default, Debug, PartialEq, Clone, Copy)]
enum InputState {
    /// No active transfer
    #[default]
    Idle,
    /// The addr/data parts are directly connected to the output
    Transparent,
    /// The addr-phase is retried while it advanced from the master perspective,
    /// so we drive the data-phase wires.
    Buffer,
    /// The `InputStage` needs to generate ready responses to keep main CMEmu AHB invariants
    /// (e.g., after `Idle`).
    Terminator,
}
impl StateMachine for InputState {}

impl<SC> Default for InputStage<SC>
where
    SC: Subcomponent<Member = Self>,
    Self: AHBMasterPortOutput,
    Self: AHBSlavePortOutput<Component = SC::Component>,
{
    fn default() -> Self {
        Self {
            state: InputState::Idle,
            upstream_track: Default::default(),
            addr_phase_rejected: None,

            phantom_sc: PhantomData,
        }
    }
}

impl<SC> TickComponentExtra for InputStage<SC>
where
    SC: Subcomponent<Member = Self>,
    Self: AhbMasterPortInputWithGranting,
    Self: AHBSlavePortOutput<Component = SC::Component>,
{
    #[cfg(debug_assertions)]
    fn tick_assertions(&self) {
        assert!(
            (self.state == InputState::Buffer).implies(self.upstream_track.is_last_addr_set()),
            "{self}: Was buffering, but master did not send a message while having an active data phase."
        );

        assert!(
            self.upstream_track
                .data_address()
                .is_some()
                .implies(self.upstream_track.is_last_addr_set()),
            "{self}: master did not send a message while having an active data phase."
        );
    }

    #[allow(clippy::match_same_arms)]
    fn tick_extra(&mut self) {
        trace!(
            "{} in upstr: {:?} deny: {:?} was: {:?}",
            self, self.upstream_track, self.addr_phase_rejected, self.state,
        );
        let had_request = self.upstream_track.is_last_addr_set();
        let TransitionInfo {
            advanced,
            has_data_ph,
            ..
        } = self.upstream_track.update();
        self.state = match (advanced, has_data_ph, self.addr_phase_rejected, self.state) {
            // Keep replying from the buffer
            (false, true, Some(true), InputState::Buffer) | (true, true, Some(true), _) => {
                InputState::Buffer
            }
            (_, true, _, _) => InputState::Transparent,
            (true, false, None | Some(false), _) if had_request => InputState::Terminator,
            (_, false, None, _) => InputState::Idle,
            state_tup => panic!(
                "Invalid state of {}: {:?}, buf: {:?}",
                self, state_tup, self.upstream_track
            ),
        };

        self.addr_phase_rejected = None;
        trace!(
            "{} in state {:?} (data_valid: {:?}) upstr-data: {:?}",
            self,
            self.state,
            has_data_ph,
            self.upstream_track.data_address()
        );
    }
}

impl<SC> AHBSlavePortInput for InputStage<SC>
where
    SC: Subcomponent<Member = Self>,
    Self: AhbMasterPortInputWithGranting,
    Self: AHBSlavePortOutput<Component = SC::Component> + AHBMasterPortOutput,
{
    fn on_ahb_input(
        comp: &mut Self::Component,
        ctx: &mut Context,
        msg: MasterToSlaveWires<Self::Data>,
    ) {
        // TODO: fix burst transfers (see [ARM-AHB-SDK] 5.1.6
        let mut this = SubcomponentProxyMut::<SC>::from(comp);

        let MasterToSlaveWires {
            addr_phase,
            data_phase,
        } = msg;

        this.upstream_track.set_last_addr(addr_phase.clone());

        debug_assert!(
            !matches!(addr_phase.meta, TransferType::_Busy),
            "unsupported transfer type"
        );

        // Decide, which addr-phase to take.
        let out_addr_phase = if this.state == InputState::Buffer {
            this.upstream_track.data_address().unwrap().clone()
        } else {
            addr_phase
        };

        trace!(
            "{} with phase status {:?} mutated req to {:?}",
            *this,
            this.upstream_track.data_address().is_some(),
            out_addr_phase
        );
        // Data phase is always untouched -> it could go straight to the decoder
        let msg_with_proper_addr = MasterToSlaveWires {
            addr_phase: out_addr_phase,
            data_phase,
        };

        <Self as AHBMasterPortOutput>::send_ahb_output(
            this.component_mut(),
            ctx,
            msg_with_proper_addr,
        );
    }
}

impl<SC> AHBMasterPortInput for InputStage<SC>
where
    SC: Subcomponent<Member = Self>,
    Self: AhbMasterPortInputWithGranting,
    Self: AHBSlavePortOutput<Component = SC::Component>,
    <Self as AHBPortConfig>::Data: Debug,
{
    fn on_ahb_input(
        comp: &mut Self::Component,
        ctx: &mut Context,
        msg: SlaveToMasterWires<Self::Data>,
    ) {
        let mut this = SubcomponentProxyMut::<SC>::from(comp);

        if matches!(this.state, InputState::Buffer | InputState::Terminator) {
            trace!(
                "{} got reply, but already sent default! msg: {:?}",
                *this, msg
            );
            debug_assert!(
                msg.meta.is_done(),
                "{} sent default reply, but it received non-default msg from slave: {:?}; upstr track: {:?}",
                *this,
                msg,
                this.upstream_track,
            );
            return;
        }
        trace!("{} Forwarding reply: {:?}", *this, msg);

        Self::send_reply(this.component_mut(), ctx, msg);
    }
}

impl<SC> AhbMasterPortInputWithGranting for InputStage<SC>
where
    SC: Subcomponent<Member = Self>,
    Self: AHBMasterPortOutput,
    Self: AHBSlavePortOutput<Component = SC::Component>,
{
    fn on_grant_wire(
        comp: &mut Self::Component,
        #[allow(unused)] ctx: &mut Context,
        granted: TrackedBool,
    ) {
        let mut this = SubcomponentProxyMut::<SC>::from(comp);

        this.addr_phase_rejected = Some(!*granted);

        if !*granted {
            trace!("Input {} got a deny!", *this);
            #[cfg(feature = "cycle-debug-logger")]
            CycleDebugLoggerProxy.on_free_static_str(
                ctx,
                <Self as AHBPortConfig>::get_name(),
                "DENIED",
            );
        }
    }
}

impl<SC> InputStage<SC>
where
    SC: Subcomponent<Member = Self>,
    Self: AhbMasterPortInputWithGranting,
    Self: AHBSlavePortOutput<Component = SC::Component>,
    <Self as AHBPortConfig>::Data: Debug,
{
    fn send_reply(
        comp: &mut SC::Component,
        ctx: &mut Context,
        msg: SlaveToMasterWires<<Self as AHBPortConfig>::Data>,
    ) {
        let mut this = SC::get_proxy(comp);
        this.upstream_track.set_last_reply(msg.meta);
        <Self as AHBSlavePortOutput>::send_ahb_output(this.component_mut(), ctx, msg);
    }

    pub(super) fn generate_default_output(
        comp: &mut <Self as AHBPortConfig>::Component,
        ctx: &mut Context,
    ) {
        let mut this = SC::get_proxy(comp);
        let msg = if this.state == InputState::Terminator {
            // Nobody cared about our data, since it was idle/busy -> just ack it
            trace!(
                "{}: Input stage self-generating response for Idle/Busy ",
                *this
            );
            // Sorry, but we disregarded who sent this packet
            Some(SlaveToMasterWires {
                ..this.upstream_track.data_address().map_or_else(
                    SlaveToMasterWires::empty::<Self>,
                    SlaveToMasterWires::empty_addr_reply::<Self>,
                )
            })
        } else if this.state == InputState::Buffer {
            // Typically, response to our buffered request would be sent, but since we are
            // buffering it, it means it won't
            trace!(
                "{}: Input stage buffering response to hold the master",
                *this
            );
            Some(
                this.upstream_track
                    .data_address()
                    .unwrap()
                    .make_reply::<Self, _>(
                        AhbResponseControl::Pending,
                        <Self as AHBPortConfig>::Data::default(),
                    ),
            )
        } else {
            None
        };

        if let Some(msg) = msg {
            Self::send_reply(this.component_mut(), ctx, msg);
        }
    }

    #[allow(dead_code)]
    pub(crate) fn tick(_comp: &mut <SC as Subcomponent>::Component, _ctx: &mut Context) {}
    pub(crate) fn tock(comp: &mut <SC as Subcomponent>::Component, ctx: &mut Context) {
        Self::generate_default_output(comp, ctx);
    }
}

impl<SC> Display for InputStage<SC>
where
    SC: Subcomponent<Member = Self>,
    Self: AhbMasterPortInputWithGranting,
    Self: AHBSlavePortOutput<Component = SC::Component>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <Self as AHBPortConfig>::TAG)
    }
}
