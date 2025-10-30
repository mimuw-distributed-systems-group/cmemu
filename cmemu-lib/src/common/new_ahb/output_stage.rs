//! Output Stage part of the interconnect
//!
//! This component has multiple masters and a single slave, so it acts
//! as an arbiter.
//! It is not directly usable outside the interconnect,
//! because it is not pure AHB-Lite, which lacks GRANT wires.
use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;

use enum_map::{EnumMap, enum_map};
use log::trace;

use crate::common::new_ahb::arbiter::Arbiter;
use crate::common::new_ahb::ports::{
    AHBMasterPortInput, AHBMasterPortOutput, AHBPortConfig, AHBSlavePortInput,
};
use crate::common::new_ahb::signals::{
    BinaryWire, MasterToSlaveAddrPhase, MasterToSlaveDataPhase, MasterToSlaveWires,
    SlaveToMasterWires, TrackedBool,
};
use crate::common::new_ahb::vlan::{
    AHBSlavePortTaggedInput, AhbMultiMasterConfig, AhbSlaveOutputDispatcher,
};
use crate::common::pending::Pending;
use crate::common::utils::{FromMarker, SubcomponentProxyMut};
#[cfg(debug_assertions)]
use crate::engine::StateMachine;
use crate::engine::{
    CombFlop, Context, DisableableComponent, Subcomponent, TickComponent, TickComponentExtra,
    debug_move_state_machine,
};
use crate::make_port_struct;

pub(crate) mod combinatorial_os;

make_port_struct!(pub(crate) AhbPort<SC, PM>);

// TODO: support locked transfers
// TODO: support breaking burst transfers
// TODO: support masters passing 1KB boundary with locked transfers

/// Output Stage part of the interconnect
///
/// It has a known set of masters represented in `AhbMultiMasterConfig::MastersEnum`.
/// Only one master may be connected to the data phase, but many may want to get to the addr phase.
/// The internal arbiter is responsible for deciding which *addr phase* will go through.
/// This is done sequentially, that is the arbitration happens in `tick` and depends on messages
/// sent in the previous cycle.
///
/// It holds the following invariants:
/// - only one (or no) message is sent to the slave,
/// - at most one master will think its address phase advanced:
///   - if it provided low HREADYIN, they won't consider it advanced,
///   - if its addr-phase is not routed to the outside, it will get a DENY comb. grant response,
///   - if the slave returns low HREADY, the addr-route master will get a DENY too.
#[derive(Subcomponent, TickComponent, DisableableComponent)]
pub(crate) struct OutputStage<SC>
where
    SC: Subcomponent<Member = OutputStage<SC>>,
    OutputStage<SC>: AhbMultiMasterConfig,
    OutputStage<SC>: AHBMasterPortOutput<Component = SC::Component>,
{
    #[subcomponent]
    arbiter: <Self as AhbMultiMasterConfig>::Arbiter,

    /// Master tag that holds an active data route.
    /// `None` means nobody
    #[flop]
    data_route: CombFlop<Option<<Self as AhbMultiMasterConfig>::MastersEnum>>,
    /// Held temporarily to sync with the addr phase
    data_buffer: Option<MasterToSlaveDataPhase<<Self as AHBPortConfig>::Data>>,

    /// Used both to sync with the data phase, and to know in `tick` what was actually sent.
    out_addr: Option<MasterToSlaveAddrPhase>,
    /// Arbitration requests from masters.
    /// If any M2S message was sent, this is `Some`.
    /// Inside it means whether the master message actually needs the arbiter
    /// (i.e. has a valid addr phase)
    requests: EnumMap<<Self as AhbMultiMasterConfig>::MastersEnum, Option<bool>>,

    /// Our slave response
    reply: Pending<BinaryWire>,

    #[cfg(debug_assertions)]
    stm: OutputStageSTM,
    phantom_sc: PhantomData<SC>,
}

#[cfg(debug_assertions)]
#[derive(Debug, PartialEq)]
enum OutputStageSTM {
    Tick,
    // Strict ordering has replies before responses
    GotReply,
    // We get here if we have a part of the final message (addr phase + data phase)
    Collecting,
    // The message was sent
    DeliveredToSlave,
}
#[cfg(debug_assertions)]
impl StateMachine for OutputStageSTM {}

impl<SC> Default for OutputStage<SC>
where
    SC: Subcomponent<Member = Self>,
    Self: AhbMultiMasterConfig,
    Self: AHBMasterPortOutput<Component = SC::Component>,
{
    fn default() -> Self {
        Self {
            arbiter: Default::default(),
            data_route: CombFlop::new_from(None),
            data_buffer: None,
            out_addr: None,
            requests: enum_map! {_ => None},
            reply: Default::default(),
            #[cfg(debug_assertions)]
            stm: OutputStageSTM::Tick,
            phantom_sc: Default::default(),
        }
    }
}

impl<SC> TickComponentExtra for OutputStage<SC>
where
    SC: Subcomponent<Member = Self>,
    Self: AhbMultiMasterConfig,
    Self: AHBMasterPortOutput<Component = SC::Component>,
{
    #[cfg(debug_assertions)]
    fn tick_assertions(&self) {
        assert!(
            self.reply.is_ready(),
            "{self}: waited for slave reply to no avail",
        );
        assert!(
            self.arbiter.get_addr_in_port().is_none()
                || self.out_addr.is_some()
                || self.stm == OutputStageSTM::Tick
                || (self.stm == OutputStageSTM::GotReply && *self.reply),
            "{self}: Not received addr_phase message after granting access to {:?} (stm {:?}, trans: {:?}).",
            self.arbiter.get_addr_in_port(),
            self.stm,
            self.out_addr
        );

        // Note: we won't move this if no port is active
        assert_ne!(
            self.stm,
            OutputStageSTM::Collecting,
            "{self}: Was stuck with collecting data (Data phase missing?) {:?} trans {:?}",
            self.data_route,
            self.out_addr
        );
        assert!(
            self.data_buffer.is_none(),
            "{self}: Data buffer was not consumed!",
        );
        if !self.reply.or(true) {
            // TODO: decide if we want this assert!
            // In this case, we send DENY to that master, so nobody thinks the transfer advanced
            // assert!(
            //     !self.out_addr.as_ref().unwrap().ready,
            //     "{self}: HREADY was not properly reflected last cycle: {:?}",
            //     self.out_addr,
            // );
        } else if self.out_addr.is_none() {
            // data_route may be not touched
            self.data_route.map_or((), |_| ());
        }
    }

    fn tick_extra(&mut self) {
        #[cfg(debug_assertions)]
        {
            self.stm = OutputStageSTM::Tick;
        }

        self.reply.clear_set_expecting(self.data_route.is_some());
        let out_addr = self
            .out_addr
            .take()
            .unwrap_or_else(MasterToSlaveAddrPhase::empty::<Self>);
        let reqs = enum_map! {
            tag => self.requests[tag].take().unwrap_or(false)
        };
        self.arbiter.arbitrate(reqs, out_addr);

        self.data_route.set_next(None);
    }
}

// Convert type-based connections to tag-based input-output.
impl<SC, M> AHBSlavePortInput for AhbPort<SC, M>
where
    SC: Subcomponent<Member = OutputStage<SC>>,
    Self: AHBPortConfig<Component = SC::Component, Data = <OutputStage<SC> as AHBPortConfig>::Data>,
    OutputStage<SC>: AhbMultiMasterConfig,
    OutputStage<SC>: AHBMasterPortOutput<Component = SC::Component>,
    OutputStage<SC>:
        AhbSlaveOutputDispatcher<<OutputStage<SC> as AhbMultiMasterConfig>::MastersEnum>,
    <OutputStage<SC> as AhbMultiMasterConfig>::MastersEnum: FromMarker<M>,
{
    fn on_ahb_input(
        comp: &mut Self::Component,
        ctx: &mut Context,
        msg: MasterToSlaveWires<Self::Data>,
    ) {
        OutputStage::<SC>::on_ahb_tagged_input(comp, ctx, FromMarker::<M>::from_marker(), msg);
    }
}

impl<SC> AHBSlavePortTaggedInput for OutputStage<SC>
where
    SC: Subcomponent<Member = Self>,
    Self: AhbMultiMasterConfig,
    Self: AHBMasterPortOutput<Component = SC::Component>,
    Self: AhbSlaveOutputDispatcher<<Self as AhbMultiMasterConfig>::MastersEnum>,
{
    fn on_ahb_tagged_input(
        comp: &mut Self::Component,
        ctx: &mut Context,
        tag: Self::MastersEnum,
        msg: MasterToSlaveWires<Self::Data>,
    ) {
        let mut this = SubcomponentProxyMut::<SC>::from(comp);

        let is_sel = msg.addr_phase.is_selected();
        let wants_arbiter = msg.addr_phase.advances_to_valid();
        let addr_route = this.arbiter.get_addr_in_port();
        let data_route = *this.data_route;
        this.requests[tag] = Some(wants_arbiter);

        trace!(
            "{} with (A->{:?}, D->{:?}) from {:?} ({:} wants arbiter) got {:?}",
            *this,
            addr_route,
            data_route,
            tag,
            if wants_arbiter { "which" } else { "DOESN'T" },
            msg,
        );
        if addr_route == Some(tag) {
            // We forward low HREADY / NoSel in this case

            this.out_addr = Some(msg.addr_phase);
            Self::try_send_output(this.component_mut(), ctx);
        }
        // TODO: what was the goal of the second condition?
        // NOTE: this.reply is always set if it should be, as the LiteWrapper guarantees ordering!
        if (wants_arbiter && addr_route != Some(tag)) || (is_sel && !this.reply.or(true)) {
            <Self as AhbSlaveOutputDispatcher<_>>::on_grant_wire(
                this.component_mut(),
                ctx,
                tag,
                TrackedBool::false_::<Self>(),
            );
        } else if wants_arbiter {
            <Self as AhbSlaveOutputDispatcher<_>>::on_grant_wire(
                this.component_mut(),
                ctx,
                tag,
                TrackedBool::true_::<Self>(),
            );
        }

        if data_route == Some(tag) {
            this.data_buffer = Some(msg.data_phase);
            Self::try_send_output(this.component_mut(), ctx);
        }
    }
}

impl<SC> AHBMasterPortInput for OutputStage<SC>
where
    SC: Subcomponent<Member = Self>,
    Self: AhbMultiMasterConfig,
    Self: AHBMasterPortOutput<Component = SC::Component>,
    Self: AhbSlaveOutputDispatcher<<Self as AhbMultiMasterConfig>::MastersEnum>,
    <Self as AHBPortConfig>::Data: Debug,
{
    fn on_ahb_input(
        comp: &mut Self::Component,
        ctx: &mut Context,
        msg: SlaveToMasterWires<Self::Data>,
    ) {
        let mut this = SC::get_proxy(comp);
        let tag = *this.data_route;
        if let Some(tag) = tag {
            debug_move_state_machine!(this.stm => OutputStageSTM::Tick => OutputStageSTM::GotReply);

            let hready = msg.meta.HREADYOUT();
            if !hready {
                this.data_route.keep_current_as_next();
            }
            this.reply.supply(hready);
            trace!("{} forwarded reply to {:?}: {:?}", *this, tag, msg,);
            // We need dispatch, as the source master is known only at runtime
            <Self as AhbSlaveOutputDispatcher<_>>::dispatch_ahb_output(
                this.component_mut(),
                ctx,
                tag,
                msg,
            );
        }
    }
}

impl<SC> Display for OutputStage<SC>
where
    SC: Subcomponent<Member = OutputStage<SC>>,
    OutputStage<SC>: AhbMultiMasterConfig,
    OutputStage<SC>: AHBMasterPortOutput<Component = SC::Component>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <Self as AHBPortConfig>::TAG)
    }
}

impl<SC> OutputStage<SC>
where
    SC: Subcomponent<Member = Self>,
    Self: AhbMultiMasterConfig,
    Self: AhbSlaveOutputDispatcher<<Self as AhbMultiMasterConfig>::MastersEnum>,
    Self: AHBMasterPortOutput<Component = SC::Component>,
{
    /// Send output downstream if we have messages from both addr and data-phase masters.
    pub(crate) fn try_send_output(comp: &mut SC::Component, ctx: &mut Context) {
        let mut this = SubcomponentProxyMut::<SC>::from(comp);
        debug_move_state_machine!(this.stm => OutputStageSTM = {Tick | GotReply | Collecting => Collecting});

        // Are we there yet?
        let addr_route = this.arbiter.get_addr_in_port();
        if this.data_route.is_some() != this.data_buffer.is_some()
            || addr_route.is_some() != this.out_addr.is_some()
        {
            return;
        }

        debug_move_state_machine!(this.stm => OutputStageSTM::Collecting => OutputStageSTM::DeliveredToSlave);

        if this.data_route.is_none() && addr_route.is_none() {
            trace!(
                "{}: Output stage has nothing to do (no output required)",
                *this,
            );
            return;
        }

        let addr_phase = if addr_route.is_some() {
            // pass the original, store a copy
            let cloned = this.out_addr.clone().unwrap();
            this.out_addr.replace(cloned).unwrap()
        } else {
            MasterToSlaveAddrPhase::empty::<Self>()
        };

        if addr_phase.meta.is_address_valid() {
            this.data_route.set_next_if_not_latching(addr_route);
        } else if !addr_phase.is_selected() && this.data_route.is_none() {
            trace!(
                "{}: Output stage has nothing to do (not forwarding NoSel without data phase)",
                *this,
            );
            return;
        }

        let data_phase = if this.data_route.is_some() {
            this.data_buffer.take().expect("Data phase missing?")
        } else {
            MasterToSlaveDataPhase::empty::<Self>()
        };

        let msg = MasterToSlaveWires {
            addr_phase,
            data_phase,
        };

        trace!(
            "{}: routing addr_route: {:?}, data_route: {:?}",
            *this, addr_route, this.data_route
        );
        <Self as AHBMasterPortOutput>::send_ahb_output(this.component_mut(), ctx, msg);
    }

    pub(crate) fn get_arbiter_unchecked(&mut self) -> &mut <Self as AhbMultiMasterConfig>::Arbiter {
        &mut self.arbiter
    }
}
