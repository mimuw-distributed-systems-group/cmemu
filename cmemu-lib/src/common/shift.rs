use crate::Bitstring;
use crate::common::bitstring::constants as bsc;
use std::fmt;

/// Represents types of shift done by ARM Cortex-M3
///
/// See: [ARM-ARM] A7.4.2 for their meaning
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SRType {
    LSL,
    LSR,
    ASR,
    RRX,
    ROR,
}

impl fmt::Display for SRType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::LSL => "LSL",
                Self::LSR => "LSR",
                Self::ASR => "ASR",
                Self::RRX => "RRX",
                Self::ROR => "ROR",
            }
        )
    }
}

/// Represents pair `(shift_t, shift_n)` from [ARM-ARM].
#[derive(Clone, Copy, Debug)]
pub(crate) struct Shift {
    pub srtype: SRType,
    pub amount: u8,
}

impl Shift {
    /// Creates `LSL` shift by amount `n`.
    ///
    /// # Panics
    /// When `n` is invalid.
    #[allow(non_snake_case)]
    pub fn LSL(n: u8) -> Self {
        assert!(n < 32);
        Self {
            srtype: SRType::LSL,
            amount: n,
        }
    }

    pub(crate) fn decode_imm_shift(ty: Bitstring![2], imm: Bitstring![5]) -> Self {
        let imm: u8 = imm.into();
        match ty {
            bsc::C_00 => Self {
                srtype: SRType::LSL,
                amount: imm,
            },
            bsc::C_01 => Self {
                srtype: SRType::LSR,
                amount: if imm == 0 { 32 } else { imm },
            },
            bsc::C_10 => Self {
                srtype: SRType::ASR,
                amount: if imm == 0 { 32 } else { imm },
            },
            bsc::C_11 => {
                if imm == 0 {
                    Self {
                        srtype: SRType::RRX,
                        amount: 1,
                    }
                } else {
                    Self {
                        srtype: SRType::ROR,
                        amount: imm,
                    }
                }
            }
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for Shift {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} #{}", self.srtype, self.amount)
    }
}
