//! Implements "Operation" of each instruction described in [ARM-ARM]
//! and helper functions from [ARM-ARM] used by "Operation" sections.

#[cfg(feature = "cycle-debug-logger")]
use super::ActiveSlot;
use super::{
    Execute, InstructionExecutionContext, InstructionExecutionState, LSU,
    MultiplyLongExecutionState, ReadDataCallback, SingleLoadStoreExecutionState,
};
use crate::common::new_ahb::Size;
use crate::common::new_ahb::databus::DataBus;
use crate::common::{BitstringUtils, SRType, Shift, Word, bitstring::constants as bsc};
use crate::component::core::decode::Brchstat;
use crate::component::core::register_bank::ItState;
use crate::component::core::{
    CoreComponent, Fetch, InterruptEntryAndExitHandler, RegisterBank,
    builtins::{have_dsp_ext, have_fp_ext, integer_zero_divide_trapping_enabled},
    instruction::Instruction,
    interrupt,
    register_bank::{BasePriorityMaskRegister, RegisterID, XPSR},
};
use crate::component::nvic::InterruptId;
use crate::engine::DisableableComponent;
use crate::engine::{Context, Subcomponent};
#[cfg(feature = "cycle-debug-logger")]
use crate::proxy::CycleDebugLoggerProxy;
use crate::proxy::{DWTProxy, NVICProxy};
use crate::utils::IfExpr;
use crate::{Bitstring, bitstring_extract, bitstring_substitute};
use cc2650_constants::operation::{ExecutionMode, StackPointer};
use log::{debug, info, trace, warn};
use std::ops::Not;

pub(super) mod memory_instruction;

const LOW_HALF_MASK_I64: i64 = 0x0000_0000_FFFF_FFFF;
const LOW_HALF_MASK_U64: u64 = 0x0000_0000_FFFF_FFFF;

/// See: [ARM-TRM-G] 15.3 for branch types documentation
#[derive(Debug)]
pub(super) enum ExecutionStepResult {
    Continue {
        trigger_decode: bool,
        lsu_branch_expected: bool,
    },
    NextInstruction,
    Skipped,
    ExecuteTimeBranch {
        address: Word,
    },
    DecodeTimeBranch {
        address: Word,
        // That is, it was not confirmed to be mispredicted or valid in the last cycle
        // from the critical-path on IBus perspective.
        was_speculative: bool,
    },
    BranchNotTaken,
    LateBranch,
    ExceptionReturn,
    PreSleep,
    Sleep,
}

impl ExecutionStepResult {
    pub(super) fn runs_next_instruction(&self) -> bool {
        match self {
            Self::Continue { .. }
            | Self::ExecuteTimeBranch { .. }
            | Self::DecodeTimeBranch { .. }
            | Self::LateBranch
            | Self::PreSleep
            | Self::ExceptionReturn => false,
            Self::NextInstruction | Self::Skipped | Self::BranchNotTaken | Self::Sleep => true,
        }
    }

    pub(super) fn does_branch(&self) -> bool {
        match self {
            Self::Continue { .. }
            | Self::NextInstruction
            | Self::Skipped
            | Self::BranchNotTaken
            | Self::PreSleep
            | Self::Sleep => false,
            Self::ExecuteTimeBranch { .. }
            | Self::DecodeTimeBranch { .. }
            | Self::LateBranch
            | Self::ExceptionReturn => true,
        }
    }
}

impl Execute {
    // We `allow(clippy::similar_names)` because we frequently use here names
    // connected with register - `rn`, `rm`, `rn_val`, `rm_val`, etc.
    // which are similar to each other to Clippy.
    // We `allow(clippy::cognitive_complexity)` because we want to create one big
    // match with all instructions of the Cortex-M3.
    // We should use `allow(clippy::if_not_else)` only in line that it refers to.
    // TODO: find better solution to this problem (so the scope of allowance is limited)
    #[allow(
        clippy::similar_names,
        clippy::shadow_unrelated,
        clippy::too_many_lines,
        clippy::cognitive_complexity
    )]
    pub(super) fn execute_instruction_step(
        core: &mut CoreComponent,
        ctx: &mut Context,
    ) -> ExecutionStepResult {
        let this = Self::component_to_member_mut(core);

        let active_slot = this.active_slot;

        let iectx = this.get_active_instruction_execution_context();
        let xpsr = iectx.visible_xpsr;

        let instr = iectx.instruction().clone();

        // Notify CDL about execution
        #[cfg(feature = "cycle-debug-logger")]
        match active_slot {
            ActiveSlot::None => panic!("There should be some slot active"),
            ActiveSlot::Main => {
                CycleDebugLoggerProxy::new().on_execute(ctx, iectx.instruction_address());
            }
            ActiveSlot::Pipelined => {
                CycleDebugLoggerProxy::new().on_pipelined_execute(ctx, iectx.instruction_address());
            }
        }

        trace!(
            "{slot:?} Execute [{addr:?}]: {instr} | {instr:?}",
            slot = active_slot,
            addr = iectx.instruction_address(),
            instr = instr,
        );

        if xpsr.in_it_block() {
            // NOTE:
            //   According to [ARM-ARM] A7.3.3 this should happen *after* finishing the instruction.
            //   Underneath, we accumulate xPSR changes in the very first cycle of `iectx`
            //   execution, and report the new value to register bank after instruction execution
            //   finished. It's helpful in case if an instruction must be interrupted -
            //   there's no need to revert unwanted side effects.
            //   We also keep an invariant: propagate proper xPSR value to folded instruction,
            //   pipelined instruction and to the Decode - the values they should see in regular
            //   sequential execution.
            if iectx.cycle_cntr == 0 {
                iectx.modify_epsr(XPSR::with_it_advanced);
            }

            // It's enough to check the condition in the first cycle.
            if iectx.cycle_cntr == 0 {
                // [ARM-ARM] A7.3.1 conditional branches covered separately
                if !instr.is_unconditional_in_it_block() && !xpsr.it_condition_passed() {
                    #[cfg(feature = "cycle-debug-logger")]
                    CycleDebugLoggerProxy::new().on_execute_conditionally_skipped(ctx);
                    trace!(
                        "Execute [{:?}]: IT condition not passed, skipping the instruction",
                        iectx.instruction_address()
                    );
                    iectx.mark_all_registers_clean();

                    return if iectx.pipeline_step_pack.branch_kind.is_speculative() {
                        ExecutionStepResult::BranchNotTaken
                    } else {
                        ExecutionStepResult::Skipped
                    };
                }
            }
        }

        match instr {
            Instruction::Unsupported { name } => panic!(
                "Unsupported operation at address: {:?}. Name: \"{}\". \
                Note that instruction might not be declared by TI CC2650 (see [TI-TRM]) \
                as supported and be actually supported by underlying ARM Cortex-M3 \
                (see [ARM-ARM]). Supporting the instruction requires careful research.",
                iectx.instruction_address(),
                name,
            ),
            // [ARM-ARM] A5.1.1
            // TODO: should throw UndefinedInstruction exception
            Instruction::Undefined => panic!(
                "undefined operation at address: {:?}",
                iectx.instruction_address()
            ),
            // [ARM-ARM] A5.1.1
            Instruction::Unpredictable => panic!(
                "unpredictable operation at address: {:?}",
                iectx.instruction_address()
            ),

            // [ARM-ARM] A7.7.1
            Instruction::AddWithCarry_Immediate {
                rd,
                rn,
                imm32,
                setflags,
            } => {
                let rn_val = RegisterBank::get_register(core, rn);

                let (result, carry, overflow) =
                    Word::add_with_carry(rn_val, imm32, xpsr.carry_flag());

                Self::set_register(core, rd, result);
                if setflags {
                    Self::modify_apsr(core, |v| {
                        v.with_negative(result.get_bit(31))
                            .with_zero(result.is_zero_bit())
                            .with_carry(carry)
                            .with_overflow(overflow)
                    });
                }

                ExecutionStepResult::NextInstruction
            }

            // [ARM-ARM] A7.7.2
            Instruction::AddWithCarry_Register {
                rd,
                rn,
                rm,
                shift,
                setflags,
                ..
            } => {
                let rm_val = RegisterBank::get_register(core, rm);
                let rn_val = RegisterBank::get_register(core, rn);

                let shifted = rm_val.shift(shift, xpsr.carry_flag());
                let (result, carry, overflow) =
                    Word::add_with_carry(rn_val, shifted, xpsr.carry_flag());

                Self::set_register(core, rd, result);
                if setflags {
                    Self::modify_apsr(core, |v| {
                        v.with_negative(result.get_bit(31))
                            .with_zero(result.is_zero_bit())
                            .with_carry(carry)
                            .with_overflow(overflow)
                    });
                }

                ExecutionStepResult::NextInstruction
            }

            // [ARM-ARM] A7.7.3
            Instruction::Add_Immediate {
                rd,
                rn,
                imm32,
                setflags,
                ..
            } => {
                let rn_val = RegisterBank::get_register(core, rn);

                let (result, carry, overflow) = Word::add_with_carry(rn_val, imm32, false);

                Self::set_register(core, rd, result);
                if setflags {
                    Self::modify_apsr(core, |v| {
                        v.with_negative(result.get_bit(31))
                            .with_zero(result.is_zero_bit())
                            .with_carry(carry)
                            .with_overflow(overflow)
                    });
                }

                ExecutionStepResult::NextInstruction
            }

            // [ARM-ARM] A7.7.4
            Instruction::Add_Register {
                rd,
                rn,
                rm,
                shift,
                setflags,
                ..
            } => {
                let rm_val = RegisterBank::get_register(core, rm);
                let rn_val = RegisterBank::get_register(core, rn);

                let shifted = rm_val.shift(shift, xpsr.carry_flag());
                let (result, carry, overflow) = Word::add_with_carry(rn_val, shifted, false);

                if rd == RegisterID::PC {
                    debug_assert!(!setflags);
                    Self::alu_write_pc(core, result)
                } else {
                    Self::set_register(core, rd, result);
                    if setflags {
                        Self::modify_apsr(core, |v| {
                            v.with_negative(result.get_bit(31))
                                .with_zero(result.is_zero_bit())
                                .with_carry(carry)
                                .with_overflow(overflow)
                        });
                    }
                    ExecutionStepResult::NextInstruction
                }
            }

            // [ARM-ARM] A7.7.5
            Instruction::Add_SPPlusImmediate {
                rd,
                imm32,
                setflags,
            } => {
                let sp_val = RegisterBank::get_register(core, RegisterID::SP);

                let (result, carry, overflow) = Word::add_with_carry(sp_val, imm32, false);

                Self::set_register(core, rd, result);
                if setflags {
                    Self::modify_apsr(core, |v| {
                        v.with_negative(result.get_bit(31))
                            .with_zero(result.is_zero_bit())
                            .with_carry(carry)
                            .with_overflow(overflow)
                    });
                }

                ExecutionStepResult::NextInstruction
            }

            // [ARM-ARM] A7.7.6
            Instruction::Add_SPPlusRegister {
                rd,
                rm,
                shift,
                setflags,
            } => {
                let rm_val = RegisterBank::get_register(core, rm);
                let sp_val = RegisterBank::get_register(core, RegisterID::SP);

                let shifted = rm_val.shift(shift, xpsr.carry_flag());
                let (result, carry, overflow) = Word::add_with_carry(sp_val, shifted, false);

                if rd == RegisterID::PC {
                    debug_assert!(!setflags);
                    // "The use of the PC as <Rd> in encoding T1 is deprecated."
                    Self::alu_write_pc(core, result)
                } else {
                    Self::set_register(core, rd, result);
                    if setflags {
                        Self::modify_apsr(core, |v| {
                            v.with_negative(result.get_bit(31))
                                .with_zero(result.is_zero_bit())
                                .with_carry(carry)
                                .with_overflow(overflow)
                        });
                    }
                    ExecutionStepResult::NextInstruction
                }
            }

            // [ARM-ARM] A7.7.7
            Instruction::AddressToRegister { rd, imm32, add } => {
                let pc = RegisterBank::get_register(core, RegisterID::PC);

                let result = if add {
                    pc.align(4) + imm32
                } else {
                    pc.align(4) - imm32
                };

                Self::set_register(core, rd, result);

                ExecutionStepResult::NextInstruction
            }

            // [ARM-ARM] A7.7.8
            Instruction::And_Immediate {
                rd,
                rn,
                imm32,
                setflags,
                carry,
            } => {
                let rn_val = RegisterBank::get_register(core, rn);

                let result = rn_val & imm32;

                Self::set_register(core, rd, result);
                if setflags {
                    Self::modify_apsr(core, |v| {
                        v.with_negative(result.get_bit(31))
                            .with_zero(result.is_zero_bit())
                            .with_carry(carry)
                        // APSR.V unchanged
                    });
                }
                ExecutionStepResult::NextInstruction
            }

            // [ARM-ARM] A7.7.9
            Instruction::And_Register {
                rd,
                rn,
                rm,
                shift,
                setflags,
                ..
            } => {
                let rm_val = RegisterBank::get_register(core, rm);
                let rn_val = RegisterBank::get_register(core, rn);

                let (shifted, carry) = rm_val.shift_c(shift, xpsr.carry_flag());
                let result = rn_val & shifted;

                Self::set_register(core, rd, result);
                if setflags {
                    Self::modify_apsr(core, |v| {
                        v.with_negative(result.get_bit(31))
                            .with_zero(result.is_zero_bit())
                            .with_carry(carry)
                        // APSR.V unchanged
                    });
                }
                ExecutionStepResult::NextInstruction
            }

            // [ARM-ARM] A7.7.10
            Instruction::ArithmeticShiftRight_Immediate {
                rd,
                rm,
                shift,
                setflags,
                ..
            } => {
                debug_assert!(shift.srtype == SRType::ASR);
                let rm_val = RegisterBank::get_register(core, rm);

                let (result, carry) = rm_val.shift_c(shift, xpsr.carry_flag());

                Self::set_register(core, rd, result);
                if setflags {
                    Self::modify_apsr(core, |v| {
                        v.with_negative(result.get_bit(31))
                            .with_zero(result.is_zero_bit())
                            .with_carry(carry)
                        // APSR.V unchanged
                    });
                }

                ExecutionStepResult::NextInstruction
            }

            // [ARM-ARM] A7.7.11
            Instruction::ArithmeticShiftRight_Register {
                rd,
                rn,
                rm,
                setflags,
                ..
            } => {
                let rn_val = RegisterBank::get_register(core, rn);
                let rm_val = RegisterBank::get_register(core, rm);
                let shift_n = u8::from(bitstring_extract!(rm_val<7:0> | 8 bits));
                let shift = Shift {
                    srtype: SRType::ASR,
                    amount: shift_n,
                };

                let (result, carry) = rn_val.shift_c(shift, xpsr.carry_flag());

                Self::set_register(core, rd, result);
                if setflags {
                    Self::modify_apsr(core, |v| {
                        v.with_negative(result.get_bit(31))
                            .with_zero(result.is_zero_bit())
                            .with_carry(carry)
                        // APSR.V unchanged
                    });
                }

                ExecutionStepResult::NextInstruction
            }

            // [ARM-ARM] A7.7.12
            Instruction::Branch { imm32, cond } => {
                if cond.passed(xpsr) {
                    let target_address = RegisterBank::get_register(core, RegisterID::PC) + imm32;

                    let this = Self::component_to_member_mut(core);
                    let iectx = this.get_active_instruction_execution_context();
                    iectx.mark_register_clean(RegisterID::PC);

                    ExecutionStepResult::DecodeTimeBranch {
                        address: target_address,
                        was_speculative: iectx.pipeline_step_pack.branch_kind.is_speculative(),
                    }
                } else {
                    #[cfg(feature = "cycle-debug-logger")]
                    CycleDebugLoggerProxy::new().on_execute_conditionally_skipped(ctx);
                    trace!(
                        "Execute [{:?}]: Branch condition not passed, skipping the instruction",
                        iectx.pipeline_step_pack.address
                    );
                    // TODO: this skill keeps PC dirty -- check if a skipped branch was just 1 cycle in decode,
                    // then LDR (literal) will have a stalled AGU phase.
                    if iectx.pipeline_step_pack.branch_kind == Brchstat::ExecuteTimeConditional {
                        iectx.mark_register_clean(RegisterID::PC);
                    }
                    ExecutionStepResult::BranchNotTaken
                }
            }

            // [ARM-ARM] A7.7.13
            Instruction::BitFieldClear { rd, msbit, lsbit } => {
                let rd_uint = RegisterBank::get_register(core, rd).uint();

                let mask = mask_between_bits(msbit, lsbit);
                let result = rd_uint & !mask;

                Self::set_register(core, rd, Word::from(result));
                ExecutionStepResult::NextInstruction
            }

            // [ARM-ARM] A7.7.14
            Instruction::BitFieldInsert {
                rd,
                rn,
                msbit,
                lsbit,
            } => {
                let rd_uint = RegisterBank::get_register(core, rd).uint();
                let rn_uint = RegisterBank::get_register(core, rn).uint();

                let rd_mask = mask_between_bits(msbit, lsbit);
                let rn_mask = mask_between_bits(msbit - lsbit, 0);
                let result = (rd_uint & !rd_mask) | ((rn_uint & rn_mask) << lsbit);

                Self::set_register(core, rd, Word::from(result));
                ExecutionStepResult::NextInstruction
            }

            // [ARM-ARM] A7.7.15
            Instruction::BitClear_Immediate {
                rd,
                rn,
                imm32,
                setflags,
                carry,
            } => {
                let rn_val = RegisterBank::get_register(core, rn);

                let result = rn_val & imm32.not();

                Self::set_register(core, rd, result);
                if setflags {
                    Self::modify_apsr(core, |v| {
                        v.with_negative(result.get_bit(31))
                            .with_zero(result.is_zero_bit())
                            .with_carry(carry)
                        // APSR.V unchanged
                    });
                }

                ExecutionStepResult::NextInstruction
            }

            // [ARM-ARM] A7.7.16
            Instruction::BitClear_Register {
                rd,
                rn,
                rm,
                shift,
                setflags,
                ..
            } => {
                let rn_val = RegisterBank::get_register(core, rn);
                let rm_val = RegisterBank::get_register(core, rm);

                let (shifted, carry) = rm_val.shift_c(shift, xpsr.carry_flag());
                let result = rn_val & shifted.not();

                Self::set_register(core, rd, result);
                if setflags {
                    Self::modify_apsr(core, |v| {
                        v.with_negative(result.get_bit(31))
                            .with_zero(result.is_zero_bit())
                            .with_carry(carry)
                        // APSR.V unchanged
                    });
                }

                ExecutionStepResult::NextInstruction
            }

            // [ARM-ARM] A7.7.17
            Instruction::Breakpoint { imm32 } => unimplemented!(
                "BKPTInstrDebugEvent() [imm32 = 0x{:x}, at address {:?}]",
                imm32,
                iectx.instruction_address()
            ),

            // [ARM-ARM] A7.7.18
            Instruction::BranchWithLink_Immediate { imm32 } => {
                let branch_kind = iectx.pipeline_step_pack.branch_kind;
                let target_address = RegisterBank::get_register(core, RegisterID::PC) + imm32;
                let link_address =
                    RegisterBank::get_register(core, RegisterID::PC).with_bit_set(0, true);

                Self::set_register(core, RegisterID::LR, link_address);

                if branch_kind.is_decode_time() {
                    let this = Self::component_to_member_mut(core);
                    this.get_active_instruction_execution_context()
                        .mark_register_clean(RegisterID::PC);
                    ExecutionStepResult::DecodeTimeBranch {
                        address: target_address,
                        was_speculative: branch_kind.is_speculative(),
                    }
                } else {
                    debug_assert!(branch_kind.is_execute_time());
                    Self::branch_write_pc(core, target_address)
                }
            }

            // [ARM-ARM] A7.7.19
            Instruction::BranchWithLinkAndExchange_Register { rm } => {
                let branch_kind = iectx.pipeline_step_pack.branch_kind;
                let target_address = RegisterBank::get_register(core, rm);
                let next_instr_addr = RegisterBank::get_register(core, RegisterID::PC) - 2;
                let link_address = next_instr_addr.with_bit_set(0, true);

                Self::set_register(core, RegisterID::LR, link_address);

                if branch_kind.is_decode_time() {
                    let iectx = Self::component_to_member_mut(core)
                        .get_active_instruction_execution_context();
                    iectx.mark_register_clean(RegisterID::PC);
                    ExecutionStepResult::DecodeTimeBranch {
                        address: target_address,
                        was_speculative: branch_kind.is_speculative(),
                    }
                } else {
                    debug_assert!(branch_kind.is_execute_time());
                    Self::blx_write_pc(core, target_address)
                }
            }

            // [ARM-ARM] A7.7.20
            Instruction::BranchAndExchange { rm } => {
                let branch_kind = iectx.pipeline_step_pack.branch_kind;
                let target_address = RegisterBank::get_register(core, rm);

                if branch_kind.is_decode_time() {
                    let iectx = Self::component_to_member_mut(core)
                        .get_active_instruction_execution_context();

                    iectx.mark_register_clean(RegisterID::PC);
                    ExecutionStepResult::DecodeTimeBranch {
                        address: target_address,
                        was_speculative: branch_kind.is_speculative(),
                    }
                } else {
                    debug_assert!(branch_kind.is_execute_time());
                    Self::bx_write_pc(core, target_address)
                }
            }

            // [ARM-ARM] A7.7.21
            Instruction::CompareAndBranch { rn, imm32, nonzero } => {
                let pc_value = RegisterBank::get_register(core, RegisterID::PC);
                let rn_value = RegisterBank::get_register(core, rn);

                #[allow(clippy::if_not_else)] // To allow doc-like style of conditions.
                if nonzero != rn_value.is_zero() {
                    Self::branch_write_pc(core, pc_value + imm32)
                } else {
                    let this = Self::component_to_member_mut(core);
                    let iectx = this.get_active_instruction_execution_context();

                    #[cfg(feature = "cycle-debug-logger")]
                    CycleDebugLoggerProxy::new().on_execute_conditionally_skipped(ctx);
                    trace!(
                        "Execute [{:?}]: Branch condition not passed, skipping the instruction",
                        iectx.pipeline_step_pack.address
                    );

                    iectx.mark_register_clean(RegisterID::PC);
                    ExecutionStepResult::BranchNotTaken
                }
            }

            // [ARM-ARM] A7.7.23
            Instruction::ClearExclusive => unimplemented!(
                "ClearExclusiveLocal(ProcessorID()) [at address {:?}]",
                iectx.instruction_address()
            ),

            // [ARM-ARM] A7.7.24
            Instruction::CountLeadingZeros { rd, rm } => {
                let rm_val = RegisterBank::get_register(core, rm);

                let result = rm_val.count_leading_zero_bits();

                Self::set_register(core, rd, Word::from(result));
                ExecutionStepResult::NextInstruction
            }

            // [ARM-ARM] A7.7.25
            Instruction::CompareNegative_Immediate { rn, imm32 } => {
                let rn_val = RegisterBank::get_register(core, rn);

                let (result, carry, overflow) = Word::add_with_carry(rn_val, imm32, false);

                Self::modify_apsr(core, |v| {
                    v.with_negative(result.get_bit(31))
                        .with_zero(result.is_zero_bit())
                        .with_carry(carry)
                        .with_overflow(overflow)
                });
                ExecutionStepResult::NextInstruction
            }

            // [ARM-ARM] A7.7.26
            Instruction::CompareNegative_Register { rn, rm, shift } => {
                let rn_val = RegisterBank::get_register(core, rn);
                let rm_val = RegisterBank::get_register(core, rm);

                let shifted = rm_val.shift(shift, xpsr.carry_flag());
                let (result, carry, overflow) = Word::add_with_carry(rn_val, shifted, false);

                Self::modify_apsr(core, |v| {
                    v.with_negative(result.get_bit(31))
                        .with_zero(result.is_zero_bit())
                        .with_carry(carry)
                        .with_overflow(overflow)
                });
                ExecutionStepResult::NextInstruction
            }

            // [ARM-ARM] A7.7.27
            Instruction::Compare_Immediate { rn, imm32 } => {
                let rn_val = RegisterBank::get_register(core, rn);

                let (result, carry, overflow) = Word::add_with_carry(rn_val, imm32.not(), true);

                Self::modify_apsr(core, |v| {
                    v.with_negative(result.get_bit(31))
                        .with_zero(result.is_zero_bit())
                        .with_carry(carry)
                        .with_overflow(overflow)
                });
                ExecutionStepResult::NextInstruction
            }

            // [ARM-ARM] A7.7.28
            Instruction::Compare_Register { rn, rm, shift } => {
                let rm_val = RegisterBank::get_register(core, rm);
                let rn_val = RegisterBank::get_register(core, rn);

                let shifted = rm_val.shift(shift, xpsr.carry_flag());
                let (result, carry, overflow) = Word::add_with_carry(rn_val, shifted.not(), true);

                Self::modify_apsr(core, |v| {
                    v.with_negative(result.get_bit(31))
                        .with_zero(result.is_zero_bit())
                        .with_carry(carry)
                        .with_overflow(overflow)
                });
                ExecutionStepResult::NextInstruction
            }

            // [ARM-ARM] A7.7.29
            // [ARM-ARM] B5.2.1
            Instruction::ChangeProcessorState {
                enable,
                disable,
                affect_pri,
                affect_fault,
            } => {
                debug_assert_eq!(enable, !disable);
                debug_assert!(affect_pri || affect_fault);

                let mask = !enable;
                let first_cycle = iectx.cycle_cntr == 0;
                let primask = RegisterBank::get_primask(core);
                let faultmask = RegisterBank::get_faultmask(core);
                let increases_priority = (primask.value_increases_priority(mask) && affect_pri)
                    || (faultmask.value_increases_priority(mask) && affect_fault);

                if first_cycle && increases_priority {
                    // [ARM-ARM] B5.2.1
                    // If execution of a CPS instruction increases the execution
                    // priority, the CPS execution serializes that change to the
                    // instruction stream.
                    //
                    // Experiments show that in such a case the instruction
                    // takes one extra cycle to execute.
                    ExecutionStepResult::Continue {
                        trigger_decode: true,
                        lsu_branch_expected: false,
                    }
                } else {
                    if affect_pri {
                        RegisterBank::set_primask(core, ctx, primask.with_primask(mask));
                    }
                    if affect_fault {
                        RegisterBank::set_faultmask(core, ctx, faultmask.with_faultmask(mask));
                    }
                    ExecutionStepResult::NextInstruction
                }
            }

            // [ARM-ARM] A7.7.33
            Instruction::DataMemoryBarrier { option } => {
                debug_assert_eq!(option, bsc::C_1111, "Reserved (unsupported) DMB option");
                unimplemented!(
                    "DataMemoryBarrier(option) [option = 0x{:x}, at address {:?}]",
                    u8::from(option),
                    iectx.instruction_address()
                );
            }

            // [ARM-ARM] A7.7.34
            Instruction::DataSynchronizationBarrier { option } => {
                debug_assert_eq!(option, bsc::C_1111, "Reserved (unsupported) DSB option");
                unimplemented!(
                    "DataSynchronizationBarrier(option) [option = 0x{:x}, at address {:?}]",
                    u8::from(option),
                    iectx.instruction_address()
                );
            }

            // [ARM-ARM] A7.7.35
            Instruction::ExclusiveOr_Immediate {
                rd,
                rn,
                imm32,
                setflags,
                carry,
            } => {
                let rn_val = RegisterBank::get_register(core, rn);

                let result = rn_val ^ imm32;

                Self::set_register(core, rd, result);
                if setflags {
                    Self::modify_apsr(core, |v| {
                        v.with_negative(result.get_bit(31))
                            .with_zero(result.is_zero_bit())
                            .with_carry(carry)
                        // APSR.V unchanged
                    });
                }
                ExecutionStepResult::NextInstruction
            }

            // [ARM-ARM] A7.7.36
            Instruction::ExclusiveOr_Register {
                rd,
                rn,
                rm,
                shift,
                setflags,
                ..
            } => {
                let rm_val = RegisterBank::get_register(core, rm);
                let rn_val = RegisterBank::get_register(core, rn);

                let (shifted, carry) = rm_val.shift_c(shift, xpsr.carry_flag());
                let result = rn_val ^ shifted;

                Self::set_register(core, rd, result);
                if setflags {
                    Self::modify_apsr(core, |v| {
                        v.with_negative(result.get_bit(31))
                            .with_zero(result.is_zero_bit())
                            .with_carry(carry)
                        // APSR.V unchanged
                    });
                }
                ExecutionStepResult::NextInstruction
            }

            // [ARM-ARM] A7.7.37
            Instruction::InstructionSynchronizationBarrier { option } => {
                // See: [ARM-ARM] D6.7.29 for InstructionSynchronizationBarrier(option) definition
                // TODO: instructions after ISB must use up-to-data values of control registers
                //       currently this is easy as we don't support writing to any control register

                debug_assert_eq!(option, bsc::C_1111, "Reserved (unsupported) ISB option");
                // ISB is always 32-bit instruction
                // PC is `address of current_instruction + 4` (see: [ARM-ARM] B1.4.7)
                let next_instruction_address = RegisterBank::get_register(core, RegisterID::PC);
                ExecutionStepResult::ExecuteTimeBranch {
                    address: next_instruction_address,
                }
            }

            // [ARM-ARM] A7.7.38
            Instruction::IfThen { .. } => {
                Self::execute_if_then(iectx, &instr, xpsr);
                ExecutionStepResult::NextInstruction
            }

            // [ARM-ARM] A7.7.68
            Instruction::LogicalShiftLeft_Immediate {
                rd,
                rm,
                shift,
                setflags,
                ..
            } => {
                let rm_val = RegisterBank::get_register(core, rm);

                debug_assert!(shift.srtype == SRType::LSL);
                let (result, carry) = rm_val.shift_c(shift, xpsr.carry_flag());

                Self::set_register(core, rd, result);
                if setflags {
                    Self::modify_apsr(core, |v| {
                        v.with_negative(result.get_bit(31))
                            .with_zero(result.is_zero_bit())
                            .with_carry(carry)
                        // APSR.V unchanged
                    });
                }

                ExecutionStepResult::NextInstruction
            }

            // [ARM-ARM] A7.7.69
            Instruction::LogicalShiftLeft_Register {
                rd,
                rn,
                rm,
                setflags,
                ..
            } => {
                let rm_val = RegisterBank::get_register(core, rm);
                let rn_val = RegisterBank::get_register(core, rn);
                let shift_n = u8::from(bitstring_extract!(rm_val<7:0> | 8 bits));
                let shift = Shift {
                    srtype: SRType::LSL,
                    amount: shift_n,
                };

                let (result, carry) = rn_val.shift_c(shift, xpsr.carry_flag());

                Self::set_register(core, rd, result);
                if setflags {
                    Self::modify_apsr(core, |v| {
                        v.with_negative(result.get_bit(31))
                            .with_zero(result.is_zero_bit())
                            .with_carry(carry)
                        // APSR.V unchanged
                    });
                }

                ExecutionStepResult::NextInstruction
            }

            // [ARM-ARM] A7.7.70
            Instruction::LogicalShiftRight_Immediate {
                rd,
                rm,
                shift,
                setflags,
                ..
            } => {
                let rm_val = RegisterBank::get_register(core, rm);

                debug_assert!(shift.srtype == SRType::LSR);
                let (result, carry) = rm_val.shift_c(shift, xpsr.carry_flag());

                Self::set_register(core, rd, result);
                if setflags {
                    Self::modify_apsr(core, |v| {
                        v.with_negative(result.get_bit(31))
                            .with_zero(result.is_zero_bit())
                            .with_carry(carry)
                        // APSR.V unchanged
                    });
                }
                ExecutionStepResult::NextInstruction
            }

            // [ARM-ARM] A7.7.71
            Instruction::LogicalShiftRight_Register {
                rd,
                rm,
                rn,
                setflags,
                ..
            } => {
                let rm_val = RegisterBank::get_register(core, rm);
                let rn_val = RegisterBank::get_register(core, rn);
                let shift_n = u8::from(bitstring_extract!(rm_val<7:0> | 8 bits));
                let shift = Shift {
                    srtype: SRType::LSR,
                    amount: shift_n,
                };

                let (result, carry) = rn_val.shift_c(shift, xpsr.carry_flag());

                Self::set_register(core, rd, result);
                if setflags {
                    Self::modify_apsr(core, |v| {
                        v.with_negative(result.get_bit(31))
                            .with_zero(result.is_zero_bit())
                            .with_carry(carry)
                        // APSR.V unchanged
                    });
                }

                ExecutionStepResult::NextInstruction
            }

            // [ARM-ARM] A7.7.74
            Instruction::MultiplyAccumulate {
                rd,
                rn,
                rm,
                ra,
                setflags,
            } => {
                // [ARM-TRM] 3.3.1, how long does the instruction take to execute
                // XXX(cm4): This may need an update in "is_multicycle" predicate
                if !cfg!(feature = "soc-cc2652") && iectx.cycle_cntr < 1 {
                    // TODO: tests to check if we should trigger decode
                    ExecutionStepResult::Continue {
                        trigger_decode: true,
                        lsu_branch_expected: false,
                    }
                } else {
                    let operand1: i32 = RegisterBank::get_register(core, rn).into();
                    let operand2: i32 = RegisterBank::get_register(core, rm).into();
                    let addend: i32 = RegisterBank::get_register(core, ra).into();

                    let result = operand1.wrapping_mul(operand2).wrapping_add(addend);

                    debug_assert!(!setflags, "setflags for MLA is always false, see decode");

                    Self::set_register(core, rd, result.into());
                    ExecutionStepResult::NextInstruction
                }
            }

            // [ARM-ARM] A7.7.75
            Instruction::MultiplyAndSubtract { rd, rn, rm, ra } => {
                // [ARM-TRM] 3.3.1, how long does the instruction take to execute
                if !cfg!(feature = "soc-cc2652") && iectx.cycle_cntr < 1 {
                    // TODO: tests to check if we should trigger decode
                    ExecutionStepResult::Continue {
                        trigger_decode: true,
                        lsu_branch_expected: false,
                    }
                } else {
                    let operand1: i32 = RegisterBank::get_register(core, rn).into();
                    let operand2: i32 = RegisterBank::get_register(core, rm).into();
                    let addend: i32 = RegisterBank::get_register(core, ra).into();

                    let result = addend.wrapping_sub(operand1.wrapping_mul(operand2));

                    Self::set_register(core, rd, result.into());
                    ExecutionStepResult::NextInstruction
                }
            }

            // [ARM-ARM] A7.7.76
            Instruction::Move_Immediate {
                rd,
                imm32,
                setflags,
                carry,
                ..
            } => {
                debug_assert!(rd != RegisterID::PC);

                Self::set_register(core, rd, imm32);
                if setflags {
                    Self::modify_apsr(core, |v| {
                        v.with_negative(imm32.get_bit(31))
                            .with_zero(imm32.is_zero_bit())
                            .with_carry(carry)
                        // APSR.V unchanged
                    });
                }
                ExecutionStepResult::NextInstruction
            }

            // [ARM-ARM] A7.7.77
            Instruction::Move_Register { rd, rm, setflags } => {
                if rd == RegisterID::PC {
                    debug_assert!(!setflags);
                    let branch_kind = iectx.pipeline_step_pack.branch_kind;
                    let target_address = RegisterBank::get_register(core, rm);
                    if branch_kind.is_decode_time() {
                        let iectx = Self::component_to_member_mut(core)
                            .get_active_instruction_execution_context();
                        iectx.mark_register_clean(RegisterID::PC);
                        ExecutionStepResult::DecodeTimeBranch {
                            address: target_address,
                            was_speculative: branch_kind.is_speculative(),
                        }
                    } else {
                        debug_assert!(branch_kind.is_execute_time());
                        // Address is directly provided from register
                        // Self::alu_write_pc(core, target_address)
                        Self::branch_write_pc(core, target_address)
                    }
                } else {
                    let result = RegisterBank::get_register(core, rm);
                    Self::set_register(core, rd, result);
                    // It is same as above -- no ALU, can forward
                    // Decode::fast_forward_agu_register(
                    //     core,
                    //     #[cfg(feature = "cycle-debug-logger")]
                    //     ctx,
                    //     rd,
                    //     result,
                    // );
                    if setflags {
                        Self::modify_apsr(core, |v| {
                            v.with_negative(result.get_bit(31))
                                .with_zero(result.is_zero_bit())
                            // APSR.C unchanged
                            // APSR.V unchanged
                        });
                    }
                    ExecutionStepResult::NextInstruction
                }
            }

            // [ARM-ARM] A7.7.79
            Instruction::MoveTop { rd, imm16 } => {
                let mut rd_val = RegisterBank::get_register(core, rd);

                bitstring_substitute!(rd_val<31:16> = imm16);
                Self::set_register(core, rd, rd_val);

                ExecutionStepResult::NextInstruction
            }

            // [ARM-ARM] A7.7.82, T1
            // [ARM-ARM] B5.2.2, T1
            Instruction::MoveToRegisterFromSpecialRegister { rd, sysm } => {
                let mut rd_val = Word::from(0u32);
                match bitstring_extract!(sysm<7:3> | 5 bits) {
                    // xPSR accesses
                    bsc::C_0_0000 => {
                        if sysm.get_bit(0) {
                            // FIXME: Notes of [ARM-ARM] B5.2.2 state that IPSR should read zero (not in pseudocode)
                            let ipsr = xpsr.ipsr_as_word();
                            let ipsr_8_0 = bitstring_extract!(ipsr<8:0> | 9 bits);
                            bitstring_substitute!(rd_val<8:0> = ipsr_8_0);
                        }
                        if sysm.get_bit(1) {
                            // EPSR reads as zero
                            bitstring_substitute!(rd_val<26:24> = bsc::C_000);
                            bitstring_substitute!(rd_val<15:10> = bsc::C_00_0000);
                        }
                        if !sysm.get_bit(2) {
                            let apsr = xpsr.apsr_as_word();
                            let apsr_31_27 = bitstring_extract!(apsr<31:27> | 5 bits);
                            bitstring_substitute!(rd_val<31:27> = apsr_31_27);
                            // Not supported by CM, so don't implement it.
                            // New scope as clippy work around.
                            {
                                const _: () = assert!(!have_dsp_ext());
                            }
                        }
                    }
                    bsc::C_0_0001 => {
                        if RegisterBank::current_mode_is_privileged(core) {
                            match bitstring_extract!(sysm<2:0> | 3 bits) {
                                bsc::C_000 => {
                                    rd_val =
                                        RegisterBank::get_stack_pointer(core, StackPointer::Main);
                                }
                                bsc::C_001 => {
                                    rd_val = RegisterBank::get_stack_pointer(
                                        core,
                                        StackPointer::Process,
                                    );
                                }
                                _ => unreachable!(
                                    "Unexpected SYSm encoding and unpredictable behaviour"
                                ),
                            }
                        }
                    }
                    bsc::C_0_0010 => match bitstring_extract!(sysm<2:0> | 3 bits) {
                        bsc::C_000 => {
                            let primask = if RegisterBank::current_mode_is_privileged(core) {
                                RegisterBank::get_primask(core).primask()
                            } else {
                                false
                            };
                            rd_val = rd_val.with_bit_set(0, primask);
                        }
                        bsc::C_001 | bsc::C_010 => {
                            let basepri = if RegisterBank::current_mode_is_privileged(core) {
                                RegisterBank::get_basepri(core).basepri()
                            } else {
                                bsc::C_0000_0000
                            };
                            bitstring_substitute!(rd_val<7:0> = basepri);
                        }
                        bsc::C_011 => {
                            let faultmask = if RegisterBank::current_mode_is_privileged(core) {
                                RegisterBank::get_faultmask(core).faultmask()
                            } else {
                                false
                            };
                            rd_val = rd_val.with_bit_set(0, faultmask);
                        }
                        bsc::C_100 => {
                            if have_fp_ext() {
                                unimplemented!("FP not supported by CM");
                            } else {
                                let control = RegisterBank::get_control(core).into();
                                let control_1_0 = bitstring_extract!(control<1:0> | 2 bits);
                                bitstring_substitute!(rd_val<1:0> = control_1_0);
                            }
                        }
                        _ => {
                            unreachable!("Unexpected SYSm encoding and unpredictable behaviour");
                        }
                    },
                    _ => unreachable!("Unexpected SYSm encoding and unpredictable behaviour"),
                }

                Self::set_register(core, rd, rd_val);
                ExecutionStepResult::NextInstruction
            }

            // [ARM-ARM] A7.7.83, T1
            // [ARM-ARM] B5.2.3, T1
            Instruction::MoveToSpecialRegisterFromARMRegister { rn, sysm, mask } => {
                if iectx.cycle_cntr == 0 {
                    let mut extend_execution_by_one_cycle = false;
                    match bitstring_extract!(sysm<7:3> | 5 bits) {
                        bsc::C_0_0000 => {
                            if !sysm.get_bit(2) {
                                if mask.get_bit(0) {
                                    #[allow(clippy::if_not_else)]
                                    if !have_dsp_ext() {
                                        panic!(
                                            "unpredictable operation at address: {:?}, cause: DSP not supported, see [ARM-ARM] B5.2.3",
                                            iectx.instruction_address()
                                        );
                                    } else {
                                        unimplemented!("DSP is not supported by CM");
                                    }
                                }
                                if mask.get_bit(1) {
                                    let rn_val = RegisterBank::get_register(core, rn);
                                    Self::modify_apsr(core, |v| {
                                        let mut apsr = Word::from(v);
                                        let rn_31_27 = bitstring_extract!(rn_val<31:27> | 5 bits);
                                        bitstring_substitute!(apsr<31:27> = rn_31_27);
                                        XPSR::from(apsr)
                                    });
                                }
                            }
                        }
                        bsc::C_0_0001 => {
                            if RegisterBank::current_mode_is_privileged(core) {
                                match bitstring_extract!(sysm<2:0> | 3 bits) {
                                    bsc::C_000 => {
                                        let rn_val = RegisterBank::get_register(core, rn);
                                        RegisterBank::set_stack_pointer(
                                            core,
                                            StackPointer::Main,
                                            rn_val,
                                        );
                                    }
                                    bsc::C_001 => {
                                        let rn_val = RegisterBank::get_register(core, rn);
                                        RegisterBank::set_stack_pointer(
                                            core,
                                            StackPointer::Process,
                                            rn_val,
                                        );
                                    }
                                    _ => unreachable!(
                                        "Unexpected SYSm encoding and unpredictable behaviour"
                                    ),
                                }
                            }
                        }
                        bsc::C_0_0010 => {
                            match bitstring_extract!(sysm<2:0> | 3 bits) {
                                bsc::C_000 => {
                                    if RegisterBank::current_mode_is_privileged(core) {
                                        let rn_val = RegisterBank::get_register(core, rn);
                                        let old_primask = RegisterBank::get_primask(core);
                                        let primask = old_primask.with_primask(rn_val.get_bit(0));
                                        RegisterBank::set_primask(core, ctx, primask);

                                        // If value which is written to PRIMASK
                                        // increases stored priority, MSR should last 2 cycles
                                        // - verified by run experiments and counterexample hasn't been found.
                                        if old_primask.value_increases_priority(rn_val.get_bit(0)) {
                                            extend_execution_by_one_cycle = true;
                                        }
                                    }
                                }
                                bsc::C_001 => {
                                    if RegisterBank::current_mode_is_privileged(core) {
                                        let rn_val = RegisterBank::get_register(core, rn);
                                        let rn_val_7_0 = bitstring_extract!(rn_val<7:0> | 8 bits);
                                        let old_basepri = RegisterBank::get_basepri(core);
                                        let basepri =
                                            BasePriorityMaskRegister::with_basepri(rn_val_7_0);
                                        RegisterBank::set_basepri(core, ctx, basepri);

                                        // If value which is written to BASEPRI
                                        // increases stored priority, MSR should last 2 cycles
                                        // - verified by run experiments and counterexample hasn't been found.
                                        if old_basepri.value_increases_priority(rn_val_7_0) {
                                            extend_execution_by_one_cycle = true;
                                        }
                                    }
                                }
                                bsc::C_010 => {
                                    let rn_val = RegisterBank::get_register(core, rn);
                                    let rn_val_7_0 = bitstring_extract!(rn_val<7:0> | 8 bits);
                                    let basepri = RegisterBank::get_basepri(core);

                                    if RegisterBank::current_mode_is_privileged(core)
                                        && basepri.value_increases_priority(rn_val_7_0)
                                    {
                                        let basepri =
                                            BasePriorityMaskRegister::with_basepri(rn_val_7_0);
                                        RegisterBank::set_basepri(core, ctx, basepri);

                                        // MSR which successfully writes to BASEPRI_MAX
                                        // lasts 2 cycles - verified by run experiments
                                        // and counterexample hasn't been found.
                                        extend_execution_by_one_cycle = true;
                                    }
                                }
                                bsc::C_011 => {
                                    // In pseudocode of this instruction ([ARM-ARM B5.2.3]), write to
                                    // faultmask can happen if ExecutionPriority() > -1. It cannot be
                                    // checked here, because it requires communication with nvic. But
                                    // negation of this condition is faster and easier to check here, because
                                    // it can only be satisfied if core is handling one of the following
                                    // exceptions: Reset, NMI or Hard Fault. It is so, because only these
                                    // 3 exceptions have priority <= -1.
                                    // One important thing to notice is that ExecutionPriority() == -1
                                    // if faultmask is set to 1. But experiments have shown that
                                    // it doesn't influence writing to faultmask.
                                    let too_high_execution_priority =
                                        if xpsr.current_mode() == ExecutionMode::Handler {
                                            let exc_id = InterruptId::try_from_exception_number(
                                                xpsr.get_exception_number() as usize,
                                            )
                                            .unwrap();
                                            exc_id == InterruptId::NMI
                                                || exc_id == InterruptId::Reset
                                                || exc_id == InterruptId::HardFault
                                        } else {
                                            false
                                        };
                                    if RegisterBank::current_mode_is_privileged(core)
                                        && !too_high_execution_priority
                                    {
                                        let rn_val = RegisterBank::get_register(core, rn);
                                        let old_faultmask = RegisterBank::get_faultmask(core);
                                        let faultmask =
                                            old_faultmask.with_faultmask(rn_val.get_bit(0));
                                        RegisterBank::set_faultmask(core, ctx, faultmask);

                                        // If value which is written to FAULTMASK
                                        // increases stored priority, MSR should last 2 cycles
                                        // - verified by run experiments and counterexample hasn't been found.
                                        if old_faultmask.value_increases_priority(rn_val.get_bit(0))
                                        {
                                            extend_execution_by_one_cycle = true;
                                        }
                                    }
                                }
                                bsc::C_100 => {
                                    if RegisterBank::current_mode_is_privileged(core) {
                                        let rn_val = RegisterBank::get_register(core, rn);
                                        let mut control = RegisterBank::get_control(core)
                                            .with_unprivileged(rn_val.get_bit(0));
                                        if xpsr.current_mode() == ExecutionMode::Thread {
                                            control = control.with_stack_pointer_selector(
                                                rn_val
                                                    .get_bit(1)
                                                    .ife(StackPointer::Process, StackPointer::Main),
                                            );
                                        }
                                        // Not supported by CherryMote, so don't implement it.
                                        // New scope as clippy work around.
                                        {
                                            const _: () = assert!(!have_fp_ext());
                                        }
                                        RegisterBank::set_control(core, control);
                                    }
                                }
                                _ => {
                                    unreachable!(
                                        "Unexpected SYSm encoding and unpredictable behaviour"
                                    );
                                }
                            }
                        }
                        _ => unreachable!("Unexpected SYSm encoding and unpredictable behaviour"),
                    }
                    if extend_execution_by_one_cycle {
                        ExecutionStepResult::Continue {
                            trigger_decode: true,
                            lsu_branch_expected: false,
                        }
                    } else {
                        ExecutionStepResult::NextInstruction
                    }
                } else {
                    ExecutionStepResult::NextInstruction
                }
            }

            // [ARM-ARM] A7.7.84
            Instruction::Multiply {
                rd,
                rn,
                rm,
                setflags,
                ..
            } => {
                let operand1 = RegisterBank::get_register(core, rn).sint(); // UInt(R[n]) produces the same final results
                let operand2 = RegisterBank::get_register(core, rm).sint(); // UInt(R[m]) produces the same final results
                let result = Word::from(operand1.wrapping_mul(operand2));

                Self::set_register(core, rd, result);
                if setflags {
                    Self::modify_apsr(core, |v| {
                        v.with_negative(result.get_bit(31))
                            .with_zero(result.is_zero_bit())
                        // APSR.C unchanged
                        // APSR.V unchanged
                    });
                }
                ExecutionStepResult::NextInstruction
            }

            // [ARM-ARM] A7.7.85
            Instruction::BitwiseNot_Immediate {
                rd,
                imm32,
                setflags,
                carry,
            } => {
                let result = !imm32;
                Self::set_register(core, rd, result);
                if setflags {
                    Self::modify_apsr(core, |v| {
                        v.with_negative(result.get_bit(31))
                            .with_zero(result.is_zero_bit())
                            .with_carry(carry)
                        // APSR.V unchanged
                    });
                }
                ExecutionStepResult::NextInstruction
            }

            // [ARM-ARM] A7.7.86
            Instruction::BitwiseNot_Register {
                rd,
                rm,
                shift,
                setflags,
                ..
            } => {
                let rm_val = RegisterBank::get_register(core, rm);

                let (shifted, carry) = rm_val.shift_c(shift, xpsr.carry_flag());
                let result = !shifted;

                Self::set_register(core, rd, result);
                if setflags {
                    Self::modify_apsr(core, |v| {
                        v.with_negative(result.get_bit(31))
                            .with_zero(result.is_zero_bit())
                            .with_carry(carry)
                        // APSR.V unchanged
                    });
                }
                ExecutionStepResult::NextInstruction
            }

            // [ARM-ARM] A7.7.88
            Instruction::NoOperation => ExecutionStepResult::NextInstruction,

            // [ARM-ARM] A7.7.89
            Instruction::LogicalOrNot_Immediate {
                rd,
                rn,
                imm32,
                setflags,
                carry,
            } => {
                let rn_val = RegisterBank::get_register(core, rn);

                let result = rn_val | !imm32;

                Self::set_register(core, rd, result);
                if setflags {
                    Self::modify_apsr(core, |v| {
                        v.with_negative(result.get_bit(31))
                            .with_zero(result.is_zero_bit())
                            .with_carry(carry)
                        // APSR.V unchanged
                    });
                }
                ExecutionStepResult::NextInstruction
            }

            // [ARM-ARM] A7.7.90
            Instruction::LogicalOrNot_Register {
                rd,
                rn,
                rm,
                shift,
                setflags,
            } => {
                let rm_val = RegisterBank::get_register(core, rm);
                let rn_val = RegisterBank::get_register(core, rn);

                let (shifted, carry) = rm_val.shift_c(shift, xpsr.carry_flag());
                let result = rn_val | shifted.not();

                Self::set_register(core, rd, result);
                if setflags {
                    Self::modify_apsr(core, |v| {
                        v.with_negative(result.get_bit(31))
                            .with_zero(result.is_zero_bit())
                            .with_carry(carry)
                        // APSR.V unchanged
                    });
                }
                ExecutionStepResult::NextInstruction
            }

            // [ARM-ARM] A7.7.91
            Instruction::LogicalOr_Immediate {
                rd,
                rn,
                imm32,
                setflags,
                carry,
            } => {
                let rn_val = RegisterBank::get_register(core, rn);

                let result = rn_val | imm32;

                Self::set_register(core, rd, result);
                if setflags {
                    Self::modify_apsr(core, |v| {
                        v.with_negative(result.get_bit(31))
                            .with_zero(result.is_zero_bit())
                            .with_carry(carry)
                        // APSR.V unchanged
                    });
                }
                ExecutionStepResult::NextInstruction
            }

            // [ARM-ARM] A7.7.92
            Instruction::LogicalOr_Register {
                rd,
                rn,
                rm,
                shift,
                setflags,
                ..
            } => {
                debug_assert!(rd != RegisterID::PC);
                debug_assert!(rd != RegisterID::SP);

                let rm_val = RegisterBank::get_register(core, rm);
                let rn_val = RegisterBank::get_register(core, rn);

                let (shifted, carry) = rm_val.shift_c(shift, xpsr.carry_flag());
                let result = rn_val | shifted;

                Self::set_register(core, rd, result);
                if setflags {
                    Self::modify_apsr(core, |v| {
                        v.with_negative(result.get_bit(31))
                            .with_zero(result.is_zero_bit())
                            .with_carry(carry)
                        // APSR.V unchanged
                    });
                }
                ExecutionStepResult::NextInstruction
            }

            // [ARM-ARM] A7.7.99 - POP is not a real instruction.
            //  (see: `cmemu-lib/src/component/core/instruction.rs` ctrl+f "A7.7.99")
            //
            //  It is decoded to one of the following instructions:
            //   * [ARM-ARM] A7.7.41 `LoadMultiple`
            //   * [ARM-ARM] A7.7.43 `LoadRegister_Immediate`

            // [ARM-ARM] A7.7.101 - PUSH is not a real instruction.
            //  (see: `cmemu-lib/src/component/core/instruction.rs` ctrl+f "A7.7.101")
            //
            //  It is decoded to one of the following instructions:
            //   * [ARM-ARM] A7.7.160 `StoreMultipleDecrementBefore`
            //   * [ARM-ARM] A7.7.161 `StoreRegister_Immediate`

            // [ARM-ARM] A7.7.112
            Instruction::ReverseBits { rd, rm } => {
                let rm_val = RegisterBank::get_register(core, rm);

                let result = u32::from(rm_val).reverse_bits().into();

                Self::set_register(core, rd, result);
                ExecutionStepResult::NextInstruction
            }

            // [ARM-ARM] A7.7.113
            Instruction::ByteReverseWord { rd, rm } => {
                let rm_val = RegisterBank::get_register(core, rm);

                let mut result = Word::from(0u32);
                bitstring_substitute!(result<31:24> = bitstring_extract!(rm_val<7:0> | 8 bits));
                bitstring_substitute!(result<23:16> = bitstring_extract!(rm_val<15:8> | 8 bits));
                bitstring_substitute!(result<15:8> = bitstring_extract!(rm_val<23:16> | 8 bits));
                bitstring_substitute!(result<7:0> = bitstring_extract!(rm_val<31:24> | 8 bits));

                Self::set_register(core, rd, result);
                ExecutionStepResult::NextInstruction
            }

            // [ARM-ARM] A7.7.114
            Instruction::ByteReversePackedHalfword { rd, rm } => {
                let rm_val = RegisterBank::get_register(core, rm);

                let mut result = Word::from(0u32);
                bitstring_substitute!(result<31:24> = bitstring_extract!(rm_val<23:16> | 8 bits));
                bitstring_substitute!(result<23:16> = bitstring_extract!(rm_val<31:24> | 8 bits));
                bitstring_substitute!(result<15:8> = bitstring_extract!(rm_val<7:0> | 8 bits));
                bitstring_substitute!(result<7:0> = bitstring_extract!(rm_val<15:8> | 8 bits));

                Self::set_register(core, rd, result);
                ExecutionStepResult::NextInstruction
            }

            // [ARM-ARM] A7.7.115
            Instruction::ByteReverseSignedHalfword { rd, rm } => {
                let rm_val = RegisterBank::get_register(core, rm);

                let mut result = Word::from(0u32);
                let prefix: Bitstring![24] = bitstring_extract!(rm_val<7:0> | 8 bits).sign_extend();
                bitstring_substitute!(result<31:8> = prefix);
                bitstring_substitute!(result<7:0> = bitstring_extract!(rm_val<15:8> | 8 bits));

                Self::set_register(core, rd, result);
                ExecutionStepResult::NextInstruction
            }

            // [ARM-ARM] A7.7.116
            Instruction::RotateRight_Immediate {
                rd,
                rm,
                shift,
                setflags,
            } => {
                let rm_val = RegisterBank::get_register(core, rm);

                debug_assert!(shift.srtype == SRType::ROR);
                let (result, carry) = rm_val.shift_c(shift, xpsr.carry_flag());

                Self::set_register(core, rd, result);
                if setflags {
                    Self::modify_apsr(core, |v| {
                        v.with_negative(result.get_bit(31))
                            .with_zero(result.is_zero_bit())
                            .with_carry(carry)
                        // APSR.V unchanged
                    });
                }
                ExecutionStepResult::NextInstruction
            }

            // [ARM-ARM] A7.7.117
            Instruction::RotateRight_Register {
                rd,
                rn,
                rm,
                setflags,
                ..
            } => {
                let rm_val = RegisterBank::get_register(core, rm);
                let rn_val = RegisterBank::get_register(core, rn);
                let shift_n = u8::from(bitstring_extract!(rm_val<7:0> | 8 bits));
                let shift = Shift {
                    srtype: SRType::ROR,
                    amount: shift_n,
                };

                let (result, carry) = rn_val.shift_c(shift, xpsr.carry_flag());

                Self::set_register(core, rd, result);
                if setflags {
                    Self::modify_apsr(core, |v| {
                        v.with_negative(result.get_bit(31))
                            .with_zero(result.is_zero_bit())
                            .with_carry(carry)
                        // APSR.V unchanged
                    });
                }
                ExecutionStepResult::NextInstruction
            }

            // [ARM-ARM] A7.7.118
            Instruction::RotateRightWithExtend { rd, rm, setflags } => {
                let rm_val = RegisterBank::get_register(core, rm);
                let shift = Shift {
                    srtype: SRType::RRX,
                    amount: 1,
                };

                let (result, carry) = rm_val.shift_c(shift, xpsr.carry_flag());
                Self::set_register(core, rd, result);
                if setflags {
                    Self::modify_apsr(core, |v| {
                        v.with_negative(result.get_bit(31))
                            .with_zero(result.is_zero_bit())
                            .with_carry(carry)
                        // APSR.V unchanged
                    });
                }
                ExecutionStepResult::NextInstruction
            }

            // [ARM-ARM] A7.7.119
            Instruction::ReverseSubtract_Immediate {
                rd,
                rn,
                imm32,
                setflags,
                ..
            } => {
                let rn_val = RegisterBank::get_register(core, rn);

                let (result, carry, overflow) = Word::add_with_carry(rn_val.not(), imm32, true);

                Self::set_register(core, rd, result);
                if setflags {
                    Self::modify_apsr(core, |v| {
                        v.with_negative(result.get_bit(31))
                            .with_zero(result.is_zero_bit())
                            .with_carry(carry)
                            .with_overflow(overflow)
                    });
                }
                ExecutionStepResult::NextInstruction
            }

            // [ARM-ARM] A7.7.120
            Instruction::ReverseSubtract_Register {
                rd,
                rn,
                rm,
                shift,
                setflags,
            } => {
                let rn_val = RegisterBank::get_register(core, rn);
                let rm_val = RegisterBank::get_register(core, rm);

                let shifted = rm_val.shift(shift, xpsr.carry_flag());
                let (result, carry, overflow) = Word::add_with_carry(rn_val.not(), shifted, true);

                Self::set_register(core, rd, result);
                if setflags {
                    Self::modify_apsr(core, |v| {
                        v.with_negative(result.get_bit(31))
                            .with_zero(result.is_zero_bit())
                            .with_carry(carry)
                            .with_overflow(overflow)
                    });
                }
                ExecutionStepResult::NextInstruction
            }

            // [ARM-ARM] A7.7.124
            Instruction::SubtractWithCarry_Immediate {
                rd,
                rn,
                imm32,
                setflags,
            } => {
                let rn_val = RegisterBank::get_register(core, rn);

                let (result, carry, overflow) =
                    Word::add_with_carry(rn_val, imm32.not(), xpsr.carry_flag());

                Self::set_register(core, rd, result);
                if setflags {
                    Self::modify_apsr(core, |v| {
                        v.with_negative(result.get_bit(31))
                            .with_zero(result.is_zero_bit())
                            .with_carry(carry)
                            .with_overflow(overflow)
                    });
                }
                ExecutionStepResult::NextInstruction
            }

            // [ARM-ARM] A7.7.125
            Instruction::SubtractWithCarry_Register {
                rd,
                rn,
                rm,
                shift,
                setflags,
                ..
            } => {
                let rn_val = RegisterBank::get_register(core, rn);
                let rm_val = RegisterBank::get_register(core, rm);

                let shifted = rm_val.shift(shift, xpsr.carry_flag());
                let (result, carry, overflow) =
                    Word::add_with_carry(rn_val, shifted.not(), xpsr.carry_flag());

                Self::set_register(core, rd, result);
                if setflags {
                    Self::modify_apsr(core, |v| {
                        v.with_negative(result.get_bit(31))
                            .with_zero(result.is_zero_bit())
                            .with_carry(carry)
                            .with_overflow(overflow)
                    });
                }
                ExecutionStepResult::NextInstruction
            }

            // [ARM-ARM] A7.7.126
            Instruction::SignedBitFieldExtract {
                rn,
                rd,
                lsbit,
                widthminus1,
            } => {
                let msbit = lsbit + widthminus1;
                if msbit <= 31 {
                    let rn_val = RegisterBank::get_register(core, rn);
                    let mask = mask_between_bits(msbit, lsbit);
                    let mask_extend = (0xFFFF_FFFF ^ mask_between_bits(widthminus1, 0))
                        * u32::from(rn_val.get_bit(msbit.into()));

                    let result = Word::from(((rn_val.uint() & mask) >> lsbit) | mask_extend);
                    Self::set_register(core, rd, result);
                    ExecutionStepResult::NextInstruction
                } else {
                    panic!(
                        "unpredictable operation at address: {:?}, cause: bit index overflow at sbfx",
                        iectx.instruction_address()
                    )
                }
            }

            // [ARM-ARM] A7.7.127
            Instruction::SignedDivide { rd, rn, rm } => {
                let cycle_cntr = iectx.cycle_cntr;
                let rn_val = RegisterBank::get_register(core, rn);
                let rm_val = RegisterBank::get_register(core, rm);
                let finish = sdiv_execution_result_writing_cycle(rn_val, rm_val);

                if finish == cycle_cntr {
                    let result = if rm_val.sint() == 0 {
                        // It is assumed that false will be returned.
                        // After implementation both values will be valid and
                        // const assert could be removed.
                        const _: () = assert!(!integer_zero_divide_trapping_enabled());
                        if integer_zero_divide_trapping_enabled() {
                            // exceptions haven't been implemented yet
                            unimplemented!("GenerateIntegerZeroDivide()");
                        } else {
                            Word::from(0u32)
                        }
                    } else if rn_val.sint() == (1 << 31) && rm_val.sint() == -1 {
                        // This is an edge case described in [ARM-ARM] A7.7.127 - Notes.
                        // This division is equal to 0x80000000 / 0xFFFFFFFF.
                        // Result of this should be 0x80000000 = rn_val.
                        rn_val
                    } else {
                        // In [ARM-ARM] division is done in the way that result
                        // of division is real, and it is rounded towards zero
                        // (section [ARM-ARM] D6.5.4 describes how it is done.
                        // Div for i32 rounds towards zero, so it is used instead.
                        // https://doc.rust-lang.org/std/primitive.i32.html#impl-Div%3Ci32%3E
                        Word::from(rn_val.sint() / rm_val.sint())
                    };

                    Self::set_register(core, rd, result);
                    ExecutionStepResult::NextInstruction
                } else {
                    ExecutionStepResult::Continue {
                        trigger_decode: true,
                        lsu_branch_expected: false,
                    }
                }
            }

            // [ARM-ARM] A7.7.129
            Instruction::SendEvent => unimplemented!(
                "Hint_SendEvent() [at address {:?}]",
                iectx.instruction_address()
            ),

            // [ARM-ARM] A7.7.138
            Instruction::SignedMultiplyAccumulateLong {
                rn,
                rm,
                rd_hi,
                rd_lo,
            } => {
                let cycle_cntr = iectx.cycle_cntr;
                let mut ret = ExecutionStepResult::Continue {
                    trigger_decode: true,
                    lsu_branch_expected: false,
                };

                if cycle_cntr == 0 {
                    let rn_sint = RegisterBank::get_register(core, rn).sint();
                    let rm_sint = RegisterBank::get_register(core, rm).sint();
                    let rd_hi_uint = RegisterBank::get_register(core, rd_hi).uint();
                    let rd_lo_uint = RegisterBank::get_register(core, rd_lo).uint();

                    #[allow(clippy::cast_possible_wrap)] // wrap is expected
                    let result = (i64::from(rn_sint) * i64::from(rm_sint)).wrapping_add(
                        ((u64::from(rd_hi_uint) << 32) | u64::from(rd_lo_uint)) as i64,
                    );
                    #[allow(clippy::cast_possible_truncation)] // mask enforces that value is 32 bit
                    let result_hi = Word::from(((result >> 32) & LOW_HALF_MASK_I64) as i32);
                    #[allow(clippy::cast_possible_truncation)] // mask enforces that value is 32 bit
                    let result_lo = Word::from((result & LOW_HALF_MASK_I64) as i32);
                    let (write_hi_cycle_no, write_lo_cycle_no) =
                        smlal_execution_result_writing_cycles(rn_sint, rm_sint);
                    debug_assert!(
                        cfg!(feature = "soc-cc2652") || write_hi_cycle_no != write_lo_cycle_no
                    );

                    Self::set_state(
                        core,
                        InstructionExecutionState::MultiplyLong(MultiplyLongExecutionState {
                            result_hi,
                            result_lo,
                            write_hi_cycle_no,
                            write_lo_cycle_no,
                        }),
                    );
                }
                {
                    let state = *Self::get_state(core).unwrap_multiply_long_state();

                    if cycle_cntr == state.write_lo_cycle_no {
                        let result_lo = state.result_lo;
                        Self::set_register(core, rd_lo, result_lo);
                    }
                    if cycle_cntr == state.write_hi_cycle_no {
                        let result_hi = state.result_hi;
                        Self::set_register(core, rd_hi, result_hi);
                        ret = ExecutionStepResult::NextInstruction;
                    }
                }

                ret
            }

            // [ARM-ARM] A7.7.149
            Instruction::SignedMultiplyLong {
                rn,
                rm,
                rd_hi,
                rd_lo,
            } => {
                let cycle_cntr = iectx.cycle_cntr;
                let mut ret = ExecutionStepResult::Continue {
                    trigger_decode: true,
                    lsu_branch_expected: false,
                };

                if cycle_cntr == 0 {
                    let rn_sint = RegisterBank::get_register(core, rn).sint();
                    let rm_sint = RegisterBank::get_register(core, rm).sint();

                    let result = i64::from(rn_sint) * i64::from(rm_sint);
                    #[allow(clippy::cast_possible_truncation)] // mask enforces that value is 32 bit
                    let result_hi = Word::from(((result >> 32) & LOW_HALF_MASK_I64) as i32);
                    #[allow(clippy::cast_possible_truncation)] // mask enforces that value is 32 bit
                    let result_lo = Word::from((result & LOW_HALF_MASK_I64) as i32);
                    let (write_hi_cycle_no, write_lo_cycle_no) =
                        smull_execution_result_writing_cycles(rn_sint, rm_sint);
                    debug_assert!(
                        cfg!(feature = "soc-cc2652") || write_hi_cycle_no != write_lo_cycle_no
                    );

                    Self::set_state(
                        core,
                        InstructionExecutionState::MultiplyLong(MultiplyLongExecutionState {
                            result_hi,
                            result_lo,
                            write_hi_cycle_no,
                            write_lo_cycle_no,
                        }),
                    );
                }
                {
                    let state = *Self::get_state(core).unwrap_multiply_long_state();

                    if cycle_cntr == state.write_lo_cycle_no {
                        let result_lo = state.result_lo;
                        Self::set_register(core, rd_lo, result_lo);
                    }
                    if cycle_cntr == state.write_hi_cycle_no {
                        let result_hi = state.result_hi;
                        Self::set_register(core, rd_hi, result_hi);
                        ret = ExecutionStepResult::NextInstruction;
                    }
                }

                ret
            }

            // [ARM-ARM] A7.7.152
            Instruction::SignedSaturate {
                rd,
                rn,
                saturate_to,
                shift,
            } => {
                let rn_val = RegisterBank::get_register(core, rn);

                let operand = rn_val.shift(shift, xpsr.carry_flag()); // APSR.C ignored
                let (result, sat) = operand.signed_sat_q_with_zero_extend(saturate_to);

                Self::set_register(core, rd, result);
                if sat {
                    Self::modify_apsr(core, |v| v.with_saturation(true));
                }

                ExecutionStepResult::NextInstruction
            }

            // [ARM-ARM] A7.7.172 - see A7.7.67

            // [ARM-ARM] A7.7.173 - see A7.7.67

            // [ARM-ARM] A7.7.174
            Instruction::Subtract_Immediate {
                rd,
                rn,
                imm32,
                setflags,
                ..
            } => {
                let rn_val = RegisterBank::get_register(core, rn);

                let (result, carry, overflow) = Word::add_with_carry(rn_val, imm32.not(), true);

                Self::set_register(core, rd, result);
                if setflags {
                    Self::modify_apsr(core, |v| {
                        v.with_negative(result.get_bit(31))
                            .with_zero(result.is_zero_bit())
                            .with_carry(carry)
                            .with_overflow(overflow)
                    });
                }

                ExecutionStepResult::NextInstruction
            }

            // [ARM-ARM] A7.7.175
            Instruction::Subtract_Register {
                rd,
                rn,
                rm,
                shift,
                setflags,
                ..
            } => {
                debug_assert!(rd != RegisterID::PC);
                let rm_val = RegisterBank::get_register(core, rm);
                let rn_val = RegisterBank::get_register(core, rn);

                let shifted = rm_val.shift(shift, xpsr.carry_flag());
                let (result, carry, overflow) = Word::add_with_carry(rn_val, shifted.not(), true);

                Self::set_register(core, rd, result);
                if setflags {
                    Self::modify_apsr(core, |v| {
                        v.with_negative(result.get_bit(31))
                            .with_zero(result.is_zero_bit())
                            .with_carry(carry)
                            .with_overflow(overflow)
                    });
                }
                ExecutionStepResult::NextInstruction
            }

            // [ARM-ARM] A7.7.176
            Instruction::Subtract_SPMinusImmediate {
                rd,
                imm32,
                setflags,
            } => {
                let sp_val = RegisterBank::get_register(core, RegisterID::SP);

                let (result, carry, overflow) = Word::add_with_carry(sp_val, imm32.not(), true);

                Self::set_register(core, rd, result);
                if setflags {
                    Self::modify_apsr(core, |v| {
                        v.with_negative(result.get_bit(31))
                            .with_zero(result.is_zero_bit())
                            .with_carry(carry)
                            .with_overflow(overflow)
                    });
                }

                ExecutionStepResult::NextInstruction
            }

            // [ARM-ARM] A7.7.177
            Instruction::Subtract_SPMinusRegister {
                rd,
                rm,
                shift,
                setflags,
            } => {
                debug_assert!(rd != RegisterID::PC);
                let rm_val = RegisterBank::get_register(core, rm);
                let sp_val = RegisterBank::get_register(core, RegisterID::SP);

                let shifted = rm_val.shift(shift, xpsr.carry_flag());
                let (result, carry, overflow) = Word::add_with_carry(sp_val, shifted.not(), true);

                Self::set_register(core, rd, result);
                if setflags {
                    Self::modify_apsr(core, |v| {
                        v.with_negative(result.get_bit(31))
                            .with_zero(result.is_zero_bit())
                            .with_carry(carry)
                            .with_overflow(overflow)
                    });
                }
                ExecutionStepResult::NextInstruction
            }

            // [ARM-ARM] A7.7.178
            Instruction::SupervisorCall { imm32 } => unimplemented!(
                "CallSupervisor() [imm32 = 0x{:x}, at address {:?}]",
                imm32,
                iectx.instruction_address()
            ),

            // [ARM-ARM] A7.7.182
            Instruction::SignedExtendByte { rd, rm, rotation } => {
                debug_assert!(rotation.srtype == SRType::ROR);
                let rm_val = RegisterBank::get_register(core, rm);
                let rotated = rm_val.shift(rotation, false);
                let result = bitstring_extract!(rotated<7:0> | 8 bits).sign_extend();

                Self::set_register(core, rd, result);
                ExecutionStepResult::NextInstruction
            }

            // [ARM-ARM] A7.7.184
            Instruction::SignedExtendHalfword { rd, rm, rotation } => {
                debug_assert!(rotation.srtype == SRType::ROR);
                let rm_val = RegisterBank::get_register(core, rm);
                let rotated = rm_val.shift(rotation, false);
                let result = bitstring_extract!(rotated<15:0> | 16 bits).sign_extend();

                Self::set_register(core, rd, result);
                ExecutionStepResult::NextInstruction
            }

            // [ARM-ARM] A7.7.185
            Instruction::TableBranch { is_tbh, .. } => {
                // The instruction is executed in multiple steps across many cycles
                // (including data load callback).
                let mut ret = ExecutionStepResult::Continue {
                    trigger_decode: true,
                    lsu_branch_expected: true,
                };

                if iectx.cycle_cntr == 0 {
                    // Step 1: Prepare memory request.
                    let address = RegisterBank::get_agu_result(core);
                    let size = if is_tbh { Size::Halfword } else { Size::Byte };
                    LSU::request_read(
                        core,
                        address,
                        size,
                        ReadDataCallback::WithDecodeFn(
                            |core, decode, data| {
                                // Step 3: Store loaded data for next cycle.
                                let this = Self::component_to_member_mut(core);
                                let ctx = this
                                    .main_slot
                                    .as_mut()
                                    .expect("No main_slot in TableBranch read_data function");
                                ctx.state = InstructionExecutionState::TableBranchOffset {
                                    branch_offset: decode(data),
                                    dest_addr: None,
                                };
                            },
                            DataBus::zero_extend_into_word,
                        ),
                    );

                    // TODO: Most likely the fetch is suppressed. Create a test.
                    // See: [ARM-TRM-G] 15.3, Notes under table 15-3.
                    Fetch::disable_fetch(core);

                    Self::set_state(
                        core,
                        InstructionExecutionState::SingleLoadStore(
                            SingleLoadStoreExecutionState {},
                        ),
                    );
                } else {
                    let pc = RegisterBank::get_register(core, RegisterID::PC);
                    match Self::get_state(core) {
                        InstructionExecutionState::SingleLoadStore(_state) => {
                            // Step 2: Wait for acceptance/response.
                        }
                        InstructionExecutionState::TableBranchOffset {
                            branch_offset,
                            dest_addr,
                        } if dest_addr.is_none() => {
                            // Step 3: Shift/ALU
                            let branch_offset = *branch_offset;
                            let halfwords = branch_offset.uint();
                            *dest_addr = Some(pc + 2 * halfwords);
                        }
                        InstructionExecutionState::TableBranchOffset {
                            dest_addr: Some(dest_addr),
                            ..
                        } => {
                            // [ARM-TRM-G] 18.2 Processor instruction timings
                            // States that TBB/TBH take 4 + P cycles, whereas LDR to PC is 2 + P
                            // and B is 1 + P. It also mentions some "shift" which may be an ALU cycle.
                            // Step 4: Branch with given offset.
                            let dest_addr = *dest_addr;
                            // TODO: use alu_write_PC instead
                            ret = Self::branch_write_pc(core, dest_addr);
                        }
                        state => {
                            unreachable!("Unexpected state in TableBranch execution: {:?}", state);
                        }
                    }
                }

                ret
            }

            // [ARM-ARM] A7.7.186
            Instruction::TestEquivalence_Immediate { rn, imm32, carry } => {
                let rn_val = RegisterBank::get_register(core, rn);
                let result = rn_val ^ imm32;

                Self::modify_apsr(core, |v| {
                    v.with_negative(result.get_bit(31))
                        .with_zero(result.is_zero_bit())
                        .with_carry(carry)
                    // APSR.V unchanged
                });
                ExecutionStepResult::NextInstruction
            }

            // [ARM-ARM] A7.7.187
            Instruction::TestEquivalence_Register { rn, rm, shift } => {
                let rn_val = RegisterBank::get_register(core, rn);
                let rm_val = RegisterBank::get_register(core, rm);

                let (shifted, carry) = rm_val.shift_c(shift, xpsr.carry_flag());
                let result = rn_val ^ shifted;

                Self::modify_apsr(core, |v| {
                    v.with_negative(result.get_bit(31))
                        .with_zero(result.is_zero_bit())
                        .with_carry(carry)
                    // APSR.V unchanged
                });
                ExecutionStepResult::NextInstruction
            }

            // [ARM-ARM] A7.7.188
            Instruction::Test_Immediate { rn, imm32, carry } => {
                let rn_val = RegisterBank::get_register(core, rn);
                let result = rn_val & imm32;

                Self::modify_apsr(core, |v| {
                    v.with_negative(result.get_bit(31))
                        .with_zero(result.is_zero_bit())
                        .with_carry(carry)
                    // APSR.V unchanged
                });
                ExecutionStepResult::NextInstruction
            }

            // [ARM-ARM] A7.7.189
            Instruction::Test_Register { rn, rm, shift } => {
                let rn_val = RegisterBank::get_register(core, rn);
                let rm_val = RegisterBank::get_register(core, rm);

                let (shifted, carry) = rm_val.shift_c(shift, xpsr.carry_flag());
                let result = rn_val & shifted;

                Self::modify_apsr(core, |v| {
                    v.with_negative(result.get_bit(31))
                        .with_zero(result.is_zero_bit())
                        .with_carry(carry)
                    // APSR.V unchanged
                });
                ExecutionStepResult::NextInstruction
            }

            // [ARM-ARM] A7.7.193
            Instruction::UnsignedBitFieldExtract {
                rn,
                rd,
                lsbit,
                widthminus1,
            } => {
                let msbit = lsbit + widthminus1;
                if msbit <= 31 {
                    let rn_val = RegisterBank::get_register(core, rn).uint();
                    let mask = mask_between_bits(msbit, lsbit);

                    let result = Word::from((rn_val & mask) >> lsbit);
                    Self::set_register(core, rd, result);
                    ExecutionStepResult::NextInstruction
                } else {
                    panic!(
                        "unpredictable operation at address: {:?}, cause: bit index overflow at ubfx",
                        iectx.instruction_address()
                    )
                }
            }

            // [ARM-ARM] A7.7.194
            Instruction::PermanentlyUndefined { imm32 } => panic!(
                "permanently undefined operation at address: {:?}, with imm: {:?}",
                iectx.instruction_address(),
                imm32
            ),

            // [ARM-ARM] A7.7.195
            Instruction::UnsignedDivide { rd, rn, rm } => {
                let cycle_cntr = iectx.cycle_cntr;
                let rn_val = RegisterBank::get_register(core, rn);
                let rm_val = RegisterBank::get_register(core, rm);
                let finish = udiv_execution_result_writing_cycle(rn_val, rm_val);

                if finish == cycle_cntr {
                    let result = if rm_val.uint() == 0 {
                        // It is assumed that false will be returned.
                        // After implementation both values will be valid and
                        // const assert could be removed.
                        const _: () = assert!(!integer_zero_divide_trapping_enabled());
                        if integer_zero_divide_trapping_enabled() {
                            // exceptions haven't been implemented yet
                            unimplemented!("GenerateIntegerZeroDivide()");
                        } else {
                            Word::from(0u32)
                        }
                    } else {
                        // In [ARM-ARM] division is done in the way that result
                        // of division is real, and it is rounded towards zero
                        // (section [ARM-ARM] D6.5.4 describes how it is done.
                        // Div for u32 rounds towards zero, so it is used instead.
                        // https://doc.rust-lang.org/std/primitive.u32.html#impl-Div%3Cu32%3E
                        Word::from(rn_val.uint() / rm_val.uint())
                    };

                    Self::set_register(core, rd, result);
                    ExecutionStepResult::NextInstruction
                } else {
                    ExecutionStepResult::Continue {
                        trigger_decode: true,
                        lsu_branch_expected: false,
                    }
                }
            }

            // [ARM-ARM] A7.7.203
            Instruction::UnsignedMultiplyAccumulateLong {
                rn,
                rm,
                rd_hi,
                rd_lo,
            } => {
                let cycle_cntr = iectx.cycle_cntr;
                let mut ret = ExecutionStepResult::Continue {
                    trigger_decode: true,
                    lsu_branch_expected: false,
                };

                if cycle_cntr == 0 {
                    let rn_uint = RegisterBank::get_register(core, rn).uint();
                    let rm_uint = RegisterBank::get_register(core, rm).uint();
                    let rd_hi_uint = RegisterBank::get_register(core, rd_hi).uint();
                    let rd_lo_uint = RegisterBank::get_register(core, rd_lo).uint();

                    let result = (u64::from(rn_uint) * u64::from(rm_uint))
                        .wrapping_add((u64::from(rd_hi_uint) << 32) | u64::from(rd_lo_uint));
                    #[allow(clippy::cast_possible_truncation)] // mask enforces that value is 32 bit
                    let result_hi = Word::from(((result >> 32) & LOW_HALF_MASK_U64) as u32);
                    #[allow(clippy::cast_possible_truncation)] // mask enforces that value is 32 bit
                    let result_lo = Word::from((result & LOW_HALF_MASK_U64) as u32);
                    let (write_hi_cycle_no, write_lo_cycle_no) =
                        umlal_execution_result_writing_cycles(rn_uint, rm_uint);
                    debug_assert!(
                        cfg!(feature = "soc-cc2652") || write_hi_cycle_no != write_lo_cycle_no
                    );

                    Self::set_state(
                        core,
                        InstructionExecutionState::MultiplyLong(MultiplyLongExecutionState {
                            result_hi,
                            result_lo,
                            write_hi_cycle_no,
                            write_lo_cycle_no,
                        }),
                    );
                }
                {
                    let state = *Self::get_state(core).unwrap_multiply_long_state();

                    if cycle_cntr == state.write_lo_cycle_no {
                        let result_lo = state.result_lo;
                        Self::set_register(core, rd_lo, result_lo);
                    }
                    if cycle_cntr == state.write_hi_cycle_no {
                        let result_hi = state.result_hi;
                        Self::set_register(core, rd_hi, result_hi);
                        ret = ExecutionStepResult::NextInstruction;
                    }
                }

                ret
            }

            // [ARM-ARM] A7.7.204
            Instruction::UnsignedMultiplyLong {
                rn,
                rm,
                rd_hi,
                rd_lo,
            } => {
                let cycle_cntr = iectx.cycle_cntr;
                let mut ret = ExecutionStepResult::Continue {
                    trigger_decode: true,
                    lsu_branch_expected: false,
                };

                if cycle_cntr == 0 {
                    let rn_uint = RegisterBank::get_register(core, rn).uint();
                    let rm_uint = RegisterBank::get_register(core, rm).uint();

                    let result = u64::from(rn_uint) * u64::from(rm_uint);
                    #[allow(clippy::cast_possible_truncation)] // mask enforces that value is 32 bit
                    let result_hi = Word::from(((result >> 32) & LOW_HALF_MASK_U64) as u32);
                    #[allow(clippy::cast_possible_truncation)] // mask enforces that value is 32 bit
                    let result_lo = Word::from((result & LOW_HALF_MASK_U64) as u32);
                    let (write_hi_cycle_no, write_lo_cycle_no) =
                        umull_execution_result_writing_cycles(rn_uint, rm_uint);
                    debug_assert!(
                        cfg!(feature = "soc-cc2652") || write_hi_cycle_no != write_lo_cycle_no
                    );

                    Self::set_state(
                        core,
                        InstructionExecutionState::MultiplyLong(MultiplyLongExecutionState {
                            result_hi,
                            result_lo,
                            write_hi_cycle_no,
                            write_lo_cycle_no,
                        }),
                    );
                }
                {
                    let state = *Self::get_state(core).unwrap_multiply_long_state();

                    if cycle_cntr == state.write_lo_cycle_no {
                        let result_lo = state.result_lo;
                        Self::set_register(core, rd_lo, result_lo);
                    }
                    if cycle_cntr == state.write_hi_cycle_no {
                        let result_hi = state.result_hi;
                        Self::set_register(core, rd_hi, result_hi);
                        ret = ExecutionStepResult::NextInstruction;
                    }
                }

                ret
            }

            // [ARM-ARM] A7.7.213
            Instruction::UnsignedSaturate {
                rd,
                rn,
                saturate_to,
                shift,
            } => {
                let rn_val = RegisterBank::get_register(core, rn);

                let operand = rn_val.shift(shift, xpsr.carry_flag()); // APSR.C ignored
                let (result, sat) = operand.unsigned_sat_q_with_zero_extend(saturate_to);

                Self::set_register(core, rd, result);
                if sat {
                    Self::modify_apsr(core, |v| v.with_saturation(true));
                }

                ExecutionStepResult::NextInstruction
            }

            // [ARM-ARM] A7.7.221
            Instruction::UnsignedExtendByte { rd, rm, rotation } => {
                debug_assert!(rotation.srtype == SRType::ROR);
                let rm_val = RegisterBank::get_register(core, rm);
                let rotated = rm_val.shift(rotation, false);
                let result = bitstring_extract!(rotated<7:0> | 8 bits).zero_extend();

                Self::set_register(core, rd, result);
                ExecutionStepResult::NextInstruction
            }

            // [ARM-ARM] A7.7.223
            Instruction::UnsignedExtendHalfword { rd, rm, rotation } => {
                debug_assert!(rotation.srtype == SRType::ROR);
                let rm_val = RegisterBank::get_register(core, rm);
                let rotated = rm_val.shift(rotation, false);
                let result = bitstring_extract!(rotated<15:0> | 16 bits).zero_extend();

                Self::set_register(core, rd, result);

                ExecutionStepResult::NextInstruction
            }

            // [ARM-ARM] A7.7.261
            Instruction::WaitForEvent => unimplemented!(
                "EventRegistered() / ClearEventRegister() / WaitForEvent() [at address {:?}]",
                iectx.instruction_address()
            ),

            // [ARM-ARM] A7.7.262
            Instruction::WaitForInterrupt => {
                debug_assert!(!iectx.has_folded_instruction());
                // FIXME: this implementation is purely functional
                // TODO: check if this instruction flushes the pipeline!
                info!(
                    "wfi at cycle {} @{:?}",
                    ctx.cycle_no(),
                    iectx.instruction_address()
                );

                if iectx.cycle_cntr == 0 {
                    debug!("Preparing for sleep.");
                    Fetch::disable_fetch(core);

                    ExecutionStepResult::PreSleep
                } else if core.can_be_disabled_now() {
                    warn!("Really going to sleep");
                    NVICProxy.start_sleep(ctx);
                    ExecutionStepResult::Sleep
                } else {
                    warn!("Waiting for Core to be disableable");
                    ExecutionStepResult::PreSleep
                }
            }
            Instruction::LoadMultiple { .. }
            | Instruction::LoadMultipleDecrementBefore { .. }
            | Instruction::LoadRegisterDual_Literal { .. }
            | Instruction::StoreMultiple { .. }
            | Instruction::StoreMultipleDecrementBefore { .. }
            | Instruction::LoadRegisterDual_Immediate { .. }
            | Instruction::StoreRegisterDual_Immediate { .. }
            | Instruction::LoadRegister_Literal { .. }
            | Instruction::LoadRegisterByteUnprivileged { .. }
            | Instruction::LoadRegisterByte_Literal { .. }
            | Instruction::LoadRegisterExclusive { .. }
            | Instruction::LoadRegisterHalfword_Literal { .. }
            | Instruction::LoadRegisterHalfwordUnprivileged { .. }
            | Instruction::LoadRegisterSignedByte_Literal { .. }
            | Instruction::LoadRegisterSignedHalfword_Literal { .. }
            | Instruction::LoadRegisterSignedByteUnprivileged { .. }
            | Instruction::LoadRegisterSignedHalfwordUnprivileged { .. }
            | Instruction::LoadRegisterUnprivileged { .. }
            | Instruction::LoadRegisterExclusiveByte { .. }
            | Instruction::LoadRegisterExclusiveHalfword { .. }
            | Instruction::StoreRegisterByteUnprivileged { .. }
            | Instruction::StoreRegisterExclusive { .. }
            | Instruction::StoreRegisterExclusiveByte { .. }
            | Instruction::StoreRegisterExclusiveHalfword { .. }
            | Instruction::StoreRegisterHalfwordUnprivileged { .. }
            | Instruction::StoreRegisterUnprivileged { .. }
            | Instruction::LoadRegister_Immediate { .. }
            | Instruction::LoadRegister_Register { .. }
            | Instruction::LoadRegisterByte_Immediate { .. }
            | Instruction::LoadRegisterByte_Register { .. }
            | Instruction::LoadRegisterHalfword_Immediate { .. }
            | Instruction::LoadRegisterHalfword_Register { .. }
            | Instruction::LoadRegisterSignedByte_Immediate { .. }
            | Instruction::LoadRegisterSignedByte_Register { .. }
            | Instruction::LoadRegisterSignedHalfword_Immediate { .. }
            | Instruction::LoadRegisterSignedHalfword_Register { .. }
            | Instruction::StoreRegister_Immediate { .. }
            | Instruction::StoreRegister_Register { .. }
            | Instruction::StoreRegisterByte_Immediate { .. }
            | Instruction::StoreRegisterByte_Register { .. }
            | Instruction::StoreRegisterHalfword_Immediate { .. }
            | Instruction::StoreRegisterHalfword_Register { .. } => {
                Self::execute_memory_instruction_step(core, ctx, instr)
            }
        }
    }

    // Always runs only one cycle
    pub(super) fn execute_folded_instruction(core: &mut CoreComponent, ctx: &mut Context) {
        let this = Self::component_to_member_mut(core);
        let iectx = this.get_active_instruction_execution_context();

        let folded_instr = if let Some(instr) = iectx.folded_instruction() {
            instr.clone()
        } else {
            return;
        };

        #[cfg(debug_assertions)]
        {
            assert!(!iectx.folded_instr_executed);
            iectx.folded_instr_executed = true;
        }

        let folded_instr_addr = iectx.folded_instruction_address();
        #[cfg(feature = "cycle-debug-logger")]
        CycleDebugLoggerProxy::new().on_folded_execute(ctx, folded_instr_addr);
        trace!(
            "Execute (folded) [{addr:?}]: {instr} | {instr:?}",
            addr = folded_instr_addr,
            instr = folded_instr,
        );

        match folded_instr {
            // [ARM-ARM] A7.7.38
            Instruction::IfThen { .. } => {
                let xpsr = iectx.next_xpsr; // updated by the main instruction
                Self::execute_if_then(iectx, &folded_instr, xpsr);
            }

            // [ARM-TRM] 3.3.1
            _ => panic!("Only IT instruction is allowed to be folded"),
        }

        // see: `Execute::handle_dwt_counters(...)` for more information
        DWTProxy.increment_fold_counter(ctx);
    }

    // Helper method to eliminate code duplication
    fn execute_if_then(
        iectx: &mut InstructionExecutionContext,
        it_instr: &Instruction,
        xpsr: XPSR,
    ) {
        // [ARM-ARM] A7.7.38
        debug_assert!(
            !xpsr.in_it_block(),
            "cannot execute IT instruction inside IT block"
        );
        iectx.modify_epsr(|v| v.with_itstate(ItState::from_instruction(it_instr)));
    }

    // Represents pseudocode from docs: `R[d] = result`
    fn set_register(core: &mut CoreComponent, register: RegisterID, value: Word) {
        debug_assert_ne!(
            register,
            RegisterID::PC,
            "Use `*_write_pc` to write to PC register."
        );

        let this = Self::component_to_member_mut(core);
        let ctx = this.get_active_instruction_execution_context();

        ctx.mark_register_clean(register);
        RegisterBank::set_register(core, register, value);
    }

    fn modify_apsr<F>(core: &mut CoreComponent, f: F)
    where
        F: FnOnce(XPSR) -> XPSR,
    {
        let this = Self::component_to_member_mut(core);
        let ctx = this.get_active_instruction_execution_context();

        ctx.modify_apsr(f);
    }
}

impl Execute {
    fn get_state(core: &mut CoreComponent) -> &mut InstructionExecutionState {
        let this = Self::component_to_member_mut(core);
        let iectx = this.get_active_instruction_execution_context();

        &mut iectx.state
    }

    fn set_state(core: &mut CoreComponent, state: InstructionExecutionState) {
        let this = Self::component_to_member_mut(core);
        let iectx = this.get_active_instruction_execution_context();

        iectx.state = state;
    }
}

// ----------------------------------------------------------------------------
// [ARM-ARM] A2.3.1 Registers and Execution state :: ARM core registers
// ----------------------------------------------------------------------------

impl Execute {
    /// [ARM-ARM] A2.3.1
    #[must_use]
    #[inline]
    fn branch_write_pc(core: &mut CoreComponent, address: Word) -> ExecutionStepResult {
        Self::branch_to(core, address.with_bit_set(0, false))
    }

    /// [ARM-ARM] A2.3.1
    /// Performs an interworking branch
    #[must_use]
    #[inline]
    fn bx_write_pc(core: &mut CoreComponent, address: Word) -> ExecutionStepResult {
        if interrupt::check_if_branch_to_given_address_returns_from_exception(core, address) {
            Self::request_interruption(core);
            InterruptEntryAndExitHandler::init_exception_return(core, address);

            let this = Self::component_to_member_mut(core);
            this.get_active_instruction_execution_context()
                .mark_register_clean(RegisterID::PC);

            ExecutionStepResult::ExceptionReturn
        } else {
            // TODO: Set EPSR.T bit.
            if !address.get_bit(0) {
                unimplemented!("UsageFault on interworking to ARM")
            }
            Self::branch_to(core, address.with_bit_set(0, false))
        }
    }

    /// [ARM-ARM] A2.3.1
    /// Performs an interworking branch
    #[must_use]
    #[inline]
    fn blx_write_pc(core: &mut CoreComponent, address: Word) -> ExecutionStepResult {
        // TODO: Set EPSR.T bit.
        if !address.get_bit(0) {
            unimplemented!("UsageFault on interworking to ARM")
        }
        Self::branch_to(core, address.with_bit_set(0, false))
    }

    /// [ARM-ARM] A2.3.1
    /// Called late in the cycle (tock phase)
    /// Performs an interworking branch (because it is delegated to `BXWritePC()`.
    fn load_write_pc(core: &mut CoreComponent, address: Word) {
        // TODO: assert we're after Tock
        // This is a duplicate of bx_write_pc, but in the context of Tock phase
        // PC is cleaned in finish_instruction_in_tock
        if interrupt::check_if_branch_to_given_address_returns_from_exception(core, address) {
            Self::request_interruption(core);
            InterruptEntryAndExitHandler::init_exception_return(core, address);
        } else {
            // Delayed "make branch", analogous to alu_write_pc
            if !address.get_bit(0) {
                unimplemented!("UsageFault on interworking to ARM")
            }

            // Analogous code in case of non-LSU branches is run
            let address = address.with_bit_set(0, false);
            Self::late_branch_to(core, address);
        }
    }

    /// [ARM-ARM] A2.3.1
    #[must_use]
    #[inline]
    fn alu_write_pc(core: &mut CoreComponent, address: Word) -> ExecutionStepResult {
        // TODO: this seems too late for writing address on fetch in this cycle,
        // Self::branch_write_pc(core, address)
        let address = address.with_bit_set(0, false);
        Fetch::disable_fetch(core);
        Self::late_branch_to(core, address);
        let this = Self::component_to_member_mut(core);
        this.get_active_instruction_execution_context()
            .mark_register_clean(RegisterID::PC);
        ExecutionStepResult::LateBranch
    }

    /// [ARM-ARM] B1.4.7 Pseudocode details of ARM core register accesses
    #[must_use]
    #[inline]
    fn branch_to(core: &mut CoreComponent, address: Word) -> ExecutionStepResult {
        let this = Self::component_to_member_mut(core);
        this.get_active_instruction_execution_context()
            .mark_register_clean(RegisterID::PC);
        ExecutionStepResult::ExecuteTimeBranch { address }
    }

    /// A version of `branch_to`, which is run after 50% of cycle (before or after tock)
    /// [ARM-ARM] B1.4.7 Pseudocode details of ARM core register accesses
    #[inline]
    fn late_branch_to(core: &mut CoreComponent, address: Word) {
        let this = Self::component_to_member_mut(core);
        // TODO: Move logic handling to handle_exec_result?
        this.next_instr_addr = address;
        Fetch::make_delayed_branch(core, address);
    }
}

// Returns mask such that:
// w[i] = 1, for i in [msbit, lsbit]
// w[i] = 0, otherwise
// Useful to extraction of bits, when offsets are dynamic. Like in:
// [ARM-ARM] A7.7.13, A7.7.14 and A7.7.126
#[allow(clippy::similar_names)] // To follow documentation convention
fn mask_between_bits(msbit: u8, lsbit: u8) -> u32 {
    debug_assert!(msbit < 32);
    debug_assert!(msbit >= lsbit);

    let ms_mask = (1_u64 << (msbit + 1)) - 1;
    let ls_mask = (1_u64 << lsbit) - 1;
    (ms_mask ^ ls_mask).try_into().unwrap()
}

// ----------------------------------------------------------------------------
// Helper methods to decide execution time of multi-cycle instructions
// ----------------------------------------------------------------------------
// NOTE: cycle counter starts from 0

const HIGH_HALF_MASK_I32: i32 = -0x0001_0000; // aka. 0xFFFF_0000
const LOW_HALF_MASK_I32: i32 = 0x0000_FFFF;
const HIGH_HALF_MASK_U32: u32 = 0xFFFF_0000;
const LOW_HALF_MASK_U32: u32 = 0x0000_FFFF;

// IMPORTANT: UMULL, SMULL, UMLAL and SMLAL execution time:
// Basic idea/guess is that in order to implement (32bit, 32bit) -> 64bit multiplication.
// Cortex-M3 uses circuits, that output 32bit results and feeds them with 16bit parts of inputs.
// Then those results are combined with some additional operations (adds, shifts etc.).
// If one of those parts is zero then some operations are not needed and execution is shorter.

// Returns cycles, in which high and low bits are written to target registers.
#[allow(clippy::similar_names)] // To be consistent with execute names.
fn umull_execution_result_writing_cycles(rn_uint: u32, rm_uint: u32) -> (u32, u32) {
    if cfg!(feature = "soc-cc2652") {
        return (0, 0);
    }

    let rn_hi = rn_uint & HIGH_HALF_MASK_U32 != 0;
    let rn_lo = rn_uint & LOW_HALF_MASK_U32 != 0;

    let rm_hi = rm_uint & HIGH_HALF_MASK_U32 != 0;
    let rm_lo = rm_uint & LOW_HALF_MASK_U32 != 0;
    #[allow(clippy::unnested_or_patterns)]
    match (rn_hi, rn_lo, rm_hi, rm_lo) {
        (_, _, false, false)
        | (false, false, _, _)
        | (false, true, false, true)
        | (true, false, true, false) => (2, 1),
        (true, false, false, true)
        | (false, true, true, false)
        | (true, true, true, false)
        | (true, true, false, true)
        | (true, false, true, true)
        | (false, true, true, true) => (3, 2),
        (true, true, true, true) => (4, 3),
    }
}

// Returns cycles, in which high and low bits are written to target registers.
#[allow(clippy::similar_names)] // To be consistent with execute names.
fn smull_execution_result_writing_cycles(rn_sint: i32, rm_sint: i32) -> (u32, u32) {
    if cfg!(feature = "soc-cc2652") {
        return (0, 0);
    }

    let rn_hi = rn_sint & HIGH_HALF_MASK_I32 != 0;
    let rn_lo = rn_sint & LOW_HALF_MASK_I32 != 0;
    let rn_neg = rn_sint.is_negative();

    let rm_hi = rm_sint & HIGH_HALF_MASK_I32 != 0;
    let rm_lo = rm_sint & LOW_HALF_MASK_I32 != 0;
    let rm_neg = rm_sint.is_negative();

    match (rn_hi, rn_lo, rm_hi, rm_lo) {
        (_, _, false, false) | (false, false, _, _) => (2, 1),
        (false, true, false, true) | (true, false, true, false) => {
            if rn_neg || rm_neg {
                (3, 2)
            } else {
                (2, 1)
            }
        }
        (true, true, true, false) => {
            if rn_neg || rm_neg {
                (4, 3)
            } else {
                (3, 2)
            }
        }
        (true, false, true, true) => (3, 2),
        (true, true, true, true) => (4, 3),
        _ => {
            if rm_neg {
                (4, 3)
            } else {
                (3, 2)
            }
        }
    }
}

// Returns cycles, in which high and low bits are written to target registers.
#[allow(clippy::similar_names)] // To be consistent with execute names.
fn umlal_execution_result_writing_cycles(rn_uint: u32, rm_uint: u32) -> (u32, u32) {
    if cfg!(feature = "soc-cc2652") {
        return (0, 0);
    }

    let rn_hi = rn_uint & HIGH_HALF_MASK_U32 != 0;
    let rn_lo = rn_uint & LOW_HALF_MASK_U32 != 0;

    let rm_hi = rm_uint & HIGH_HALF_MASK_U32 != 0;
    let rm_lo = rm_uint & LOW_HALF_MASK_U32 != 0;
    #[allow(clippy::unnested_or_patterns)]
    match (rn_hi, rn_lo, rm_hi, rm_lo) {
        (_, _, false, false) | (false, false, _, _) | (false, true, false, true) => (2, 1),
        (true, false, true, false)
        | (true, false, false, true)
        | (true, true, false, true)
        | (false, true, true, false)
        | (false, true, true, true) => (4, 3),
        (true, true, true, false) | (true, false, true, true) => (5, 4),
        (true, true, true, true) => (6, 5),
    }
}

// Returns cycles, in which high and low bits are written to target registers.
#[allow(clippy::similar_names)] // To be consistent with execute names.
fn smlal_execution_result_writing_cycles(rn_sint: i32, rm_sint: i32) -> (u32, u32) {
    if cfg!(feature = "soc-cc2652") {
        return (0, 0);
    }

    let rn_hi = rn_sint & HIGH_HALF_MASK_I32 != 0;
    let rn_lo = rn_sint & LOW_HALF_MASK_I32 != 0;
    let rn_neg = rn_sint.is_negative();

    let rm_hi = rm_sint & HIGH_HALF_MASK_I32 != 0;
    let rm_lo = rm_sint & LOW_HALF_MASK_I32 != 0;
    let rm_neg = rm_sint.is_negative();

    match (rn_hi, rn_lo, rm_hi, rm_lo) {
        (_, _, false, false) | (false, false, _, _) | (false, true, false, true) => (2, 1),
        (true, false, true, false) => {
            if rn_neg || rm_neg {
                (5, 4)
            } else {
                (4, 3)
            }
        }
        (true, true, true, false) => {
            if rn_neg || rm_neg {
                (6, 5)
            } else {
                (5, 4)
            }
        }
        (true, false, true, true) => (5, 4),
        (true, true, true, true) => (6, 5),
        _ => {
            if rm_neg {
                (5, 4)
            } else {
                (4, 3)
            }
        }
    }
}

// IMPORTANT: UDIV and SDIV execution timings.
// We assumed that Cortex can do 4 operations per cycle and timing depends on the
// highest bit of absolute values of dividend and divisor.

// Let's call dividend the highest bit position - X and divisor - Y
// We found 3 cases to calculate udiv cycles:
// - dividend or divisor are 0 - 2 cycle
// - X < Y - 3 cycles
// - otherwise 5 + (X - Y) / 4.
#[allow(clippy::similar_names)] // To be consistent with execute names.
fn udiv_execution_result_writing_cycle(rn_val: Word, rm_val: Word) -> u32 {
    let timing = if rn_val.is_zero() || rm_val.is_zero() {
        2
    } else if rn_val.highest_set_bit() < rm_val.highest_set_bit() {
        3
    } else {
        let diff = rn_val.highest_set_bit() - rm_val.highest_set_bit();
        debug_assert!(diff >= 0);
        #[allow(clippy::cast_sign_loss)]
        let timing = (diff as u32) / 4 + 5;
        timing
    };

    timing - 1
}

// It was observed that taking absolute value of negative inputs can give almost
// the same timings as UDIV. Two observations are:
// 1) if a divisor is a negative power of 2, and a dividend isn't 0x8000_0000,
//    0.25 of the cycle should be added to the timing.
// 2) if a divisor isn't negative power of 2 and dividend is 0x8000_0000,
//    0.25 of the cycle should be subtracted from the timing.
// In this way, powers of two with an exponent that is multiplication of
// 4 have greater timing than next numbers.
#[allow(clippy::similar_names)] // To be consistent with execute names.
fn sdiv_execution_result_writing_cycle(mut rn_val: Word, mut rm_val: Word) -> u32 {
    let timing = if rn_val.is_zero() || rm_val.is_zero() {
        2
    } else {
        let mut rm_was_negative = false;
        rn_val = Word::from(i32::wrapping_abs(rn_val.sint()));

        if rm_val.sint() < 0 {
            rm_val = Word::from(i32::wrapping_neg(rm_val.sint()));
            rm_was_negative = true;
        }

        let mut diff = rn_val.highest_set_bit() - rm_val.highest_set_bit();
        let increase_diff =
            rm_was_negative && rm_val.uint().is_power_of_two() && rn_val.uint() != 0x8000_0000;
        let decrease_diff =
            rm_was_negative && !rm_val.uint().is_power_of_two() && rn_val.uint() == 0x8000_0000;
        debug_assert!(!increase_diff || !decrease_diff);
        if increase_diff {
            diff += 1;
        } else if decrease_diff {
            diff -= 1;
        }

        if diff < 0 {
            3
        } else {
            #[allow(clippy::cast_sign_loss)]
            let timing = (diff as u32) / 4 + 5;
            timing
        }
    };

    timing - 1
}
