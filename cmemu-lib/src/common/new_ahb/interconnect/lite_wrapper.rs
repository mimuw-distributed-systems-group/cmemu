//! A `LiteWrapper` is an interface which wraps AHB-"almost lite" interconnect into a fully compliant one.
//! In practice, this means enforcing ordering on AHB-Lite message processing (right-to-left, then left-to-right):
//! - all downstream slaves must reply first (modulo inactive ones),
//! - they flow through the interconnect up to its inputs, which reply upstream,
//! - only then messages coming from upstream (left-to-right) may be passed to the interconnect,
//!   and through the interconnect to its slaves (there is caching required).
//!
//! In other ways, status (`SlaveToMasterWires`) flows from the slaves first all the way up.
//! This is required here because `MasterToSlavesWires` contains HREADYIN field, which needs
//! to be "reflected" (search for this term in the codebase):
//! - if an `InputStage<A>` has a low HREADYOUT (waitstate), then it must get low HREADYIN;
//! - this is also true just for slaves, but the main use case is this wire flowing through
//!   decoders from one slave to the next one, so it knows whether to start processing the request:
//!
//! ```text
//!                 /----------------- LITE WRAPPER ----------------\  /- 1. signals waitstate
//!                |             [interconnect inside]              |  |
//!     MasterA  - ║ - InputStage<A>  - Decoder<A>  - Output<O1>  - ║ <- Slave1
//!                |\                          \                    |
//!                | \- 2. reflects ws         \----- Output<O2>  - ║ -> Slave2
//!                |                                                |  \- 3. low HREADIN
//! ```
//!
//! The bold cells on the box boundary on the figure above correspond to
//! `LiteOutput` and `LiteInput` types sich are ATM unhygienically created by the main macro.
//! Moreover, ATM the macro unhygienically assumes the presence of ``Input`` and ``Output`` ports.
use crate::common::new_ahb::signals::MasterToSlaveWires;
use enum_map::EnumArray;
use std::fmt::Debug;

pub(crate) trait LiteWrapperCfg {
    type Data: Debug + Clone + Default + 'static;
    type InputTag: EnumArray<Option<MasterToSlaveWires<Self::Data>>>
        + EnumArray<Option<bool>>
        + EnumArray<bool>
        + Copy
        + Debug;
    type OutputTag: EnumArray<Option<bool>> + EnumArray<bool> + Copy + Debug;
}
// This exports LiteWrapper, LiteInput & LiteOutput
// In the future the goal is to make it generic if possible.
#[macro_export]
macro_rules! codegen_line_wrapper_for_interconnect {
    ($icname:ident; $vis:vis) => {
      codegen_line_wrapper_for_interconnect!(@derive_fix $icname; $icname; $vis);
    };
    (@derive_fix $icname:ident; $icname_path:path; $vis:vis) => {

make_port_struct!($vis LiteInput<PM>);
make_port_struct!($vis LiteOutput<PM>);
bridge_ports!(<PM> @no_link @slave LiteInput<PM> => @auto_configured @slave Input<PM> where LiteInput<PM>: AHBSlavePortOutput);
bridge_ports!(<PM> @no_link @auto_configured @master Output<PM> => @master LiteOutput<PM> where LiteOutput<PM>: AHBMasterPortOutput);

impl<PM> AHBMasterPortInput for LiteOutput<PM>
where
    Self: AHBPortConfig<Component = <LiteWrapper as Subcomponent>::Component>,
    Output<PM>: AHBMasterPortInput<Component = Self::Component, Data = Self::Data>,
    <LiteWrapper as LiteWrapperCfg>::OutputTag: FromMarker<PM>,
    PM: Unit,
{
    fn on_ahb_input(
        comp: &mut Self::Component,
        ctx: &mut Context,
        msg: SlaveToMasterWires<Self::Data>,
    ) {
        let this = <LiteWrapper as Subcomponent>::component_to_member_mut(comp);
        log::trace!("Wrapper response from slave {:?}: {:?}", PM::unit(), msg);
        if this.active {
            let tag = FromMarker::<PM>::from_marker();
            #[cfg(debug_assertions)]
            {
                assert!(
                    !this.dedup_check_output[tag],
                    "Got duplicate response from {:?}: {:?}",
                    PM::unit(),
                    msg
                );
                this.dedup_check_output[tag] = true;
            }
            if this.output_expect[tag].is_some() {
                this.missing_slaves -= 1;
                Output::<PM>::on_ahb_input(comp, ctx, msg);
                LiteWrapper::try_dispatch(comp, ctx);
            } else {
                assert!(
                    msg.meta.is_done(),
                    "Got an unexpected reply to {}, but it is usually ok if empty: {:?}",
                    <Self as AHBPortConfig>::TAG,
                    msg
                );
            }
        } else {
            Output::<PM>::on_ahb_input(comp, ctx, msg);
        }
    }
}
impl<PM> AHBMasterPortOutput for Output<PM>
where
    Self: AHBPortConfig<Component = <LiteWrapper as Subcomponent>::Component>,
    LiteOutput<PM>: AHBMasterPortOutput<Component = Self::Component, Data = Self::Data>,
    <LiteWrapper as LiteWrapperCfg>::OutputTag: FromMarker<PM>,
{
    fn send_ahb_output(
        comp: &mut Self::Component,
        ctx: &mut Context,
        msg: MasterToSlaveWires<Self::Data>,
    ) {
        let this = <LiteWrapper as Subcomponent>::component_to_member_mut(comp);
        // we were supposed to not require reply from NoSel, but only when it goes to address phase
        // addr_phase.HREADY is already reflected
        if !msg.addr_phase.ready || msg.addr_phase.is_selected() {
            this.output_expect.next_builder()[FromMarker::<PM>::from_marker()] =
                Some(msg.addr_phase.ready);
        }
        LiteOutput::<PM>::send_ahb_output(comp, ctx, msg);
    }
}

impl<PM> AHBSlavePortOutput for Input<PM>
where
    Self: AHBPortConfig<
        Component = <LiteWrapper as Subcomponent>::Component,
        Data = <LiteWrapper as LiteWrapperCfg>::Data,
    >,
    LiteInput<PM>: AHBSlavePortOutput<Component = Self::Component, Data = Self::Data>,
    <LiteWrapper as LiteWrapperCfg>::InputTag: FromMarker<PM>,
    PM: Unit,
{
    fn send_ahb_output(
        comp: &mut Self::Component,
        ctx: &mut Context,
        msg: SlaveToMasterWires<Self::Data>,
    ) {
        let this = <LiteWrapper as Subcomponent>::component_to_member_mut(comp);
        let tag = FromMarker::<PM>::from_marker();
        this.input_reply[tag] = Some(msg.meta.HREADYOUT());
        #[cfg(debug_assertions)]
        {
            assert!(
                !this.dedup_check_input[tag],
                "Got duplicate response for {:?}: {:?}",
                PM::unit(),
                msg
            );
            this.dedup_check_input[tag] = true;
        }
        LiteInput::<PM>::send_ahb_output(comp, ctx, msg);
    }
}

// The most important struct -- the reflection HREADYIN is modified here!
impl<PM> AHBSlavePortInput for LiteInput<PM>
where
    Self: AHBPortConfig<
        Component = <LiteWrapper as Subcomponent>::Component,
        Data = <LiteWrapper as LiteWrapperCfg>::Data,
    >,
    Input<PM>: AHBSlavePortInput<Component = Self::Component, Data = Self::Data>,
    <LiteWrapper as LiteWrapperCfg>::InputTag: FromMarker<PM>,
    PM: Unit,
{
    fn on_ahb_input(
        comp: &mut Self::Component,
        ctx: &mut Context,
        mut msg: MasterToSlaveWires<Self::Data>,
    ) {
        let mut this = SubcomponentProxyMut::<LiteWrapper>::from(comp);
        let tag = FromMarker::<PM>::from_marker();
        if this.active {
            // TODO: shouldn't we trace data phase
            this.input_expect.next_builder()[tag] = msg.addr_phase.is_selected();
            LiteWrapper::tock(this.component_mut(), ctx);
            msg.addr_phase.ready &= this.input_reply[tag].unwrap_or(true);
        }

        if this.needs_buffering(tag) {
            log::trace!("Wrapper buffering {:?}: {:?}", PM::unit(), msg);
            this.input_buffer[tag] = Some(msg);
        } else {
            Input::<PM>::on_ahb_input(this.component_mut(), ctx, msg);
        }
    }
}

#[derive(Subcomponent, TickComponent)]
#[subcomponent_1to1]
$vis struct LiteWrapper {
    input_buffer: EnumMap<
        <Self as LiteWrapperCfg>::InputTag,
        Option<MasterToSlaveWires<<Self as LiteWrapperCfg>::Data>>,
    >,
    input_reply: EnumMap<<Self as LiteWrapperCfg>::InputTag, Option<bool>>,
    #[flop]
    input_expect: CombFlop<EnumMap<<Self as LiteWrapperCfg>::InputTag, bool>>,
    #[flop]
    output_expect: CombFlop<EnumMap<<Self as LiteWrapperCfg>::OutputTag, Option<bool>>>,

    #[subcomponent($icname_path)]
    interconnect: $icname,
    missing_slaves: u8,
    pub active: bool,
    force_tocked_ic: bool,

    #[cfg(debug_assertions)]
    dedup_check_input: EnumMap<<Self as LiteWrapperCfg>::InputTag, bool>,
    #[cfg(debug_assertions)]
    dedup_check_output: EnumMap<<Self as LiteWrapperCfg>::OutputTag, bool>,
}

impl $crate::engine::DisableableComponent for LiteWrapper {
    fn can_be_disabled_now(&self) -> bool {
        if log::log_enabled!(log::Level::Debug) {
            // Note: this would be evaluated before checking the condition anyway
            let inner = self.interconnect.can_be_disabled_now();
            log::debug!("Can LW be disabled? {:?} ... (wrapped IC: {:?})",
                self, inner);
        }
        // Let's ignore IC state for a while, as we should know all the details
        if !self.active {
            return self.interconnect.can_be_disabled_now();
        }
        self.missing_slaves == 0
        && !self.input_expect.is_set_and(|m| m.iter().any(|(_k, b)| *b))
        && !self.input_expect.is_next_set_and(|m| m.iter().any(|(_k, b)| *b))
        && !self.output_expect.is_set_and(|m| m.iter().any(|(_k, b)| b.is_some()))
        && !self.output_expect.is_next_set_and(|m| m.iter().any(|(_k, b)| b.is_some()))
    }
}

impl std::fmt::Debug for LiteWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LiteWrapper of {} {{", stringify!($icname))?;
        write!(f, "{}, miss_s:{:?}, f_tock:{:?}, ",
            if self.active {"ON"} else {"OFF"}, self.missing_slaves, self.force_tocked_ic)?;
        write!(f, "in_reply:{:?}, ", self.input_reply)?;
        write!(f, "in_exp:{:?}, ", self.input_expect)?;
        write!(f, "out_exp:{:?}, ", self.output_expect)?;
        write!(f, "}}")?;
        Ok(())
    }
}

impl Default for LiteWrapper {
    fn default() -> Self {
        Self {
            input_buffer: Default::default(),
            input_reply: Default::default(),
            // on HRESET we need all
            input_expect: CombFlop::new_from(enum_map!(_ => true)),
            output_expect: CombFlop::new_from(enum_map!(_ => None)),
            interconnect: $icname::new(),
            missing_slaves: 0,
            active: true,
            force_tocked_ic: false,
            #[cfg(debug_assertions)]
            dedup_check_input: Default::default(),
            #[cfg(debug_assertions)]
            dedup_check_output: Default::default(),
        }
    }
}

impl LiteWrapper {
    pub(crate) fn new() -> Self {
        Default::default()
    }
    fn try_dispatch(comp: &mut <Self as Subcomponent>::Component, ctx: &mut Context) {
        let mut this = SubcomponentProxyMut::<LiteWrapper>::from(comp);
        debug_assert!(this.active, "Tried to dispatch from a transparent wrapper?");
        LiteWrapper::tock(this.component_mut(), ctx);

        // Do once
        if this.missing_slaves > 0 {
            return;
        }
        // TODO: be more lazy?

        for key in iter_enum::<<LiteWrapper as LiteWrapperCfg>::InputTag>() {
            if this.needs_buffering(key) || this.input_buffer[key].is_none() {
                continue;
            }

            let mut msg = this.input_buffer[key].take().unwrap();
            msg.addr_phase.ready &= this.input_reply[key].unwrap_or(true);
            log::trace!("Wrapper dispatch buffered for {:?}: {:?}", key, msg);
            $icname::on_ahb_soft_tagged_input(this.component_mut(), ctx, key, msg);
        }
    }

    fn needs_buffering(&self, tag: <Self as LiteWrapperCfg>::InputTag) -> bool {
        if !self.active {
            return false;
        }
        // TODO(matrach): optimize the check for any missing slaves, to prevent buffering as much
        //                as possible. This is currently required, as otherwise output stage
        //                may get a request before the reply and fail assertions.
        (self.input_expect[tag] && self.input_reply[tag].is_none()) || self.missing_slaves > 0
    }
}
impl LiteWrapper {
    pub(super) fn tick(comp: &mut <Self as Subcomponent>::Component, ctx: &mut Context) {
        $icname::tick(comp, ctx);
    }
    pub(super) fn tock(comp: &mut <Self as Subcomponent>::Component, ctx: &mut Context) {
        let mut this = SubcomponentProxyMut::<LiteWrapper>::from(comp);
        // This sends messages from DefaultSlave and buffering InputStages
        if !this.force_tocked_ic {
            this.force_tocked_ic = true;
            log::trace!("Reverse tock for an interconnect?");
            $icname::tock(this.component_mut(), ctx);

            if this.active {
                Self::try_dispatch(this.component_mut(), ctx);
            }
        }
    }
}

impl TickComponentExtra for LiteWrapper {
    #[cfg(debug_assertions)]
    fn tick_assertions(&self) {
        if !self.input_expect.is_set() {
            // First cycle?
            return;
        }

        if self.active {
            debug_assert!(
                self.output_expect.values().zip(self.dedup_check_output.values()).all(|(&exp, &got)| exp.is_none() || got),
                "Expected replies from slaves, but not got from everyone. Got {:?}, expected (Some) {:?}",
                self.dedup_check_output, self.output_expect
            );
            debug_assert!(
                self.input_expect.values().zip(self.dedup_check_input.values()).all(|(&exp, &got)| !exp || got),
                "Expected replies to masters, but not got for everyone. Got {:?}, expected (Some) {:?}",
                self.dedup_check_input, self.input_expect
            );
            assert!(
                self.input_buffer.values().all(|d| d.is_none()),
                "Input buffer was not fully consumed!. Missing (Some) {:?}",
                self.input_buffer
            );
        }
    }

    fn tick_extra(&mut self) {
        self.missing_slaves = u8::try_from(self.output_expect.values().filter(|r| r.is_some()).count()).unwrap();

        self.force_tocked_ic = false;
        self.input_buffer.clear();
        self.input_reply.clear();

        // initialize next expectations to empty
        self.input_expect.next_builder();
        self.output_expect.next_builder();

        if !self.active {
            self.input_expect.ignore();
        }

        #[cfg(debug_assertions)]
        {
            self.dedup_check_input.clear();
            self.dedup_check_output.clear();
        }
    }
}
    };
}
