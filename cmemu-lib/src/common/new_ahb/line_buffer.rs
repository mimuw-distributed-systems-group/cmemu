//! AHB-Lite line buffer (sub-)component
//!
//! To use, incorporate the [`LineBuffer`] structure as a subcomponent and implement [`LineBufferCfg`].
use crate::common::Address;
use crate::common::new_ahb::ports::{
    AHBMasterPortInput, AHBMasterPortOutput, AHBPortConfig, AHBSlavePortInput, AHBSlavePortOutput,
};
use crate::common::new_ahb::signals::{
    AhbResponseControl, Direction, MasterToSlaveAddrPhase, MasterToSlaveWires, Size,
    SlaveToMasterWires, TransferMeta, TransferType,
};
use crate::common::new_ahb::state_track::AHBStateTrack;
use crate::engine::{
    CombFlop, Context, DisableableComponent, SeqFlopMemoryBankSimpleLarge, SeqRegister,
    Subcomponent, TickComponent, TickComponentExtra,
};
use crate::utils::dife;
use log::{debug, trace};
use owo_colors::OwoColorize;
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;

/// Configuration trait for the line buffer
// Note: currently input and output must share a type, but it could be simply fixed by
// extracting output to another structure.
pub(crate) trait LineBufferCfg: AHBPortConfig {
    /// The "line size", i.e., the transfer size downstream
    const UPSIZED: Size;
    /// Whether the line buffer is enabled after reset
    const ENABLED_BY_DEFAULT: bool = true;
    /// Whether to tie the "cacheable" `HPROT` wire low
    ///
    /// This means that if one line buffer is placed before the other,
    /// the transfer won't invalidate the other's cache.
    const MASKS_CACHEABLE: bool = true;

    /// Extract the requested sub-fragment from the whole "line"
    ///
    /// The `data` comes from a request to an aligned, full-width `UPSIZED` transfer.
    fn extract_upstream_from_upsized(
        addr: &TransferMeta,
        data: &<Self as AHBPortConfig>::Data,
    ) -> <Self as AHBPortConfig>::Data;
}

/// Line buffer AHB-Lite subcomponent
///
/// This component is combinatorial and may be placed between any AHB-Lite entities like so:
///
/// ```text
/// // [MasterPort] ==> [Line buffer] ==> [SlavePort]
/// bridge_ports!(MasterPort => TheLineBuffer);
/// bridge_ports!(@auto_configured @master TheLineBuffer => @slave SlavePort);
/// ```
///
/// **Note**: writing through the line buffer is not implemented!
#[derive(Subcomponent, TickComponent)]
pub(crate) struct LineBuffer<SC>
where
    SC: Subcomponent<Member = Self>,
    Self: LineBufferCfg
        + AHBMasterPortOutput
        + AHBSlavePortOutput
        + AHBPortConfig<Component = SC::Component>,
{
    /// State tracking for our slave port
    upstream_track: AHBStateTrack,
    /// State tracking for our master port
    downstream_track: AHBStateTrack,

    #[flop]
    buffered_data: SeqFlopMemoryBankSimpleLarge<<Self as AHBPortConfig>::Data>,
    #[flop]
    buffer_addr: SeqRegister<Address>,

    /// The current mode of operation
    // Comb, as we need to decide in two places
    #[flop]
    mode: CombFlop<LBMode>,

    lb_handles_response: bool,

    phantom_sc: PhantomData<SC>,

    enabled: bool,
    // waits for transfer to be done
    enabled_next: Option<bool>,
}

impl<SC> DisableableComponent for LineBuffer<SC>
where
    SC: Subcomponent<Member = Self>,
    Self: LineBufferCfg
        + AHBMasterPortOutput
        + AHBSlavePortOutput
        + AHBPortConfig<Component = SC::Component>,
{
    fn can_be_disabled_now(&self) -> bool {
        !self.upstream_track.seems_active()
            && !self.downstream_track.seems_active()
            && self.buffer_addr.is_empty()
            && self.buffered_data.is_empty()
            && self.enabled_next.is_none()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum LBMode {
    /// Transfer not active
    _Idle,
    /// We didn't modify the transfer (e.g., it was not cacheable)
    Transparent,
    /// We need to request data from the backing storage
    Fetching,
    /// Responding from local buffer
    Local,
}

// TODO: unit-tests
impl<SC> Debug for LineBuffer<SC>
where
    SC: Subcomponent<Member = Self>,
    Self: LineBufferCfg
        + AHBMasterPortOutput
        + AHBSlavePortOutput
        + AHBPortConfig<Component = SC::Component>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "LineBuffer mode={:#?} ", self.mode)?;
        if let Some(next) = self.enabled_next {
            write!(f, " {} ", dife(next, "ENABLING".green(), "DISABLING".red()))?;
        } else if !self.enabled {
            return write!(f, "{}", "(DISABLED)".bold());
        }
        write!(
            f,
            "D: {:#?} @ {:#?} TR: U:{:?}, D:{:?}",
            self.buffered_data, self.buffer_addr, self.upstream_track, self.downstream_track
        )?;

        Ok(())
    }
}

impl<SC> TickComponentExtra for LineBuffer<SC>
where
    SC: Subcomponent<Member = Self>,
    Self: LineBufferCfg
        + AHBMasterPortOutput
        + AHBSlavePortOutput
        + AHBPortConfig<Component = SC::Component>,
{
    fn tick_extra(&mut self) {
        let up_trans = self.upstream_track.update();
        let _down_trans = self.downstream_track.update();
        self.lb_handles_response = self.mode.is_set_and(|&m| m == LBMode::Local);
        trace!("LineBuffer {}: {:?}", <Self as AHBPortConfig>::TAG, self);

        // Properly wait for quiescence before the switch.
        // TODO: this needs timing tests
        if self.enabled_next.is_some()
            && up_trans.advanced
            && !self.lb_handles_response
            && !self.mode.is_set_and(|&m| m != LBMode::Fetching)
        {
            self.enabled = self.enabled_next.take().unwrap();
            if !self.enabled {
                self.clear();
            }
        }
    }
}

impl<SC> LineBuffer<SC>
where
    SC: Subcomponent<Member = Self>,
    Self: LineBufferCfg
        + AHBMasterPortOutput
        + AHBSlavePortOutput
        + AHBPortConfig<Component = SC::Component>,
{
    pub(crate) fn new() -> Self {
        Self {
            upstream_track: Default::default(),
            downstream_track: Default::default(),
            buffered_data: SeqFlopMemoryBankSimpleLarge::new(Default::default()),
            buffer_addr: SeqRegister::new(Address::from_const(u32::MAX)),
            mode: CombFlop::new(),
            lb_handles_response: false,

            enabled: <Self as LineBufferCfg>::ENABLED_BY_DEFAULT,
            enabled_next: None,
            phantom_sc: PhantomData,
        }
    }
    pub(crate) fn tick(_comp: &mut SC::Component, _ctx: &mut Context) {}

    pub(crate) fn tock(comp: &mut SC::Component, ctx: &mut Context) {
        let mut this = SC::get_proxy(comp);

        // Generate our reply
        if this.lb_handles_response
            && let Some(addr_phase) = this.upstream_track.data_address()
        {
            let meta = addr_phase.meta.meta().unwrap();
            debug_assert_eq!(
                <Self as LineBufferCfg>::UPSIZED.align_addr(meta.addr),
                *this.buffer_addr
            );
            let data =
                <Self as LineBufferCfg>::extract_upstream_from_upsized(meta, &*this.buffered_data);
            let msg = addr_phase.make_reply::<Self, _>(AhbResponseControl::Success, data);
            this.upstream_track.set_last_reply(msg.meta);
            <Self as AHBSlavePortOutput>::send_ahb_output(this.component_mut(), ctx, msg);
        }
    }

    /// Returns (current, wants to) pair (aka reply to STAT and CTL registers)
    pub(crate) fn get_mode(comp: &SC::Component) -> (bool, bool) {
        let this = SC::component_to_member(comp);
        (this.enabled, this.enabled_next.unwrap_or(this.enabled))
    }

    /// Ask the component to disable caching (it will transition after a while)
    pub(crate) fn set_cache_enabled(comp: &mut SC::Component, enabled: bool) {
        let mut this = SC::get_proxy(comp);

        if this.enabled != enabled || this.enabled_next.is_some_and(|n| n != enabled) {
            this.enabled_next = Some(enabled);
        }
        // clear is delayed till quiescence
        // TODO: add tests measuring actual timings
    }

    fn clear(&mut self) {
        // TODO: it is not verified if it is consistent with invariants
        self.buffered_data.set_next(Default::default());
        self.buffer_addr.set_next(Address::from_const(u32::MAX));
    }
}

impl<SC> AHBSlavePortInput for LineBuffer<SC>
where
    SC: Subcomponent<Member = Self>,
    Self: LineBufferCfg
        + AHBMasterPortOutput
        + AHBSlavePortOutput
        + AHBPortConfig<Component = SC::Component>,
{
    fn on_ahb_input(
        comp: &mut Self::Component,
        ctx: &mut Context,
        msg: MasterToSlaveWires<Self::Data>,
    ) {
        let mut this = SC::get_proxy(comp);
        let MasterToSlaveWires {
            addr_phase,
            data_phase,
        } = msg;
        this.upstream_track.set_last_addr(addr_phase.clone());
        // Decide how to mutate the address phase
        let addr_phase =
            if !addr_phase.meta.is_address_valid() || !this.enabled_next.unwrap_or(this.enabled) {
                // TODO: don't propagate idle transfers
                addr_phase
            } else if !addr_phase.ready {
                // We may be required to reflect HREADY
                MasterToSlaveAddrPhase {
                    meta: TransferType::Idle,
                    ..addr_phase
                }
            } else if addr_phase.meta.is_writing() {
                unimplemented!("Writing through a line buffer is not implemented!: {addr_phase:?}");
            } else if !addr_phase.meta.is_cacheable() {
                this.mode.set_next_if_not_latching(LBMode::Transparent);
                addr_phase
            } else {
                let meta = addr_phase.meta.meta().unwrap();
                let aligned_addr = <Self as LineBufferCfg>::UPSIZED.align_addr(meta.addr);
                let next_buf_addr = if this.mode.is_set_and(|&m| m == LBMode::Fetching) {
                    this.downstream_track
                        .data_address()
                        .unwrap()
                        .meta
                        .address()
                        .unwrap()
                } else {
                    *this.buffer_addr
                };
                if aligned_addr == next_buf_addr {
                    this.mode.set_next_if_not_latching(LBMode::Local);
                    if this.downstream_track.data_address().is_some() {
                        MasterToSlaveAddrPhase {
                            meta: TransferType::Idle,
                            ..addr_phase
                        }
                    } else {
                        return; // Don't send a downstream message!
                    }
                } else {
                    // New line to cache!
                    debug!(
                        "LB {}, Requesting next address {:?}",
                        <Self as AHBPortConfig>::TAG,
                        aligned_addr
                    );
                    let mut protection = meta.prot;
                    if <Self as LineBufferCfg>::MASKS_CACHEABLE {
                        // We prevent caching downstream: this allows composition (multiple line buffers on a way)
                        protection = protection.with_cacheable(false);
                    }
                    this.mode.set_next_if_not_latching(LBMode::Fetching);
                    MasterToSlaveAddrPhase {
                        meta: TransferType::new_single(
                            aligned_addr,
                            <Self as LineBufferCfg>::UPSIZED,
                            Direction::Read,
                            protection,
                        ),
                        ..addr_phase
                    }
                }
            };
        this.downstream_track.set_last_addr(addr_phase.clone());
        <Self as AHBMasterPortOutput>::send_ahb_output(
            this.component_mut(),
            ctx,
            MasterToSlaveWires {
                addr_phase,
                data_phase,
            },
        );
    }
}
impl<SC> AHBMasterPortInput for LineBuffer<SC>
where
    SC: Subcomponent<Member = Self>,
    Self: LineBufferCfg
        + AHBMasterPortOutput
        + AHBSlavePortOutput
        + AHBPortConfig<Component = SC::Component>,
{
    fn on_ahb_input(
        comp: &mut Self::Component,
        ctx: &mut Context,
        mut msg: SlaveToMasterWires<Self::Data>,
    ) {
        let mut this = SC::get_proxy(comp);

        this.downstream_track.set_last_reply(msg.meta);
        if this.mode.is_set_and(|&m| m == LBMode::Fetching) {
            if msg.meta.is_done() {
                this.buffered_data.set_next(msg.data.clone());
                let requested_meta = this
                    .upstream_track
                    .data_address()
                    .expect("Who discarded transfer metadata?")
                    .meta
                    .meta()
                    .unwrap();
                msg.data = <Self as LineBufferCfg>::extract_upstream_from_upsized(
                    requested_meta,
                    &msg.data,
                );
                let aligned_addr = <Self as LineBufferCfg>::UPSIZED.align_addr(requested_meta.addr);
                this.buffer_addr.set_next(aligned_addr);
            } else if msg.meta.is_waitstate() {
                this.mode.keep_current_as_next();
            }
        }
        if !this.lb_handles_response {
            this.upstream_track.set_last_reply(msg.meta);
            <Self as AHBSlavePortOutput>::send_ahb_output(this.component_mut(), ctx, msg);
        }
    }
}
