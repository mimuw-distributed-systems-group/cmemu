//! README:
//! - the macro takes the following argument:
//!   * unpredictable - a path to unpredictable instruction variant returned
//!     when (0) and (1) bits check fails, see [\[ARM-ARM\]] A5.1.1
//! - the function must take at least one argument; the first argument must be u16 or u32,
//!   it will be used inside `instr!` and `instr_bind!` macro calls
//! - there has to be single match in the function body that matches against the instruction
//! - macro traverses recursively match arms' bodies:
//!   * if it is a single expression (or block with single expression) that is a match against
//!     `instr!` expression, then the macro parses recursively the body;
//!   * if it is a block with first statement being `instr_bind!`, it touches only this macro call;
//!   * if it is a single expression (or block with single expression) that is an if with
//!     `instr_bind!` condition, then the macro creates check for (0) and (1) bits;
//!   * otherwise it doesn't touch the body (hence you can get errors like "undefined macro `instr!`");
//!   * taking a look at recursive traversal of the decoding tree:
//!     the intention is `match instr!` are "inner nodes" while `instr_bind!` are "leaf nodes";
//! - `instr_bind!` binds the matched bits while `instr!` doesn't
//! - the rest should be pretty self-explanatory and error messages should help fixing the problems
//! - each `instr*` call has a comment with the name of a section in docs it corresponds to;
//!   it should be placed in the line preceding the macro call
//!
//!
//! ### Handling `SEE <instruction>` notes in [\[ARM-ARM\]] A7.7.x?
//!
//! There are two cases:
//! 1) the `SEE ...` note is unreachable (because of earlier condition checks)
//! 2) the `SEE ...` note is reachable – we should put there specific encoding parsing
//!
//! * Put `unreachable!("SEE <instr>")` when you are sure that the `SEE` note is unreachable.
//! * Put `unimplemented!("SEE <instr>")` when the `SEE` note is reachable,
//!   but decoding `<instr>` is unsupported now.
//! * Put `unimplemented!("SEE <instr>")` when you don't know if case 1 or 2 holds.
//!
//! Please, try not to leave any `unimplemented!("SEE <instr>")` when implementing
//! decode step for instruction `<instr>`.
//!
//! If unreachability is not obvious, i.e. register value on "match instr!"
//! one level higher, please leave comment with an explanation.
//!
//! ### Docs
//!
//! \[ARM-ARM\]: <https://static.docs.arm.com/ddi0403/ed/DDI0403E_d_armv7m_arm.pdf>
//!
//!
//! [\[ARM-ARM\]]: https://static.docs.arm.com/ddi0403/ed/DDI0403E_d_armv7m_arm.pdf
//!                "ARMv7-M Architecture Reference Manual"

use crate::common::{BitstringUtils, SRType, Shift, Word, bitstring::constants as bsc};
use crate::component::core::{
    builtins::have_dsp_ext,
    instruction::{Condition, Instruction},
    register_bank::{RegisterID, XPSR},
};
use crate::{Bitstring, bitstring_concat, bitstring_extract};
use cmemu_proc_macros::decode_instr;

// TODO:
//    All instructions listed by [TI-TRM] are decoded. However some instructions
//    should be supported according to [ARM-ARM], but they are not according
//    to [TI-TRM]. It should be researched. It's worth consulting [ARM-TDG], too.

/// Helper macro to convert bitstring to `u32`.
///
/// Used to shorten conversions. Also improves readability and looks more similar to the docs.
macro_rules! rebind_as_u32 {
    ( $($name:ident;)+ ) => {
        let ( $($name,)* ) = ( $(u32::from($name),)* );
    };
}

#[allow(clippy::cognitive_complexity, clippy::too_many_lines)]
#[decode_instr(unpredictable = Instruction::Unpredictable)]
pub(super) fn decode_short_instruction(instr: u16, xpsr: XPSR) -> Instruction {
    // A5.2
    match instr!("<opcode:6>|xxxxxxxxxx" in order (opcode,)) {
        // A5.2.1
        ("00xxxx") => match instr!("00|<opcode:5>|xxxxxxxxx" in order (opcode,)) {
            ("000xx") => {
                // A7.7.68, T1
                instr_bind!("000|00|<imm5:5>|<rm:3>|<rd:3>" identifiers (imm5, rm, rd));
                if imm5 == bsc::C_0_0000 {
                    // SEE MOV (register)
                    // A7.7.77, T2
                    if xpsr.in_it_block() {
                        Instruction::Unpredictable
                    } else {
                        Instruction::Move_Register {
                            rd: RegisterID::from_index(rd),
                            rm: RegisterID::from_index(rm),
                            setflags: true,
                        }
                    }
                } else {
                    Instruction::LogicalShiftLeft_Immediate {
                        rd: RegisterID::from_index(rd),
                        rm: RegisterID::from_index(rm),
                        shift: Shift::decode_imm_shift(bsc::C_00, imm5),
                        setflags: !xpsr.in_it_block(),
                        setflags_depends_on_it: true,
                    }
                }
            }
            ("001xx") => {
                // A7.7.70, T1
                instr_bind!("000|01|<imm5:5>|<rm:3>|<rd:3>" identifiers (imm5, rm, rd));
                Instruction::LogicalShiftRight_Immediate {
                    rd: RegisterID::from_index(rd),
                    rm: RegisterID::from_index(rm),
                    shift: Shift::decode_imm_shift(bsc::C_01, imm5),
                    setflags: !xpsr.in_it_block(),
                    setflags_depends_on_it: true,
                }
            }
            ("010xx") => {
                // A7.7.10, T1
                instr_bind!("000|10|<imm5:5>|<rm:3>|<rd:3>" identifiers (imm5, rm, rd));
                Instruction::ArithmeticShiftRight_Immediate {
                    rd: RegisterID::from_index(rd),
                    rm: RegisterID::from_index(rm),
                    shift: Shift::decode_imm_shift(bsc::C_10, imm5),
                    setflags: !xpsr.in_it_block(),
                    setflags_depends_on_it: true,
                }
            }
            ("01100") => {
                // A7.7.4, T1
                instr_bind!("000|11|0|0|<rm:3>|<rn:3>|<rd:3>" identifiers (rm, rn, rd));
                Instruction::Add_Register {
                    rd: RegisterID::from_index(rd),
                    rn: RegisterID::from_index(rn),
                    rm: RegisterID::from_index(rm),
                    shift: Shift::LSL(0),
                    setflags: !xpsr.in_it_block(),
                    setflags_depends_on_it: true,
                }
            }
            ("01101") => {
                // A7.7.175, T1
                instr_bind!("000|11|0|1|<rm:3>|<rn:3>|<rd:3>" identifiers (rm, rn, rd));
                Instruction::Subtract_Register {
                    rd: RegisterID::from_index(rd),
                    rn: RegisterID::from_index(rn),
                    rm: RegisterID::from_index(rm),
                    shift: Shift::LSL(0),
                    setflags: !xpsr.in_it_block(),
                    setflags_depends_on_it: true,
                }
            }
            ("01110") => {
                // A7.7.3, T1
                instr_bind!("000|11|1|0|<imm3:3>|<rn:3>|<rd:3>" identifiers (imm3, rn, rd));
                Instruction::Add_Immediate {
                    rd: RegisterID::from_index(rd),
                    rn: RegisterID::from_index(rn),
                    imm32: imm3.zero_extend(),
                    setflags: !xpsr.in_it_block(),
                    setflags_depends_on_it: true,
                }
            }
            ("01111") => {
                // A7.7.174, T1
                instr_bind!("000|11|1|1|<imm3:3>|<rn:3>|<rd:3>" identifiers (imm3, rn, rd));
                Instruction::Subtract_Immediate {
                    rd: RegisterID::from_index(rd),
                    rn: RegisterID::from_index(rn),
                    imm32: imm3.zero_extend(),
                    setflags: !xpsr.in_it_block(),
                    setflags_depends_on_it: true,
                }
            }
            ("100xx") => {
                // A7.7.76, T1
                instr_bind!("001|00|<rd:3>|<imm8:8>" identifiers (rd, imm8));
                Instruction::Move_Immediate {
                    rd: RegisterID::from_index(rd),
                    imm32: imm8.zero_extend(),
                    setflags: !xpsr.in_it_block(),
                    setflags_depends_on_it: true,
                    carry: xpsr.carry_flag(),
                }
            }
            ("101xx") => {
                // A7.7.27, T1
                instr_bind!("001|01|<rn:3>|<imm8:8>" identifiers (rn, imm8));
                Instruction::Compare_Immediate {
                    rn: RegisterID::from_index(rn),
                    imm32: imm8.zero_extend(),
                }
            }
            ("110xx") => {
                // A7.7.3, T2
                instr_bind!("001|10|<rdn:3>|<imm8:8>" identifiers (rdn, imm8));
                Instruction::Add_Immediate {
                    rd: RegisterID::from_index(rdn),
                    rn: RegisterID::from_index(rdn),
                    imm32: imm8.zero_extend(),
                    setflags: !xpsr.in_it_block(),
                    setflags_depends_on_it: true,
                }
            }
            ("111xx") => {
                // A7.7.174, T2
                instr_bind!("001|11|<rdn:3>|<imm8:8>" identifiers (rdn, imm8));
                Instruction::Subtract_Immediate {
                    rd: RegisterID::from_index(rdn),
                    rn: RegisterID::from_index(rdn),
                    imm32: imm8.zero_extend(),
                    setflags: !xpsr.in_it_block(),
                    setflags_depends_on_it: true,
                }
            }
            _ => unreachable!("All cases are covered."),
        },
        // A5.2.2
        ("010000") => match instr!("010000|<opcode:4>|xxxxxx" in order (opcode,)) {
            ("0000") => {
                // A7.7.9, T1
                instr_bind!("010000|0000|<rm:3>|<rdn:3>" identifiers (rm, rdn));
                Instruction::And_Register {
                    rd: RegisterID::from_index(rdn),
                    rn: RegisterID::from_index(rdn),
                    rm: RegisterID::from_index(rm),
                    shift: Shift::LSL(0),
                    setflags: !xpsr.in_it_block(),
                    setflags_depends_on_it: true,
                }
            }
            ("0001") => {
                // A7.7.36, T1
                instr_bind!("010000|0001|<rm:3>|<rdn:3>" identifiers (rm, rdn));
                Instruction::ExclusiveOr_Register {
                    rd: RegisterID::from_index(rdn),
                    rn: RegisterID::from_index(rdn),
                    rm: RegisterID::from_index(rm),
                    shift: Shift::LSL(0),
                    setflags: !xpsr.in_it_block(),
                    setflags_depends_on_it: true,
                }
            }
            ("0010") => {
                // A7.7.69, T1
                instr_bind!("010000|0010|<rm:3>|<rdn:3>" identifiers (rm, rdn));
                Instruction::LogicalShiftLeft_Register {
                    rd: RegisterID::from_index(rdn),
                    rn: RegisterID::from_index(rdn),
                    rm: RegisterID::from_index(rm),
                    setflags: !xpsr.in_it_block(),
                    setflags_depends_on_it: true,
                }
            }
            ("0011") => {
                // A7.7.71, T1
                instr_bind!("010000|0011|<rm:3>|<rdn:3>" identifiers (rm, rdn));
                Instruction::LogicalShiftRight_Register {
                    rd: RegisterID::from_index(rdn),
                    rn: RegisterID::from_index(rdn),
                    rm: RegisterID::from_index(rm),
                    setflags: !xpsr.in_it_block(),
                    setflags_depends_on_it: true,
                }
            }
            ("0100") => {
                // A7.7.11, T1
                instr_bind!("010000|0100|<rm:3>|<rdn:3>" identifiers (rm, rdn));
                Instruction::ArithmeticShiftRight_Register {
                    rd: RegisterID::from_index(rdn),
                    rn: RegisterID::from_index(rdn),
                    rm: RegisterID::from_index(rm),
                    setflags: !xpsr.in_it_block(),
                    setflags_depends_on_it: true,
                }
            }
            ("0101") => {
                // A7.7.2, T1
                instr_bind!("010000|0101|<rm:3>|<rdn:3>" identifiers (rm, rdn));
                Instruction::AddWithCarry_Register {
                    rd: RegisterID::from_index(rdn),
                    rn: RegisterID::from_index(rdn),
                    rm: RegisterID::from_index(rm),
                    shift: Shift::LSL(0),
                    setflags: !xpsr.in_it_block(),
                    setflags_depends_on_it: true,
                }
            }
            ("0110") => {
                // A7.7.125, T1
                instr_bind!("010000|0110|<rm:3>|<rdn:3>" identifiers (rm, rdn));
                Instruction::SubtractWithCarry_Register {
                    rd: RegisterID::from_index(rdn),
                    rn: RegisterID::from_index(rdn),
                    rm: RegisterID::from_index(rm),
                    shift: Shift::LSL(0),
                    setflags: !xpsr.in_it_block(),
                    setflags_depends_on_it: true,
                }
            }
            ("0111") => {
                // A7.7.117, T1
                instr_bind!("010000|0111|<rm:3>|<rdn:3>" identifiers (rm, rdn));
                Instruction::RotateRight_Register {
                    rd: RegisterID::from_index(rdn),
                    rn: RegisterID::from_index(rdn),
                    rm: RegisterID::from_index(rm),
                    setflags: !xpsr.in_it_block(),
                    setflags_depends_on_it: true,
                }
            }
            ("1000") => {
                // A7.7.189, T1
                instr_bind!("010000|1000|<rm:3>|<rn:3>" identifiers (rm, rn));
                Instruction::Test_Register {
                    rn: RegisterID::from_index(rn),
                    rm: RegisterID::from_index(rm),
                    shift: Shift::LSL(0),
                }
            }
            ("1001") => {
                // A7.7.119, T1
                instr_bind!("010000|1001|<rn:3>|<rd:3>" identifiers (rn, rd));
                Instruction::ReverseSubtract_Immediate {
                    rd: RegisterID::from_index(rd),
                    rn: RegisterID::from_index(rn),
                    imm32: Word::from(0),
                    setflags: !xpsr.in_it_block(),
                    setflags_depends_on_it: true,
                }
            }
            ("1010") => {
                // A7.7.28, T1
                instr_bind!("010000|1010|<rm:3>|<rn:3>" identifiers (rm, rn));
                Instruction::Compare_Register {
                    rn: RegisterID::from_index(rn),
                    rm: RegisterID::from_index(rm),
                    shift: Shift::LSL(0),
                }
            }
            ("1011") => {
                // A7.7.26, T1
                instr_bind!("010000|1011|<rm:3>|<rn:3>" identifiers (rm, rn));
                Instruction::CompareNegative_Register {
                    rn: RegisterID::from_index(rn),
                    rm: RegisterID::from_index(rm),
                    shift: Shift::LSL(0),
                }
            }
            ("1100") => {
                // A7.7.92, T1
                instr_bind!("010000|1100|<rm:3>|<rdn:3>" identifiers (rm, rdn));
                Instruction::LogicalOr_Register {
                    rd: RegisterID::from_index(rdn),
                    rn: RegisterID::from_index(rdn),
                    rm: RegisterID::from_index(rm),
                    shift: Shift::LSL(0),
                    setflags: !xpsr.in_it_block(),
                    setflags_depends_on_it: true,
                }
            }
            ("1101") => {
                // A7.7.84, T1
                instr_bind!("010000|1101|<rn:3>|<rdm:3>" identifiers (rn, rdm));
                Instruction::Multiply {
                    rd: RegisterID::from_index(rdm),
                    rn: RegisterID::from_index(rn),
                    rm: RegisterID::from_index(rdm),
                    setflags: !xpsr.in_it_block(),
                    setflags_depends_on_it: true,
                }
            }
            ("1110") => {
                // A7.7.16, T1
                instr_bind!("010000|1110|<rm:3>|<rdn:3>" identifiers (rm, rdn));
                Instruction::BitClear_Register {
                    rd: RegisterID::from_index(rdn),
                    rn: RegisterID::from_index(rdn),
                    rm: RegisterID::from_index(rm),
                    shift: Shift::LSL(0),
                    setflags: !xpsr.in_it_block(),
                    setflags_depends_on_it: true,
                }
            }
            ("1111") => {
                // A7.7.86, T1
                instr_bind!("010000|1111|<rm:3>|<rd:3>" identifiers (rm, rd));
                Instruction::BitwiseNot_Register {
                    rd: RegisterID::from_index(rd),
                    rm: RegisterID::from_index(rm),
                    shift: Shift::LSL(0),
                    setflags: !xpsr.in_it_block(),
                    setflags_depends_on_it: true,
                }
            }
            _ => unreachable!("All cases are covered."),
        },
        // A5.2.3
        ("010001") => match instr!("010001|<opcode:4>|xxxxxx" in order (opcode,)) {
            ("00xx") => {
                // A7.7.4, T2
                instr_bind!("010001|00|<dn:1>|<rm:4>|<rdn:3>" identifiers (dn, rm, rdn));
                let reg_dn = bitstring_concat!(dn : rdn | 4 bits);
                if reg_dn == bsc::C_1101 || rm == bsc::C_1101 {
                    // SEE ADD (SP plus register)
                    // A7.7.6, T2
                    if rm == bsc::C_1101 {
                        // SEE encoding T1
                        // A7.7.6, T1
                        // NOTE: As we can't run instr_bind macro second time: reg_dm == reg_dn
                        rebind_as_u32! {reg_dn;}
                        if reg_dn == 15 && xpsr.in_it_block() && !xpsr.last_in_it_block() {
                            Instruction::Unpredictable
                        } else {
                            Instruction::Add_SPPlusRegister {
                                rd: RegisterID::from_index(reg_dn),
                                rm: RegisterID::from_index(reg_dn),
                                shift: Shift::LSL(0),
                                setflags: false,
                            }
                        }
                    } else {
                        rebind_as_u32! {rm;}
                        Instruction::Add_SPPlusRegister {
                            rd: RegisterID::from_index(13),
                            rm: RegisterID::from_index(rm),
                            shift: Shift::LSL(0),
                            setflags: false,
                        }
                    }
                } else {
                    rebind_as_u32! {reg_dn; rm;}
                    if (reg_dn == 15 && xpsr.in_it_block() && !xpsr.last_in_it_block())
                        || (reg_dn == 15 && rm == 15)
                    {
                        Instruction::Unpredictable
                    } else {
                        Instruction::Add_Register {
                            rd: RegisterID::from_index(reg_dn),
                            rn: RegisterID::from_index(reg_dn),
                            rm: RegisterID::from_index(rm),
                            shift: Shift::LSL(0),
                            setflags: false,
                            setflags_depends_on_it: false,
                        }
                    }
                }
            }
            ("0100") => Instruction::Unpredictable,
            ("0101") | ("011x") => {
                // A7.7.28, T2
                instr_bind!("010001|01|<n:1>|<rm:4>|<rn:3>" identifiers (n, rm, rn));
                let reg_n = bitstring_concat!(n : rn | 4 bits);
                rebind_as_u32! {reg_n; rm;}
                if (reg_n < 8 && rm < 8) || reg_n == 15 || rm == 15 {
                    Instruction::Unpredictable
                } else {
                    Instruction::Compare_Register {
                        rn: RegisterID::from_index(reg_n),
                        rm: RegisterID::from_index(rm),
                        shift: Shift::LSL(0),
                    }
                }
            }
            ("10xx") => {
                // A7.7.77, T1
                instr_bind!("010001|10|<d:1>|<rm:4>|<rd:3>" identifiers (d, rm, rd));
                let reg_d = bitstring_concat!(d : rd | 4 bits);
                rebind_as_u32! {reg_d;}
                if reg_d == 15 && xpsr.in_it_block() && !xpsr.last_in_it_block() {
                    Instruction::Unpredictable
                } else {
                    Instruction::Move_Register {
                        rd: RegisterID::from_index(reg_d),
                        rm: RegisterID::from_index(rm),
                        setflags: false,
                    }
                }
            }
            ("110x") => {
                // A7.7.20, T1
                if instr_bind!("010001|11|0|<rm:4>|(0)(0)(0)" identifiers (rm,)) {
                    if xpsr.in_it_block() && !xpsr.last_in_it_block() {
                        Instruction::Unpredictable
                    } else {
                        Instruction::BranchAndExchange {
                            rm: RegisterID::from_index(rm),
                        }
                    }
                }
            }
            ("111x") => {
                // A7.7.19, T1
                if instr_bind!("010001|11|1|<rm:4>|(0)(0)(0)" identifiers (rm,)) {
                    rebind_as_u32! {rm;}
                    if rm == 15 || (xpsr.in_it_block() && !xpsr.last_in_it_block()) {
                        Instruction::Unpredictable
                    } else {
                        Instruction::BranchWithLinkAndExchange_Register {
                            rm: RegisterID::from_index(rm),
                        }
                    }
                }
            }
            _ => unreachable!("All cases are covered."),
        },
        ("01001x") => {
            // A7.7.44, T1
            instr_bind!("01001|<rt:3>|<imm8:8>" identifiers (rt, imm8));
            Instruction::LoadRegister_Literal {
                rt: RegisterID::from_index(rt),
                imm32: bitstring_concat!(imm8 : bsc::C_00 | 10 bits).zero_extend(),
                add: true,
            }
        }
        ("0101xx") | ("011xxx") | ("100xxx") => {
            // A5.2.4
            match instr!("<opa:4>|<opb:3>|xxxxxxxxx" in order (opa, opb)) {
                ("0101", "000") => {
                    // A7.7.162, T1
                    instr_bind!("0101|000|<rm:3>|<rn:3>|<rt:3>" identifiers (rm, rn, rt));
                    Instruction::StoreRegister_Register {
                        rt: RegisterID::from_index(rt),
                        rn: RegisterID::from_index(rn),
                        rm: RegisterID::from_index(rm),
                        shift: Shift::LSL(0),

                        index: true,
                        add: true,
                        wback: false,
                    }
                }
                ("0101", "001") => {
                    // A7.7.171, T1
                    instr_bind!("0101|001|<rm:3>|<rn:3>|<rt:3>" identifiers (rm, rn, rt));
                    Instruction::StoreRegisterHalfword_Register {
                        rt: RegisterID::from_index(rt),
                        rn: RegisterID::from_index(rn),
                        rm: RegisterID::from_index(rm),
                        shift: Shift::LSL(0),

                        index: true,
                        add: true,
                        wback: false,
                    }
                }
                ("0101", "010") => {
                    // A7.7.164, T1
                    instr_bind!("0101|010|<rm:3>|<rn:3>|<rt:3>" identifiers (rm, rn, rt));
                    Instruction::StoreRegisterByte_Register {
                        rt: RegisterID::from_index(rt),
                        rn: RegisterID::from_index(rn),
                        rm: RegisterID::from_index(rm),
                        shift: Shift::LSL(0),

                        index: true,
                        add: true,
                        wback: false,
                    }
                }
                ("0101", "011") => {
                    // A7.7.61, T1
                    instr_bind!("0101|011|<rm:3>|<rn:3>|<rt:3>" identifiers (rm, rn, rt));
                    Instruction::LoadRegisterSignedByte_Register {
                        rt: RegisterID::from_index(rt),
                        rn: RegisterID::from_index(rn),
                        rm: RegisterID::from_index(rm),
                        shift: Shift::LSL(0),

                        index: true,
                        add: true,
                        wback: false,
                    }
                }
                ("0101", "100") => {
                    // A7.7.45, T1
                    instr_bind!("0101|100|<rm:3>|<rn:3>|<rt:3>" identifiers (rm, rn, rt));
                    Instruction::LoadRegister_Register {
                        rt: RegisterID::from_index(rt),
                        rn: RegisterID::from_index(rn),
                        rm: RegisterID::from_index(rm),
                        shift: Shift::LSL(0),

                        index: true,
                        add: true,
                        wback: false,
                    }
                }
                ("0101", "101") => {
                    // A7.7.57, T1
                    instr_bind!("0101|101|<rm:3>|<rn:3>|<rt:3>" identifiers (rm, rn, rt));
                    Instruction::LoadRegisterHalfword_Register {
                        rt: RegisterID::from_index(rt),
                        rn: RegisterID::from_index(rn),
                        rm: RegisterID::from_index(rm),
                        shift: Shift::LSL(0),

                        index: true,
                        add: true,
                        wback: false,
                    }
                }
                ("0101", "110") => {
                    // A7.7.48, T1
                    instr_bind!("0101|110|<rm:3>|<rn:3>|<rt:3>" identifiers (rm, rn, rt));
                    Instruction::LoadRegisterByte_Register {
                        rt: RegisterID::from_index(rt),
                        rn: RegisterID::from_index(rn),
                        rm: RegisterID::from_index(rm),
                        shift: Shift::LSL(0),

                        index: true,
                        add: true,
                        wback: false,
                    }
                }
                ("0101", "111") => {
                    // A7.7.65, T1
                    instr_bind!("0101|111|<rm:3>|<rn:3>|<rt:3>" identifiers (rm, rn, rt));
                    Instruction::LoadRegisterSignedHalfword_Register {
                        rt: RegisterID::from_index(rt),
                        rn: RegisterID::from_index(rn),
                        rm: RegisterID::from_index(rm),
                        shift: Shift::LSL(0),

                        index: true,
                        add: true,
                        wback: false,
                    }
                }
                ("0110", "0xx") => {
                    // A7.7.161, T1
                    instr_bind!("011|0|0|<imm5:5>|<rn:3>|<rt:3>" identifiers (imm5, rn, rt));
                    Instruction::StoreRegister_Immediate {
                        rt: RegisterID::from_index(rt),
                        rn: RegisterID::from_index(rn),
                        imm32: bitstring_concat!(imm5 : bsc::C_00 | 7 bits).zero_extend(),

                        index: true,
                        add: true,
                        wback: false,
                    }
                }
                ("0110", "1xx") => {
                    // A7.7.43, T1
                    instr_bind!("011|0|1|<imm5:5>|<rn:3>|<rt:3>" identifiers (imm5, rn, rt));
                    Instruction::LoadRegister_Immediate {
                        rt: RegisterID::from_index(rt),
                        rn: RegisterID::from_index(rn),
                        imm32: bitstring_concat!(imm5 : bsc::C_00 | 7 bits).zero_extend(),

                        index: true,
                        add: true,
                        wback: false,
                    }
                }
                ("0111", "1xx") => {
                    // A7.7.46, T1
                    instr_bind!("011|1|1|<imm5:5>|<rn:3>|<rt:3>" identifiers (imm5, rn, rt));
                    Instruction::LoadRegisterByte_Immediate {
                        rt: RegisterID::from_index(rt),
                        rn: RegisterID::from_index(rn),
                        imm32: imm5.zero_extend(),

                        index: true,
                        add: true,
                        wback: false,
                    }
                }
                ("0111", "0xx") => {
                    // A7.7.163, T1
                    instr_bind!("011|1|0|<imm5:5>|<rn:3>|<rt:3>" identifiers (imm5, rn, rt));
                    Instruction::StoreRegisterByte_Immediate {
                        rt: RegisterID::from_index(rt),
                        rn: RegisterID::from_index(rn),
                        imm32: imm5.zero_extend(),

                        index: true,
                        add: true,
                        wback: false,
                    }
                }
                ("1000", "0xx") => {
                    // A7.7.170, T1
                    instr_bind!("1000|0|<imm5:5>|<rn:3>|<rt:3>" identifiers (imm5, rn, rt));
                    Instruction::StoreRegisterHalfword_Immediate {
                        rt: RegisterID::from_index(rt),
                        rn: RegisterID::from_index(rn),
                        imm32: bitstring_concat!(imm5 : bsc::C_0 | 6 bits).zero_extend(),

                        index: true,
                        add: true,
                        wback: false,
                    }
                }
                ("1000", "1xx") => {
                    // A7.7.55, T1
                    instr_bind!("1000|1|<imm5:5>|<rn:3>|<rt:3>" identifiers (imm5, rn, rt));
                    Instruction::LoadRegisterHalfword_Immediate {
                        rt: RegisterID::from_index(rt),
                        rn: RegisterID::from_index(rn),
                        imm32: bitstring_concat!(imm5 : bsc::C_0 | 6 bits).zero_extend(),

                        index: true,
                        add: true,
                        wback: false,
                    }
                }
                ("1001", "0xx") => {
                    // A7.7.161, T2
                    instr_bind!("1001|0|<rt:3>|<imm8:8>" identifiers (rt, imm8));
                    Instruction::StoreRegister_Immediate {
                        rt: RegisterID::from_index(rt),
                        rn: RegisterID::SP,
                        imm32: bitstring_concat!(imm8 : bsc::C_00 | 10 bits).zero_extend(),

                        index: true,
                        add: true,
                        wback: false,
                    }
                }
                ("1001", "1xx") => {
                    // A7.7.43, T2
                    instr_bind!("1001|1|<rt:3>|<imm8:8>" identifiers (rt, imm8));
                    Instruction::LoadRegister_Immediate {
                        rt: RegisterID::from_index(rt),
                        rn: RegisterID::SP,
                        imm32: bitstring_concat!(imm8 : bsc::C_00 | 10 bits).zero_extend(),

                        index: true,
                        add: true,
                        wback: false,
                    }
                }
                _ => unreachable!("All cases are covered."),
            }
        }
        ("10100x") => {
            // A7.7.7, T1
            instr_bind!("1010|0|<rd:3>|<imm8:8>" identifiers (rd, imm8));
            Instruction::AddressToRegister {
                rd: RegisterID::from_index(rd),
                imm32: bitstring_concat!(imm8 : bsc::C_00 | 10 bits).zero_extend(),
                add: true,
            }
        }
        ("10101x") => {
            // A7.7.5, T1
            instr_bind!("1010|1|<rd:3>|<imm8:8>" identifiers (rd, imm8));
            Instruction::Add_SPPlusImmediate {
                rd: RegisterID::from_index(rd),
                imm32: bitstring_concat!(imm8 : bsc::C_00 | 10 bits).zero_extend(),
                setflags: false,
            }
        }
        // A5.2.5
        ("1011xx") => match instr!("1011|<opcode:7>|xxxxx" in order (opcode,)) {
            ("0110011") => {
                // A7.7.29, T1
                // B5.2.1, T1
                if instr_bind!("1011|0110|011|<im:1>|(0)|(0)|<i:1>|<f:1>" identifiers (im, i, f)) {
                    if (i == bsc::C_0 && f == bsc::C_0) || xpsr.in_it_block() {
                        Instruction::Unpredictable
                    } else {
                        Instruction::ChangeProcessorState {
                            enable: im == bsc::C_0,
                            disable: im == bsc::C_1,
                            affect_pri: i == bsc::C_1,
                            affect_fault: f == bsc::C_1,
                        }
                    }
                }
            }
            ("00000xx") => {
                // A7.7.5, T2
                instr_bind!("1011|0000|0|<imm7:7>" identifiers (imm7,));
                Instruction::Add_SPPlusImmediate {
                    rd: RegisterID::from_index(13),
                    imm32: bitstring_concat!(imm7 : bsc::C_00 | 9 bits).zero_extend(),
                    setflags: false,
                }
            }
            ("00001xx") => {
                // A7.7.176, T1
                instr_bind!("1011|0000|1|<imm7:7>" identifiers (imm7,));
                Instruction::Subtract_SPMinusImmediate {
                    rd: RegisterID::from_index(13),
                    imm32: bitstring_concat!(imm7 : bsc::C_00 | 9 bits).zero_extend(),
                    setflags: false,
                }
            }
            ("0001xxx") | ("0011xxx") | ("1001xxx") | ("1011xxx") => {
                // A7.7.21, T1
                instr_bind!("1011|<op:1>|0|<i:1>|1|<imm5:5>|<rn:3>" identifiers (op, i, imm5, rn));
                if xpsr.in_it_block() {
                    Instruction::Unpredictable
                } else {
                    Instruction::CompareAndBranch {
                        rn: RegisterID::from_index(rn),
                        imm32: bitstring_concat!(i : imm5 : bsc::C_0 | 7 bits).zero_extend(),
                        nonzero: op == bsc::C_1,
                    }
                }
            }
            ("001000x") => {
                // A7.7.184, T1
                instr_bind!("1011|0010|00|<rm:3>|<rd:3>" identifiers (rm, rd));
                Instruction::SignedExtendHalfword {
                    rd: RegisterID::from_index(rd),
                    rm: RegisterID::from_index(rm),
                    rotation: Shift {
                        srtype: SRType::ROR,
                        amount: 0,
                    },
                }
            }
            ("001001x") => {
                // A7.7.182, T1
                instr_bind!("1011|0010|01|<rm:3>|<rd:3>" identifiers (rm, rd));
                Instruction::SignedExtendByte {
                    rd: RegisterID::from_index(rd),
                    rm: RegisterID::from_index(rm),
                    rotation: Shift {
                        srtype: SRType::ROR,
                        amount: 0,
                    },
                }
            }
            ("001010x") => {
                // A7.7.223, T1
                instr_bind!("1011|0010|10|<rm:3>|<rd:3>" identifiers (rm, rd));
                Instruction::UnsignedExtendHalfword {
                    rd: RegisterID::from_index(rd),
                    rm: RegisterID::from_index(rm),
                    rotation: Shift {
                        srtype: SRType::ROR,
                        amount: 0,
                    },
                }
            }
            ("001011x") => {
                // A7.7.221, T1
                instr_bind!("1011|0010|11|<rm:3>|<rd:3>" identifiers (rm, rd));
                Instruction::UnsignedExtendByte {
                    rd: RegisterID::from_index(rd),
                    rm: RegisterID::from_index(rm),
                    rotation: Shift {
                        srtype: SRType::ROR,
                        amount: 0,
                    },
                }
            }
            ("010xxxx") => {
                // A7.7.101, T1

                // NOTE: `push.n { regs }` seems to be a shortcut notation for `stmdb sp!, { regs }`.
                //       (see: `cmemu-lib/src/component/core/instruction.rs` ctrl+f "A7.7.101")
                //
                //       Therefore we decode this encoding to `StoreMultipleDecrementBefore`.
                instr_bind!("1011|0|10|<m:1>|<register_list:8>" identifiers (m, register_list));
                let registers =
                    bitstring_concat!(bsc::C_0 : m : bsc::C_00_0000 : register_list | 16 bits);
                if registers.bit_count() < 1 {
                    Instruction::Unpredictable
                } else {
                    Instruction::StoreMultipleDecrementBefore {
                        rn: RegisterID::SP,
                        registers: registers.into(),
                        wback: true,
                    }
                }
            }
            ("101000x") => {
                // A7.7.113, T1
                instr_bind!("1011|1010|00|<rm:3>|<rd:3>" identifiers (rm, rd));
                Instruction::ByteReverseWord {
                    rd: RegisterID::from_index(rd),
                    rm: RegisterID::from_index(rm),
                }
            }
            ("101001x") => {
                // A7.7.114, T1
                instr_bind!("1011|1010|01|<rm:3>|<rd:3>" identifiers (rm, rd));
                Instruction::ByteReversePackedHalfword {
                    rd: RegisterID::from_index(rd),
                    rm: RegisterID::from_index(rm),
                }
            }
            ("101011x") => {
                // A7.7.115, T1
                instr_bind!("1011|1010|11|<rm:3>|<rd:3>" identifiers (rm, rd));
                Instruction::ByteReverseSignedHalfword {
                    rd: RegisterID::from_index(rd),
                    rm: RegisterID::from_index(rm),
                }
            }
            ("110xxxx") => {
                // A7.7.99, T1

                // NOTE: `pop.n { regs }` seems to be a shortcut notation for `ldm sp!, { regs }`.
                //       (see: `cmemu-lib/src/component/core/instruction.rs` ctrl+f "A7.7.99")
                //
                //       Therefore we decode this encoding to `LoadMultiple`.
                instr_bind!("1011|1|10|<p:1>|<register_list:8>" identifiers (p, register_list));
                let registers = bitstring_concat!(p : bsc::C_000_0000 : register_list | 16 bits);
                if registers.bit_count() < 1
                    || (bitstring_extract!(registers<15> | 1 bits) == bsc::C_1
                        && xpsr.in_it_block()
                        && !xpsr.last_in_it_block())
                {
                    Instruction::Unpredictable
                } else {
                    Instruction::LoadMultiple {
                        rn: RegisterID::SP,
                        registers: registers.into(),
                        wback: true,
                        is_narrow: true,
                    }
                }
            }
            ("1110xxx") => {
                // A7.7.17, T1
                instr_bind!("1011|1110|<imm8:8>" identifiers (imm8,));
                Instruction::Breakpoint {
                    imm32: imm8.zero_extend(),
                }
            }
            // A5.2.5 If-Then, and hints
            ("1111xxx") => match instr!("1011|1111|<opA:4>|<opB:4>" in order (opA, opB)) {
                // A7.7.38, T1
                ("xxxx", "not 0000") => {
                    instr_bind!("1011|1111|<firstcond:4>|<mask:4>" identifiers (firstcond, mask));
                    if mask == bsc::C_0000 {
                        unreachable!("SEE \"Related encodings\"");
                    } else if firstcond == bsc::C_1111
                        || (firstcond == bsc::C_1110 && mask.bit_count() != 1)
                        || xpsr.in_it_block()
                    {
                        Instruction::Unpredictable
                    } else {
                        Instruction::IfThen {
                            firstcond: Condition(firstcond),
                            mask,
                        }
                    }
                }
                ("0000", "0000") => {
                    // A7.7.88, T1
                    instr_bind!("1011|1111|0000|0000" identifiers ());
                    Instruction::NoOperation
                }
                ("0001", "0000") => Instruction::Unsupported { name: "YIELD" }, // Most likely a NOP ([ARM-ARM] A5.2.5 & [ARM-TDG] 4.2.1).
                ("0010", "0000") => {
                    // A7.7.261, T1
                    instr_bind!("1011|1111|0010|0000" identifiers ());
                    Instruction::WaitForEvent
                }
                ("0011", "0000") => {
                    // A7.7.262, T1
                    instr_bind!("1011|1111|0011|0000" identifiers ());
                    Instruction::WaitForInterrupt
                }
                ("0100", "0000") => {
                    // A7.7.129, T1
                    instr_bind!("1011|1111|0100|0000" identifiers ());
                    Instruction::SendEvent
                }
                _ => Instruction::Unsupported {
                    name: "Unallocated hint. Executes as NOP, but software must not use it.", // So panic during execution attempt is desired.
                },
            },
            _ => Instruction::Undefined,
        },
        ("11000x") => {
            // A7.7.159, T1
            instr_bind!("1100|0|<rn:3>|<register_list:8>" identifiers (rn, register_list));
            let registers = bitstring_concat!(bsc::C_0000_0000 : register_list | 16 bits);

            // TODO: If the base register is not the lowest-numbered register in the
            //       list, such an instruction stores an UNKNOWN value for the base register.
            if registers.bit_count() < 1 {
                Instruction::Unpredictable
            } else {
                Instruction::StoreMultiple {
                    rn: RegisterID::from_index(rn),
                    registers: registers.into(),
                    wback: true,
                }
            }
        }
        ("11001x") => {
            // A7.7.41, T1
            instr_bind!("1100|1|<rn:3>|<register_list:8>" identifiers (rn, register_list));
            rebind_as_u32! {rn;}
            let registers = bitstring_concat!(bsc::C_0000_0000 : register_list | 16 bits);
            let wback = !registers.get_bit(rn);

            if registers.bit_count() < 1 {
                Instruction::Unpredictable
            } else {
                Instruction::LoadMultiple {
                    rn: RegisterID::from_index(rn),
                    registers: registers.into(),
                    wback,
                    is_narrow: true,
                }
            }
        }
        // A5.2.6
        ("1101xx") => match instr!("1101|<opcode:4>|xxxxxxxx" in order (opcode,)) {
            ("not 111x") => {
                // A7.7.12, T1
                instr_bind!("1101|<cond:4>|<imm8:8>" identifiers (cond, imm8));
                if cond == bsc::C_1110 {
                    unreachable!("SEE UDF");
                } else if cond == bsc::C_1111 {
                    unreachable!("SEE SVC");
                } else if xpsr.in_it_block() {
                    Instruction::Unpredictable
                } else {
                    Instruction::Branch {
                        cond: Condition(cond),
                        imm32: bitstring_concat!(imm8 : bsc::C_0 | 9 bits).sign_extend(),
                    }
                }
            }
            ("1110") => {
                // A7.7.194, T1
                instr_bind!("1101|1110|<imm8:8>" identifiers (imm8,));
                Instruction::PermanentlyUndefined {
                    imm32: imm8.zero_extend(),
                }
            }
            ("1111") => {
                // A7.7.178, T1
                instr_bind!("1101|1111|<imm8:8>" identifiers (imm8,));
                Instruction::SupervisorCall {
                    imm32: imm8.zero_extend(),
                }
            }
            _ => unreachable!("All cases are covered."),
        },
        ("11100x") => {
            // A7.7.12, T2
            instr_bind!("11100|<imm11:11>" identifiers (imm11,));
            let imm32 = bitstring_concat!(imm11 : bsc::C_0 | 12 bits).sign_extend();
            if xpsr.in_it_block() && !xpsr.last_in_it_block() {
                Instruction::Unpredictable
            } else {
                Instruction::Branch {
                    cond: Condition::AL, // always = no condition
                    imm32,
                }
            }
        }
        // Extra assertion (see A5.1):
        ("11101x") | ("11110x") | ("11111x") => panic!(
            "Invalid 16-bit instruction: halfword {instr:016b} is first halfword of 32-bit instruction."
        ),
        _ => unreachable!("All cases are covered."),
    }
}

#[allow(clippy::cognitive_complexity, clippy::too_many_lines)]
// `nonminimal_bool` should be allowed only for the condition in `A7.7.77, T3`, but at the moment of writing this code,
// there was some bug in rustc which ignored the `allow` attribute. Move this `allow` there once the bug is fixed.
#[allow(clippy::nonminimal_bool)]
#[decode_instr(unpredictable = Instruction::Unpredictable)]
pub(super) fn decode_long_instruction(instr: u32, xpsr: XPSR) -> Instruction {
    // A5.3
    match instr!("111|<op1:2>|<op2:7>|xxxx|<op:1>|xxxxxxxxxxxxxxx" in order (op1, op2, op)) {
        ("01", "00xx0xx", "x") => {
            // A5.3.5
            match instr!("111|0100|<op:2>|0|<w:1>|<l:1>|<rn:4>|xxxxxxxxxxxxxxxx" in order (op, l, w, rn))
            {
                ("01", "0", "x", "xxxx") => {
                    // A7.7.159, T2
                    if instr_bind!("11101|00|010|<w:1>|0|<rn:4>|(0)|<m:1>|(0)|<register_list:13>" identifiers (w, rn, m, register_list))
                    {
                        rebind_as_u32! {rn;}
                        let registers =
                            bitstring_concat!(bsc::C_0 : m : bsc::C_0 : register_list | 16 bits);
                        let wback = w == bsc::C_1;
                        if rn == 15 || registers.bit_count() < 2 || (wback && registers.get_bit(rn))
                        {
                            Instruction::Unpredictable
                        } else {
                            Instruction::StoreMultiple {
                                rn: RegisterID::from_index(rn),
                                registers: registers.into(),
                                wback,
                            }
                        }
                    }
                }
                // note: "W:Rn" with value "not 11101" is equivalent to the two following cases ("W not 1" or "Rn not 1101")
                ("01", "1", "not 1", "xxxx") | ("01", "1", "x", "not 1101") => {
                    // A7.7.41, T2
                    if instr_bind!("11101|00|010|<w:1>|1|<rn:4>|<p:1>|<m:1>|(0)|<register_list:13>" identifiers (w, rn, p, m, register_list))
                    {
                        if w == bsc::C_1 && rn == bsc::C_1101 {
                            unreachable!("SEE POP (Thumb)");
                        }
                        rebind_as_u32! {rn;}
                        let registers =
                            bitstring_concat!(p : m : bsc::C_0 : register_list | 16 bits);
                        let wback = w == bsc::C_1;
                        if rn == 15
                            || registers.bit_count() < 2
                            || (p == bsc::C_1 && m == bsc::C_1)
                            || (registers.get_bit(15)
                                && xpsr.in_it_block()
                                && !xpsr.last_in_it_block())
                            || (wback && registers.get_bit(rn))
                        {
                            Instruction::Unpredictable
                        } else {
                            Instruction::LoadMultiple {
                                rn: RegisterID::from_index(rn),
                                registers: registers.into(),
                                wback,
                                is_narrow: false,
                            }
                        }
                    }
                }
                ("01", "1", "1", "1101") => {
                    // A7.7.99, T2

                    // NOTE: `pop.w { regs }` seems to be a shortcut notation for `ldm sp!, { regs }`.
                    //       (see: `cmemu-lib/src/component/core/instruction.rs` ctrl+f "A7.7.99")
                    //
                    //       Therefore we decode this encoding to `LoadMultiple`.
                    //       To be more consistent with docs, we leave their decoding stages
                    //       separate although after constant propagation they do the same operations.
                    //       (note that `rn == 13 && wback == true && registers.get_bit(13) == false`)
                    if instr_bind!("11101|00|010|1|1|1101|<p:1>|<m:1>|(0)|<register_list:13>" identifiers (p, m, register_list))
                    {
                        let registers =
                            bitstring_concat!(p : m : bsc::C_0 : register_list | 16 bits);
                        if registers.bit_count() < 2
                            || (p == bsc::C_1 && m == bsc::C_1)
                            || (registers.get_bit(15)
                                && xpsr.in_it_block()
                                && !xpsr.last_in_it_block())
                        {
                            Instruction::Unpredictable
                        } else {
                            Instruction::LoadMultiple {
                                rn: RegisterID::SP,
                                registers: registers.into(),
                                wback: true,
                                is_narrow: false,
                            }
                        }
                    }
                }
                // note: "W:Rn" with value "not 11101" is equivalent to the two following cases ("W not 1" or "Rn not 1101")
                ("10", "0", "not 1", "xxxx") | ("10", "0", "x", "not 1101") => {
                    // A7.7.160, T1
                    if instr_bind!("11101|00|100|<w:1>|0|<rn:4>|(0)|<m:1>|(0)|<register_list:13>" identifiers (w, rn, m, register_list))
                    {
                        if w == bsc::C_1 && rn == bsc::C_1101 {
                            unreachable!("SEE PUSH");
                        }
                        rebind_as_u32! {rn;}
                        let registers =
                            bitstring_concat!(bsc::C_0 : m : bsc::C_0 : register_list | 16 bits);
                        let wback = w == bsc::C_1;
                        if rn == 15 || registers.bit_count() < 2 || (wback && registers.get_bit(rn))
                        {
                            Instruction::Unpredictable
                        } else {
                            Instruction::StoreMultipleDecrementBefore {
                                rn: RegisterID::from_index(rn),
                                registers: registers.into(),
                                wback,
                            }
                        }
                    }
                }
                ("10", "0", "1", "1101") => {
                    // A7.7.101, T2

                    // NOTE: `push.w { regs }` seems to be a shortcut notation for `stmdb sp!, { regs }`.
                    //       (see: `cmemu-lib/src/component/core/instruction.rs` ctrl+f "A7.7.101")
                    //
                    //       Therefore we decode this encoding to `StoreMultipleDecrementBefore`.
                    //       To be more consistent with docs, we leave their decoding stages
                    //       separate although after constant propagation they do the same operations.
                    //       (note that `rn == 13 && wback == true && registers.get_bit(13) == false`)
                    if instr_bind!("11101|00|100|1|0|1101|(0)|<m:1>|(0)|<register_list:13>" identifiers (m, register_list))
                    {
                        let registers =
                            bitstring_concat!(bsc::C_0 : m : bsc::C_0 : register_list | 16 bits);
                        if registers.bit_count() < 2 {
                            Instruction::Unpredictable
                        } else {
                            Instruction::StoreMultipleDecrementBefore {
                                rn: RegisterID::SP,
                                registers: registers.into(),
                                wback: true,
                            }
                        }
                    }
                }
                ("10", "1", "x", "xxxx") => {
                    // A7.7.42, T1
                    if instr_bind!("11101|00|100|<w:1>|1|<rn:4>|<p:1>|<m:1>|(0)|<register_list:13>" identifiers (w, rn, p, m, register_list))
                    {
                        rebind_as_u32! {rn;}
                        let registers =
                            bitstring_concat!(p : m : bsc::C_0 : register_list | 16 bits);
                        let wback = w == bsc::C_1;
                        if rn == 15
                            || registers.bit_count() < 2
                            || (p == bsc::C_1 && m == bsc::C_1)
                            || (registers.get_bit(15)
                                && xpsr.in_it_block()
                                && !xpsr.last_in_it_block())
                            || (wback && registers.get_bit(rn))
                        {
                            Instruction::Unpredictable
                        } else {
                            Instruction::LoadMultipleDecrementBefore {
                                rn: RegisterID::from_index(rn),
                                registers: registers.into(),
                                wback,
                            }
                        }
                    }
                }
                _ => Instruction::Undefined,
            }
        }
        ("01", "00xx1xx", "x") => {
            // A5.3.6
            // NOTE: In [ARM-ARM] rn is extracted but not included in table, hence we ignore it here.
            match instr!("111|0100|<op1:2>|1|<op2:2>|xxxxxxxxxxxx|<op3:4>|xxxx" in order (op1, op2, op3))
            {
                ("00", "00", "xxxx") => {
                    // A7.7.167, T1
                    instr_bind!("11101|00|0|0|1|0|0|<rn:4>|<rt:4>|<rd:4>|<imm8:8>" identifiers (rn, rt, rd, imm8));
                    rebind_as_u32! {rd; rt; rn;}
                    if rd == 13
                        || rd == 15
                        || rt == 13
                        || rt == 15
                        || rn == 15
                        || rd == rn
                        || rd == rt
                    {
                        Instruction::Unpredictable
                    } else {
                        Instruction::StoreRegisterExclusive {
                            rd: RegisterID::from_index(rd),
                            rt: RegisterID::from_index(rt),
                            rn: RegisterID::from_index(rn),
                            imm32: bitstring_concat!(imm8 : bsc::C_00 | 10 bits).zero_extend(),
                        }
                    }
                }
                ("00", "01", "xxxx") => {
                    // A7.7.52, T1
                    if instr_bind!("11101|00|0|0|1|0|1|<rn:4>|<rt:4>|(1)(1)(1)(1)|<imm8:8>" identifiers (rn, rt, imm8))
                    {
                        rebind_as_u32! {rt; rn;}
                        if rt == 13 || rt == 15 || rn == 15 {
                            Instruction::Unpredictable
                        } else {
                            Instruction::LoadRegisterExclusive {
                                rt: RegisterID::from_index(rt),
                                rn: RegisterID::from_index(rn),
                                imm32: bitstring_concat!(imm8 : bsc::C_00 | 10 bits).zero_extend(),
                            }
                        }
                    }
                }
                ("0x", "10", "xxxx") | ("1x", "x0", "xxxx") => {
                    // A7.7.166, T1
                    instr_bind!("11101|00|<p:1>|<u:1>|1|<w:1>|0|<rn:4>|<rt:4>|<rt2:4>|<imm8:8>" identifiers (p, u, w, rn, rt, rt2, imm8));
                    if p == bsc::C_0 && w == bsc::C_0 {
                        unreachable!("SEE Related encodings")
                    } else {
                        rebind_as_u32! {rt; rt2; rn;};
                        let index = p == bsc::C_1;
                        let add = u == bsc::C_1;
                        let wback = w == bsc::C_1;

                        if wback && (rn == rt || rn == rt2)
                            || rn == 15
                            || rt == 13
                            || rt == 15
                            || rt2 == 13
                            || rt2 == 15
                        {
                            Instruction::Unpredictable
                        } else {
                            Instruction::StoreRegisterDual_Immediate {
                                rt: RegisterID::from_index(rt),
                                rt2: RegisterID::from_index(rt2),
                                rn: RegisterID::from_index(rn),
                                imm32: bitstring_concat!(imm8 : bsc::C_00 | 10 bits).zero_extend(),
                                index,
                                add,
                                wback,
                            }
                        }
                    }
                }
                ("0x", "11", "xxxx") | ("1x", "x1", "xxxx") => {
                    // A7.7.50, T1
                    instr_bind!("11101|00|<p:1>|<u:1>|1|<w:1>|1|<rn:4>|<rt:4>|<rt2:4>|<imm8:8>" identifiers (p, u, w, rn, rt, rt2, imm8));
                    if p == bsc::C_0 && w == bsc::C_0 {
                        unreachable!("SEE Related encodings")
                    } else if rn == bsc::C_1111 {
                        // A7.7.51, T1
                        if p == bsc::C_0 && w == bsc::C_0 {
                            unreachable!("SEE Related encodings")
                        } else {
                            rebind_as_u32! {rt; rt2;};
                            let add = u == bsc::C_1;

                            if rt == 13
                                || rt == 15
                                || rt2 == 13
                                || rt2 == 15
                                || rt == rt2
                                || w == bsc::C_1
                            {
                                Instruction::Unpredictable
                            } else {
                                Instruction::LoadRegisterDual_Literal {
                                    rt: RegisterID::from_index(rt),
                                    rt2: RegisterID::from_index(rt2),
                                    imm32: bitstring_concat!(imm8 : bsc::C_00 | 10 bits)
                                        .zero_extend(),
                                    add,
                                }
                            }
                        }
                    } else {
                        rebind_as_u32! {rt; rt2; rn;};
                        let index = p == bsc::C_1;
                        let add = u == bsc::C_1;
                        let wback = w == bsc::C_1;

                        if wback && (rn == rt || rn == rt2)
                            || rt == 13
                            || rt == 15
                            || rt2 == 13
                            || rt2 == 15
                            || rt == rt2
                        {
                            Instruction::Unpredictable
                        } else {
                            Instruction::LoadRegisterDual_Immediate {
                                rt: RegisterID::from_index(rt),
                                rt2: RegisterID::from_index(rt2),
                                rn: RegisterID::from_index(rn),
                                imm32: bitstring_concat!(imm8 : bsc::C_00 | 10 bits).zero_extend(),
                                index,
                                add,
                                wback,
                            }
                        }
                    }
                }
                ("01", "00", "0100") => {
                    // A7.7.168, T1
                    if instr_bind!("11101|000110|0|<rn:4>|<rt:4>|(1)(1)(1)(1)|0100|<rd:4>" identifiers (rn, rt, rd))
                    {
                        rebind_as_u32! {rd; rt; rn;}
                        if rd == 13
                            || rd == 15
                            || rt == 13
                            || rt == 15
                            || rn == 15
                            || rd == rn
                            || rd == rt
                        {
                            Instruction::Unpredictable
                        } else {
                            Instruction::StoreRegisterExclusiveByte {
                                rd: RegisterID::from_index(rd),
                                rt: RegisterID::from_index(rt),
                                rn: RegisterID::from_index(rn),
                            }
                        }
                    }
                }
                ("01", "00", "0101") => {
                    // A7.7.169, T1
                    if instr_bind!("11101|000110|0|<rn:4>|<rt:4>|(1)(1)(1)(1)|0101|<rd:4>" identifiers (rn, rt, rd))
                    {
                        rebind_as_u32! {rd; rt; rn;}
                        if rd == 13
                            || rd == 15
                            || rt == 13
                            || rt == 15
                            || rn == 15
                            || rd == rn
                            || rd == rt
                        {
                            Instruction::Unpredictable
                        } else {
                            Instruction::StoreRegisterExclusiveHalfword {
                                rd: RegisterID::from_index(rd),
                                rt: RegisterID::from_index(rt),
                                rn: RegisterID::from_index(rn),
                            }
                        }
                    }
                }
                ("01", "01", "0000") | ("01", "01", "0001") => {
                    // A7.7.185, T1
                    if instr_bind!("11101|00|0|1|1|0|1|<rn:4>|(1)(1)(1)(1)|(0)(0)(0)(0)|000|<h:1>|<rm:4>" identifiers (rn, h, rm))
                    {
                        rebind_as_u32! {rn; rm;}
                        if rn == 13
                            || rm == 13
                            || rm == 15
                            || (xpsr.in_it_block() && !xpsr.last_in_it_block())
                        {
                            Instruction::Unpredictable
                        } else {
                            Instruction::TableBranch {
                                rn: RegisterID::from_index(rn),
                                rm: RegisterID::from_index(rm),
                                is_tbh: h == bsc::C_1,
                            }
                        }
                    }
                }
                ("01", "01", "0100") => {
                    // A7.7.53, T1
                    if instr_bind!("11101|000110|1|<rn:4>|<rt:4>|(1)(1)(1)(1)|0100|(1)(1)(1)(1)" identifiers (rn, rt))
                    {
                        rebind_as_u32! {rt; rn;}
                        if rt == 13 || rt == 15 || rn == 15 {
                            Instruction::Unpredictable
                        } else {
                            Instruction::LoadRegisterExclusiveByte {
                                rt: RegisterID::from_index(rt),
                                rn: RegisterID::from_index(rn),
                            }
                        }
                    }
                }
                ("01", "01", "0101") => {
                    // A7.7.54, T1
                    if instr_bind!("11101|000110|1|<rn:4>|<rt:4>|(1)(1)(1)(1)|0101|(1)(1)(1)(1)" identifiers (rn, rt))
                    {
                        rebind_as_u32! {rt; rn;}
                        if rt == 13 || rt == 15 || rn == 15 {
                            Instruction::Unpredictable
                        } else {
                            Instruction::LoadRegisterExclusiveHalfword {
                                rt: RegisterID::from_index(rt),
                                rn: RegisterID::from_index(rn),
                            }
                        }
                    }
                }
                _ => Instruction::Undefined,
            }
        }
        ("01", "01xxxxx", "x") => {
            // A5.3.11
            match instr!("111|0101|<op:4>|<s:1>|<rn:4>|xxxx|<rd:4>|xxxxxxxx" in order (op, rn, rd, s))
            {
                ("0000", "xxxx", "not 1111", "x") => {
                    // A7.7.9, T2
                    if instr_bind!("11101|01|0000|<s:1>|<rn:4>|(0)|<imm3:3>|<rd:4>|<imm2:2>|<ty:2>|<rm:4>" identifiers (s, rn, imm3, rd, imm2, ty, rm))
                    {
                        if rd == bsc::C_1111 && s == bsc::C_1 {
                            unreachable!("SEE TST (register)")
                        } else {
                            rebind_as_u32! {rd; rn; rm;}
                            if rd == 13
                                || (rd == 15 && s == bsc::C_0)
                                || rn == 13
                                || rn == 15
                                || rm == 13
                                || rm == 15
                            {
                                Instruction::Unpredictable
                            } else {
                                Instruction::And_Register {
                                    rd: RegisterID::from_index(rd),
                                    rn: RegisterID::from_index(rn),
                                    rm: RegisterID::from_index(rm),
                                    shift: Shift::decode_imm_shift(
                                        ty,
                                        bitstring_concat!(imm3 : imm2 | 5 bits),
                                    ),
                                    setflags: s == bsc::C_1,
                                    setflags_depends_on_it: false,
                                }
                            }
                        }
                    }
                }
                ("0000", "xxxx", "1111", "0") => Instruction::Unpredictable,
                ("0000", "xxxx", "1111", "1") => {
                    // A7.7.189, T2
                    if instr_bind!("11101|01|0000|1|<rn:4>|(0)|<imm3:3>|1111|<imm2:2>|<ty:2>|<rm:4>" identifiers (rn, imm3, imm2, ty, rm))
                    {
                        rebind_as_u32! {rn; rm;}
                        if rn == 13 || rn == 15 || rm == 13 || rm == 15 {
                            Instruction::Unpredictable
                        } else {
                            Instruction::Test_Register {
                                rn: RegisterID::from_index(rn),
                                rm: RegisterID::from_index(rm),
                                shift: Shift::decode_imm_shift(
                                    ty,
                                    bitstring_concat!(imm3 : imm2 | 5 bits),
                                ),
                            }
                        }
                    }
                }
                ("0001", "xxxx", "xxxx", "x") => {
                    // A7.7.16, T2
                    if instr_bind!("11101|01|0001|<s:1>|<rn:4>|(0)|<imm3:3>|<rd:4>|<imm2:2>|<ty:2>|<rm:4>" identifiers (s, rn, imm3, rd, imm2, ty, rm))
                    {
                        rebind_as_u32! {rd; rn; rm;}
                        if rd == 13 || rd == 15 || rn == 13 || rn == 15 || rm == 13 || rm == 15 {
                            Instruction::Unpredictable
                        } else {
                            Instruction::BitClear_Register {
                                rd: RegisterID::from_index(rd),
                                rn: RegisterID::from_index(rn),
                                rm: RegisterID::from_index(rm),
                                shift: Shift::decode_imm_shift(
                                    ty,
                                    bitstring_concat!(imm3 : imm2 | 5 bits),
                                ),
                                setflags: s == bsc::C_1,
                                setflags_depends_on_it: false,
                            }
                        }
                    }
                }
                ("0010", "not 1111", "xxxx", "x") => {
                    // A7.7.92, T2
                    if instr_bind!("11101|01|0010|<s:1>|<rn:4>|(0)|<imm3:3>|<rd:4>|<imm2:2>|<ty:2>|<rm:4>" identifiers (s, rn, imm3, rd, imm2, ty, rm))
                    {
                        if rn == bsc::C_1111 {
                            unreachable!("SEE Related encodings")
                        } else {
                            rebind_as_u32! {rd; rn; rm;}
                            if rd == 13 || rd == 15 || rn == 13 || rm == 13 || rm == 15 {
                                Instruction::Unpredictable
                            } else {
                                Instruction::LogicalOr_Register {
                                    rd: RegisterID::from_index(rd),
                                    rn: RegisterID::from_index(rn),
                                    rm: RegisterID::from_index(rm),
                                    shift: Shift::decode_imm_shift(
                                        ty,
                                        bitstring_concat!(imm3 : imm2 | 5 bits),
                                    ),
                                    setflags: s == bsc::C_1,
                                    setflags_depends_on_it: false,
                                }
                            }
                        }
                    }
                }
                ("0010", "1111", "xxxx", "x") => {
                    // A5.3.11, Move register and immediate shifts
                    match instr!("111|0101|0010|x|1111|x|<imm3:3>|xxxx|<imm2:2>|<ty:2>|xxxx" in order (ty, imm3, imm2))
                    {
                        ("00", "000", "00") => {
                            // A7.7.77, T3
                            if instr_bind!("11101|01|0010|<s:1>|1111|(0)|000|<rd:4>|0000|<rm:4>" identifiers (s, rd, rm))
                            {
                                rebind_as_u32! {rd; rm;}
                                let setflags = s == bsc::C_1;
                                if (setflags && (rd == 13 || rd == 15 || rm == 13 || rm == 15))
                                    || (!setflags
                                        && (rd == 15 || rm == 15 || (rd == 13 && rm == 13)))
                                {
                                    Instruction::Unpredictable
                                } else {
                                    Instruction::Move_Register {
                                        rd: RegisterID::from_index(rd),
                                        rm: RegisterID::from_index(rm),
                                        setflags,
                                    }
                                }
                            }
                        }
                        // note: "imm3:imm2" with value "not 00000" is equivalent to the three following cases
                        ("00", "000", "not 00")
                        | ("00", "not 000", "00")
                        | ("00", "not 000", "not 00") => {
                            // A7.7.68, T2
                            if instr_bind!("11101|01|0010|<s:1>|1111|(0)|<imm3:3>|<rd:4>|<imm2:2>|00|<rm:4>" identifiers (s, imm3, rd, imm2, rm))
                            {
                                rebind_as_u32! {rd; rm;}
                                let imm5 = bitstring_concat!(imm3 : imm2 | 5 bits);
                                if imm5 == bsc::C_0_0000 {
                                    unreachable!("SEE MOV (register)")
                                } else if rd == 13 || rd == 15 || rm == 13 || rm == 15 {
                                    Instruction::Unpredictable
                                } else {
                                    Instruction::LogicalShiftLeft_Immediate {
                                        rd: RegisterID::from_index(rd),
                                        rm: RegisterID::from_index(rm),
                                        shift: Shift::decode_imm_shift(bsc::C_00, imm5),
                                        setflags: s == bsc::C_1,
                                        setflags_depends_on_it: false,
                                    }
                                }
                            }
                        }
                        ("01", "xxx", "xx") => {
                            // A7.7.70, T2
                            if instr_bind!("11101|01|0010|<s:1>|1111|(0)|<imm3:3>|<rd:4>|<imm2:2>|01|<rm:4>" identifiers (s, imm3, rd, imm2, rm))
                            {
                                rebind_as_u32! {rd; rm;}
                                if rd == 13 || rd == 15 || rm == 13 || rm == 15 {
                                    Instruction::Unpredictable
                                } else {
                                    Instruction::LogicalShiftRight_Immediate {
                                        rd: RegisterID::from_index(rd),
                                        rm: RegisterID::from_index(rm),
                                        shift: Shift::decode_imm_shift(
                                            bsc::C_01,
                                            bitstring_concat!(imm3 : imm2 | 5 bits),
                                        ),
                                        setflags: s == bsc::C_1,
                                        setflags_depends_on_it: false,
                                    }
                                }
                            }
                        }
                        ("10", "xxx", "xx") => {
                            // A7.7.10, T2
                            if instr_bind!("11101|01|0010|<s:1>|1111|(0)|<imm3:3>|<rd:4>|<imm2:2>|10|<rm:4>" identifiers (s, imm3, rd, imm2, rm))
                            {
                                rebind_as_u32! {rd; rm;}
                                if rd == 13 || rd == 15 || rm == 13 || rm == 15 {
                                    Instruction::Unpredictable
                                } else {
                                    Instruction::ArithmeticShiftRight_Immediate {
                                        rd: RegisterID::from_index(rd),
                                        rm: RegisterID::from_index(rm),
                                        shift: Shift::decode_imm_shift(
                                            bsc::C_10,
                                            bitstring_concat!(imm3 : imm2 | 5 bits),
                                        ),
                                        setflags: s == bsc::C_1,
                                        setflags_depends_on_it: false,
                                    }
                                }
                            }
                        }
                        ("11", "000", "00") => {
                            // A7.7.118, T1
                            if instr_bind!("11101|01|0010|<s:1>|1111|(0)|000|<rd:4>|00|11|<rm:4>" identifiers (s, rd, rm))
                            {
                                rebind_as_u32! {rd; rm;}
                                if rd == 13 || rd == 15 || rm == 13 || rm == 15 {
                                    Instruction::Unpredictable
                                } else {
                                    Instruction::RotateRightWithExtend {
                                        rd: RegisterID::from_index(rd),
                                        rm: RegisterID::from_index(rm),
                                        setflags: s == bsc::C_1,
                                    }
                                }
                            }
                        }
                        // note: "imm3:imm2" with value "not 00000" is equivalent to the three following cases
                        ("11", "000", "not 00")
                        | ("11", "not 000", "00")
                        | ("11", "not 000", "not 00") => {
                            // A7.7.116, T1
                            if instr_bind!("11101|01|0010|<s:1>|1111|(0)|<imm3:3>|<rd:4>|<imm2:2>|11|<rm:4>" identifiers (s, imm3, rd, imm2, rm))
                            {
                                if bitstring_concat!(imm3 : imm2 | 5 bits) == bsc::C_0_0000 {
                                    unreachable!("SEE RRX")
                                } else {
                                    rebind_as_u32! {rd; rm;}
                                    if rd == 13 || rd == 15 || rm == 13 || rm == 15 {
                                        Instruction::Unpredictable
                                    } else {
                                        Instruction::RotateRight_Immediate {
                                            rd: RegisterID::from_index(rd),
                                            rm: RegisterID::from_index(rm),
                                            shift: Shift::decode_imm_shift(
                                                bsc::C_11,
                                                bitstring_concat!(imm3 : imm2 | 5 bits),
                                            ),
                                            setflags: s == bsc::C_1,
                                        }
                                    }
                                }
                            }
                        }
                        _ => unreachable!("All cases are covered."),
                    }
                }
                ("0011", "not 1111", "xxxx", "x") => {
                    // A7.7.90, T1
                    if instr_bind!("11101|01|0011|<s:1>|<rn:4>|(0)|<imm3:3>|<rd:4>|<imm2:2>|<ty:2>|<rm:4>" identifiers (s, rn, imm3, rd, imm2, ty, rm))
                    {
                        if rn == bsc::C_1111 {
                            unreachable!("SEE MVN (register)")
                        } else {
                            rebind_as_u32! {rd; rn; rm;}
                            if rd == 13 || rd == 15 || rn == 13 || rm == 13 || rm == 15 {
                                Instruction::Unpredictable
                            } else {
                                Instruction::LogicalOrNot_Register {
                                    rd: RegisterID::from_index(rd),
                                    rn: RegisterID::from_index(rn),
                                    rm: RegisterID::from_index(rm),
                                    shift: Shift::decode_imm_shift(
                                        ty,
                                        bitstring_concat!(imm3 : imm2 | 5 bits),
                                    ),
                                    setflags: s == bsc::C_1,
                                }
                            }
                        }
                    }
                }
                ("0011", "1111", "xxxx", "x") => {
                    // A7.7.86, T2
                    if instr_bind!("11101|01|0011|<s:1>|1111|(0)|<imm3:3>|<rd:4>|<imm2:2>|<ty:2>|<rm:4>" identifiers (s, imm3, rd, imm2, ty, rm))
                    {
                        rebind_as_u32! {rd; rm;}
                        if rd == 13 || rd == 15 || rm == 13 || rm == 15 {
                            Instruction::Unpredictable
                        } else {
                            Instruction::BitwiseNot_Register {
                                rd: RegisterID::from_index(rd),
                                rm: RegisterID::from_index(rm),
                                shift: Shift::decode_imm_shift(
                                    ty,
                                    bitstring_concat!(imm3 : imm2 | 5 bits),
                                ),
                                setflags: s == bsc::C_1,
                                setflags_depends_on_it: false,
                            }
                        }
                    }
                }
                ("0100", "xxxx", "not 1111", "x") => {
                    // A7.7.36, T2
                    if instr_bind!("11101|01|0100|<s:1>|<rn:4>|(0)|<imm3:3>|<rd:4>|<imm2:2>|<ty:2>|<rm:4>" identifiers (s, rn, imm3, rd, imm2, ty, rm))
                    {
                        if rd == bsc::C_1111 && s == bsc::C_1 {
                            unreachable!("SEE TEQ (register)")
                        } else {
                            rebind_as_u32! {rd; rn; rm;}
                            if rd == 13
                                || (rd == 15 && s == bsc::C_0)
                                || rn == 13
                                || rn == 15
                                || rm == 13
                                || rm == 15
                            {
                                Instruction::Unpredictable
                            } else {
                                Instruction::ExclusiveOr_Register {
                                    rd: RegisterID::from_index(rd),
                                    rn: RegisterID::from_index(rn),
                                    rm: RegisterID::from_index(rm),
                                    shift: Shift::decode_imm_shift(
                                        ty,
                                        bitstring_concat!(imm3 : imm2 | 5 bits),
                                    ),
                                    setflags: s == bsc::C_1,
                                    setflags_depends_on_it: false,
                                }
                            }
                        }
                    }
                }
                ("0100", "xxxx", "1111", "0") => Instruction::Unpredictable,
                ("0100", "xxxx", "1111", "1") => {
                    // A7.7.187, T1
                    if instr_bind!("11101|01|0100|1|<rn:4>|(0)|<imm3:3>|1111|<imm2:2>|<ty:2>|<rm:4>" identifiers (rn, imm3, imm2, ty, rm))
                    {
                        rebind_as_u32! {rn; rm;}
                        if rn == 13 || rn == 15 || rm == 13 || rm == 15 {
                            Instruction::Unpredictable
                        } else {
                            Instruction::TestEquivalence_Register {
                                rn: RegisterID::from_index(rn),
                                rm: RegisterID::from_index(rm),
                                shift: Shift::decode_imm_shift(
                                    ty,
                                    bitstring_concat!(imm3 : imm2 | 5 bits),
                                ),
                            }
                        }
                    }
                }
                // PKHBT, PKHTB; CherryMote doesn't include DSP extension (v7E-M; [ARM-ARM] A1.3), so this is UNDEFINED ([ARM-ARM] A5.1.1).
                ("0110", "xxxx", "xxxx", "x") => Instruction::Undefined,
                ("1000", "xxxx", "not 1111", "x") => {
                    // A7.7.4, T3
                    if instr_bind!("11101|01|1000|<s:1>|<rn:4>|(0)|<imm3:3>|<rd:4>|<imm2:2>|<ty:2>|<rm:4>" identifiers (s, rn, imm3, rd, imm2, ty, rm))
                    {
                        if rd == bsc::C_1111 && s == bsc::C_1 {
                            unreachable!("SEE CMN (register)")
                        } else if rn == bsc::C_1101 {
                            // SEE ADD (SP plus register)
                            // A7.7.6, T3
                            if rd == bsc::C_1111 && s == bsc::C_1 {
                                unreachable!("SEE CMN (register)")
                            } else {
                                rebind_as_u32! {rd; rm;}
                                let shift = Shift::decode_imm_shift(
                                    ty,
                                    bitstring_concat!(imm3 : imm2 | 5 bits),
                                );

                                if (rd == 13 && (shift.srtype != SRType::LSL || shift.amount > 3))
                                    || (rd == 15 && s == bsc::C_0)
                                    || rm == 13
                                    || rm == 15
                                {
                                    Instruction::Unpredictable
                                } else {
                                    Instruction::Add_SPPlusRegister {
                                        rd: RegisterID::from_index(rd),
                                        rm: RegisterID::from_index(rm),
                                        shift,
                                        setflags: s == bsc::C_1,
                                    }
                                }
                            }
                        } else {
                            rebind_as_u32! {rd; rn; rm;}
                            if rd == 13
                                || (rd == 15 && s == bsc::C_0)
                                || rn == 15
                                || rm == 13
                                || rm == 15
                            {
                                Instruction::Unpredictable
                            } else {
                                Instruction::Add_Register {
                                    rd: RegisterID::from_index(rd),
                                    rn: RegisterID::from_index(rn),
                                    rm: RegisterID::from_index(rm),
                                    shift: Shift::decode_imm_shift(
                                        ty,
                                        bitstring_concat!(imm3 : imm2 | 5 bits),
                                    ),
                                    setflags: s == bsc::C_1,
                                    setflags_depends_on_it: false,
                                }
                            }
                        }
                    }
                }
                ("1000", "xxxx", "1111", "0") => Instruction::Unpredictable,
                ("1000", "xxxx", "1111", "1") => {
                    // A7.7.26, T2
                    if instr_bind!("11101|01|1000|1|<rn:4>|(0)|<imm3:3>|1111|<imm2:2>|<ty:2>|<rm:4>" identifiers (rn, imm3, imm2, ty, rm))
                    {
                        rebind_as_u32! {rn; rm;}
                        if rn == 15 || rm == 13 || rm == 15 {
                            Instruction::Unpredictable
                        } else {
                            Instruction::CompareNegative_Register {
                                rn: RegisterID::from_index(rn),
                                rm: RegisterID::from_index(rm),
                                shift: Shift::decode_imm_shift(
                                    ty,
                                    bitstring_concat!(imm3 : imm2 | 5 bits),
                                ),
                            }
                        }
                    }
                }
                ("1010", "xxxx", "xxxx", "x") => {
                    // A7.7.2, T2
                    if instr_bind!("11101|01|1010|<s:1>|<rn:4>|(0)|<imm3:3>|<rd:4>|<imm2:2>|<ty:2>|<rm:4>" identifiers (s, rn, imm3, rd, imm2, ty, rm))
                    {
                        rebind_as_u32! {rd; rn; rm;}
                        if rd == 13 || rd == 15 || rn == 13 || rn == 15 || rm == 13 || rm == 15 {
                            Instruction::Unpredictable
                        } else {
                            Instruction::AddWithCarry_Register {
                                rd: RegisterID::from_index(rd),
                                rn: RegisterID::from_index(rn),
                                rm: RegisterID::from_index(rm),
                                shift: Shift::decode_imm_shift(
                                    ty,
                                    bitstring_concat!(imm3 : imm2 | 5 bits),
                                ),
                                setflags: s == bsc::C_1,
                                setflags_depends_on_it: false,
                            }
                        }
                    }
                }
                ("1011", "xxxx", "xxxx", "x") => {
                    // A7.7.125, T2
                    if instr_bind!("11101|01|1011|<s:1>|<rn:4>|(0)|<imm3:3>|<rd:4>|<imm2:2>|<ty:2>|<rm:4>" identifiers (s, rn, imm3, rd, imm2, ty, rm))
                    {
                        rebind_as_u32! {rd; rn; rm;}
                        if rd == 13 || rd == 15 || rn == 13 || rn == 15 || rm == 13 || rm == 15 {
                            Instruction::Unpredictable
                        } else {
                            Instruction::SubtractWithCarry_Register {
                                rd: RegisterID::from_index(rd),
                                rn: RegisterID::from_index(rn),
                                rm: RegisterID::from_index(rm),
                                shift: Shift::decode_imm_shift(
                                    ty,
                                    bitstring_concat!(imm3 : imm2 | 5 bits),
                                ),
                                setflags: s == bsc::C_1,
                                setflags_depends_on_it: false,
                            }
                        }
                    }
                }
                ("1101", "xxxx", "not 1111", "x") => {
                    // A7.7.175, T2
                    if instr_bind!("11101|01|1101|<s:1>|<rn:4>|(0)|<imm3:3>|<rd:4>|<imm2:2>|<ty:2>|<rm:4>" identifiers (s, rn, imm3, rd, imm2, ty, rm))
                    {
                        if rd == bsc::C_1111 && s == bsc::C_1 {
                            unreachable!("SEE CMN (register)")
                        } else if rn == bsc::C_1101 {
                            // A7.7.177, T1
                            if rd == bsc::C_1111 && s == bsc::C_1 {
                                unreachable!("SEE CMN (register)")
                            } else {
                                rebind_as_u32! {rd; rm;}
                                let shift = Shift::decode_imm_shift(
                                    ty,
                                    bitstring_concat!(imm3 : imm2 | 5 bits),
                                );

                                if (rd == 13 && (shift.srtype != SRType::LSL || shift.amount > 3))
                                    || (rd == 15 && s == bsc::C_0)
                                    || rm == 13
                                    || rm == 15
                                {
                                    Instruction::Unpredictable
                                } else {
                                    Instruction::Subtract_SPMinusRegister {
                                        rd: RegisterID::from_index(rd),
                                        rm: RegisterID::from_index(rm),
                                        shift,
                                        setflags: s == bsc::C_1,
                                    }
                                }
                            }
                        } else {
                            rebind_as_u32! {rd; rn; rm;}
                            if rd == 13
                                || (rd == 15 && s == bsc::C_0)
                                || rn == 15
                                || rm == 13
                                || rm == 15
                            {
                                Instruction::Unpredictable
                            } else {
                                Instruction::Subtract_Register {
                                    rd: RegisterID::from_index(rd),
                                    rn: RegisterID::from_index(rn),
                                    rm: RegisterID::from_index(rm),
                                    shift: Shift::decode_imm_shift(
                                        ty,
                                        bitstring_concat!(imm3 : imm2 | 5 bits),
                                    ),
                                    setflags: s == bsc::C_1,
                                    setflags_depends_on_it: false,
                                }
                            }
                        }
                    }
                }
                ("1101", "xxxx", "1111", "0") => Instruction::Unpredictable,
                ("1101", "xxxx", "1111", "1") => {
                    // A7.7.28, T3
                    if instr_bind!("11101|01|1101|1|<rn:4>|(0)|<imm3:3>|1111|<imm2:2>|<ty:2>|<rm:4>" identifiers (rn, imm3, imm2, ty, rm))
                    {
                        rebind_as_u32! {rn; rm;}
                        if rn == 15 || rm == 13 || rm == 15 {
                            Instruction::Unpredictable
                        } else {
                            Instruction::Compare_Register {
                                rn: RegisterID::from_index(rn),
                                rm: RegisterID::from_index(rm),
                                shift: Shift::decode_imm_shift(
                                    ty,
                                    bitstring_concat!(imm3 : imm2 | 5 bits),
                                ),
                            }
                        }
                    }
                }
                ("1110", "xxxx", "xxxx", "x") => {
                    // A7.7.120, T1
                    if instr_bind!("11101|01|1110|<s:1>|<rn:4>|(0)|<imm3:3>|<rd:4>|<imm2:2>|<ty:2>|<rm:4>" identifiers (s, rn, imm3, rd, imm2, ty, rm))
                    {
                        rebind_as_u32! {rd; rn; rm;}
                        if rd == 13 || rd == 15 || rn == 13 || rn == 15 || rm == 13 || rm == 15 {
                            Instruction::Unpredictable
                        } else {
                            Instruction::ReverseSubtract_Register {
                                rd: RegisterID::from_index(rd),
                                rn: RegisterID::from_index(rn),
                                rm: RegisterID::from_index(rm),
                                shift: Shift::decode_imm_shift(
                                    ty,
                                    bitstring_concat!(imm3 : imm2 | 5 bits),
                                ),
                                setflags: s == bsc::C_1,
                            }
                        }
                    }
                }
                _ => Instruction::Undefined,
            }
        }
        ("01", "1xxxxxx", "x") | ("11", "1xxxxxx", "x") => {
            // A5.3.18
            // Note: it is a two-level match (see footnote "a." under the table in the docs).
            match instr!("111|x|11|<op1:6>|xxxx|xxxx|<coproc:4>|xxx|<op:1>|xxxx" in order (op1, op, coproc))
            {
                ("0xxxx0", "x", "xxxx") => {
                    // See parent match.
                    match instr!("111|x|11|<op1:6>|xxxx|xxxx|<coproc:4>|xxx|<op:1>|xxxx" in order (op1, op, coproc))
                    {
                        ("not 000x0x", "x", "xxxx") | ("000100", "x", "xxxx") => {
                            Instruction::Unsupported {
                                name: "Coprocessor instruction. Should cause Usage Fault with UFSR.NOCP bit set to 1. ([ARM-ARM] A5.3.18, [ARM-TDG] 4.2.1)",
                            }
                        }
                        _ => Instruction::Undefined,
                    }
                }
                ("0xxxx1", "x", "xxxx") => {
                    // See parent match.
                    match instr!("111|x|11|<op1:6>|xxxx|xxxx|<coproc:4>|xxx|<op:1>|xxxx" in order (op1, op, coproc))
                    {
                        ("not 000x0x", "x", "xxxx") | ("000101", "x", "xxxx") => {
                            Instruction::Unsupported {
                                name: "Coprocessor instruction. Should cause Usage Fault with UFSR.NOCP bit set to 1. ([ARM-ARM] A5.3.18, [ARM-TDG] 4.2.1)",
                            }
                        }
                        _ => Instruction::Undefined,
                    }
                }
                ("10xxxx", "0", "xxxx") | ("10xxx0", "1", "xxxx") | ("10xxx1", "1", "xxxx") => {
                    Instruction::Unsupported {
                        name: "Coprocessor instruction. Should cause Usage Fault with UFSR.NOCP bit set to 1. ([ARM-ARM] A5.3.18, [ARM-TDG] 4.2.1)",
                    }
                }
                _ => Instruction::Undefined,
            }
        }
        ("10", "x0xxxxx", "0") => {
            // A5.3.1
            match instr!("111|10|x|0|<op:5>|<rn:4>|0|xxx|<rd:4>|xxxxxxxx" in order (op, rn, rd)) {
                ("0000x", "xxxx", "not 1111") => {
                    // A7.7.8, T1
                    instr_bind!("11110|<i:1>|0|0000|<s:1>|<rn:4>|0|<imm3:3>|<rd:4>|<imm8:8>" identifiers (i, s, rn, imm3, rd, imm8));
                    if rd == bsc::C_1111 && s == bsc::C_1 {
                        unreachable!("SEE TST (immediate)")
                    } else {
                        rebind_as_u32! {rd; rn;}
                        let (imm32, carry) = match thumb_expand_imm_c(
                            bitstring_concat!(i : imm3 : imm8 | 12 bits),
                            xpsr.carry_flag(),
                        ) {
                            Ok(v) => v,
                            Err(insrt) => return insrt,
                        };
                        if rd == 13 || (rd == 15 && s == bsc::C_0) || rn == 13 || rn == 15 {
                            Instruction::Unpredictable
                        } else {
                            Instruction::And_Immediate {
                                rd: RegisterID::from_index(rd),
                                rn: RegisterID::from_index(rn),
                                imm32,
                                setflags: s == bsc::C_1,
                                carry,
                            }
                        }
                    }
                }
                ("0000x", "xxxx", "1111") => {
                    // A7.7.188, T1
                    instr_bind!("11110|<i:1>|0|0000|1|<rn:4>|0|<imm3:3>|1111|<imm8:8>" identifiers (i, rn, imm3, imm8));
                    rebind_as_u32! {rn;}
                    let (imm32, carry) = match thumb_expand_imm_c(
                        bitstring_concat!(i : imm3 : imm8 | 12 bits),
                        xpsr.carry_flag(),
                    ) {
                        Ok(v) => v,
                        Err(instr) => return instr,
                    };

                    if rn == 13 || rn == 15 {
                        Instruction::Unpredictable
                    } else {
                        Instruction::Test_Immediate {
                            rn: RegisterID::from_index(rn),
                            imm32,
                            carry,
                        }
                    }
                }
                ("0001x", "xxxx", "xxxx") => {
                    // A7.7.15, T1
                    instr_bind!("11110|<i:1>|0|0001|<s:1>|<rn:4>|0|<imm3:3>|<rd:4>|<imm8:8>" identifiers (i, s, rn, imm3, rd, imm8));
                    rebind_as_u32! {rd; rn;}
                    let (imm32, carry) = match thumb_expand_imm_c(
                        bitstring_concat!(i : imm3 : imm8 | 12 bits),
                        xpsr.carry_flag(),
                    ) {
                        Ok(v) => v,
                        Err(instr) => return instr,
                    };

                    if rd == 13 || rd == 15 || rn == 13 || rn == 15 {
                        Instruction::Unpredictable
                    } else {
                        Instruction::BitClear_Immediate {
                            rd: RegisterID::from_index(rd),
                            rn: RegisterID::from_index(rn),
                            imm32,
                            setflags: s == bsc::C_1,
                            carry,
                        }
                    }
                }
                ("0010x", "not 1111", "xxxx") => {
                    // A7.7.91, T1
                    instr_bind!("11110|<i:1>|0|0010|<s:1>|<rn:4>|0|<imm3:3>|<rd:4>|<imm8:8>" identifiers (i, s, rn, imm3, rd, imm8));
                    if rn == bsc::C_1111 {
                        unreachable!("SEE MOV (immediate)")
                    } else {
                        rebind_as_u32! {rd; rn;}
                        let (imm32, carry) = match thumb_expand_imm_c(
                            bitstring_concat!(i : imm3 : imm8 | 12 bits),
                            xpsr.carry_flag(),
                        ) {
                            Ok(v) => v,
                            Err(instr) => return instr,
                        };

                        if rd == 13 || rd == 15 || rn == 13 {
                            Instruction::Unpredictable
                        } else {
                            Instruction::LogicalOr_Immediate {
                                rd: RegisterID::from_index(rd),
                                rn: RegisterID::from_index(rn),
                                imm32,
                                setflags: s == bsc::C_1,
                                carry,
                            }
                        }
                    }
                }
                ("0010x", "1111", "xxxx") => {
                    // A7.7.76, T2
                    instr_bind!("11110|<i:1>|0|0010|<s:1>|1111|0|<imm3:3>|<rd:4>|<imm8:8>" identifiers (i, s, imm3, rd, imm8));
                    rebind_as_u32! {rd;}
                    let (imm32, carry) = match thumb_expand_imm_c(
                        bitstring_concat!(i : imm3 : imm8 | 12 bits),
                        xpsr.carry_flag(),
                    ) {
                        Ok(v) => v,
                        Err(instr) => return instr,
                    };

                    if rd == 13 || rd == 15 {
                        Instruction::Unpredictable
                    } else {
                        Instruction::Move_Immediate {
                            rd: RegisterID::from_index(rd),
                            imm32,
                            setflags: s == bsc::C_1,
                            setflags_depends_on_it: false,
                            carry,
                        }
                    }
                }
                ("0011x", "not 1111", "xxxx") => {
                    // A7.7.89, T1
                    instr_bind!("11110|<i:1>|0|0011|<s:1>|<rn:4>|0|<imm3:3>|<rd:4>|<imm8:8>" identifiers (i, s, rn, imm3, rd, imm8));
                    if rd == bsc::C_1111 {
                        unreachable!("SEE MVN (immediate)")
                    } else {
                        rebind_as_u32! {rd; rn;}
                        let (imm32, carry) = match thumb_expand_imm_c(
                            bitstring_concat!(i : imm3 : imm8 | 12 bits),
                            xpsr.carry_flag(),
                        ) {
                            Ok(v) => v,
                            Err(instr) => return instr,
                        };

                        if rd == 13 || rd == 15 || rn == 13 {
                            Instruction::Unpredictable
                        } else {
                            Instruction::LogicalOrNot_Immediate {
                                rd: RegisterID::from_index(rd),
                                rn: RegisterID::from_index(rn),
                                imm32,
                                setflags: s == bsc::C_1,
                                carry,
                            }
                        }
                    }
                }
                ("0011x", "1111", "xxxx") => {
                    // A7.7.85, T1
                    instr_bind!("11110|<i:1>|0|0011|<s:1>|1111|0|<imm3:3>|<rd:4>|<imm8:8>" identifiers (i, s, imm3, rd, imm8));
                    rebind_as_u32! {rd;}
                    let (imm32, carry) = match thumb_expand_imm_c(
                        bitstring_concat!(i : imm3 : imm8 | 12 bits),
                        xpsr.carry_flag(),
                    ) {
                        Ok(v) => v,
                        Err(instr) => return instr,
                    };

                    if rd == 13 || rd == 15 {
                        Instruction::Unpredictable
                    } else {
                        Instruction::BitwiseNot_Immediate {
                            rd: RegisterID::from_index(rd),
                            imm32,
                            setflags: s == bsc::C_1,
                            carry,
                        }
                    }
                }
                ("0100x", "xxxx", "not 1111") => {
                    // A7.7.35, T1
                    instr_bind!("11110|<i:1>|0|0100|<s:1>|<rn:4>|0|<imm3:3>|<rd:4>|<imm8:8>" identifiers (i, s, rn, imm3, rd, imm8));
                    if rd == bsc::C_1111 && s == bsc::C_1 {
                        unreachable!("SEE TEQ (immediate)")
                    } else {
                        rebind_as_u32! {rd; rn;}
                        let (imm32, carry) = match thumb_expand_imm_c(
                            bitstring_concat!(i : imm3 : imm8 | 12 bits),
                            xpsr.carry_flag(),
                        ) {
                            Ok(v) => v,
                            Err(instr) => return instr,
                        };
                        if rd == 13 || (rd == 15 && s == bsc::C_0) || rn == 13 || rn == 15 {
                            Instruction::Unpredictable
                        } else {
                            Instruction::ExclusiveOr_Immediate {
                                rd: RegisterID::from_index(rd),
                                rn: RegisterID::from_index(rn),
                                imm32,
                                setflags: s == bsc::C_1,
                                carry,
                            }
                        }
                    }
                }
                ("0100x", "xxxx", "1111") => {
                    // A7.7.186, T1
                    instr_bind!("11110|<i:1>|0|0100|1|<rn:4>|0|<imm3:3>|1111|<imm8:8>" identifiers (i, rn, imm3, imm8));
                    rebind_as_u32! {rn;}
                    let (imm32, carry) = match thumb_expand_imm_c(
                        bitstring_concat!(i : imm3 : imm8 | 12 bits),
                        xpsr.carry_flag(),
                    ) {
                        Ok(v) => v,
                        Err(instr) => return instr,
                    };
                    if rn == 13 || rn == 15 {
                        Instruction::Unpredictable
                    } else {
                        Instruction::TestEquivalence_Immediate {
                            rn: RegisterID::from_index(rn),
                            imm32,
                            carry,
                        }
                    }
                }
                ("1000x", "xxxx", "not 1111") => {
                    // A7.7.3, T3
                    instr_bind!("11110|<i:1>|0|1000|<s:1>|<rn:4>|0|<imm3:3>|<rd:4>|<imm8:8>" identifiers (i, s, rn, imm3, rd, imm8));
                    if rd == bsc::C_1111 && s == bsc::C_1 {
                        unreachable!("SEE CMN (immediate)")
                    } else if rn == bsc::C_1101 {
                        // SEE ADD (SP plus immediate)
                        // A7.7.5, T3
                        if rd == bsc::C_1111 && s == bsc::C_1 {
                            unreachable!("SEE CMN (immediate)")
                        } else {
                            rebind_as_u32! {rd;}
                            let imm32 = match thumb_expand_imm(
                                bitstring_concat!(i : imm3 : imm8 | 12 bits),
                                xpsr,
                            ) {
                                Ok(v) => v,
                                Err(instr) => return instr,
                            };

                            if rd == 15 && s == bsc::C_0 {
                                Instruction::Unpredictable
                            } else {
                                Instruction::Add_SPPlusImmediate {
                                    rd: RegisterID::from_index(rd),
                                    imm32,
                                    setflags: s == bsc::C_1,
                                }
                            }
                        }
                    } else {
                        rebind_as_u32! {rd; rn;}
                        let imm32 = match thumb_expand_imm(
                            bitstring_concat!(i : imm3 : imm8 | 12 bits),
                            xpsr,
                        ) {
                            Ok(v) => v,
                            Err(instr) => return instr,
                        };

                        if rd == 13 || (rd == 15 && s == bsc::C_0) || rn == 15 {
                            Instruction::Unpredictable
                        } else {
                            Instruction::Add_Immediate {
                                rd: RegisterID::from_index(rd),
                                rn: RegisterID::from_index(rn),
                                imm32,
                                setflags: s == bsc::C_1,
                                setflags_depends_on_it: false,
                            }
                        }
                    }
                }
                ("1000x", "xxxx", "1111") => {
                    // A7.7.25, T1
                    instr_bind!("11110|<i:1>|0|1000|1|<rn:4>|0|<imm3:3>|1111|<imm8:8>" identifiers (i, rn, imm3, imm8));
                    rebind_as_u32! {rn;}
                    let imm32 = match thumb_expand_imm(
                        bitstring_concat!(i : imm3 : imm8 | 12 bits),
                        xpsr,
                    ) {
                        Ok(v) => v,
                        Err(instr) => return instr,
                    };

                    if rn == 15 {
                        Instruction::Unpredictable
                    } else {
                        Instruction::CompareNegative_Immediate {
                            rn: RegisterID::from_index(rn),
                            imm32,
                        }
                    }
                }
                ("1010x", "xxxx", "xxxx") => {
                    // A7.7.1, T1
                    instr_bind!("11110|<i:1>|0|1010|<s:1>|<rn:4>|0|<imm3:3>|<rd:4>|<imm8:8>" identifiers (i, s, rn, imm3, rd, imm8));
                    rebind_as_u32! {rd; rn;}
                    let imm32 = match thumb_expand_imm(
                        bitstring_concat!(i : imm3 : imm8 | 12 bits),
                        xpsr,
                    ) {
                        Ok(v) => v,
                        Err(instr) => return instr,
                    };

                    if rd == 13 || rd == 15 || rn == 13 || rn == 15 {
                        Instruction::Unpredictable
                    } else {
                        Instruction::AddWithCarry_Immediate {
                            rd: RegisterID::from_index(rd),
                            rn: RegisterID::from_index(rn),
                            imm32,
                            setflags: s == bsc::C_1,
                        }
                    }
                }
                ("1011x", "xxxx", "xxxx") => {
                    // A7.7.124, T1
                    instr_bind!("11110|<i:1>|0|1011|<s:1>|<rn:4>|0|<imm3:3>|<rd:4>|<imm8:8>" identifiers (i, s, rn, imm3, rd, imm8));
                    rebind_as_u32! {rd; rn;}
                    let imm32 = match thumb_expand_imm(
                        bitstring_concat!(i : imm3 : imm8 | 12 bits),
                        xpsr,
                    ) {
                        Ok(v) => v,
                        Err(instr) => return instr,
                    };

                    if rd == 13 || rd == 15 || rn == 13 || rn == 15 {
                        Instruction::Unpredictable
                    } else {
                        Instruction::SubtractWithCarry_Immediate {
                            rd: RegisterID::from_index(rd),
                            rn: RegisterID::from_index(rn),
                            imm32,
                            setflags: s == bsc::C_1,
                        }
                    }
                }
                ("1101x", "xxxx", "not 1111") => {
                    // A7.7.174, T3
                    instr_bind!("11110|<i:1>|0|1101|<s:1>|<rn:4>|0|<imm3:3>|<rd:4>|<imm8:8>" identifiers (i, s, rn, imm3, rd, imm8));
                    if rd == bsc::C_1111 && s == bsc::C_1 {
                        unreachable!("SEE CMP (immediate)")
                    } else if rn == bsc::C_1101 {
                        // A7.7.176, T2
                        if rd == bsc::C_1111 && s == bsc::C_1 {
                            unreachable!("SEE CMP (immediate)")
                        } else {
                            rebind_as_u32! {rd;}
                            let imm32 = match thumb_expand_imm(
                                bitstring_concat!(i : imm3 : imm8 | 12 bits),
                                xpsr,
                            ) {
                                Ok(v) => v,
                                Err(instr) => return instr,
                            };

                            if rd == 15 && s == bsc::C_0 {
                                Instruction::Unpredictable
                            } else {
                                Instruction::Subtract_SPMinusImmediate {
                                    rd: RegisterID::from_index(rd),
                                    imm32,
                                    setflags: s == bsc::C_1,
                                }
                            }
                        }
                    } else {
                        rebind_as_u32! {rd; rn;}
                        let imm32 = match thumb_expand_imm(
                            bitstring_concat!(i : imm3 : imm8 | 12 bits),
                            xpsr,
                        ) {
                            Ok(v) => v,
                            Err(instr) => return instr,
                        };

                        if rd == 13 || (rd == 15 && s == bsc::C_0) || rn == 15 {
                            Instruction::Unpredictable
                        } else {
                            Instruction::Subtract_Immediate {
                                rd: RegisterID::from_index(rd),
                                rn: RegisterID::from_index(rn),
                                imm32,
                                setflags: s == bsc::C_1,
                                setflags_depends_on_it: false,
                            }
                        }
                    }
                }
                ("1101x", "xxxx", "1111") => {
                    // A7.7.27, T2
                    instr_bind!("11110|<i:1>|0|1101|1|<rn:4>|0|<imm3:3>|1111|<imm8:8>" identifiers (i, rn, imm3, imm8));
                    rebind_as_u32! {rn;}
                    let imm32 = match thumb_expand_imm(
                        bitstring_concat!(i : imm3 : imm8 | 12 bits),
                        xpsr,
                    ) {
                        Ok(v) => v,
                        Err(instr) => return instr,
                    };
                    if rn == 15 {
                        Instruction::Unpredictable
                    } else {
                        Instruction::Compare_Immediate {
                            rn: RegisterID::from_index(rn),
                            imm32,
                        }
                    }
                }
                ("1110x", "xxxx", "xxxx") => {
                    // A7.7.119, T2
                    instr_bind!("11110|<i:1>|0|1110|<s:1>|<rn:4>|0|<imm3:3>|<rd:4>|<imm8:8>" identifiers (i, s, rn, imm3, rd, imm8));
                    rebind_as_u32! {rd; rn;}
                    let imm32 = match thumb_expand_imm(
                        bitstring_concat!(i : imm3 : imm8 | 12 bits),
                        xpsr,
                    ) {
                        Ok(v) => v,
                        Err(instr) => return instr,
                    };

                    if rd == 13 || rd == 15 || rn == 13 || rn == 15 {
                        Instruction::Unpredictable
                    } else {
                        Instruction::ReverseSubtract_Immediate {
                            rd: RegisterID::from_index(rd),
                            rn: RegisterID::from_index(rn),
                            imm32,
                            setflags: s == bsc::C_1,
                            setflags_depends_on_it: false,
                        }
                    }
                }
                _ => Instruction::Undefined,
            }
        }
        ("10", "x1xxxxx", "0") => {
            // A5.3.3
            match instr!("111|10|x|1|<op:5>|<rn:4>|0|xxxxxxxxxxxxxxx" in order (op, rn)) {
                ("00000", "not 1111") => {
                    // A7.7.3, T4
                    instr_bind!("11110|<i:1>|1|0000|0|<rn:4>|0|<imm3:3>|<rd:4>|<imm8:8>" identifiers (i, rn, imm3, rd, imm8));
                    if rn == bsc::C_1111 {
                        unreachable!("SEE ADR")
                    } else if rn == bsc::C_1101 {
                        // SEE ADD (SP plus immediate)
                        // A7.7.5, T4
                        rebind_as_u32! {rd;}
                        if rd == 15 {
                            Instruction::Unpredictable
                        } else {
                            Instruction::Add_SPPlusImmediate {
                                rd: RegisterID::from_index(rd),
                                imm32: bitstring_concat!(i : imm3 : imm8 | 12 bits).zero_extend(),
                                setflags: false,
                            }
                        }
                    } else {
                        rebind_as_u32! {rn; rd;}
                        if rd == 13 || rd == 15 {
                            Instruction::Unpredictable
                        } else {
                            Instruction::Add_Immediate {
                                rd: RegisterID::from_index(rd),
                                rn: RegisterID::from_index(rn),
                                imm32: bitstring_concat!(i : imm3 : imm8 | 12 bits).zero_extend(),
                                setflags: false,
                                setflags_depends_on_it: false,
                            }
                        }
                    }
                }
                ("00000", "1111") => {
                    // A7.7.7, T3
                    instr_bind!("11110|<i:1>|10000|0|1111|0|<imm3:3>|<rd:4>|<imm8:8>" identifiers (i, imm3, rd, imm8));
                    rebind_as_u32! {rd;}
                    if rd == 13 || rd == 15 {
                        Instruction::Unpredictable
                    } else {
                        Instruction::AddressToRegister {
                            rd: RegisterID::from_index(rd),
                            imm32: bitstring_concat!(i : imm3 : imm8 | 12 bits).zero_extend(),
                            add: true,
                        }
                    }
                }
                ("00100", "xxxx") => {
                    // A7.7.76, T3
                    instr_bind!("11110|<i:1>|10|0|1|0|0|<imm4:4>|0|<imm3:3>|<rd:4>|<imm8:8>" identifiers (i, imm4, imm3, rd, imm8));
                    rebind_as_u32! {rd;}
                    if rd == 13 || rd == 15 {
                        Instruction::Unpredictable
                    } else {
                        let imm32 =
                            bitstring_concat!(imm4 : i : imm3 : imm8 | 16 bits).zero_extend();
                        Instruction::Move_Immediate {
                            rd: RegisterID::from_index(rd),
                            imm32,
                            setflags: false,
                            setflags_depends_on_it: false,
                            carry: xpsr.carry_flag(), // note: setflags = false anyway
                        }
                    }
                }
                ("01010", "not 1111") => {
                    // A7.7.174, T4
                    instr_bind!("11110|<i:1>|1|0101|0|<rn:4>|0|<imm3:3>|<rd:4>|<imm8:8>" identifiers (i, rn, imm3, rd, imm8));
                    if rn == bsc::C_1111 {
                        unreachable!("SEE ADR")
                    } else if rn == bsc::C_1101 {
                        // A7.7.176, T3
                        rebind_as_u32! {rd;}
                        if rd == 15 {
                            Instruction::Unpredictable
                        } else {
                            Instruction::Subtract_SPMinusImmediate {
                                rd: RegisterID::from_index(rd),
                                imm32: bitstring_concat!(i : imm3 : imm8 | 12 bits).zero_extend(),
                                setflags: false,
                            }
                        }
                    } else {
                        rebind_as_u32! {rd; rn;}
                        if rd == 13 || rd == 15 {
                            Instruction::Unpredictable
                        } else {
                            Instruction::Subtract_Immediate {
                                rd: RegisterID::from_index(rd),
                                rn: RegisterID::from_index(rn),
                                imm32: bitstring_concat!(i : imm3 : imm8 | 12 bits).zero_extend(),
                                setflags: false,
                                setflags_depends_on_it: false,
                            }
                        }
                    }
                }
                ("01010", "1111") => {
                    // A7.7.7, T2
                    instr_bind!("11110|<i:1>|10101|0|1111|0|<imm3:3>|<rd:4>|<imm8:8>" identifiers (i, imm3, rd, imm8));
                    rebind_as_u32! {rd;}
                    if rd == 13 || rd == 15 {
                        Instruction::Unpredictable
                    } else {
                        Instruction::AddressToRegister {
                            rd: RegisterID::from_index(rd),
                            imm32: bitstring_concat!(i : imm3 : imm8 | 12 bits).zero_extend(),
                            add: false,
                        }
                    }
                }
                ("01100", "xxxx") => {
                    // A7.7.79, T1
                    instr_bind!("11110|<i:1>|10|1|1|0|0|<imm4:4>|0|<imm3:3>|<rd:4>|<imm8:8>" identifiers (i, imm4, imm3, rd, imm8));
                    rebind_as_u32! {rd;}
                    if rd == 13 || rd == 15 {
                        Instruction::Unpredictable
                    } else {
                        Instruction::MoveTop {
                            rd: RegisterID::from_index(rd),
                            imm16: bitstring_concat!(imm4 : i : imm3 : imm8 | 16 bits),
                        }
                    }
                }
                ("10000", "xxxx") | ("10010", "xxxx") => {
                    // A7.7.152, T1
                    if instr_bind!("11110|(0)|11|00|<sh:1>|0|<rn:4>|0|<imm3:3>|<rd:4>|<imm2:2>|(0)|<sat_imm:5>" identifiers (sh, rn, imm3, rd, imm2, sat_imm))
                    {
                        let imm5 = bitstring_concat!(imm3 : imm2 | 5 bits);
                        if sh == bsc::C_1 && imm5 == bsc::C_0_0000 {
                            if have_dsp_ext() {
                                unimplemented!("SEE SSAT16"); // SSAT16 is not supported by CM
                            } else {
                                Instruction::Undefined
                            }
                        } else {
                            rebind_as_u32! {rd; rn;}
                            if rd == 13 || rd == 15 || rn == 13 || rn == 15 {
                                Instruction::Unpredictable
                            } else {
                                Instruction::SignedSaturate {
                                    rd: RegisterID::from_index(rd),
                                    rn: RegisterID::from_index(rn),
                                    saturate_to: u8::from(sat_imm) + 1,
                                    shift: Shift::decode_imm_shift(
                                        bitstring_concat!(sh : bsc::C_0 | 2 bits),
                                        imm5,
                                    ),
                                }
                            }
                        }
                    }
                }
                ("10100", "xxxx") => {
                    // A7.7.126, T1
                    if instr_bind!("11110|(0)|11|010|0|<rn:4>|0|<imm3:3>|<rd:4>|<imm2:2>|(0)|<widthm1:5>" identifiers (rn, imm3, rd, imm2, widthm1))
                    {
                        rebind_as_u32! {rd; rn;}
                        if rd == 13 || rd == 15 || rn == 13 || rn == 15 {
                            Instruction::Unpredictable
                        } else {
                            Instruction::SignedBitFieldExtract {
                                rd: RegisterID::from_index(rd),
                                rn: RegisterID::from_index(rn),
                                lsbit: bitstring_concat!(imm3 : imm2 | 5 bits).into(),
                                widthminus1: widthm1.into(),
                            }
                        }
                    }
                }
                ("10110", "not 1111") => {
                    // A7.7.14, T1
                    if instr_bind!("11110|(0)|11|011|0|<rn:4>|0|<imm3:3>|<rd:4>|<imm2:2>|(0)|<msb:5>" identifiers (rn, imm3, rd, imm2, msb))
                    {
                        if rn == bsc::C_1111 {
                            unreachable!("SEE BFC")
                        } else {
                            rebind_as_u32! {rd; rn;}
                            if rd == 13 || rd == 15 || rn == 13 {
                                Instruction::Unpredictable
                            } else {
                                Instruction::BitFieldInsert {
                                    rd: RegisterID::from_index(rd),
                                    rn: RegisterID::from_index(rn),
                                    msbit: msb.into(),
                                    lsbit: bitstring_concat!(imm3 : imm2 | 5 bits).into(),
                                }
                            }
                        }
                    }
                }
                ("10110", "1111") => {
                    // A7.7.13, T1
                    if instr_bind!("11110|(0)|11|011|0|1111|0|<imm3:3>|<rd:4>|<imm2:2>|(0)|<msb:5>" identifiers (imm3, rd, imm2, msb))
                    {
                        rebind_as_u32! {rd;}
                        if rd == 13 || rd == 15 {
                            Instruction::Unpredictable
                        } else {
                            Instruction::BitFieldClear {
                                rd: RegisterID::from_index(rd),
                                msbit: msb.into(),
                                lsbit: bitstring_concat!(imm3 : imm2 | 5 bits).into(),
                            }
                        }
                    }
                }
                ("11000", "xxxx") | ("11010", "xxxx") => {
                    // A7.7.213, T1
                    if instr_bind!("11110|(0)|11|10|<sh:1>|0|<rn:4>|0|<imm3:3>|<rd:4>|<imm2:2>|(0)|<sat_imm:5>" identifiers (sh, rn, imm3, rd, imm2, sat_imm))
                    {
                        let imm5 = bitstring_concat!(imm3 : imm2 | 5 bits);
                        if sh == bsc::C_1 && imm5 == bsc::C_0_0000 {
                            if have_dsp_ext() {
                                unimplemented!("SEE USAT16"); // USAT16 is not supported by CM
                            } else {
                                Instruction::Undefined
                            }
                        } else {
                            rebind_as_u32! {rd; rn;}
                            if rd == 13 || rd == 15 || rn == 13 || rn == 15 {
                                Instruction::Unpredictable
                            } else {
                                Instruction::UnsignedSaturate {
                                    rd: RegisterID::from_index(rd),
                                    rn: RegisterID::from_index(rn),
                                    saturate_to: u8::from(sat_imm),
                                    shift: Shift::decode_imm_shift(
                                        bitstring_concat!(sh : bsc::C_0 | 2 bits),
                                        imm5,
                                    ),
                                }
                            }
                        }
                    }
                }
                ("11100", "xxxx") => {
                    // A7.7.193, T1
                    if instr_bind!("11110|(0)|11|110|0|<rn:4>|0|<imm3:3>|<rd:4>|<imm2:2>|(0)|<widthm1:5>" identifiers (rn, imm3, rd, imm2, widthm1))
                    {
                        rebind_as_u32! {rd; rn;}
                        if rd == 13 || rd == 15 || rn == 13 || rn == 15 {
                            Instruction::Unpredictable
                        } else {
                            Instruction::UnsignedBitFieldExtract {
                                rd: RegisterID::from_index(rd),
                                rn: RegisterID::from_index(rn),
                                lsbit: bitstring_concat!(imm3 : imm2 | 5 bits).into(),
                                widthminus1: widthm1.into(),
                            }
                        }
                    }
                }
                _ => Instruction::Undefined,
            }
        }
        ("10", "xxxxxxx", "1") => {
            // A5.3.4
            match instr!("111|10|<op:7>|xxxx|1|<op1:3>|xxxxxxxxxxxx" in order (op1, op)) {
                ("0x0", "not x111xxx") => {
                    // A7.7.12, T3
                    instr_bind!("11110|<s:1>|<cond:4>|<imm6:6>|10|<j1:1>|0|<j2:1>|<imm11:11>" identifiers (s, cond, imm6, j1, j2, imm11));
                    if bitstring_extract!(cond<3:1> | 3 bits) == bsc::C_111 {
                        unreachable!("SEE \"Related encodings\"");
                    } else if xpsr.in_it_block() {
                        Instruction::Unpredictable
                    } else {
                        Instruction::Branch {
                            cond: Condition(cond),
                            imm32:
                                bitstring_concat!(s : j2 : j1 : imm6 : imm11 : bsc::C_0 | 21 bits)
                                    .sign_extend(),
                        }
                    }
                }
                ("0x0", "011100x") => {
                    // A7.7.83, T1
                    // B5.2.3, T1
                    if instr_bind!("11110|0|1110|0|(0)|<rn:4>|10|(0)|0|<mask:2>|(0)|(0)|<sysm:8>" identifiers (rn, mask, sysm))
                    {
                        // note: merged conditions to make clippy happy
                        rebind_as_u32! {rn;}
                        let sysm_u32 = u32::from(sysm);
                        if mask == bsc::C_00
                            || (mask != bsc::C_10 && !(0..=3).contains(&sysm_u32))
                            || rn == 13
                            || rn == 15
                            || !((0..=3).contains(&sysm_u32)
                                || (5..=9).contains(&sysm_u32)
                                || (16..=20).contains(&sysm_u32))
                        {
                            Instruction::Unpredictable
                        } else {
                            Instruction::MoveToSpecialRegisterFromARMRegister {
                                rn: RegisterID::from_index(rn),
                                sysm,
                                mask,
                            }
                        }
                    }
                }
                ("0x0", "0111010") => {
                    // A5.3.4 Hint instructions
                    match instr!("111|10|0111010|xxxx|10|x|0|x|<op1:3>|<op2:8>" in order (op1, op2))
                    {
                        ("not 000", "xxxxxxxx") => Instruction::Undefined,
                        ("000", "00000000") => {
                            // A7.7.88, T2
                            if instr_bind!("11110|0|111|01|0|(1)(1)(1)(1)|10|(0)|0|(0)|000|00000000" identifiers ())
                            {
                                Instruction::NoOperation
                            }
                        }
                        ("000", "00000001") => Instruction::Unsupported { name: "YIELD" }, // Most likely a NOP ([ARM-ARM] A5.2.5 & [ARM-TDG] 4.2.1).
                        ("000", "00000010") => {
                            // A7.7.261, T2
                            if instr_bind!("11110|0|111|01|0|(1)(1)(1)(1)|10|(0)|0|(0)|000|00000010" identifiers ())
                            {
                                Instruction::WaitForEvent
                            }
                        }
                        ("000", "00000011") => {
                            // A7.7.262, T2
                            if instr_bind!("11110|0|111|01|0|(1)(1)(1)(1)|10|(0)|0|(0)|000|00000011" identifiers ())
                            {
                                Instruction::WaitForInterrupt
                            }
                        }
                        ("000", "00000100") => {
                            // A7.7.129, T2
                            if instr_bind!("11110|0|111|01|0|(1)(1)(1)(1)|10|(0)|0|(0)|000|00000100" identifiers ())
                            {
                                Instruction::SendEvent
                            }
                        }
                        ("000", "00010100") => Instruction::Unsupported { name: "CSDB" },
                        ("000", "1111xxxx") => Instruction::Unsupported { name: "DBG" }, // Most likely a NOP ([ARM-ARM] A5.2.5 & [ARM-TDG] 4.2.1).
                        _ => Instruction::Unsupported {
                            name: "Unallocated hint. Executes as NOP, but software must not use it.",
                        },
                    }
                }
                ("0x0", "0111011") => {
                    // A5.3.4 Miscellaneous control instructions
                    match instr!("111|10|0111011|xxxx|10|x|0|xxxx|<op:4>|<option:4>" in order(op, option))
                    {
                        ("0010", "xxxx") => {
                            // A7.7.23, T1
                            if instr_bind!("11110|0|111|01|1|(1)(1)(1)(1)|10|(0)|0|(1)(1)(1)(1)|0010|(1)(1)(1)(1)" identifiers ())
                            {
                                Instruction::ClearExclusive
                            }
                        }
                        ("0100", "not 0x00") => {
                            // A7.7.34, T1
                            if instr_bind!("11110|0|111|01|1|(1)(1)(1)(1)|10|(0)|0|(1)(1)(1)(1)|0100|<option:4>" identifiers (option,))
                            {
                                Instruction::DataSynchronizationBarrier { option }
                            }
                        }
                        ("0100", "0000") => Instruction::Unsupported { name: "SSBB" },
                        ("0100", "0100") => Instruction::Unsupported { name: "PSSBB" },
                        ("0101", "xxxx") => {
                            // A7.7.33, T1
                            if instr_bind!("11110|0|111|01|1|(1)(1)(1)(1)|10|(0)|0|(1)(1)(1)(1)|0101|<option:4>" identifiers (option,))
                            {
                                Instruction::DataMemoryBarrier { option }
                            }
                        }
                        ("0110", "xxxx") => {
                            // A7.7.37, T1
                            if instr_bind!("11110|0|111|01|1|(1)(1)(1)(1)|10|(0)|0|(1)(1)(1)(1)|0110|<option:4>" identifiers (option,))
                            {
                                if xpsr.in_it_block() {
                                    Instruction::Unpredictable
                                } else {
                                    Instruction::InstructionSynchronizationBarrier { option }
                                }
                            }
                        }
                        _ => Instruction::Undefined,
                    }
                }
                ("0x0", "011111x") => {
                    // A7.7.82, T1
                    // B5.2.2, T1
                    if instr_bind!("11110|0|1111|1|(0)|(1)(1)(1)(1)|10|(0)|0|<rd:4>|<sysm:8>" identifiers (rd, sysm))
                    {
                        rebind_as_u32! {rd;}
                        let sysm_u32 = u32::from(sysm);
                        if rd == 13
                            || rd == 15
                            || !((0..=3).contains(&sysm_u32)
                                || (5..=9).contains(&sysm_u32)
                                || (16..=20).contains(&sysm_u32))
                        {
                            Instruction::Unpredictable
                        } else {
                            Instruction::MoveToRegisterFromSpecialRegister {
                                rd: RegisterID::from_index(rd),
                                sysm,
                            }
                        }
                    }
                }
                ("010", "1111111") => {
                    // A7.7.194, T2
                    instr_bind!("111|10|1111111|<imm4:4>|1|010|<imm12:12>" identifiers (imm4, imm12));
                    Instruction::PermanentlyUndefined {
                        imm32: bitstring_concat!(imm4 : imm12 | 16 bits).sign_extend(),
                    }
                }
                ("0x1", "xxxxxxx") => {
                    // A7.7.12, T4
                    instr_bind!("11110|<s:1>|<imm10:10>|10|<j1:1>|1|<j2:1>|<imm11:11>" identifiers (s, j1, j2, imm10, imm11));
                    let i1 = !(j1 ^ s);
                    let i2 = !(j2 ^ s);
                    let imm32 = bitstring_concat!(s : i1 : i2 : imm10 : imm11 : bsc::C_0 | 25 bits)
                        .sign_extend();
                    if xpsr.in_it_block() && !xpsr.last_in_it_block() {
                        Instruction::Unpredictable
                    } else {
                        Instruction::Branch {
                            cond: Condition::AL, // always = no condition
                            imm32,
                        }
                    }
                }
                ("1x1", "xxxxxxx") => {
                    // A7.7.18, T1
                    instr_bind!("11110|<s:1>|<imm10:10>|11|<j1:1>|1|<j2:1>|<imm11:11>" identifiers (s, j1, j2, imm10, imm11));
                    let i1 = !(j1 ^ s);
                    let i2 = !(j2 ^ s);
                    let imm32 = bitstring_concat!(s : i1 : i2 : imm10 : imm11 : bsc::C_0 | 25 bits)
                        .sign_extend();
                    // TODO: in ARMv7-A, last bit of imm11 has to be 0
                    if xpsr.in_it_block() && !xpsr.last_in_it_block() {
                        Instruction::Unpredictable
                    } else {
                        Instruction::BranchWithLink_Immediate { imm32 }
                    }
                }
                _ => Instruction::Undefined,
            }
        }
        ("11", "000xxx0", "x") => {
            // A5.3.10
            match instr!("111|1100|0|<op1:3>|0|xxxx|xxxx|<op2:6>|xxxxxx" in order (op1, op2)) {
                ("100", "xxxxxx") => {
                    // A7.7.163, T2
                    instr_bind!("11111|00|0|1|00|0|<rn:4>|<rt:4>|<imm12:12>" identifiers (rn, rt, imm12));
                    if rn == bsc::C_1111 {
                        Instruction::Undefined
                    } else {
                        rebind_as_u32! {rt; rn;}
                        if rt == 13 || rt == 15 {
                            Instruction::Unpredictable
                        } else {
                            Instruction::StoreRegisterByte_Immediate {
                                rt: RegisterID::from_index(rt),
                                rn: RegisterID::from_index(rn),
                                imm32: imm12.zero_extend(),

                                index: true,
                                add: true,
                                wback: false,
                            }
                        }
                    }
                }
                ("000", "1xxxxx") => {
                    // A7.7.163, T3
                    instr_bind!("11111|00|0|0|00|0|<rn:4>|<rt:4>|1|<p:1>|<u:1>|<w:1>|<imm8:8>" identifiers (rn, rt, p, u, w, imm8));
                    if p == bsc::C_1 && u == bsc::C_1 && w == bsc::C_0 {
                        // A7.7.165, T1
                        if rn == bsc::C_1111 {
                            Instruction::Undefined
                        } else {
                            rebind_as_u32! {rt; rn;}
                            if rt == 13 || rt == 15 {
                                Instruction::Unpredictable
                            } else {
                                Instruction::StoreRegisterByteUnprivileged {
                                    rt: RegisterID::from_index(rt),
                                    rn: RegisterID::from_index(rn),
                                    imm32: imm8.zero_extend(),
                                    // Regarding fields, see Note on variant definition.
                                }
                            }
                        }
                    } else if rn == bsc::C_1111 || (p == bsc::C_0 && w == bsc::C_0) {
                        Instruction::Undefined
                    } else {
                        rebind_as_u32! {rt; rn;}
                        let index = p == bsc::C_1;
                        let add = u == bsc::C_1;
                        let wback = w == bsc::C_1;

                        if rt == 13 || rt == 15 || (wback && rn == rt) {
                            Instruction::Unpredictable
                        } else {
                            Instruction::StoreRegisterByte_Immediate {
                                rt: RegisterID::from_index(rt),
                                rn: RegisterID::from_index(rn),
                                imm32: imm8.zero_extend(),

                                index,
                                add,
                                wback,
                            }
                        }
                    }
                }
                ("000", "0xxxxx") => {
                    // A7.7.164, T2
                    instr_bind!("11111|00|0|0|00|0|<rn:4>|<rt:4>|0|00000|<imm2:2>|<rm:4>" identifiers (rn, rt, imm2, rm));
                    if rn == bsc::C_1111 {
                        Instruction::Undefined
                    } else {
                        rebind_as_u32! {rt; rn; rm;}
                        if rt == 13 || rt == 15 || rm == 13 || rm == 15 {
                            Instruction::Unpredictable
                        } else {
                            Instruction::StoreRegisterByte_Register {
                                rt: RegisterID::from_index(rt),
                                rn: RegisterID::from_index(rn),
                                rm: RegisterID::from_index(rm),
                                shift: Shift::LSL(imm2.into()),

                                index: true,
                                add: true,
                                wback: false,
                            }
                        }
                    }
                }
                ("101", "xxxxxx") => {
                    // A7.7.170, T2
                    instr_bind!("11111|00|0|1|01|0|<rn:4>|<rt:4>|<imm12:12>" identifiers (rn, rt, imm12));
                    if rn == bsc::C_1111 {
                        Instruction::Undefined
                    } else {
                        rebind_as_u32! {rt; rn;}
                        if rt == 13 || rt == 15 {
                            Instruction::Unpredictable
                        } else {
                            Instruction::StoreRegisterHalfword_Immediate {
                                rt: RegisterID::from_index(rt),
                                rn: RegisterID::from_index(rn),
                                imm32: imm12.zero_extend(),

                                index: true,
                                add: true,
                                wback: false,
                            }
                        }
                    }
                }
                ("001", "1xxxxx") => {
                    // A7.7.170, T3
                    instr_bind!("11111|00|0|0|01|0|<rn:4>|<rt:4>|1|<p:1>|<u:1>|<w:1>|<imm8:8>" identifiers (rn, rt, p, u, w, imm8));
                    if p == bsc::C_1 && u == bsc::C_1 && w == bsc::C_0 {
                        // A7.7.172, T1
                        if rn == bsc::C_1111 {
                            Instruction::Undefined
                        } else {
                            rebind_as_u32! {rt; rn;}
                            if rt == 13 || rt == 15 {
                                Instruction::Unpredictable
                            } else {
                                Instruction::StoreRegisterHalfwordUnprivileged {
                                    rt: RegisterID::from_index(rt),
                                    rn: RegisterID::from_index(rn),
                                    imm32: imm8.zero_extend(),
                                    // Regarding fields, see Note on variant definition.
                                }
                            }
                        }
                    } else if rn == bsc::C_1111 || (p == bsc::C_0 && w == bsc::C_0) {
                        Instruction::Undefined
                    } else {
                        rebind_as_u32! {rt; rn;}
                        let index = p == bsc::C_1;
                        let add = u == bsc::C_1;
                        let wback = w == bsc::C_1;

                        if rt == 13 || rt == 15 || (wback && rn == rt) {
                            Instruction::Unpredictable
                        } else {
                            Instruction::StoreRegisterHalfword_Immediate {
                                rt: RegisterID::from_index(rt),
                                rn: RegisterID::from_index(rn),
                                imm32: imm8.zero_extend(),

                                index,
                                add,
                                wback,
                            }
                        }
                    }
                }
                ("001", "0xxxxx") => {
                    // A7.7.171, T2
                    instr_bind!("11111|00|0|0|01|0|<rn:4>|<rt:4>|0|00000|<imm2:2>|<rm:4>" identifiers (rn, rt, imm2, rm));
                    if rn == bsc::C_1111 {
                        Instruction::Undefined
                    } else {
                        rebind_as_u32! {rt; rn; rm;}
                        if rt == 13 || rt == 15 || rm == 13 || rm == 15 {
                            Instruction::Unpredictable
                        } else {
                            Instruction::StoreRegisterHalfword_Register {
                                rt: RegisterID::from_index(rt),
                                rn: RegisterID::from_index(rn),
                                rm: RegisterID::from_index(rm),
                                shift: Shift::LSL(imm2.into()),

                                index: true,
                                add: true,
                                wback: false,
                            }
                        }
                    }
                }
                ("110", "xxxxxx") => {
                    // A7.7.161, T3
                    instr_bind!("11111|00|0|1|10|0|<rn:4>|<rt:4>|<imm12:12>" identifiers (rn, rt, imm12));
                    if rn == bsc::C_1111 {
                        Instruction::Undefined
                    } else {
                        rebind_as_u32! {rt; rn;}
                        if rt == 15 {
                            Instruction::Unpredictable
                        } else {
                            Instruction::StoreRegister_Immediate {
                                rt: RegisterID::from_index(rt),
                                rn: RegisterID::from_index(rn),
                                imm32: imm12.zero_extend(),

                                index: true,
                                add: true,
                                wback: false,
                            }
                        }
                    }
                }
                ("010", "1xxxxx") => {
                    // A7.7.161, T4
                    instr_bind!("11111|00|0|0|10|0|<rn:4>|<rt:4>|1|<p:1>|<u:1>|<w:1>|<imm8:8>" identifiers (rn, rt, p, u, w, imm8));
                    if p == bsc::C_1 && u == bsc::C_1 && w == bsc::C_0 {
                        // A7.7.173, T1
                        if rn == bsc::C_1111 {
                            Instruction::Undefined
                        } else {
                            rebind_as_u32! {rt; rn;}
                            if rt == 13 || rt == 15 {
                                Instruction::Unpredictable
                            } else {
                                Instruction::StoreRegisterUnprivileged {
                                    rt: RegisterID::from_index(rt),
                                    rn: RegisterID::from_index(rn),
                                    imm32: imm8.zero_extend(),
                                    // Regarding fields, see Note on variant definition.
                                }
                            }
                        }
                    } else if rn == bsc::C_1101
                        && p == bsc::C_1
                        && u == bsc::C_0
                        && w == bsc::C_1
                        && imm8 == bsc::C_0000_0100
                    {
                        // SEE PUSH
                        // A7.7.101, T3

                        // Note: `push.w { rt }` seems to be a shortcut notation for `str rt, [sp, -4]!`.
                        //       (see: `cmemu-lib/src/component/core/instruction.rs` ctrl+f "A7.7.101")
                        //
                        //       To be more consistent with docs, we leave their decoding stages
                        //       separate although after constant propagation they do the same operations.
                        //       (note that: `rn == bsc::C_1101 && w == bsc::C_1` (from `if` condition)
                        //       and `(wback && rn == rt) <=> rt == 13` when `rn == 13 && wback == true`)
                        rebind_as_u32! {rt;}
                        if rt == 13 || rt == 15 {
                            Instruction::Unpredictable
                        } else {
                            Instruction::StoreRegister_Immediate {
                                rt: RegisterID::from_index(rt),
                                rn: RegisterID::SP,
                                imm32: bsc::C_0000_0100.zero_extend(),

                                index: true,
                                add: false,
                                wback: true,
                            }
                        }
                    } else if rn == bsc::C_1111 || (p == bsc::C_0 && w == bsc::C_0) {
                        Instruction::Undefined
                    } else {
                        rebind_as_u32! {rt; rn;}
                        let index = p == bsc::C_1;
                        let add = u == bsc::C_1;
                        let wback = w == bsc::C_1;

                        if rt == 15 || (wback && rn == rt) {
                            Instruction::Unpredictable
                        } else {
                            Instruction::StoreRegister_Immediate {
                                rt: RegisterID::from_index(rt),
                                rn: RegisterID::from_index(rn),
                                imm32: imm8.zero_extend(),

                                index,
                                add,
                                wback,
                            }
                        }
                    }
                }
                ("010", "0xxxxx") => {
                    // A7.7.162, T2
                    instr_bind!("11111|00|0|0|10|0|<rn:4>|<rt:4>|0|00000|<imm2:2>|<rm:4>" identifiers (rn, rt, imm2, rm));
                    if rn == bsc::C_1111 {
                        Instruction::Undefined
                    } else {
                        rebind_as_u32! {rt; rn; rm;}
                        if rt == 15 || rm == 13 || rm == 15 {
                            Instruction::Unpredictable
                        } else {
                            Instruction::StoreRegister_Register {
                                rt: RegisterID::from_index(rt),
                                rn: RegisterID::from_index(rn),
                                rm: RegisterID::from_index(rm),
                                shift: Shift::LSL(imm2.into()),

                                index: true,
                                add: true,
                                wback: false,
                            }
                        }
                    }
                }
                _ => Instruction::Undefined,
            }
        }
        ("11", "00xx001", "x") => {
            // A5.3.9
            match instr!("111|1100|<op1:2>|00|1|<rn:4>|<rt:4>|<op2:6>|xxxxxx" in order(op1, op2, rn, rt))
            {
                ("0x", "xxxxxx", "1111", "not 1111") => {
                    // A7.7.47, T1
                    instr_bind!("11111|00|0|<u:1>|00|1|1111|<rt:4>|<imm12:12>" identifiers (u, rt, imm12));
                    if rt == bsc::C_1111 {
                        unreachable!("SEE PLD")
                    } else {
                        rebind_as_u32! {rt;}
                        if rt == 13 {
                            Instruction::Unpredictable
                        } else {
                            Instruction::LoadRegisterByte_Literal {
                                rt: RegisterID::from_index(rt),
                                imm32: imm12.zero_extend(),
                                add: u == bsc::C_1,
                            }
                        }
                    }
                }
                ("01", "xxxxxx", "not 1111", "not 1111") => {
                    // A7.7.46, T2
                    instr_bind!("11111|00|0|1|00|1|<rn:4>|<rt:4>|<imm12:12>" identifiers (rn, rt, imm12));
                    if rt == bsc::C_1111 {
                        unreachable!("SEE PLD")
                    } else if rn == bsc::C_1111 {
                        unreachable!("SEE LDRB (literal)")
                    } else {
                        rebind_as_u32! {rt; rn;}
                        if rt == 13 {
                            Instruction::Unpredictable
                        } else {
                            Instruction::LoadRegisterByte_Immediate {
                                rt: RegisterID::from_index(rt),
                                rn: RegisterID::from_index(rn),
                                imm32: imm12.zero_extend(),

                                index: true,
                                add: true,
                                wback: false,
                            }
                        }
                    }
                }
                ("00", "1xx1xx", "not 1111", "not 1111")
                | ("00", "1100xx", "not 1111", "not 1111") => {
                    // A7.7.46, T3
                    instr_bind!("11111|00|0|0|00|1|<rn:4>|<rt:4>|1|<p:1>|<u:1>|<w:1>|<imm8:8>" identifiers (rn, rt, p, u, w, imm8));
                    if rt == bsc::C_1111 && p == bsc::C_1 && u == bsc::C_0 && w == bsc::C_0 {
                        unreachable!("SEE PLD (immediate)")
                    } else if rn == bsc::C_1111 {
                        unreachable!("SEE LDRB (literal)")
                    } else if p == bsc::C_1 && u == bsc::C_1 && w == bsc::C_0 {
                        unreachable!("SEE LDRBT")
                    } else if p == bsc::C_0 && w == bsc::C_0 {
                        unreachable!("UNDEFINED")
                    } else {
                        rebind_as_u32! {rt; rn;}
                        let index = p == bsc::C_1;
                        let add = u == bsc::C_1;
                        let wback = w == bsc::C_1;
                        if rt == 13 || (wback && rn == rt) {
                            Instruction::Unpredictable
                        } else if rt == 15 && (p == bsc::C_0 || u == bsc::C_1 || w == bsc::C_1) {
                            unreachable!("UNPREDICTABLE")
                        } else {
                            Instruction::LoadRegisterByte_Immediate {
                                rt: RegisterID::from_index(rt),
                                rn: RegisterID::from_index(rn),
                                imm32: imm8.zero_extend(),

                                index,
                                add,
                                wback,
                            }
                        }
                    }
                }
                ("00", "1110xx", "not 1111", "not 1111") => {
                    // A7.7.49, T1
                    instr_bind!("11111|00|0|0|00|1|<rn:4>|<rt:4>|1|110|<imm8:8>" identifiers (rn, rt, imm8));
                    if rn == bsc::C_1111 {
                        unreachable!("SEE LDRB (literal)");
                    } else {
                        rebind_as_u32! {rt; rn;}
                        if rt == 13 || rt == 15 {
                            Instruction::Unpredictable
                        } else {
                            Instruction::LoadRegisterByteUnprivileged {
                                rt: RegisterID::from_index(rt),
                                rn: RegisterID::from_index(rn),
                                imm32: imm8.zero_extend(),
                                // Regarding fields, see Note on variant definition.
                            }
                        }
                    }
                }
                ("00", "000000", "not 1111", "not 1111") => {
                    // A7.7.48, T2
                    instr_bind!("11111|00|0|0|00|1|<rn:4>|<rt:4>|0|00000|<imm2:2>|<rm:4>" identifiers(rn, rt, imm2, rm));
                    if rt == bsc::C_1111 {
                        unreachable!("SEE PLD")
                    } else if rn == bsc::C_1111 {
                        unreachable!("SEE LDRB (literal)")
                    } else {
                        rebind_as_u32! {rt; rn; rm;}
                        if rt == 13 || rm == 13 || rm == 15 {
                            Instruction::Unpredictable
                        } else {
                            Instruction::LoadRegisterByte_Register {
                                rt: RegisterID::from_index(rt),
                                rn: RegisterID::from_index(rn),
                                rm: RegisterID::from_index(rm),
                                shift: Shift::LSL(imm2.into()),

                                index: true,
                                add: true,
                                wback: false,
                            }
                        }
                    }
                }
                ("1x", "xxxxxx", "1111", "not 1111") => {
                    // A7.7.60, T1
                    instr_bind!("11111|00|1|<u:1>|00|1|1111|<rt:4>|<imm12:12>" identifiers(u, rt, imm12));
                    if rt == bsc::C_1111 {
                        unreachable!("SEE PLI")
                    } else {
                        rebind_as_u32! {rt;}
                        if rt == 13 {
                            Instruction::Unpredictable
                        } else {
                            Instruction::LoadRegisterSignedByte_Literal {
                                rt: RegisterID::from_index(rt),
                                imm32: imm12.zero_extend(),
                                add: u == bsc::C_1,
                            }
                        }
                    }
                }
                ("11", "xxxxxx", "not 1111", "not 1111") => {
                    // A7.7.59, T1
                    instr_bind!("11111|00|1|1|00|1|<rn:4>|<rt:4>|<imm12:12>" identifiers(rn, rt, imm12));
                    if rt == bsc::C_1111 {
                        unreachable!("SEE PLI")
                    } else if rn == bsc::C_1111 {
                        unreachable!("SEE LDRSB (literal)")
                    } else {
                        rebind_as_u32! {rt; rn;}
                        if rt == 13 {
                            Instruction::Unpredictable
                        } else {
                            Instruction::LoadRegisterSignedByte_Immediate {
                                rt: RegisterID::from_index(rt),
                                rn: RegisterID::from_index(rn),
                                imm32: imm12.zero_extend(),

                                index: true,
                                add: true,
                                wback: false,
                            }
                        }
                    }
                }
                ("10", "1xx1xx", "not 1111", "not 1111")
                | ("10", "1100xx", "not 1111", "not 1111") => {
                    // A7.7.59, T2
                    instr_bind!("11111|00|1|0|00|1|<rn:4>|<rt:4>|1|<p:1>|<u:1>|<w:1>|<imm8:8>" identifiers(rn, rt, p, u, w, imm8));
                    if rt == bsc::C_1111 && p == bsc::C_1 && u == bsc::C_0 && w == bsc::C_0 {
                        unreachable!("SEE PLI")
                    } else if rn == bsc::C_1111 {
                        unreachable!("SEE LDRSB (literal)")
                    } else if p == bsc::C_1 && u == bsc::C_1 && w == bsc::C_0 {
                        unreachable!("SEE LDRSBT")
                    } else if p == bsc::C_0 && w == bsc::C_0 {
                        unreachable!("UNDEFINED")
                    } else {
                        rebind_as_u32! {rt; rn;}
                        let index = p == bsc::C_1;
                        let add = u == bsc::C_1;
                        let wback = w == bsc::C_1;
                        if rt == 13 || (rt == 15 && w == bsc::C_1) || (wback && rn == rt) {
                            Instruction::Unpredictable
                        } else {
                            Instruction::LoadRegisterSignedByte_Immediate {
                                rt: RegisterID::from_index(rt),
                                rn: RegisterID::from_index(rn),
                                imm32: imm8.zero_extend(),

                                index,
                                add,
                                wback,
                            }
                        }
                    }
                }
                ("10", "1110xx", "not 1111", "not 1111") => {
                    // A7.7.62, T1
                    instr_bind!("11111|00|1|0|00|1|<rn:4>|<rt:4>|1|110|<imm8:8>" identifiers (rn, rt, imm8));
                    if rn == bsc::C_1111 {
                        unreachable!("SEE LDRSB (literal)");
                    } else {
                        rebind_as_u32! {rt; rn;}
                        if rt == 13 || rt == 15 {
                            Instruction::Unpredictable
                        } else {
                            Instruction::LoadRegisterSignedByteUnprivileged {
                                rt: RegisterID::from_index(rt),
                                rn: RegisterID::from_index(rn),
                                imm32: imm8.zero_extend(),
                                // Regarding fields, see Note on variant definition.
                            }
                        }
                    }
                }
                ("10", "000000", "not 1111", "not 1111") => {
                    // A7.7.61, T2
                    instr_bind!("11111|00|1|0|00|1|<rn:4>|<rt:4>|0|00000|<imm2:2>|<rm:4>" identifiers(rn, rt, imm2, rm));
                    if rt == bsc::C_1111 {
                        unreachable!("SEE PLI")
                    } else if rn == bsc::C_1111 {
                        unreachable!("SEE LDRSB (literal)")
                    } else {
                        rebind_as_u32! {rt; rn; rm;}
                        if rt == 13 || rm == 13 || rm == 15 {
                            Instruction::Unpredictable
                        } else {
                            Instruction::LoadRegisterSignedByte_Register {
                                rt: RegisterID::from_index(rt),
                                rn: RegisterID::from_index(rn),
                                rm: RegisterID::from_index(rm),
                                shift: Shift::LSL(imm2.into()),

                                index: true,
                                add: true,
                                wback: false,
                            }
                        }
                    }
                }
                ("0x", "xxxxxx", "1111", "1111") => Instruction::Unsupported {
                    // Most likely executes as NOP (see [ARM-TDG] 4.2.1).
                    name: "PLD (literal)",
                },
                ("00", "1100xx", "not 1111", "1111") | ("01", "xxxxxx", "not 1111", "1111") => {
                    Instruction::Unsupported {
                        // Most likely executes as NOP (see [ARM-TDG] 4.2.1).
                        name: "PLD (immediate)",
                    }
                }
                ("00", "000000", "not 1111", "1111") => Instruction::Unsupported {
                    // Most likely executes as NOP (see [ARM-TDG] 4.2.1).
                    name: "PLD (register)",
                },
                ("00", "1xx1xx", "not 1111", "1111")
                | ("00", "1110xx", "not 1111", "1111")
                | ("10", "1xx1xx", "not 1111", "1111")
                | ("10", "1110xx", "not 1111", "1111") => Instruction::Unpredictable,
                ("1x", "xxxxxx", "1111", "1111")
                | ("11", "xxxxxx", "not 1111", "1111")
                | ("10", "1100xx", "not 1111", "1111") => Instruction::Unsupported {
                    // Most likely executes as NOP (see [ARM-TDG] 4.2.1).
                    name: "PLI (immediate, literal)",
                },
                ("10", "000000", "not 1111", "1111") => Instruction::Unsupported {
                    // Most likely executes as NOP (see [ARM-TDG] 4.2.1).
                    name: "PLI (register)",
                },
                _ => Instruction::Undefined,
            }
        }
        ("11", "00xx011", "x") => {
            // A5.3.8
            match instr!("111|1100|<op1:2>|01|1|<rn:4>|<rt:4>|<op2:6>|xxxxxx" in order(op1, op2, rn, rt))
            {
                ("0x", "xxxxxx", "1111", "not 1111") => {
                    // A7.7.56, T1
                    instr_bind!("11111|00|0|<u:1>|01|1|1111|<rt:4>|<imm12:12>" identifiers(u, rt, imm12));
                    if rt == bsc::C_1111 {
                        unreachable!("SEE PLD (literal)")
                    } else {
                        rebind_as_u32! {rt;}
                        if rt == 13 {
                            Instruction::Unpredictable
                        } else {
                            Instruction::LoadRegisterHalfword_Literal {
                                rt: RegisterID::from_index(rt),
                                imm32: imm12.zero_extend(),
                                add: u == bsc::C_1,
                            }
                        }
                    }
                }
                ("00", "1xx1xx", "not 1111", "not 1111")
                | ("00", "1100xx", "not 1111", "not 1111") => {
                    // A7.7.55, T3
                    instr_bind!("11111|00|0|0|01|1|<rn:4>|<rt:4>|1|<p:1>|<u:1>|<w:1>|<imm8:8>" identifiers(rn, rt, p, u, w, imm8));
                    if rn == bsc::C_1111 {
                        unreachable!("SEE LDRH (literal)")
                    } else if rt == bsc::C_1111 && p == bsc::C_1 && u == bsc::C_0 && w == bsc::C_0 {
                        unreachable!("SEE PLD")
                    } else if p == bsc::C_1 && u == bsc::C_1 && w == bsc::C_0 {
                        unreachable!("SEE LDRHT")
                    } else if p == bsc::C_0 && w == bsc::C_0 {
                        unreachable!("UNDEFINED")
                    } else {
                        rebind_as_u32! {rt; rn;}
                        let index = p == bsc::C_1;
                        let add = u == bsc::C_1;
                        let wback = w == bsc::C_1;
                        if rt == 13 || (rt == 15 && w == bsc::C_1) || (wback && rn == rt) {
                            Instruction::Unpredictable
                        } else {
                            Instruction::LoadRegisterHalfword_Immediate {
                                rt: RegisterID::from_index(rt),
                                rn: RegisterID::from_index(rn),
                                imm32: imm8.zero_extend(),

                                index,
                                add,
                                wback,
                            }
                        }
                    }
                }
                ("01", "xxxxxx", "not 1111", "not 1111") => {
                    // A7.7.55, T2
                    instr_bind!("11111|00|0|1|01|1|<rn:4>|<rt:4>|<imm12:12>" identifiers(rn, rt, imm12));
                    if rt == bsc::C_1111 {
                        unreachable!("SEE PLD (immediate)")
                    } else if rn == bsc::C_1111 {
                        unreachable!("SEE LDRH (literal)")
                    } else {
                        rebind_as_u32! {rt; rn;}
                        if rt == 13 {
                            Instruction::Unpredictable
                        } else {
                            Instruction::LoadRegisterHalfword_Immediate {
                                rt: RegisterID::from_index(rt),
                                rn: RegisterID::from_index(rn),
                                imm32: imm12.zero_extend(),

                                index: true,
                                add: true,
                                wback: false,
                            }
                        }
                    }
                }
                ("00", "000000", "not 1111", "not 1111") => {
                    // A7.7.57, T2
                    instr_bind!("11111|00|0|0|01|1|<rn:4>|<rt:4>|0|00000|<imm2:2>|<rm:4>" identifiers(rn, rt, imm2, rm));
                    if rn == bsc::C_1111 {
                        unreachable!("SEE LDRH (literal)")
                    } else if rt == bsc::C_1111 {
                        unreachable!("SEE \"Related instructions\"")
                    } else {
                        rebind_as_u32! {rt; rn; rm;}
                        if rt == 13 || rm == 13 || rm == 15 {
                            Instruction::Unpredictable
                        } else {
                            Instruction::LoadRegisterHalfword_Register {
                                rt: RegisterID::from_index(rt),
                                rn: RegisterID::from_index(rn),
                                rm: RegisterID::from_index(rm),
                                shift: Shift::LSL(imm2.into()),

                                index: true,
                                add: true,
                                wback: false,
                            }
                        }
                    }
                }
                ("00", "1110xx", "not 1111", "not 1111") => {
                    // A7.7.58, T1
                    instr_bind!("11111|00|0|0|01|1|<rn:4>|<rt:4>|1|110|<imm8:8>" identifiers (rn, rt, imm8));
                    if rn == bsc::C_1111 {
                        unreachable!("SEE LDRH (literal)");
                    } else {
                        rebind_as_u32! {rt; rn;}
                        if rt == 13 || rt == 15 {
                            Instruction::Unpredictable
                        } else {
                            Instruction::LoadRegisterHalfwordUnprivileged {
                                rt: RegisterID::from_index(rt),
                                rn: RegisterID::from_index(rn),
                                imm32: imm8.zero_extend(),
                                // Regarding fields, see Note on variant definition.
                            }
                        }
                    }
                }
                // Note: this match arm somehow causes that rustfmt doesn't work.
                ("00", "000000", "not 1111", "1111")
                | ("00", "1100xx", "not 1111", "1111")
                | ("01", "xxxxxx", "not 1111", "1111")
                | ("10", "000000", "not 1111", "1111")
                | ("10", "1100xx", "not 1111", "1111")
                | ("1x", "xxxxxx", "1111", "1111")
                | ("11", "xxxxxx", "not 1111", "1111") => Instruction::Unsupported {
                    // So a panic is desired.
                    name: "Unallocated memory hint. Treat as NOP. Software must not use these encodings.",
                },
                ("00", "1xx1xx", "not 1111", "1111")
                | ("00", "1110xx", "not 1111", "1111")
                | ("0x", "xxxxxx", "1111", "1111")
                | ("10", "1xx1xx", "not 1111", "1111")
                | ("10", "1110xx", "not 1111", "1111") => Instruction::Unpredictable,
                ("10", "1xx1xx", "not 1111", "not 1111")
                | ("10", "1100xx", "not 1111", "not 1111") => {
                    // A7.7.63, T2
                    instr_bind!("11111|00|1|0|01|1|<rn:4>|<rt:4>|1|<p:1>|<u:1>|<w:1>|<imm8:8>" identifiers(rn, rt, p, u, w, imm8));
                    if rn == bsc::C_1111 {
                        unreachable!("SEE LDRSH (literal)")
                    } else if rt == bsc::C_1111 && p == bsc::C_1 && u == bsc::C_0 && w == bsc::C_0 {
                        unreachable!("SEE \"Related instructions\"")
                    } else if p == bsc::C_1 && u == bsc::C_1 && w == bsc::C_0 {
                        unreachable!("SEE LDRSHT")
                    } else if p == bsc::C_0 && w == bsc::C_0 {
                        unreachable!("UNDEFINED")
                    } else {
                        rebind_as_u32! {rt; rn;}
                        let index = p == bsc::C_1;
                        let add = u == bsc::C_1;
                        let wback = w == bsc::C_1;
                        if rt == 13 || (rt == 15 && w == bsc::C_1) || (wback && rn == rt) {
                            Instruction::Unpredictable
                        } else {
                            Instruction::LoadRegisterSignedHalfword_Immediate {
                                rt: RegisterID::from_index(rt),
                                rn: RegisterID::from_index(rn),
                                imm32: imm8.zero_extend(),

                                index,
                                add,
                                wback,
                            }
                        }
                    }
                }
                ("11", "xxxxxx", "not 1111", "not 1111") => {
                    // A7.7.63, T1
                    instr_bind!("11111|00|1|1|01|1|<rn:4>|<rt:4>|<imm12:12>" identifiers(rn, rt, imm12));
                    if rn == bsc::C_1111 {
                        unreachable!("SEE LDRSH (literal)")
                    } else if rt == bsc::C_1111 {
                        unreachable!("SEE \"Related instructions\"")
                    } else {
                        rebind_as_u32! {rt; rn;}
                        if rt == 13 {
                            Instruction::Unpredictable
                        } else {
                            Instruction::LoadRegisterSignedHalfword_Immediate {
                                rt: RegisterID::from_index(rt),
                                rn: RegisterID::from_index(rn),
                                imm32: imm12.zero_extend(),

                                index: true,
                                add: true,
                                wback: false,
                            }
                        }
                    }
                }
                ("1x", "xxxxxx", "1111", "not 1111") => {
                    // A7.7.64, T1
                    instr_bind!("11111|00|1|<u:1>|01|1|1111|<rt:4>|<imm12:12>" identifiers(u, rt, imm12));
                    if rt == bsc::C_1111 {
                        unreachable!("SEE \"Related instructions\"")
                    } else {
                        rebind_as_u32! {rt;}
                        if rt == 13 {
                            Instruction::Unpredictable
                        } else {
                            Instruction::LoadRegisterSignedHalfword_Literal {
                                rt: RegisterID::from_index(rt),
                                imm32: imm12.zero_extend(),
                                add: u == bsc::C_1,
                            }
                        }
                    }
                }
                ("10", "000000", "not 1111", "not 1111") => {
                    // A7.7.65, T2
                    instr_bind!("11111|00|1|0|01|1|<rn:4>|<rt:4>|0|00000|<imm2:2>|<rm:4>" identifiers(rn, rt, imm2, rm));
                    if rn == bsc::C_1111 {
                        unreachable!("SEE LDRSH (literal)")
                    } else if rt == bsc::C_1111 {
                        unreachable!("SEE \"Related instructions\"")
                    } else {
                        rebind_as_u32! {rt; rn; rm;}
                        if rt == 13 || rm == 13 || rm == 15 {
                            Instruction::Unpredictable
                        } else {
                            Instruction::LoadRegisterSignedHalfword_Register {
                                rt: RegisterID::from_index(rt),
                                rn: RegisterID::from_index(rn),
                                rm: RegisterID::from_index(rm),
                                shift: Shift::LSL(imm2.into()),

                                index: true,
                                add: true,
                                wback: false,
                            }
                        }
                    }
                }
                ("10", "1110xx", "not 1111", "not 1111") => {
                    // A7.7.66, T1
                    instr_bind!("11111|00|1|0|01|1|<rn:4>|<rt:4>|1|110|<imm8:8>" identifiers (rn, rt, imm8));
                    if rn == bsc::C_1111 {
                        unreachable!("SEE LDRSH (literal)");
                    } else {
                        rebind_as_u32! {rt; rn;}
                        if rt == 13 || rt == 15 {
                            Instruction::Unpredictable
                        } else {
                            Instruction::LoadRegisterSignedHalfwordUnprivileged {
                                rt: RegisterID::from_index(rt),
                                rn: RegisterID::from_index(rn),
                                imm32: imm8.zero_extend(),
                                // Regarding fields, see Note on variant definition.
                            }
                        }
                    }
                }
                _ => Instruction::Undefined,
            }
        }
        ("11", "00xx101", "x") => {
            // A5.3.7
            match instr!("111|1100|<op1:2>|10|1|<rn:4>|xxxx|<op2:6>|xxxxxx" in order (op1, op2, rn))
            {
                ("01", "xxxxxx", "not 1111") => {
                    // A7.7.43, T3
                    instr_bind!("11111|00|0|1|10|1|<rn:4>|<rt:4>|<imm12:12>" identifiers (rn, rt, imm12));
                    if rn == bsc::C_1111 {
                        unreachable!("SEE LDR (literal)");
                    }
                    rebind_as_u32! {rt; rn;}
                    if rt == 15 && xpsr.in_it_block() && !xpsr.last_in_it_block() {
                        Instruction::Unpredictable
                    } else {
                        Instruction::LoadRegister_Immediate {
                            rt: RegisterID::from_index(rt),
                            rn: RegisterID::from_index(rn),
                            imm32: imm12.zero_extend(),

                            index: true,
                            add: true,
                            wback: false,
                        }
                    }
                }
                ("00", "1xx1xx", "not 1111") | ("00", "1100xx", "not 1111") => {
                    // A7.7.43, T4
                    instr_bind!("11111|00|0|0|10|1|<rn:4>|<rt:4>|1|<p:1>|<u:1>|<w:1>|<imm8:8>" identifiers (rn, rt, p, u, w, imm8));
                    if rn == bsc::C_1111 {
                        unreachable!("SEE LDR (literal)")
                    } else if p == bsc::C_1 && u == bsc::C_1 && w == bsc::C_0 {
                        unreachable!("SEE LDRT")
                    } else if rn == bsc::C_1101
                        && p == bsc::C_0
                        && u == bsc::C_1
                        && w == bsc::C_1
                        && imm8 == bsc::C_0000_0100
                    {
                        // SEE POP
                        // A7.7.99, T3

                        // Note: `pop.w { rt }` seems to be a shortcut notation for `ldr rt, [sp], 4`.
                        //       (see: `cmemu-lib/src/component/core/instruction.rs` ctrl+f "A7.7.99")
                        //
                        //       To be more consistent with docs, we leave their decoding stages
                        //       separate although after constant propagation they do the same operations.
                        //       (note that: `rn == bsc::C_1101 && w == bsc::C_1` (from `if` condition)
                        //       and `(wback && rn == rt) <=> rt == 13` when `rn == 13 && wback == true`)
                        rebind_as_u32! {rt;}
                        if rt == 13 || (rt == 15 && xpsr.in_it_block() && !xpsr.last_in_it_block())
                        {
                            Instruction::Unpredictable
                        } else {
                            Instruction::LoadRegister_Immediate {
                                rn: RegisterID::SP,
                                rt: RegisterID::from_index(rt),
                                imm32: bsc::C_0000_0100.zero_extend(),

                                index: false,
                                add: true,
                                wback: true,
                            }
                        }
                    } else if p == bsc::C_0 && w == bsc::C_0 {
                        Instruction::Undefined
                    } else {
                        rebind_as_u32! {rt; rn;}
                        let index = p == bsc::C_1;
                        let add = u == bsc::C_1;
                        let wback = w == bsc::C_1;

                        if (wback && rn == rt)
                            || (rt == 15 && xpsr.in_it_block() && !xpsr.last_in_it_block())
                        {
                            Instruction::Unpredictable
                        } else {
                            Instruction::LoadRegister_Immediate {
                                rt: RegisterID::from_index(rt),
                                rn: RegisterID::from_index(rn),
                                imm32: imm8.zero_extend(),

                                index,
                                add,
                                wback,
                            }
                        }
                    }
                }
                ("00", "1110xx", "not 1111") => {
                    // A7.7.67, T1
                    instr_bind!("11111|00|0|0|10|1|<rn:4>|<rt:4>|1|110|<imm8:8>" identifiers (rn, rt, imm8));
                    if rn == bsc::C_1111 {
                        unreachable!("SEE LDR (literal)")
                    } else {
                        rebind_as_u32! {rt; rn;}
                        if rt == 13 || rt == 15 {
                            Instruction::Unpredictable
                        } else {
                            Instruction::LoadRegisterUnprivileged {
                                rt: RegisterID::from_index(rt),
                                rn: RegisterID::from_index(rn),
                                imm32: imm8.zero_extend(),
                                // Regarding fields, see Note on variant definition.
                            }
                        }
                    }
                }
                ("00", "000000", "not 1111") => {
                    // A7.7.45, T2
                    instr_bind!("11111|00|0|0|10|1|<rn:4>|<rt:4>|0|00000|<imm2:2>|<rm:4>" identifiers (rt, rn, rm, imm2));
                    if rn == bsc::C_1111 {
                        unreachable!("SEE LDR (literal)")
                    } else {
                        rebind_as_u32! {rt; rn; rm;}
                        if rm == 13
                            || rm == 15
                            || (rt == 15 && xpsr.in_it_block() && !xpsr.last_in_it_block())
                        {
                            Instruction::Unpredictable
                        } else {
                            Instruction::LoadRegister_Register {
                                rt: RegisterID::from_index(rt),
                                rn: RegisterID::from_index(rn),
                                rm: RegisterID::from_index(rm),
                                shift: Shift::LSL(imm2.into()),

                                index: true,
                                add: true,
                                wback: false,
                            }
                        }
                    }
                }
                ("0x", "xxxxxx", "1111") => {
                    // A7.7.44, T2
                    instr_bind!("11111|00|0|<u:1>|10|1|1111|<rt:4>|<imm12:12>" identifiers (u, rt, imm12));
                    rebind_as_u32! {rt;}
                    if rt == 15 && xpsr.in_it_block() && !xpsr.last_in_it_block() {
                        Instruction::Unpredictable
                    } else {
                        Instruction::LoadRegister_Literal {
                            rt: RegisterID::from_index(rt),
                            imm32: imm12.zero_extend(),
                            add: u == bsc::C_1,
                        }
                    }
                }
                _ => Instruction::Undefined,
            }
        }
        ("11", "00xx111", "x") => Instruction::Undefined,
        ("11", "010xxxx", "x") => {
            // A5.3.12
            match instr!("111|1101|0|<op1:4>|<rn:4>|1111|xxxx|<op2:4>|xxxx" in order(op1, op2, rn))
            {
                ("000x", "0000", "xxxx") => {
                    // A7.7.69, T2
                    instr_bind!("11111|010|0|00|<s:1>|<rn:4>|1111|<rd:4>|0|000|<rm:4>" identifiers(rd, rn, rm, s));
                    rebind_as_u32! {rd; rn; rm;}

                    if rd == 13 || rd == 15 || rn == 13 || rn == 15 || rm == 13 || rm == 15 {
                        Instruction::Unpredictable
                    } else {
                        Instruction::LogicalShiftLeft_Register {
                            rd: RegisterID::from_index(rd),
                            rn: RegisterID::from_index(rn),
                            rm: RegisterID::from_index(rm),
                            setflags: s == bsc::C_1,
                            setflags_depends_on_it: false,
                        }
                    }
                }
                ("001x", "0000", "xxxx") => {
                    // A7.7.71, T2
                    instr_bind!("11111|010|0|01|<s:1>|<rn:4>|1111|<rd:4>|0|000|<rm:4>" identifiers (rd, rn, rm, s));
                    rebind_as_u32! {rd; rn; rm;}

                    if rd == 13 || rd == 15 || rn == 13 || rn == 15 || rm == 13 || rm == 15 {
                        Instruction::Unpredictable
                    } else {
                        Instruction::LogicalShiftRight_Register {
                            rd: RegisterID::from_index(rd),
                            rn: RegisterID::from_index(rn),
                            rm: RegisterID::from_index(rm),
                            setflags: s == bsc::C_1,
                            setflags_depends_on_it: false,
                        }
                    }
                }
                ("010x", "0000", "xxxx") => {
                    // A7.7.11, T2
                    instr_bind!("11111|010|0|10|<s:1>|<rn:4>|1111|<rd:4>|0|000|<rm:4>" identifiers (s, rn, rd, rm));
                    rebind_as_u32! {rd; rn; rm;}

                    if rd == 13 || rd == 15 || rn == 13 || rn == 15 || rm == 13 || rm == 15 {
                        Instruction::Unpredictable
                    } else {
                        Instruction::ArithmeticShiftRight_Register {
                            rd: RegisterID::from_index(rd),
                            rn: RegisterID::from_index(rn),
                            rm: RegisterID::from_index(rm),
                            setflags: s == bsc::C_1,
                            setflags_depends_on_it: false,
                        }
                    }
                }
                ("011x", "0000", "xxxx") => {
                    // A7.7.117, T2
                    instr_bind!("11111|010|0|11|<s:1>|<rn:4>|1111|<rd:4>|0|000|<rm:4>" identifiers (s, rn, rd, rm));
                    rebind_as_u32! {rd; rn; rm;}

                    if rd == 13 || rd == 15 || rn == 13 || rn == 15 || rm == 13 || rm == 15 {
                        Instruction::Unpredictable
                    } else {
                        Instruction::RotateRight_Register {
                            rd: RegisterID::from_index(rd),
                            rn: RegisterID::from_index(rn),
                            rm: RegisterID::from_index(rm),
                            setflags: s == bsc::C_1,
                            setflags_depends_on_it: false,
                        }
                    }
                }
                ("0000", "1xxx", "1111") => {
                    // A7.7.184, T2
                    if instr_bind!("11111|010|0|000|1111|1111|<rd:4>|1|(0)|<rotate:2>|<rm:4>" identifiers (rd, rotate, rm))
                    {
                        rebind_as_u32! {rd; rm;}
                        if rd == 13 || rd == 15 || rm == 13 || rm == 15 {
                            Instruction::Unpredictable
                        } else {
                            Instruction::SignedExtendHalfword {
                                rd: RegisterID::from_index(rd),
                                rm: RegisterID::from_index(rm),
                                rotation: Shift {
                                    srtype: SRType::ROR,
                                    amount: bitstring_concat!(rotate : bsc::C_000 | 5 bits).into(),
                                },
                            }
                        }
                    }
                }
                ("0001", "1xxx", "1111") => {
                    // A7.7.223, T2
                    if instr_bind!("11111|010|0|001|1111|1111|<rd:4>|1|(0)|<rotate:2>|<rm:4>" identifiers (rd, rotate, rm))
                    {
                        rebind_as_u32! {rd; rm;}
                        if rd == 13 || rd == 15 || rm == 13 || rm == 15 {
                            Instruction::Unpredictable
                        } else {
                            Instruction::UnsignedExtendHalfword {
                                rd: RegisterID::from_index(rd),
                                rm: RegisterID::from_index(rm),
                                rotation: Shift {
                                    srtype: SRType::ROR,
                                    amount: bitstring_concat!(rotate : bsc::C_000 | 5 bits).into(),
                                },
                            }
                        }
                    }
                }
                ("0100", "1xxx", "1111") => {
                    // A7.7.182, T2
                    if instr_bind!("11111|010|0|100|1111|1111|<rd:4>|1|(0)|<rotate:2>|<rm:4>" identifiers (rd, rotate, rm))
                    {
                        rebind_as_u32! {rd; rm;}
                        if rd == 13 || rd == 15 || rm == 13 || rm == 15 {
                            Instruction::Unpredictable
                        } else {
                            Instruction::SignedExtendByte {
                                rd: RegisterID::from_index(rd),
                                rm: RegisterID::from_index(rm),
                                rotation: Shift {
                                    srtype: SRType::ROR,
                                    amount: bitstring_concat!(rotate : bsc::C_000 | 5 bits).into(),
                                },
                            }
                        }
                    }
                }
                ("0101", "1xxx", "1111") => {
                    // A7.7.221, T2
                    if instr_bind!("11111|010|0|101|1111|1111|<rd:4>|1|(0)|<rotate:2>|<rm:4>" identifiers (rd, rotate, rm))
                    {
                        rebind_as_u32! {rd; rm;}
                        if rd == 13 || rd == 15 || rm == 13 || rm == 15 {
                            Instruction::Unpredictable
                        } else {
                            Instruction::UnsignedExtendByte {
                                rd: RegisterID::from_index(rd),
                                rm: RegisterID::from_index(rm),
                                rotation: Shift {
                                    srtype: SRType::ROR,
                                    amount: bitstring_concat!(rotate : bsc::C_000 | 5 bits).into(),
                                },
                            }
                        }
                    }
                }
                ("10xx", "10xx", "xxxx") => {
                    // A5.3.15
                    match instr!("111|1101|010|<op1:2>|xxxx|1111|xxxx|10|<op2:2>|xxxx" in order (op1, op2))
                    {
                        ("01", "00") => {
                            // A7.7.113, T2
                            instr_bind!("11111|010|1|001|<rm:4>|1111|<rd:4>|1|000|<rm2:4>" identifiers (rd, rm, rm2));
                            #[allow(clippy::if_not_else)] // for better compatibility with docs
                            if !consistent(rm, rm2) {
                                Instruction::Unpredictable
                            } else {
                                rebind_as_u32! {rd; rm;}
                                if rd == 13 || rd == 15 || rm == 13 || rm == 15 {
                                    Instruction::Unpredictable
                                } else {
                                    Instruction::ByteReverseWord {
                                        rd: RegisterID::from_index(rd),
                                        rm: RegisterID::from_index(rm),
                                    }
                                }
                            }
                        }
                        ("01", "01") => {
                            // A7.7.114, T2
                            instr_bind!("11111|010|1|001|<rm:4>|1111|<rd:4>|1|001|<rm2:4>" identifiers (rd, rm, rm2));
                            #[allow(clippy::if_not_else)] // for better compatibility with docs
                            if !consistent(rm, rm2) {
                                Instruction::Unpredictable
                            } else {
                                rebind_as_u32! {rd; rm;}
                                if rd == 13 || rd == 15 || rm == 13 || rm == 15 {
                                    Instruction::Unpredictable
                                } else {
                                    Instruction::ByteReversePackedHalfword {
                                        rd: RegisterID::from_index(rd),
                                        rm: RegisterID::from_index(rm),
                                    }
                                }
                            }
                        }
                        ("01", "10") => {
                            // A7.7.112, T1
                            instr_bind!("11111|010|1|001|<rm:4>|1111|<rd:4>|1|010|<rm2:4>" identifiers (rd, rm, rm2));
                            #[allow(clippy::if_not_else)] // for better compatibility with docs
                            if !consistent(rm, rm2) {
                                Instruction::Unpredictable
                            } else {
                                rebind_as_u32! {rd; rm;}
                                if rd == 13 || rd == 15 || rm == 13 || rm == 15 {
                                    Instruction::Unpredictable
                                } else {
                                    Instruction::ReverseBits {
                                        rd: RegisterID::from_index(rd),
                                        rm: RegisterID::from_index(rm),
                                    }
                                }
                            }
                        }
                        ("01", "11") => {
                            // A7.7.115, T2
                            instr_bind!("11111|010|1|001|<rm:4>|1111|<rd:4>|1|011|<rm2:4>" identifiers (rd, rm, rm2));
                            #[allow(clippy::if_not_else)] // for better compatibility with docs
                            if !consistent(rm, rm2) {
                                Instruction::Unpredictable
                            } else {
                                rebind_as_u32! {rd; rm;}
                                if rd == 13 || rd == 15 || rm == 13 || rm == 15 {
                                    Instruction::Unpredictable
                                } else {
                                    Instruction::ByteReverseSignedHalfword {
                                        rd: RegisterID::from_index(rd),
                                        rm: RegisterID::from_index(rm),
                                    }
                                }
                            }
                        }
                        ("11", "00") => {
                            // A7.7.24, T1
                            instr_bind!("11111|010|1|011|<rm:4>|1111|<rd:4>|1|000|<rm2:4>" identifiers (rd, rm, rm2));
                            #[allow(clippy::if_not_else)]
                            if !consistent(rm, rm2) {
                                Instruction::Unpredictable
                            } else {
                                rebind_as_u32! {rd; rm;}
                                if rd == 13 || rd == 15 || rm == 13 || rm == 15 {
                                    Instruction::Unpredictable
                                } else {
                                    Instruction::CountLeadingZeros {
                                        rd: RegisterID::from_index(rd),
                                        rm: RegisterID::from_index(rm),
                                    }
                                }
                            }
                        }
                        // Merge of:
                        // 1) CherryMote doesn't include DSP extension (v7E-M; [ARM-ARM] A1.3),
                        //    so this is UNDEFINED ([ARM-ARM] A5.1.1).
                        // 2) "bits[15:12] != 0b1111". The bits won't match any arm,
                        //     since they are checked for being "1111" in `match instr!(<here>)`.
                        // 3) Regular "other encodings".
                        _ => Instruction::Undefined,
                    }
                }
                // Merge of:
                // 1) CherryMote doesn't include DSP extension (v7E-M; [ARM-ARM] A1.3),
                //    so this is UNDEFINED ([ARM-ARM] A5.1.1).
                //    Includes "Parallel addition and subtraction".
                // 2) "bits[15:12] != 0b1111". The bits won't match any arm,
                //     since they are checked for being "1111" in `match instr!(<here>)`.
                // 3) Regular "other encodings".
                _ => Instruction::Undefined,
            }
        }
        ("11", "0110xxx", "x") => {
            // A5.3.16
            match instr!("111|1101|10|<op1:3>|xxxx|<ra:4>|xxxx|00|<op2:2>|xxxx" in order (op1, op2, ra))
            {
                ("000", "00", "not 1111") => {
                    // A7.7.74, T1
                    instr_bind!("11111|0110|000|<rn:4>|<ra:4>|<rd:4>|0000|<rm:4>" identifiers (rd, rn, rm, ra));
                    if ra == bsc::C_1111 {
                        unreachable!("SEE MUL");
                    }

                    rebind_as_u32! {rd; rn; rm; ra;}
                    if rd == 13
                        || rd == 15
                        || rn == 13
                        || rn == 15
                        || rm == 13
                        || rm == 15
                        || ra == 13
                    {
                        Instruction::Unpredictable
                    } else {
                        Instruction::MultiplyAccumulate {
                            rd: RegisterID::from_index(rd),
                            rn: RegisterID::from_index(rn),
                            rm: RegisterID::from_index(rm),
                            ra: RegisterID::from_index(ra),
                            setflags: false,
                        }
                    }
                }
                ("000", "00", "1111") => {
                    // A7.7.84, T2
                    instr_bind!("11111|0110|000|<rn:4>|1111|<rd:4>|0000|<rm:4>" identifiers (rd, rn, rm));
                    rebind_as_u32! {rd; rn; rm;}
                    if rd == 13 || rd == 15 || rn == 13 || rn == 15 || rm == 13 || rm == 15 {
                        Instruction::Unpredictable
                    } else {
                        Instruction::Multiply {
                            rd: RegisterID::from_index(rd),
                            rn: RegisterID::from_index(rn),
                            rm: RegisterID::from_index(rm),
                            setflags: false,
                            setflags_depends_on_it: false,
                        }
                    }
                }
                ("000", "01", "xxxx") => {
                    // A7.7.75, T1
                    instr_bind!("11111|0110|000|<rn:4>|<ra:4>|<rd:4>|0001|<rm:4>" identifiers (rd, rn, rm, ra));
                    rebind_as_u32! {rd; rn; rm; ra;}

                    if rd == 13
                        || rd == 15
                        || rn == 13
                        || rn == 15
                        || rm == 13
                        || rm == 15
                        || ra == 13
                        || ra == 15
                    {
                        Instruction::Unpredictable
                    } else {
                        Instruction::MultiplyAndSubtract {
                            rd: RegisterID::from_index(rd),
                            rn: RegisterID::from_index(rn),
                            rm: RegisterID::from_index(rm),
                            ra: RegisterID::from_index(ra),
                        }
                    }
                }
                // Merge of:
                // 1) CherryMote doesn't include DSP extension (v7E-M; [ARM-ARM] A1.3),
                //    so this is UNDEFINED ([ARM-ARM] A5.1.1).
                // 2) "bits[7:6] != 0b00". The bits won't match any arm,
                //     since they are checked for being "00" in `match instr!(<here>)`.
                // 3) Regular "other encodings".
                _ => Instruction::Undefined,
            }
        }
        ("11", "0111xxx", "x") => {
            // A5.3.17
            match instr!("111|1101|11|<op1:3>|xxxx|xxxxxxxx|<op2:4>|xxxx" in order (op1, op2)) {
                ("000", "0000") => {
                    // A7.7.149, T1
                    instr_bind!("11111|0111|0|00|<rn:4>|<rd_lo:4>|<rd_hi:4>|0000|<rm:4>" identifiers (rd_lo, rd_hi, rn, rm));
                    rebind_as_u32! {rd_lo; rd_hi; rn; rm;}
                    if rd_lo == 13
                        || rd_lo == 15
                        || rd_hi == 13
                        || rd_hi == 15
                        || rn == 13
                        || rn == 15
                        || rm == 13
                        || rm == 15
                        || rd_lo == rd_hi
                    {
                        Instruction::Unpredictable
                    } else {
                        Instruction::SignedMultiplyLong {
                            rn: RegisterID::from_index(rn),
                            rm: RegisterID::from_index(rm),
                            rd_hi: RegisterID::from_index(rd_hi),
                            rd_lo: RegisterID::from_index(rd_lo),
                        }
                    }
                }
                ("001", "1111") => {
                    // A7.7.127, T1
                    if instr_bind!("11111|011100|1|<rn:4>|(1)(1)(1)(1)|<rd:4>|1111|<rm:4>" identifiers (rn, rd, rm))
                    {
                        rebind_as_u32! {rd; rn; rm;}
                        if rd == 13 || rd == 15 || rn == 13 || rn == 15 || rm == 13 || rm == 15 {
                            Instruction::Unpredictable
                        } else {
                            Instruction::SignedDivide {
                                rd: RegisterID::from_index(rd),
                                rn: RegisterID::from_index(rn),
                                rm: RegisterID::from_index(rm),
                            }
                        }
                    }
                }
                ("010", "0000") => {
                    // A7.7.204, T1
                    instr_bind!("11111|0111|010|<rn:4>|<rd_lo:4>|<rd_hi:4>|0000|<rm:4>" identifiers (rd_lo, rd_hi, rn, rm));
                    rebind_as_u32! {rd_lo; rd_hi; rn; rm;}
                    if rd_lo == 13
                        || rd_lo == 15
                        || rd_hi == 13
                        || rd_hi == 15
                        || rn == 13
                        || rn == 15
                        || rm == 13
                        || rm == 15
                        || rd_lo == rd_hi
                    {
                        Instruction::Unpredictable
                    } else {
                        Instruction::UnsignedMultiplyLong {
                            rn: RegisterID::from_index(rn),
                            rm: RegisterID::from_index(rm),
                            rd_hi: RegisterID::from_index(rd_hi),
                            rd_lo: RegisterID::from_index(rd_lo),
                        }
                    }
                }
                ("011", "1111") => {
                    // A7.7.195, T1
                    if instr_bind!("11111|011101|1|<rn:4>|(1)(1)(1)(1)|<rd:4>|1111|<rm:4>" identifiers (rn, rd, rm))
                    {
                        rebind_as_u32! {rd; rn; rm;}
                        if rd == 13 || rd == 15 || rn == 13 || rn == 15 || rm == 13 || rm == 15 {
                            Instruction::Unpredictable
                        } else {
                            Instruction::UnsignedDivide {
                                rd: RegisterID::from_index(rd),
                                rn: RegisterID::from_index(rn),
                                rm: RegisterID::from_index(rm),
                            }
                        }
                    }
                }
                ("100", "0000") => {
                    // A7.7.138, T1
                    instr_bind!("11111|0111|1|00|<rn:4>|<rd_lo:4>|<rd_hi:4>|0000|<rm:4>" identifiers (rd_lo, rd_hi, rn, rm));
                    rebind_as_u32! {rd_lo; rd_hi; rn; rm;}
                    if rd_lo == 13
                        || rd_lo == 15
                        || rd_hi == 13
                        || rd_hi == 15
                        || rn == 13
                        || rn == 15
                        || rm == 13
                        || rm == 15
                        || rd_lo == rd_hi
                    {
                        Instruction::Unpredictable
                    } else {
                        Instruction::SignedMultiplyAccumulateLong {
                            rn: RegisterID::from_index(rn),
                            rm: RegisterID::from_index(rm),
                            rd_hi: RegisterID::from_index(rd_hi),
                            rd_lo: RegisterID::from_index(rd_lo),
                        }
                    }
                }
                ("110", "0000") => {
                    // A7.7.203, T1
                    instr_bind!("11111|0111|110|<rn:4>|<rd_lo:4>|<rd_hi:4>|0000|<rm:4>" identifiers (rd_lo, rd_hi, rn, rm));
                    rebind_as_u32! {rd_lo; rd_hi; rn; rm;}
                    if rd_lo == 13
                        || rd_lo == 15
                        || rd_hi == 13
                        || rd_hi == 15
                        || rn == 13
                        || rn == 15
                        || rm == 13
                        || rm == 15
                        || rd_lo == rd_hi
                    {
                        Instruction::Unpredictable
                    } else {
                        Instruction::UnsignedMultiplyAccumulateLong {
                            rn: RegisterID::from_index(rn),
                            rm: RegisterID::from_index(rm),
                            rd_hi: RegisterID::from_index(rd_hi),
                            rd_lo: RegisterID::from_index(rd_lo),
                        }
                    }
                }
                // Merge of:
                // 1) CherryMote doesn't include DSP extension (v7E-M; [ARM-ARM] A1.3),
                //    so this is UNDEFINED ([ARM-ARM] A5.1.1).
                // 2) Regular "other encodings".
                _ => Instruction::Undefined,
            }
        }
        // Extra assertion (see A5.1 and A5.3):
        ("00", "xxxxxxx", "x") => {
            panic!("Invalid 32-bit instruction: first halfword encodes 16-bit instruction.")
        }
        // Includes: ("11", "001xxx0", "x")
        _ => Instruction::Unsupported {
            name: "Not covered by the docs.",
        },
    }
}

// ----------------------------------------------------------------------------
// Helpers
// ----------------------------------------------------------------------------

/// [ARM-ARM] A5.3.2
fn thumb_expand_imm(imm12: Bitstring![12], xpsr: XPSR) -> Result<Word, Instruction> {
    thumb_expand_imm_c(imm12, xpsr.carry_flag()).map(|v| v.0)
}

/// [ARM-ARM] A5.3.2
fn thumb_expand_imm_c(imm12: Bitstring![12], carry_in: bool) -> Result<(Word, bool), Instruction> {
    if bitstring_extract!(imm12<11:10> | 2 bits) == bsc::C_00 {
        // For better consistency with docs and readability.
        #[allow(clippy::needless_late_init)]
        let imm32;
        let imm12_7_0 = bitstring_extract!(imm12<7:0> | 8 bits);
        match bitstring_extract!(imm12<9:8> | 2 bits) {
            bsc::C_00 => imm32 = imm12_7_0.zero_extend(),
            bsc::C_01 => {
                if imm12_7_0 == bsc::C_0000_0000 {
                    return Err(Instruction::Unpredictable);
                }
                imm32 = bitstring_concat!(bsc::C_0000_0000 : imm12_7_0 : bsc::C_0000_0000 : imm12_7_0 | 32 bits);
            }
            bsc::C_10 => {
                if imm12_7_0 == bsc::C_0000_0000 {
                    return Err(Instruction::Unpredictable);
                }
                imm32 = bitstring_concat!(imm12_7_0 : bsc::C_0000_0000 : imm12_7_0 : bsc::C_0000_0000 | 32 bits);
            }
            bsc::C_11 => {
                if imm12_7_0 == bsc::C_0000_0000 {
                    return Err(Instruction::Unpredictable);
                }
                imm32 = bitstring_concat!(imm12_7_0 : imm12_7_0 : imm12_7_0 : imm12_7_0 | 32 bits);
            }
            _ => unreachable!(),
        }
        Ok((imm32, carry_in))
    } else {
        let imm12_6_0 = bitstring_extract!(imm12<6:0> | 7 bits);
        let unrotated_value: Word = bitstring_concat!(bsc::C_1 : imm12_6_0 | 8 bits).zero_extend();
        Ok(unrotated_value.ror_c(u32::from(bitstring_extract!(imm12<11:7> | 5 bits))))
    }
}

/// [ARM-ARM] D6.1, D7.2
/// For better compatibility with docs
fn consistent<T>(lhs: T, rhs: T) -> bool
where
    T: PartialEq + Copy,
{
    lhs == rhs
}
