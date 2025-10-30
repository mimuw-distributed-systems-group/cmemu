use std::marker::PhantomData;

use log::trace;

use cmemu_common::Address;
use owo_colors::OwoColorize;

use crate::common::new_ahb::ports::AhbMasterPortInputWithGranting;
use crate::common::new_ahb::ports::{
    AHBMasterPortInput, AHBMasterPortOutput, AHBSlavePortInput, AHBSlavePortOutput,
};
use crate::common::new_ahb::signals::AhbResponseControl::{Pending, Success};
use crate::common::new_ahb::signals::TransferType::NonSeq;
use crate::common::new_ahb::signals::{HRESP, MasterToSlaveAddrPhase, TrackedBool, TransferMeta};
use crate::common::new_ahb::state_track::{AHBStateTrack, TransitionInfo};
use crate::common::new_ahb::{
    AHBPortConfig, DataBus, MasterToSlaveWires, Size, SlaveToMasterWires,
};
use crate::common::{BitstringUtils, Word};
use crate::engine::{
    Context, DisableableComponent, Subcomponent, TickComponent, TickComponentExtra,
};
#[cfg(feature = "cycle-debug-logger")]
use crate::proxy::CycleDebugLoggerProxy;
#[cfg_attr(not(debug_assertions), allow(unused_imports))]
use crate::utils::{IfExpr, Implies};

/// An aligner converts unaligned AHB accesses to two or three aligned accesses.
/// For example, a word read from address 0x1 is converted to: byte from 0x1, half from 0x2 and byte from 0x4.
/// From `mm319369/experiments/unaligned.S` it seems that the aligned performs optimal pipelining
/// and is placed before the write buffer.
/// The flow is as follows:
/// 1. When a request comes, if it is aligned, then we just forward the request.
///    a. If it is unaligned, we generate a schedule and forward the request with mutated width.
/// 2. Upon response: if aligned, it is just forwarded
///    a. If it is not the last transfer, we inject a waitstate
///    b. If it is last, we compose the response.
/// 3. TODO: research what to do with DENY (we pass for transparent, but what otherwise?)
// TODO: this should be probably split in two -- aligned and wires reinterpreter (barrel shifter)
#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub(crate) enum AddrHandlingMode {
    Aligner,
    AddrTruncatedReinterpret,
}
pub(crate) trait AlignerCfg: AHBPortConfig {
    // a bit weird, but there is no default types yet
    // TODO: use this normally
    const GRANTER: Option<fn(&mut Self::Component, &mut Context, TrackedBool)> = None;

    fn how_to_handle_unaligned(_addr: Address) -> AddrHandlingMode {
        AddrHandlingMode::Aligner
    }
}

#[derive(Subcomponent, TickComponent, DisableableComponent)]
pub(crate) struct Aligner<SC>
where
    SC: Subcomponent<Member = Self>,
    Self: AHBMasterPortOutput
        + AHBSlavePortOutput
        + AlignerCfg
        + AHBPortConfig<Component = SC::Component, Data = DataBus>,
{
    // Write only outside tick_extra
    upstream_ahb_state: AHBStateTrack,
    downstream_ahb_state: AHBStateTrack,

    collected_data: Option<DataBus>,
    collection_data_addr: Option<Address>,

    enabled: bool,
    // waits for transfer to be done
    enabled_next: Option<bool>,

    phantom_sc: PhantomData<SC>,
}

impl<SC> TickComponentExtra for Aligner<SC>
where
    SC: Subcomponent<Member = Self>,
    Self: AHBMasterPortOutput
        + AHBSlavePortOutput
        + AlignerCfg
        + AHBPortConfig<Component = SC::Component, Data = DataBus>,
{
    #[cfg(debug_assertions)]
    fn tick_assertions(&self) {
        debug_assert!((!self.enabled).implies(self.collected_data.is_none()));

        self.collected_data.as_ref().expect_implies_then(
            self.upstream_ahb_state.data_address(),
            |d, a| {
                assert!(d.size() == a.meta.size().unwrap());
                assert!(self.collection_data_addr.is_some());
                assert!(
                    self.collection_data_addr
                        .unwrap()
                        .offset_from(a.meta.address().unwrap())
                        <= d.size().bytes32()
                );
            },
        );
    }

    fn tick_extra(&mut self) {
        trace!(
            "{} {} collected state: upstream {:?} downstream {:?}",
            "Aligner".cyan(),
            <Self as AHBPortConfig>::TAG,
            self.upstream_ahb_state,
            self.downstream_ahb_state,
        );

        let upstream_transition_info = self.upstream_ahb_state.update();
        let downstream_transition_info = self.downstream_ahb_state.update();

        trace!(
            "{} {} ticking up {:?} down {:?}, internal: {:?} now at {:?} ",
            "Aligner".cyan(),
            <Self as AHBPortConfig>::TAG,
            upstream_transition_info,
            downstream_transition_info,
            self.collected_data,
            self.collection_data_addr,
        );

        let TransitionInfo {
            advanced: upstream_advanced,
            finished: upstream_finished,
            has_data_ph: upstream_has_data_ph,
        } = upstream_transition_info;

        let TransitionInfo {
            advanced: downstream_advanced,
            finished: downstream_finished,
            ..
        } = downstream_transition_info;

        if self.collection_data_addr.is_some() && upstream_finished {
            debug_assert!(downstream_finished);
            self.collected_data = None;
            self.collection_data_addr = None;
        }

        if let Some(meta) = self
            .upstream_ahb_state
            .data_address()
            .and_then(|a| a.meta.meta())
            .filter(|m| {
                !m.is_aligned()
                    && <Self as AlignerCfg>::how_to_handle_unaligned(m.addr)
                        == AddrHandlingMode::Aligner
            })
        {
            if upstream_advanced {
                // New request
                debug_assert!(self.collection_data_addr.is_none());
                self.collection_data_addr = Some(meta.addr);
                self.collected_data = Some(DataBus::clip_word(0.into(), meta.size));
            } else if downstream_advanced {
                let prev_addr = self
                    .collection_data_addr
                    .expect("It should be there if up not advanced");
                let (next_addr, remaining) = get_next_addr(prev_addr, meta.addr, meta.size);
                debug_assert!(remaining > 0, "We should have finished");
                self.collection_data_addr = Some(next_addr);
            }
        }

        // probably not correct timing
        if !upstream_has_data_ph && self.enabled_next.is_some() {
            self.enabled = self.enabled_next.take().unwrap();
        }
    }
}

/// Only up to a word
pub(crate) fn largest_aligned_size(addr: Address, bytes: u32) -> Size {
    for s in [Size::Word, Size::Halfword, Size::Byte] {
        if s.bytes32() <= bytes && s.is_addr_aligned(addr) {
            return s;
        }
    }
    unreachable!()
}

pub(crate) fn get_next_addr(last_addr: Address, base_addr: Address, size: Size) -> (Address, u32) {
    let prev_remaining = size.bytes32() - last_addr.offset_from(base_addr);
    let prev_size = largest_aligned_size(last_addr, prev_remaining);
    (
        prev_size.shift_addr(last_addr),
        prev_remaining - prev_size.bytes32(),
    )
}

impl DataBus {
    // That's kinda hacky..., thus nonpublic
    #[must_use]
    fn extract_chunk(self, offset: u32, size: Size) -> DataBus {
        self.extract_from_aligned(offset.into(), size)
    }
    #[must_use]
    fn emplace_chunk(self, offset: u32, data: Self) -> DataBus {
        self.emplace_in_aligned(offset.into(), data)
    }
}

impl<SC> Aligner<SC>
where
    SC: Subcomponent<Member = Self>,
    Self: AHBMasterPortOutput
        + AHBSlavePortOutput
        + AlignerCfg
        + AHBPortConfig<Component = SC::Component, Data = DataBus>,
{
    pub(crate) fn new() -> Self {
        Self {
            upstream_ahb_state: Default::default(),
            downstream_ahb_state: Default::default(),
            collected_data: None,
            collection_data_addr: None,
            enabled: true,
            enabled_next: None,
            phantom_sc: PhantomData,
        }
    }
    pub(crate) fn tick(_comp: &mut SC::Component, _ctx: &mut Context) {}
    pub(crate) fn tock(_comp: &mut SC::Component, _ctx: &mut Context) {}

    fn remaining_aligning(&self) -> Option<(Address, u32)> {
        match (
            self.upstream_ahb_state.data_address(),
            self.collection_data_addr,
        ) {
            (_, None) => None,
            (Some(MasterToSlaveAddrPhase { meta, .. }), Some(ref last_addr))
                if meta.is_address_valid() =>
            {
                let meta = meta.meta().unwrap();
                let remaining = get_next_addr(*last_addr, meta.addr, meta.size);
                (remaining.1 > 0).ife(Some(remaining), None)
            }
            _ => unreachable!("inconsistent state"),
        }
    }

    fn is_transparent(&self) -> bool {
        self.collection_data_addr.is_none()
    }

    fn get_data_phase_chunk(&self) -> (u32, Size) {
        let data_addr = self.collection_data_addr.unwrap();
        let orig_meta = self
            .upstream_ahb_state
            .data_address()
            .unwrap()
            .meta
            .meta()
            .unwrap();
        let offset = data_addr.offset_from(orig_meta.addr);
        let prev_remaining = orig_meta.size.bytes32() - offset;
        let prev_size = largest_aligned_size(data_addr, prev_remaining);

        (offset, prev_size)
    }
}

impl<SC> AHBSlavePortInput for Aligner<SC>
where
    SC: Subcomponent<Member = Self>,
    Self: AHBMasterPortOutput
        + AHBSlavePortOutput
        + AlignerCfg
        + AHBPortConfig<Component = SC::Component, Data = DataBus>,
{
    fn on_ahb_input(
        comp: &mut Self::Component,
        ctx: &mut Context,
        msg: MasterToSlaveWires<DataBus>,
    ) {
        let mut this = SC::get_proxy(comp);

        let MasterToSlaveWires {
            addr_phase,
            mut data_phase,
        } = msg;

        this.upstream_ahb_state.set_last_addr(addr_phase.clone());

        let addr_phase = if let Some((next_addr, remaining)) = this.remaining_aligning() {
            trace!(
                "Aligner {} is wait-stating and ignores incoming addr_phase {:?}",
                <Self as AHBPortConfig>::TAG,
                addr_phase
            );
            let orig_addr_phase = this.upstream_ahb_state.data_address().unwrap().to_owned();
            let req_size = largest_aligned_size(next_addr, remaining);
            MasterToSlaveAddrPhase {
                meta: NonSeq(TransferMeta {
                    addr: next_addr,
                    size: req_size,
                    ..orig_addr_phase.meta.meta().unwrap().to_owned()
                }),
                ..orig_addr_phase
            }
        } else if addr_phase.meta.is_address_valid_and(|m| !m.is_aligned()) && this.enabled {
            let mode =
                &&<Self as AlignerCfg>::how_to_handle_unaligned(addr_phase.meta.address().unwrap());
            let meta = addr_phase.meta.meta().unwrap().to_owned();

            MasterToSlaveAddrPhase {
                meta: NonSeq(match mode {
                    AddrHandlingMode::Aligner => TransferMeta {
                        size: largest_aligned_size(meta.addr, meta.size.bytes32()),
                        ..meta
                    },
                    AddrHandlingMode::AddrTruncatedReinterpret => TransferMeta {
                        addr: meta.size.align_addr(meta.addr),
                        ..meta
                    },
                }),
                ..addr_phase
            }
        } else {
            addr_phase
        };
        if this.collection_data_addr.is_some()
            && this
                .upstream_ahb_state
                .data_address()
                .unwrap()
                .meta
                .is_writing()
        {
            let (offset, size) = this.get_data_phase_chunk();

            data_phase.data = data_phase.data.extract_chunk(offset, size);
        }

        this.downstream_ahb_state.set_last_addr(addr_phase.clone());
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

impl<SC> AHBMasterPortInput for Aligner<SC>
where
    SC: Subcomponent<Member = Self>,
    Self: AHBMasterPortOutput
        + AHBSlavePortOutput
        + AlignerCfg
        + AHBPortConfig<Component = SC::Component, Data = DataBus>,
{
    fn on_ahb_input(
        comp: &mut Self::Component,
        ctx: &mut Context,
        mut msg: SlaveToMasterWires<Self::Data>,
    ) {
        let this = SC::component_to_member_mut(comp);

        this.downstream_ahb_state.set_last_reply(msg.meta);

        let Some(up_addr_phase) = this.upstream_ahb_state.data_address() else {
            // No addr phase -> probably idle
            debug_assert!(msg.meta.is_done());
            this.upstream_ahb_state.set_last_reply(msg.meta);
            return <Self as AHBSlavePortOutput>::send_ahb_output(comp, ctx, msg);
        };

        if !this.is_transparent() {
            let (offset, size) = this.get_data_phase_chunk();

            trace!(
                "Aligner {} for {:?} got response {:?} while internally collector:{:?}, last:{:?} (i.e. should be +{} of {:?})",
                <Self as AHBPortConfig>::TAG,
                up_addr_phase,
                msg,
                this.collected_data,
                this.collection_data_addr,
                offset,
                size,
            );
            if this.downstream_ahb_state.data_address().is_none() {
                // We probably got denied previously and it is now our time to advance
                debug_assert!(
                    msg.meta.is_done(),
                    "Non-success response with no data-phase in aligner"
                );
                msg = SlaveToMasterWires {
                    meta: Pending,
                    ..SlaveToMasterWires::empty_addr_reply::<Self>(up_addr_phase)
                };
            } else if msg.meta.is_done() {
                if !up_addr_phase.meta.is_writing() {
                    debug_assert!(msg.data.size() == size);
                    let collector = this.collected_data.take().unwrap();
                    this.collected_data = Some(collector.emplace_chunk(offset, msg.data));
                }
                // TODO: or consider to copy the downstream responder_tag to this message
                let data = if this.remaining_aligning().is_some() || up_addr_phase.meta.is_writing()
                {
                    Default::default()
                } else {
                    this.collected_data.take().unwrap()
                };
                msg = SlaveToMasterWires {
                    meta: (
                        this.remaining_aligning().is_some()
                        // && <Self as AlignerCfg>::GRANTER.is_none()
                    )
                    .ife(Pending, Success),
                    data,
                    ..SlaveToMasterWires::empty_addr_reply::<Self>(up_addr_phase)
                };
            } else if msg.meta.HRESP() == HRESP::ERROR {
                panic!("Error are not supported");
            } else {
                // TODO: WRONG TAG?
                // if (this.remaining_aligning().is_some() && <Self as AlignerCfg>::GRANTER.is_some()) {
                //     msg.meta = AhbResponseControl::Pending;
                // }
                msg = SlaveToMasterWires::takeover_reply::<Self>(msg);
            }
        } else if up_addr_phase.meta.is_address_valid()
            && up_addr_phase
                .meta
                .meta()
                .is_some_and(|m| !m.is_aligned() && m.is_reading())
            && <Self as AlignerCfg>::how_to_handle_unaligned(up_addr_phase.meta.address().unwrap())
                == AddrHandlingMode::AddrTruncatedReinterpret
        {
            // Special case handling for regions without aligning
            let meta = up_addr_phase.meta.meta().unwrap();
            // Here we observe the side effects of actual data -> wire mapping
            // TODO: ref to ARM-AHB
            trace!(
                "Aligner {} for {:?} got response {:?} that needs to be reinterpreted by their wire placement",
                <Self as AHBPortConfig>::TAG,
                up_addr_phase,
                msg,
            );
            // FIXME: this needs checks + this depends on the bus width
            let shift = u32::try_from(meta.size.offset_from_aligned(meta.addr)).unwrap();
            let word: Word = msg.data.zero_extend_into_word();
            msg.data = DataBus::clip_word(word.ror(8 * shift), meta.size);
        }

        debug_assert!(
            !this.upstream_ahb_state.is_last_reply_set(),
            "Double message from aligner!"
        );
        this.upstream_ahb_state.set_last_reply(msg.meta);
        <Self as AHBSlavePortOutput>::send_ahb_output(comp, ctx, msg);
    }
}

impl<SC> AhbMasterPortInputWithGranting for Aligner<SC>
where
    SC: Subcomponent<Member = Self>,
    Self: AHBMasterPortOutput
        + AHBSlavePortOutput
        + AlignerCfg
        + AHBPortConfig<Component = SC::Component, Data = DataBus>,
{
    fn on_grant_wire(comp: &mut Self::Component, ctx: &mut Context, granted: TrackedBool) {
        let mut this = Self::get_proxy(comp);
        this.downstream_ahb_state.set_last_deny(!*granted);
        this.upstream_ahb_state.set_last_deny(!*granted);
        if *granted {
            if let Some(d) = <Self as AlignerCfg>::GRANTER {
                d(this.component_mut(), ctx, granted);
            }
        } else {
            let mut this = Self::get_proxy(comp);

            // TODO: what if we're mid-unaligned and already accepted next addr phase?
            trace!("Aligner {} got a deny!", <Self as AHBPortConfig>::TAG);

            #[cfg(feature = "cycle-debug-logger")]
            CycleDebugLoggerProxy.on_free_static_str(
                ctx,
                <Self as AHBPortConfig>::get_name(),
                "DENIED",
            );

            <Self as AlignerCfg>::GRANTER
                .expect("Aligned received grant deny, but has not implemented GRANTER")(
                this.component_mut(),
                ctx,
                granted,
            );

            if !this.is_transparent() && this.downstream_ahb_state.data_address().is_none() {
                // We probably got denied previously and it is now our time to advance
                trace!(
                    "Aligner {} is HELD in address phase mid-part!",
                    <Self as AHBPortConfig>::TAG
                );
                let msg = SlaveToMasterWires {
                    meta: Pending,
                    ..SlaveToMasterWires::empty_addr_reply::<Self>(
                        this.upstream_ahb_state.data_address().as_ref().unwrap(),
                    )
                };
                debug_assert!(
                    !this.upstream_ahb_state.is_last_reply_set(),
                    "Double message from aligner!"
                );
                this.upstream_ahb_state.set_last_reply(msg.meta);
                <Self as AHBSlavePortOutput>::send_ahb_output(comp, ctx, msg);
            }
        }
    }
}
