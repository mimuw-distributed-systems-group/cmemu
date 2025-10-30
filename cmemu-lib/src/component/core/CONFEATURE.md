# Using confeature for hyperparameter search

This example shows how we used hyperparameter search to optimize `can_lsu_be_pipelined rules`.

The Rust function looked like this:

```rust
/// Returns true if instruction in `main_ctx` and (`next_instruction`, `has_folded_instruction`) can be pipelined.
fn can_lsu_instr_be_pipelined_old(
    core: &CoreComponent,
    main_ctx: &InstructionExecutionContext,
    next_instruction: &Instruction,
    is_skipped: bool,
    has_folded_instruction: bool,
) -> PipeliningResult {
    use crate::confeature::cm_hyp::*;
    // Only on first cycle of data-phase (see tests with stalled multi-cycle ldrs)
    if !LSU::can_pipeline_new_request(core) || !main_ctx.instruction().is_lsu_instruction() {
        return PipeliningResult::NotPipelineableAtAll;
    }

    // TODO: [ARM-TRM-G] 18.3 Load-store timings has notes about pipelining, but it is useless
    // TODO: How was handled the following errata? -- answ: LDR ..., SP cannot pipeline; STR ..., any! cannot pipeline
    // [ARM-ERR-2008] 377681: Interrupted fault-generating load/store pair with SP base-writeback may corrupt stack
    // The errata concerns a case of ldr/str rA, sp!; ldr/str any and fault is generated e.g. in second cycle
    // and some other interrupts comes as well.

    let pre_desc = main_ctx.instruction().get_memory_description();
    let pre = Self::get_memorier_description(main_ctx.instruction());
    let pre_base_reg = pre.addr_base_reg();
    // This may be related with errata for r1p1:
    let is_lsu_unaligned = LSU::in_unaligned_special(core);
    let prev_length = main_ctx.pipeline_step_pack.length;

    let decode_saw_full_word = Decode::was_full_word_available(core);
    let is_narrow_tainted = Decode::is_instr_tainted(core);

    let last_narrow = !decode_saw_full_word
        && (prev_length == 1).implies(*last_narrow::AFTER_NARROW)
        && pre.writeback.implies(*last_narrow::AFTER_WRITEBACK);
    //
    let can_preceed_pipelined_nonreg = match pre_desc {
        MemoryInstructionDescription::None
        | MemoryInstructionDescription::LoadMultiple { .. }
        | MemoryInstructionDescription::StoreMultiple { .. } => false,

        MemoryInstructionDescription::LoadSingle { rt, .. } => {
            rt != RegisterID::PC && rt != RegisterID::SP
            // FIXME: LAST? STORE?
            // && !(ctx.visible_xpsr.in_it_block() && pre.addr_base_reg() == RegisterID::SP)
        }

        // TODO: [ARM-TRM-G] 18.3
        MemoryInstructionDescription::StoreSingle { rt, writeback, .. } => {
            rt != RegisterID::PC
                && rt != RegisterID::SP
                && !(writeback && is_lsu_unaligned && *str_nop::PRE_STR_REG_WRITEBACK_UNALIGNED)
        }
    };

    // We can distinguish ALU-AGU RAW vs nonpipelining with an unaligned address
    let can_preceed_pipelined_lsu = match pre_desc {
        MemoryInstructionDescription::None
        | MemoryInstructionDescription::LoadMultiple { .. }
        | MemoryInstructionDescription::StoreMultiple { .. } => false,

        MemoryInstructionDescription::LoadSingle { rt, .. } => {
            rt != RegisterID::PC
                // TODO: we need large tests with all valid destination/source
                && rt != RegisterID::SP
            // && pre.addr_base_reg() != RegisterID::SP /* && !writeback */
        }
        // TODO: [ARM-TRM-G] 18.3
        MemoryInstructionDescription::StoreSingle {
            register_offset,
            writeback,
            ..
        } => {
            // No sp, see definitive_lsu_reg_offset-partial8
            // Store with writeback pull data in execute, because the second bus is busy for writeback by ALU
            !register_offset && !writeback // && rt != RegisterID::SP
            // && pre.addr_base_reg() != RegisterID::SP
        }
    };
    let xpsr = main_ctx.visible_xpsr;
    let next_desc = next_instruction.get_memory_description();

    let data_addr_dep = !pre.is_store
        && next_instruction.is_lsu_instruction()
        && Self::get_memorier_description(next_instruction)
        .addr_registers_bitmap
        .bitand(pre.data_registers_bitmap)
        .count()
        != 0;

    // This is handled by implicit AGU stall, but the same logic may be used elsewhere.
    if data_addr_dep {
        return PipeliningResult::DataAddressDependency;
    }

    if !next_instruction.is_lsu_instruction()
    /* || is_skipped */
    {
        // if last_narrow {
        //     todo!("LAST_NARROW V0");
        // }
        // TODO: expand usages of this var to hyperparams
        let tainted_last_narrow = last_narrow && is_narrow_tainted;
        // if last_narrow
        //     && is_narrow_tainted
        //     && (data_addr_dep || !next_instruction.is_lsu_instruction())
        // {
        //     return false;
        //     //   todo!("LAST_NARROW V1");
        // }
        match next_instruction {
            Instruction::NoOperation if /*next_*/xpsr.in_it_block() => match pre_desc {
                MemoryInstructionDescription::None
                | MemoryInstructionDescription::LoadMultiple { .. }
                | MemoryInstructionDescription::StoreMultiple { .. } => false,
                MemoryInstructionDescription::StoreSingle {
                    rt,
                    register_offset,
                    writeback,
                } if (rt == RegisterID::SP && *str_nop::PRE_STR_SP)
                    || (rt == RegisterID::PC && *str_nop::PRE_STR_PC)
                    || (rt == RegisterID::LR && *str_nop::PRE_STR_LR)
                    || (writeback && *str_nop::PRE_STR_REG_WRITEBACK)
                    || (writeback
                    && is_lsu_unaligned
                    && *str_nop::PRE_STR_REG_WRITEBACK_UNALIGNED)
                    || (register_offset && *str_nop::PRE_STR_REG_OFF)
                    || (pre_base_reg == RegisterID::SP && *str_nop::PRE_STR_ADDR_SP)
                    || (pre_base_reg == RegisterID::PC && *str_nop::PRE_STR_ADDR_PC) =>
                    {
                        false
                    }
                MemoryInstructionDescription::LoadSingle {
                    rt,
                    register_offset,
                    writeback,
                } if (rt == RegisterID::SP && *ldr_nop::PRE_LDR_SP)
                    || (rt == RegisterID::PC && *ldr_nop::PRE_LDR_PC)
                    || (rt == RegisterID::LR && *ldr_nop::PRE_LDR_LR)
                    || (writeback && *ldr_nop::PRE_LDR_REG_WRITEBACK)
                    || (writeback
                    && is_lsu_unaligned
                    && *ldr_nop::PRE_LDR_REG_WRITEBACK_UNALIGNED)
                    || (register_offset && *ldr_nop::PRE_LDR_REG_OFF)
                    || (pre_base_reg == RegisterID::SP && *ldr_nop::PRE_LDR_ADDR_SP)
                    || (pre_base_reg == RegisterID::PC && *ldr_nop::PRE_LDR_ADDR_PC) =>
                    {
                        false
                    }
                MemoryInstructionDescription::LoadSingle { .. }
                | MemoryInstructionDescription::StoreSingle { .. }
                if (last_narrow && *lsu_nop::POST_LAST_NARROW) =>
                    {
                        false
                    }
                _ => !has_folded_instruction && !tainted_last_narrow,
            },
            // TODO: check if nop after it block bahaves as the above!!!
            Instruction::NoOperation => !has_folded_instruction && can_preceed_pipelined_nonreg, // taint here?
            // What about LSU/TBH branches?
            // TODO: we need a grid search
            // _ if next_instruction.is_branch() => false,

            // Instruction::IfThen { .. } => !has_folded_instruction,
            // TODO: IT can decode and allow another instruction?

            // STR (register) only allows consuming actual NOPs (even if RegBank is not used)
            // proof: see `memory/str_reg_pipelining.asm`
            // Yet, it seems that such may pipeline after unaligned STR.
            // See `definitive_lsu_reg_offset-partial12` conf:
            // {'code': 'flash', 'cnt': 'CYCCNT', 'memory': 'sram', 'base_offset': 2, 't': 'ttet',
            //  'flags': 'eq', 'lbEn': True, 'tested_instr': 'str.w {Rt}, [{Rn}, {Rm}]',
            //  'pad1': 'mov.n r6, r5', 'pad2': 'add.n r1, r1', 'stall1': 'x_cyc', 'stall2': 'x_cyc',
            //  'addr_reg': 'r5', 'prev_instr': 'adds.w {dest}, {dest}, {reg_0_or_2}'}
            // Question: what about DENY-stalled STR?

            // definitive_lsu_reg_offset-partial12.1019120
            // it seems that for :ite ok; str sp. [r3]; ldr r6, [sp,4]
            // the skipped ldr can pipeline under the str (in the test the str doesn't inc LSUCNT)
            // TODO: maybe it has wider implications?
            // TODO: test with ldm instead of ldr?
            _ if next_instruction.is_lsu_instruction() => {
                let post = Self::get_memorier_description(next_instruction);
                let is_tbb = matches!(next_instruction, Instruction::TableBranch {..});
                let post_base_reg = post.addr_base_reg();
                match (pre_desc, next_desc) {
                    (
                        MemoryInstructionDescription::None
                        | MemoryInstructionDescription::LoadMultiple { .. }
                        | MemoryInstructionDescription::StoreMultiple { .. },
                        _,
                    ) => false,
                    // (_, _) if !decode_saw_full_word => todo!("LAST_NARROW"),
                    (
                        MemoryInstructionDescription::LoadSingle {
                            writeback: true, ..
                        }
                        | MemoryInstructionDescription::StoreSingle {
                            writeback: true, ..
                        },
                        _,
                    ) if ((*lsu_lsu::POST_WRITEBACK_DEP || tainted_last_narrow)
                        && Self::get_memorier_description(next_instruction)
                        .addr_registers_bitmap
                        .get(pre.addr_base_reg())) =>
                        {
                            false
                        }
                    (MemoryInstructionDescription::LoadSingle { .. }, _)
                    if (*lsu_lsu::POST_LOAD_DEP && data_addr_dep
                        && (!*lsu_lsu::POST_LOAD_DEP_ONLY_LAST_NARROW || last_narrow))
                        || (tainted_last_narrow && data_addr_dep && true)
                    =>
                        {
                            false
                        }
                    (
                        _,
                        MemoryInstructionDescription::StoreSingle {
                            rt,
                            register_offset,
                            writeback,
                        }
                        | MemoryInstructionDescription::LoadSingle {
                            rt,
                            register_offset,
                            writeback,
                        },
                    ) if (rt == RegisterID::SP && *lsu_lsu::POST_SP)
                        || (rt == RegisterID::PC && *lsu_lsu::POST_PC)
                        || (rt == RegisterID::LR && *lsu_lsu::POST_LR)
                        || (writeback && *lsu_lsu::POST_REG_WRITEBACK)
                        || (register_offset && *lsu_lsu::POST_REG_OFF)
                        || (last_narrow && *lsu_lsu::POST_LAST_NARROW)
                        || (post_base_reg == RegisterID::SP && *lsu_lsu::POST_ADDR_SP)
                        || (post_base_reg == RegisterID::PC && *lsu_lsu::POST_ADDR_PC)
                    =>
                        {
                            false
                        }
                    (
                        MemoryInstructionDescription::StoreSingle {
                            rt,
                            register_offset,
                            writeback,
                        },
                        _,
                    ) if (rt == RegisterID::SP && *str_lsu::PRE_STR_SP)
                        || (rt == RegisterID::PC && *str_lsu::PRE_STR_PC)
                        || (rt == RegisterID::LR && *str_lsu::PRE_STR_LR)
                        || (writeback && *str_lsu::PRE_STR_REG_WRITEBACK)
                        || (writeback && pre_base_reg == RegisterID::SP && *lsu_lsu::PRE_REG_WRITEBACK_SP)
                        || (writeback && rt == RegisterID::SP && *str_lsu::PRE_STR_SP_AND_WRITEBACK)
                        || (writeback
                        && is_lsu_unaligned
                        && *str_lsu::PRE_STR_REG_WRITEBACK_UNALIGNED)
                        || (register_offset && *str_lsu::PRE_STR_REG_OFF)
                        || (pre_base_reg == RegisterID::SP && *str_lsu::PRE_STR_ADDR_SP)
                        || (rt == RegisterID::SP && (post.writeback || is_tbb) && *str_lsu::POST_WRITEBACK_AFTER_STORE_SP)
                        || (writeback && tainted_last_narrow)
                    =>
                        {
                            false
                        }
                    (
                        MemoryInstructionDescription::LoadSingle {
                            rt,
                            register_offset,
                            writeback,
                        },
                        _,
                    ) if (rt == RegisterID::SP && *ldr_lsu::PRE_LDR_SP)
                        || (rt == RegisterID::PC && *ldr_lsu::PRE_LDR_PC)
                        || (rt == RegisterID::LR && *ldr_lsu::PRE_LDR_LR)
                        || (writeback && *ldr_lsu::PRE_LDR_REG_WRITEBACK)
                        || (writeback && pre_base_reg == RegisterID::SP && *lsu_lsu::PRE_REG_WRITEBACK_SP)
                        || (register_offset && *ldr_lsu::PRE_LDR_REG_OFF)
                        || (writeback
                        && is_lsu_unaligned
                        && *ldr_lsu::PRE_LDR_REG_WRITEBACK_UNALIGNED)
                        || (pre_base_reg == RegisterID::SP && *ldr_lsu::PRE_LDR_ADDR_SP)
                        || (pre_base_reg == RegisterID::PC && *ldr_lsu::PRE_LDR_ADDR_PC)
                    =>
                        {
                            false
                        }
                    (
                        _,
                        MemoryInstructionDescription::None
                        | MemoryInstructionDescription::LoadMultiple { .. }
                        | MemoryInstructionDescription::StoreMultiple { .. },
                        // Latest developments
                    ) => !tainted_last_narrow && !(pre.is_store && pre.single_data_reg == Some(RegisterID::SP)),
                    _ => true,
                }
            }
            // TODO: check for folded instruction
            _ if is_skipped => match (pre_desc, next_instruction.get_read_registers()) {
                (
                    MemoryInstructionDescription::None
                    | MemoryInstructionDescription::LoadMultiple { .. }
                    | MemoryInstructionDescription::StoreMultiple { .. },
                    _,
                ) => false,

                (
                    MemoryInstructionDescription::StoreSingle {
                        rt,
                        register_offset,
                        writeback,
                    },
                    _,
                ) if (rt == RegisterID::SP && *str_nonlsu::PRE_STR_SP)
                    || (rt == RegisterID::PC && *str_nonlsu::PRE_STR_PC)
                    || (rt == RegisterID::LR && *str_nonlsu::PRE_STR_LR)
                    || (writeback && *str_nonlsu::PRE_STR_REG_WRITEBACK)
                    || (writeback
                    && is_lsu_unaligned
                    && *str_nonlsu::PRE_STR_REG_WRITEBACK_UNALIGNED)
                    || (register_offset && *str_nonlsu::PRE_STR_REG_OFF)
                    || (pre_base_reg == RegisterID::SP && *str_nonlsu::PRE_STR_ADDR_SP)
                //|| (pre_base_reg == RegisterID::PC && *str_nonlsu::PRE_STR_ADDR_PC)
                =>
                    {
                        false
                    }
                (
                    MemoryInstructionDescription::LoadSingle {
                        rt,
                        register_offset,
                        writeback,
                    },
                    _,
                ) if (rt == RegisterID::SP && *ldr_nonlsu::PRE_LDR_SP)
                    || (rt == RegisterID::PC && *ldr_nonlsu::PRE_LDR_PC)
                    || (rt == RegisterID::LR && *ldr_nonlsu::PRE_LDR_LR)
                    || (writeback && *ldr_nonlsu::PRE_LDR_REG_WRITEBACK)
                    || (register_offset && *ldr_nonlsu::PRE_LDR_REG_OFF)
                    || (pre_base_reg == RegisterID::SP && *ldr_nonlsu::PRE_LDR_ADDR_SP)
                    || (pre_base_reg == RegisterID::PC && *ldr_nonlsu::PRE_LDR_ADDR_PC)
                =>
                    {
                        false
                    }
                (MemoryInstructionDescription::LoadSingle { rt, .. }, read_regs)
                if (*lsu_nonlsu::POST_REG_DEP && read_regs.get(rt))
                    && (!*lsu_nonlsu::POST_LOAD_DEP_ONLY_LAST_NARROW || last_narrow) =>
                    {
                        false
                    }
                (_, read_regs)
                if (*lsu_nonlsu::POST_READS_SP && read_regs.get(RegisterID::SP)) =>
                    {
                        false
                    }
                (_, _)
                if (*lsu_nonlsu::POST_WRITES_SP
                    && next_instruction.get_written_registers().get(RegisterID::SP)) =>
                    {
                        false
                    }
                (_, _) if (last_narrow && *lsu_nonlsu::POST_LAST_NARROW) => false,
                (
                    MemoryInstructionDescription::LoadSingle {
                        writeback: true, ..
                    }
                    | MemoryInstructionDescription::StoreSingle {
                        writeback: true, ..
                    },
                    read_regs,
                ) if (*lsu_nonlsu::POST_WRITEBACK_DEP
                    && read_regs.get(pre.addr_base_reg())) =>
                    {
                        false
                    }
                _ if tainted_last_narrow => false,
                _ => true,
            }, // TODO: check for folded instruction

            // TODO: check for cmp, tst and not-taken it
            _ => false,
        }
    } else if next_instruction.is_lsu_instruction() && can_preceed_pipelined_lsu {
        // TODO: just a check
        match next_instruction.get_memory_description() {
            // TODO: [ARM-TRM-G] 18.3 mentions writeack only for STR
            // TODO: try to BAN the case when prev updates addr register (aka disable forwarding)
            //       per [ARM-CM4-TRM-D] 3.3.3
            // XXX: Check the register offset non-pipelineing (it is not properly tested) - this if comes from ARMISTICE paper
            // TODO: there may be more data dependency (maybe code like from the next branch is needed?)
            // The idea is that, STR (imm) may both read the addr base and the data in the first cycle (D/Ex)
            // But STR (reg), may only read the data register in the following cycle.
            // Thus: a) STR (reg) may allow extra cycle when dependent on data reg
            //       b) Decode may be unable to use the register bank? (E.g. bx lr?)
            // && (!register_offset
            //     || (!pre.is_store && pre.single_data_reg != next.single_data_reg))
            MemoryInstructionDescription::LoadSingle { writeback, .. }
            | MemoryInstructionDescription::StoreSingle { writeback, .. } => {
                // See the `tbb_tbh_after_ldr.asm` test: TBB/TBH doesn't pipeline after LDR at all.
                writeback.ife(PipeliningResult::PostWriteback, PipeliningResult::Pipelines)
            }
            // If the previous one is store, we are delayed by a store buffer. (What was the proof of str/ldr pipelining?)
            // Nevertheless, the waiting is done with a DENY instead of a waitstate.
            // This is seen with unaligned accesses, when we can pipeline only in the cycle-next-after advance).
            // str:   X:DA  X:DD          /- pipelining happens here, so D1 must be first cycle of advancing
            // ldr unalign: X:DA  X:D? X:DD1, X:DD2
            // nop:         D                 X
            // branch:                        D

            // In theory, this should be noticeable with simpler tests (TODO?) like so: NOT VERIFIED
            // str:   X:DA  X:DD       /- pipelining should happen here, so D? must be deny (X:DA-)
            // ldr aligned: X:DA  X:D? X:DD
            // nop:         D          X
            // branch:                 D
            // TODO: check if the behavior is same on GPRAM
            // But we explicitly implemented the WriteBuffer to behave like InputStage (keep waiting, not denying)
            // changing that would require either support from combinatorial_os (there is a lot of old-experimental code for denies).
            // Was there any proof why waitstating and not denying? Maybe because of conflicts resolution with fetch?

            // Moreover, it only seems to break with SRAM, and GPRAM uses waitstates instead.
            // See `definitive_lsu_reg_offset-partial10`.  Is it the case or LSU/Fetch collision occludes this case?

            // For STR_register, it seems that:
            // in D: address regs are read
            // in X:A: the target register is read
            // This can mean two things:
            // - if the reg is not fast-forwarded, it must use RegBank
            // - it may conflict with decoding of a following instr (especially if it requires two registers)
            // See: `definitive_lsu_reg_offset-partial12`. With: mem=gpram code
            // isb.w; ldr.w CYC; add.w; add.w; iETTE (pseudo-it); add.n; ldr.w [reg_off]; str.w [reg_off]; pad2
            // This is extra to the reg-dep case and sp-case
            // TODO: [ARM-TRM-G] 18.3 Load-store timings state, that LDR Rx!, [any] cannot be
            //  normally pipelined, except for "non register writing instructions": e.g., cmp, tst, nop, skipped it
            // The practice seems to be different and just the writeback register cannot be address source of the next
            // TODO: we need a test for that
            // ldr ra, [lr (unaligned)], 2
            // ldr rb, [lr, 4]
            // seem to pipeline
            // RESULT: apparently the non-pipelining comes from AGU stall and only at A->D transition
            //         with unaligned, the transition happen at the next cycle, so that
            //         the address makes that in time.
            MemoryInstructionDescription::LoadMultiple { .. }
            | MemoryInstructionDescription::StoreMultiple { .. } => PipeliningResult::Other,

            MemoryInstructionDescription::None => unreachable!(),
        }
    } else {
        PipeliningResult::NotPipelineableAtAll
    }
}
```

while the `confeature` configuration file contained:

```yaml
cm_hyp:
  .confeature:
    doc: Hyperparameters of the model.
    #    mode_cap: comptime
    default_mode: anytime
  str_lsu:
    type: namespace
    ns:
      .confeature:
        doc: STR; STR/LDR in IT block

      pre_str_sp:
        type: bool
        # ???
        #        default: true
        default: false
        #        mode: comptime
      pre_str_pc:
        type: bool
        # unconclusive - no effect
        default: true # unpredictable
        # skopt: no influence
        mode: comptime
      pre_str_lr:
        type: bool
        # unconclusive - no effect
        default: false
        # skopt: no influence
        mode: comptime
      pre_str_reg_off:
        type: bool
        default: true
        mode: comptime
      pre_str_reg_writeback:
        type: bool
        #     mid-confindence
        default: false # ???
        mode: comptime
      pre_str_reg_writeback_unaligned:
        type: bool
        default: true # skopt: highly likely
      pre_str_addr_sp:
        type: bool
        default: false # skopt all top 50
      pre_str_addr_lr:
        type: bool
        # skopt: no influence
      post_writeback_after_store_sp:
        type: bool
        default: true # it_pipelining.asm
      pre_str_sp_and_writeback:
        type: bool
        default: true # it_pipelining.asm

  ldr_lsu:
    type: namespace
    ns:
      .confeature:
        doc: LDR; STR/LDR in IT block

      pre_ldr_sp:
        type: bool
        default: true
        mode: comptime
      pre_ldr_pc:
        type: bool
        # unconclusive - no effect
      #        default: true
      pre_ldr_lr:
        type: bool
        # low-confidence
        default: false
        mode: comptime
      pre_ldr_reg_off:
        type: bool
        default: false
        mode: comptime
      pre_ldr_reg_writeback:
        type: bool
        default: false
        # mid-sure
        mode: comptime
      pre_ldr_reg_writeback_unaligned:
        type: bool
        default: false # skopt all top 50
      pre_ldr_addr_sp:
        type: bool
        default: false # skopt all top 50
        # Breaks a lot of tests otherwise
        mode: comptime
      pre_ldr_addr_pc:
        type: bool
        default: false # skopt all top 50
        # Breaks a lot of tests otherwise
        mode: comptime

  # All false: no dep observed
  lsu_lsu:
    type: namespace
    ns:
      .confeature:
        doc: LDR/STR; STR/LDR in IT block

      post_sp:
        type: bool
        default: false
        # mid-sure
        mode: comptime
      post_pc:
        default: false # no data
        type: bool
        mode: comptime # skopt: no influence
      post_lr:
        type: bool
        default: false # no data
        mode: comptime # skopt: no influence
      post_reg_off:
        type: bool
        default: false
        mode: comptime
      post_reg_writeback:
        type: bool
        # low confidence (second bad)
        # skopt suggest these two are very likely false
        default: false
        mode: comptime # skoopt (all from top)
      pre_reg_writeback_sp:
        type: bool
      #        default: true
      post_writeback_dep:
        type: bool
        default: false
        mode: comptime # skoopt (all from top)
      post_load_dep:
        type: bool
        default: false # skopt all top 50
        # Breaks a lot of tests otherwise
        mode: comptime
      post_last_narrow:
        doc: Handling of the special case, when Decode sees only a narrow instruction.
        type: bool
        default: false # skopt: high confidence
      post_load_dep_only_last_narrow:
        type: bool
      #        default: true # skopt: unconclusive, but used in the model
      post_addr_sp:
        type: bool
        default: false # skopt all top 50
        mode: comptime # skoopt (all from top)
      post_addr_pc:
        type: bool # skopt: unconclusive

  str_nonlsu:
    type: namespace
    ns:
      .confeature:
        doc: STR; skipped add/mov/etc in IT block

      pre_str_sp:
        type: bool
        default: true # mixed results
        mode: comptime # skoopt (all from top)
      pre_str_pc:
        type: bool
        # unconclusive - no effect
        default: true # unpredictable
        mode: comptime
      pre_str_lr:
        type: bool
        # unconclusive - no effect
        default: false
        mode: comptime
      pre_str_reg_off:
        type: bool
        default: true
        mode: comptime
      pre_str_reg_writeback:
        type: bool
        # mid-confindence
      #        default: false # ???
      #        default: true # ???
      #        mode: comptime # skoopt (all from top)
      pre_str_reg_writeback_unaligned:
        type: bool
        default: true
        # skopt: not that certain! probably other stuff with dependency
      pre_str_addr_sp:
        type: bool
        default: false # skopt all top 50
        mode: comptime
        #pre_str_addr_pc:
        #type: bool

  ldr_nonlsu:
    type: namespace
    ns:
      .confeature:
        doc: LDR; skipped add/mov/etc in IT block

      pre_ldr_sp:
        type: bool
        default: true
        mode: comptime
      pre_ldr_pc:
        type: bool
        #     unconclusive - no effect
        default: true
      pre_ldr_lr:
        type: bool
        # low-confidence
        default: false
      pre_ldr_reg_off:
        type: bool
        default: false
        mode: comptime # skoopt (all from top)
        # Likely false, but it's true for STR
      ##    mode: comptime
      pre_ldr_reg_writeback:
        type: bool
        default: false
        mode: comptime # skoopt (all from top)
      #     mid-sure
      #    mode: comptime
      pre_ldr_reg_writeback_unaligned:
        type: bool
        # unconclusive (no checks?)
      pre_ldr_addr_sp:
        type: bool
        default: false # skopt all top 50
        mode: comptime
      pre_ldr_addr_pc:
        type: bool

  lsu_nonlsu:
    type: namespace
    ns:
      .confeature:
        doc: LDR/STR; skipped add/mov/etc in IT block -- post dependencies

      # TODO: Mixed results, but influential
      # unconclusive
      post_reads_sp:
        type: bool
        default: false
      post_writes_sp:
        type: bool
        default: false
      post_reg_dep:
        type: bool
        default: false
      post_writeback_dep:
        type: bool
        # TODO: thoroughly test this interaction!
      #    default: true # TODO: test on larger set
      post_last_narrow:
        doc: Handling of the special case, when Decode sees only a narrow instruction.
        type: bool
        # Search shows that FALSE IS BETTER HERE! (all top 50)
        #default: true
        default: false
      post_load_dep_only_last_narrow:
        type: bool

  str_nop:
    type: namespace
    ns:
      .confeature:
        doc: STR; NOP in IT block

      pre_str_sp:
        type: bool
        # nonconclusive
        default: true
        mode: comptime
      pre_str_pc:
        type: bool
        # unconclusive - no effect
        default: true # unpredictable
        mode: comptime
      pre_str_lr:
        type: bool
        # unconclusive - no effect
        default: false
        mode: comptime
      pre_str_reg_off:
        type: bool
        # mid-confindence
        default: false
        mode: comptime
      pre_str_reg_writeback:
        type: bool
        # mid-confindence
        #     unconclusive - no effect
      #    default: false # ???
      #    mode: comptime
      pre_str_reg_writeback_unaligned:
        type: bool
        default: true
      pre_str_addr_sp:
        type: bool
        default: false # high confindence
        mode: comptime
      pre_str_addr_pc:
        type: bool

  ldr_nop:
    type: namespace
    ns:
      .confeature:
        doc: LDR; NOP in IT block

      pre_ldr_sp:
        type: bool
        default: true
        mode: comptime # skoopt (all from top)
      pre_ldr_pc:
        type: bool
        #     unconclusive - no effect
        default: true
        mode: comptime  # unconclusive
      pre_ldr_lr:
        type: bool
        # low-confidence
        default: false
        mode: comptime  # unconclusive
      pre_ldr_reg_off:
        type: bool
        # mid-confindence
        default: false
        mode: comptime # skoopt (all from top)
      pre_ldr_reg_writeback:
        type: bool
        #     unconclusive - no effect
        #    default: false
        #     mid-sure
      #    mode: comptime
      pre_ldr_reg_writeback_unaligned:
        type: bool
        #     unconclusive - no effect
      pre_ldr_addr_sp:
        type: bool
        default: false # skopt: all top
        mode: comptime
      pre_ldr_addr_pc:
        type: bool


  lsu_nop:
    type: namespace
    ns:
      .confeature:
        doc: LDR/STR; skipped NOP in IT block -- post dependencies
      post_last_narrow:
        doc: Handling of the special case, when Decode sees only a narrow instruction.
        type: bool
        #        default: true
        # This is conclusive from sklearn with high confidence!

  last_narrow:
    type: namespace
    ns:
      after_narrow:
        type: bool
        default: true
      #        default: false
      after_writeback:
        type: bool
        default: true
        #        default: false
```

See `mm319369/icm/run_on_dask.py` for instructions how `skopt.Optimize` was used with running the tests on a cluster.
