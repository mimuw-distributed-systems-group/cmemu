use super::fetch::FDReg;
use super::instruction::Instruction;
use super::register_bank::{RegisterBitmap, RegisterID, XPSR};
use super::{CoreComponent, Fetch, RegisterBank, interrupt};
use crate::common::{Address, BitstringUtils, SRType, Word};
use crate::component::core::execute::{Execute, PipeliningResult};
use crate::confeature::{cm_hyp, println_traces};
use crate::engine::{
    Context, DisableableComponent, Subcomponent, TickComponent, TickComponentExtra,
};
#[cfg(feature = "cycle-debug-logger")]
use crate::proxy::CycleDebugLoggerProxy;
use crate::utils::{IfExpr, dife};
use log::trace;
use owo_colors::OwoColorize;
use std::fmt::{Display, Formatter};

mod instruction;

#[derive(Subcomponent, TickComponent, TickComponentExtra, DisableableComponent)]
#[subcomponent_1to1]
pub(super) struct Decode {
    state: DecodeState,

    /// This value determines if previously decoded instruction writes to LR.
    /// It is used to determine if given branch is executed during decode or execute phase.
    /// The full list of instructions and phases can be found in:
    /// [ARM-TRM-G] 15.3 Table 15-3 Branches and stages evaluated by the processor
    ///
    /// NOTE: This approach is different than checking if LR is `dirty`. Some tests (`control_flow`/*_lr.asm)
    /// pass for current implementation and not for the other one.
    /// It might be the case the silicon design was simpler with this approach.
    ///
    /// If simply the same concept of dirty registers were applied, as modelled for AGU,
    /// then errata 377494 (Errata-r1p0-2008) would not be there.
    prev_instruction_writes_lr: bool,

    /// It seems that normal "mul(s)" (actual settings of the flags doesn't matter) can stall
    /// subsequent conditional branches. It is reasonable, as mul has a very long critical path,
    /// however it is unclear why it affects conditional branches.
    /// TODO: this is hacky right now, make the implementation nicer
    muls_bcond_stall: bool,
}

/// Represents decode as state machine.
#[derive(Clone, Debug)]
enum DecodeState {
    /// Decode is waiting for execute to be ready and for fetch to have data.
    WaitingForFetchAndExecute,
    /// Decode of instruction has run, but AGU is waiting for data to be fast-forwarded.
    AGUWaitingForData {
        /// Contains address of decoded instruction.
        addr: Address,
        /// Contains decoded instruction.
        instr: Instruction,
        /// Contains *candidate* for folded decoded instruction if there's such.
        folded_instr: Option<Instruction>,
        /// Length of main and folded instructions in halfwords.
        /// Possible values are 1 and 2.
        length: u8,
        /// Contains information about registers that cannot be read during AGU phase.
        /// This field is updated each cycle with arrival of trigger data.
        dirty_registers: RegisterBitmap,
        /// Marks if decoded instruction had `advance_head` call postponed to `move_pipeline`.
        postponed_advance_head: bool,
        xpsr: XPSR,
    },
    /// Result is computed, notification of fetch is required.
    NotifyFetchAboutBranch {
        /// Contains address of decoded instruction.
        addr: Address,
        /// Contains decoded instruction.
        instr: Instruction,
        /// Contains *candidate* for folded decoded instruction if there's such.
        folded_instr: Option<Instruction>,
        /// Length of main and folded instructions in halfwords.
        /// Possible values are 1 and 2.
        length: u8,
        // TODO: only in debug_assertions
        dirty_registers: RegisterBitmap,
        /// Marks if decoded instruction had `advance_head` call postponed to `move_pipeline`.
        postponed_advance_head: bool,
        /// Marks whether LR was deemed dirty for BX/MOV PC LR
        lr_is_dirty: bool,
        xpsr: XPSR,
    },
    /// Result is waiting for move of pipeline.
    ResultReady {
        /// Contains value that is consumed by execute subcomponent.
        value: PipelineStepPack,
        /// Marks if decoded instruction had `advance_head` call postponed to `move_pipeline`.
        postponed_advance_head: bool,

        xpsr: XPSR,
    },
}

/// Represents all information produced in decode phase
/// that is needed in execute phase.
#[derive(Clone, Debug)]
pub(super) struct PipelineStepPack {
    /// The main instruction
    pub(super) instruction: Instruction,

    /// *Candidate* for instruction folded with the main instruction (if such exists)
    pub(super) folded_instruction: Option<Instruction>,

    /// Address of the main instruction
    pub(super) address: Address,

    pub(super) branch_kind: Brchstat,

    /// Notifies if decode time conditional branch was made and result was mispredicted.
    ///
    /// With the help of fast forwarded xPSR^1, it's determined in decode phase if a speculative branch is taken or not.
    /// However, it is the subject of speculative branch i.e. telling fetch to request data from branch target address and
    /// not discard current cache, and during execute phase, once we know if branch is taken, discarding the unwanted fetch data.
    /// See: [ARM-TRM-G] `1.4 Prefetch Unit` and `1.5 Branch target forwarding`
    /// ^1: This is possible, because in Cortex-M3 only single-cycle instructions may update flags.
    ///     This assumption should be verified when implementing Cortex-M4 or M0 (e.g. CC2652 platform).
    pub(super) branch_was_mispredicted: bool,

    /// Length of main and folded instructions in halfwords.
    /// Possible values are 1 and 2.
    pub(super) length: u8,
}

impl PipelineStepPack {
    pub(super) fn following_instruction_address(&self) -> Address {
        self.address.offset((self.length * 2).into())
    }

    pub(super) fn folded_instruction_address(&self) -> Option<Address> {
        self.folded_instruction
            .is_some()
            .then(|| self.address.offset(2))
    }
}

/// See [ARM-TRM-G] 14.3 Branch status interface
/// Execute time branches may keep this while the previous op is stalled
/// Valid when branch op is in decode.
///
/// Note that some upcoming execute-time branches may supress prefetching next instructions.
/// See: [ARM-TRM-G] 15.3 Note below Table 15-3.
/// But it turns out, ADD PC also suppresses fetching -- what makes sense for an unconditional branch.
/// Yet, MOV PC, R4 is unconditional and apparently doesn't suppress that fetching.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Brchstat {
    NoBranch = 0b0000,
    DecodeTimeConditionalBackwards = 0b0001,
    DecodeTimeConditional = 0b0010,
    ExecuteTimeConditional = 0b0011,
    DecodeTimeUnconditional = 0b0100,
    ExecuteTimeUnconditional = 0b0101,
    // Only a cycle after DecodeTimeConditional*
    ConditionalTaken = 0b1000,
}

impl Brchstat {
    pub fn is_speculative(self) -> bool {
        // [ARM-TRM-G] 1.5 Branch target mentions, that controller outside core (e.g., Bus Matrix)
        // may allow speculation only on backward branches
        // [ARM-TRM-G] 15.3: "Speculative fetches might be cancelled during wait states."
        // -> but this is prevented by AHB_COMPAT bit.
        matches!(
            self,
            Self::DecodeTimeConditional | Self::DecodeTimeConditionalBackwards
        )
    }

    pub fn is_decode_time(self) -> bool {
        matches!(
            self,
            Self::DecodeTimeConditional
                | Self::DecodeTimeConditionalBackwards
                | Self::DecodeTimeUnconditional
                | Self::ConditionalTaken
        )
    }
    pub fn is_execute_time(self) -> bool {
        matches!(
            self,
            Self::ExecuteTimeConditional | Self::ExecuteTimeUnconditional
        )
    }

    pub fn is_conditional(self) -> bool {
        matches!(
            self,
            Self::DecodeTimeConditional
                | Self::DecodeTimeConditionalBackwards
                | Self::ExecuteTimeConditional
        )
    }

    #[allow(dead_code)]
    pub fn changes_pc(self) -> bool {
        // TODO: do we need to distinguish cancelled speculative branches from non-branching instructions?
        !matches!(self, Self::NoBranch)
    }

    /// Evaluate condition while still in Decode. CBZ is out of scope.
    pub(super) fn evaluate_condition(self, instr: &Instruction, xpsr: XPSR) -> bool {
        debug_assert!(instr.is_branch());
        debug_assert!(!xpsr.in_it_block() || xpsr.last_in_it_block());
        let condition_failed = (xpsr.in_it_block() && !xpsr.it_condition_passed())
            || matches!(instr, Instruction::Branch {cond, ..} if !cond.passed(xpsr));
        !condition_failed
    }
    pub(super) fn speculative_with_condition(self, instr: &Instruction, xpsr: XPSR) -> Self {
        if self.is_speculative() {
            self.evaluate_condition(instr, xpsr)
                .ife(Self::ConditionalTaken, Self::NoBranch)
        } else {
            self
        }
    }

    // TODO: write a test for that, not sure if we can believe instr methods
    #[allow(clippy::match_same_arms)]
    pub(super) fn from_instruction(
        instr: &Instruction,
        lr_is_dirty: bool,
        in_it_block: bool,
    ) -> Self {
        // Some of them are not correctly recognized as branches
        // FIXME: can we just make it to pass .is_branch() test? Something else breaks then.
        if !instr.is_branch()
            && !matches!(instr,
                // | Self::ClearExclusive
                | Instruction::InstructionSynchronizationBarrier { .. }
                // | Self::DataSynchronizationBarrier { .. }
                // | Self::DataMemoryBarrier { .. }
            )
        {
            return Self::NoBranch;
        }
        // [ARM-TRM-G] 15.3 Paragraph above Table 15-3
        // "In IT block, ALU register based branches and LSU PC modifying instructions are recognized
        //  as 0b0011 conditional branches." [ExecuteTimeConditional]
        // TODO: Check if decode-time BLX LR etc. is conditional in IT block!
        let conditional = (in_it_block && !instr.is_unconditional_in_it_block())
            || matches!(instr, Instruction::Branch {cond, ..} if !cond.is_al())
            || matches!(instr, Instruction::CompareAndBranch { .. });
        // Table 15-3
        let execute_time = match instr {
            Instruction::Branch { .. } => false,
            // TODO: Does this interact with further LR code? is the "adjusted" lr_is_dirty passed here?
            Instruction::BranchWithLinkAndExchange_Register { rm: RegisterID::LR }
            | Instruction::BranchAndExchange { rm: RegisterID::LR }
            | Instruction::Move_Register {
                rd: RegisterID::PC,
                rm: RegisterID::LR,
                ..
            } => lr_is_dirty,
            // NOTE: According to [ARM-TRM-G] 15.3, Table 15-3 `BL` should be decode-time
            // iff LR is not being written during decode.
            // But experiments with writing to `LR` in instruction just before `BL`
            // are contradicting this. Because of that, we assume that `BL` is always decode-time.
            // Also notice that in the table there is no execute-time record for `BL` instruction (unlike for `BLX` or `BX`).
            Instruction::BranchWithLink_Immediate { .. } => false,
            // ALU & LSU with PC as dest
            _ => true,
        };
        let is_backwards = matches!(instr, Instruction::Branch{imm32, ..} if imm32.get_bit(31));
        match (conditional, execute_time, is_backwards) {
            (true, false, true) => Self::DecodeTimeConditionalBackwards,
            (true, false, false) => Self::DecodeTimeConditional,
            (true, true, _) => Self::ExecuteTimeConditional,
            (false, false, _) => Self::DecodeTimeUnconditional,
            (false, true, _) => Self::ExecuteTimeUnconditional,
        }
    }
}

/// Lists types of additional decode actions.
#[derive(Clone, Copy, Debug)]
enum AdditionalAction {
    /// Computation of address required (e.g. ldr, str instructions).
    ComputeAGUValue,
    /// Computation of target address and informing fetch component.
    /// For details check: [ARM-TRM-G] 15.3 Branch status interface
    Branch,
    /// No special actions is required.
    NoAction,
}

/// Information from execute component needed to decode next instruction.
#[derive(Clone, Copy, Debug)]
pub(super) enum TriggerData {
    /// Marks if decode should reset it's state, because of pipeline refill.
    IgnoreCurrent,
    /// Normal execution - try to decode an instruction
    DecodeCurrent {
        /// Contains registers that are `dirty` in this cycle.
        /// Meaning that execute has written to them new values and we cannot use them.
        dirty_registers: RegisterBitmap,
        /// Fast forwarded xPSR.
        ///
        /// NOTE: Flags are valid very late in the cycle -- we should not change behavior based on this.
        xpsr: XPSR,
        postponed_advance_head: bool,
    },
}

impl Decode {
    pub(super) fn new() -> Self {
        Self {
            state: DecodeState::WaitingForFetchAndExecute,
            prev_instruction_writes_lr: false,
            muls_bcond_stall: false,
        }
    }

    pub(super) fn is_ready(core: &CoreComponent) -> bool {
        let this = Self::component_to_member(core);
        matches!(this.state, DecodeState::ResultReady { .. })
    }

    pub(super) fn is_agu_stalled(core: &CoreComponent) -> bool {
        let this = Self::component_to_member(core);
        matches!(this.state, DecodeState::AGUWaitingForData { .. })
    }

    pub(super) fn is_instr_tainted(core: &CoreComponent) -> bool {
        // A tainted instruction (with IT curse) happens when some logic behaves
        // *as if* it had a folded IT with the original rules.
        // For instance, such an instruction cannot pipeline after another.

        // During experimentation, we found that:
        // a) It doesn't matter if the IT is actually folded.
        // b) It doesn't matter if the IT is actually present in the instruction stream
        //    (i.e., a second half word may just **look** like an IT).
        // c) The bits don't even have to be there: this was the IT Curse.
        //    Finally, the Curse turned out to be reducible to b)
        //    when we consider a "use after free" -- that is seeing
        //    "IT bits" that were there previously.
        //    The original Curse was researched to
        //       need 2 words space, an unaligned skipped instr, which is last in PIQ
        //    like in the example below:
        //
        //         itete.n  ge
        //         ldrge.n  r2, [r0, #-29]
        //         rorlt.n  r1, r3, r1
        //         strge.w  r3, [r13, #-41]
        //         noplt.n                  @ <- Cursed if last!
        let head = Fetch::peek_shadow_head(core);
        looks_like_it(*head.get(1).unwrap()) && !is_long_instruction(*head.get(0).unwrap())
    }

    pub(super) fn get_decoded_size(core: &mut CoreComponent) -> (u8, bool) {
        let this = Self::component_to_member(core);
        match &this.state {
            DecodeState::WaitingForFetchAndExecute => (0, false),
            DecodeState::AGUWaitingForData {
                length,
                folded_instr,
                ..
            }
            | DecodeState::NotifyFetchAboutBranch {
                length,
                folded_instr,
                ..
            }
            | DecodeState::ResultReady {
                value:
                    PipelineStepPack {
                        length,
                        folded_instruction: folded_instr,
                        ..
                    },
                ..
            } => (*length, folded_instr.is_some()),
        }
    }

    /// Returns: `(instruction: &Instruction, instruction_it_skipped: bool, has_folded_instruction: bool)`
    pub(super) fn peek_instruction<'component>(
        core: &'component CoreComponent,
        #[allow(unused)] ctx: &mut Context,
    ) -> (&'component Instruction, bool, bool) {
        let it_second_narrow = Self::is_instr_tainted(core);
        let this = Self::component_to_member(core);
        if let DecodeState::ResultReady { value, xpsr, .. } = &this.state {
            if it_second_narrow {
                #[cfg(feature = "cycle-debug-logger")]
                CycleDebugLoggerProxy.on_free_static_str(
                    ctx,
                    "foldable_it_peek",
                    match (value.folded_instruction.is_some(), value.length == 1) {
                        (true, true) => "WTF",
                        (true, false) => "folded",
                        (false, true) => "lurks",
                        (false, false) => "phantom",
                    },
                );
            }
            // TODO: call instr_unconditional_in_it here?
            (
                &value.instruction,
                xpsr.in_it_block() && !xpsr.it_condition_passed(),
                // value.folded_instruction.is_some(),
                it_second_narrow,
            )
        } else {
            panic!("peek_instruction should be called only when result is ready!")
        }
    }

    /// Moves pipeline by one pipeline step
    #[allow(clippy::shadow_unrelated)]
    pub(super) fn move_pipeline(
        core: &mut CoreComponent,
        #[allow(unused)] ctx: &mut Context,
    ) -> PipelineStepPack {
        // Updating spec branches is moved to here
        Self::handle_stalled_speculative_branches(core, None);

        let this = Self::component_to_member(core);
        if let DecodeState::ResultReady {
            ref value,
            postponed_advance_head,
            xpsr,
        } = this.state
        {
            let value = value.clone();

            let instr = &value.instruction;
            let skipped = xpsr.in_it_block()
                && !xpsr.it_condition_passed()
                && !instr.is_unconditional_in_it_block();

            // Turns out pipelining with "folding" prevention works only on the bits!
            let head = Fetch::peek_shadow_head(core);
            let it_second_narrow = looks_like_it(*head.get(1).unwrap());

            if it_second_narrow {
                #[cfg(feature = "cycle-debug-logger")]
                CycleDebugLoggerProxy.on_free_static_str(
                    ctx,
                    "foldable_it",
                    match (value.folded_instruction.is_some(), value.length == 1) {
                        (true, true) => "WTF",
                        (true, false) => "folded",
                        (false, true) => "lurks",
                        (false, false) => "phantom",
                    },
                );
            }

            let can_pipeline = Execute::can_nop_think_it_is_multicycle(
                core,
                instr,
                skipped,
                // value.folded_instruction.is_some(),
                it_second_narrow,
                ctx,
            );
            #[cfg(feature = "cycle-debug-logger")]
            if let Some(reason) = can_pipeline {
                CycleDebugLoggerProxy.on_free_static_str(ctx, "can_pipeline_decode", reason.into());
            }
            let phony_multicycle_nop = can_pipeline == Some(PipeliningResult::Pipelines) /* && ??  !instr.is_lsu_instruction()*/;

            let this = Self::component_to_member_mut(core);
            this.state = DecodeState::WaitingForFetchAndExecute;
            let clear_lr_flag = match *cm_hyp::bx_lr::FLAG_CLEARING_MODE {
                cm_hyp::bx_lr::FlagClearingModeValues::Always => true,
                // Idea for BX LR errata: the dirty flag clearing is gated with the same wire as that NOP
                cm_hyp::bx_lr::FlagClearingModeValues::MulticycleNop => !phony_multicycle_nop,
            };
            if clear_lr_flag {
                this.prev_instruction_writes_lr = false;
                this.muls_bcond_stall = false;
            }
            if (!skipped || *cm_hyp::bx_lr::SKIPPED_SET_FLAG)
                && (!instr.is_branch_with_link_instruction() || *cm_hyp::bx_lr::BL_SET_FLAG)
            {
                // Check "Note(prev_instruction_writes_lr)" about setting this flag by bl(x)
                this.prev_instruction_writes_lr |=
                    instr.get_written_registers().get(RegisterID::LR);
            }
            if matches!(
                instr,
                Instruction::Multiply {
                    setflags_depends_on_it: true,
                    ..
                }
            ) {
                this.muls_bcond_stall = true;
                #[cfg(feature = "cycle-debug-logger")]
                CycleDebugLoggerProxy.on_free_static_str(
                    ctx,
                    "mult_bcond_stall",
                    this.muls_bcond_stall.ife("true", "false"),
                );
            }

            Fetch::pipeline_moved(
                core,
                postponed_advance_head,
                phony_multicycle_nop,
                &value,
                xpsr,
            );

            value
        } else {
            panic!("move_pipeline should be called only when decode is ready!")
        }
    }

    pub(super) fn run_decode(
        core: &mut CoreComponent,
        #[cfg(feature = "cycle-debug-logger")] ctx: &mut Context,
        trigger_data: TriggerData,
    ) {
        trace!("Decode {}", trigger_data);
        match trigger_data {
            TriggerData::IgnoreCurrent => {
                let mut this = Self::get_proxy(core);
                // TODO: write a test that would hit this assert! (it is hit by a random radio code)
                // debug_assert!(
                //     // Special case of stacking is properly handled already.
                //     Execute::is_stacking_or_unstacking_running(this.component_mut())
                //         || !matches!(this.state, DecodeState::ResultReady {value: PipelineStepPack {branch_kind, ..}, postponed_advance_head, ..} if branch_kind.is_speculative() || postponed_advance_head),
                //     "TODO: check the behavior here"
                // );
                this.state = DecodeState::WaitingForFetchAndExecute;
                // Note(prev_instruction_writes_lr): intuitively, we would clean the  flag here,
                // but see definitive_branch_deps.asm (or large_tests_extract/branches_regs.asm).
                // It shows that, `bx lr` is execute-time after `ldr pc`. On the other hand,
                // `bl*` does not cause such behavior, so it should not set the flag in the first place.
                // An interesting case is "ldr pc, [lr], #off; target: mov pc, lr",
                // which sets the dirty flag in writeback.
                this.muls_bcond_stall = false;
                assert!(!Self::is_ready(this.component_mut()));
            }
            TriggerData::DecodeCurrent {
                dirty_registers,
                xpsr,
                postponed_advance_head,
            } => {
                if Self::is_agu_stalled(core) && xpsr.in_it_block() && !xpsr.it_condition_passed() {
                    // AGU seems to be only stalled during the first cycle of a skipped instruction
                    // Apparently after that it is known that the value won't be necessary,
                    // but the instr is taking an Ex slot anyway.
                    let this = Self::component_to_member_mut(core);
                    let DecodeState::AGUWaitingForData {
                        addr,
                        instr,
                        folded_instr,
                        length,
                        postponed_advance_head,
                        xpsr,
                        ..
                    } = this.state.clone()
                    else {
                        unreachable!()
                    };
                    this.state = DecodeState::ResultReady {
                        value: PipelineStepPack {
                            address: addr,
                            instruction: instr,
                            length,
                            folded_instruction: folded_instr,
                            branch_was_mispredicted: false,
                            branch_kind: Brchstat::NoBranch,
                        },
                        postponed_advance_head,
                        xpsr,
                    };

                    trace!("Decode  [{:?}]: AGU phase finished", addr);
                    #[cfg(feature = "cycle-debug-logger")]
                    CycleDebugLoggerProxy::new().on_decode_agu(ctx, addr);
                }
                if Self::is_ready(core) {
                    Self::handle_stalled_speculative_branches(core, Some(xpsr));

                    let this = Self::component_to_member(core);
                    if !this.state.has_folded_instruction() {
                        Self::run_decode_phase_for_folded_instruction(
                            core,
                            #[cfg(feature = "cycle-debug-logger")]
                            ctx,
                            xpsr,
                        );
                    }
                    return;
                }

                Self::run_decode_phase(
                    core,
                    #[cfg(feature = "cycle-debug-logger")]
                    ctx,
                    dirty_registers,
                    xpsr,
                    postponed_advance_head,
                );
                Self::run_agu_phase(
                    core,
                    #[cfg(feature = "cycle-debug-logger")]
                    ctx,
                    Some(dirty_registers),
                    None,
                );
                Self::run_branch_phase(core, xpsr);
            }
        }
    }

    pub(super) fn fast_forward_agu_register(
        core: &mut CoreComponent,
        #[cfg(feature = "cycle-debug-logger")] ctx: &mut Context,
        register: RegisterID,
        mut value: Word,
    ) {
        // TODO: Incorrectly handling masking of register write (logic from set_register)
        //       On the other hand, forwarding may have weird bugs bcoz of being another logic
        // [ARM-ARM] B1.4.7 Pseudocode details of ARM core register accesses
        // The ARMv7-M architecture guarantees that stack pointer values are at least 4-byte aligned:
        // when 13 _R[LookUpSP()] = value<31:2>:'00';
        if register == RegisterID::SP {
            value = value.align(4);
        }
        Self::run_agu_phase(
            core,
            #[cfg(feature = "cycle-debug-logger")]
            ctx,
            None,
            Some((register, value)),
        );
    }

    fn run_decode_phase(
        core: &mut CoreComponent,
        #[cfg(feature = "cycle-debug-logger")] ctx: &mut Context,
        dirty_registers: RegisterBitmap,
        xpsr: XPSR,
        postponed_advance_head: bool,
    ) {
        #[cfg(feature = "cycle-debug-logger")]
        Fetch::log_cache_status(core, ctx, "fetch_6_decode");

        Self::run_decode_phase_for_main_instruction(
            core,
            #[cfg(feature = "cycle-debug-logger")]
            ctx,
            dirty_registers,
            xpsr,
            postponed_advance_head,
        );
        Self::run_decode_phase_for_folded_instruction(
            core,
            #[cfg(feature = "cycle-debug-logger")]
            ctx,
            xpsr,
        );
    }

    #[allow(clippy::shadow_unrelated)]
    fn run_decode_phase_for_main_instruction(
        core: &mut CoreComponent,
        #[cfg(feature = "cycle-debug-logger")] ctx: &mut Context,
        dirty_registers: RegisterBitmap,
        xpsr: XPSR,
        postponed_advance_head: bool,
    ) {
        let this = Self::component_to_member_mut(core);
        if let DecodeState::WaitingForFetchAndExecute = this.state {
            let head = Fetch::peek_head(core);
            if head.is_empty() {
                return;
            }

            let head_is_long = is_long_instruction(*head.front().unwrap());
            if head_is_long && head.len() < 2 {
                return;
            }

            let instr_address = Fetch::head_address(core);
            let (decoded_instr, instr_len) = if head_is_long {
                let fst_hw = u32::from(*head.get(0).unwrap());
                let snd_hw = u32::from(*head.get(1).unwrap());
                let instr_value = (fst_hw << 16) | snd_hw;

                trace!(
                    "Decode  [{:?}]: {:032b}",
                    instr_address.yellow(),
                    instr_value
                );
                (instruction::decode_long_instruction(instr_value, xpsr), 2)
            } else {
                let instr_value = *head.get(0).unwrap();
                trace!(
                    "Decode  [{:?}]: {:016b}",
                    instr_address.yellow(),
                    instr_value
                );
                (instruction::decode_short_instruction(instr_value, xpsr), 1)
            };
            trace!(
                "Decode  [{:?}]: is {} len: {}",
                instr_address.yellow(),
                decoded_instr.yellow(),
                instr_len
            );
            println_trace_decoded(&decoded_instr, instr_len);

            #[cfg(feature = "cycle-debug-logger")]
            CycleDebugLoggerProxy::new().on_decode(
                ctx,
                instr_address,
                decoded_instr.clone(),
                instr_len,
            );

            let mut this = Self::get_proxy(core);
            let lr_considered_dirty = this.prev_instruction_writes_lr;
            let brchstat =
                Brchstat::from_instruction(&decoded_instr, lr_considered_dirty, xpsr.in_it_block());
            let skipped = xpsr.in_it_block() && !xpsr.it_condition_passed();

            let effective_instruction_has_writeback =
                Execute::get_last_effective_instruction(this.component()).is_some_and(|instr| {
                    instr.is_lsu_instruction() && Execute::get_memorier_description(instr).writeback
                });
            let tainted_dependency =
                effective_instruction_has_writeback && Decode::is_instr_tainted(this.component());
            this.state = match Self::get_additional_action(&decoded_instr) {
                // Skipped instructions seem
                // to have a stalled AGU cycle only sometimes after a dependency on address.
                // Let's try aborting AGU after the first cycle of waiting. For example,
                //    itt true; adds.w (flip) Rx, ...; ldr ..., [Rx, ...]
                // has a stalled AGU latency.
                // Finally, it was determined that the stall of a skipped instruction is only
                // when there is a register dependency AND the instruction sets flags / has folded IT:.
                // The caveat is that instructions with `setflags=!InItBlock()` are considered
                // the same way as if they would be setting the flags.
                // See `misc/large_extract/agu_reg_dep.asm` for a wide exploration of preceeding
                // instructions. Moreover, it was established that the LSU variant (e.g., reg offset)
                // doesn't influence this stall.
                AdditionalAction::ComputeAGUValue
                if !skipped || Execute::flags_on_critical_path_to_agu(this.component_mut()) || tainted_dependency
                =>
                    {
                        DecodeState::AGUWaitingForData {
                            addr: instr_address,
                            instr: decoded_instr,
                            folded_instr: None,
                            length: instr_len,
                            dirty_registers,
                            postponed_advance_head,
                            xpsr,
                        }
                    }
                AdditionalAction::Branch => DecodeState::NotifyFetchAboutBranch {
                    addr: instr_address,
                    instr: decoded_instr,
                    folded_instr: None,
                    length: instr_len,
                    dirty_registers,
                    postponed_advance_head,
                    lr_is_dirty: lr_considered_dirty,
                    xpsr,
                },
                AdditionalAction::NoAction | AdditionalAction::ComputeAGUValue // AGU ignored
                => {
                    DecodeState::ResultReady {
                        value: PipelineStepPack {
                            address: instr_address,
                            instruction: decoded_instr,
                            length: instr_len,
                            folded_instruction: None,
                            branch_was_mispredicted: false,
                            branch_kind: brchstat,
                        },
                        postponed_advance_head,
                        xpsr,
                    }
                }
            };
        }
    }

    #[allow(clippy::shadow_unrelated)]
    fn run_decode_phase_for_folded_instruction(
        core: &mut CoreComponent,
        #[cfg(feature = "cycle-debug-logger")] ctx: &mut Context,
        _current_xpsr: XPSR,
    ) {
        // check if the following instruction can fold
        let this = Self::component_to_member(core);
        if !matches!(this.state, DecodeState::WaitingForFetchAndExecute) {
            let head = Fetch::peek_head(core); // ask earlier because of borrow checker
            let this = Self::component_to_member_mut(core);
            if let DecodeState::AGUWaitingForData {
                addr,
                ref instr,
                ref mut folded_instr,
                ref mut length,
                xpsr,
                ..
            }
            | DecodeState::NotifyFetchAboutBranch {
                addr,
                ref instr,
                ref mut folded_instr,
                ref mut length,
                xpsr,
                ..
            }
            | DecodeState::ResultReady {
                value:
                    PipelineStepPack {
                        address: addr,
                        folded_instruction: ref mut folded_instr,
                        instruction: ref instr,
                        ref mut length,
                        ..
                    },
                xpsr,
                ..
            } = this.state
            {
                //  Proof: control_flow/it_fold.asm for the following conditions
                if *length == 1
                    && is_second_instruction_short(&head)
                    && !instr.is_lsu_instruction()
                    && !instr.is_branch()
                    && !matches!(instr, Instruction::ChangeProcessorState { .. })
                    // This is likely decoded as sub.n sp, rm; with ignored flags, this only for rn
                    // Note: This is in contrast to "cmp.n rn, sp", which folds! (such encoding is deprecated though)
                    && !matches!(instr, Instruction::Compare_Register { rn: RegisterID::SP, .. })
                    // This covers add/sub/mov.n with SP
                    && !instr.get_written_registers().get(RegisterID::SP)
                {
                    debug_assert!(
                        folded_instr.is_none(),
                        "length == 1, so there mustn't be any folded instruction"
                    );

                    // *Caution*: xPSR is "outdated" - the main instruction might change the xPSR.
                    //            However we only allow if-then instruction.
                    //            It only checks the it-state, so if main instruction is outside
                    //            it-block, then we're fine. Otherwise we're also fine:
                    //            the hardware doesn't allow to fold if-then with the last
                    //            instruction of previous it-block, and our decoder returns
                    //            `Unpredictable` (old xPSR tells it's inside it-block).
                    //  Check: it_fold.asm test
                    let instr_value = *head.get(1).unwrap();
                    let instr = instruction::decode_short_instruction(instr_value, xpsr);
                    // [ARM-TRM] 3.3.1
                    if matches!(instr, Instruction::IfThen { .. }) {
                        let folded_instr_addr = addr.offset(2);
                        #[cfg(feature = "cycle-debug-logger")]
                        CycleDebugLoggerProxy::new().on_folded_decode(
                            ctx,
                            folded_instr_addr,
                            instr.clone(),
                        );
                        trace!(
                            "Decode (folded) [{:?}]: {:016b} is {}",
                            folded_instr_addr.yellow().bold(),
                            instr_value,
                            instr.yellow().bold(),
                        );
                        println_trace_decoded(&instr, 1);

                        *folded_instr = Some(instr);
                        *length = 2; // was 1, +1 for folded instr
                    }
                }
            } else {
                unreachable!("Have you forgot to update matching state above?");
            }
        }
    }

    // Computes AGU value and passes it to register bank.
    #[allow(clippy::shadow_unrelated)]
    fn run_agu_phase(
        core: &mut CoreComponent,
        #[cfg(feature = "cycle-debug-logger")] ctx: &mut Context,
        new_dirty_registers: Option<RegisterBitmap>,
        fast_forwarded_reg: Option<(RegisterID, Word)>,
    ) {
        if let Some(dirty_registers) = new_dirty_registers {
            let this = Self::component_to_member_mut(core);
            // We match this state two times, because it is easier to reason about
            // and due to issues with borrow checker.
            // TODO: And the second one is here just for the update... (should be only in debug)
            if let DecodeState::AGUWaitingForData {
                dirty_registers: this_dirty_registers,
                ..
            }
            | DecodeState::NotifyFetchAboutBranch {
                dirty_registers: this_dirty_registers,
                ..
            } = &mut this.state
            {
                *this_dirty_registers = dirty_registers;
            }
        }

        let this = Self::component_to_member(core);
        let pc_ctx = if let DecodeState::AGUWaitingForData { addr, .. } = this.state {
            Some(RegisterBank::with_pc(core, get_pc_value(addr)))
        } else {
            None
        };

        let this = Self::component_to_member(core);
        if let DecodeState::AGUWaitingForData {
            addr,
            ref instr,
            ref folded_instr,
            length,
            dirty_registers,
            postponed_advance_head,
            xpsr,
        } = this.state
            && let Some(value) =
                Self::compute_agu_value(core, addr, instr, dirty_registers, fast_forwarded_reg)
        {
            let instr = instr.clone();
            let folded_instr = folded_instr.clone();

            RegisterBank::set_agu_result(core, value);
            let this = Self::component_to_member_mut(core);
            // LSU branches are always execute-time anyway
            let brchstat = Brchstat::from_instruction(
                &instr,
                dirty_registers.get(RegisterID::LR),
                xpsr.in_it_block(),
            );
            debug_assert!(!brchstat.is_decode_time());
            this.state = DecodeState::ResultReady {
                value: PipelineStepPack {
                    address: addr,
                    instruction: instr,
                    length,
                    folded_instruction: folded_instr,
                    branch_was_mispredicted: false,
                    branch_kind: brchstat,
                },
                postponed_advance_head,
                xpsr,
            };

            trace!("Decode  [{:?}]: AGU phase finished", addr);
            #[cfg(feature = "cycle-debug-logger")]
            CycleDebugLoggerProxy::new().on_decode_agu(ctx, addr);
        }

        if let Some(ctx) = pc_ctx {
            ctx.release(core);
        }
    }

    #[inline(always)]
    fn handle_stalled_speculative_branches(core: &mut CoreComponent, new_xpsr: Option<XPSR>) {
        // TODO: HANDLE HERE  AGU OF SKIPPED IT (should be allowed to skip on 2nd cycle)
        let mut this = Self::get_proxy(core);
        if let DecodeState::ResultReady {
            value:
                PipelineStepPack {
                    ref instruction,
                    ref mut branch_kind,
                    #[cfg_attr(not(debug_assertions), allow(unused))]
                    ref mut branch_was_mispredicted,
                    ..
                },
            xpsr,
            ..
        } = this.state
            && branch_kind.is_speculative()
        {
            let condition_failed =
                !branch_kind.evaluate_condition(instruction, new_xpsr.unwrap_or(xpsr));

            #[cfg(debug_assertions)]
            debug_assert_eq!(
                condition_failed, *branch_was_mispredicted,
                "XPSR changed while speculative branch was stalled in decode!"
            );

            debug_assert!(new_xpsr.is_none_or(|new| new == xpsr));

            if condition_failed {
                *branch_kind = Brchstat::NoBranch;
                Fetch::cancel_mispredicted_branch(this.component_mut());
            } else {
                *branch_kind = Brchstat::ConditionalTaken;
                Fetch::confirm_speculative_branch(this.component_mut());
            }
        }
    }

    // Informs fetch about decode time branch or speculative branch.
    // For details check: [ARM-TRM-G] 15.3 Branch status interface
    #[allow(clippy::shadow_unrelated, clippy::too_many_lines)]
    fn run_branch_phase(core: &mut CoreComponent, _cur_xpsr: XPSR) {
        let mut this = Self::get_proxy(core);
        if let DecodeState::NotifyFetchAboutBranch {
            addr,
            ref instr,
            ref folded_instr,
            length,
            dirty_registers,
            postponed_advance_head,
            lr_is_dirty,
            xpsr,
        } = this.state
        {
            // [ARM-ARM] A7.3.3 Note
            // "Instructions that can complete their normal execution by branching are only permitted in an IT block as its last instruction, (...)"
            debug_assert!(!xpsr.in_it_block() || xpsr.last_in_it_block());

            let branch_kind = Brchstat::from_instruction(instr, lr_is_dirty, xpsr.in_it_block());

            let instr_clone = instr.clone();
            let folded_instr_clone = folded_instr.clone();

            if matches!(instr, Instruction::Branch {cond, ..} if !cond.is_al())
                && this.muls_bcond_stall
                && !Execute::is_free(this.component_mut())
            {
                return; // TODO: it's a hacky way to stall branches depending on flags from MULS
            }

            // See description of `PipelineStepPack::branch_was_mispredicted`
            let new_state = 'haxxx: {
                if branch_kind.is_decode_time() {
                    // This stuff we actually know pretty late in the cycle (around tock), so it shouldn't be
                    // used to drive IBus
                    let condition_will_fail = branch_kind.is_conditional()
                        && !branch_kind.evaluate_condition(&instr_clone, xpsr);

                    // TODO: this is a bit hacky to do it here, need some tests
                    #[allow(unused_variables)]
                    let folded_instr_clone = if !condition_will_fail {
                        // folded_instr_clone is discarded
                        if folded_instr_clone.is_some() {
                            todo!("Cannot happen?");
                            // length -= 1;
                        }
                        None
                    } else {
                        folded_instr_clone
                    };

                    // See: [ARM-TRM-G] 15.3, Table 15-3
                    let branch_is_mispredicted =
                        branch_kind.is_speculative() && condition_will_fail;

                    let address = match &instr_clone {
                        // `B <imm>`, `BL`
                        Instruction::BranchWithLink_Immediate { imm32 }
                        | Instruction::Branch { imm32, .. } => {
                            let pc = get_pc_value(addr);

                            pc + *imm32
                        }
                        // See: [ARM-TRM-G] 15.3, Table 15-3
                        // `BLX LR` `BX LR` and `MOV PC, LR`
                        Instruction::BranchWithLinkAndExchange_Register { rm: RegisterID::LR }
                        | Instruction::BranchAndExchange { rm: RegisterID::LR }
                        | Instruction::Move_Register {
                            rd: RegisterID::PC,
                            rm: RegisterID::LR,
                            ..
                        } => {
                            // This is not a debug assertion, as this may actually happen.
                            assert!(
                                dirty_registers.get(RegisterID::LR).implies(lr_is_dirty),
                                "LR is not considered dirty! The real hardware may branch to an unpredictable location. \
                                For instance, it may happen with `xMULL ra, LR, rb, rc; MLA ...; BX LR` sequence. \
                                Make sure to check if it is not the case before investigating a missing interaction."
                            );
                            // NOTE: We can directly call RegisterBank::get_register,
                            // because `!lr_is_dirty` guarantees that LR isn't being written.
                            let lr_value = RegisterBank::get_register(core, RegisterID::LR);

                            let is_bx =
                                matches!(instr_clone, Instruction::BranchAndExchange { .. });

                            // Branch shouldn't be taken if BX LR is decoded and interrupt exit condition has been found.
                            if is_bx
                                && interrupt::check_if_branch_to_given_address_returns_from_exception(
                                core, lr_value,
                            )
                            {
                                // let this = Self::component_to_member_mut(core);
                                // this.state = DecodeState::new_branch_ready(
                                // instr_clone, addr, length, false, false,
                                // );
                                // haxxx, not decode time at all!
                                break 'haxxx
                                    DecodeState::ResultReady {
                                        value: PipelineStepPack {
                                            instruction: instr_clone,
                                            folded_instruction: folded_instr_clone,
                                            address: addr,
                                            branch_kind: Brchstat::ExecuteTimeUnconditional,
                                            branch_was_mispredicted: false,
                                            length,
                                        },
                                        postponed_advance_head,
                                        xpsr,
                                    };
                            } else {
                                lr_value
                            }
                        }
                        _ => panic!(
                            "Cannot run_branch_phase for instruction: {instr_clone:?}, at address: {addr:?}",
                        ),
                    };

                    Fetch::make_decode_time_branch(
                        core,
                        address,
                        branch_kind,
                        branch_is_mispredicted,
                    );

                    DecodeState::ResultReady {
                        value: PipelineStepPack {
                            instruction: instr_clone,
                            folded_instruction: folded_instr_clone,
                            address: addr,
                            branch_kind,
                            branch_was_mispredicted: branch_is_mispredicted,
                            length,
                        },
                        postponed_advance_head,
                        xpsr,
                    }
                } else {
                    DecodeState::ResultReady {
                        value: PipelineStepPack {
                            instruction: instr_clone,
                            folded_instruction: folded_instr_clone,
                            address: addr,
                            branch_kind,
                            branch_was_mispredicted: false,
                            length,
                        },
                        postponed_advance_head,
                        xpsr,
                    }
                }
            };
            let this = Self::component_to_member_mut(core);
            this.state = new_state;
        }
    }

    // Returns None in case of register conflict. On success returns computed value.
    #[allow(clippy::similar_names, clippy::too_many_lines)]
    fn compute_agu_value(
        core: &CoreComponent,
        addr: Address,
        instr: &Instruction,
        dirty_registers: RegisterBitmap,
        fast_forwarded_reg: Option<(RegisterID, Word)>,
    ) -> Option<Word> {
        match instr {
            Instruction::LoadRegister_Register {
                rn, rm, shift, add, ..
            }
            | Instruction::LoadRegisterByte_Register {
                rn, rm, shift, add, ..
            }
            | Instruction::LoadRegisterHalfword_Register {
                rn, rm, shift, add, ..
            }
            | Instruction::LoadRegisterSignedByte_Register {
                rn, rm, shift, add, ..
            }
            | Instruction::LoadRegisterSignedHalfword_Register {
                rn, rm, shift, add, ..
            }
            | Instruction::StoreRegister_Register {
                rn, rm, shift, add, ..
            }
            | Instruction::StoreRegisterByte_Register {
                rn, rm, shift, add, ..
            }
            | Instruction::StoreRegisterHalfword_Register {
                rn, rm, shift, add, ..
            } => {
                let rm_value =
                    Self::get_register_value(core, *rm, dirty_registers, fast_forwarded_reg)?;
                let rn_value =
                    Self::get_register_value(core, *rn, dirty_registers, fast_forwarded_reg)?;

                debug_assert!(
                    shift.srtype == SRType::LSL,
                    "AGU supports only LSL shifts which ignores `carry_in`"
                );
                let offset = rm_value.shift(*shift, false);
                let agu_result = if *add {
                    rn_value + offset
                } else {
                    rn_value - offset
                };
                Some(agu_result)
            }
            Instruction::LoadRegister_Literal { imm32, add, .. }
            | Instruction::LoadRegisterByte_Literal { imm32, add, .. }
            | Instruction::LoadRegisterHalfword_Literal { imm32, add, .. }
            | Instruction::LoadRegisterSignedByte_Literal { imm32, add, .. }
            | Instruction::LoadRegisterSignedHalfword_Literal { imm32, add, .. }
            | Instruction::LoadRegisterDual_Literal { imm32, add, .. } => {
                let pc = Self::get_register_value(
                    core,
                    RegisterID::PC,
                    dirty_registers,
                    fast_forwarded_reg,
                )?;
                let base = pc.align(4);
                let agu_result = if *add { base + *imm32 } else { base - *imm32 };
                Some(agu_result)
            }
            Instruction::LoadRegister_Immediate { rn, imm32, add, .. }
            | Instruction::LoadRegisterByte_Immediate { rn, imm32, add, .. }
            | Instruction::LoadRegisterHalfword_Immediate { rn, imm32, add, .. }
            | Instruction::LoadRegisterSignedByte_Immediate { rn, imm32, add, .. }
            | Instruction::LoadRegisterSignedHalfword_Immediate { rn, imm32, add, .. }
            | Instruction::LoadRegisterDual_Immediate { rn, imm32, add, .. }
            | Instruction::StoreRegister_Immediate { rn, imm32, add, .. }
            | Instruction::StoreRegisterByte_Immediate { rn, imm32, add, .. }
            | Instruction::StoreRegisterHalfword_Immediate { rn, imm32, add, .. }
            | Instruction::StoreRegisterDual_Immediate { rn, imm32, add, .. } => {
                let rn_val =
                    Self::get_register_value(core, *rn, dirty_registers, fast_forwarded_reg)?;

                let agu_result = if *add {
                    rn_val + *imm32
                } else {
                    rn_val - *imm32
                };
                Some(agu_result)
            }

            Instruction::LoadRegisterUnprivileged { rn, imm32, .. }
            | Instruction::LoadRegisterByteUnprivileged { rn, imm32, .. }
            | Instruction::LoadRegisterHalfwordUnprivileged { rn, imm32, .. }
            | Instruction::LoadRegisterSignedByteUnprivileged { rn, imm32, .. }
            | Instruction::LoadRegisterSignedHalfwordUnprivileged { rn, imm32, .. }
            | Instruction::LoadRegisterExclusive { rn, imm32, .. }
            | Instruction::StoreRegisterUnprivileged { rn, imm32, .. }
            | Instruction::StoreRegisterByteUnprivileged { rn, imm32, .. }
            | Instruction::StoreRegisterHalfwordUnprivileged { rn, imm32, .. }
            | Instruction::StoreRegisterExclusive { rn, imm32, .. } => {
                let rn_val =
                    Self::get_register_value(core, *rn, dirty_registers, fast_forwarded_reg)?;

                let agu_result = rn_val + *imm32;
                Some(agu_result)
            }

            Instruction::LoadMultiple { rn, .. } | Instruction::StoreMultiple { rn, .. } => {
                let rn_value =
                    Self::get_register_value(core, *rn, dirty_registers, fast_forwarded_reg)?;
                Some(rn_value)
            }
            Instruction::LoadMultipleDecrementBefore { rn, registers, .. }
            | Instruction::StoreMultipleDecrementBefore { rn, registers, .. } => {
                let rn_value =
                    Self::get_register_value(core, *rn, dirty_registers, fast_forwarded_reg)?;
                Some(rn_value - 4 * registers.count())
            }

            Instruction::TableBranch { rn, rm, is_tbh } => {
                let rn_val =
                    Self::get_register_value(core, *rn, dirty_registers, fast_forwarded_reg)?;
                let rm_val =
                    Self::get_register_value(core, *rm, dirty_registers, fast_forwarded_reg)?;

                let agu_result = if *is_tbh {
                    rn_val + rm_val.lsl(1)
                } else {
                    rn_val + rm_val
                };

                Some(agu_result)
            }
            Instruction::Add_Register {
                rd: RegisterID::PC, ..
            }
            | Instruction::Add_SPPlusRegister {
                rd: RegisterID::PC, ..
            } => {
                // RATIONALE(agu in add pc): see misc/large_tests_extract/add_pc_uses_r0_in_agu.asm
                //
                // During the experiments, it turns out there is an extra CPI stall when the previous instruction
                // does an ALU write to R0.
                // Importantly, there is a difference between UMULL R0, RB, ... and UMULL RB, R0, ...
                // the same as when used with an LSU address dependency!
                //
                // As to why, there is no answer and I doubt we could detect anything without resorting to EM leakage.
                // only idea is that the CPU may do something alike to storing somewhere the base offset of LDM
                // for recovery during interrupts.
                // Maybe the microcode (PLA) encodes the ADD PC, RX to something reassembling:
                // : READ r0->AGU, rx -> ALU, PUT pc -> ALU;
                // EX1: ALU r0 <- rx + pc;
                // EX2: MOV pc <- r0; -- do the jump
                // during jump: MOV r0 <- AGU
                Self::get_register_value(core, RegisterID::R0, dirty_registers, fast_forwarded_reg)
            }
            _ => panic!("AGU not supported for instruction: {instr:?}, at address: {addr:?}"),
        }
    }

    #[allow(clippy::similar_names)]
    #[allow(dead_code)] // Leave for a while, as amy come useful.
    fn does_agu_wait_for_data(instr: &Instruction, dirty_registers: RegisterBitmap) -> bool {
        // TODO: should I take care of fast-forwarded value?
        // Probably no - fast forwarding appears during multi-cycle instruction execution
        // and in these cases postponing advancing fetch is done anyway.

        // TODO: verify whether all necessary instructions are listed here
        let is_dirty = |reg| dirty_registers.get(reg);
        match instr {
            Instruction::LoadRegister_Immediate { rn, .. }
            | Instruction::LoadRegisterByte_Immediate { rn, .. }
            | Instruction::LoadRegisterHalfword_Immediate { rn, .. }
            | Instruction::LoadRegisterSignedByte_Immediate { rn, .. }
            | Instruction::LoadRegisterSignedHalfword_Immediate { rn, .. }
            | Instruction::LoadRegisterDual_Immediate { rn, .. }
            | Instruction::StoreRegister_Immediate { rn, .. }
            | Instruction::StoreRegisterByte_Immediate { rn, .. }
            | Instruction::StoreRegisterHalfword_Immediate { rn, .. }
            | Instruction::StoreRegisterDual_Immediate { rn, .. }
            | Instruction::LoadRegisterUnprivileged { rn, .. }
            | Instruction::LoadRegisterByteUnprivileged { rn, .. }
            | Instruction::LoadRegisterHalfwordUnprivileged { rn, .. }
            | Instruction::LoadRegisterSignedByteUnprivileged { rn, .. }
            | Instruction::LoadRegisterSignedHalfwordUnprivileged { rn, .. }
            | Instruction::LoadRegisterExclusive { rn, .. }
            | Instruction::StoreRegisterUnprivileged { rn, .. }
            | Instruction::StoreRegisterByteUnprivileged { rn, .. }
            | Instruction::StoreRegisterHalfwordUnprivileged { rn, .. }
            | Instruction::StoreRegisterExclusive { rn, .. }
            | Instruction::LoadMultiple { rn, .. }
            | Instruction::LoadMultipleDecrementBefore { rn, .. }
            | Instruction::StoreMultiple { rn, .. }
            | Instruction::StoreMultipleDecrementBefore { rn, .. } => is_dirty(*rn),

            Instruction::LoadRegister_Register { rn, rm, .. }
            | Instruction::LoadRegisterByte_Register { rn, rm, .. }
            | Instruction::LoadRegisterHalfword_Register { rn, rm, .. }
            | Instruction::LoadRegisterSignedByte_Register { rn, rm, .. }
            | Instruction::LoadRegisterSignedHalfword_Register { rn, rm, .. }
            | Instruction::StoreRegister_Register { rn, rm, .. }
            | Instruction::StoreRegisterByte_Register { rn, rm, .. }
            | Instruction::StoreRegisterHalfword_Register { rn, rm, .. }
            | Instruction::TableBranch { rn, rm, .. } => is_dirty(*rn) || is_dirty(*rm),

            _ => false,
        }
    }

    fn get_additional_action(instr: &Instruction) -> AdditionalAction {
        match instr {
            Instruction::LoadRegister_Immediate { .. }
            | Instruction::LoadRegister_Literal { .. }
            | Instruction::LoadRegister_Register { .. }
            | Instruction::LoadRegisterByte_Immediate { .. }
            | Instruction::LoadRegisterByte_Literal { .. }
            | Instruction::LoadRegisterByte_Register { .. }
            | Instruction::LoadRegisterHalfword_Immediate { .. }
            | Instruction::LoadRegisterHalfword_Literal { .. }
            | Instruction::LoadRegisterHalfword_Register { .. }
            | Instruction::LoadRegisterSignedByte_Immediate { .. }
            | Instruction::LoadRegisterSignedByte_Literal { .. }
            | Instruction::LoadRegisterSignedByte_Register { .. }
            | Instruction::LoadRegisterSignedHalfword_Immediate { .. }
            | Instruction::LoadRegisterSignedHalfword_Literal { .. }
            | Instruction::LoadRegisterSignedHalfword_Register { .. }
            | Instruction::LoadRegisterDual_Immediate { .. }
            | Instruction::LoadRegisterDual_Literal { .. }
            | Instruction::LoadRegisterUnprivileged { .. }
            | Instruction::LoadRegisterByteUnprivileged { .. }
            | Instruction::LoadRegisterHalfwordUnprivileged { .. }
            | Instruction::LoadRegisterSignedByteUnprivileged { .. }
            | Instruction::LoadRegisterSignedHalfwordUnprivileged { .. }
            | Instruction::LoadRegisterExclusive { .. }
            | Instruction::LoadMultiple { .. }
            | Instruction::LoadMultipleDecrementBefore { .. }
            | Instruction::StoreRegister_Immediate { .. }
            | Instruction::StoreRegister_Register { .. }
            | Instruction::StoreRegisterByte_Immediate { .. }
            | Instruction::StoreRegisterByte_Register { .. }
            | Instruction::StoreRegisterHalfword_Immediate { .. }
            | Instruction::StoreRegisterHalfword_Register { .. }
            | Instruction::StoreRegisterDual_Immediate { .. }
            | Instruction::StoreRegisterUnprivileged { .. }
            | Instruction::StoreRegisterByteUnprivileged { .. }
            | Instruction::StoreRegisterHalfwordUnprivileged { .. }
            | Instruction::StoreRegisterExclusive { .. }
            | Instruction::StoreMultiple { .. }
            | Instruction::StoreMultipleDecrementBefore { .. }
            | Instruction::TableBranch { .. } => AdditionalAction::ComputeAGUValue,

            // search for "RATIONALE(agu in add pc)"
            Instruction::Add_Register {
                rd: RegisterID::PC, ..
            }
            | Instruction::Add_SPPlusRegister {
                rd: RegisterID::PC, ..
            } if *cm_hyp::ADD_PC_USES_AGU_R0 => AdditionalAction::ComputeAGUValue,

            Instruction::Branch { .. }
            | Instruction::BranchWithLink_Immediate { .. }
            | Instruction::BranchWithLinkAndExchange_Register { rm: RegisterID::LR }
            | Instruction::BranchAndExchange { rm: RegisterID::LR }
            | Instruction::Move_Register {
                rd: RegisterID::PC,
                rm: RegisterID::LR,
                ..
            } => AdditionalAction::Branch,
            _ => AdditionalAction::NoAction,
        }
    }

    fn get_register_value(
        core: &CoreComponent,
        register: RegisterID,
        dirty_registers: RegisterBitmap,
        fast_forwarded_reg: Option<(RegisterID, Word)>,
    ) -> Option<Word> {
        if !dirty_registers.get(register) {
            return Some(RegisterBank::get_register(core, register));
        }

        if let Some((ff_register, ff_value)) = fast_forwarded_reg
            && register == ff_register
        {
            return Some(ff_value);
        }
        None
    }
}

// ---------------------------------------------------------------------------
// helper functions and types implementation
// ---------------------------------------------------------------------------

impl TriggerData {
    pub(super) fn ignore_currently_decoding_instruction() -> Self {
        TriggerData::IgnoreCurrent
    }

    pub(super) fn next_instruction(
        dirty_registers: RegisterBitmap,
        fast_forwarded_xpsr: XPSR,
        postponed_advance_head: bool,
    ) -> Self {
        TriggerData::DecodeCurrent {
            dirty_registers,
            xpsr: fast_forwarded_xpsr,
            postponed_advance_head,
        }
    }
}

impl Display for TriggerData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IgnoreCurrent => write!(f, "{}", "IgnoreCurrent".red()),
            Self::DecodeCurrent {
                dirty_registers,
                xpsr,
                postponed_advance_head,
            } => write!(
                f,
                "{} {} {} {}",
                "DecodeCurrent".green(),
                dirty_registers.red(),
                xpsr,
                dife(
                    *postponed_advance_head,
                    "POSTPONED".magenta(),
                    "not postponed".cyan()
                )
            ),
        }
    }
}

impl DecodeState {
    #[inline]
    fn has_folded_instruction(&self) -> bool {
        match self {
            Self::AGUWaitingForData { folded_instr, .. }
            | Self::NotifyFetchAboutBranch { folded_instr, .. } => folded_instr.is_some(),
            Self::ResultReady { value, .. } => value.folded_instruction.is_some(),
            Self::WaitingForFetchAndExecute => false,
        }
    }
}

// Given instruction address returns PC value
#[inline]
fn get_pc_value(instruction_address: Address) -> Word {
    // See: [ARM-ARM] B1.4.7
    // See: [ARM-ARM] A5.1.2
    Word::from(instruction_address) + 4
}

#[allow(clippy::manual_range_patterns)]
fn is_long_instruction(first_halfword: u16) -> bool {
    // [ARM-ARM] A5.1 Thumb instruction set encoding
    matches!(
        (first_halfword >> 11) & 0b11111,
        0b11101 | 0b11110 | 0b11111
    )
}

fn is_second_instruction_short(fetch_head: &FDReg) -> bool {
    fetch_head.get(1).is_some_and(|&b| !is_long_instruction(b))
}

fn looks_like_it(halfword: u16) -> bool {
    // [ARM-ARM] A7.7.38 IT - in contrast to the decoder, we don't care about unpredictable
    // Better bitstring extract/match pls!
    halfword >> 8 == 0b1011_1111 && halfword & 0xf != 0
}

#[allow(clippy::print_stdout, clippy::print_stderr)]
fn println_trace_decoded(decoded_instr: &Instruction, instr_len: u8) {
    if *println_traces::DECODED_INSTRUCTIONS {
        println!(
            "{} {}",
            <&Instruction as Into<&'static str>>::into(decoded_instr),
            instr_len
        );
    }
}
