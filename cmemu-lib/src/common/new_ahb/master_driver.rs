// Locked transfers
// Using a multi-layer AHB system requires certain restrictions to be placed on the use of locked
// transfers to prevent a system deadlock. A sequence of locked transfers must be performed to the
// same slave in the system. Because the minimum address space that you can allocate to a single
// slave is 1KB, a bus master can ensure that this restriction is met by ensuring that it does not
// perform a locked sequence of transfers over a 1KB boundary. This ensures that it never crosses
// an address decode boundary.
// Therefore, if a bus master is to perform two locked transfer sequences to different address
// regions, the bus master must not start the second locked transfer sequence until the final data
// phase of the first locked transfer sequence has completed.

#[cfg(feature = "cycle-debug-logger")]
use crate::common::new_ahb::cdl::CdlTag;
#[cfg(debug_assertions)]
use crate::common::new_ahb::databus::DataBus;
use crate::common::new_ahb::ports::{
    AHBMasterPortInput, AHBMasterPortOutput, AHBPortConfig, AhbMasterPortInputWithGranting,
};
use crate::common::new_ahb::signals::{
    AhbResponseControl, Burst, MasterToSlaveAddrPhase, MasterToSlaveDataPhase, MasterToSlaveWires,
    Protection, SlaveToMasterWires, TrackedBool, TransferMeta, TransferType,
};
use crate::common::utils::SubcomponentProxyMut;
use crate::engine::{
    BufferFlop, Context, DisableableComponent, StateMachine, Subcomponent, TickComponent,
    TickComponentExtra,
};
use crate::move_state_machine;
#[cfg(debug_assertions)]
use crate::static_downcast;
use log::trace;
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;

pub(crate) mod stateless_helpers;

// TODO: mark it cdl-only
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum TransferStatus {
    AddrPhaseNew,
    AddrPhaseStalled,
    AddrPhaseDenied,
    DataPhaseWaiting,
    DataPhaseDone,
}
impl StateMachine for TransferStatus {}

pub(crate) struct TransferInfo<P>
where
    P: Handler,
{
    pub meta: TransferMeta,
    pub status: TransferStatus,
    // None if data not yet known/provided
    pub data: Option<P::Data>,
    pub user: P::UserData,
    #[cfg(feature = "cycle-debug-logger")]
    pub tag: Option<CdlTag>,
}
impl<P> TransferInfo<P>
where
    P: Handler,
{
    #[cfg(test)]
    pub(crate) fn new(
        meta: TransferMeta,
        user: P::UserData,
        #[allow(unused)] tag: &'static str,
    ) -> Self {
        Self {
            meta,
            status: TransferStatus::AddrPhaseNew,
            data: None,
            user,
            #[cfg(feature = "cycle-debug-logger")]
            tag: Some(tag.into()),
        }
    }

    fn view(info: &mut TransferInfo<P>) -> TransferInfoView<'_, P> {
        TransferInfoView {
            status: info.status,
            meta: &info.meta,
            user: &mut info.user,
        }
    }
}

#[allow(clippy::missing_fields_in_debug)]
impl<P> Debug for TransferInfo<P>
where
    P: Handler,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TransferInfo")
            .field("meta", &self.meta)
            .field("status", &self.status)
            .field("data", &self.data)
            .field("user", &self.user)
            // #[cfg(feature = "cycle-debug-logger")]
            // .field("tag", &self.tag)
            .finish()
    }
}

/// The view is to provide access to metadata in a safe way.
/// A User cannot modify metadata and see or provide data.
pub(crate) struct TransferInfoView<'a, P>
where
    P: Handler,
    // <P as Handler>::UserData: 'a,
{
    pub meta: &'a TransferMeta,
    pub status: TransferStatus,
    pub user: &'a mut P::UserData,
}

impl<P> Debug for TransferInfoView<'_, P>
where
    P: Handler,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TransferInfoView")
            .field("meta", &self.meta)
            .field("status", &self.status)
            .field("user", &self.user)
            .finish()
    }
}

pub(crate) trait Handler: AHBPortConfig + Sized {
    type UserData: Debug;
    /// Can the Master change valid address to another during waitstates?
    const AHB_LITE_COMPAT: bool = false;

    // TODO: move this to MasterOutput for routing
    const HAS_GRANTING_WIRE: bool = false;

    /// The interface presented an address on the bus (it was granted an access).
    /// This doesn't mean that the transfer will necessarily advance on the edge.
    /// The `cancellable` bool indicates whether calling `try_force_*` in the next cycle may succeed
    /// if the transfer didn't advance.
    /// Always comes before `transfer_will_advance`.
    /// Called in combinatorial way very late in the cycle – use it only for gating logic.
    /// Use `driver::get_addr_phase` to get reference to the address struct.
    /// Called only when `HAS_GRANTING_WIRE`
    #[allow(unused_variables)]
    fn address_presented(
        comp: &mut <Self as AHBPortConfig>::Component,
        ctx: &mut Context,
        cancellable: bool,
    ) {
    }

    /// The interface presented an address on the bus and it will advance to data phase on the edge.
    /// Called very late in the cycle – use it only for gating logic.
    /// `address_presented` callback may be triggered in the same cycle
    /// Combinatorial call -> write requests may provide data here
    /// Use `driver::get_addr_phase` to get reference to struct
    #[allow(unused_variables)]
    fn transfer_will_advance(
        comp: &mut <Self as AHBPortConfig>::Component,
        ctx: &mut Context,
    ) -> Option<Self::Data> {
        None
    }

    /// Registered call – called in tock, if data was not provided
    /// Use `driver::view_data_phase` to get reference to info struct
    /// Take care, since other handlers may be called first!
    #[allow(unused_variables)]
    fn write_needs_data_this_cycle(
        comp: &mut <Self as AHBPortConfig>::Component,
        ctx: &mut Context,
    ) -> Self::Data {
        panic!("Data was not provided for write transfer!")
    }

    #[allow(unused_variables)]
    fn transfers_will_stall(
        comp: &mut <Self as AHBPortConfig>::Component,
        ctx: &mut Context,
        has_addr: bool,
        has_data: bool,
    ) {
    }

    #[allow(unused_variables)]
    fn transfer_done(
        comp: &mut <Self as AHBPortConfig>::Component,
        ctx: &mut Context,
        info: TransferInfo<Self>,
    );

    fn transfers_aborted(
        comp: &mut <Self as AHBPortConfig>::Component,
        ctx: &mut Context,
        addr_phase: Option<TransferInfo<Self>>,
        data_phase: Option<TransferInfo<Self>>,
    );

    #[allow(unused_variables)]
    fn override_prot(prot: &mut Protection, user: &mut Self::UserData) {}

    #[allow(unused_variables)]
    fn tap_request(
        ctx: &mut Context,
        req: &mut MasterToSlaveWires<Self::Data>,
        addr_info: Option<TransferInfoView<Self>>,
        data_info: Option<TransferInfoView<Self>>,
    ) {
    }

    #[allow(unused_variables)]
    fn tap_response(
        ctx: &mut Context,
        data: &SlaveToMasterWires<Self::Data>,
        has_addr: bool,
        has_data: bool,
    ) {
    }
}

#[derive(Subcomponent, TickComponent)]
pub(crate) struct MasterDriver<SC, P>
where
    SC: Subcomponent<Member = Self>,
    P: Handler<Component = SC::Component>,
{
    addr_phase: Option<TransferInfo<P>>,
    data_phase: Option<TransferInfo<P>>,
    #[flop]
    last_resp: BufferFlop<AhbResponseControl>,
    #[flop]
    held: BufferFlop<bool>,
    just_advanced: bool,

    stm: MDStm,
    ph_sc: PhantomData<SC>,
}

impl<SC, P> DisableableComponent for MasterDriver<SC, P>
where
    SC: Subcomponent<Member = Self>,
    P: Handler<Component = SC::Component>,
{
    fn can_be_disabled_now(&self) -> bool {
        trace!("Can MS be disabled? {self:?}");
        self.addr_phase.is_none()
            && self.data_phase.is_none()
            && self.last_resp.try_this_cycle().is_none_or(|r| r.HREADY())
            && self.last_resp.try_prev_cycle().is_none_or(|r| r.HREADY())
            && self.held.try_this_cycle().is_none_or(|x| !x)
            && self.held.try_prev_cycle().is_none_or(|x| !x)
            && !self.just_advanced
        // TODO: this should be asked at a predetermined point in the cycle!
        // && self.stm != MDStm::RunDriver
    }
}

#[derive(Debug, PartialEq)]
enum MDStm {
    TickExtra,
    RunDriver,
    SentMessages,
    GotMessages,
}
impl StateMachine for MDStm {}

impl<SC, P> Debug for MasterDriver<SC, P>
where
    SC: Subcomponent<Member = Self>,
    P: Handler<Component = SC::Component>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "MasterDriver of {} ", <P as AHBPortConfig>::TAG.bold())?;
        write!(
            f,
            "addr_phase: {}{}{:?}, ",
            dife(
                self.held.try_this_cycle().is_some_and(|r| *r),
                "DENY ".magenta(),
                ""
            ),
            dife(self.held.is_set_and(|r| *r), "HELD ".magenta(), ""),
            self.addr_phase
        )?;
        write!(
            f,
            "data_phase: {} {:?}, ",
            dife(self.just_advanced, "=J>".green(), "WS".blue()),
            self.data_phase
        )?;
        write!(f, "status: {:#?} {:?}", self.last_resp, self.stm)?;
        Ok(())
    }
}

impl<SC, P> TickComponentExtra for MasterDriver<SC, P>
where
    SC: Subcomponent<Member = Self>,
    P: Handler<Component = SC::Component>,
{
    #[cfg(debug_assertions)]
    fn tick_assertions(&self) {
        assert!(
            !self.last_resp.has_this_cycle()
                || self.last_resp.get_this_cycle().is_waitstate()
                || self.data_phase.is_none(),
            "{}: Master Driver data phase {:?} not consumed after getting success: {:?}",
            <P as AHBPortConfig>::TAG,
            self.data_phase,
            self.last_resp,
        );
        assert!(
            self.data_phase.is_none() || self.last_resp.has_this_cycle(),
            "{}: Master Driver had data phase {:?} but didn't receive response: {:?}",
            <P as AHBPortConfig>::TAG,
            self.data_phase,
            self.last_resp,
        );
        assert!(
            !<P as Handler>::HAS_GRANTING_WIRE
                || self.addr_phase.is_none()
                || self.held.has_this_cycle(),
            "{}: Master Driver had addr phase {:?} but didn't receive grant wire: {:?}",
            <P as AHBPortConfig>::TAG,
            self.addr_phase,
            self.held,
        );
    }

    fn tick_extra(&mut self) {
        move_state_machine!(self.stm => MDStm = {GotMessages | SentMessages => TickExtra});
        let ahb_ready = self.last_resp.map_or(true, |r| r.HREADY());
        let advance_blocked = self.held.is_set_and(|r| *r);
        if ahb_ready && !advance_blocked {
            // We should advance - data_phase is emptied
            self.data_phase = self.addr_phase.take();
            self.just_advanced = self.data_phase.is_some();
        } else {
            self.just_advanced = false;
        }

        // We mark trailing addr_phase AFTER the first one
        if let Some(TransferInfo { ref mut status, .. }) = self.addr_phase {
            *status = advance_blocked.ife(
                TransferStatus::AddrPhaseDenied,
                TransferStatus::AddrPhaseStalled,
            );
        }
        // But we mark all data_phase BEFORE the last one
        if let Some(TransferInfo { ref mut status, .. }) = self.data_phase {
            *status = self
                .just_advanced
                .ife(TransferStatus::DataPhaseWaiting, *status);
        }
    }
}

impl<SC, P> Default for MasterDriver<SC, P>
where
    SC: Subcomponent<Member = Self>,
    Self: AHBPortConfig<Component = SC::Component, Data = P::Data>,
    P: Handler<Component = SC::Component> + AHBMasterPortOutput,
{
    fn default() -> Self {
        Self::new()
    }
}
impl<SC, P> AHBMasterPortInput for MasterDriver<SC, P>
where
    SC: Subcomponent<Member = Self>,
    Self: AHBPortConfig<Component = SC::Component, Data = P::Data>,
    P: Handler<Component = SC::Component> + AHBMasterPortOutput,
{
    fn on_ahb_input(
        comp: &mut Self::Component,
        ctx: &mut Context,
        msg: SlaveToMasterWires<Self::Data>,
    ) {
        let mut this = SubcomponentProxyMut::<SC>::from(comp);
        // We need to send transfers first, since we return ownership of transfers here
        if this.stm != MDStm::SentMessages {
            Self::dispatch_transfers(this.component_mut(), ctx);
        }
        move_state_machine!(this.stm => MDStm = {SentMessages => GotMessages});
        trace!(
            "{}: Master driver received {:?}, while addr_phase: {:?}, data_phase: {:?} ",
            <P as AHBPortConfig>::TAG,
            msg,
            this.addr_phase,
            this.data_phase,
        );

        this.last_resp.set_this_cycle(msg.meta);
        let has_addr = this.addr_phase.is_some();
        let has_data = this.data_phase.is_some();

        #[cfg(debug_assertions)]
        if let Some(msg @ SlaveToMasterWires::<DataBus> { meta, data, .. }) = static_downcast!(msg)
        {
            assert!(
                !data.is_present() || meta.is_done(),
                "Data present with resp!=Success"
            );
            assert!(
                !data.is_present() || has_data,
                "Data present without state in data_phase"
            );
            assert!(
                !meta.is_done()
                    || !has_data
                    || !this
                        .data_phase
                        .as_ref()
                        .is_some_and(|a| a.meta.is_reading())
                    || data.is_present(),
                "Master expected data with Success state, but it was not provided in addr: {:?}, data: {:?} recv msg: {:?}",
                this.addr_phase,
                this.data_phase,
                msg
            );
        }
        #[cfg(all(debug_assertions, feature = "cycle-debug-logger"))]
        if has_data {
            // TODO: use tag.eq_if_present? Do we still have some modules that reply with waitstate
            //       without remembering who was asking?
            assert_eq!(
                this.data_phase.as_ref().unwrap().tag.as_ref().unwrap(),
                &msg.sender_tag,
                "Reply tag mismatch! (if reply is ?unknown, consider softening this check)"
            );
        }

        if has_data && msg.meta.is_done() {
            this.data_phase.as_mut().unwrap().status = TransferStatus::DataPhaseDone;
        }

        <P as Handler>::tap_response(ctx, &msg, has_data, has_data);

        match msg.meta {
            AhbResponseControl::Success => {
                // Return ownership of the transfer back to the outer module.
                if let Some(mut data_phase) = this.data_phase.take() {
                    if data_phase.meta.is_reading() {
                        data_phase.data = Some(msg.data);
                    }
                    <P as Handler>::transfer_done(this.component_mut(), ctx, data_phase);
                }
            }
            AhbResponseControl::Pending => {
                <P as Handler>::transfers_will_stall(this.component_mut(), ctx, has_addr, has_data);
            }
            AhbResponseControl::Error1 => {
                let addr_phase = this.addr_phase.take();
                let data_phase = this.data_phase.take();
                <P as Handler>::transfers_aborted(
                    this.component_mut(),
                    ctx,
                    addr_phase,
                    data_phase,
                );
            }
            AhbResponseControl::Error2 => {
                debug_assert!(this.data_phase.is_none(), "Data phase should be aborted!");
            }
        }

        Self::consider_addr_phase_may_advance(this.component_mut(), ctx);
    }
}

impl<SC, P> MasterDriver<SC, P>
where
    SC: Subcomponent<Member = Self>,
    P: Handler<Component = SC::Component> + AHBMasterPortOutput,
{
    pub(crate) fn new() -> Self {
        Self {
            addr_phase: None,
            data_phase: None,
            last_resp: BufferFlop::new(),
            held: BufferFlop::new(),
            just_advanced: false,
            stm: MDStm::SentMessages,
            ph_sc: PhantomData,
        }
    }
    pub fn consider_addr_phase_may_advance(comp: &mut SC::Component, ctx: &mut Context) {
        let mut this = SubcomponentProxyMut::<SC>::from(comp);
        if this
            .last_resp
            .try_this_cycle()
            .is_some_and(|meta| meta.HREADY())
            && this.addr_phase.is_some()
            && (!<P as Handler>::HAS_GRANTING_WIRE
                || this.held.try_this_cycle().is_some_and(|h| !*h))
            && let Some(data) = <P as Handler>::transfer_will_advance(this.component_mut(), ctx)
        {
            this.addr_phase.as_mut().unwrap().data = Some(data);
        }
    }

    pub(crate) fn run_driver(comp: &mut SC::Component, _ctx: &mut Context) {
        let mut this = SubcomponentProxyMut::<SC>::from(comp);
        move_state_machine!(this.stm => MDStm::TickExtra => MDStm::RunDriver);

        // TODO: consider moving write_needs_data_this_cycle here
    }

    pub(crate) fn tock(comp: &mut SC::Component, ctx: &mut Context) {
        let mut this = SubcomponentProxyMut::<SC>::from(comp);
        if this.stm == MDStm::RunDriver {
            Self::dispatch_transfers(this.component_mut(), ctx);
        }
        debug_assert!(
            matches!(this.stm, MDStm::SentMessages | MDStm::GotMessages),
            "run_driver not called?"
        );
    }

    pub(crate) fn dispatch_transfers(comp: &mut SC::Component, ctx: &mut Context) {
        // TOOD: remove debug
        // eprintln!("{}", P::TAG);
        let mut this = SubcomponentProxyMut::<SC>::from(comp);
        move_state_machine!(this.stm => MDStm = {RunDriver => SentMessages}, "run_driver not called?");

        if this.addr_phase.is_none() && this.data_phase.is_none() {
            return;
        }

        // TODO: implement burts (round_robin arbiter needs it (Seq vs Nonseq))
        // TODO: pass locked state
        let mut addr_phase = MasterToSlaveAddrPhase::empty::<P>();
        if let Some(ref mut addr_info) = this.addr_phase {
            assert!(
                addr_info.meta.burst == Burst::Single,
                "bursts not implemented"
            );
            addr_phase.meta = TransferType::NonSeq(addr_info.meta.clone());

            // TODO: make it more lean
            #[cfg(feature = "cycle-debug-logger")]
            if let Some(ref tag) = addr_info.tag {
                addr_phase.tag = tag.clone();
            } else {
                addr_info.tag = Some(addr_phase.tag.clone());
            }
        }
        let mut data_phase = MasterToSlaveDataPhase::empty::<P>();
        // Last call for getting data for writing (we cannot do it in a natural way due to the borrow checker)
        if this
            .data_phase
            .as_ref()
            .is_some_and(|di| di.meta.is_writing() && di.data.is_none())
        {
            let data = <P as Handler>::write_needs_data_this_cycle(this.component_mut(), ctx);
            this.data_phase.as_mut().unwrap().data = Some(data);
        }

        if let Some(ref mut data_info) = this.data_phase {
            #[cfg(feature = "cycle-debug-logger")]
            if let Some(ref tag) = data_info.tag {
                data_phase.tag = tag.clone();
            }
            if data_info.meta.is_writing() {
                data_phase.data = data_info
                    .data
                    .clone()
                    .expect("Data presence was checked a few lines earlier...");
            }
        }
        let mut msg = MasterToSlaveWires {
            addr_phase,
            data_phase,
        };
        {
            // Trickery to allow multiple mutable views on subcomponent fields
            let this = this.this_mut();
            let addr_info = this.addr_phase.as_mut().map(TransferInfo::view);
            let data_info = this.data_phase.as_mut().map(TransferInfo::view);
            <P as Handler>::tap_request(ctx, &mut msg, addr_info, data_info);
        };
        <P as AHBMasterPortOutput>::send_ahb_output(this.component_mut(), ctx, msg);
    }

    // Call only on callback, panik if not
    pub(crate) fn view_data_phase(&mut self) -> TransferInfoView<'_, P> {
        let x = self
            .data_phase
            .as_mut()
            .expect("You shall call it only from callback");
        TransferInfo::view(x)
    }

    pub(crate) fn view_addr_phase(&mut self) -> TransferInfoView<'_, P> {
        let x = self
            .addr_phase
            .as_mut()
            .expect("You shall call it only from callback");
        TransferInfo::view(x)
    }

    pub(crate) fn try_request(&mut self, info: TransferInfo<P>) -> bool {
        assert!(matches!(self.stm, MDStm::RunDriver | MDStm::TickExtra));
        if self.can_pipeline() {
            self.try_force_request(info)
        } else {
            false
        }
    }

    #[allow(dead_code)]
    pub(crate) fn provide_data(&mut self, data: <P as AHBPortConfig>::Data) -> bool {
        assert!(matches!(self.stm, MDStm::RunDriver | MDStm::TickExtra));
        let trans = if self.just_advanced {
            self.data_phase.as_mut().unwrap()
        } else {
            self.addr_phase
                .as_mut()
                .expect("Attempting to provide data without transfer!")
        };
        trans.data.replace(data).is_none()
    }

    pub(crate) fn try_force_request(&mut self, mut info: TransferInfo<P>) -> bool {
        assert!(matches!(self.stm, MDStm::RunDriver | MDStm::TickExtra));
        if !self.can_force_pipeline() {
            return false;
        }
        <P as Handler>::override_prot(&mut info.meta.prot, &mut info.user);
        self.addr_phase = Some(info);
        true
    }

    pub(crate) fn try_force_cancel(&mut self) -> bool {
        assert!(matches!(self.stm, MDStm::RunDriver | MDStm::TickExtra));
        if self.addr_phase.is_none() {
            return true;
        }
        if <P as Handler>::AHB_LITE_COMPAT {
            return false;
        }
        let _cancelled = self.addr_phase.take();
        // should we call it here?
        // <P as Handler>::transfers_aborted(comp, ctx, cancelled, None);
        true
    }

    #[allow(dead_code)]
    pub(crate) fn lock(&mut self) {
        unimplemented!()
    }
    #[allow(dead_code)]
    pub(crate) fn unlock(&mut self) {
        unimplemented!()
    }

    pub fn has_data_phase(&self) -> bool {
        debug_assert!(
            self.stm != MDStm::GotMessages,
            "Data may be already handed back. You are not supposed to call this in tock."
        );
        self.data_phase.is_some()
    }

    pub fn has_addr_phase(&self) -> bool {
        debug_assert!(
            self.stm != MDStm::GotMessages,
            "Data may be already handed back. You are not supposed to call this in tock."
        );
        self.addr_phase.is_some()
    }

    // old will_accept_this_cycle?
    pub fn is_free(&self) -> bool {
        self.last_resp.map_or(true, |r| r.HREADY()) && !self.has_data_phase()
    }
    pub fn can_pipeline(&self) -> bool {
        !self.has_addr_phase()
    }
    pub fn can_force_pipeline(&self) -> bool {
        !<P as Handler>::AHB_LITE_COMPAT || self.can_pipeline()
    }
    pub fn pipeline_advanced(&self) -> bool {
        self.just_advanced
    }
}

#[cfg(feature = "cycle-debug-logger")]
use crate::proxy::CycleDebugLoggerProxy;
use crate::utils::{IfExpr, dife};
use owo_colors::OwoColorize;

impl<SC, P> AhbMasterPortInputWithGranting for MasterDriver<SC, P>
where
    SC: Subcomponent<Member = Self>,
    Self: AHBPortConfig<Component = SC::Component, Data = P::Data>,
    Self: AHBMasterPortOutput,
    P: Handler<Component = SC::Component> + AHBMasterPortOutput,
{
    fn on_grant_wire(comp: &mut Self::Component, ctx: &mut Context, granted: TrackedBool) {
        debug_assert!(
            <P as Handler>::HAS_GRANTING_WIRE,
            "{} got a GRANT message {:?} while it was configured without this wire.",
            <Self as AHBPortConfig>::TAG,
            granted
        );

        let mut this = SubcomponentProxyMut::<SC>::from(comp);
        trace!(
            "{}: Master driver {} (grant: {:?}), while addr_phase: {:?}, data_phase: {:?} ",
            <P as AHBPortConfig>::TAG,
            dife(*granted, "GRANTED".green(), "DENIED".red()),
            granted,
            this.addr_phase,
            this.data_phase,
        );

        this.held.set_this_cycle(!*granted);

        if !*granted {
            #[cfg(feature = "cycle-debug-logger")]
            CycleDebugLoggerProxy.on_free_static_str(
                ctx,
                <Self as AHBPortConfig>::get_name(),
                "DENIED",
            );
        } else if this.addr_phase.is_some() {
            P::address_presented(this.component_mut(), ctx, !P::AHB_LITE_COMPAT);
            Self::consider_addr_phase_may_advance(this.component_mut(), ctx);
        }
    }
}
