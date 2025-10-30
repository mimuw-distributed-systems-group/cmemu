//! A write buffer shields AHB master from wait-states during write.
//!
//! This is done by "faking" SUCCESS response in the first data phase cycle, and then finishing
//! the transfer asynchronously from the AHB master. This has two caveats:
//! - the next transfer from AHB Master would have to wait until the write is finished,
//! - errors reported on HRESP after the first write cycle cannot be routed back to the master normally.
//!
//! According to [ARM-SDK], the write buffer may potentially be required to:
//! - [ ] Observe HPROT to check if the transfer is bufferable, or allow injecting this info.
//! - [ ] Support slower slave clock?
//! - [ ] Handle unlocking transfers, including inserting IDLE
//! - [ ] Hold next transfer address phase regardless of its type
//! - [ ] Reflect HREADY signal (AHB-full?)
//! - [ ] Handle errors on buffered write transfers

use crate::common::new_ahb::ports::{
    AHBMasterPortInput, AHBMasterPortOutput, AHBPortConfig, AHBSlavePortInput, AHBSlavePortOutput,
};
use crate::common::new_ahb::signals::{
    AhbResponseControl, MasterToSlaveAddrPhase, MasterToSlaveDataPhase, MasterToSlaveWires,
    SlaveToMasterWires,
};
use crate::common::new_ahb::state_track::{AHBStateTrack, TransitionInfo};
use crate::engine::{
    Context, DisableableComponent, StateMachine, Subcomponent, TickComponent, TickComponentExtra,
};
use crate::utils::IfExpr;
use log::trace;
use owo_colors::OwoColorize;
use std::fmt::Debug;
use std::marker::PhantomData;
// Naming of sides:
// The request come from the side, which we may call:
// - our master
// - upstream
// - [our] slave port
// The requests' destination is called:
// - [our] slave
// - downstream
// - [our] master port

pub(crate) trait WriteBufferCfg {
    /// Whether a transition from buffered store to a load needs to have and `Idle` transfer
    /// injected in between.
    const IS_BUF_TO_LOAD_FAST: bool = true;
}

#[derive(Subcomponent, TickComponent, DisableableComponent)]
pub(crate) struct WriteBuffer<SC>
where
    SC: Subcomponent<Member = Self>,
    Self: AHBMasterPortOutput
        + AHBSlavePortOutput
        + AHBPortConfig<Component = SC::Component>
        + WriteBufferCfg,
{
    upstream_track: AHBStateTrack,
    downstream_track: AHBStateTrack,

    // Write-only (outside tick_extra)
    last_reply: Option<AhbResponseControl>,
    last_addr: Option<MasterToSlaveAddrPhase>,

    data_buffer: Option<MasterToSlaveDataPhase<<Self as AHBPortConfig>::Data>>,

    state: WriteBufSTM,

    enabled: bool,
    // waits for transfer to be done
    enabled_next: Option<bool>,

    // FIXME: requirement of strict ordering between our tock and incoming messages breaks things
    // for now just make sure that this `tock` is called last.
    buffered_msg_sent: bool,
    phantom_sc: PhantomData<SC>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WriteBufSTM {
    Transparent,       // aka data phase active
    IdleInjectionDone, // aka addr phase wait in buffer, but no data phase
    BufferedWriteFirst,
    BufferedWriteAsync,
}

impl StateMachine for WriteBufSTM {}

fn is_bufferable(trans: Option<&MasterToSlaveAddrPhase>) -> bool {
    trans
        .and_then(|t| t.meta.meta())
        .is_some_and(|m| m.is_writing() && m.prot.is_bufferable)
}

impl<SC> TickComponentExtra for WriteBuffer<SC>
where
    SC: Subcomponent<Member = Self>,
    Self: AHBMasterPortOutput
        + AHBSlavePortOutput
        + AHBPortConfig<Component = SC::Component>
        + WriteBufferCfg,
{
    #[cfg(debug_assertions)]
    fn tick_assertions(&self) {
        assert!(
            self.data_buffer
                .implies(self.state != WriteBufSTM::Transparent)
        );
    }

    fn tick_extra(&mut self) {
        assert!(self.enabled);
        trace!(
            "WriteBuf {} in state={:?}, data_buf={:?}, upstream {:?}, down: {:?}",
            <Self as AHBPortConfig>::TAG.bright_purple().bold(),
            self.state,
            self.data_buffer,
            self.upstream_track,
            self.downstream_track,
        );

        let TransitionInfo {
            advanced: downstream_advanced,
            ..
        } = self.downstream_track.update();
        let TransitionInfo {
            advanced: upstream_advanced,
            has_data_ph: up_has_data_ph,
            ..
        } = self.upstream_track.update();

        let next_bufferable = self
            .upstream_track
            .data_address()
            .is_some_and(|a| a.meta.is_bufferable());
        let bufferable_m = next_bufferable;

        let idle_injection_now = matches!(
            self.state,
            WriteBufSTM::BufferedWriteFirst | WriteBufSTM::BufferedWriteAsync
        ) && !<Self as WriteBufferCfg>::IS_BUF_TO_LOAD_FAST
            && upstream_advanced
            && up_has_data_ph
            && !next_bufferable;

        self.state = if downstream_advanced {
            self.data_buffer = None;
            if bufferable_m {
                WriteBufSTM::BufferedWriteFirst
            } else if idle_injection_now {
                WriteBufSTM::IdleInjectionDone
            } else {
                WriteBufSTM::Transparent
            }
        } else if self.state == WriteBufSTM::BufferedWriteFirst {
            if let Some(AhbResponseControl::Error1) = self.last_reply {
                // Let us handle Error2
                WriteBufSTM::Transparent
            } else {
                debug_assert!(
                    self.data_buffer.is_some(),
                    "Data missing in first cycle of write buffer!"
                );
                WriteBufSTM::BufferedWriteAsync
            }
        } else {
            self.state
        };

        self.buffered_msg_sent = false;
        self.last_reply = None;
        self.last_addr = None;
        // Logic:
        // a) bufferable write advanced to data phase on our output:
        //    -> move to BufferedWriteFirst
        // b) buffered write active and new addr_phase comes in:
        //    -> output next addr phase from the master
        //    -> buffer the addr phase to keep it on the next output while buffered write is wait-stating
        //    -> the master will consider the addr_phase advanced, so we must return wait-states for it
        //    -> the master will pipeline next (third) transfer -- we can ignore it?
        // b) buffered write ends in the first cycle with SUCC or ERR1
        //    -> route the response back to master
        //    -> if ERR1, just change state to Transparent in order to route ERR2
        //    -> if SUCC, consider the pipelined addr_phase advanced, and move to either Transparent or BufFirst
        // b) buffered write first cyc is PENDING
        //    -> return SUCC to master
        //    -> remember the addr_phase in buffer
        //    -> remember the data in buffer
        // b) buffered write is still PENDING
        //    -> return PENDING if addr_phase was in buffer
        //    -> ignore incoming addr_phase
        // c) buffered write is finished:
        //    -> still return pending
        //    -> move next to first or transparent
        // d) transparent in progress
        //    -> remember next addr_phase in case of advancing
        // e) transparent advanced:
        //    -> move next to first or transparent
        // f) ERR1 during wait-stated buffered write:
        //   -> keep the mode
        //   -> handle the error
    }
}

impl<SC> WriteBuffer<SC>
where
    SC: Subcomponent<Member = Self>,
    Self: AHBMasterPortOutput
        + AHBSlavePortOutput
        + AHBPortConfig<Component = SC::Component>
        + WriteBufferCfg,
{
    pub(crate) fn new() -> Self {
        Self {
            upstream_track: AHBStateTrack::default(),
            downstream_track: AHBStateTrack::default(),
            last_reply: None,
            last_addr: None,
            data_buffer: None,
            state: WriteBufSTM::Transparent,
            enabled: true,
            enabled_next: None,
            buffered_msg_sent: false,
            phantom_sc: PhantomData,
        }
    }

    pub(crate) fn tick(_comp: &mut SC::Component, _ctx: &mut Context) {}

    pub(crate) fn tock(comp: &mut SC::Component, ctx: &mut Context) {
        let mut this = SC::get_proxy(comp);

        if this.state == WriteBufSTM::BufferedWriteAsync {
            let mut addr_phase = this.upstream_track.data_address().cloned();
            if <Self as WriteBufferCfg>::IS_BUF_TO_LOAD_FAST
                || is_bufferable(this.last_addr.as_ref())
            {
                // Try to fast-forward addr phase that would go this cycle.
                addr_phase = addr_phase.or_else(|| this.last_addr.clone());
            }
            let msg = MasterToSlaveWires {
                addr_phase: addr_phase.unwrap_or_else(MasterToSlaveAddrPhase::empty::<Self>),
                // Should we takeover sender tag?
                data_phase: this.data_buffer.clone().expect("Write buffer missing data"),
            };
            this.buffered_msg_sent = true;
            <Self as AHBMasterPortOutput>::send_ahb_output(this.component_mut(), ctx, msg);
        } else if this.state == WriteBufSTM::IdleInjectionDone {
            let msg = MasterToSlaveWires {
                addr_phase: this
                    .upstream_track
                    .data_address()
                    .expect("Idle injection done should only hold addr phase")
                    .clone(),
                data_phase: MasterToSlaveDataPhase::empty::<Self>(),
            };
            <Self as AHBMasterPortOutput>::send_ahb_output(this.component_mut(), ctx, msg);
        }
    }

    #[allow(dead_code)]
    pub(crate) fn is_enabled(comp: &mut SC::Component) -> bool {
        SC::component_to_member(comp).enabled
    }

    #[allow(dead_code)]
    pub(crate) fn set_enabled(comp: &mut SC::Component, enabled: bool) {
        assert!(enabled);
        SC::component_to_member_mut(comp).enabled_next = Some(enabled);
        // clear state?
    }
}

impl<SC> AHBSlavePortInput for WriteBuffer<SC>
where
    SC: Subcomponent<Member = Self>,
    Self: AHBMasterPortOutput
        + AHBSlavePortOutput
        + AHBPortConfig<Component = SC::Component>
        + WriteBufferCfg,
{
    fn on_ahb_input(
        comp: &mut Self::Component,
        ctx: &mut Context,
        msg: MasterToSlaveWires<Self::Data>,
    ) {
        let mut this = SC::get_proxy(comp);
        this.last_addr = Some(msg.addr_phase.clone());
        this.upstream_track.set_last_addr(msg.addr_phase.clone());
        // logic:
        // in transparent copy in to out, but buffer addr_phase for reference
        // in buffered write first: same, but also copy data to buffer
        // in buffered next: if addr_buffer empty, copy addr_phase, otherwise provide addr from buffer
        let msg = match this.state {
            WriteBufSTM::Transparent => msg,
            WriteBufSTM::BufferedWriteFirst => {
                this.data_buffer = Some(msg.data_phase.clone());
                if !<Self as WriteBufferCfg>::IS_BUF_TO_LOAD_FAST
                    && !is_bufferable(Some(&msg.addr_phase))
                {
                    MasterToSlaveWires {
                        addr_phase: MasterToSlaveAddrPhase::empty::<Self>(),
                        data_phase: msg.data_phase,
                    }
                } else {
                    msg
                }
            }
            // NOTE: This is handled in tock
            WriteBufSTM::BufferedWriteAsync | WriteBufSTM::IdleInjectionDone => {
                assert!(
                    this.upstream_track.data_address().is_some() || !this.buffered_msg_sent,
                    "Incoming slave message came with an address phase after we sent our messages. This is a hard issue, read NOTEs here."
                );
                return;
            } // NOTE: we have to push this addr_phase if the buffer was empty, but we cannot know if it will ever come
        };
        this.downstream_track.set_last_addr(msg.addr_phase.clone());
        <Self as AHBMasterPortOutput>::send_ahb_output(this.component_mut(), ctx, msg);
    }
}

impl<SC> AHBMasterPortInput for WriteBuffer<SC>
where
    SC: Subcomponent<Member = Self>,
    Self: AHBMasterPortOutput
        + AHBSlavePortOutput
        + AHBPortConfig<Component = SC::Component>
        + WriteBufferCfg,
{
    #[allow(clippy::match_same_arms)]
    fn on_ahb_input(
        comp: &mut Self::Component,
        ctx: &mut Context,
        msg: SlaveToMasterWires<Self::Data>,
    ) {
        let mut this = SC::get_proxy(comp);
        // in transparent, copy to out and store resp for reference
        // in buffer_write_first: always return success (except err1), store resp
        // in buffer_write_next: return SUCC to idles and PENDING for virtually-advanced transfers. panic on err1
        // No need to terminate idles
        this.last_reply = Some(msg.meta);
        this.downstream_track.set_last_reply(msg.meta);
        let msg = match this.state {
            WriteBufSTM::Transparent => msg,
            WriteBufSTM::IdleInjectionDone => {
                if msg.meta.is_waitstate() {
                    msg
                } else {
                    this.upstream_track
                        .data_address()
                        .expect("Idle injection should hold transfer")
                        .make_reply::<Self, _>(AhbResponseControl::Pending, Default::default())
                }
            }
            WriteBufSTM::BufferedWriteFirst if msg.meta == AhbResponseControl::Pending => {
                SlaveToMasterWires {
                    meta: AhbResponseControl::Success,
                    ..SlaveToMasterWires::takeover_reply::<Self>(msg)
                }
            }
            WriteBufSTM::BufferedWriteFirst => msg,
            WriteBufSTM::BufferedWriteAsync => {
                // Our master forgot about this transfer long time ago...
                let upstream_data_address = this.upstream_track.data_address();
                if msg.meta == AhbResponseControl::Error1 {
                    unimplemented!("Handling errors on waited writes is unimplemented");
                }
                SlaveToMasterWires {
                    meta: upstream_data_address
                        .ife(AhbResponseControl::Pending, AhbResponseControl::Success),
                    ..upstream_data_address.map_or_else(
                        SlaveToMasterWires::empty::<Self>,
                        SlaveToMasterWires::empty_addr_reply::<Self>,
                    )
                }
            }
        };

        this.upstream_track.set_last_reply(msg.meta);
        <Self as AHBSlavePortOutput>::send_ahb_output(this.component_mut(), ctx, msg);
    }
}
