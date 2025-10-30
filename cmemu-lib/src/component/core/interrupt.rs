use cc2650_constants::operation::{ExecutionMode, StackPointer};
use log::{debug, trace};

use crate::common::new_ahb::databus::DataBus;
use crate::common::{BitstringUtils, Word, bitstring::constants as bsc, new_ahb};
use crate::component::core::register_bank::ItState;
use crate::component::core::{
    CoreComponent, Execute, Fetch, RegisterBank,
    builtins::have_dsp_ext,
    register_bank::{RegisterID, XPSR},
};
use crate::component::nvic::{CoreStateChange, InterruptData, InterruptId, SCBRegister, VTOR};
use crate::engine::{
    CombFlopMemoryBankSimple, Context, DisableableComponent, SeqFlop, SeqFlopMemoryBankSimple,
    Subcomponent, TickComponent, TickComponentExtra,
};
#[cfg(feature = "cycle-debug-logger")]
use crate::proxy::CycleDebugLoggerProxy;
use crate::{bitstring_concat, bitstring_extract, bitstring_substitute};
use cmemu_common::Address;

use super::builtins::have_fp_ext;
use super::decode::TriggerData;
use super::lsu::ReadDataCallback;
use super::{LSU, PipelineAction};

// Remark: [TI-TRM] 4.1.2: Interrupt is an exception signaled by a software signal.
// Terms "exception" and "interrupt" are used interchangeable in docs.
// To not confuse readers with using sometimes exception and sometimes interrupt,
// interrupt is used to describe both exceptions and interrupts.

// TODO: [ARM-ARM] A2.4 Exceptions, faults and interrupts actually define the difference:
// An exception can be caused by the execution of an exception generating instruction or triggered as a response to a
// system behavior such as an interrupt, memory management protection violation, alignment or bus fault, or a debug
// event. Synchronous and asynchronous exceptions can occur within the architecture.
// -> In essence, an interrupt can cause an exception -- an abrupt change of program flow.

/// [ARM-ARM] B1.5.8 Exception return behavior
///
enum ReturnBehavior {
    ExcReturn {
        return_to: ExecutionMode,
        return_stack: StackPointer,
        _extended_frame: bool,
    },
    Reserved,
    NormalBranch,
}

impl ReturnBehavior {
    /// [ARM-ARM] Table B1-8
    const TO_HANDLER_MODE: Word = Word::from_const(0xFFFF_FFF1);
    /// [ARM-ARM] Table B1-8
    const TO_THREAD_MODE_WITH_MSP: Word = Word::from_const(0xFFFF_FFF9);
    /// [ARM-ARM] Table B1-8
    const TO_THREAD_MODE_WITH_PSP: Word = Word::from_const(0xFFFF_FFFD);

    /// [ARM-ARM] B1.5.8 Exception return behavior / under Table B1-9
    fn from_addr_and_state(core: &CoreComponent, addr: Word) -> Self {
        if RegisterBank::get_xpsr(core).current_mode() != ExecutionMode::Handler {
            ReturnBehavior::NormalBranch
        } else {
            Self::from(addr)
        }
    }
}

impl From<Word> for ReturnBehavior {
    #[inline(always)]
    fn from(addr: Word) -> Self {
        let addr_31_28 = bitstring_extract!(addr<31:28> | 4 bits);
        let addr_27_5 = bitstring_extract!(addr<27:5> | 23 bits);
        let addr_4 = addr.get_bit(4);
        let addr_3_0 = bitstring_extract!(addr<3:0> | 4 bits);
        if addr_31_28 != bsc::C_1111 {
            return ReturnBehavior::NormalBranch;
        }

        #[allow(clippy::manual_assert)] // Not an assertion.
        if !addr_27_5.is_ones() {
            panic!("UNPREDICTABLE exception return value {addr:?} on bits 27-5");
        }

        #[allow(clippy::manual_assert)] // Not an assertion.
        if !addr_4 && !have_fp_ext() {
            panic!("Exception return with Addr[4]==0 is UNPREDICTABLE without FP extension");
        }

        match addr_3_0 {
            bsc::C_0001 => ReturnBehavior::ExcReturn {
                return_to: ExecutionMode::Handler,
                return_stack: StackPointer::Main,
                _extended_frame: !addr_4,
            },
            bsc::C_1001 => ReturnBehavior::ExcReturn {
                return_to: ExecutionMode::Thread,
                return_stack: StackPointer::Main,
                _extended_frame: !addr_4,
            },
            bsc::C_1101 => ReturnBehavior::ExcReturn {
                return_to: ExecutionMode::Thread,
                return_stack: StackPointer::Process,
                _extended_frame: !addr_4,
            },
            _ => {
                paranoid!(error, "Reserved EXC_RETURN is not properly supported yet!");
                ReturnBehavior::Reserved
            }
        }
    }
}

/// `InterruptEntryAndExitHandler` is a core subcomponent that is responsible for:
/// executing interrupt entry and interrupt exit.
/// It provides implementation of:
/// - stacking
/// - register updates
/// - unstacking
/// - exception return
#[derive(Subcomponent, TickComponent, TickComponentExtra, DisableableComponent)]
#[subcomponent_1to1]
pub(super) struct InterruptEntryAndExitHandler {
    interrupt_entry_exit_state: InterruptEntryExitState,
    #[flop]
    fetching_handler_state: SeqFlopMemoryBankSimple<FetchingHandlerState>,

    /// Stores interrupt data to be used in the next cycle for the interrupt entry.
    #[flop]
    interrupt_data: SeqFlop<InterruptEntryData>,
    /// [ARM-ARM] B1.5.8
    /// Stores `exc_return` value to be used in the next cycle for the interrupt exit.
    #[flop]
    exc_return: SeqFlop<Word>,
    /// `CombFlop` is necessary here, because there is one cycle during which
    /// [`NVICComponent`] can send data of new tail chained interrupt and
    /// [`Self::tail_chained_interrupt`] is set to [`Option::None`], because
    /// the value has been used.
    ///
    /// [`NVICComponent`]: crate::component::nvic::NVICComponent
    #[flop]
    tail_chained_interrupt: CombFlopMemoryBankSimple<Option<InterruptData>>,
    /// Stores a read-only copy of [`VTOR`]. Can be updated
    /// only by [`Self::update_vector_table_offset_register()`], that is used to
    /// synchronize value.
    #[flop]
    vtor_copy: VTOR,
}

/// `InterruptEntryExitState` enum represents the state of the interrupt entry and exit.
#[derive(Debug)]
enum InterruptEntryExitState {
    None,
    ReadyToEntry(EntryState),
    /// [ARM-TDG] 9.1.1
    EntryStacking(StackingState),
    TailChain(TailChainState),
    ReadyToExit(ExitState),
    /// [ARM-TDG] 9.2
    ExitUnstacking(UnstackingState),
}

#[derive(Debug)]
struct EntryState {
    interrupt_data: InterruptEntryData,
}

#[derive(Debug)]
enum StackingState {
    InProgress(StackingInProgressState),
    Finished(InterruptId),
}

#[derive(Debug)]
struct StackingInProgressState {
    /// Iterator over registers that have to be pushed onto stack.
    stack_frame_iterator: stack_frame::Iter,
    sp_address_after_stacking: Address,

    /// Register, which transfer is currently in address phase.
    address_phase_reg: Option<stack_frame::Register>,
    /// Register, which transfer is currently in data phase.
    data_phase_reg: Option<stack_frame::Register>,

    sp_align_was_required: bool,

    interrupt_id: InterruptId,

    this_instr_addr: Word,
    next_instr_addr: Word,
}

#[derive(Debug)]
struct TailChainState {
    exc_return: Word,
    tail_chained_interrupt_data: InterruptData,
}

#[derive(Debug)]
struct ExitState {
    exc_return: Word,
}

#[derive(Debug, Copy, Clone)]
pub(super) struct InterruptEntryData {
    pub(super) interrupt_id: InterruptId,
    pub(super) this_instr_addr: Word,
    pub(super) next_instr_addr: Word,
}

#[derive(Debug)]
struct UnstackingState {
    /// Iterator over registers that have to be popped from stack.
    stack_frame_iterator: stack_frame::Iter,
    sp_address_before_unstacking: Address,

    /// Register, which transfer is currently in address phase.
    address_phase_reg: Option<stack_frame::Register>,
    /// Register, which transfer is currently in data phase.
    data_phase_reg: Option<stack_frame::Register>,

    /// If set this state can request fetching instructions from this address.
    address_to_branch_to_after_unstacking: Option<Word>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FetchingHandlerState {
    None,
    InProgress,
    Finished,
}

// API to execute interrupt entry and exit.
impl InterruptEntryAndExitHandler {
    pub(super) fn new() -> Self {
        Self {
            interrupt_entry_exit_state: InterruptEntryExitState::None,
            fetching_handler_state: SeqFlopMemoryBankSimple::new(FetchingHandlerState::None),
            interrupt_data: SeqFlop::new(),
            exc_return: SeqFlop::new(),
            tail_chained_interrupt: CombFlopMemoryBankSimple::new(None),
            vtor_copy: VTOR::initial(),
        }
    }

    pub(super) fn init_interrupt_entry(
        core: &mut CoreComponent,
        interrupt_data: InterruptEntryData,
    ) {
        let mut this = Self::get_proxy(core);
        this.interrupt_data.set_next(interrupt_data);
    }

    pub(super) fn delay_interrupt_entry_if_scheduled(core: &mut CoreComponent) {
        let mut this = Self::get_proxy(core);
        if this.interrupt_data.is_set() {
            this.interrupt_data.keep_current_as_next();
        }
    }

    /// [ARM-ARM] B1.5.8 Exception return behavior.
    /// Exception return operation will be started in the next cycle after calling this.
    pub(super) fn init_exception_return(core: &mut CoreComponent, exc_return: Word) {
        let this = Self::component_to_member_mut(core);
        this.exc_return.set_next(exc_return);
    }

    pub(super) fn tail_chain_interrupt(
        core: &mut CoreComponent,
        interrupt_data: Option<InterruptData>,
    ) {
        debug_assert_eq!(
            RegisterBank::get_xpsr(core).current_mode(),
            ExecutionMode::Handler
        );

        let mut this = Self::get_proxy(core);
        this.tail_chained_interrupt.set_next(interrupt_data);
    }

    pub(super) fn run_interrupt_entry_and_exit(
        core: &mut CoreComponent,
        ctx: &mut Context,
    ) -> PipelineAction {
        Self::update_state_if_new_events_arrived(core, ctx);
        Self::increment_exception_counter_if_possible(core, ctx);
        Self::run_interrupt_entry_and_exit_main_logic(core, ctx)
    }
}

// Main logic helpers.
impl InterruptEntryAndExitHandler {
    /// Must be called at the beginning of [`Self::run_interrupt_entry_and_exit`].
    fn update_state_if_new_events_arrived(core: &mut CoreComponent, ctx: &Context) {
        let mut this = Self::get_proxy(core);

        debug_assert!(!(this.interrupt_data.is_set() && this.exc_return.is_set()));

        if this.interrupt_data.is_set() {
            let mut interrupt_data = *this.interrupt_data;

            // XXX: Currently next_instr_addr is originally captured when the interrupt first shows up.
            // This might happen during a multicycle instruction that in the end changes next_instr_addr such as `pop {r4-r5, pc}`.
            interrupt_data.next_instr_addr = Execute::this_instr_addr(this.component());

            this.interrupt_entry_exit_state =
                InterruptEntryExitState::ReadyToEntry(EntryState { interrupt_data });

            debug!(
                "Interrupt entry: {:?} at {:?}",
                *this.interrupt_data,
                ctx.cycle_no()
            );
        }

        if let Some(exc_return) = this.exc_return.try_take() {
            // TODO: do we really need nested Option inside or could we just use try_take?
            this.interrupt_entry_exit_state = (*this.tail_chained_interrupt).map_or_else(
                || InterruptEntryExitState::ReadyToExit(ExitState { exc_return }),
                |interrupt_data| {
                    InterruptEntryExitState::TailChain(TailChainState {
                        exc_return,
                        tail_chained_interrupt_data: interrupt_data,
                    })
                },
            );
            this.tail_chained_interrupt.set_next(None);
        }
    }

    /// Must be called in [`Self::run_interrupt_entry_and_exit`] after
    /// [`Self::update_state_if_new_events_arrived`] to make sure that it's called in
    /// the correct state.
    fn increment_exception_counter_if_possible(core: &CoreComponent, ctx: &mut Context) {
        let this = Self::component_to_member(core);

        if !matches!(
            this.interrupt_entry_exit_state,
            InterruptEntryExitState::None
        ) {
            core.dwt.increment_exception_counter(ctx);
        }
    }

    /// Executes main logic of interrupt entry/exit.
    /// Must be called in [`Self::run_interrupt_entry_and_exit`] after
    /// [`Self::update_state_if_new_events_arrived`] to make sure that it's called
    /// in the correct state.
    #[allow(clippy::shadow_unrelated)]
    #[allow(clippy::match_same_arms)]
    fn run_interrupt_entry_and_exit_main_logic(
        core: &mut CoreComponent,
        ctx: &mut Context,
    ) -> PipelineAction {
        let interrupt_entry_exit_state =
            &Self::component_to_member(core).interrupt_entry_exit_state;
        if matches!(
            interrupt_entry_exit_state,
            InterruptEntryExitState::ReadyToEntry(_) // | InterruptEntryExitState::EntryStacking(_)
        ) {
            Fetch::disable_fetch(core);
        }

        let this = Self::component_to_member(core);

        trace!(
            "Interrupt entry/exit state: {:?}",
            this.interrupt_entry_exit_state
        );
        match this.interrupt_entry_exit_state {
            InterruptEntryExitState::ReadyToEntry(EntryState { interrupt_data }) => {
                Self::fetch_handler(core, interrupt_data.interrupt_id);
                Self::run_stacking(core, ctx);
                PipelineAction::RunDecode(TriggerData::IgnoreCurrent)
            }
            InterruptEntryExitState::EntryStacking(ref state) => {
                if matches!(state, StackingState::InProgress(..)) {
                    Self::run_stacking(core, ctx);
                }

                let this = Self::component_to_member(core);
                let handler_state = *this.fetching_handler_state;
                debug_assert_ne!(handler_state, FetchingHandlerState::None);

                let state = Self::get_state(core).unwrap_stacking_state();
                match (state, handler_state) {
                    (StackingState::Finished(interrupt_id), FetchingHandlerState::Finished) => {
                        let interrupt_id = *interrupt_id;
                        Self::complete_entry(core, ctx, interrupt_id);
                        PipelineAction::RunFullPipeline
                    }
                    (_, FetchingHandlerState::InProgress) => PipelineAction::None,
                    // TODO: we should probably start decoding as early as possible? TODO: test with first instr being decode branch
                    (_, FetchingHandlerState::Finished) => PipelineAction::None,
                    (StackingState::InProgress(..), _) => PipelineAction::None,
                    (StackingState::Finished(_), FetchingHandlerState::None) => unreachable!(),
                }
            }
            InterruptEntryExitState::ReadyToExit(..) => {
                Self::run_exception_return(core, ctx);
                Self::run_unstacking(core, ctx);
                // Unlike entry to interrupt, exit doesn't ignore currently
                // decoding instruction. It is done by `Execute` subcomponent.
                // The reason for this asymmetry is that `Execute` doesn't know
                // about the raised interrupt, but it intercepts the exit from interrupt,
                // so it's able to ignore instruction decoding.
                PipelineAction::None
            }
            InterruptEntryExitState::TailChain(TailChainState {
                tail_chained_interrupt_data,
                ..
            }) => {
                let this = Self::component_to_member(core);

                match *this.fetching_handler_state {
                    FetchingHandlerState::None => {
                        Self::fetch_handler(core, tail_chained_interrupt_data.interrupt_id);
                        Self::run_tail_chain(core, ctx);
                        PipelineAction::RunDecode(TriggerData::IgnoreCurrent)
                    }
                    FetchingHandlerState::InProgress => PipelineAction::None,
                    FetchingHandlerState::Finished => {
                        Self::complete_entry(core, ctx, tail_chained_interrupt_data.interrupt_id);
                        PipelineAction::RunFullPipeline
                    }
                }
            }
            InterruptEntryExitState::ExitUnstacking(..) => {
                let finished = Self::run_unstacking(core, ctx);
                if finished {
                    Execute::restore_execution(core);

                    // In case of nested interrupts, core should return to
                    // handling preempted interrupt. Its id can be read from
                    // xPSR, because at this stage this register has been popped
                    // from the stack.
                    let preempted_interrupt = InterruptId::try_from_exception_number(
                        RegisterBank::get_xpsr(core).get_exception_number() as usize,
                    )
                    .map(|id| InterruptData { interrupt_id: id })
                    .ok();
                    core.nvic
                        .change_core_state(ctx, CoreStateChange::FinishedExit(preempted_interrupt));

                    PipelineAction::RunFullPipeline
                } else {
                    PipelineAction::None
                }
            }
            InterruptEntryExitState::None => PipelineAction::RunFullPipeline,
        }
    }

    /// Can be called ONLY by core which receives new value of [`VTOR`].
    pub(super) fn update_vector_table_offset_register(core: &mut CoreComponent, vtor: Word) {
        let mut this = Self::get_proxy(core);
        this.vtor_copy.write(vtor, Word::from(!0_u32));
    }
}

// Interrupt entry (based on ExceptionEntry() and PushStack() pseudocode).
impl InterruptEntryAndExitHandler {
    /// `run_stacking` executes stacking.
    #[allow(clippy::shadow_unrelated)]
    fn run_stacking(core: &mut CoreComponent, #[allow(unused)] ctx: &mut Context) {
        let state = Self::get_state(core);
        trace!("Stacking in state: {:?}", state);

        #[cfg(feature = "cycle-debug-logger")]
        CycleDebugLoggerProxy::new().on_core_run_stacking(ctx);

        if let InterruptEntryExitState::ReadyToEntry(EntryState { interrupt_data }) = state {
            let this_instr_addr = interrupt_data.this_instr_addr;
            let next_instr_addr = interrupt_data.next_instr_addr;
            let interrupt_id = interrupt_data.interrupt_id;

            // TODO: After implementing CCR.STKALIGN register in NVIC, use it to align SP address.
            // [ARM-ARM] B1.5.7 - stack pointer is aligned based on value of CCR.STKALIGN bit.
            // [ARM-TDG] 9.1.1 - in Cortex-M3 revision 2, stack frame is aligned to 8 bytes by default.
            let sp_address = Address::from(RegisterBank::get_register(core, RegisterID::SP));
            let sp_align_required = !sp_address.is_aligned_to_8_bytes();
            let sp_address_aligned = sp_address.aligned_down_to_8_bytes();

            // Writeback is done here from the same reasons as in StoreMultipleDecrementBefore instruction.
            // See `cmemu-lib/src/component/core/execute/instruction.rs` ctrl+f "A7.7.160".
            #[allow(
                clippy::cast_sign_loss,
                clippy::cast_possible_truncation,
                clippy::cast_possible_wrap
            )]
            let address_offset = (-4_i32 * stack_frame::SIZE as i32) as u32;
            let address = sp_address_aligned.offset(address_offset);
            RegisterBank::set_register(core, RegisterID::SP, address.into());

            Self::set_state(
                core,
                InterruptEntryExitState::EntryStacking(StackingState::InProgress(
                    StackingInProgressState {
                        stack_frame_iterator: stack_frame::new_register_iterator(),
                        sp_address_after_stacking: address,
                        address_phase_reg: None,
                        data_phase_reg: None,
                        sp_align_was_required: sp_align_required,
                        interrupt_id,
                        this_instr_addr,
                        next_instr_addr,
                    },
                )),
            );
        }

        if LSU::can_request(core) {
            let state = Self::get_state(core).unwrap_stacking_in_progress_state();
            let _burst_start = state.address_phase_reg.is_some();
            state.data_phase_reg = state.address_phase_reg.take();
            state.address_phase_reg = state.stack_frame_iterator.next().copied();
            let is_last = state.stack_frame_iterator.clone().next().is_none();

            if let Some(reg) = state.address_phase_reg {
                let offset = stack_frame::register_offset_from_sp(reg);
                let address = state.sp_address_after_stacking.offset(offset);

                // We can sample data in this cycle already to offload most of the logic to LSU
                let data = match reg {
                    stack_frame::Register::XPSR => {
                        // If stack alignment was required, it is remembered in `xPSR` before pushing value to the stack.
                        let sp_align_was_required = state.sp_align_was_required;
                        let xpsr_value = RegisterBank::get_xpsr(core)
                            .with_sp_align_was_required_bit(sp_align_was_required);

                        Word::from(xpsr_value)
                    }
                    // [ARM-ARM] B1.5.6 Exception entry behavior - PushStack() pseudocode.
                    stack_frame::Register::Reg(RegisterID::PC) => {
                        let interrupt_id = state.interrupt_id;
                        Self::return_address(core, interrupt_id, ctx)
                    }
                    stack_frame::Register::Reg(reg_id) => RegisterBank::get_register(core, reg_id),
                };

                // TODO: consider implementing and using burst transfers
                LSU::request_write_multiple(
                    core,
                    address.into(),
                    new_ahb::Size::Word,
                    ReadDataCallback::WriteCallbacks {
                        get_data: |_core, _reg, _size| unreachable!(),
                        write_done: if is_last {
                            |core, _reg| {
                                let state =
                                    Self::get_state(core).unwrap_stacking_in_progress_state();
                                // Stacking finished
                                debug_assert!(state.stack_frame_iterator.next().is_none());
                                let interrupt_id = state.interrupt_id;
                                Self::update_lr(core);
                                Self::set_state(
                                    core,
                                    InterruptEntryExitState::EntryStacking(
                                        StackingState::Finished(interrupt_id),
                                    ),
                                );
                            }
                        } else {
                            |_core, _reg| {}
                        },
                        reg: RegisterID::LR,
                    },
                    DataBus::Word(data.into()),
                );
            }

            // This code doesn't make sense now or never did?
            // let state = Self::get_state(core).unwrap_stacking_in_progress_state();
            // match (state.address_phase_reg, state.data_phase_reg) {
            //     (Some(_), None | Some(_)) => PipelineAction::None,
            //     // Last data phase - can trigger decode already
            //     (None, Some(_)) => PipelineAction::RunDecode(TriggerData::IgnoreCurrent),
            //     // (None, Some(_)) => PipelineAction::RunFullPipeline,
            //     (None, None) => {
            //         // Stacking finished
            //         debug_assert!(state.stack_frame_iterator.next().is_none());
            //
            //         // TODO: it may be not valid yet, if fetching took more time than stacking
            //         Self::exception_taken(core, ctx, interrupt_id);
            //
            //         PipelineAction::RunFullPipeline
            //     }
            // }
        }
    }

    /// [ARM-ARM] B1.5.6 - `PushStack()` pseudocode. At the end of this function
    /// there is code that updates `LR` register in a specific way.
    /// `update_lr()` corresponds to this code.
    fn update_lr(core: &mut CoreComponent) {
        // [ARM-TDG] 9.6 More on the exception return value.
        // [ARM-ARM] B1.5.6 Exception entry behavior - PushStack() pseudocode.
        let exc_return = if have_fp_ext() {
            unimplemented!("FP not supported by CM");
        } else if RegisterBank::get_xpsr(core).current_mode() == ExecutionMode::Handler {
            ReturnBehavior::TO_HANDLER_MODE
        } else if RegisterBank::get_control(core).stack_pointer_selector() == StackPointer::Process
        {
            ReturnBehavior::TO_THREAD_MODE_WITH_PSP
        } else {
            ReturnBehavior::TO_THREAD_MODE_WITH_MSP
        };
        RegisterBank::set_register(core, RegisterID::LR, exc_return);
    }

    /// [ARM-ARM] B1.5.6 Exception entry behavior - `ReturnAddress()` pseudocode.
    fn return_address(core: &mut CoreComponent, interrupt_id: InterruptId, ctx: &Context) -> Word {
        let state = Self::get_state(core).unwrap_stacking_in_progress_state();
        let next_instr_addr = state.next_instr_addr;
        let this_instr_addr = state.this_instr_addr;

        #[allow(clippy::match_same_arms)]
        let pc = match interrupt_id {
            InterruptId::NMI => next_instr_addr,
            InterruptId::HardFault => {
                if Self::is_exception_synchronous(interrupt_id) {
                    this_instr_addr
                } else {
                    next_instr_addr
                }
            }
            InterruptId::MemManage => this_instr_addr,
            InterruptId::BusFault => {
                if Self::is_exception_synchronous(interrupt_id) {
                    this_instr_addr
                } else {
                    next_instr_addr
                }
            }
            InterruptId::UsageFault => this_instr_addr,
            InterruptId::SVCall => next_instr_addr,
            InterruptId::DebugMonitor => {
                if Self::is_exception_synchronous(interrupt_id) {
                    this_instr_addr
                } else {
                    next_instr_addr
                }
            }
            InterruptId::PendSV => next_instr_addr,
            InterruptId::SysTick => next_instr_addr,
            InterruptId::Interrupt(_) => next_instr_addr,
            InterruptId::Reset => unreachable!(
                "Device reset does not return. This exception should be handled separately."
            ),
        };

        debug!(
            "Exception return address: {:x} @ cycle {:?}",
            pc,
            ctx.cycle_no()
        );
        // [ARM-ARM] B1.5.6 Exception entry behaviour - ReturnAddress() pseudocode.
        // Return address is always halfword aligned - bit<0> is always 0.
        debug_assert!(!pc.get_bit(0));

        pc
    }

    /// [ARM-ARM] B1.5.6 Exception entry behaviour.
    /// [ARM-TRM-G] Table 5-1 Exception types.
    /// [TI-TRM] Table 4-1 Exception types.
    // Note: Do not flatten this function with `return_address`, because it can
    // introduce unnecessary bugs in the future, e.g., see that both
    // `USAGE_FAULT` and `SV_CALL` are synchronous, but their match arms differ
    // in the `return_address`.
    fn is_exception_synchronous(exception: InterruptId) -> bool {
        #[allow(clippy::match_same_arms)]
        match exception {
            InterruptId::Reset => false,
            InterruptId::NMI => false,
            InterruptId::HardFault => true,
            InterruptId::MemManage => true,
            InterruptId::BusFault => {
                // [ARM-TRM-G] Table 5-1 - it's synchronous when precise and asynchronous otherwise.
                // TODO: Check `PRECISERR` and `IMPRECISERR` fields of `BFSR`
                // register to return correct value (described in [ARM-FLT-EXC]).

                // It isn't correct value, but BUS FAULT isn't supported yet.
                true
            }
            InterruptId::UsageFault => true,
            InterruptId::SVCall => true,
            InterruptId::DebugMonitor => true,
            InterruptId::PendSV => false,
            InterruptId::SysTick => false,
            InterruptId::Interrupt(_) => false,
        }
    }
}

// Tail chain.
impl InterruptEntryAndExitHandler {
    /// [ARM-ARM] B1.5.12 - `TailChain()` pseudocode without `ExceptionTaken()`
    /// part. The reason for that is [`Self::exception_taken()`] should be called
    /// if fetching handler has completed.
    fn run_tail_chain(core: &mut CoreComponent, ctx: &mut Context) {
        debug_assert_eq!(
            RegisterBank::get_xpsr(core).current_mode(),
            ExecutionMode::Handler
        );

        let state = Self::get_state(core).unwrap_tail_chain_state();

        let exc_return = state.exc_return;
        let exc_return_27_4 = bitstring_extract!(exc_return<27:4> | 24 bits);
        // Note: this weirdly differs for handling extended frame with FpExt from ExceptionEntry
        #[allow(clippy::manual_assert)] // Not an assertion.
        if !exc_return_27_4.is_ones() {
            panic!("UNPREDICTABLE");
        }

        RegisterBank::set_register(core, RegisterID::LR, exc_return);

        let returning_exception_number = InterruptId::try_from_exception_number(
            RegisterBank::get_xpsr(core).get_exception_number() as usize,
        )
        .unwrap();
        Self::deactivate(core, ctx, returning_exception_number);
    }
}

// Common entry operation.
impl InterruptEntryAndExitHandler {
    fn fetch_handler(core: &mut CoreComponent, interrupt_id: InterruptId) {
        let mut this = Self::get_proxy(core);
        let vector_table_offset = this.vtor_copy.read(Word::from(!0_u32));
        let interrupt_handler_address =
            compute_interrupt_handler_address(interrupt_id, vector_table_offset);
        this.fetching_handler_state
            .set_next(FetchingHandlerState::InProgress);
        let callback: fn(&mut CoreComponent) = |core| {
            let mut this = Self::get_proxy(core);
            this.fetching_handler_state
                .set_next(FetchingHandlerState::Finished);
        };

        Fetch::make_vector_call(core, interrupt_handler_address, Some(callback));
    }

    fn complete_entry(core: &mut CoreComponent, ctx: &mut Context, interrupt_id: InterruptId) {
        let mut this = Self::get_proxy(core);

        debug_assert_eq!(*this.fetching_handler_state, FetchingHandlerState::Finished);

        this.fetching_handler_state
            .set_next(FetchingHandlerState::None);

        Self::exception_taken(core, ctx, interrupt_id);
        Execute::restore_execution(core);
    }

    /// [ARM-ARM] B1.5.6 Exception entry behavior - `ExceptionTaken()` pseudocode
    /// without fetching interrupt handler.
    /// [ARM-TDG] 9.1.3 Register updates.
    /// Must be called if fetching handler has completed.
    // Some registers mentioned in the chapter aren't changed in this function. Those are:
    // * SP - changed in `run_stacking`
    // * PC - changed in `run_interrupt_entry_and_exit` by calling `Fetch::make_vector_call`
    // * LR - changed in either `run_stacking` or `run_tail_chain`.
    fn exception_taken(core: &mut CoreComponent, ctx: &mut Context, interrupt_id: InterruptId) {
        #[allow(clippy::cast_possible_truncation)]
        let interrupt_id = interrupt_id.as_exception_number() as u32;
        // Note: the handler mode is simply when an exception is executing (exception_no!=0)
        // We don't set EPSR.T, since there is no ARM mode
        let xpsr = RegisterBank::get_xpsr(core)
            .with_exception_number(interrupt_id)
            .with_itstate(ItState::new_outside_it_block());
        RegisterBank::set_xpsr(core, xpsr);

        let control =
            RegisterBank::get_control(core).with_stack_pointer_selector(StackPointer::Main);
        RegisterBank::set_control(core, control);

        // TODO: Add implementation of 4 last function calls in
        // [ARM-ARM] ExceptionTaken() pseudocode, i.e.:
        // - disable floating-point mode
        // - ClearExclusiveLocal(),
        // - SetEventRegister(),
        // - InstructionSynchronizationBarrier().

        let this = Self::component_to_member(core);
        match this.interrupt_entry_exit_state {
            InterruptEntryExitState::EntryStacking(..) => core
                .nvic
                .change_core_state(ctx, CoreStateChange::FinishedEntry),
            InterruptEntryExitState::TailChain(..) => core
                .nvic
                .change_core_state(ctx, CoreStateChange::FinishedTailChain),
            _ => panic!(
                "exception_taken() has been called with wrong state: {:?}",
                this.interrupt_entry_exit_state
            ),
        };

        Self::set_state(core, InterruptEntryExitState::None);
    }
}

// Interrupt exit.
impl InterruptEntryAndExitHandler {
    /// [ARM-ARM] B1.5.8 Exception return behavior - Exception return operation.
    /// This function is only executed once, then it is followed with `run_unstacking`
    fn run_exception_return(core: &mut CoreComponent, ctx: &mut Context) {
        let running_mode = RegisterBank::get_xpsr(core).current_mode();
        debug_assert_eq!(running_mode, ExecutionMode::Handler);

        let state = Self::get_state(core).unwrap_exit_state();
        let exc_return = ReturnBehavior::from(state.exc_return);

        // Remaining checks done in the ReturnBehavior
        if have_fp_ext() {
            unimplemented!("FP not supported by CM");
        }

        let returning_exception_number = InterruptId::try_from_exception_number(
            RegisterBank::get_xpsr(core).get_exception_number() as usize,
        )
        .unwrap();

        // TODO: Get number of active exceptions.

        // TODO: Check if current interrupt/exception is active - it cannot be
        // done, because UsageFault isn't supported yet.

        // Similar arms will be changed when nested interrupts are implemented.
        #[allow(clippy::match_same_arms)]
        match exc_return {
            // when '0001'
            ReturnBehavior::ExcReturn {
                return_to: ExecutionMode::Handler,
                return_stack,
                ..
            } => {
                let control =
                    RegisterBank::get_control(core).with_stack_pointer_selector(return_stack);
                RegisterBank::set_control(core, control);
            }
            // when '1x01'
            ReturnBehavior::ExcReturn {
                return_to: ExecutionMode::Thread,
                return_stack,
                ..
            } => {
                // TODO: Check if there are any nested interrupts - not implemented yet.
                let control =
                    RegisterBank::get_control(core).with_stack_pointer_selector(return_stack);
                RegisterBank::set_control(core, control);
            }
            _ => unimplemented!("Illegal EXC_RETURN is not supported yet!"),
        }

        Self::deactivate(core, ctx, returning_exception_number);

        // see PopStack() below
    }

    /// `run_unstacking` executes unstacking. Returned value informs whether unstacking finished.
    /// Maps to first part of `PopStack()` pseudocode
    #[allow(clippy::shadow_unrelated)]
    fn run_unstacking(core: &mut CoreComponent, #[allow(unused)] ctx: &mut Context) -> bool {
        let state = Self::get_state(core);
        trace!("Unstacking in state: {:?}", state);

        #[cfg(feature = "cycle-debug-logger")]
        CycleDebugLoggerProxy::new().on_core_run_unstacking(ctx);

        if let InterruptEntryExitState::ReadyToExit(..) = state {
            // We already have SP fixed back to the original stack
            let sp_address = Address::from(RegisterBank::get_register(core, RegisterID::SP));

            Self::set_state(
                core,
                InterruptEntryExitState::ExitUnstacking(UnstackingState {
                    stack_frame_iterator: stack_frame::new_register_iterator(),
                    sp_address_before_unstacking: sp_address,
                    address_phase_reg: None,
                    data_phase_reg: None,
                    address_to_branch_to_after_unstacking: None,
                }),
            );
        }

        let state = Self::get_state(core).unwrap_unstacking_state();
        if let Some(addr) = state.address_to_branch_to_after_unstacking.take() {
            Self::branch_to(core, addr);
        }

        if LSU::can_request(core) {
            let state = Self::get_state(core).unwrap_unstacking_state();
            state.data_phase_reg = state.address_phase_reg.take();
            state.address_phase_reg = state.stack_frame_iterator.next().copied();
            let is_last = state.stack_frame_iterator.clone().next().is_none();

            // Handle address phase register.
            if let Some(reg) = state.address_phase_reg {
                let offset = stack_frame::register_offset_from_sp(reg);
                let address = state.sp_address_before_unstacking.offset(offset);

                // TODO: consider implementing and using burst transfers
                LSU::request_read(
                    core,
                    address.into(),
                    new_ahb::Size::Word,
                    Self::load_read_data_to_register_callback(reg, is_last),
                );
            }

            // Check if finished.
            let state = Self::get_state(core).unwrap_unstacking_state();
            let unstacking_finished =
                state.data_phase_reg.is_none() && state.address_phase_reg.is_none();
            if unstacking_finished {
                debug_assert!(state.address_phase_reg.is_none());
                debug_assert!(state.stack_frame_iterator.next().is_none());

                Self::set_state(core, InterruptEntryExitState::None);
            }

            unstacking_finished
        } else {
            false
        }
    }

    fn late_finish_unstacking(core: &mut CoreComponent) {
        // Called in tock

        // Check if finished.
        let state = Self::get_state(core).unwrap_unstacking_state();
        debug_assert!(state.address_phase_reg.is_none());
        debug_assert!(state.stack_frame_iterator.next().is_none());

        // TODO: we may do some forwarding here at the end of the cycle.
        // Self::set_state(core, InterruptEntryExitState::None);
    }

    /// Creates callback that loads data received on data bus in the current cycle to the given register.
    fn load_read_data_to_register_callback(
        reg: stack_frame::Register,
        is_last: bool,
    ) -> ReadDataCallback {
        let decode_fn = DataBus::unwrap_word;
        match reg {
            // [ARM-ARM] B1.5.8 Exception return behaviour - PopStack() pseudocode.
            // PC is popped from the stack in different way than the other registers.
            // BranchTo() is used - unpredictable if the new PC is not halfword aligned.
            stack_frame::Register::Reg(RegisterID::PC) => ReadDataCallback::WithDecodeFn(
                |core, decode, data| {
                    let address = decode(data);
                    Self::set_address_to_branch_to(core, address);
                },
                decode_fn,
            ),
            stack_frame::Register::XPSR => ReadDataCallback::WithDecodeFn(
                |core, decode, data| {
                    let xpsr = XPSR::from(decode(data));

                    // Execute writeback - set value of SP to the one that is expected after unstacking.
                    // It has to be done after reading value of XPSR from stack to know if SP alignment was required during stacking.
                    // With this information new value for SP can be computed.
                    Self::do_sp_writeback(core, xpsr);

                    Self::update_current_xpsr(core, xpsr);
                },
                decode_fn,
            ),
            stack_frame::Register::Reg(reg) => ReadDataCallback::WithRegisterAndDecodeFn(
                if is_last {
                    |core, #[cfg(feature = "cycle-debug-logger")] _ctx, reg, decode, data| {
                        RegisterBank::set_register(core, reg, decode(data));
                        Self::late_finish_unstacking(core);
                    }
                } else {
                    |core, #[cfg(feature = "cycle-debug-logger")] _ctx, reg, decode, data| {
                        RegisterBank::set_register(core, reg, decode(data));
                    }
                },
                reg,
                decode_fn,
            ),
        }
    }

    fn set_address_to_branch_to(core: &mut CoreComponent, address: Word) {
        let state = Self::get_state(core).unwrap_unstacking_state();
        state.address_to_branch_to_after_unstacking = Some(address);
    }

    /// [ARM-ARM] B1.4.7 Pseudocode details of ARM core register accesses.
    fn branch_to(core: &mut CoreComponent, address: Word) {
        debug_assert!(
            !address.get_bit(0),
            "Interworking in exception return PC is UNPREDICTABLE"
        );
        Fetch::make_branch(core, address);
    }

    fn do_sp_writeback(core: &mut CoreComponent, xpsr: XPSR) {
        let state = Self::get_state(core).unwrap_unstacking_state();

        #[allow(clippy::cast_possible_truncation)]
        let mut address = state
            .sp_address_before_unstacking
            .offset(4 * stack_frame::SIZE as u32);

        // [ARM-ARM] B1.5.8: Exception return operation - PopStack() pseudocode.
        // TODO: After implementing CCR.STKALIGN register in NVIC, use to check if align was required in the same way as in the pseudocode.
        if xpsr.sp_align_was_required() {
            address = address.offset(4);
        }

        RegisterBank::set_register(core, RegisterID::SP, address.into());
    }

    /// [ARM-ARM] B1.5.8: Exception return operation - `PopStack()` pseudocode with `xPSR` update.
    #[allow(clippy::similar_names)]
    fn update_current_xpsr(core: &mut CoreComponent, xpsr_from_stack: XPSR) {
        let mut xpsr = Word::from(RegisterBank::get_xpsr(core));
        let apsr_from_stack = xpsr_from_stack.apsr_as_word();
        bitstring_substitute!(xpsr<31:27> = bitstring_extract!(apsr_from_stack<31:27> | 5 bits));

        if have_dsp_ext() {
            bitstring_substitute!(xpsr<19:16> = bitstring_extract!(apsr_from_stack<19:16> | 4 bits));
        }

        let ipsr_from_stack = xpsr_from_stack.ipsr_as_word();
        bitstring_substitute!(xpsr<8:0> = bitstring_extract!(ipsr_from_stack<8:0> | 9 bits));

        let epsr_from_stack = xpsr_from_stack.epsr_as_word();
        bitstring_substitute!(xpsr<26:24> = bitstring_extract!(epsr_from_stack<26:24> | 3 bits));
        bitstring_substitute!(xpsr<15:10> = bitstring_extract!(epsr_from_stack<15:10> | 6 bits));

        RegisterBank::set_xpsr(core, XPSR::from(xpsr));
    }

    /// [ARM-ARM] B1.5.8 - `DeActivate()` pseudocode.
    fn deactivate(
        core: &mut CoreComponent,
        ctx: &mut Context,
        returning_exception_number: InterruptId,
    ) {
        let this = Self::component_to_member(core);
        match this.interrupt_entry_exit_state {
            InterruptEntryExitState::TailChain(..) => core
                .nvic
                .change_core_state(ctx, CoreStateChange::StartedTailChain),
            InterruptEntryExitState::ReadyToExit(..) => core
                .nvic
                .change_core_state(ctx, CoreStateChange::StartedExit),
            _ => panic!(
                "Deactivate has been called in wrong state: {:?}",
                this.interrupt_entry_exit_state
            ),
        };

        if returning_exception_number != InterruptId::NMI {
            let faultmask = RegisterBank::get_faultmask(core).with_faultmask(false);
            RegisterBank::set_faultmask(core, ctx, faultmask);
        }
    }
}

impl InterruptEntryAndExitHandler {
    fn get_state(core: &mut CoreComponent) -> &mut InterruptEntryExitState {
        let this = Self::component_to_member_mut(core);
        &mut this.interrupt_entry_exit_state
    }

    fn set_state(core: &mut CoreComponent, state: InterruptEntryExitState) {
        let mut this = Self::get_proxy(core);
        this.interrupt_entry_exit_state = state;
    }
}

impl InterruptEntryExitState {
    #[track_caller]
    fn unwrap_stacking_state(&mut self) -> &mut StackingState {
        if let InterruptEntryExitState::EntryStacking(state) = self {
            state
        } else {
            panic!("Expected `InterruptEntryExitState::EntryStacking(_)`, but got: {self:?}")
        }
    }

    #[track_caller]
    fn unwrap_stacking_in_progress_state(&mut self) -> &mut StackingInProgressState {
        if let InterruptEntryExitState::EntryStacking(StackingState::InProgress(state)) = self {
            state
        } else {
            panic!(
                "Expected `InterruptEntryExitState::EntryStacking(StackingState::InProgress(_))`, but got: {self:?}"
            )
        }
    }

    #[track_caller]
    fn unwrap_tail_chain_state(&mut self) -> &mut TailChainState {
        if let InterruptEntryExitState::TailChain(state) = self {
            state
        } else {
            panic!("Expected `InterruptEntryExitState::TailChain(_)`, but got: {self:?}")
        }
    }

    #[track_caller]
    fn unwrap_exit_state(&mut self) -> &mut ExitState {
        if let InterruptEntryExitState::ReadyToExit(state) = self {
            state
        } else {
            panic!("Expected `InterruptEntryExitState::ReadyToExit(_)`, but got: {self:?}")
        }
    }

    #[track_caller]
    fn unwrap_unstacking_state(&mut self) -> &mut UnstackingState {
        if let InterruptEntryExitState::ExitUnstacking(state) = self {
            state
        } else {
            panic!("Expected `InterruptEntryExitState::ExitUnstacking(_)`, but got: {self:?}")
        }
    }
}

mod stack_frame {
    use super::RegisterID;

    /// [ARM-ARM] B1.5.6 Exception entry behaviour.
    /// During an exception entry, the core saves a context by pushing 8 registers
    /// (stack frame) onto the stack. These registers are: R0-R3, R12, LR, PC and `xPSR`.
    pub(super) const SIZE: usize = 8;

    /// [ARM-TDG] Figure 9.2 - order of registers in stacking sequence.
    /// [ARM-TDG] 9.2 - order of registers during unstacking sequence is the same as during stacking.
    const REGISTER_ORDER: [Register; SIZE] = [
        Register::Reg(RegisterID::PC),
        Register::XPSR,
        Register::Reg(RegisterID::R0),
        Register::Reg(RegisterID::R1),
        Register::Reg(RegisterID::R2),
        Register::Reg(RegisterID::R3),
        Register::Reg(RegisterID::R12),
        Register::Reg(RegisterID::LR),
    ];

    /// `Register` enum represents register that can be pushed/popped to the stack.
    /// [ARM-ARM] B1.5.6 Exception entry behaviour, these registers are: R0-R3, R12, LR, PC and `xPSR`.
    #[derive(Copy, Clone, Debug)]
    pub(super) enum Register {
        XPSR,
        Reg(RegisterID),
    }

    pub(super) type Iter = std::slice::Iter<'static, Register>;

    pub(super) fn new_register_iterator() -> Iter {
        REGISTER_ORDER.iter()
    }

    /// Returns offset from SP address after stacking.
    /// [ARM-TDG] Figure 9.2 - contains offsets for each stacked register.
    pub(super) fn register_offset_from_sp(reg: Register) -> u32 {
        match reg {
            Register::XPSR => 28,
            Register::Reg(RegisterID::R0) => 0,
            Register::Reg(RegisterID::R1) => 4,
            Register::Reg(RegisterID::R2) => 8,
            Register::Reg(RegisterID::R3) => 12,
            Register::Reg(RegisterID::R12) => 16,
            Register::Reg(RegisterID::LR) => 20,
            Register::Reg(RegisterID::PC) => 24,
            Register::Reg(..) => panic!("Got unexpected stack_frame::Register: {reg:?}"),
        }
    }
}

/// [ARM-ARM] B1.5.6 Exception entry behavior - `ExceptionTaken()` pseudocode.
fn compute_interrupt_handler_address(
    interrupt_id: InterruptId,
    vector_table_offset: Word,
) -> Address {
    let vector_table_offset_31_7 = bitstring_extract!(vector_table_offset<31:7> | 25 bits);
    let vector_table_address =
        bitstring_concat!(vector_table_offset_31_7: bsc::C_000_0000 | 32 bits);
    #[allow(clippy::cast_possible_truncation)]
    Address::from(vector_table_address).offset(4 * interrupt_id.as_exception_number() as u32)
}

/// [ARM-ARM] B1.5.8 Exception return behavior.
/// An exception return occurs when the processor is in Handler mode and a value of 0xFXXXXXXX is loaded into the PC.
pub(super) fn check_if_branch_to_given_address_returns_from_exception(
    core: &CoreComponent,
    addr: Word,
) -> bool {
    // TODO: does copy-pasting the simplified code results in a more performant code?
    // let addr_31_28 = bitstring_extract!(addr<31:28> | 4 bits);
    // RegisterBank::get_xpsr(core).current_mode() == ExecutionMode::Handler
    //     && addr_31_28 == bsc::C_1111
    !matches!(
        ReturnBehavior::from_addr_and_state(core, addr),
        ReturnBehavior::NormalBranch
    )
}
