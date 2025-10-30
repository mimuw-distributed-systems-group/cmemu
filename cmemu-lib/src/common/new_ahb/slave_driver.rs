use crate::common::new_ahb::ports::{AHBPortConfig, AHBSlavePortInput, AHBSlavePortOutput};
use crate::common::new_ahb::signals::wire::{HIGH, LOW};
use crate::common::new_ahb::signals::{
    AhbResponseControl, MasterToSlaveAddrPhase, MasterToSlaveWires, SlaveToMasterWires,
    TransferMeta,
};
use crate::common::utils::SubcomponentProxyMut;
use crate::debug_move_state_machine;
use crate::engine::{
    BufferFlop, CombFlop, Context, DisableableComponent, SeqFlop, Subcomponent, TickComponent,
    TickComponentExtra,
};
#[cfg_attr(not(debug_assertions), allow(unused))]
use log::{debug, trace};
use owo_colors::OwoColorize;
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;

pub(crate) mod faking_slave_driver;
pub(crate) mod stateless_simplifiers;

///////////////////////////////////////////////////////////////////////
///
////////
/// This is roughly based on [ARM-SDK-TRM].
/// Especially sections 3.2 and 3.7 (AHB to SRAM interface module)
#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) enum SimpleResponse<D> {
    Success(D),
    Pending,
    #[allow(dead_code)]
    Error,
}

impl Copy for SimpleResponse<()> {}

impl<T> SimpleResponse<T> {
    pub(crate) fn map_success<D, F>(self, f: F) -> SimpleResponse<D>
    where
        F: FnOnce(T) -> D,
    {
        use SimpleResponse::*;
        match self {
            Success(d) => Success(f(d)),
            Error => Error,
            Pending => Pending,
        }
    }
}

impl<T> From<SimpleResponse<T>> for AhbResponseControl {
    fn from(sr: SimpleResponse<T>) -> Self {
        match sr {
            SimpleResponse::Error => AhbResponseControl::Error1,
            SimpleResponse::Pending => AhbResponseControl::Pending,
            SimpleResponse::Success(_) => AhbResponseControl::Success,
        }
    }
}

pub(crate) type SimpleWriteResponse = SimpleResponse<()>;
impl SimpleResponse<()> {
    /// An alias to be returned from *write* methods.
    pub const SUCCESS: SimpleWriteResponse = SimpleResponse::Success(());

    #[allow(dead_code)]
    pub fn with_data<D>(self, data: D) -> SimpleResponse<D> {
        use SimpleResponse::{Error, Pending, Success};
        match self {
            Success(()) => Success(data),
            Error => Error,
            Pending => Pending,
        }
    }
}

pub(crate) enum WriteMode {
    /// `write_data` is called early in the next cycle as data phase
    /// useful for triggers
    #[allow(dead_code)]
    #[doc(alias = "Synchronous")]
    Registered,
    /// `write_data` is called late in the same cycle as data phase
    /// useful for setting Flop(Banks)
    #[doc(alias = "Asynchronous")]
    Combinatorial,
    // if there would be a need, add Mixed and fn write_reminder(slave, ctx, req)
    // that calls write_data asynchronously and then frees the slave from having to
    // remember to do stuff on the next cycle
    // Mainly useful for synchronous sync-down AHB bridge
}

/// Handler for events from the Synchronous Interface, to be implemented at the `P` type parameter.
///
/// If you don't need to access all the protocol information, consider implementing a simpler
/// trait from [`stateless_simplifiers`] or [`faking_slave_driver`].
/// This one will be implemented automatically.
pub(crate) trait SimpleHandler: AHBPortConfig {
    /// Determines whether `write_data` will be called in the tock phase (`Combinatorial`)
    /// or same as `read_data`: on calls to `run_driver` (`Registered`).
    const WRITE_MODE: WriteMode;

    /// Called on the next Tick on read transfers (always registered => data phase tick).
    ///
    /// When returning a `Pending` status, this will be called again
    /// with the same address/size on the next cycle.
    /// `Success` returns the data to be put on the wires and sets HREADY to high.
    fn read_data(
        slave: &mut Self::Component,
        ctx: &mut Context,
        request: TransferMeta,
    ) -> SimpleResponse<Self::Data>;

    /// Called on the next Tick on new write transfers.
    ///
    /// That is, it is called at the start of the first data-phase cycle of a write transfer,
    /// where the data has not yet arrived.
    /// The Slave must decide on waitstating without looking at the data, only the address.
    ///
    /// Use [`SimpleWriteResponse::SUCCESS`] to return a success state.
    fn pre_write(
        slave: &mut Self::Component,
        ctx: &mut Context,
        request: TransferMeta,
    ) -> SimpleWriteResponse;

    /// Delivers write data at time specified in `WRITE_MODE`.
    ///
    ///
    /// Because the response is propagated on the next cycle, it is important to observe
    /// the `post_success` parameter when using [`WriteMode::Combinatorial`]:
    /// it is true when this method will be called for the last time for a given transfer.
    /// Therefore, you most likely want to do the actual write at that point.
    ///
    /// Returning a `Pending` status will make the method be called again (by the protocol).
    /// Use [`SimpleWriteResponse::SUCCESS`] to return a success state.
    /// Returning a non-success on `post_success` is an error, as we already sent a
    /// success on the bus for this transfer.
    fn write_data(
        slave: &mut Self::Component,
        ctx: &mut Context,
        request: TransferMeta,
        data: Self::Data,
        post_success: bool,
    ) -> SimpleWriteResponse;
}

#[cfg(debug_assertions)]
#[derive(Debug, PartialEq)]
enum SssiLastState {
    PreTick,
    Tick,
    Tock,
}

/// SSSI: Simple Synchronous Slave Interface -> Synchronous (Registering) Interface.
///
/// The AHB Slave driver implementation assures there is no combinatorial dependency
/// between data incoming and outgoing on the bus (i.e., responding based on data in the same cycle).
/// A component communicates with the driver by implementing a Handler interface (on `P`)
/// and calling `tick` (`run_driver`)/`tock` methods.
/// In the simple case, the interface registers (latches) all incoming signals,
/// and presents them to the Handler on the beginning (`tick`) of the next cycle.
/// In a slightly more complex case, write data may be presented to the component in `tock`,
/// but the response will be in the next cycle.
///
/// The response is based solely on information set up before `tock`, therefore,
/// since in the given cycle messages go out before they go in, they cannot have intra-cycle dependency.
/// Also, because Decoders need to reflect the HREADY signal, we will always send an AHB message
/// before receiving one. (That is, this holds for "non-null" messages.)
///
/// See documentation for [`SimpleHandler`] trait and its simplifications for details on usage.
#[derive(Subcomponent, TickComponent)]
pub(crate) struct SimpleSynchronousSlaveInterface<SC, P>
where
    SC: Subcomponent<Member = SimpleSynchronousSlaveInterface<SC, P>>,
    P: AHBSlavePortOutput<Component = SC::Component> + SimpleHandler,
{
    /// AHB bus data to be sent at the next tock.
    // Visible in tests
    pub(super) delayed_reply: Option<SlaveToMasterWires<P::Data>>,
    /// Just the response part from `delayed_reply` to know what we sent once it's consumed.
    #[flop]
    response: BufferFlop<AhbResponseControl>,

    /// The last valid AHB address wires (could be held in a waitstate).
    /// That is, the 'current value' of the flop corresponds to the address phase associated
    /// with the current data phase, and there is no data phase if this flop is not set.
    // It is a CombFlop for the `default_keep_current_as_next` method, so we hold the data
    // in case we don't get a msg while returning a wait-state.
    #[flop]
    address_phase_reg: CombFlop<MasterToSlaveAddrPhase>,

    /// Captured data for sequential write handling: it will be delivered in tick in this case.
    /// None means already consumed data (also in combinatorial write mode).
    #[flop]
    data: SeqFlop<Option<P::Data>>,
    /// Response generated by a combinatorial write handling, to be sent out on the next cycle.
    #[flop]
    registered_response: SeqFlop<SimpleWriteResponse>,

    #[cfg(debug_assertions)]
    stm: SssiLastState,

    phantom_subcomponent: PhantomData<SC>,
    phantom_connection: PhantomData<P>,
}

impl<SC, P> DisableableComponent for SimpleSynchronousSlaveInterface<SC, P>
where
    SC: Subcomponent<Member = Self>,
    P: AHBSlavePortOutput<Component = SC::Component> + SimpleHandler,
{
    fn can_be_disabled_now(&self) -> bool {
        debug!("Can SS be disabled? {self:?}");
        // In essence, no ongoing transfer (only Idle/Idle is allowed)
        self.response.try_prev_cycle().is_none_or(|r| r.is_done())
            && self.response.try_this_cycle().is_none_or(|r| r.is_done())
            && self.delayed_reply.is_none()
            && !self
                .address_phase_reg
                .is_set_and(|a| a.meta.is_address_valid())
            && !self
                .address_phase_reg
                .is_next_set_and(|a| a.meta.is_address_valid())
            && self.data.is_empty()
            && self.registered_response.is_empty()
        // TODO: this should be asked at a predetermined point in the cycle!
        // && self.stm == SssiLastState::Tock
    }
}

impl<SC, P> Debug for SimpleSynchronousSlaveInterface<SC, P>
where
    SC: Subcomponent<Member = Self>,
    P: AHBSlavePortOutput<Component = SC::Component> + SimpleHandler,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "SlaveDriver of {} ", <P as AHBPortConfig>::TAG.bold())?;
        write!(
            f,
            "next_reply {:?} (was stat {:?}) ",
            self.delayed_reply, self.response
        )?;
        write!(
            f,
            "waiting addr/data ph: {:?} (data {:?} / resp: {:?}), ",
            self.address_phase_reg, self.data, self.registered_response
        )?;
        #[cfg(debug_assertions)]
        write!(f, "stm: {:?}", self.stm)?;
        Ok(())
    }
}

impl<SC, P> SimpleSynchronousSlaveInterface<SC, P>
where
    SC: Subcomponent<Member = Self>,
    P: AHBSlavePortOutput<Component = SC::Component> + SimpleHandler,
{
    pub(crate) fn new() -> Self {
        // We don's support Racy yet
        assert!(matches!(
            P::WRITE_MODE,
            WriteMode::Registered | WriteMode::Combinatorial
        ));
        Self {
            response: BufferFlop::new_from(AhbResponseControl::Success),
            delayed_reply: None,
            address_phase_reg: Default::default(),
            data: Default::default(),
            registered_response: Default::default(),
            #[cfg(debug_assertions)]
            stm: SssiLastState::Tock,

            phantom_subcomponent: PhantomData,
            phantom_connection: PhantomData,
        }
    }

    pub(crate) fn tock(slave: &mut SC::Component, ctx: &mut Context) {
        let mut this = SC::get_proxy(slave);
        debug_move_state_machine!(this.stm => SssiLastState = {Tick | Tock => Tock});

        if let Some(msg) = this.delayed_reply.take() {
            <P as AHBSlavePortOutput>::send_ahb_output(this.component_mut(), ctx, msg);
        }
    }
}

impl<SC, P> TickComponentExtra for SimpleSynchronousSlaveInterface<SC, P>
where
    SC: Subcomponent<Member = Self>,
    P: AHBSlavePortOutput<Component = SC::Component> + SimpleHandler,
{
    #[cfg(debug_assertions)]
    fn tick_assertions(&self) {
        assert_eq!(self.stm, SssiLastState::Tock);

        assert!(self.delayed_reply.is_none(), "Data reply was not consumed!");
    }

    fn tick_extra(&mut self) {
        debug_move_state_machine!(self.stm => SssiLastState::Tock => SssiLastState::PreTick);
    }
}

// Our port
impl<SC, P> AHBPortConfig for SimpleSynchronousSlaveInterface<SC, P>
where
    SC: Subcomponent<Member = Self>,
    P: AHBSlavePortOutput<Component = SC::Component> + SimpleHandler,
{
    type Data = <P as AHBPortConfig>::Data;
    type Component = SC::Component;
    const TAG: &'static str = <P as AHBPortConfig>::TAG;
}

impl<SC, P> AHBSlavePortInput for SimpleSynchronousSlaveInterface<SC, P>
where
    SC: Subcomponent<Member = Self>,
    P: AHBSlavePortOutput<Component = SC::Component> + SimpleHandler,
{
    fn on_ahb_input(
        slave: &mut Self::Component,
        ctx: &mut Context,
        msg: MasterToSlaveWires<Self::Data>,
    ) {
        let mut this = SC::get_proxy(slave);
        #[cfg(debug_assertions)]
        trace!(
            "SS-SlaveInterface {} got msg {:?} while stm={:?}",
            <Self as AHBPortConfig>::TAG,
            msg,
            this.stm
        );

        debug_move_state_machine!(this.stm => SssiLastState = {Tick | Tock => Tock});

        let MasterToSlaveWires {
            addr_phase,
            data_phase,
        } = msg;

        // If we haven't finished the transfer, ignore the address phase request
        // Note: in essence AHB-lite slaves are allowed to sample ADDR during waitstates (HREADYIN=L)
        // But [ARM-TRM] 2.3.1 "Bus interfaces" indicates that in the default synthesis
        // parameters Cortex-M3/4 are allowed to change the address during waitstates (AMBA incompatibility).
        // Yet, they note that if slaves don't sample ADDR during waitstates, everything works.
        // It was later found that CC2650 indeed has the AHB_CONST_CTRL flag set,
        // but none of the slaves seems to depend on this (e.g., it could speed up Cache a bit)...
        //
        // Therefore, right now we ignore address phase wires during a wait state.
        if addr_phase.ready == LOW
            || this.response.get_this_cycle().is_waitstate()
            || !addr_phase.is_selected()
        {
            if this.address_phase_reg.is_set() && this.response.get_this_cycle().is_waitstate() {
                this.address_phase_reg.keep_current_as_next();
            }
        } else {
            this.address_phase_reg.set_next(addr_phase);
        }

        // Errors are two-phase, so we can drop the pipelined address phase.
        if !this.address_phase_reg.is_set()
            || this.response.get_this_cycle() == &AhbResponseControl::Error2
        {
            return;
        }
        let request: &MasterToSlaveAddrPhase = &this.address_phase_reg;
        if request.meta.is_address_valid() {
            #[cfg(all(feature = "cycle-debug-logger", debug_assertions))]
            debug_assert!(
                request.tag == data_phase.tag,
                "CDL Tag mismatch at {} between addr_phase: {:?} and data_phase: {:?}!",
                <Self as AHBPortConfig>::TAG,
                request.tag,
                data_phase.tag
            );
        }
        // Reads will always be handled in Tick, and we need only the address phase data.
        // Let's handle incoming data for a write, which may be delayed or delivered right away.
        if request.meta.is_writing() {
            let resp = this.response.get_this_cycle();
            debug_assert!(
                resp != &AhbResponseControl::Error2,
                "Master cannot send data during second cycle of Error."
            );
            // debug_assert!(
            //     data_phase.data.is_present(),
            //     "No data provided after write cycle!"
            // );

            if resp == &AhbResponseControl::Error1 {
                // We are in error state (the master sent us data, but we were responding with error)
                // Take merci on the user and don't deliver this data.
                return;
            }

            match P::WRITE_MODE {
                WriteMode::Registered => this.data.set_next(Some(data_phase.data)),
                WriteMode::Combinatorial => {
                    // We may be calling the handler until it returns success and one last time!
                    let post_success = resp.HREADYOUT() == HIGH;
                    let meta = request.meta.meta().unwrap().clone();
                    let response = P::write_data(
                        this.component_mut(),
                        ctx,
                        meta,
                        data_phase.data,
                        post_success,
                    );

                    if post_success {
                        // Write "generates" response always one cycle ahead
                        debug_assert!(matches!(response, SimpleResponse::SUCCESS));
                    } else {
                        // We will have to return the response
                        this.data.set_next(None);
                        this.registered_response.set_next(response);
                    }
                }
            }
        }
    }
}

impl<SC, P> Default for SimpleSynchronousSlaveInterface<SC, P>
where
    SC: Subcomponent<Member = Self>,
    P: AHBSlavePortOutput<Component = SC::Component> + SimpleHandler,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<SC, P> SimpleSynchronousSlaveInterface<SC, P>
where
    SC: Subcomponent<Member = Self>,
    P: AHBSlavePortOutput<Component = SC::Component> + SimpleHandler,
    P::Data: Default,
{
    /// Runs events handlers for this cycle (defined via instance of [`SimpleHandler`] trait),
    /// and prepare a response for `tock`.
    ///
    /// Call this method during `tick(...)` of component.
    pub(crate) fn run_driver(slave: &mut SC::Component, ctx: &mut Context) {
        let mut this = SubcomponentProxyMut::<SC>::from(slave);

        debug_move_state_machine!(this.stm => SssiLastState::PreTick => SssiLastState::Tick);

        let mut data = P::Data::default();
        let response = if *this.response.get_prev_cycle() == AhbResponseControl::Error1 {
            AhbResponseControl::Error2
        } else if this.address_phase_reg.is_set() {
            let request: &MasterToSlaveAddrPhase = &this.address_phase_reg;
            AhbResponseControl::from(match request.meta.meta() {
                None => SimpleResponse::SUCCESS,
                Some(meta) => {
                    // We need to clone as this might be necessary in the next cycle
                    let meta = meta.clone();

                    if meta.is_writing() {
                        // we already started receiving the data
                        if this.data.is_set() {
                            // Data may be None for combinatorial writing mode
                            // TODO: it's likely invalid "post-success" for the Registered mode
                            let post_success = this.response.get_prev_cycle().HREADYOUT() == HIGH;
                            match this.data.take() {
                                Some(d) => {
                                    P::write_data(this.component_mut(), ctx, meta, d, post_success)
                                }
                                None => this.registered_response.take(),
                            }
                        } else {
                            // we haven't received data yet
                            // in case of combinatorial write handler, we will register
                            // the response
                            P::pre_write(this.component_mut(), ctx, meta)
                        }
                    } else {
                        P::read_data(this.component_mut(), ctx, meta).map_success(|d| {
                            data = d;
                        })
                    }
                }
            })
        } else {
            AhbResponseControl::Success
        };
        this.response.set_this_cycle(response);
        this.delayed_reply = Some(SlaveToMasterWires {
            meta: response,
            data,
            ..SlaveToMasterWires::empty_addr_reply::<P>(
                this.address_phase_reg.deref_or(&Default::default()),
            )
        });
        if response.is_waitstate() {
            // This is to handle non-compliant masters, which doesn't send an idle msg when expecting a response
            this.address_phase_reg.default_keep_current_as_next();
        }
    }
}
