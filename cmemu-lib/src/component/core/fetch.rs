use std::fmt;
use std::fmt::{Debug, Formatter};

use log::{debug, trace};

use transfers::Transfers;

use crate::common::Word;
use crate::engine::{
    Context, DisableableComponent, StateMachine, Subcomponent, TickComponent, TickComponentExtra,
};
use crate::move_state_machine;
use crate::utils::{IfExpr, dife};
use cc2650_constants::FLASHMEM;
use cmemu_common::Address;

use crate::component::core::decode::{Brchstat, Decode, PipelineStepPack};
use crate::component::core::register_bank::XPSR;
use owo_colors::OwoColorize;

use super::{CoreComponent, RegisterBank, RegisterID, instruction};

mod piq;
pub(super) mod transfers; // TODO: export is_inflight() in a cleaner way

use crate::component::core::fetch::piq::PIQShiftMode;
use crate::confeature::cm_hyp::spec_fetch;
#[cfg(feature = "cycle-debug-logger")]
use crate::proxy::CycleDebugLoggerProxy;
pub(super) use piq::FDReg;
use piq::PrefetchInputQueue;
pub(super) use transfers::IBusDriver;

/// [ARM-ARM] B1.5.3 The vector table
///
/// TODO documentation reference for STM32VLDISCOVERY
const STACK_POINTER_INITIAL_VALUE_ADDRESS: Address = FLASHMEM::ADDR;

#[allow(rustdoc::broken_intra_doc_links)] // TODO(matrach): is the ordered list at the end any close to real?
/// Represents the Fetch stage of the Core.
///
/// It is responsible for:
/// - Fetching instructions to be executed by core;
/// - Booting procedure:
///   * Reading stack address and setting it up,
///   * Reading reset interrupt handler address from vector table and branching to it;
/// - Interrupt handler fetching:
///   * Reading interrupt handler address from vector table and branching to it.
///
/// It consists of three elements:
/// - State machine that generates transfers. The state can be modified using provided API.
/// - Logic for transfers tracking.
/// - Prefetch Input Queue that stores fetched data.
///
/// Within the cycle, there are four moments, when fetch does important work.
/// Related methods should be called in following order:
/// - [`tick_extra()`](Self::tick_extra()) (called automatically)
/// - [`handle_requested_data()`](Self::handle_requested_data())
/// - [`run_fetch()`](Self::run_fetch())
/// - [`on_ibus_data()`](Self::on_ibus_data()) (called by [`IBusDriver`])
///

#[derive(Subcomponent, TickComponent, DisableableComponent)]
#[subcomponent_1to1]
#[allow(clippy::struct_excessive_bools)] // To allow helper bools
pub(super) struct Fetch {
    // TODO: make it debug only
    /// Tracks in which part of the cycle we are.
    cycle_phase: CyclePhase,

    // TODO: switch to flop if possible in the future
    /// State of fetch in current cycle
    state: State,
    /// State of fetch in the next cycle
    next_state: State,
    next_override_state: Option<State>,

    /// Tracks all transfers requested by the Fetch subcomponent on Instruction bus.
    #[subcomponent(pub(super) Transfers)]
    pub(crate) transfers: Transfers, // TODO: export is_inflight() in a cleaner way

    /// Prefetch Input Queue
    piq: PrefetchInputQueue,

    multicycle_in_execute: bool,
    // prev was postpone, multicycle nop, Brchstat of moved, consumed items
    pipeline_shifted: Option<(bool, bool, Brchstat, u8)>,
    branching: bool,
    decode_stall: bool,

    // ------------------------------------------------------------------------
    // Buffers that delays API requests to the moment of running [`run_fetch()`].
    // ------------------------------------------------------------------------
    /// Target address of branch that was requested in current cycle.
    branch_buffer: Option<Word>,

    /// Target address of speculative branch that was requested in the current cycle.
    speculative_branch_buffer: Option<Word>,

    /// Whether speculative branch, requested in one of previous cycles,
    /// should be confirmed/cancelled in the current cycle.
    speculative_taken_buffer: Option<bool>,

    /// Address of entry in Vector Table that was requested to fetch in current cycle.
    vector_buffer: Option<VectorBuffer>,

    fetch_disable_buffer: bool,
}

struct VectorBuffer(Address, FetchedHandlerCallback);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum CyclePhase {
    /// [`Fetch::tick_extra()`] was called.
    TickExtra,

    /// [`Fetch::handle_requested_data()`] was called.
    HandleRequestedData,

    /// [`Fetch::tick_piq()`] was called.
    TickPiq,

    /// [`Fetch::run_fetch()`] was called.
    RunFetch,

    /// Tock phase `Fetch::Tock` and received messages
    Tock,
}
impl StateMachine for CyclePhase {}

/// State machine of the Fetch subcomponent. It generates transfers.
#[derive(Clone, Copy)]
enum State {
    Idle,
    Initial,
    RequestVectorTable {
        address: Address,
        callback_after_handler_fetched: FetchedHandlerCallback,
    },
    RequestInstruction {
        address: Address,
    },
    RequestInstructionSpeculatively {
        speculated: Address,
        // This may be different, if we asked for transfer
        if_confirm: Address,
        // This is whether Fetch presented the address internally
        // This doesn't mean it was visible on ICode, since it could be held back
        // by a DENY.
        had_addr_phase: bool,
        if_cancel: Address,
        is_branch: bool,
    },
    DelayedBranch {
        address: Address,
    },
}

// Internal type, but passed to CDL (so the type is pub(crate)).
#[cfg(feature = "cycle-debug-logger")]
#[derive(Clone, Copy, Debug)]
pub(crate) enum TransferType {
    StackPointer,
    Instruction,
    VectorTableValue,
}

#[cfg(feature = "cycle-debug-logger")]
impl From<TransferType> for &'static str {
    fn from(val: TransferType) -> Self {
        match val {
            TransferType::StackPointer => "FetchSP",
            TransferType::Instruction => "FetchInstr",
            TransferType::VectorTableValue => "FetchVector",
        }
    }
}

type FetchedHandlerCallback = Option<fn(&mut CoreComponent)>;
#[derive(Clone, Copy)]
pub(super) enum TransferState {
    AddrPhase,
    DataPhase,
}
#[derive(Clone, Copy)]
pub(super) enum DataReadCallback {
    SetStackPointerAndFetchResetVector,
    BranchToAddressAfterVectorCall { callback: FetchedHandlerCallback },
    AddToPiq { skip_half: bool },
    Ignore { in_state: TransferState },
}

impl Debug for DataReadCallback {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DataReadCallback::SetStackPointerAndFetchResetVector => "Reset",
                DataReadCallback::BranchToAddressAfterVectorCall { .. } => "BranchAfterVec",
                DataReadCallback::AddToPiq { skip_half: true } => "AddToPiq1",
                DataReadCallback::AddToPiq { skip_half: false } => "AddToPiq2",
                DataReadCallback::Ignore {
                    in_state: TransferState::AddrPhase,
                } => "IgnoreInAddr",
                DataReadCallback::Ignore {
                    in_state: TransferState::DataPhase,
                } => "IgnoreInData",
            }
        )
    }
}

impl Debug for VectorBuffer {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Vec@{:?}", self.0)
    }
}

impl Debug for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            State::Idle => write!(f, "Idle"),
            State::Initial => write!(f, "Initial"),
            State::RequestVectorTable {
                address,
                callback_after_handler_fetched,
            } => write!(
                f,
                "GetVec@{:?} {}",
                address,
                callback_after_handler_fetched.ife("with cb", "")
            ),
            State::RequestInstruction { address } => write!(f, "GetInstr@{address:?}"),
            State::RequestInstructionSpeculatively {
                speculated,
                if_confirm,
                if_cancel: following,
                had_addr_phase,
                is_branch,
            } => write!(
                f,
                "{}{}@{:?}{}{} (or {:?})",
                "Speculate".bright_yellow(),
                dife(*is_branch, " BRANCH".magenta(), ""),
                speculated,
                had_addr_phase.ife("+", "="),
                if_confirm.offset_from(*speculated),
                following
            ),
            State::DelayedBranch { address } => write!(f, "{}@{:?}", "Delayed".magenta(), address),
        }
    }
}

impl Fetch {
    pub(super) fn new() -> Self {
        Self {
            cycle_phase: CyclePhase::Tock,

            state: State::Initial,
            next_state: State::Initial,
            next_override_state: None,

            transfers: Transfers::new(),

            piq: PrefetchInputQueue::new(),
            multicycle_in_execute: false,
            branching: false,
            decode_stall: false,
            pipeline_shifted: None,

            branch_buffer: None,
            speculative_branch_buffer: None,
            speculative_taken_buffer: None,
            vector_buffer: None,
            fetch_disable_buffer: false,
        }
    }

    pub(super) fn run_driver(core: &mut CoreComponent, ctx: &mut Context) {
        IBusDriver::run_driver(core, ctx);
    }

    pub(super) fn tock(core: &mut CoreComponent, ctx: &mut Context) {
        let this = Self::component_to_member_mut(core);
        move_state_machine!(this.cycle_phase => CyclePhase::RunFetch => CyclePhase::Tock);
        IBusDriver::tock(core, ctx);
    }
}

impl TickComponentExtra for Fetch {
    #[cfg(debug_assertions)]
    fn tick_assertions(&self) {
        assert_eq!(self.cycle_phase, CyclePhase::Tock);

        trace!("End fo cycle: {:?}", self.transfers);
    }

    fn tick_extra(&mut self) {
        self.cycle_phase = CyclePhase::TickExtra;
        if let Some(over) = self.next_override_state.take() {
            self.next_state = over;
        }
        self.state = self.next_state;

        // Clear buffers
        self.fetch_disable_buffer = false;
        self.vector_buffer = None;
        self.branch_buffer = None;
        self.speculative_branch_buffer = None;
        self.speculative_taken_buffer = None;
    }
}

impl Fetch {
    /// Handles [`Transfers::delayed_transfer`] - runs associated callback.
    pub(super) fn handle_requested_data(core: &mut CoreComponent) {
        let this = Self::component_to_member_mut(core);
        move_state_machine!(this.cycle_phase => CyclePhase::TickExtra => CyclePhase::HandleRequestedData);

        // TODO: we should be able to move it to late tock phase of the previous cycle
        #[allow(unused_variables)]
        if let Some((data, address, callback)) = this.transfers.take_delayed_transfer() {
            match callback {
                DataReadCallback::Ignore { in_state } => {
                    // with the current ordering of execution, the transfer would be already marked as ignored,
                    // but it sometimes lands in the shadow buffer of PIQ
                    if let TransferState::DataPhase = in_state {
                        // TODO: or make the PIQ decide?
                        this.piq.ignored_data(data, address);
                    }
                }
                DataReadCallback::SetStackPointerAndFetchResetVector => {
                    let stack_pointer_value = data;
                    // TODO: change to `RegisterBank::set_msp(...)` call in the future
                    RegisterBank::set_register(core, RegisterID::SP, stack_pointer_value);

                    if let Some(entrypoint) = core.nonstandard_entrypoint {
                        Self::make_branch(core, entrypoint.into());
                    } else {
                        Self::make_vector_call(
                            core,
                            STACK_POINTER_INITIAL_VALUE_ADDRESS.offset(4),
                            None,
                        );
                    }
                }
                DataReadCallback::BranchToAddressAfterVectorCall { callback } => {
                    if let Some(callback) = callback {
                        callback(core);
                    }
                    let target_address = data;
                    Self::make_branch(core, target_address);
                }
                DataReadCallback::AddToPiq { skip_half } => {
                    this.piq.push_back_bytes(data, skip_half, address);
                }
            }
        }
    }

    // Update status
    pub(super) fn pipeline_moved(
        core: &mut CoreComponent,
        postponed_advance_head: bool,
        phony_multicycle_nop: bool,
        step: &PipelineStepPack,
        xpsr: XPSR,
    ) {
        let mut this = Self::get_proxy(core);
        debug_assert_eq!(this.cycle_phase, CyclePhase::HandleRequestedData);

        // TODO: when AGU is stalled, this is not up to date

        let instr = &step.instruction;
        let brstat = step.branch_kind.speculative_with_condition(instr, xpsr);
        let was_skipped = xpsr.in_it_block()
            && !xpsr.it_condition_passed()
            && !instr.is_unconditional_in_it_block();
        let atypical_multicycle = phony_multicycle_nop
            && (matches!(instr, instruction::Instruction::NoOperation)
                || (*spec_fetch::SKIPPED_CAUSES_PHONY_MULTICYCLE && was_skipped));
        debug!("Moving D/E pipeline: {:?}, brchstat: {:?}", instr, brstat);
        this.pipeline_shifted = Some((
            postponed_advance_head,
            atypical_multicycle,
            brstat,
            step.length,
        ));
    }

    // Do the actual shifting in PIQ
    pub(super) fn tick_piq(
        core: &mut CoreComponent,
        #[allow(unused)] ctx: &mut Context,
        was_last_cycle: bool,
        postpones: bool,
    ) {
        let mut this = Self::get_proxy(core);
        move_state_machine!(this.cycle_phase => CyclePhase::HandleRequestedData => CyclePhase::TickPiq);

        this.multicycle_in_execute = postpones;
        this.decode_stall = false;
        // TODO: research this!
        let shift_mode =
            if let Some((prev_postponed, atypical_multicycle_decoded, brchstat, consumed)) =
                this.pipeline_shifted.take()
            {
                #[cfg(feature = "cycle-debug-logger")]
                CycleDebugLoggerProxy.on_free_static_str(
                    ctx,
                    "fetch_atypical",
                    atypical_multicycle_decoded.ife("true", "false"),
                );
                let shift_mode = match () {
                    () if prev_postponed && brchstat.is_decode_time() => {
                        PIQShiftMode::HoldBranch(false)
                    }
                    () if this.branching => PIQShiftMode::CompleteBranch,
                    () if consumed == 1 => PIQShiftMode::ShiftHalf,
                    () => PIQShiftMode::ShiftFull,
                };
                this.branching = brchstat.is_decode_time(); // TODO: this is racy
                if atypical_multicycle_decoded {
                    trace!(
                        "Fetch next_postpones while recent multicycle is {}",
                        this.multicycle_in_execute
                    );
                    this.multicycle_in_execute = true;
                }
                shift_mode
            } else if was_last_cycle {
                let shift_mode = this
                    .branching
                    .ife(PIQShiftMode::CompleteBranch, PIQShiftMode::Populate);

                this.branching = false;
                this.decode_stall = Decode::is_agu_stalled(this.component_mut());

                shift_mode
            } else {
                // Reason test: ldr,CYC [multicycle instr], b .+4, add.w, add.w, ldr CYC -> or any b,
                // check for failed speculation
                this.branching
                    .ife(PIQShiftMode::HoldBranch(true), PIQShiftMode::Populate)
            };
        #[cfg(feature = "cycle-debug-logger")]
        CycleDebugLoggerProxy.on_free_static_str(ctx, "tick_piq_mode", shift_mode.into());
        this.piq.tick_piq(shift_mode);
    }

    /// Requests transfers basing on the current state and buffers.
    /// Sets state for the next cycle.
    #[allow(clippy::shadow_unrelated)]
    pub(super) fn run_fetch(core: &mut CoreComponent, ctx: &mut Context) {
        Fetch::log_cache_status(core, ctx, "fetch_7_run");
        let this = Self::component_to_member_mut(core);
        move_state_machine!(this.cycle_phase => CyclePhase::TickPiq => CyclePhase::RunFetch);

        let any_buffer_handled = Self::handle_buffered_signals(core, ctx);

        if !any_buffer_handled {
            Self::handle_fetch_state(core, ctx);
        }

        trace!(
            "After run_fetch: {:?}",
            Transfers::component_to_member(core)
        );
    }
}

impl Fetch {
    /// Handle buffered signals.
    /// When many signals were buffered, the most prioritized one is handled.
    /// The other signals are ignored.
    /// Returns information if any buffer was handled.
    fn handle_buffered_signals(core: &mut CoreComponent, ctx: &mut Context) -> bool {
        // Vector calls (interrupts and exceptions)
        let mut this = Self::get_proxy(core);

        trace!(
            "Fetch buffers report br: {:x?} spec_br: {:x?} spec_ok: {:x?} vec: {:x?} dis: {:?}. While state: {:x?} multi: {:?}, branch: {:?}, dstall: {:?}",
            this.branch_buffer,
            this.speculative_branch_buffer,
            this.speculative_taken_buffer,
            this.vector_buffer,
            this.fetch_disable_buffer,
            this.state.bold(),
            this.multicycle_in_execute,
            this.branching,
            this.decode_stall,
        );
        trace!("--> {:?}", this.transfers);

        if let Some(VectorBuffer(address, callback)) = this.vector_buffer {
            debug_assert!(address.is_aligned_to_4_bytes());

            // TODO: unify the code
            Transfers::ignore(this.component_mut());
            this.piq.prepare_for_impact(0xffff_ffff.into());
            let can_request = Transfers::can_request(this.component_mut());

            if can_request {
                this.next_state = State::Idle;
                Transfers::request(
                    core,
                    ctx,
                    address,
                    DataReadCallback::BranchToAddressAfterVectorCall { callback },
                    #[cfg(feature = "cycle-debug-logger")]
                    TransferType::VectorTableValue,
                );
            } else {
                this.next_state = State::RequestVectorTable {
                    address,
                    callback_after_handler_fetched: callback,
                }
            }
            return true;
        }

        // Branches
        if let Some(word) = this.branch_buffer {
            let address = Address::from(word).aligned_down_to_2_bytes();
            Self::flush(this.component_mut(), address);
            Self::request_instruction(this.component_mut(), ctx, address);
            return true;
        }

        // Speculative

        // First - check if there is a new request -> this will override the previous if possible
        if let Some(word) = this.speculative_branch_buffer {
            let address = Address::from(word).aligned_down_to_2_bytes();
            // There is a question whether the speculative branch may change an address while it
            // was not presented outside (got DENY from BuxMatrix), and then change it again.
            // See `misc/b_speculative_deny.asm` for a test that apparently disproves it.
            // The case looks like follows (PIQ full):
            // LDR:  D . . . XA XD <- should be one cycle to check the "new address"
            // NOP:  . . . . D  X  <- should be another one cycle to check "old again"
            // BREQ: . . . . .  D  <- NOT taken
            // next: . . . . FA ? <- the FA got DENY, so the branch could change the address here.
            //                       We test this by fetching something not from a line buffer
            // TODO: extend the test with conditional bx reg to check access to a different space
            // In case normal fetch cannot be cancelled, we test whether speculative one might be:
            // LDR:  D . . . XA XD <- should be one cycle to check the "new address"
            // BREQ: . . . . D  .  <- NOT taken
            // next: . . . . FA ? <- the FA got DENY, so the branch could change the address here.
            Self::request_instruction_speculatively(this.component_mut(), ctx, address, true);
            return true;
        }

        // Then check if it's time for cancellation
        if let Some(taken) = this.speculative_taken_buffer
            && !taken
            && let State::RequestInstructionSpeculatively {
                if_cancel: following,
                had_addr_phase,
                is_branch: true,
                ..
            } = this.state
        {
            if had_addr_phase {
                Transfers::ignore_prev_cycle(this.component_mut());
                this.piq.abort();
            }
            if Self::should_speculate(this.component_mut()) {
                Self::request_instruction_speculatively(
                    this.component_mut(),
                    ctx,
                    following,
                    false, // would be handled by the above
                );
            } else {
                Self::request_instruction(this.component_mut(), ctx, following);
            }
            return true;
        }

        // Disabling fetch during LSU branches
        // TODO, what if we have LDR x, pc; b(spec) ?
        if this.fetch_disable_buffer {
            Transfers::ignore(this.component_mut());
            this.piq.prepare_for_impact(0xffff_ffff.into());
            this.next_state = State::Idle;
            return true;
        }

        false
    }

    /// Usual fetch execution, when there is no buffered signal.
    #[allow(clippy::used_underscore_binding, reason = "Only in traces")]
    fn handle_fetch_state(core: &mut CoreComponent, ctx: &mut Context) {
        let mut this = Self::get_proxy(core);

        match this.state {
            State::Idle => {}
            State::Initial => {
                this.next_state = State::Idle;
                Transfers::request(
                    this.component_mut(),
                    ctx,
                    STACK_POINTER_INITIAL_VALUE_ADDRESS,
                    DataReadCallback::SetStackPointerAndFetchResetVector,
                    #[cfg(feature = "cycle-debug-logger")]
                    TransferType::StackPointer,
                );
            }
            State::RequestVectorTable {
                address,
                callback_after_handler_fetched,
            } => {
                // TODO: implement forceful cancellation for variants without compatible AHB-Lite
                Transfers::ignore(this.component_mut());
                this.piq.prepare_for_impact(0xffff_ffff.into());
                if Transfers::can_request(this.component_mut()) {
                    this.next_state = State::Idle;
                    // Note: exact timing in case of conflicts is not thoroughly verified
                    Transfers::request(
                        this.component_mut(),
                        ctx,
                        address,
                        DataReadCallback::BranchToAddressAfterVectorCall {
                            callback: callback_after_handler_fetched,
                        },
                        #[cfg(feature = "cycle-debug-logger")]
                        TransferType::VectorTableValue,
                    );
                }
            }
            State::RequestInstruction { address } => {
                if Self::should_speculate(this.component_mut()) {
                    Self::request_instruction_speculatively(
                        this.component_mut(),
                        ctx,
                        address,
                        false,
                    );
                } else {
                    Self::request_instruction(this.component_mut(), ctx, address);
                }
            }
            State::RequestInstructionSpeculatively {
                // the original address?
                speculated,
                if_confirm,
                if_cancel: following,
                had_addr_phase,
                is_branch,
            } => {
                // If we're here it means the speculation wasn't canceled -- we may get data in this tock
                debug!("Speculated confirmed?");
                if is_branch {
                    let held = this.multicycle_in_execute;
                    this.piq.confirm_speculated_branch(speculated, held);
                    if had_addr_phase {
                        // If we haven't advanced, data phase needs to be canceled.
                        Transfers::ignore_data_if_stalled(this.component_mut());
                        this.piq.reserve(); // reserve back in flight
                    } else {
                        Transfers::ignore(this.component_mut());
                    }
                    this.branching = true;
                }
                if had_addr_phase && this.piq.is_overcommit() {
                    // See misc/agu_double.tzst and large_tests/agu_vs_piq
                    // It turns out, that when we have mov and then ldr with addr dependency,
                    // in D the address of the next instruction is presented to the bus,
                    // but the transfer is tried to be aborted (but cannot due to AHB_LITE_COMPAT=1).
                    // If in the next instr, the instr will go to X, then fetch could reuse this transfer,
                    // but only while in address phase.
                    // This seems to only happen, when the fetched result won't fit in PIQ.
                    // In certain cases, this leads the address to be loaded two times.

                    // we need to cancel if not advanced
                    // TODO: why not always cancel prev_cycle? maybe we have broken logic on speculation
                    if let Some((addr, _cb)) = Transfers::cancel_addr_phase(this.component_mut()) {
                        trace!(
                            "{} speculative instruction fetch from {:?} {:?}",
                            "Cancelling".red(),
                            addr.bright_red(),
                            _cb,
                        );
                        this.piq.abort();
                        this.next_state = State::RequestInstruction { address: following }
                    } else {
                        // Too late to cancel anyway
                        this.next_state = State::RequestInstruction {
                            address: if_confirm,
                        }
                    }
                } else if Self::should_speculate(this.component_mut()) {
                    Self::request_instruction_speculatively(
                        this.component_mut(),
                        ctx,
                        if_confirm,
                        false,
                    );
                } else {
                    Self::request_instruction(this.component_mut(), ctx, if_confirm);
                }
            }
            State::DelayedBranch { address } => {
                Self::flush(this.component_mut(), address);
                Self::request_instruction(this.component_mut(), ctx, address);
            }
        }
    }

    fn should_speculate(core: &mut CoreComponent) -> bool {
        let (mut size, has_folded) = Decode::get_decoded_size(core);
        if *spec_fetch::CAN_SPEC_DISREGARDS_FOLDED {
            size -= has_folded.ife(1, 0);
        }
        let shift_if_advances = match size {
            0 => PIQShiftMode::Populate,
            1 => PIQShiftMode::ShiftHalf,
            2 => PIQShiftMode::ShiftFull,
            _ => unreachable!(),
        };
        let this = Fetch::get_proxy(core);
        !this.multicycle_in_execute
            && !this.branching
            && !this.decode_stall
            && !this
                .piq
                .speculated_would_fit(PIQShiftMode::HoldBranch(true))
            && this.piq.speculated_would_fit(shift_if_advances)
    }
}

impl Fetch {
    pub(super) fn log_cache_status(
        core: &CoreComponent,
        #[allow(unused)] ctx: &mut Context,
        tag: &'static str,
    ) {
        let this = Self::component_to_member(core);
        trace!("F-Cache at {tag} {}", &this.piq);
        #[cfg(feature = "cycle-debug-logger")]
        this.piq.log_in_cdl(ctx, tag);
    }

    pub(super) fn peek_head(core: &CoreComponent) -> FDReg {
        let this = Self::component_to_member(core);
        debug_assert_eq!(this.cycle_phase, CyclePhase::TickPiq);
        this.piq.peek_head()
    }
    pub(super) fn peek_shadow_head(core: &CoreComponent) -> FDReg {
        let this = Self::component_to_member(core);
        this.piq.peek_shadow_head()
    }

    pub(super) fn head_address(core: &CoreComponent) -> Address {
        let this = Self::component_to_member(core);
        debug_assert_eq!(this.cycle_phase, CyclePhase::TickPiq);
        this.piq.get_head_address()
    }
}

// API for state changing
impl Fetch {
    pub(super) fn make_delayed_branch(core: &mut CoreComponent, word: Word) {
        // Request done after calling [`run_fetch`],
        // so handle it in the next cycle via changing the state for the next cycle.
        let this = Self::component_to_member_mut(core);
        // LSU branches disable fetch from other parts
        debug_assert!(matches!(
            this.cycle_phase,
            CyclePhase::Tock | CyclePhase::TickPiq
        ));

        let address = Address::from(word).aligned_down_to_2_bytes();
        trace!("Make delayed branch: {:?} {:?}", word, address);

        debug_assert!(matches!(this.next_state, State::Idle) || this.fetch_disable_buffer);
        this.next_override_state = Some(State::DelayedBranch { address });
    }

    /// Address known too late for 50% stability on Instruction
    #[allow(dead_code)] // Leave for a while if we move the disabling logic to fetch
    pub(super) fn make_late_branch(core: &mut CoreComponent, word: Word) {
        Self::disable_fetch(core);
        Self::make_delayed_branch(core, word);
    }

    pub(super) fn make_branch(core: &mut CoreComponent, word: Word) {
        let this = Self::component_to_member_mut(core);
        debug_assert!(matches!(
            this.cycle_phase,
            CyclePhase::HandleRequestedData | CyclePhase::TickPiq
        ));
        debug_assert!(this.branch_buffer.is_none());
        this.branch_buffer = Some(word);

        trace!("Make branch: {:?} {:?}", this.cycle_phase, word);
    }

    pub(super) fn make_vector_call(
        core: &mut CoreComponent,
        address: Address,
        callback_on_handler_instruction_fetched: FetchedHandlerCallback,
    ) {
        let this = Self::component_to_member_mut(core);

        debug_assert!(address.is_aligned_to_4_bytes());

        debug_assert!(this.vector_buffer.is_none());
        this.vector_buffer = Some(VectorBuffer(
            address,
            callback_on_handler_instruction_fetched,
        ));

        trace!("Make vector call: {:?} {:?}", this.cycle_phase, address);

        if this.cycle_phase >= CyclePhase::RunFetch {
            todo!("this probably shouldn't happen.");
            // Request done after calling [`run_fetch`],
            // so handle it in the next cycle via changing the state for the next cycle.
            // let aligned_address = address.aligned_down_to_2_bytes();
            // this.next_state = State::RequestVectorTable {
            //     address: aligned_address,
            //     callback_after_handler_fetched: callback_on_handler_instruction_fetched,
            // };
        }
    }

    pub(super) fn make_decode_time_branch(
        core: &mut CoreComponent,
        address: Word,
        branch_kind: Brchstat,
        will_be_mispredicted: bool,
    ) {
        let this = Self::component_to_member_mut(core);
        debug_assert_eq!(this.cycle_phase, CyclePhase::TickPiq);
        debug_assert!(branch_kind.is_decode_time());
        let is_speculative = branch_kind.is_speculative();
        trace!(
            "Decode time branch [{:?}] to {:#x} {} {}",
            branch_kind.yellow(),
            address,
            dife(is_speculative, "SPECULATIVE".cyan(), "UNCOND"),
            dife(will_be_mispredicted, "MISPRED".bright_red(), "")
        );
        if is_speculative {
            this.speculative_branch_buffer = Some(address);
        } else {
            Self::make_branch(core, address);
        }
    }

    pub(super) fn confirm_speculative_branch(core: &mut CoreComponent) {
        let this = Self::component_to_member_mut(core);
        trace!("Fetch Speculative branch {}", "confirmed".bright_green());
        debug_assert!(matches!(
            this.cycle_phase,
            CyclePhase::HandleRequestedData | CyclePhase::TickPiq
        ));
        debug_assert!(this.speculative_taken_buffer.is_none());
        this.speculative_taken_buffer = Some(true);
    }

    /// This function is used to notify fetch that misinterpreted branch got to execution phase and is canceled.
    pub(super) fn cancel_mispredicted_branch(core: &mut CoreComponent) {
        let this = Self::component_to_member_mut(core);
        trace!("Fetch Speculative branch {}", "cancelled".bright_magenta());
        debug_assert!(matches!(
            this.cycle_phase,
            CyclePhase::HandleRequestedData | CyclePhase::TickPiq
        ));
        debug_assert!(this.speculative_taken_buffer.is_none());
        this.speculative_taken_buffer = Some(false);
    }

    /// Suppresses requesting new transfers by fetch.
    ///
    /// This is done during LSU unconditional branches
    /// (see: [ARM-TRM-G] 15.3, Notes under table 15-3)
    ///
    /// Fetch is enabled again by making branch.
    pub(super) fn disable_fetch(core: &mut CoreComponent) {
        debug!("Fetch disabled");
        let this = Self::component_to_member_mut(core);
        debug_assert!(matches!(
            this.cycle_phase,
            CyclePhase::HandleRequestedData | CyclePhase::TickPiq // TODO: uncomment? | CyclePhase::Tock
        ));
        this.fetch_disable_buffer = true;
    }
}

impl Fetch {
    fn request_instruction(core: &mut CoreComponent, ctx: &mut Context, address: Address) {
        let this = Self::component_to_member_mut(core);

        let next_addr = if this.piq.available_slots_for_new_entries() >= 2 {
            trace!("Requesting {:?}", address);
            Fetch::try_transfer(core, ctx, address)
        } else {
            address
        };
        let this = Self::component_to_member_mut(core);
        this.next_state = State::RequestInstruction { address: next_addr };
    }

    /// Like `request_instruction`, but attempts to change the current addr phase.
    #[allow(dead_code)] // leave for now
    fn force_request_instruction(core: &mut CoreComponent, ctx: &mut Context, address: Address) {
        let mut this = Self::get_proxy(core);
        if Transfers::cancel_addr_phase(this.component_mut()).is_some() {
            this.piq.abort();
        }
        Fetch::request_instruction(this.component_mut(), ctx, address);
    }

    fn request_instruction_speculatively(
        core: &mut CoreComponent,
        ctx: &mut Context,
        address: Address,
        branches: bool,
    ) {
        let mut this = Self::get_proxy(core);
        let following_address = if branches {
            match this.state {
                State::RequestInstructionSpeculatively {
                    if_cancel: following,
                    if_confirm,
                    had_addr_phase,
                    is_branch,
                    ..
                } => {
                    #[allow(clippy::collapsible_else_if)] // for clarity
                    if had_addr_phase {
                        if is_branch {
                            Transfers::ignore_prev_cycle(this.component_mut());
                            this.piq.abort();
                            following
                        } else if let Some((addr, DataReadCallback::AddToPiq { skip_half })) =
                            Transfers::cancel_addr_phase(this.component_mut())
                        {
                            this.piq.abort();
                            addr.offset(skip_half.ife(2, 0))
                        } else {
                            if_confirm
                        }
                    } else {
                        if let Some((addr, DataReadCallback::AddToPiq { skip_half })) =
                            Transfers::cancel_addr_phase(this.component_mut())
                        {
                            this.piq.abort();
                            addr.offset(skip_half.ife(2, 0))
                        } else {
                            following
                        }
                    }
                }
                State::RequestInstruction { address } => {
                    if let Some((addr, DataReadCallback::AddToPiq { skip_half })) =
                        Transfers::cancel_addr_phase(this.component_mut())
                    {
                        this.piq.abort();
                        addr.offset(skip_half.ife(2, 0))
                    } else {
                        address
                    }
                }
                _ => panic!("Mispredicted branch should not be requested in this cycle"),
            }
        } else {
            match this.state {
                State::RequestInstruction { address } => address,
                State::RequestInstructionSpeculatively {
                    if_cancel,
                    if_confirm,
                    is_branch,
                    ..
                } => is_branch.ife(if_cancel, if_confirm),
                _ => panic!("Speculative should not be requested in this cycle"),
            }
        };

        // This should be asserted in other place
        let if_confirm = if branches || Self::should_speculate(this.component_mut()) {
            trace!("Speculatively Requesting {:?}", address);
            Self::try_transfer(this.component_mut(), ctx, address)
        } else {
            address
        };
        this.next_state = State::RequestInstructionSpeculatively {
            speculated: address,
            if_confirm,
            had_addr_phase: address != if_confirm,
            if_cancel: following_address,
            is_branch: branches,
        };
    }

    // Returns next addr to try to fetch
    fn try_transfer(core: &mut CoreComponent, ctx: &mut Context, address: Address) -> Address {
        let mut this = Self::get_proxy(core);
        let can_request = Transfers::can_request(this.component_mut());
        // Try to reuse transfer in addr phase
        let req_addr = address.aligned_down_to_4_bytes();
        let skip_half = !address.is_aligned_to_4_bytes();

        let callback = Transfers::try_reuse_ignored_requested(this.component_mut(), req_addr);
        if let Some(callback) = callback {
            *callback = DataReadCallback::AddToPiq { skip_half };
            this.piq.reserve();
            req_addr.offset(4)
        } else if can_request {
            this.piq.reserve();
            Transfers::request(
                core,
                ctx,
                req_addr,
                DataReadCallback::AddToPiq { skip_half },
                #[cfg(feature = "cycle-debug-logger")]
                TransferType::Instruction,
            );
            req_addr.offset(4)
        } else {
            address
        }
    }

    fn flush(core: &mut CoreComponent, new_addr: Address) {
        let mut this = Self::get_proxy(core);
        Transfers::ignore(this.component_mut());
        this.branching = true;
        this.piq.branch(new_addr);
    }
}
