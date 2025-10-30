//! [`OutputStage`] implementation that decides arbitration in the same cycle as the transfers come
use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;

use enum_map::{EnumMap, enum_map};
use log::trace;
use owo_colors::OwoColorize;

#[cfg(debug_assertions)]
use super::OutputStageSTM;
use crate::common::new_ahb::arbiter::Arbiter;
use crate::common::new_ahb::ports::AhbMasterPortInputWithGranting;
use crate::common::new_ahb::ports::{
    AHBMasterPortInput, AHBMasterPortOutput, AHBPortConfig, AHBSlavePortInput,
};
use crate::common::new_ahb::signals::{
    AhbResponseControl, MasterToSlaveAddrPhase, MasterToSlaveDataPhase, MasterToSlaveWires,
    SlaveToMasterWires, TrackedBool,
};
use crate::common::new_ahb::state_track::{AHBStateTrack, TransitionInfo};
use crate::common::new_ahb::vlan::{
    AHBSlavePortTaggedInput, AhbMultiMasterConfig, AhbSlaveOutputDispatcher,
};
use crate::common::utils::{FromMarker, SubcomponentProxyMut};
use crate::confeature::cm_hyp;
use crate::engine::{
    BufferFlop, CombFlop, Context, DisableableComponent, Subcomponent, TickComponent,
    TickComponentExtra,
};
use crate::utils::{IfExpr, deref};
use crate::{debug_move_state_machine, make_port_struct};

make_port_struct!(pub(crate) AhbPort<SC, PM>);

/// A combinatorial output stage implementation that decides arbitration in the same cycle.
#[derive(Subcomponent, TickComponent, DisableableComponent)]
pub(crate) struct OutputStage<SC>
where
    SC: Subcomponent<Member = OutputStage<SC>>,
    OutputStage<SC>: AhbMultiMasterConfig,
    OutputStage<SC>: AHBMasterPortOutput<Component = SC::Component>,
{
    #[subcomponent]
    arbiter: <Self as AhbMultiMasterConfig>::Arbiter,

    // None -> no_port
    #[flop]
    data_route: CombFlop<Option<<Self as AhbMultiMasterConfig>::MastersEnum>>,
    data_buffer: Option<MasterToSlaveDataPhase<<Self as AHBPortConfig>::Data>>,
    read_in_data: bool,
    write_in_data: bool,
    #[flop]
    pipelining_locked: BufferFlop<bool>,

    requests: EnumMap<<Self as AhbMultiMasterConfig>::MastersEnum, Option<MasterToSlaveAddrPhase>>,

    down_track: AHBStateTrack,
    #[flop]
    addr_phase_out: BufferFlop<MasterToSlaveAddrPhase>,
    #[flop]
    last_reply: BufferFlop<AhbResponseControl>,

    #[cfg(debug_assertions)]
    stm: OutputStageSTM,
    phantom_sc: PhantomData<SC>,
}

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
            read_in_data: false,
            write_in_data: false,
            data_buffer: None,
            pipelining_locked: BufferFlop::new(),
            down_track: AHBStateTrack::default(),
            addr_phase_out: BufferFlop::new(),
            requests: enum_map! {_ => None},
            last_reply: Default::default(),
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
        // assert!(
        //     self.arbiter.get_addr_in_port().is_none()
        //         || self.addr_phase_out.is_some()
        //         || self.stm == OutputStageSTM::Tick
        //     "{}: Not received addr_phase message after granting access to {:?} (stm {:?}, trans: {:?}).",
        //     self, self.arbiter.get_addr_in_port(), self.stm, self.trans
        // );

        assert_ne!(
            self.stm,
            OutputStageSTM::Collecting,
            "{self}: Was stuck with collecting data (Data phase missing?) {:?} trans {:?}",
            self.data_route,
            self.down_track,
        );
        assert!(
            self.data_buffer.is_none(),
            "{self}: Data buffer was not consumed!"
        );
    }

    fn tick_extra(&mut self) {
        #[cfg(debug_assertions)]
        {
            self.stm = OutputStageSTM::Tick;
        }

        self.requests.clear();
        // TODO: retire these fields in favor of the track
        self.last_reply.allow_skip();
        // may be overwritten
        self.data_route.set_default_next(None);
        let TransitionInfo { finished, .. } = self.down_track.update();

        // Update if we wait for an STR to finish in this cycle
        if finished {
            if self.pipelining_locked.is_set_and(deref) {
                self.read_in_data = false;
                self.write_in_data = false;
            } else {
                // just advanced, change
                self.read_in_data = self
                    .down_track
                    .data_address()
                    .is_some_and(|aph| !aph.meta.is_writing());
                self.write_in_data = self
                    .down_track
                    .data_address()
                    .is_some_and(|aph| aph.meta.is_writing());
            }
        }
        self.pipelining_locked.allow_skip();
    }
}

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
impl<SC> AhbMasterPortInputWithGranting for OutputStage<SC>
where
    SC: Subcomponent<Member = Self>,
    Self: AhbMultiMasterConfig,
    Self: AHBMasterPortOutput<Component = SC::Component>,
    Self: AhbSlaveOutputDispatcher<<Self as AhbMultiMasterConfig>::MastersEnum>,
{
    #[allow(unused_variables, unreachable_code)]
    fn on_grant_wire(comp: &mut Self::Component, ctx: &mut Context, granted: TrackedBool) {
        todo!("We need a IS_GRANTER configuration");
        // XXX: This is never called? Or only from the following WriteBuffer?
        //  We need to know whether we should return the positive grant or delegate this downstream
        let mut this = SC::get_proxy(comp);
        this.down_track.set_last_deny(!*granted);
        let upstream_tag = this
            .arbiter
            .get_addr_in_port()
            .expect("Grant wire without source");
        this.pipelining_locked.set_next(!*granted);

        <Self as AhbSlaveOutputDispatcher<_>>::on_grant_wire(
            this.component_mut(),
            ctx,
            upstream_tag,
            granted,
        );
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
        _ctx: &mut Context,
        tag: Self::MastersEnum,
        msg: MasterToSlaveWires<Self::Data>,
    ) {
        let mut this = SubcomponentProxyMut::<SC>::from(comp);

        debug_move_state_machine!(this.stm => OutputStageSTM = {Tick | GotReply | Collecting => Collecting});

        let wants_arbiter = msg.addr_phase.advances_to_valid();
        let data_route = *this.data_route;

        trace!(
            "{} with (, D->{:?}) from {:?} ({:} wants arbiter) got {:?}",
            *this,
            data_route,
            tag,
            if wants_arbiter { "which" } else { "DOESN'T" },
            msg,
        );

        // TODO: rewrite HTRANS on broken bursts
        this.requests[tag] = Some(msg.addr_phase);

        if data_route == Some(tag) {
            this.data_buffer = Some(msg.data_phase);
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
        let mut this = SubcomponentProxyMut::<SC>::from(comp);
        this.down_track.set_last_reply(msg.meta);
        this.last_reply.set_this_cycle(msg.meta);
        let tag = *this.data_route;
        if let Some(tag) = tag {
            let hready = msg.meta.HREADYOUT();
            if !hready {
                this.data_route.keep_current_as_next();
                // XXX: we need to latch addr_route
            }
            // symmetrical code if response is first
            if this.addr_phase_out.has_this_cycle()
                && this.addr_phase_out.get_this_cycle().meta.is_address_valid()
                && this.arbiter.get_addr_in_port().unwrap() != tag
                && !this.pipelining_locked.try_this_cycle().is_some_and(deref)
            {
                let tag = this.arbiter.get_addr_in_port().unwrap();
                // XXX: TODO: what to do with Error1 here? We should err them?
                if !hready {
                    trace!(
                        "{} access to {:?} for {:?}",
                        "DENYING BY RESPONSE".bright_red(),
                        tag,
                        this.addr_phase_out.get_this_cycle()
                    );
                    <Self as AhbSlaveOutputDispatcher<_>>::on_grant_wire(
                        this.component_mut(),
                        ctx,
                        tag,
                        TrackedBool::false_from_s2m::<Self, _>(&msg),
                    );
                } else {
                    // This should be OK, since we cannot have a valid addr phase if it is not expected to advance
                    trace!(
                        "{} access to {:?} for {:?}",
                        "ALLOW BY RESPONSE".bright_green(),
                        tag,
                        this.addr_phase_out.get_this_cycle()
                    );
                    <Self as AhbSlaveOutputDispatcher<_>>::on_grant_wire(
                        this.component_mut(),
                        ctx,
                        tag,
                        TrackedBool::true_from_s2m::<Self, _>(&msg),
                    );
                    <Self as AhbSlaveOutputDispatcher<_>>::dispatch_ahb_output(
                        this.component_mut(),
                        ctx,
                        tag,
                        SlaveToMasterWires::empty::<Self>(),
                    );
                }
            }
            trace!("{} forwarded reply to {:?}: {:?}", *this, tag, msg,);
            <Self as AhbSlaveOutputDispatcher<_>>::dispatch_ahb_output(
                this.component_mut(),
                ctx,
                tag,
                msg,
            );
        }
    }
}

impl<SC> OutputStage<SC>
where
    SC: Subcomponent<Member = Self>,
    Self: AhbMultiMasterConfig,
    Self: AhbSlaveOutputDispatcher<<Self as AhbMultiMasterConfig>::MastersEnum>,
    Self: AHBMasterPortOutput<Component = SC::Component>,
{
    pub(crate) fn send_output(comp: &mut SC::Component, ctx: &mut Context) {
        let mut this = SubcomponentProxyMut::<SC>::from(comp);
        debug_move_state_machine!(this.stm => OutputStageSTM = {Tick | GotReply | Collecting => DeliveredToSlave});

        // TODO: check HREADY as well?
        let reqs = enum_map! {
            tag => this.requests[tag].as_ref().is_some_and(|a| a.meta.is_address_valid())
        };
        let prev_out_addr_phase = this
            .addr_phase_out
            .try_take()
            .unwrap_or(MasterToSlaveAddrPhase::empty::<Self>());
        let prev_cycle_hready = this.last_reply.map_or(true, |r| r.is_done());
        // See bus_matrix::interconnect for explanation
        let can_pipeline = prev_cycle_hready && this.pipelining_locked.map_or(true, |v| !*v); // && !this.write_in_data;
        let mut addr_route = this
            .arbiter
            .arbitrate(reqs, prev_out_addr_phase.with_hready(can_pipeline));

        // XXX: Arbiter may select something even if not requested? (difference between seq and comb arbiters)
        if !addr_route.is_some_and(|tag| {
            this.requests[tag]
                .as_ref()
                .is_some_and(|a| a.meta.is_address_valid())
        }) {
            addr_route = None;
        }

        debug_assert!(
            !this.data_route.is_some() || this.data_buffer.is_some(),
            "Missing needed data from {:?}",
            this.data_route
        );
        debug_assert!(this.read_in_data.implies(this.data_route.is_some()));
        if this.data_route.is_none() && addr_route.is_none() {
            trace!(
                "{}: Output stage has nothing to do (no output required)",
                *this,
            );
            // Nothing to do HERE
            return;
        }

        // TODO: support masters passing 1KB boundary with locked transfers
        let addr_phase = if let Some(addr_route) = addr_route {
            let aph = this.requests[addr_route]
                .take()
                .unwrap_or(MasterToSlaveAddrPhase::empty::<Self>());

            // XXX: this hack should  probably be implemented in the write buffer, bcoz after first cycle of write, we may be still waiting there
            // Simulate DENY for LDR after STR from the write buffer
            let is_pipe_blocked = if *cm_hyp::busmatrix::SYSTEM_WB_DENIES_LDR_AFTER_STR
                && <Self as AHBPortConfig>::TAG.contains("System")
            {
                this.write_in_data && aph.meta.is_address_valid() && !aph.meta.is_writing()
            } else {
                false
            };

            if is_pipe_blocked {
                trace!(
                    "{}: {} from {:?} due to write",
                    *this,
                    "STOPPING LDR".bright_red(),
                    addr_route
                );
                this.pipelining_locked.set_next(true);
                <Self as AhbSlaveOutputDispatcher<_>>::on_grant_wire(
                    this.component_mut(),
                    ctx,
                    addr_route,
                    TrackedBool::false_::<Self>(),
                );
                MasterToSlaveAddrPhase::empty::<Self>()
            } else if this.last_reply.has_this_cycle()
                && aph.meta.is_address_valid()
                && Some(addr_route) != *this.data_route
            {
                let tag = addr_route;
                if this.last_reply.get_this_cycle().is_waitstate() {
                    // XXX: TODO: what to do with Error1 here? We should err them?
                    trace!(
                        "{} access to {:?} for {:?}",
                        "DENYING BY RESPONSE".bright_red(),
                        tag,
                        aph
                    );
                    <Self as AhbSlaveOutputDispatcher<_>>::on_grant_wire(
                        this.component_mut(),
                        ctx,
                        tag,
                        TrackedBool::false_::<Self>(),
                    );
                } else {
                    // This should be OK, since we cannot have a valid addr phase if it is not expected to advance
                    trace!(
                        "{} access to {:?} for {:?}",
                        "ALLOW BY RESPONSE".bright_green(),
                        tag,
                        aph,
                    );
                    <Self as AhbSlaveOutputDispatcher<_>>::on_grant_wire(
                        this.component_mut(),
                        ctx,
                        tag,
                        TrackedBool::true_::<Self>(),
                    );
                    <Self as AhbSlaveOutputDispatcher<_>>::dispatch_ahb_output(
                        this.component_mut(),
                        ctx,
                        tag,
                        SlaveToMasterWires::empty::<Self>(),
                    );
                }
                aph
            } else if this.data_route.is_none() || Some(addr_route) == *this.data_route {
                // TODO: we need the IS_GRANTER field
                trace!(
                    "{} access to {:?} for {:?}",
                    "ALLOW BY PIPELINING".bright_green(),
                    addr_route,
                    aph,
                );
                <Self as AhbSlaveOutputDispatcher<_>>::on_grant_wire(
                    this.component_mut(),
                    ctx,
                    addr_route,
                    TrackedBool::true_::<Self>(),
                );
                aph
            } else {
                aph
            }
        } else {
            MasterToSlaveAddrPhase::empty::<Self>()
        };
        this.addr_phase_out.set_next(addr_phase.clone());
        this.down_track.set_last_addr(addr_phase.clone());
        debug_assert!(
            addr_phase.HREADYIN(),
            "Low HREADYIN is not supported in combinatorial output stage"
        );

        let remaining_requests = enum_map! {t => this.requests[t].take()};
        for (tag, addr_phase) in remaining_requests
            .into_iter()
            .filter_map(|(t, a)| a.map(|a| (t, a)))
        {
            let wants_arbiter = addr_phase.meta.is_address_valid() && addr_phase.HREADYIN();
            // TODO: do we still need to deny owner on a wait state?
            if wants_arbiter {
                trace!(
                    "{} access to {:?} for {:?}",
                    "DENYING".bright_red(),
                    tag,
                    addr_phase
                );
                <Self as AhbSlaveOutputDispatcher<_>>::on_grant_wire(
                    this.component_mut(),
                    ctx,
                    tag,
                    TrackedBool::false_::<Self>(),
                );
            }
        }

        if addr_phase.meta.is_address_valid() {
            this.data_route.set_next_if_not_latching(addr_route);
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
            *this,
            this.arbiter.get_addr_in_port(),
            this.data_route
        );
        <Self as AHBMasterPortOutput>::send_ahb_output(this.component_mut(), ctx, msg);
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
