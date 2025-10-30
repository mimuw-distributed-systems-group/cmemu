use super::register_bank::{RegisterBitmap, RegisterID, XPSR};
use crate::common::{BitstringUtils, SRType, Shift, Word, bitstring::constants as bsc};
use crate::{Bitstring, bitstring_concat, bitstring_extract};
use std::fmt;
use strum::IntoStaticStr;

// How to generate `Instruction` enum variant name?
// 1) Get proper section in [ARM-ARM] describing the instruction.
// 2) Get the "full" name of the instruction from the beginning
//    of the introductory paragraph.
// 3) Join the words into identifier,
//    treat words in parentheses as sub-instruction (denote it with '_' character).
//
// E.g.:
//   documentation of `LDR (register)` starts with "Load Register (literal)",
//   so the generated enum variant name is `LoadRegister_Literal`

// Used inside core (methods are pub(super)),
// but passed to CDL (so the type is pub(crate)).
/// ARMv7-M instruction representation
#[allow(non_camel_case_types, clippy::enum_variant_names)]
#[derive(Clone, Debug, IntoStaticStr)]
pub(crate) enum Instruction {
    /// Instruction not supported by the emulator.
    Unsupported { name: &'static str },
    /// [ARM-ARM] A5.1.1
    Undefined,
    /// [ARM-ARM] A5.1.1
    Unpredictable,
    /// [ARM-ARM] A7.7.1
    AddWithCarry_Immediate {
        rd: RegisterID,
        rn: RegisterID,
        imm32: Word,
        setflags: bool,
    },
    /// [ARM-ARM] A7.7.2
    AddWithCarry_Register {
        rd: RegisterID,
        rn: RegisterID,
        rm: RegisterID,
        shift: Shift,
        setflags: bool,
        setflags_depends_on_it: bool,
    },
    /// [ARM-ARM] A7.7.3
    Add_Immediate {
        rd: RegisterID,
        rn: RegisterID,
        imm32: Word,
        setflags: bool,
        setflags_depends_on_it: bool,
    },
    /// [ARM-ARM] A7.7.4
    Add_Register {
        rd: RegisterID,
        rn: RegisterID,
        rm: RegisterID,
        shift: Shift,
        setflags: bool,
        setflags_depends_on_it: bool,
    },
    /// [ARM-ARM] A7.7.5
    Add_SPPlusImmediate {
        rd: RegisterID,
        imm32: Word,
        setflags: bool,
    },
    /// [ARM-ARM] A7.7.6
    Add_SPPlusRegister {
        rd: RegisterID,
        rm: RegisterID,
        shift: Shift,
        setflags: bool,
    },
    /// [ARM-ARM] A7.7.7
    AddressToRegister {
        rd: RegisterID,
        imm32: Word,
        add: bool,
    },
    /// [ARM-ARM] A7.7.8
    And_Immediate {
        rd: RegisterID,
        rn: RegisterID,
        imm32: Word,
        setflags: bool,
        carry: bool,
    },
    /// [ARM-ARM] A7.7.9
    And_Register {
        rd: RegisterID,
        rn: RegisterID,
        rm: RegisterID,
        shift: Shift,
        setflags: bool,
        setflags_depends_on_it: bool,
    },
    /// [ARM-ARM] A7.7.10
    ArithmeticShiftRight_Immediate {
        rd: RegisterID,
        rm: RegisterID,
        shift: Shift,
        setflags: bool,
        setflags_depends_on_it: bool,
    },
    /// [ARM-ARM] A7.7.11
    ArithmeticShiftRight_Register {
        rd: RegisterID,
        rn: RegisterID,
        rm: RegisterID,
        setflags: bool,
        setflags_depends_on_it: bool,
    },
    /// [ARM-ARM] A7.7.12
    Branch { cond: Condition, imm32: Word },
    /// [ARM-ARM] A7.7.13
    BitFieldClear {
        rd: RegisterID,
        msbit: u8,
        lsbit: u8,
    },
    /// [ARM-ARM] A7.7.14
    BitFieldInsert {
        rd: RegisterID,
        rn: RegisterID,
        msbit: u8,
        lsbit: u8,
    },
    /// [ARM-ARM] A7.7.15
    BitClear_Immediate {
        rd: RegisterID,
        rn: RegisterID,
        imm32: Word,
        setflags: bool,
        carry: bool,
    },
    /// [ARM-ARM] A7.7.16
    BitClear_Register {
        rd: RegisterID,
        rn: RegisterID,
        rm: RegisterID,
        shift: Shift,
        setflags: bool,
        setflags_depends_on_it: bool,
    },
    /// [ARM-ARM] A7.7.17
    // TODO: add tests
    Breakpoint { imm32: Word },
    /// [ARM-ARM] A7.7.18
    BranchWithLink_Immediate { imm32: Word },
    /// [ARM-ARM] A7.7.19
    BranchWithLinkAndExchange_Register { rm: RegisterID },
    /// [ARM-ARM] A7.7.20
    BranchAndExchange { rm: RegisterID },
    /// [ARM-ARM] A7.7.21
    CompareAndBranch {
        rn: RegisterID,
        imm32: Word,
        nonzero: bool,
    },
    /// [ARM-ARM] A7.7.23
    // TODO: add tests
    ClearExclusive,
    /// [ARM-ARM] A7.7.24
    CountLeadingZeros { rd: RegisterID, rm: RegisterID },
    /// [ARM-ARM] A7.7.25
    CompareNegative_Immediate { rn: RegisterID, imm32: Word },
    /// [ARM-ARM] A7.7.26
    CompareNegative_Register {
        rn: RegisterID,
        rm: RegisterID,
        shift: Shift,
    },
    /// [ARM-ARM] A7.7.27
    Compare_Immediate { rn: RegisterID, imm32: Word },
    /// [ARM-ARM] A7.7.28
    Compare_Register {
        rn: RegisterID,
        rm: RegisterID,
        shift: Shift,
    },
    /// [ARM-ARM] A7.7.29
    /// [ARM-ARM] B5.2.1
    // TODO: add tests
    ChangeProcessorState {
        enable: bool,
        disable: bool,
        affect_pri: bool,
        affect_fault: bool,
    },
    /// [ARM-ARM] A7.7.33
    // TODO: add tests
    DataMemoryBarrier { option: Bitstring![4] },
    /// [ARM-ARM] A7.7.34
    // TODO: add tests
    DataSynchronizationBarrier { option: Bitstring![4] },
    /// [ARM-ARM] A7.7.35
    ExclusiveOr_Immediate {
        rd: RegisterID,
        rn: RegisterID,
        imm32: Word,
        setflags: bool,
        carry: bool,
    },
    /// [ARM-ARM] A7.7.36
    ExclusiveOr_Register {
        rd: RegisterID,
        rn: RegisterID,
        rm: RegisterID,
        shift: Shift,
        setflags: bool,
        setflags_depends_on_it: bool,
    },
    /// [ARM-ARM] A7.7.37
    InstructionSynchronizationBarrier { option: Bitstring![4] },
    /// [ARM-ARM] A7.7.38
    IfThen {
        firstcond: Condition,
        mask: Bitstring![4],
    },
    /// [ARM-ARM] A7.7.41
    LoadMultiple {
        rn: RegisterID,
        registers: RegisterBitmap,
        wback: bool,
        is_narrow: bool, // Additional information used by our model.
    },
    /// [ARM-ARM] A7.7.42
    LoadMultipleDecrementBefore {
        rn: RegisterID,
        registers: RegisterBitmap,
        wback: bool,
    },
    /// [ARM-ARM] A7.7.43
    LoadRegister_Immediate {
        rt: RegisterID,
        rn: RegisterID,
        imm32: Word,

        index: bool,
        add: bool,
        wback: bool,
    },
    /// [ARM-ARM] A7.7.44
    LoadRegister_Literal {
        rt: RegisterID,
        imm32: Word,
        add: bool,
    },
    /// [ARM-ARM] A7.7.45
    LoadRegister_Register {
        rt: RegisterID,
        rn: RegisterID,
        rm: RegisterID,
        shift: Shift,

        index: bool,
        add: bool,
        wback: bool,
    },
    /// [ARM-ARM] A7.7.46
    LoadRegisterByte_Immediate {
        rt: RegisterID,
        rn: RegisterID,
        imm32: Word,

        index: bool,
        add: bool,
        wback: bool,
    },
    /// [ARM-ARM] A7.7.47
    LoadRegisterByte_Literal {
        rt: RegisterID,
        imm32: Word,
        add: bool,
    },
    /// [ARM-ARM] A7.7.48
    LoadRegisterByte_Register {
        rt: RegisterID,
        rn: RegisterID,
        rm: RegisterID,
        shift: Shift,

        index: bool,
        add: bool,
        wback: bool,
    },
    /// [ARM-ARM] A7.7.49
    // TODO: add tests (including timing, AGU, pipelining)
    LoadRegisterByteUnprivileged {
        rt: RegisterID,
        rn: RegisterID,
        imm32: Word,
        // Note: fields `postindex`, `add`, and `register_form` are unused in docs.
    },
    /// [ARM-ARM] A7.7.50
    LoadRegisterDual_Immediate {
        rt: RegisterID,
        rt2: RegisterID,
        rn: RegisterID,
        imm32: Word,

        index: bool,
        add: bool,
        wback: bool,
    },
    /// [ARM-ARM] A7.7.51
    LoadRegisterDual_Literal {
        rt: RegisterID,
        rt2: RegisterID,
        imm32: Word,

        add: bool,
    },
    /// [ARM-ARM] A7.7.52
    // TODO: add tests (including timing, AGU, pipelining)
    LoadRegisterExclusive {
        rt: RegisterID,
        rn: RegisterID,
        imm32: Word,
    },
    /// [ARM-ARM] A7.7.53
    // TODO: add tests (including timing, AGU, pipelining)
    LoadRegisterExclusiveByte { rt: RegisterID, rn: RegisterID },
    /// [ARM-ARM] A7.7.54
    // TODO: add tests (including timing, AGU, pipelining)
    LoadRegisterExclusiveHalfword { rt: RegisterID, rn: RegisterID },
    /// [ARM-ARM] A7.7.55
    LoadRegisterHalfword_Immediate {
        rt: RegisterID,
        rn: RegisterID,
        imm32: Word,

        index: bool,
        add: bool,
        wback: bool,
    },
    /// [ARM-ARM] A7.7.56
    LoadRegisterHalfword_Literal {
        rt: RegisterID,
        imm32: Word,
        add: bool,
    },
    /// [ARM-ARM] A7.7.57
    LoadRegisterHalfword_Register {
        rt: RegisterID,
        rn: RegisterID,
        rm: RegisterID,
        shift: Shift,

        index: bool,
        add: bool,
        wback: bool,
    },
    /// [ARM-ARM] A7.7.58
    // TODO: add tests (including timing, AGU, pipelining)
    LoadRegisterHalfwordUnprivileged {
        rt: RegisterID,
        rn: RegisterID,
        imm32: Word,
        // Note: fields `postindex`, `add`, and `register_form` are unused in docs.
    },
    /// [ARM-ARM] A7.7.59
    LoadRegisterSignedByte_Immediate {
        rt: RegisterID,
        rn: RegisterID,
        imm32: Word,

        index: bool,
        add: bool,
        wback: bool,
    },
    /// [ARM-ARM] A7.7.60
    LoadRegisterSignedByte_Literal {
        rt: RegisterID,
        imm32: Word,
        add: bool,
    },
    /// [ARM-ARM] A7.7.61
    LoadRegisterSignedByte_Register {
        rt: RegisterID,
        rn: RegisterID,
        rm: RegisterID,
        shift: Shift,

        index: bool,
        add: bool,
        wback: bool,
    },
    /// [ARM-ARM] A7.7.62
    // TODO: add tests (including timing, AGU, pipelining)
    LoadRegisterSignedByteUnprivileged {
        rt: RegisterID,
        rn: RegisterID,
        imm32: Word,
        // Note: fields `postindex`, `add`, and `register_form` are unused in docs.
    },
    /// [ARM-ARM] A7.7.63
    LoadRegisterSignedHalfword_Immediate {
        rt: RegisterID,
        rn: RegisterID,
        imm32: Word,

        index: bool,
        add: bool,
        wback: bool,
    },
    /// [ARM-ARM] A7.7.64
    LoadRegisterSignedHalfword_Literal {
        rt: RegisterID,
        imm32: Word,
        add: bool,
    },
    /// [ARM-ARM] A7.7.65
    LoadRegisterSignedHalfword_Register {
        rt: RegisterID,
        rn: RegisterID,
        rm: RegisterID,
        shift: Shift,

        index: bool,
        add: bool,
        wback: bool,
    },
    /// [ARM-ARM] A7.7.66
    // TODO: add tests (including timing, AGU, pipelining)
    LoadRegisterSignedHalfwordUnprivileged {
        rt: RegisterID,
        rn: RegisterID,
        imm32: Word,
        // Note: fields `postindex`, `add`, and `register_form` are unused in docs.
    },
    /// [ARM-ARM] A7.7.67
    // TODO: add tests (including timing, AGU, pipelining)
    LoadRegisterUnprivileged {
        rt: RegisterID,
        rn: RegisterID,
        imm32: Word,
        // Note: fields `postindex`, `add`, and `register_form` are unused in docs.
    },
    /// [ARM-ARM] A7.7.68
    LogicalShiftLeft_Immediate {
        rd: RegisterID,
        rm: RegisterID,
        shift: Shift,
        setflags: bool,
        setflags_depends_on_it: bool,
    },
    /// [ARM-ARM] A7.7.69
    LogicalShiftLeft_Register {
        rd: RegisterID,
        rn: RegisterID,
        rm: RegisterID,
        setflags: bool,
        setflags_depends_on_it: bool,
    },
    /// [ARM-ARM] A7.7.70
    LogicalShiftRight_Immediate {
        rd: RegisterID,
        rm: RegisterID,
        shift: Shift,
        setflags: bool,
        setflags_depends_on_it: bool,
    },
    /// [ARM-ARM] A7.7.71
    LogicalShiftRight_Register {
        rd: RegisterID,
        rn: RegisterID,
        rm: RegisterID,
        setflags: bool,
        setflags_depends_on_it: bool,
    },
    /// [ARM-ARM] A7.7.74
    MultiplyAccumulate {
        rd: RegisterID,
        rn: RegisterID,
        rm: RegisterID,
        ra: RegisterID,
        setflags: bool,
    },
    /// [ARM-ARM] A7.7.75
    MultiplyAndSubtract {
        rd: RegisterID,
        rn: RegisterID,
        rm: RegisterID,
        ra: RegisterID,
    },
    /// [ARM-ARM] A7.7.76
    Move_Immediate {
        rd: RegisterID,
        imm32: Word,
        setflags: bool,
        setflags_depends_on_it: bool,
        carry: bool,
    },
    /// [ARM-ARM] A7.7.77
    Move_Register {
        rd: RegisterID,
        rm: RegisterID,
        setflags: bool,
    },
    /// [ARM-ARM] A7.7.79
    MoveTop {
        rd: RegisterID,
        imm16: Bitstring![16],
    },
    /// [ARM-ARM] A7.7.82 and B5.2.2
    MoveToRegisterFromSpecialRegister { rd: RegisterID, sysm: Bitstring![8] },
    /// [ARM-ARM] A7.7.83 and B5.2.3
    MoveToSpecialRegisterFromARMRegister {
        rn: RegisterID,
        sysm: Bitstring![8],
        mask: Bitstring![2],
    },
    /// [ARM-ARM] A7.7.84
    Multiply {
        rd: RegisterID,
        rn: RegisterID,
        rm: RegisterID,
        setflags: bool,
        setflags_depends_on_it: bool,
    },
    /// [ARM-ARM] A7.7.85
    BitwiseNot_Immediate {
        rd: RegisterID,
        imm32: Word,
        setflags: bool,
        carry: bool,
    },
    /// [ARM-ARM] A7.7.86
    BitwiseNot_Register {
        rd: RegisterID,
        rm: RegisterID,
        shift: Shift,
        setflags: bool,
        setflags_depends_on_it: bool,
    },
    /// [ARM-ARM] A7.7.88
    NoOperation,
    /// [ARM-ARM] A7.7.89
    LogicalOrNot_Immediate {
        rd: RegisterID,
        rn: RegisterID,
        imm32: Word,
        setflags: bool,
        carry: bool,
    },
    /// [ARM-ARM] A7.7.90
    LogicalOrNot_Register {
        rd: RegisterID,
        rn: RegisterID,
        rm: RegisterID,
        shift: Shift,
        setflags: bool,
    },
    /// [ARM-ARM] A7.7.91
    LogicalOr_Immediate {
        rd: RegisterID,
        rn: RegisterID,
        imm32: Word,
        setflags: bool,
        carry: bool,
    },
    /// [ARM-ARM]  A7.7.92
    LogicalOr_Register {
        rd: RegisterID,
        rn: RegisterID,
        rm: RegisterID,
        shift: Shift,
        setflags: bool,
        setflags_depends_on_it: bool,
    },
    // [ARM-ARM] A7.7.99 - POP is not a real instruction.
    //     It is a shortcut notation for `ldr _, [sp], 4` or `ldm sp!, _`
    //     (depending on the encoding).
    //
    //     There are some arguments for this:
    //      * Majority of POP encodings are special cases of `ldr (immediate)` and `ldm` encodings.
    //      * `cmemu-tests/tests/flash/instructions/memory/push_pop_pipelining.asm` test,
    //        which shows that some encodings of POP pipelines (same as `ldr (immediate)`),
    //        and some do not (same as `ldm`).
    //      * "Assembler syntax" of POP says that `ldm sp!, <registers>` is an alternative syntax.
    //      * `ldr (immediate)` and `ldm` encodings references `pop` instruction.
    //
    //     Thus, we decode `pop` as one of the following:
    //      * [ARM-ARM] A7.7.41 `LoadMultiple`
    //      * [ARM-ARM] A7.7.43 `LoadRegister_Immediate`

    // [ARM-ARM] A7.7.101 - PUSH is not a real instruction.
    //     It is a shortcut notation for `str _, [sp, -4]!` or `stmdb sp!, _`
    //     (depending on the encoding)
    //
    //     There are some arguments for this:
    //      * Majority of PUSH encodings are special cases of `str (immediate)` and `stmdb` encodings.
    //      * `cmemu-tests/tests/flash/instructions/memory/push_pop_pipelining.asm` test,
    //        which shows that some encodings of PUSH pipelines (same as `str (immediate)`),
    //        and some do not (same as `stmdb`).
    //      * "Assembler syntax" of PUSH says that `stmdb sp!, <registers>` is an alternative syntax.
    //      * `str (immediate)` and `stmdb` encodings references `push` instruction.
    //
    //     Thus, we decode `push` as one of the following:
    //      * [ARM-ARM] A7.7.160 `StoreMultipleDecrementBefore`
    //      * [ARM-ARM] A7.7.161 `StoreRegister_Immediate`
    /// [ARM-ARM] A7.7.112
    ReverseBits { rd: RegisterID, rm: RegisterID },
    /// [ARM-ARM] A7.7.113
    ByteReverseWord { rd: RegisterID, rm: RegisterID },
    /// [ARM-ARM] A7.7.114
    ByteReversePackedHalfword { rd: RegisterID, rm: RegisterID },
    /// [ARM-ARM] A7.7.115
    ByteReverseSignedHalfword { rd: RegisterID, rm: RegisterID },
    /// [ARM-ARM] A7.7.116
    RotateRight_Immediate {
        rd: RegisterID,
        rm: RegisterID,
        shift: Shift,
        setflags: bool,
    },
    /// [ARM-ARM] A7.7.117
    RotateRight_Register {
        rd: RegisterID,
        rn: RegisterID,
        rm: RegisterID,
        setflags: bool,
        setflags_depends_on_it: bool,
    },
    /// [ARM-ARM] A7.7.118
    RotateRightWithExtend {
        rd: RegisterID,
        rm: RegisterID,
        setflags: bool,
    },
    /// [ARM-ARM] A7.7.119
    ReverseSubtract_Immediate {
        rd: RegisterID,
        rn: RegisterID,
        imm32: Word,
        setflags: bool,
        setflags_depends_on_it: bool,
    },
    /// [ARM-ARM] A7.7.120
    ReverseSubtract_Register {
        rd: RegisterID,
        rn: RegisterID,
        rm: RegisterID,
        shift: Shift,
        setflags: bool,
    },
    /// [ARM-ARM] A7.7.124
    SubtractWithCarry_Immediate {
        rd: RegisterID,
        rn: RegisterID,
        imm32: Word,
        setflags: bool,
    },
    /// [ARM-ARM] A7.7.125
    SubtractWithCarry_Register {
        rd: RegisterID,
        rn: RegisterID,
        rm: RegisterID,
        shift: Shift,
        setflags: bool,
        setflags_depends_on_it: bool,
    },
    /// [ARM-ARM] A7.7.126
    SignedBitFieldExtract {
        rn: RegisterID,
        rd: RegisterID,
        lsbit: u8,
        widthminus1: u8,
    },
    /// [ARM-ARM] A7.7.127
    SignedDivide {
        rd: RegisterID,
        rn: RegisterID,
        rm: RegisterID,
    },
    /// [ARM-ARM] A7.7.129
    // TODO: add tests
    SendEvent,
    /// [ARM-ARM] A7.7.138
    SignedMultiplyAccumulateLong {
        rn: RegisterID,
        rm: RegisterID,
        rd_hi: RegisterID,
        rd_lo: RegisterID,
    },
    /// [ARM-ARM] A7.7.149
    SignedMultiplyLong {
        rn: RegisterID,
        rm: RegisterID,
        rd_hi: RegisterID,
        rd_lo: RegisterID,
    },
    /// [ARM-ARM] A7.7.152
    SignedSaturate {
        rd: RegisterID,
        rn: RegisterID,
        saturate_to: u8,
        shift: Shift,
    },
    /// [ARM-ARM] A7.7.159
    StoreMultiple {
        rn: RegisterID,
        registers: RegisterBitmap,
        wback: bool,
    },
    /// [ARM-ARM] A7.7.160
    StoreMultipleDecrementBefore {
        rn: RegisterID,
        registers: RegisterBitmap,
        wback: bool,
    },
    /// [ARM-ARM] A7.7.161
    StoreRegister_Immediate {
        rt: RegisterID,
        rn: RegisterID,
        imm32: Word,

        index: bool,
        add: bool,
        wback: bool,
    },
    /// [ARM-ARM] A7.7.162
    StoreRegister_Register {
        rt: RegisterID,
        rn: RegisterID,
        rm: RegisterID,
        shift: Shift,

        index: bool,
        add: bool,
        wback: bool,
    },
    /// [ARM-ARM] A7.7.163
    StoreRegisterByte_Immediate {
        rt: RegisterID,
        rn: RegisterID,
        imm32: Word,

        index: bool,
        add: bool,
        wback: bool,
    },
    /// [ARM-ARM] A7.7.164
    StoreRegisterByte_Register {
        rt: RegisterID,
        rn: RegisterID,
        rm: RegisterID,
        shift: Shift,

        index: bool,
        add: bool,
        wback: bool,
    },
    /// [ARM-ARM] A7.7.165
    // TODO: add tests (including timing, AGU, pipelining)
    StoreRegisterByteUnprivileged {
        rt: RegisterID,
        rn: RegisterID,
        imm32: Word,
        // Note: fields `postindex`, `add`, and `register_form` are unused in docs.
    },
    /// [ARM-ARM] A7.7.166
    StoreRegisterDual_Immediate {
        rt: RegisterID,
        rt2: RegisterID,
        rn: RegisterID,
        imm32: Word,

        index: bool,
        add: bool,
        wback: bool,
    },
    /// [ARM-ARM] A7.7.167
    // TODO: add tests (including timing, AGU, pipelining)
    StoreRegisterExclusive {
        rd: RegisterID,
        rt: RegisterID,
        rn: RegisterID,
        imm32: Word,
    },
    /// [ARM-ARM] A7.7.168
    // TODO: add tests (including timing, AGU, pipelining)
    StoreRegisterExclusiveByte {
        rd: RegisterID,
        rt: RegisterID,
        rn: RegisterID,
    },
    /// [ARM-ARM] A7.7.169
    // TODO: add tests (including timing, AGU, pipelining)
    StoreRegisterExclusiveHalfword {
        rd: RegisterID,
        rt: RegisterID,
        rn: RegisterID,
    },
    /// [ARM-ARM] A7.7.170
    StoreRegisterHalfword_Immediate {
        rt: RegisterID,
        rn: RegisterID,
        imm32: Word,

        index: bool,
        add: bool,
        wback: bool,
    },
    /// [ARM-ARM] A7.7.171
    StoreRegisterHalfword_Register {
        rt: RegisterID,
        rn: RegisterID,
        rm: RegisterID,
        shift: Shift,

        index: bool,
        add: bool,
        wback: bool,
    },
    /// [ARM-ARM] A7.7.172
    // todo: add tests
    StoreRegisterHalfwordUnprivileged {
        rt: RegisterID,
        rn: RegisterID,
        imm32: Word,
        // Note: fields `postindex`, `add`, and `register_form` are unused in docs.
    },
    /// [ARM-ARM] A7.7.173
    // todo: add tests
    StoreRegisterUnprivileged {
        rt: RegisterID,
        rn: RegisterID,
        imm32: Word,
        // Note: fields `postindex`, `add`, and `register_form` are unused in docs.
    },
    /// [ARM-ARM] A7.7.174
    Subtract_Immediate {
        rd: RegisterID,
        rn: RegisterID,
        imm32: Word,
        setflags: bool,
        setflags_depends_on_it: bool,
    },
    /// [ARM-ARM] A7.7.175
    Subtract_Register {
        rd: RegisterID,
        rn: RegisterID,
        rm: RegisterID,
        shift: Shift,
        setflags: bool,
        setflags_depends_on_it: bool,
    },
    /// [ARM-ARM] A7.7.176
    Subtract_SPMinusImmediate {
        rd: RegisterID,
        imm32: Word,
        setflags: bool,
    },
    /// [ARM-ARM] A7.7.177
    Subtract_SPMinusRegister {
        rd: RegisterID,
        rm: RegisterID,
        shift: Shift,
        setflags: bool,
    },
    /// [ARM-ARM] A7.7.178
    SupervisorCall { imm32: Word },
    /// [ARM-ARM] A7.7.182
    SignedExtendByte {
        rd: RegisterID,
        rm: RegisterID,
        rotation: Shift,
    },
    /// [ARM-ARM] A7.7.184
    SignedExtendHalfword {
        rd: RegisterID,
        rm: RegisterID,
        rotation: Shift,
    },
    /// [ARM-ARM] A7.7.185
    TableBranch {
        rn: RegisterID,
        rm: RegisterID,
        is_tbh: bool,
    },
    /// [ARM-ARM] A7.7.186
    TestEquivalence_Immediate {
        rn: RegisterID,
        imm32: Word,
        carry: bool,
    },
    /// [ARM-ARM] A7.7.187
    TestEquivalence_Register {
        rn: RegisterID,
        rm: RegisterID,
        shift: Shift,
    },
    /// [ARM-ARM] A7.7.188
    Test_Immediate {
        rn: RegisterID,
        imm32: Word,
        carry: bool,
    },
    /// [ARM-ARM] A7.7.189
    Test_Register {
        rn: RegisterID,
        rm: RegisterID,
        shift: Shift,
    },
    /// [ARM-ARM] A7.7.193
    UnsignedBitFieldExtract {
        rd: RegisterID,
        rn: RegisterID,
        lsbit: u8,
        widthminus1: u8,
    },
    /// [ARM-ARM] A7.7.194
    PermanentlyUndefined { imm32: Word },
    /// [ARM-ARM] A7.7.195
    UnsignedDivide {
        rd: RegisterID,
        rn: RegisterID,
        rm: RegisterID,
    },
    /// [ARM-ARM] A7.7.203
    UnsignedMultiplyAccumulateLong {
        rn: RegisterID,
        rm: RegisterID,
        rd_hi: RegisterID,
        rd_lo: RegisterID,
    },
    /// [ARM-ARM] A7.7.204
    UnsignedMultiplyLong {
        rn: RegisterID,
        rm: RegisterID,
        rd_hi: RegisterID,
        rd_lo: RegisterID,
    },
    /// [ARM-ARM] A7.7.213
    UnsignedSaturate {
        rd: RegisterID,
        rn: RegisterID,
        saturate_to: u8,
        shift: Shift,
    },
    /// [ARM-ARM] A7.7.221
    UnsignedExtendByte {
        rd: RegisterID,
        rm: RegisterID,
        rotation: Shift,
    },
    /// [ARM-ARM] A7.7.223
    UnsignedExtendHalfword {
        rd: RegisterID,
        rm: RegisterID,
        rotation: Shift,
    },
    /// [ARM-ARM] A7.7.261
    WaitForEvent,
    /// [ARM-ARM] A7.7.262
    WaitForInterrupt,
}

// Used inside core (methods are pub(super)),
// but passed to CDL as part of Instruction (so the type is pub(crate)).
/// Newtype helper type for branch and if-then conditions.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Condition(pub(super) Bitstring![4]);

/// Describes some properties of instructions operating on memory
pub(super) enum MemoryInstructionDescription {
    None,
    LoadSingle {
        rt: RegisterID,
        register_offset: bool,
        writeback: bool,
    },
    StoreSingle {
        rt: RegisterID,
        register_offset: bool,
        writeback: bool,
    },
    LoadMultiple {
        writeback: bool,
        is_narrow_ldm: bool,
    },
    StoreMultiple {
        writeback: bool,
    },
}

impl Instruction {
    #[allow(clippy::too_many_lines)]
    #[allow(clippy::match_same_arms)]
    #[allow(dead_code)]
    pub(super) const fn get_read_registers(&self) -> RegisterBitmap {
        match self {
            // TODO: MoveToSpecialRegisterFromARMRegister
            | Self::Unsupported { .. }
            | Self::Undefined
            | Self::Unpredictable
            | Self::Breakpoint { .. }
            | Self::ChangeProcessorState { .. }
            | Self::ClearExclusive
            | Self::DataSynchronizationBarrier { .. }
            | Self::DataMemoryBarrier { .. }
            | Self::InstructionSynchronizationBarrier { .. }
            | Self::IfThen { .. }
            | Self::Move_Immediate {  .. }
            | Self::MoveToRegisterFromSpecialRegister {  .. }
            | Self::BitwiseNot_Immediate { .. }
            | Self::NoOperation
            | Self::WaitForEvent
            | Self::WaitForInterrupt
            | Self::PermanentlyUndefined { .. }
            | Self::SupervisorCall { .. }
            | Self::SendEvent => RegisterBitmap::new(),

            // Simple branches & co.
            Self::Branch { .. }
            | Self::BranchWithLink_Immediate { .. }
            | Self::AddressToRegister { .. }
            => RegisterBitmap::new().with(RegisterID::PC, true),

            // No need for PC (from the arch perspective)
            | Self::BranchAndExchange { rm } => RegisterBitmap::new().with(*rm, true),

            Self::CompareAndBranch { rn, .. }
            => RegisterBitmap::new().with(*rn, true).with(RegisterID::PC, true),
            // PC is written to LR
            Self::BranchWithLinkAndExchange_Register { rm } => RegisterBitmap::new()
                .with(*rm, true)
                .with(RegisterID::PC, true),

            // TBH/TBB is PC-relative
            | Self::TableBranch { rn, rm, .. } => RegisterBitmap::new()
                .with(*rn, true)
                .with(*rm, true)
                .with(RegisterID::PC, true),

            // Special
            | Self::MoveToSpecialRegisterFromARMRegister { rn: reg, .. }
            // Regular ALU IMM
            | Self::AddWithCarry_Immediate { rn: reg, .. }
            | Self::Add_Immediate { rn: reg, .. }
            | Self::And_Immediate { rn: reg, .. }
            | Self::ArithmeticShiftRight_Immediate { rm: reg, .. }
            | Self::BitFieldClear { rd: reg, .. } // This is inplace op
            | Self::BitClear_Immediate { rn: reg, .. }
            | Self::CountLeadingZeros { rm: reg, .. }
            | Self::CompareNegative_Immediate { rn: reg, .. }
            | Self::Compare_Immediate { rn: reg, .. }
            | Self::ExclusiveOr_Immediate { rn: reg, .. }
            | Self::LogicalShiftLeft_Immediate { rm: reg, .. }
            | Self::LogicalShiftRight_Immediate { rm: reg, .. }
            | Self::Move_Register { rm: reg, .. }
            | Self::MoveTop { rd: reg, .. } // inplace op
            | Self::BitwiseNot_Register { rm: reg, .. }
            | Self::LogicalOrNot_Immediate { rn: reg, .. }
            | Self::LogicalOr_Immediate { rn: reg, .. }
            | Self::ReverseBits { rm: reg, .. }
            | Self::ByteReverseWord { rm: reg, .. }
            | Self::ByteReversePackedHalfword { rm: reg, .. }
            | Self::ByteReverseSignedHalfword { rm: reg, .. }
            | Self::RotateRight_Immediate { rm: reg, .. }
            | Self::RotateRightWithExtend { rm: reg, .. }
            | Self::ReverseSubtract_Immediate { rn: reg, .. }
            | Self::SubtractWithCarry_Immediate { rn: reg, .. }
            | Self::SignedBitFieldExtract { rn: reg, .. }
            | Self::SignedSaturate { rn: reg, .. }
            | Self::Subtract_Immediate { rn: reg, .. }
            | Self::SignedExtendByte { rm: reg, .. }
            | Self::SignedExtendHalfword { rm: reg, .. }
            | Self::TestEquivalence_Immediate { rn: reg, .. }
            | Self::Test_Immediate { rn: reg, .. }
            | Self::UnsignedBitFieldExtract { rn: reg, .. }
            | Self::UnsignedSaturate { rn: reg, .. }
            | Self::UnsignedExtendByte { rm: reg, .. }
            | Self::UnsignedExtendHalfword { rm: reg, .. }
             => RegisterBitmap::new().with(*reg, true),

            // Special-cased ALU SP+IMM
            | Self::Add_SPPlusImmediate {  .. }
            | Self::Subtract_SPMinusImmediate { .. }
            => RegisterBitmap::new().with(RegisterID::SP, true),

            // Regular ALU Register (two registers read)
            | Self::AddWithCarry_Register { rn, rm, .. }
            | Self::Add_Register { rn, rm, .. }
            | Self::And_Register { rn, rm, .. }
            | Self::ArithmeticShiftRight_Register { rn, rm, .. }
            | Self::BitFieldInsert { rd: rm, rn, .. } // inplace op
            | Self::BitClear_Register { rn, rm, .. }
            | Self::CompareNegative_Register { rn, rm, .. }
            | Self::Compare_Register { rn, rm, .. }
            | Self::ExclusiveOr_Register { rn, rm, .. }
            | Self::LogicalShiftLeft_Register { rn, rm, .. }
            | Self::LogicalShiftRight_Register { rn, rm, .. }
            | Self::Multiply { rn, rm, .. }
            | Self::LogicalOrNot_Register { rn, rm, .. }
            | Self::LogicalOr_Register { rn, rm, .. }
            | Self::RotateRight_Register { rn, rm, .. }
            | Self::ReverseSubtract_Register { rn, rm, .. }
            | Self::SubtractWithCarry_Register { rn, rm, .. }
            | Self::SignedDivide { rn, rm, .. }
            | Self::SignedMultiplyLong { rn, rm, .. }
            | Self::Subtract_Register { rn, rm, .. }
            | Self::TestEquivalence_Register { rn, rm, .. }
            | Self::Test_Register { rn, rm, .. }
            | Self::UnsignedDivide { rn, rm, .. }
            | Self::UnsignedMultiplyLong { rn, rm, .. }
            => RegisterBitmap::new() .with(*rn, true) .with(*rm, true),

            // Special-cased ALU SP+Reg
            | Self::Subtract_SPMinusRegister { rm, .. }
            | Self::Add_SPPlusRegister { rm, .. }
            => RegisterBitmap::new().with(RegisterID::SP, true).with(*rm, true),


            // 3+ register ALU
            | Self::MultiplyAccumulate { rn, rm, ra, .. }
            | Self::MultiplyAndSubtract { rn, rm, ra, .. } =>
                RegisterBitmap::new() .with(*rn, true) .with(*rm, true).with(*ra, true),

            Self::SignedMultiplyAccumulateLong { rn, rm, rd_hi, rd_lo, .. }
            | Self::UnsignedMultiplyAccumulateLong { rn, rm, rd_hi, rd_lo, .. } =>
                RegisterBitmap::new() .with(*rn, true) .with(*rm, true).with(*rd_hi, true).with(*rd_lo, true),


            // LSU: Loads IMM
            | Self::LoadMultiple { rn, .. }
            | Self::LoadMultipleDecrementBefore { rn, .. }
            | Self::LoadRegister_Immediate { rn, .. }
            | Self::LoadRegisterByte_Immediate { rn, .. }
            | Self::LoadRegisterSignedByte_Immediate { rn, .. }
            | Self::LoadRegisterHalfword_Immediate { rn, .. }
            | Self::LoadRegisterSignedHalfword_Immediate { rn, .. }
            | Self::LoadRegisterDual_Immediate { rn, .. }
            | Self::LoadRegisterUnprivileged { rn, .. }
            | Self::LoadRegisterByteUnprivileged { rn, .. }
            | Self::LoadRegisterHalfwordUnprivileged { rn, .. }
            | Self::LoadRegisterSignedByteUnprivileged { rn, .. }
            | Self::LoadRegisterSignedHalfwordUnprivileged { rn, .. }
            | Self::LoadRegisterExclusive { rn, .. }
            | Self::LoadRegisterExclusiveByte { rn, .. }
            | Self::LoadRegisterExclusiveHalfword { rn, .. }
            => RegisterBitmap::new().with(*rn, true),

            // Loads Literal (PC + imm)
            | Self::LoadRegister_Literal { .. }
            | Self::LoadRegisterByte_Literal { .. }
            | Self::LoadRegisterHalfword_Literal { .. }
            | Self::LoadRegisterSignedByte_Literal { .. }
            | Self::LoadRegisterSignedHalfword_Literal {  .. }
            | Self::LoadRegisterDual_Literal { .. }
            => RegisterBitmap::new().with(RegisterID::PC, true),

            // LDR two registers
            | Self::LoadRegister_Register { rn, rm, .. }
            | Self::LoadRegisterByte_Register {rn, rm, .. }
            | Self::LoadRegisterHalfword_Register {rn, rm, .. }
            | Self::LoadRegisterSignedByte_Register {rn, rm, .. }
            | Self::LoadRegisterSignedHalfword_Register {rn, rm, .. }
            => RegisterBitmap::new().with(*rn, true).with(*rm, true),

            // STR that use <=2 registers (can be fully decoded)
            | Self::StoreRegister_Immediate { rt, rn,  .. }
            | Self::StoreRegisterByte_Immediate { rt, rn, .. }
            | Self::StoreRegisterHalfword_Immediate {rt,  rn, .. }
            | Self::StoreRegisterUnprivileged { rt, rn, .. }
            | Self::StoreRegisterByteUnprivileged { rt, rn, .. }
            | Self::StoreRegisterHalfwordUnprivileged { rt, rn, .. }
            | Self::StoreRegisterExclusive { rt, rn, .. }
            | Self::StoreRegisterExclusiveByte { rt, rn, .. }
            | Self::StoreRegisterExclusiveHalfword { rt, rn, .. }
            => RegisterBitmap::new().with(*rn, true).with(*rt, true),

            // Stores that must fetch stored vals later than D
            | Self::StoreMultiple { rn, registers, .. }
            | Self::StoreMultipleDecrementBefore { rn, registers, .. }
            => registers.with(*rn, true),


            Self::StoreRegisterDual_Immediate { rt, rt2, rn, .. }
            => RegisterBitmap::new().with(*rt, true).with(*rt2, true).with(*rn, true),

            // STR (register) - always reads 3 registers
            | Self::StoreRegister_Register { rt, rn, rm, .. }
            | Self::StoreRegisterByte_Register { rt, rn, rm, .. }
            | Self::StoreRegisterHalfword_Register { rt, rn, rm, .. }
            => RegisterBitmap::new().with(*rt, true).with(*rn, true).with(*rm, true),
        }
    }

    #[allow(clippy::too_many_lines)]
    pub(super) const fn get_written_registers(&self) -> RegisterBitmap {
        match self {
            // TODO: MoveToSpecialRegisterFromARMRegister
            Self::Unsupported { .. }
            | Self::Undefined
            | Self::Unpredictable
            | Self::Breakpoint { .. }
            | Self::ChangeProcessorState { .. }
            | Self::MoveToSpecialRegisterFromARMRegister { .. }
            | Self::ClearExclusive
            | Self::DataSynchronizationBarrier { .. }
            | Self::DataMemoryBarrier { .. }
            // TODO: should those also halt fetching? (is it writing PC?) -> need tests
            | Self::InstructionSynchronizationBarrier { .. }
            | Self::IfThen { .. }
            | Self::NoOperation
            | Self::WaitForEvent
            | Self::WaitForInterrupt
            | Self::SendEvent
            | Self::CompareNegative_Immediate { .. }
            | Self::CompareNegative_Register { .. }
            | Self::Compare_Register { .. }
            | Self::Compare_Immediate { .. }
            | Self::TestEquivalence_Immediate { .. }
            | Self::TestEquivalence_Register { .. }
            | Self::Test_Immediate { .. }
            | Self::Test_Register { .. }
            | Self::PermanentlyUndefined { .. }
            | Self::SupervisorCall { .. }
            | Self::StoreRegisterUnprivileged { .. }
            | Self::StoreRegisterByteUnprivileged { .. }
            | Self::StoreRegisterHalfwordUnprivileged { .. } => RegisterBitmap::new(),

            Self::Branch { .. }
            | Self::CompareAndBranch { .. }
            | Self::BranchAndExchange { .. }
            | Self::TableBranch { .. } => RegisterBitmap::new().with(RegisterID::PC, true),

            Self::BranchWithLink_Immediate { .. }
            | Self::BranchWithLinkAndExchange_Register { .. } => RegisterBitmap::new()
                .with(RegisterID::LR, true)
                .with(RegisterID::PC, true),

            Self::LoadMultiple {
                rn,
                registers,
                wback,
                ..
            }
            | Self::LoadMultipleDecrementBefore {
                rn,
                registers,
                wback,
            } => {
                if *wback {
                    registers.with(*rn, true)
                } else {
                    *registers
                }
            }
            Self::StoreMultiple { rn, wback, .. }
            | Self::StoreMultipleDecrementBefore { rn, wback, .. } => {
                RegisterBitmap::new().with(*rn, *wback)
            }

            Self::AddressToRegister { rd, .. }
            | Self::CountLeadingZeros { rd, .. }
            | Self::Move_Immediate { rd, .. }
            | Self::Move_Register { rd, .. }
            | Self::MoveTop { rd, .. }
            | Self::LogicalOrNot_Immediate { rd, .. }
            | Self::LogicalOrNot_Register { rd, .. }
            | Self::LogicalOr_Immediate { rd, .. }
            | Self::LogicalOr_Register { rd, .. }
            | Self::RotateRight_Immediate { rd, .. }
            | Self::RotateRight_Register { rd, .. }
            | Self::RotateRightWithExtend { rd, .. }
            | Self::ReverseSubtract_Immediate { rd, .. }
            | Self::ReverseSubtract_Register { rd, .. }
            | Self::SubtractWithCarry_Immediate { rd, .. }
            | Self::SubtractWithCarry_Register { rd, .. }
            | Self::SignedSaturate { rd, .. }
            | Self::AddWithCarry_Immediate { rd, .. }
            | Self::AddWithCarry_Register { rd, .. }
            | Self::Add_Immediate { rd, .. }
            | Self::Add_Register { rd, .. }
            | Self::Add_SPPlusImmediate { rd, .. }
            | Self::Add_SPPlusRegister { rd, .. }
            | Self::And_Immediate { rd, .. }
            | Self::And_Register { rd, .. }
            | Self::ArithmeticShiftRight_Immediate { rd, .. }
            | Self::ArithmeticShiftRight_Register { rd, .. }
            | Self::BitFieldClear { rd, .. }
            | Self::BitFieldInsert { rd, .. }
            | Self::BitClear_Immediate { rd, .. }
            | Self::BitClear_Register { rd, .. }
            | Self::ExclusiveOr_Immediate { rd, .. }
            | Self::ExclusiveOr_Register { rd, .. }
            | Self::LogicalShiftLeft_Immediate { rd, .. }
            | Self::LogicalShiftLeft_Register { rd, .. }
            | Self::LogicalShiftRight_Immediate { rd, .. }
            | Self::LogicalShiftRight_Register { rd, .. }
            | Self::MoveToRegisterFromSpecialRegister { rd, .. }
            | Self::Subtract_Immediate { rd, .. }
            | Self::Subtract_Register { rd, .. }
            | Self::Subtract_SPMinusImmediate { rd, .. }
            | Self::Subtract_SPMinusRegister { rd, .. }
            | Self::SignedExtendByte { rd, .. }
            | Self::SignedExtendHalfword { rd, .. }
            | Self::MultiplyAccumulate { rd, .. }
            | Self::MultiplyAndSubtract { rd, .. }
            | Self::Multiply { rd, .. }
            | Self::BitwiseNot_Immediate { rd, .. }
            | Self::BitwiseNot_Register { rd, .. }
            | Self::ReverseBits { rd, .. }
            | Self::ByteReverseWord { rd, .. }
            | Self::SignedBitFieldExtract { rd, .. }
            | Self::UnsignedBitFieldExtract { rd, .. }
            | Self::ByteReversePackedHalfword { rd, .. }
            | Self::ByteReverseSignedHalfword { rd, .. }
            | Self::UnsignedSaturate { rd, .. }
            | Self::UnsignedExtendByte { rd, .. }
            | Self::UnsignedExtendHalfword { rd, .. }
            | Self::UnsignedDivide { rd, .. }
            | Self::SignedDivide { rd, .. }
            | Self::StoreRegisterExclusive { rd, .. }
            | Self::StoreRegisterExclusiveByte { rd, .. }
            | Self::StoreRegisterExclusiveHalfword { rd, .. } => {
                RegisterBitmap::new().with(*rd, true)
            }

            Self::LoadRegister_Literal { rt, .. }
            | Self::LoadRegisterByte_Literal { rt, .. }
            | Self::LoadRegisterHalfword_Literal { rt, .. }
            | Self::LoadRegisterSignedByte_Literal { rt, .. }
            | Self::LoadRegisterSignedHalfword_Literal { rt, .. }
            | Self::LoadRegisterUnprivileged { rt, .. }
            | Self::LoadRegisterByteUnprivileged { rt, .. }
            | Self::LoadRegisterHalfwordUnprivileged { rt, .. }
            | Self::LoadRegisterSignedByteUnprivileged { rt, .. }
            | Self::LoadRegisterSignedHalfwordUnprivileged { rt, .. }
            | Self::LoadRegisterExclusive { rt, .. }
            | Self::LoadRegisterExclusiveByte { rt, .. }
            | Self::LoadRegisterExclusiveHalfword { rt, .. } => {
                RegisterBitmap::new().with(*rt, true)
            }

            Self::LoadRegister_Immediate { rt, rn, wback, .. }
            | Self::LoadRegister_Register { rt, rn, wback, .. }
            | Self::LoadRegisterByte_Immediate { rt, rn, wback, .. }
            | Self::LoadRegisterByte_Register { rt, rn, wback, .. }
            | Self::LoadRegisterHalfword_Immediate { rt, rn, wback, .. }
            | Self::LoadRegisterHalfword_Register { rt, rn, wback, .. }
            | Self::LoadRegisterSignedByte_Immediate { rt, rn, wback, .. }
            | Self::LoadRegisterSignedByte_Register { rt, rn, wback, .. }
            | Self::LoadRegisterSignedHalfword_Immediate { rt, rn, wback, .. }
            | Self::LoadRegisterSignedHalfword_Register { rt, rn, wback, .. } => {
                if *wback {
                    RegisterBitmap::new().with(*rt, true).with(*rn, true)
                } else {
                    RegisterBitmap::new().with(*rt, true)
                }
            }

            Self::LoadRegisterDual_Immediate {
                rt, rt2, rn, wback, ..
            } => {
                if *wback {
                    RegisterBitmap::new()
                        .with(*rt, true)
                        .with(*rt2, true)
                        .with(*rn, true)
                } else {
                    RegisterBitmap::new().with(*rt, true).with(*rt2, true)
                }
            }
            Self::LoadRegisterDual_Literal { rt, rt2, .. } => {
                RegisterBitmap::new().with(*rt, true).with(*rt2, true)
            }

            Self::StoreRegister_Immediate { rn, wback, .. }
            | Self::StoreRegister_Register { rn, wback, .. }
            | Self::StoreRegisterByte_Immediate { rn, wback, .. }
            | Self::StoreRegisterByte_Register { rn, wback, .. }
            | Self::StoreRegisterHalfword_Immediate { rn, wback, .. }
            | Self::StoreRegisterHalfword_Register { rn, wback, .. }
            | Self::StoreRegisterDual_Immediate { rn, wback, .. } => {
                if *wback {
                    RegisterBitmap::new().with(*rn, true)
                } else {
                    RegisterBitmap::new()
                }
            }

            Self::SignedMultiplyAccumulateLong { rd_hi, rd_lo, .. }
            | Self::SignedMultiplyLong { rd_hi, rd_lo, .. }
            | Self::UnsignedMultiplyAccumulateLong { rd_hi, rd_lo, .. }
            | Self::UnsignedMultiplyLong { rd_hi, rd_lo, .. } => {
                RegisterBitmap::new().with(*rd_hi, true).with(*rd_lo, true)
            }
        }
    }

    /// Returns a pair of (`does_set_any_of_NZCV`, `does_it_depends_on_being_in_it`).
    /// The Q flag and ``GE[3:0]`` are not considered here.
    #[allow(clippy::too_many_lines, clippy::match_same_arms)]
    pub(super) fn does_set_flags(&self) -> (bool, bool) {
        // TODO: const fn (bitstring)
        // TODO: Consider marking WHICH flags may be set! (e.g. TST cannot set V)
        //       And saturating instructions set the special sticky Q flag.
        match self {
            Self::AddWithCarry_Register {
                setflags,
                setflags_depends_on_it,
                ..
            }
            | Self::Add_Immediate {
                setflags,
                setflags_depends_on_it,
                ..
            }
            | Self::Add_Register {
                setflags,
                setflags_depends_on_it,
                ..
            }
            | Self::And_Register {
                setflags,
                setflags_depends_on_it,
                ..
            }
            | Self::ArithmeticShiftRight_Immediate {
                setflags,
                setflags_depends_on_it,
                ..
            }
            | Self::ArithmeticShiftRight_Register {
                setflags,
                setflags_depends_on_it,
                ..
            }
            | Self::BitClear_Register {
                setflags,
                setflags_depends_on_it,
                ..
            }
            | Self::ExclusiveOr_Register {
                setflags,
                setflags_depends_on_it,
                ..
            }
            | Self::LogicalShiftLeft_Immediate {
                setflags,
                setflags_depends_on_it,
                ..
            }
            | Self::LogicalShiftLeft_Register {
                setflags,
                setflags_depends_on_it,
                ..
            }
            | Self::LogicalShiftRight_Immediate {
                setflags,
                setflags_depends_on_it,
                ..
            }
            | Self::LogicalShiftRight_Register {
                setflags,
                setflags_depends_on_it,
                ..
            }
            | Self::Move_Immediate {
                setflags,
                setflags_depends_on_it,
                ..
            }
            | Self::Multiply {
                setflags,
                setflags_depends_on_it,
                ..
            }
            | Self::BitwiseNot_Register {
                setflags,
                setflags_depends_on_it,
                ..
            }
            | Self::LogicalOr_Register {
                setflags,
                setflags_depends_on_it,
                ..
            }
            | Self::RotateRight_Register {
                setflags,
                setflags_depends_on_it,
                ..
            }
            | Self::ReverseSubtract_Immediate {
                setflags,
                setflags_depends_on_it,
                ..
            }
            | Self::SubtractWithCarry_Register {
                setflags,
                setflags_depends_on_it,
                ..
            }
            | Self::Subtract_Immediate {
                setflags,
                setflags_depends_on_it,
                ..
            }
            | Self::Subtract_Register {
                setflags,
                setflags_depends_on_it,
                ..
            } => (*setflags, *setflags_depends_on_it),

            Self::AddWithCarry_Immediate { setflags, .. }
            | Self::Add_SPPlusImmediate { setflags, .. }
            | Self::Add_SPPlusRegister { setflags, .. }
            | Self::BitClear_Immediate { setflags, .. }
            | Self::And_Immediate { setflags, .. }
            | Self::ExclusiveOr_Immediate { setflags, .. }
            | Self::Move_Register { setflags, .. }
            | Self::BitwiseNot_Immediate { setflags, .. }
            | Self::LogicalOrNot_Immediate { setflags, .. }
            | Self::LogicalOrNot_Register { setflags, .. }
            | Self::LogicalOr_Immediate { setflags, .. }
            | Self::RotateRight_Immediate { setflags, .. }
            | Self::RotateRightWithExtend { setflags, .. }
            | Self::ReverseSubtract_Register { setflags, .. }
            | Self::SubtractWithCarry_Immediate { setflags, .. }
            | Self::Subtract_SPMinusImmediate { setflags, .. }
            | Self::Subtract_SPMinusRegister { setflags, .. } => (*setflags, false),

            Self::CompareNegative_Immediate { .. }
            | Self::CompareNegative_Register { .. }
            | Self::Compare_Register { .. }
            | Self::Compare_Immediate { .. }
            | Self::TestEquivalence_Immediate { .. }
            | Self::TestEquivalence_Register { .. }
            | Self::Test_Immediate { .. }
            | Self::Test_Register { .. } => (true, false),

            // TODO: Can set Q, but it is not used for conditional evaluation
            Self::SignedSaturate { .. } | Self::UnsignedSaturate { .. } => (false, false),
            // [ARM-ARM] A7.7.74 has pseudocode for setflags, but is hardwired to FALSE
            // We extract it for clarity.
            Self::MultiplyAccumulate { .. } => (false, false),
            // Can access APSR
            Self::MoveToRegisterFromSpecialRegister { .. } => (false, false),
            Self::MoveToSpecialRegisterFromARMRegister { sysm, mask, .. } => (
                {
                    // [ARM-ARM] B5.2.3 MSR pseudocode
                    let sysm = *sysm;
                    bitstring_extract!(sysm<7:2> | 6 bits) == bsc::C_00_0000 && !mask.is_zero()
                },
                false,
            ),

            // Remaining ALU
            Self::AddressToRegister { .. }
            | Self::BitFieldClear { .. }
            | Self::BitFieldInsert { .. }
            | Self::CountLeadingZeros { .. }
            | Self::MoveTop { .. }
            | Self::ReverseBits { .. }
            | Self::ByteReverseWord { .. }
            | Self::ByteReversePackedHalfword { .. }
            | Self::ByteReverseSignedHalfword { .. }
            | Self::SignedBitFieldExtract { .. }
            | Self::SignedExtendByte { .. }
            | Self::SignedExtendHalfword { .. }
            | Self::UnsignedBitFieldExtract { .. }
            | Self::UnsignedExtendByte { .. }
            | Self::UnsignedExtendHalfword { .. }
            | Self::MultiplyAndSubtract { .. }
            | Self::SignedDivide { .. }
            | Self::SignedMultiplyAccumulateLong { .. }
            | Self::SignedMultiplyLong { .. }
            | Self::UnsignedDivide { .. }
            | Self::UnsignedMultiplyAccumulateLong { .. }
            | Self::UnsignedMultiplyLong { .. } => (false, false),

            // Special
            Self::Unsupported { .. }
            | Self::Undefined
            | Self::Unpredictable
            | Self::Breakpoint { .. }
            | Self::ChangeProcessorState { .. }
            | Self::DataMemoryBarrier { .. }
            | Self::DataSynchronizationBarrier { .. }
            | Self::InstructionSynchronizationBarrier { .. }
            | Self::IfThen { .. }
            | Self::NoOperation
            | Self::SendEvent
            | Self::SupervisorCall { .. }
            | Self::PermanentlyUndefined { .. }
            | Self::WaitForEvent
            | Self::WaitForInterrupt => (false, false),

            // Branches
            Self::Branch { .. }
            | Self::BranchWithLink_Immediate { .. }
            | Self::BranchWithLinkAndExchange_Register { .. }
            | Self::BranchAndExchange { .. }
            | Self::TableBranch { .. }
            | Self::CompareAndBranch { .. } => (false, false),

            // LSU
            Self::ClearExclusive
            | Self::LoadMultiple { .. }
            | Self::LoadMultipleDecrementBefore { .. }
            | Self::LoadRegister_Immediate { .. }
            | Self::LoadRegister_Literal { .. }
            | Self::LoadRegister_Register { .. }
            | Self::LoadRegisterByte_Immediate { .. }
            | Self::LoadRegisterByte_Literal { .. }
            | Self::LoadRegisterByte_Register { .. }
            | Self::LoadRegisterByteUnprivileged { .. }
            | Self::LoadRegisterDual_Immediate { .. }
            | Self::LoadRegisterDual_Literal { .. }
            | Self::LoadRegisterExclusive { .. }
            | Self::LoadRegisterExclusiveByte { .. }
            | Self::LoadRegisterExclusiveHalfword { .. }
            | Self::LoadRegisterHalfword_Immediate { .. }
            | Self::LoadRegisterHalfword_Literal { .. }
            | Self::LoadRegisterHalfword_Register { .. }
            | Self::LoadRegisterHalfwordUnprivileged { .. }
            | Self::LoadRegisterSignedByte_Immediate { .. }
            | Self::LoadRegisterSignedByte_Literal { .. }
            | Self::LoadRegisterSignedByte_Register { .. }
            | Self::LoadRegisterSignedByteUnprivileged { .. }
            | Self::LoadRegisterSignedHalfword_Immediate { .. }
            | Self::LoadRegisterSignedHalfword_Literal { .. }
            | Self::LoadRegisterSignedHalfword_Register { .. }
            | Self::LoadRegisterSignedHalfwordUnprivileged { .. }
            | Self::LoadRegisterUnprivileged { .. }
            | Self::StoreMultiple { .. }
            | Self::StoreMultipleDecrementBefore { .. }
            | Self::StoreRegister_Immediate { .. }
            | Self::StoreRegister_Register { .. }
            | Self::StoreRegisterByte_Immediate { .. }
            | Self::StoreRegisterByte_Register { .. }
            | Self::StoreRegisterByteUnprivileged { .. }
            | Self::StoreRegisterDual_Immediate { .. }
            | Self::StoreRegisterExclusive { .. }
            | Self::StoreRegisterExclusiveByte { .. }
            | Self::StoreRegisterExclusiveHalfword { .. }
            | Self::StoreRegisterHalfword_Immediate { .. }
            | Self::StoreRegisterHalfword_Register { .. }
            | Self::StoreRegisterHalfwordUnprivileged { .. }
            | Self::StoreRegisterUnprivileged { .. } => (false, false),
        }
    }

    pub(super) const fn is_unconditional_in_it_block(&self) -> bool {
        #[allow(clippy::match_like_matches_macro)] // other instructions might be added in future
        match self {
            // [ARM-ARM] A7.7.17, see Note.
            Instruction::Breakpoint { .. } => true,
            _ => false,
        }
    }

    /// Current hypothesis states that multicycle instructions postpone advance head.
    ///
    /// The main assumptions are:
    ///  * An instruction is decoded throughout the whole time of execution
    ///    of the previous instruction.
    ///  * If some instruction is decoded in cycle N, then it must be in the PIQ in cycle N-1.
    ///
    /// During execution of a multi-cycle instruction, the processor does not know
    /// early enough whether the cycle is the last one or not. So advancing fetch head
    /// is postponed to the cycle, when the decoded instruction starts execution.
    /// On the other hand, during execution of a single-cycle instruction it is known
    /// that the decode stage of the following instruction will last exactly one cycle,
    /// so the entry in the fetch can be removed at the beginning of the decode stage.
    ///
    /// Currently, we treat load, store, multiplication and division instructions
    /// as the only ones that postpone advancing fetch head.
    ///
    /// Barriers, interrupt-related instructions and special instructions (like MSR)
    /// may need some research in this topic.
    /// According to [ARM-TRM] Table 3-1: MRS, MSR and CPS can take "1 or 2" cycles.
    /// Also branches may need some research. However, it might be a little hard to do so,
    /// because these instructions usually... branch.
    #[allow(clippy::too_many_lines)]
    pub(super) const fn does_postpone_advance_head(&self) -> bool {
        match self {
            Self::Unsupported { .. }
            | Self::Undefined
            | Self::Unpredictable
            | Self::AddWithCarry_Immediate { .. }
            | Self::AddWithCarry_Register { .. }
            | Self::Add_Immediate { .. }
            | Self::Add_Register { .. }
            | Self::Add_SPPlusImmediate { .. }
            | Self::Add_SPPlusRegister { .. }
            | Self::AddressToRegister { .. }
            | Self::And_Immediate { .. }
            | Self::And_Register { .. }
            | Self::ArithmeticShiftRight_Immediate { .. }
            | Self::ArithmeticShiftRight_Register { .. }
            | Self::Branch { .. }
            | Self::BitFieldClear { .. }
            | Self::BitFieldInsert { .. }
            | Self::BitClear_Immediate { .. }
            | Self::BitClear_Register { .. }
            | Self::Breakpoint { .. }
            | Self::BranchWithLink_Immediate { .. }
            | Self::BranchWithLinkAndExchange_Register { .. }
            | Self::BranchAndExchange { .. }
            | Self::ClearExclusive
            | Self::CountLeadingZeros { .. }
            | Self::ChangeProcessorState { .. }
            | Self::CompareNegative_Immediate { .. }
            | Self::CompareNegative_Register { .. }
            | Self::Compare_Register { .. }
            | Self::Compare_Immediate { .. }
            | Self::DataMemoryBarrier { .. }
            | Self::DataSynchronizationBarrier { .. }
            | Self::ExclusiveOr_Immediate { .. }
            | Self::ExclusiveOr_Register { .. }
            | Self::InstructionSynchronizationBarrier { .. }
            | Self::IfThen { .. }
            | Self::LogicalShiftLeft_Immediate { .. }
            | Self::LogicalShiftLeft_Register { .. }
            | Self::LogicalShiftRight_Immediate { .. }
            | Self::LogicalShiftRight_Register { .. }
            | Self::Move_Immediate { .. }
            | Self::Move_Register { .. }
            | Self::MoveTop { .. }
            | Self::Multiply { .. }
            | Self::NoOperation
            | Self::BitwiseNot_Immediate { .. }
            | Self::BitwiseNot_Register { .. }
            | Self::LogicalOrNot_Immediate { .. }
            | Self::LogicalOrNot_Register { .. }
            | Self::LogicalOr_Immediate { .. }
            | Self::LogicalOr_Register { .. }
            | Self::ReverseBits { .. }
            | Self::ByteReverseWord { .. }
            | Self::ByteReversePackedHalfword { .. }
            | Self::ByteReverseSignedHalfword { .. }
            | Self::RotateRight_Immediate { .. }
            | Self::RotateRight_Register { .. }
            | Self::RotateRightWithExtend { .. }
            | Self::ReverseSubtract_Immediate { .. }
            | Self::ReverseSubtract_Register { .. }
            | Self::SubtractWithCarry_Immediate { .. }
            | Self::SubtractWithCarry_Register { .. }
            | Self::SignedBitFieldExtract { .. }
            | Self::SendEvent
            | Self::SignedSaturate { .. }
            | Self::Subtract_Immediate { .. }
            | Self::Subtract_Register { .. }
            | Self::Subtract_SPMinusImmediate { .. }
            | Self::Subtract_SPMinusRegister { .. }
            | Self::SupervisorCall { .. }
            | Self::SignedExtendByte { .. }
            | Self::SignedExtendHalfword { .. }
            | Self::TestEquivalence_Immediate { .. }
            | Self::TestEquivalence_Register { .. }
            | Self::Test_Immediate { .. }
            | Self::Test_Register { .. }
            | Self::UnsignedBitFieldExtract { .. }
            | Self::PermanentlyUndefined { .. }
            | Self::UnsignedSaturate { .. }
            | Self::UnsignedExtendByte { .. }
            | Self::UnsignedExtendHalfword { .. }
            | Self::WaitForEvent
            | Self::WaitForInterrupt => false,

            // TODO(cm4): this needs research for CM4
            | Self::MultiplyAccumulate { .. }
            | Self::MultiplyAndSubtract { .. }
            | Self::SignedMultiplyAccumulateLong { .. }
            | Self::SignedMultiplyLong { .. }
            | Self::UnsignedMultiplyAccumulateLong { .. }
            | Self::UnsignedMultiplyLong { .. } => cfg!(not(feature = "soc-cc2652")),

            Self::LoadMultiple { .. }
            | Self::LoadMultipleDecrementBefore { .. }
            | Self::LoadRegister_Immediate { .. }
            | Self::LoadRegister_Literal { .. }
            | Self::LoadRegister_Register { .. }
            | Self::LoadRegisterByte_Immediate { .. }
            | Self::LoadRegisterByte_Literal { .. }
            | Self::LoadRegisterByte_Register { .. }
            | Self::LoadRegisterByteUnprivileged { .. }
            | Self::LoadRegisterDual_Immediate { .. }
            | Self::LoadRegisterDual_Literal { .. }
            | Self::LoadRegisterExclusive { .. }
            | Self::LoadRegisterExclusiveByte { .. }
            | Self::LoadRegisterExclusiveHalfword { .. }
            | Self::LoadRegisterHalfword_Immediate { .. }
            | Self::LoadRegisterHalfword_Literal { .. }
            | Self::LoadRegisterHalfword_Register { .. }
            | Self::LoadRegisterHalfwordUnprivileged { .. }
            | Self::LoadRegisterSignedByte_Immediate { .. }
            | Self::LoadRegisterSignedByte_Literal { .. }
            | Self::LoadRegisterSignedByte_Register { .. }
            | Self::LoadRegisterSignedByteUnprivileged { .. }
            | Self::LoadRegisterSignedHalfword_Immediate { .. }
            | Self::LoadRegisterSignedHalfword_Literal { .. }
            | Self::LoadRegisterSignedHalfword_Register { .. }
            | Self::LoadRegisterSignedHalfwordUnprivileged { .. }
            | Self::LoadRegisterUnprivileged { .. }
            | Self::StoreMultiple { .. }
            | Self::StoreMultipleDecrementBefore { .. }
            | Self::StoreRegister_Immediate { .. }
            | Self::StoreRegister_Register { .. }
            | Self::StoreRegisterByte_Immediate { .. }
            | Self::StoreRegisterByte_Register { .. }
            | Self::StoreRegisterByteUnprivileged { .. }
            | Self::StoreRegisterDual_Immediate { .. }
            | Self::StoreRegisterExclusive { .. }
            | Self::StoreRegisterExclusiveByte { .. }
            | Self::StoreRegisterExclusiveHalfword { .. }
            | Self::StoreRegisterHalfword_Immediate { .. }
            | Self::StoreRegisterHalfword_Register { .. }
            | Self::StoreRegisterHalfwordUnprivileged { .. }
            | Self::StoreRegisterUnprivileged { .. }
            | Self::TableBranch { .. }
            // TODO: marking it as postponing fixes half of the tests (all with code in gpram)k
            | Self::CompareAndBranch { .. }
            // TODO: marking as postponing, since it may take 2 cycles
            //       need a test for that
            | Self::MoveToRegisterFromSpecialRegister { .. }
            | Self::MoveToSpecialRegisterFromARMRegister { .. }
            | Self::SignedDivide { .. }
            | Self::UnsignedDivide { .. }
            => true,
        }
    }

    // [ARM-TRM-D] 1.4 Execution pipeline stages
    // These functions roughly classify to which part of the Execution stage an instruction belongs.
    // We can roughly classify the part by the presence of a barrel shift in the encodings.
    pub(super) const fn is_lsu_instruction(&self) -> bool {
        match self.get_memory_description() {
            MemoryInstructionDescription::LoadSingle { .. }
            | MemoryInstructionDescription::StoreSingle { .. }
            | MemoryInstructionDescription::LoadMultiple { .. }
            | MemoryInstructionDescription::StoreMultiple { .. } => true,
            MemoryInstructionDescription::None => false,
        }
    }

    #[allow(clippy::match_same_arms)]
    pub(super) const fn is_mul_div_instruction(&self) -> bool {
        match self {
            Self::Multiply { .. }
            | Self::MultiplyAccumulate { .. }
            | Self::MultiplyAndSubtract { .. }
            | Self::SignedMultiplyAccumulateLong { .. }
            | Self::SignedMultiplyLong { .. }
            | Self::UnsignedMultiplyAccumulateLong { .. }
            | Self::UnsignedMultiplyLong { .. } => true,

            Self::SignedDivide { .. } | Self::UnsignedDivide { .. } => true,

            // Should we ban windcards here?
            _ => false,
        }
    }

    pub(super) const fn is_branch(&self) -> bool {
        self.get_written_registers().get(RegisterID::PC)
    }

    pub(super) const fn is_branch_with_link_instruction(&self) -> bool {
        // Interestingly, there is one other instruction that writes both PC and LR:
        //   LDR pc, [lr], #offset
        // but it is not considered a variant for our purposes.
        matches!(
            self,
            Self::BranchWithLink_Immediate { .. } | Self::BranchWithLinkAndExchange_Register { .. }
        )
    }

    #[allow(clippy::too_many_lines)]
    pub(super) const fn get_memory_description(&self) -> MemoryInstructionDescription {
        match self {
            Self::LoadRegister_Immediate { rt, wback, .. }
            | Self::LoadRegisterByte_Immediate { rt, wback, .. }
            | Self::LoadRegisterHalfword_Immediate { rt, wback, .. }
            | Self::LoadRegisterSignedByte_Immediate { rt, wback, .. }
            | Self::LoadRegisterSignedHalfword_Immediate { rt, wback, .. } => {
                MemoryInstructionDescription::LoadSingle {
                    rt: *rt,
                    register_offset: false,
                    writeback: *wback,
                }
            }
            Self::LoadRegister_Register { rt, wback, .. }
            | Self::LoadRegisterByte_Register { rt, wback, .. }
            | Self::LoadRegisterHalfword_Register { rt, wback, .. }
            | Self::LoadRegisterSignedByte_Register { rt, wback, .. }
            | Self::LoadRegisterSignedHalfword_Register { rt, wback, .. } => {
                MemoryInstructionDescription::LoadSingle {
                    rt: *rt,
                    register_offset: true,
                    writeback: *wback,
                }
            }

            Self::LoadRegister_Literal { rt, .. }
            | Self::LoadRegisterByte_Literal { rt, .. }
            | Self::LoadRegisterHalfword_Literal { rt, .. }
            | Self::LoadRegisterSignedByte_Literal { rt, .. }
            | Self::LoadRegisterSignedHalfword_Literal { rt, .. }
            | Self::LoadRegisterUnprivileged { rt, .. }
            | Self::LoadRegisterByteUnprivileged { rt, .. }
            | Self::LoadRegisterHalfwordUnprivileged { rt, .. }
            | Self::LoadRegisterSignedByteUnprivileged { rt, .. }
            | Self::LoadRegisterSignedHalfwordUnprivileged { rt, .. }
            | Self::LoadRegisterExclusive { rt, .. }
            | Self::LoadRegisterExclusiveByte { rt, .. }
            | Self::LoadRegisterExclusiveHalfword { rt, .. } => {
                MemoryInstructionDescription::LoadSingle {
                    rt: *rt,
                    register_offset: false,
                    writeback: false,
                }
            }

            Self::StoreRegister_Immediate { rt, wback, .. }
            | Self::StoreRegisterByte_Immediate { rt, wback, .. }
            | Self::StoreRegisterHalfword_Immediate { rt, wback, .. } => {
                MemoryInstructionDescription::StoreSingle {
                    rt: *rt,
                    register_offset: false,
                    writeback: *wback,
                }
            }

            Self::StoreRegister_Register { rt, wback, .. }
            | Self::StoreRegisterByte_Register { rt, wback, .. }
            | Self::StoreRegisterHalfword_Register { rt, wback, .. } => {
                MemoryInstructionDescription::StoreSingle {
                    rt: *rt,
                    register_offset: true,
                    writeback: *wback,
                }
            }

            Self::StoreRegisterUnprivileged { rt, .. }
            | Self::StoreRegisterByteUnprivileged { rt, .. }
            | Self::StoreRegisterHalfwordUnprivileged { rt, .. }
            | Self::StoreRegisterExclusive { rt, .. }
            | Self::StoreRegisterExclusiveByte { rt, .. }
            | Self::StoreRegisterExclusiveHalfword { rt, .. } => {
                MemoryInstructionDescription::StoreSingle {
                    rt: *rt,
                    register_offset: false,
                    writeback: false,
                }
            }

            Self::LoadRegisterDual_Immediate { wback, .. }
            | Self::LoadMultipleDecrementBefore { wback, .. } => {
                MemoryInstructionDescription::LoadMultiple {
                    writeback: *wback,
                    is_narrow_ldm: false,
                }
            }

            Self::LoadMultiple {
                wback, is_narrow, ..
            } => MemoryInstructionDescription::LoadMultiple {
                writeback: *wback,
                is_narrow_ldm: *is_narrow,
            },

            Self::LoadRegisterDual_Literal { .. } => MemoryInstructionDescription::LoadMultiple {
                writeback: false,
                is_narrow_ldm: false,
            },

            Self::StoreMultiple { wback, .. }
            | Self::StoreMultipleDecrementBefore { wback, .. }
            | Self::StoreRegisterDual_Immediate { wback, .. } => {
                MemoryInstructionDescription::StoreMultiple { writeback: *wback }
            }

            // We consider TBB/TBH to be memory instructions although they require some exceptions,
            // e.g., they don't pipeline after LDR.
            Self::TableBranch { .. } => MemoryInstructionDescription::LoadSingle {
                rt: RegisterID::PC,
                register_offset: true,
                // Note: there is a reason, this may be micro-architecturally considered true:
                // This instruction does modification rather than a simple load: "add pc, [mem]" + it behaves similarly in LSU pipelining.
                // It also simplifies some conditions, which otherwise just check for writeback.
                writeback: true,
            },

            Self::Unsupported { .. }
            | Self::Undefined
            | Self::Unpredictable
            | Self::Breakpoint { .. }
            | Self::ChangeProcessorState { .. }
            | Self::AddressToRegister { .. }
            | Self::CountLeadingZeros { .. }
            | Self::MoveToRegisterFromSpecialRegister { .. }
            | Self::MoveToSpecialRegisterFromARMRegister { .. }
            | Self::ClearExclusive
            | Self::DataSynchronizationBarrier { .. }
            | Self::DataMemoryBarrier { .. }
            | Self::InstructionSynchronizationBarrier { .. }
            | Self::IfThen { .. }
            | Self::NoOperation
            | Self::WaitForEvent
            | Self::WaitForInterrupt
            | Self::SendEvent
            | Self::ReverseSubtract_Immediate { .. }
            | Self::ReverseSubtract_Register { .. }
            | Self::SubtractWithCarry_Immediate { .. }
            | Self::SubtractWithCarry_Register { .. }
            | Self::Branch { .. }
            | Self::BitFieldClear { .. }
            | Self::BitFieldInsert { .. }
            | Self::BitClear_Immediate { .. }
            | Self::BitClear_Register { .. }
            | Self::CompareAndBranch { .. }
            | Self::BranchAndExchange { .. }
            | Self::MultiplyAccumulate { .. }
            | Self::Compare_Register { .. }
            | Self::CompareNegative_Immediate { .. }
            | Self::CompareNegative_Register { .. }
            | Self::Compare_Immediate { .. }
            | Self::ExclusiveOr_Immediate { .. }
            | Self::ExclusiveOr_Register { .. }
            | Self::Move_Immediate { .. }
            | Self::Move_Register { .. }
            | Self::MoveTop { .. }
            | Self::Multiply { .. }
            | Self::BitwiseNot_Immediate { .. }
            | Self::BitwiseNot_Register { .. }
            | Self::LogicalOrNot_Immediate { .. }
            | Self::LogicalOrNot_Register { .. }
            | Self::LogicalOr_Immediate { .. }
            | Self::LogicalOr_Register { .. }
            | Self::ReverseBits { .. }
            | Self::ByteReverseWord { .. }
            | Self::ByteReversePackedHalfword { .. }
            | Self::ByteReverseSignedHalfword { .. }
            | Self::RotateRight_Immediate { .. }
            | Self::RotateRight_Register { .. }
            | Self::AddWithCarry_Immediate { .. }
            | Self::AddWithCarry_Register { .. }
            | Self::RotateRightWithExtend { .. }
            | Self::Add_Immediate { .. }
            | Self::Add_Register { .. }
            | Self::Add_SPPlusImmediate { .. }
            | Self::Add_SPPlusRegister { .. }
            | Self::And_Immediate { .. }
            | Self::And_Register { .. }
            | Self::ArithmeticShiftRight_Immediate { .. }
            | Self::ArithmeticShiftRight_Register { .. }
            | Self::LogicalShiftLeft_Immediate { .. }
            | Self::LogicalShiftLeft_Register { .. }
            | Self::LogicalShiftRight_Immediate { .. }
            | Self::LogicalShiftRight_Register { .. }
            | Self::MultiplyAndSubtract { .. }
            | Self::Subtract_Immediate { .. }
            | Self::SignedMultiplyAccumulateLong { .. }
            | Self::SignedBitFieldExtract { .. }
            | Self::SignedMultiplyLong { .. }
            | Self::SignedSaturate { .. }
            | Self::Subtract_Register { .. }
            | Self::Subtract_SPMinusImmediate { .. }
            | Self::Subtract_SPMinusRegister { .. }
            | Self::SignedExtendByte { .. }
            | Self::SignedExtendHalfword { .. }
            | Self::TestEquivalence_Immediate { .. }
            | Self::TestEquivalence_Register { .. }
            | Self::UnsignedBitFieldExtract { .. }
            | Self::UnsignedMultiplyAccumulateLong { .. }
            | Self::UnsignedMultiplyLong { .. }
            | Self::UnsignedSaturate { .. }
            | Self::UnsignedExtendByte { .. }
            | Self::UnsignedExtendHalfword { .. }
            | Self::BranchWithLink_Immediate { .. }
            | Self::BranchWithLinkAndExchange_Register { .. }
            | Self::Test_Immediate { .. }
            | Self::Test_Register { .. }
            | Self::UnsignedDivide { .. }
            | Self::SignedDivide { .. }
            | Self::PermanentlyUndefined { .. }
            | Self::SupervisorCall { .. } => MemoryInstructionDescription::None,
        }
    }
}

impl fmt::Display for Instruction {
    // We want to have one big match with all instructions.
    #[allow(clippy::too_many_lines)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unsupported { name } => write!(f, "unsupported ({name})"),
            Self::Undefined => write!(f, "undefined"),
            Self::Unpredictable => write!(f, "unpredictable"),
            Self::Breakpoint { imm32 } => write!(f, "bkpt {}", PrintImm(*imm32)),
            Self::ChangeProcessorState {
                enable,
                disable,
                affect_pri,
                affect_fault,
            } => {
                debug_assert_eq!(*enable, !*disable);
                write!(
                    f,
                    "cps{} {}{}",
                    if *enable { "ie" } else { "id" },
                    if *affect_pri { "i" } else { "" },
                    if *affect_fault { "f" } else { "" }
                )
            }
            Self::AddressToRegister { rd, imm32, add } => {
                if *add {
                    write!(f, "add {rd}, {}, {}", RegisterID::PC, PrintImm(*imm32))
                } else {
                    write!(f, "sub {rd}, {}, {}", RegisterID::PC, PrintImm(*imm32))
                }
            }
            Self::CountLeadingZeros { rd, rm } => write!(f, "clz {rd}, {rm}"),
            Self::Move_Immediate {
                rd,
                imm32,
                setflags,
                ..
            } => write!(
                f,
                "mov{} {}, {}",
                if *setflags { "s" } else { "" },
                rd,
                PrintImm(*imm32)
            ),
            Self::Move_Register { rd, rm, setflags } => {
                write!(f, "mov{} {rd}, {rm}", if *setflags { "s" } else { "" })
            }
            Self::MoveTop { rd, imm16 } => {
                write!(f, "movt {rd}, {}", PrintImm(imm16.zero_extend()))
            }
            Self::AddWithCarry_Immediate {
                rd,
                rn,
                imm32,
                setflags,
            } => write!(
                f,
                "adc{} {}, {}, {}",
                if *setflags { "s" } else { "" },
                rd,
                rn,
                PrintImm(*imm32)
            ),
            Self::AddWithCarry_Register {
                rd,
                rn,
                rm,
                shift,
                setflags,
                ..
            } => write!(
                f,
                "adc{} {}, {}, {}",
                if *setflags { "s" } else { "" },
                rd,
                rn,
                PrintShiftedReg(*rm, *shift)
            ),
            Self::Add_Immediate {
                rd,
                rn,
                imm32,
                setflags,
                ..
            } => write!(
                f,
                "add{} {}, {}, {}",
                if *setflags { "s" } else { "" },
                rd,
                rn,
                PrintImm(*imm32)
            ),
            Self::Add_Register {
                rd,
                rn,
                rm,
                shift,
                setflags,
                ..
            } => write!(
                f,
                "add{} {}, {}, {}",
                if *setflags { "s" } else { "" },
                rd,
                rn,
                PrintShiftedReg(*rm, *shift)
            ),
            Self::Add_SPPlusImmediate {
                rd,
                imm32,
                setflags,
            } => write!(
                f,
                "add{} {}, {}, {}",
                if *setflags { "s" } else { "" },
                rd,
                RegisterID::SP,
                PrintImm(*imm32)
            ),
            Self::Add_SPPlusRegister {
                rd,
                rm,
                shift,
                setflags,
            } => write!(
                f,
                "add{} {}, {}, {}",
                if *setflags { "s" } else { "" },
                rd,
                RegisterID::SP,
                PrintShiftedReg(*rm, *shift)
            ),
            Self::And_Immediate {
                rd,
                rn,
                imm32,
                setflags,
                ..
            } => write!(
                f,
                "and{} {}, {}, {}",
                if *setflags { "s" } else { "" },
                rd,
                rn,
                PrintImm(*imm32)
            ),
            Self::And_Register {
                rd,
                rn,
                rm,
                shift,
                setflags,
                ..
            } => write!(
                f,
                "and{} {}, {}, {}",
                if *setflags { "s" } else { "" },
                rd,
                rn,
                PrintShiftedReg(*rm, *shift)
            ),
            Self::ArithmeticShiftRight_Immediate {
                rd,
                rm,
                shift,
                setflags,
                ..
            } => {
                debug_assert!(shift.srtype == SRType::ASR);
                write!(
                    f,
                    "asr{} {}, {}, #{}",
                    if *setflags { "s" } else { "" },
                    rd,
                    rm,
                    shift.amount
                )
            }
            Self::ArithmeticShiftRight_Register {
                rd,
                rn,
                rm,
                setflags,
                ..
            } => write!(
                f,
                "asr{} {}, {}, {}",
                if *setflags { "s" } else { "" },
                rd,
                rn,
                rm
            ),
            Self::BitFieldClear { rd, msbit, lsbit } => {
                write!(f, "bfc {rd}, #{lsbit}, #{}", msbit - lsbit + 1)
            }
            Self::BitFieldInsert {
                rd,
                rn,
                msbit,
                lsbit,
            } => write!(f, "bfi {rd}, {rn}, #{lsbit}, #{}", msbit - lsbit + 1),
            Self::BitClear_Immediate {
                rd,
                rn,
                imm32,
                setflags,
                ..
            } => write!(
                f,
                "bic{} {}, {}, {}",
                if *setflags { "s" } else { "" },
                rd,
                rn,
                PrintImm(*imm32)
            ),
            Self::BitClear_Register {
                rd,
                rn,
                rm,
                shift,
                setflags,
                ..
            } => write!(
                f,
                "bic{} {}, {}, {}",
                if *setflags { "s" } else { "" },
                rd,
                rn,
                PrintShiftedReg(*rm, *shift)
            ),
            Self::ExclusiveOr_Immediate {
                rd,
                rn,
                imm32,
                setflags,
                ..
            } => write!(
                f,
                "eor{} {}, {}, {}",
                if *setflags { "s" } else { "" },
                rd,
                rn,
                PrintImm(*imm32)
            ),
            Self::ExclusiveOr_Register {
                rd,
                rn,
                rm,
                shift,
                setflags,
                ..
            } => write!(
                f,
                "eor{}, {}, {}, {}",
                if *setflags { "s" } else { "" },
                rd,
                rn,
                PrintShiftedReg(*rm, *shift)
            ),
            Self::LogicalShiftLeft_Immediate {
                rd,
                rm,
                shift,
                setflags,
                ..
            } => {
                debug_assert!(shift.srtype == SRType::LSL);
                write!(
                    f,
                    "lsl{} {}, {}, #{}",
                    if *setflags { "s" } else { "" },
                    rd,
                    rm,
                    shift.amount
                )
            }
            Self::LogicalShiftLeft_Register {
                rd,
                rn,
                rm,
                setflags,
                ..
            } => write!(
                f,
                "lsl{} {}, {}, {}",
                if *setflags { "s" } else { "" },
                rd,
                rn,
                rm
            ),
            Self::LogicalShiftRight_Immediate {
                rd,
                rm,
                shift,
                setflags,
                ..
            } => {
                debug_assert!(shift.srtype == SRType::LSR);
                write!(
                    f,
                    "lsr{} {}, {}, #{}",
                    if *setflags { "s" } else { "" },
                    rd,
                    rm,
                    shift.amount
                )
            }
            Self::LogicalShiftRight_Register {
                rd,
                rn,
                rm,
                setflags,
                ..
            } => write!(
                f,
                "lsr{} {}, {}, {}",
                if *setflags { "s" } else { "" },
                rd,
                rn,
                rm
            ),
            Self::ReverseSubtract_Immediate {
                rd,
                rn,
                imm32,
                setflags,
                ..
            } => write!(
                f,
                "rsb{} {}, {}, {}",
                if *setflags { "s" } else { "" },
                rd,
                rn,
                PrintImm(*imm32)
            ),
            Self::ReverseSubtract_Register {
                rd,
                rn,
                rm,
                shift,
                setflags,
            } => write!(
                f,
                "rsb{} {}, {}, {}",
                if *setflags { "s" } else { "" },
                rd,
                rn,
                PrintShiftedReg(*rm, *shift)
            ),
            Self::SubtractWithCarry_Immediate {
                rd,
                rn,
                imm32,
                setflags,
            } => write!(
                f,
                "sbc{} {}, {}, {}",
                if *setflags { "s" } else { "" },
                rd,
                rn,
                PrintImm(*imm32)
            ),
            Self::SubtractWithCarry_Register {
                rd,
                rn,
                rm,
                shift,
                setflags,
                ..
            } => write!(
                f,
                "sbc{} {}, {}, {}",
                if *setflags { "s" } else { "" },
                rd,
                rn,
                PrintShiftedReg(*rm, *shift)
            ),
            Self::Subtract_Immediate {
                rd,
                rn,
                imm32,
                setflags,
                ..
            } => write!(
                f,
                "sub{} {}, {}, {}",
                if *setflags { "s" } else { "" },
                rd,
                rn,
                PrintImm(*imm32)
            ),
            Self::Subtract_Register {
                rd,
                rn,
                rm,
                shift,
                setflags,
                ..
            } => write!(
                f,
                "sub{} {}, {}, {}",
                if *setflags { "s" } else { "" },
                rd,
                rn,
                PrintShiftedReg(*rm, *shift)
            ),
            Self::Subtract_SPMinusImmediate {
                rd,
                imm32,
                setflags,
            } => write!(
                f,
                "sub{} {}, {}, {}",
                if *setflags { "s" } else { "" },
                rd,
                RegisterID::SP,
                PrintImm(*imm32)
            ),
            Self::Subtract_SPMinusRegister {
                rd,
                rm,
                shift,
                setflags,
            } => write!(
                f,
                "sub{} {}, {}, {}",
                if *setflags { "s" } else { "" },
                rd,
                RegisterID::SP,
                PrintShiftedReg(*rm, *shift)
            ),
            Self::SignedExtendByte { rd, rm, rotation } => {
                write!(f, "sxtb {rd}, {}", PrintShiftedReg(*rm, *rotation))
            }
            Self::SignedExtendHalfword { rd, rm, rotation } => {
                write!(f, "sxth {rd}, {}", PrintShiftedReg(*rm, *rotation))
            }
            Self::TableBranch { rn, rm, is_tbh } => {
                if *is_tbh {
                    write!(f, "tbh [{rn}, {rm}, LSL #1]")
                } else {
                    write!(f, "tbb [{rn}, {rm}]")
                }
            }
            Self::TestEquivalence_Immediate { rn, imm32, .. } => {
                write!(f, "teq {rn}, {}", PrintImm(*imm32))
            }
            Self::TestEquivalence_Register { rn, rm, shift } => {
                write!(f, "teq {rn}, {}", PrintShiftedReg(*rm, *shift))
            }
            Self::Branch { cond, imm32 } => write!(f, "b{cond} [pc, {}]", PrintImm(*imm32)),
            Self::CompareAndBranch { rn, imm32, nonzero } => write!(
                f,
                "cb{}z {}, [pc, {}]",
                if *nonzero { "n" } else { "" },
                rn,
                PrintImm(*imm32)
            ),
            Self::BranchWithLink_Immediate { imm32 } => write!(f, "bl [pc, {}]", PrintImm(*imm32)),
            Self::BranchWithLinkAndExchange_Register { rm } => write!(f, "blx {rm}"),
            Self::BranchAndExchange { rm } => write!(f, "bx {rm}"),

            Self::LoadRegister_Immediate {
                rt,
                rn,
                imm32,
                index,
                add,
                wback,
            } => {
                if *rn == RegisterID::SP && *imm32 == Word::from(4) && !index && *add && *wback {
                    // `pop { rt }` is a shortcut notation for `ldr rt, [sp], 4`
                    write!(f, "pop {{ {rt} }}")
                } else {
                    match (*index, *wback) {
                        // [ARM-ARM] A7.7.43 - Assembler syntax
                        (true, false) => {
                            write!(f, "ldr {rt}, [{rn}, {}]", PrintAddImm(*add, *imm32))
                        }
                        (true, true) => {
                            write!(f, "ldr {rt}, [{rn}, {}]!", PrintAddImm(*add, *imm32))
                        }
                        (false, true) => {
                            write!(f, "ldr {rt}, [{rn}], {}", PrintAddImm(*add, *imm32))
                        }
                        _ => panic!("Undefined instruction"),
                    }
                }
            }
            Self::LoadRegister_Literal { rt, imm32, add } => write!(
                f,
                "ldr {}, [{}, {}]",
                rt,
                RegisterID::PC,
                PrintAddImm(*add, *imm32)
            ),
            Self::LoadRegister_Register {
                rt,
                rn,
                rm,
                shift,
                index,
                add,
                wback,
            } => {
                debug_assert!(*index && *add && !*wback);
                debug_assert!(shift.srtype == SRType::LSL);
                write!(f, "ldr {rt}, [{rn}, {}]", PrintShiftedReg(*rm, *shift))
            }
            Self::LoadRegisterByte_Immediate {
                rt,
                rn,
                imm32,
                index,
                add,
                wback,
            } => match (*index, *wback) {
                // [ARM-ARM] A7.7.46 - Assembler syntax
                (true, false) => write!(f, "ldrb {rt}, [{rn}, {}]", PrintAddImm(*add, *imm32)),
                (true, true) => write!(f, "ldrb {rt}, [{rn}, {}]!", PrintAddImm(*add, *imm32)),
                (false, true) => write!(f, "ldrb {rt}, [{rn}], {}", PrintAddImm(*add, *imm32)),
                _ => panic!("Undefined instruction"),
            },
            Self::LoadRegisterByte_Literal { rt, imm32, add } => write!(
                f,
                "ldrb {}, [{}, {}]",
                rt,
                RegisterID::PC,
                PrintAddImm(*add, *imm32)
            ),
            Self::LoadRegisterByte_Register {
                rt,
                rn,
                rm,
                shift,
                index,
                add,
                wback,
            } => {
                debug_assert!(*index && *add && !*wback);
                debug_assert!(shift.srtype == SRType::LSL);
                write!(f, "ldrb {rt}, [{rn}, {}]", PrintShiftedReg(*rm, *shift))
            }
            Self::LoadRegisterHalfword_Immediate {
                rt,
                rn,
                imm32,
                index,
                add,
                wback,
            } => match (*index, *wback) {
                // [ARM-ARM] A7.7.55 - Assembler syntax
                (true, false) => write!(f, "ldrh {rt}, [{rn}, {}]", PrintAddImm(*add, *imm32)),
                (true, true) => write!(f, "ldrh {rt}, [{rn}, {}]!", PrintAddImm(*add, *imm32)),
                (false, true) => write!(f, "ldrh {rt}, [{rn}], {}", PrintAddImm(*add, *imm32)),
                _ => panic!("Undefined instruction"),
            },
            Self::LoadRegisterHalfword_Literal { rt, imm32, add } => write!(
                f,
                "ldrh {}, [{}, {}]",
                rt,
                RegisterID::PC,
                PrintAddImm(*add, *imm32)
            ),
            Self::LoadRegisterHalfword_Register {
                rt,
                rn,
                rm,
                shift,
                index,
                add,
                wback,
            } => {
                debug_assert!(*index && *add && !*wback);
                debug_assert!(shift.srtype == SRType::LSL);
                write!(f, "ldrh {rt}, [{rn}, {}]", PrintShiftedReg(*rm, *shift))
            }
            Self::LoadRegisterSignedByte_Immediate {
                rt,
                rn,
                imm32,
                index,
                add,
                wback,
            } => match (*index, *wback) {
                // [ARM-ARM] A7.7.59 - Assembler syntax
                (true, false) => write!(f, "ldrsb {rt}, [{rn}, {}]", PrintAddImm(*add, *imm32)),
                (true, true) => write!(f, "ldrsb {rt}, [{rn}, {}]!", PrintAddImm(*add, *imm32)),
                (false, true) => write!(f, "ldrsb {rt}, [{rn}], {}", PrintAddImm(*add, *imm32)),
                _ => panic!("Undefined instruction"),
            },
            Self::LoadRegisterSignedByte_Literal { rt, imm32, add } => write!(
                f,
                "ldrsb {}, [{}, {}]",
                rt,
                RegisterID::PC,
                PrintAddImm(*add, *imm32)
            ),
            Self::LoadRegisterSignedByte_Register {
                rt,
                rn,
                rm,
                shift,
                index,
                add,
                wback,
            } => {
                debug_assert!(*index && *add && !*wback);
                debug_assert!(shift.srtype == SRType::LSL);
                write!(
                    f,
                    "ldrsb {}, [{}, {}]",
                    rt,
                    rn,
                    PrintShiftedReg(*rm, *shift)
                )
            }
            Self::LoadRegisterSignedHalfword_Immediate {
                rt,
                rn,
                imm32,
                index,
                add,
                wback,
            } => match (*index, *wback) {
                // [ARM-ARM] A7.7.63 - Assembler syntax
                (true, false) => write!(f, "ldrsh {rt}, [{rn}, {}]", PrintAddImm(*add, *imm32)),
                (true, true) => write!(f, "ldrsh {rt}, [{rn}, {}]!", PrintAddImm(*add, *imm32)),
                (false, true) => write!(f, "ldrsh {rt}, [{rn}], {}", PrintAddImm(*add, *imm32)),
                _ => panic!("Undefined instruction"),
            },
            Self::LoadRegisterSignedHalfword_Literal { rt, imm32, add } => write!(
                f,
                "ldrsh {}, [{}, {}]",
                rt,
                RegisterID::PC,
                PrintAddImm(*add, *imm32)
            ),
            Self::LoadRegisterSignedHalfword_Register {
                rt,
                rn,
                rm,
                shift,
                index,
                add,
                wback,
            } => {
                debug_assert!(*index && *add && !*wback);
                debug_assert!(shift.srtype == SRType::LSL);
                write!(
                    f,
                    "ldrsh {}, [{}, {}]",
                    rt,
                    rn,
                    PrintShiftedReg(*rm, *shift)
                )
            }
            Self::LoadRegisterDual_Immediate {
                rt,
                rt2,
                rn,
                imm32,
                index,
                add,
                wback,
            } => match (*index, *wback) {
                // [ARM-ARM] A7.7.50 - Assembler syntax
                (true, false) => write!(
                    f,
                    "ldrd {}, {}, [{}, {}]",
                    rt,
                    rt2,
                    rn,
                    PrintAddImm(*add, *imm32)
                ),
                (true, true) => write!(
                    f,
                    "ldrd {}, {}, [{}, {}]!",
                    rt,
                    rt2,
                    rn,
                    PrintAddImm(*add, *imm32)
                ),
                (false, true) => write!(
                    f,
                    "ldrd {}, {}, [{}], {}",
                    rt,
                    rt2,
                    rn,
                    PrintAddImm(*add, *imm32)
                ),
                _ => panic!("Undefined instruction"),
            },
            Self::LoadRegisterDual_Literal {
                rt,
                rt2,
                imm32,
                add,
            } => write!(
                f,
                "ldrd {}, {}, [{}, {}]",
                rt,
                rt2,
                RegisterID::PC,
                PrintAddImm(*add, *imm32)
            ),
            Self::LoadRegisterUnprivileged { rt, rn, imm32 } => {
                write!(f, "ldrt {rt}, [{rn}, {}]", PrintImm(*imm32))
            }
            Self::LoadRegisterByteUnprivileged { rt, rn, imm32 } => {
                write!(f, "ldrbt {rt}, [{rn}, {}]", PrintImm(*imm32))
            }
            Self::LoadRegisterHalfwordUnprivileged { rt, rn, imm32 } => {
                write!(f, "ldrht {rt}, [{rn}, {}]", PrintImm(*imm32))
            }
            Self::LoadRegisterSignedByteUnprivileged { rt, rn, imm32 } => {
                write!(f, "ldrsbt {rt}, [{rn}, {}]", PrintImm(*imm32))
            }
            Self::LoadRegisterSignedHalfwordUnprivileged { rt, rn, imm32 } => {
                write!(f, "ldrsht {rt}, [{rn}, {}]", PrintImm(*imm32))
            }
            Self::LoadRegisterExclusive { rt, rn, imm32 } => {
                write!(f, "ldrex {rt}, [{rn}, {}]", PrintImm(*imm32))
            }
            Self::LoadRegisterExclusiveByte { rt, rn } => write!(f, "ldrexb {rt}, [{rn}]"),
            Self::LoadRegisterExclusiveHalfword { rt, rn } => write!(f, "ldrexh {rt}, [{rn}]"),
            Self::LogicalOrNot_Immediate {
                rd,
                rn,
                imm32,
                setflags,
                ..
            } => write!(
                f,
                "orn{} {}, {}, {}",
                if *setflags { "s" } else { "" },
                rd,
                rn,
                PrintImm(*imm32)
            ),
            Self::LogicalOrNot_Register {
                rd,
                rn,
                rm,
                shift,
                setflags,
            } => write!(
                f,
                "orn{} {}, {}, {}",
                if *setflags { "s" } else { "" },
                rd,
                rn,
                PrintShiftedReg(*rm, *shift)
            ),
            Self::LogicalOr_Immediate {
                rd,
                rn,
                imm32,
                setflags,
                ..
            } => write!(
                f,
                "orr{} {}, {}, {}",
                if *setflags { "s" } else { "" },
                rd,
                rn,
                PrintImm(*imm32)
            ),
            Self::LogicalOr_Register {
                rd,
                rn,
                rm,
                shift,
                setflags,
                ..
            } => write!(
                f,
                "orr{} {}, {}, {}",
                if *setflags { "s" } else { "" },
                rd,
                rn,
                PrintShiftedReg(*rm, *shift)
            ),
            Self::RotateRight_Immediate {
                rd,
                rm,
                shift,
                setflags,
            } => {
                debug_assert!(shift.srtype == SRType::ROR);
                write!(
                    f,
                    "ror{} {}, {}, #{}",
                    if *setflags { "s" } else { "" },
                    rd,
                    rm,
                    shift.amount
                )
            }
            Self::RotateRight_Register {
                rd,
                rn,
                rm,
                setflags,
                ..
            } => write!(
                f,
                "ror{} {}, {}, {}",
                if *setflags { "s" } else { "" },
                rd,
                rn,
                rm
            ),
            Self::RotateRightWithExtend { rd, rm, setflags } => {
                write!(f, "rrx{} {rd}, {rm}", if *setflags { "s" } else { "" })
            }
            Self::SignedMultiplyAccumulateLong {
                rn,
                rm,
                rd_hi,
                rd_lo,
            } => write!(f, "smlal {rd_lo}, {rd_hi}, {rn}, {rm}"),
            Self::SignedBitFieldExtract {
                rn,
                rd,
                widthminus1,
                lsbit,
            } => write!(f, "sbfx {rd}, {rn}, #{lsbit}, #{}", widthminus1 + 1),
            Self::SignedMultiplyLong {
                rn,
                rm,
                rd_hi,
                rd_lo,
            } => write!(f, "smull {rd_lo}, {rd_hi}, {rn}, {rm}"),
            Self::UnsignedBitFieldExtract {
                rn,
                rd,
                widthminus1,
                lsbit,
            } => write!(f, "ubfx {rd}, {rn}, #{lsbit}, #{}", widthminus1 + 1),
            Self::SignedSaturate {
                rd,
                rn,
                saturate_to,
                shift,
            } => write!(
                f,
                "ssat {}, #{}, {}",
                rd,
                saturate_to,
                PrintShiftedReg(*rn, *shift)
            ),
            Self::UnsignedMultiplyAccumulateLong {
                rn,
                rm,
                rd_hi,
                rd_lo,
            } => write!(f, "umlal {rd_lo}, {rd_hi}, {rn}, {rm}"),
            Self::UnsignedMultiplyLong {
                rn,
                rm,
                rd_hi,
                rd_lo,
            } => write!(f, "umull {rd_lo}, {rd_hi}, {rn}, {rm}"),
            Self::UnsignedSaturate {
                rd,
                rn,
                saturate_to,
                shift,
            } => write!(
                f,
                "usat {}, #{}, {}",
                rd,
                saturate_to,
                PrintShiftedReg(*rn, *shift)
            ),
            Self::UnsignedExtendByte { rd, rm, rotation } => {
                write!(f, "uxtb {rd}, {}", PrintShiftedReg(*rm, *rotation))
            }
            Self::UnsignedExtendHalfword { rd, rm, rotation } => {
                write!(f, "uxth {rd}, {}", PrintShiftedReg(*rm, *rotation))
            }
            Self::StoreRegister_Immediate {
                rt,
                rn,
                imm32,
                index,
                add,
                wback,
            } => {
                if *rn == RegisterID::SP && *imm32 == Word::from(4) && *index && !add && *wback {
                    // `push.w { rt }` is a shortcut notation for `str rt, [sp, -4]!`
                    write!(f, "push {{ {rt} }}")
                } else {
                    match (*index, *wback) {
                        // [ARM-ARM] A7.7.161 - Assembler syntax
                        (true, false) => {
                            write!(f, "str {rt}, [{rn}, {}]", PrintAddImm(*add, *imm32))
                        }
                        (true, true) => {
                            write!(f, "str {rt}, [{rn}, {}]!", PrintAddImm(*add, *imm32))
                        }
                        (false, true) => {
                            write!(f, "str {rt}, [{rn}], {}", PrintAddImm(*add, *imm32))
                        }
                        _ => panic!("Undefined instruction"),
                    }
                }
            }
            Self::StoreRegister_Register {
                rt,
                rn,
                rm,
                shift,
                index,
                add,
                wback,
            } => {
                debug_assert!(*index && *add && !*wback);
                write!(f, "str {rt}, [{rn}, {}]", PrintShiftedReg(*rm, *shift))
            }
            Self::StoreRegisterByte_Immediate {
                rt,
                rn,
                imm32,
                index,
                add,
                wback,
            } => match (*index, *wback) {
                // [ARM-ARM] A7.7.163 - Assembler syntax
                (true, false) => write!(f, "strb {rt}, [{rn}, {}]", PrintAddImm(*add, *imm32)),
                (true, true) => write!(f, "strb {rt}, [{rn}, {}]!", PrintAddImm(*add, *imm32)),
                (false, true) => write!(f, "strb {rt}, [{rn}], {}", PrintAddImm(*add, *imm32)),
                _ => panic!("Undefined instruction"),
            },
            Self::StoreRegisterByte_Register {
                rt,
                rn,
                rm,
                shift,
                index,
                add,
                wback,
            } => {
                debug_assert!(*index && *add && !*wback);
                write!(f, "strb {rt}, [{rn}, {}]", PrintShiftedReg(*rm, *shift))
            }
            Self::StoreRegisterHalfword_Immediate {
                rt,
                rn,
                imm32,
                index,
                add,
                wback,
            } => match (*index, *wback) {
                // [ARM-ARM] A7.7.170 - Assembler syntax
                (true, false) => write!(f, "strh {rt}, [{rn}, {}]", PrintAddImm(*add, *imm32)),
                (true, true) => write!(f, "strh {rt}, [{rn}, {}]!", PrintAddImm(*add, *imm32)),
                (false, true) => write!(f, "strh {rt}, [{rn}], {}", PrintAddImm(*add, *imm32)),
                _ => panic!("Undefined instruction"),
            },
            Self::StoreRegisterHalfword_Register {
                rt,
                rn,
                rm,
                shift,
                index,
                add,
                wback,
            } => {
                debug_assert!(*index && *add && !*wback);
                write!(f, "strh {rt}, [{rn}, {}]", PrintShiftedReg(*rm, *shift))
            }
            Self::StoreRegisterDual_Immediate {
                rt,
                rt2,
                rn,
                imm32,
                index,
                add,
                wback,
            } => match (*index, *wback) {
                // [ARM-ARM] A7.7.166 - Assembler syntax
                (true, false) => write!(
                    f,
                    "strd {}, {}, [{}, {}]",
                    rt,
                    rt2,
                    rn,
                    PrintAddImm(*add, *imm32)
                ),
                (true, true) => write!(
                    f,
                    "strd {}, {}, [{}, {}]!",
                    rt,
                    rt2,
                    rn,
                    PrintAddImm(*add, *imm32)
                ),
                (false, true) => write!(
                    f,
                    "strd {}, {}, [{}], {}",
                    rt,
                    rt2,
                    rn,
                    PrintAddImm(*add, *imm32)
                ),
                _ => panic!("Undefined instruction"),
            },
            Self::StoreRegisterUnprivileged { rt, rn, imm32 } => {
                write!(f, "strt {rt}, [{rn}, {}]", PrintImm(*imm32))
            }
            Self::StoreRegisterByteUnprivileged { rt, rn, imm32 } => {
                write!(f, "strbt {rt}, [{rn}, {}]", PrintImm(*imm32))
            }
            Self::StoreRegisterHalfwordUnprivileged { rt, rn, imm32 } => {
                write!(f, "strht {rt}, [{rn}, {}]", PrintImm(*imm32))
            }
            Self::StoreRegisterExclusive { rd, rt, rn, imm32 } => {
                write!(f, "strex {rd}, {rt}, [{rn}, {}]", PrintImm(*imm32))
            }
            Self::StoreRegisterExclusiveByte { rd, rt, rn } => {
                write!(f, "strexb {rd}, {rt}, [{rn}]")
            }
            Self::StoreRegisterExclusiveHalfword { rd, rt, rn } => {
                write!(f, "strexh {rd}, {rt}, [{rn}]")
            }
            Self::MoveToRegisterFromSpecialRegister { rd, sysm } => {
                write!(f, "mrs {rd}, {}", PrintSysm::on_reads(*sysm))
            }
            Self::MoveToSpecialRegisterFromARMRegister { rn, sysm, mask } => {
                write!(f, "msr {}, {rn}", PrintSysm::on_writes(*sysm, *mask))
            }
            Self::NoOperation => write!(f, "nop"),
            Self::WaitForEvent => write!(f, "wfe"),
            Self::WaitForInterrupt => write!(f, "wfi"),
            Self::SendEvent => write!(f, "sev"),
            Self::ClearExclusive => write!(f, "clrex"),
            Self::DataSynchronizationBarrier { option } => {
                debug_assert_eq!(*option, bsc::C_1111, "reserved (unsupported) DSB option");
                write!(f, "dsb sy")
            }
            Self::DataMemoryBarrier { option } => {
                debug_assert_eq!(*option, bsc::C_1111, "reserved (unsupported) DMB option");
                write!(f, "dmb sy")
            }
            Self::InstructionSynchronizationBarrier { option } => {
                debug_assert_eq!(*option, bsc::C_1111, "reserved (unsupported) ISB option");
                write!(f, "isb sy")
            }
            Self::IfThen { firstcond, mask } => {
                // prepare "firstcond[0] : mask" since it's more jump-table friendly
                let (firstcond_bs, mask) = (firstcond.0, *mask);
                let firstcond0 = bitstring_extract!(firstcond_bs<0:0> | 1 bits);
                let extended = bitstring_concat!(firstcond0 : mask | 5 bits);

                // [ARM-ARM] A7.7.38
                let suffix = match extended {
                    bsc::C_0_0000 | bsc::C_1_0000 => panic!("invalid mask"),
                    bsc::C_0_1111 | bsc::C_1_0001 => "eee",
                    bsc::C_0_1110 | bsc::C_1_0010 => "ee",
                    bsc::C_0_1101 | bsc::C_1_0011 => "eet",
                    bsc::C_0_1100 | bsc::C_1_0100 => "e",
                    bsc::C_0_1011 | bsc::C_1_0101 => "ete",
                    bsc::C_0_1010 | bsc::C_1_0110 => "et",
                    bsc::C_0_1001 | bsc::C_1_0111 => "ett",
                    bsc::C_0_1000 | bsc::C_1_1000 => "",
                    bsc::C_0_0111 | bsc::C_1_1001 => "tee",
                    bsc::C_0_0110 | bsc::C_1_1010 => "te",
                    bsc::C_0_0101 | bsc::C_1_1011 => "tet",
                    bsc::C_0_0100 | bsc::C_1_1100 => "t",
                    bsc::C_0_0011 | bsc::C_1_1101 => "tte",
                    bsc::C_0_0010 | bsc::C_1_1110 => "tt",
                    bsc::C_0_0001 | bsc::C_1_1111 => "ttt",
                    _ => unreachable!(),
                };
                write!(f, "it{suffix} {firstcond}")
            }
            Self::MultiplyAccumulate {
                rd,
                rn,
                rm,
                ra,
                setflags,
            } => {
                debug_assert!(!setflags, "setflags for MLA is always false, see decode");
                write!(f, "mla {rd}, {rn}, {rm}, {ra}")
            }
            Self::MultiplyAndSubtract { rd, rn, rm, ra } => {
                write!(f, "mls {rd}, {rn}, {rm}, {ra}")
            }
            Self::Multiply {
                rd,
                rn,
                rm,
                setflags,
                ..
            } => write!(
                f,
                "mul{} {}, {}, {}",
                if *setflags { "s" } else { "" },
                rd,
                rn,
                rm
            ),
            Self::BitwiseNot_Immediate {
                rd,
                imm32,
                setflags,
                ..
            } => write!(
                f,
                "mvn{} {}, {}",
                if *setflags { "s" } else { "" },
                rd,
                PrintImm(*imm32)
            ),
            Self::BitwiseNot_Register {
                rd,
                rm,
                shift,
                setflags,
                ..
            } => write!(
                f,
                "mvn{} {}, {}",
                if *setflags { "s" } else { "" },
                rd,
                PrintShiftedReg(*rm, *shift)
            ),
            Self::ReverseBits { rd, rm } => write!(f, "rbit {rd}, {rm}"),
            Self::ByteReverseWord { rd, rm } => write!(f, "rev {rd}, {rm}"),
            Self::ByteReversePackedHalfword { rd, rm } => write!(f, "rev16 {rd}, {rm}"),
            Self::ByteReverseSignedHalfword { rd, rm } => write!(f, "revsh {rd}, {rm}"),
            Self::Compare_Register { rn, rm, shift } => {
                write!(f, "cmp {rn}, {}", PrintShiftedReg(*rm, *shift))
            }
            Self::CompareNegative_Immediate { rn, imm32 } => {
                write!(f, "cmn {rn}, {}", PrintImm(*imm32))
            }
            Self::CompareNegative_Register { rn, rm, shift } => {
                write!(f, "cmn {rn}, {}", PrintShiftedReg(*rm, *shift))
            }
            Self::Compare_Immediate { rn, imm32 } => write!(f, "cmp {rn}, {}", PrintImm(*imm32)),
            Self::Test_Immediate { rn, imm32, .. } => write!(f, "tst {rn}, {}", PrintImm(*imm32)),
            Self::Test_Register { rn, rm, shift } => {
                write!(f, "tst {rn}, {}", PrintShiftedReg(*rm, *shift))
            }
            Self::PermanentlyUndefined { imm32 } => write!(f, "udf {}", PrintImm(*imm32)),
            Self::SupervisorCall { imm32 } => write!(f, "svc {}", PrintImm(*imm32)),
            Self::SignedDivide { rd, rn, rm } => write!(f, "sdiv {rd}, {rn}, {rm}"),
            Self::UnsignedDivide { rd, rn, rm } => write!(f, "udiv {rd}, {rn}, {rm}"),
            Self::StoreMultiple {
                rn,
                registers,
                wback,
            } => write!(
                f,
                "stm {}{}, {}",
                rn,
                if *wback { "!" } else { "" },
                PrintRegisterList(*registers)
            ),
            Self::StoreMultipleDecrementBefore {
                rn,
                registers,
                wback,
            } => {
                if *rn == RegisterID::SP && *wback {
                    // `push { regs }` is a shortcut notation for `stmdb sp!, { regs }`
                    write!(f, "push {}", PrintRegisterList(*registers))
                } else {
                    write!(
                        f,
                        "stmdb {}{}, {}",
                        rn,
                        if *wback { "!" } else { "" },
                        PrintRegisterList(*registers)
                    )
                }
            }
            Self::LoadMultiple {
                rn,
                registers,
                wback,
                ..
            } => {
                if *rn == RegisterID::SP && *wback {
                    // `pop { regs }` is a shortcut notation for `ldm sp!, { regs }`
                    write!(f, "pop {}", PrintRegisterList(*registers))
                } else {
                    write!(
                        f,
                        "ldm {}{}, {}",
                        rn,
                        if *wback { "!" } else { "" },
                        PrintRegisterList(*registers)
                    )
                }
            }
            Self::LoadMultipleDecrementBefore {
                rn,
                registers,
                wback,
            } => write!(
                f,
                "ldmdb {}{}, {}",
                rn,
                if *wback { "!" } else { "" },
                PrintRegisterList(*registers)
            ),
        }
    }
}

impl Condition {
    pub(super) const AL: Self = Condition(bsc::C_1110); // condition "always" (=no condition)

    pub fn is_al(self) -> bool {
        self.0 == Self::AL.0
    }

    // [ARM-ARM] A7.3.1 ConditionPassed()
    pub(super) fn passed(self, xpsr: XPSR) -> bool {
        let cond = self.0;
        let result = match bitstring_extract!(cond<3:1> | 3 bits) {
            bsc::C_000 => xpsr.zero_flag(),
            bsc::C_001 => xpsr.carry_flag(),
            bsc::C_010 => xpsr.negative_flag(),
            bsc::C_011 => xpsr.overflow_flag(),
            bsc::C_100 => xpsr.carry_flag() && !xpsr.zero_flag(),
            bsc::C_101 => xpsr.negative_flag() == xpsr.overflow_flag(),
            bsc::C_110 => (xpsr.negative_flag() == xpsr.overflow_flag()) && !xpsr.zero_flag(),
            bsc::C_111 => true,
            _ => unreachable!(),
        };
        if cond.get_bit(0) && cond != bsc::C_1111 {
            !result
        } else {
            result
        }
    }
}

//
// Helper types for printing
//

struct PrintShiftedReg(RegisterID, Shift);
impl fmt::Display for PrintShiftedReg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.1.amount == 0 {
            write!(f, "{}", self.0)
        } else {
            write!(f, "{}, {}", self.0, self.1)
        }
    }
}

struct PrintImm(Word);
impl fmt::Display for PrintImm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#0x{:x}", self.0)
    }
}

struct PrintAddImm(bool, Word);
impl fmt::Display for PrintAddImm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{}0x{:x}", if self.0 { "" } else { "-" }, self.1)
    }
}

struct PrintRegisterList(RegisterBitmap);
impl fmt::Display for PrintRegisterList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn dump_register_range(
            f: &mut fmt::Formatter<'_>,
            beg: RegisterID,
            end: RegisterID,
            write_comma: &mut bool,
        ) -> fmt::Result {
            if *write_comma {
                write!(f, ", ")?;
            }
            *write_comma = true;

            if beg == end {
                write!(f, "{beg}")?;
            } else {
                write!(f, "{beg}-{end}")?;
            }

            Ok(())
        }

        write!(f, "{{")?;
        let mut write_comma = false;
        let mut beg = None;
        let mut end = None;

        for idx in 0..=12 {
            let r = RegisterID::from_index(idx);
            if self.0.get(r) {
                if beg.is_none() {
                    beg = Some(r);
                }
                end = Some(r);
            } else if let Some(rb) = beg.take() {
                let re = end.take().unwrap();
                dump_register_range(f, rb, re, &mut write_comma)?;
            }
        }
        if let Some(rb) = beg.take() {
            let re = end.take().unwrap();
            dump_register_range(f, rb, re, &mut write_comma)?;
        }

        for idx in 13..=15 {
            let r = RegisterID::from_index(idx);
            if self.0.get(r) {
                dump_register_range(f, r, r, &mut write_comma)?;
            }
        }

        write!(f, "}}")
    }
}

struct PrintSysm(Bitstring![8], Option<Bitstring![2]>);
impl PrintSysm {
    fn on_reads(value: Bitstring![8]) -> Self {
        Self(value, None)
    }
    fn on_writes(value: Bitstring![8], mask: Bitstring![2]) -> Self {
        Self(value, Some(mask))
    }
}
impl fmt::Display for PrintSysm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // [ARM-ARM] B5.1.1
        let spec_reg = match (self.0, self.1) {
            (bsc::C_0000_0000, None) => "apsr",
            (bsc::C_0000_0000, Some(bsc::C_10)) => "apsr_nzcvq",
            (bsc::C_0000_0001, None) => "iapsr",
            (bsc::C_0000_0001, Some(bsc::C_10)) => "iapsr_nzcvq",
            (bsc::C_0000_0010, None) => "eapsr",
            (bsc::C_0000_0010, Some(bsc::C_10)) => "eapsr_nzcvq",
            (bsc::C_0000_0011, None) => "xpsr",
            (bsc::C_0000_0011, Some(bsc::C_10)) => "xpsr_nzcvq",
            (
                bsc::C_0000_0000 | bsc::C_0000_0001 | bsc::C_0000_0010 | bsc::C_0000_0011,
                Some(bsc::C_01 | bsc::C_11),
            ) => unimplemented!(
                "attempting to use DSP extension, which is not supported by CherryMote"
            ),
            (bsc::C_0000_0101, _) => "ipsr",
            (bsc::C_0000_0110, _) => "epsr",
            (bsc::C_0000_0111, _) => "iepsr",
            (bsc::C_0000_1000, _) => "msp",
            (bsc::C_0000_1001, _) => "psp",
            (bsc::C_0001_0000, _) => "primask",
            (bsc::C_0001_0001, _) => "basepri",
            (bsc::C_0001_0010, _) => "basepri_max",
            (bsc::C_0001_0011, _) => "faultmask",
            (bsc::C_0001_0100, _) => "control",
            x => panic!("attempting to print invalid sysm {x:?}"),
        };
        write!(f, "{spec_reg}")
    }
}

impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // [ARM-ARM] A7.3
        write!(
            f,
            "{}",
            match self.0 {
                bsc::C_0000 => "eq",
                bsc::C_0001 => "ne",
                bsc::C_0010 => "cs", // "hs"
                bsc::C_0011 => "cc", // "lo"
                bsc::C_0100 => "mi",
                bsc::C_0101 => "pl",
                bsc::C_0110 => "vs",
                bsc::C_0111 => "vc",
                bsc::C_1000 => "hi",
                bsc::C_1001 => "ls",
                bsc::C_1010 => "ge",
                bsc::C_1011 => "lt",
                bsc::C_1100 => "gt",
                bsc::C_1101 => "le",
                bsc::C_1110 => "", // "al"
                _ => unreachable!("Invalid condition"),
            }
        )
    }
}
