use crate::Bitstring;
use crate::common::BitstringUtils;
use crate::common::{Address, SRType, Shift};
use std::fmt;

// ***************
// Note:
// `Word` is just `Bitstring![32]`. Simple operations implement on `Bitstring`.
// `Word`-specific operations and complicated operations (complicated impl in generic case)
// define here, on `Word`.
// ***************

/// `Word` is just `Bitstring![32]`.
/// It represents 32-bit data type defined in [ARM-ARM] A2.2
///
/// It implements functions on 32-bit bitstrings from [ARM-ARM]
/// that are evaluated in ARM Cortex-M3 microprocessor.
pub type Word = Bitstring![32];

impl Word {
    #[must_use]
    fn with_u32<F>(self, f: F) -> Self
    where
        F: FnOnce(u32) -> u32,
    {
        f(self.into()).into()
    }
}

// ----------------------------------------------------------------------------
// [ARM-ARM] A2.2.1  ARM processor data types and... :: Integer arithmetic
// ----------------------------------------------------------------------------

impl Word {
    /// [ARM-ARM] A2.2.1
    /// Evaluates self + rhs + carry
    ///
    /// Returns (result, carry, overflow)
    // Note: used only in 32-bit contexts, so there's no need to generalize it.
    #[inline]
    #[must_use]
    pub fn add_with_carry(self, rhs: Word, carry: bool) -> (Word, bool, bool) {
        let ux: u64 = u32::from(self).into();
        let uy: u64 = u32::from(rhs).into();
        let uc: u64 = u64::from(carry);

        let sx: i64 = i32::from(self).into();
        let sy: i64 = i32::from(rhs).into();
        let sc: i64 = i64::from(carry);

        let u_sum: u64 = ux + uy + uc;
        let s_sum: i64 = sx + sy + sc;

        #[allow(clippy::cast_possible_truncation, reason = "We want to trunc")]
        let result: u32 = (u_sum & 0xFFFF_FFFF) as u32;
        let u_res: u64 = result.into();
        let s_res: i64 = result.cast_signed().into();
        let carry_out = u_res != u_sum;
        let overflow_out = s_res != s_sum;
        (result.into(), carry_out, overflow_out)
    }

    /// [ARM-ARM] A2.2.1
    /// Combines `unsigned_sat_q` and `zero_extend`
    ///
    /// Saturates self to n bits (n < 32)
    /// Returns (result, saturated)
    ///
    /// # Panics
    /// When n >= 32
    // Note: the only usage of `unsigned_sat_q` among the instructions supported by CM
    // is followed by `zero_extend`, so we just combine them together to simplify implementation.
    // Also, UnsignedSatQ has *signed integers* as input.
    #[inline]
    #[must_use]
    pub fn unsigned_sat_q_with_zero_extend(self, n: u8) -> (Self, bool) {
        let i = i32::from(self);
        assert!(n < 32);

        let n_max = 1_u32.unbounded_shl(u32::from(n)) - 1;
        if i > n_max.cast_signed() {
            (n_max.into(), true)
        } else if i < 0 {
            (0.into(), true)
        } else {
            (self, false)
        }
    }

    /// [ARM-ARM] A2.2.1
    /// Combines `signed_sat_q` and `zero_extend`
    ///
    /// Saturates self to n bits (1 < n <= 32)
    /// Returns (result, saturated)
    ///
    /// # Panics
    /// When n >= 32
    // Note: the only usage of `signed_sat_q` among the instructions supported by CM
    // is followed by `zero_extend`, so we just combine them together to simplify implementation.
    #[must_use]
    pub fn signed_sat_q_with_zero_extend(self, n: u8) -> (Self, bool) {
        let i = i32::from(self);
        assert!(n <= 32 && n > 0);

        let n_max = 1_u32.unbounded_shl(u32::from(n - 1)) - 1;
        let n_min = (-1_i32).unbounded_shl(u32::from(n - 1));
        if i > n_max.cast_signed() {
            (n_max.into(), true)
        } else if i < n_min {
            (n_min.into(), true)
        } else {
            (self, false)
        }
    }
}

// ----------------------------------------------------------------------------
// [ARM-ARM] A7.4.2  Shifts applied to register :: Shift operations
// ----------------------------------------------------------------------------

impl Word {
    /// [ARM-ARM] A7.4.2
    // Note: used only in 32-bit contexts, so there's no need to generalize it.
    #[inline]
    #[must_use]
    pub(crate) fn shift(self, shift: Shift, carry_in: bool) -> Word {
        self.shift_c(shift, carry_in).0
    }

    /// [ARM-ARM] A7.4.2
    ///
    /// # Panics
    /// When the invariant from the documentation is not uphold.
    // Note: used only in 32-bit contexts, so there's no need to generalize it.
    #[inline]
    #[must_use]
    pub(crate) fn shift_c(self, shift: Shift, carry_in: bool) -> (Word, bool) {
        assert!(!(shift.srtype == SRType::RRX && shift.amount != 1));

        if shift.amount == 0 {
            return (self, carry_in);
        }
        let n = shift.amount;
        match shift.srtype {
            SRType::LSL => self.lsl_c(n.into()),
            SRType::LSR => self.lsr_c(n.into()),
            SRType::ASR => self.asr_c(n.into()),
            SRType::ROR => self.ror_c(n.into()),
            SRType::RRX => self.rrx_c(carry_in),
        }
    }
}

// ----------------------------------------------------------------------------
// [ARM-ARM] D6.5.3  Bitstring manipulation
// ----------------------------------------------------------------------------

impl Word {
    /// [ARM-ARM] D6.5.3
    /// Rounds down value by setting the least significant bits to zeroes.
    /// Thus `y` must be a power of two.
    ///
    /// # Panics
    /// When `y` is not a power of two.
    // Note: used only in 32-bit contexts, so there's no need to generalize it.
    #[inline]
    #[must_use]
    pub fn align(self, y: u32) -> Self {
        assert!(y.is_power_of_two());

        self.with_u32(|v| v & !(y - 1))
    }

    /// [ARM-ARM] D6.5.3 Converting bitstrings to integers
    /// If x is a bitstring, SInt(x) is the integer whose 2's complement representation is x:
    #[must_use]
    pub fn sint(self) -> i32 {
        self.into()
    }

    /// [ARM-ARM] D6.5.3 Converting bitstrings to integers
    /// UInt(x) is the integer whose unsigned representation is x:
    #[must_use]
    pub fn uint(self) -> u32 {
        self.into()
    }
}

impl std::ops::BitOr<u32> for Word {
    type Output = Self;
    /// [ARM-ARM] D6.5.3
    fn bitor(self, rhs: u32) -> Self {
        self | Self::from(rhs)
    }
}

// ----------------------------------------------------------------------------
// [ARM-ARM] D6.5.4  Operators and built-in functions :: Arithmetic
// ----------------------------------------------------------------------------

impl std::ops::Add<u32> for Word {
    type Output = Self;
    /// [ARM-ARM] D6.5.4
    fn add(self, rhs: u32) -> Self {
        self + Word::from(rhs)
    }
}

impl std::ops::Sub<u32> for Word {
    type Output = Self;
    /// [ARM-ARM] D6.5.4
    fn sub(self, rhs: u32) -> Self {
        self - Word::from(rhs)
    }
}

// ----------------------------------------------------------------------------
// Display and conversion to primitive types
// ----------------------------------------------------------------------------

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <u32 as fmt::LowerHex>::fmt(&self.0, f)
    }
}

impl fmt::LowerHex for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <u32 as fmt::LowerHex>::fmt(&self.0, f)
    }
}

impl From<i32> for Word {
    #[allow(clippy::cast_sign_loss)]
    fn from(val: i32) -> Self {
        Word::from(val.cast_unsigned())
    }
}

impl From<Address> for Word {
    fn from(address: Address) -> Self {
        <Word as From<u32>>::from(address.into())
    }
}

impl From<Word> for i32 {
    #[allow(clippy::cast_possible_wrap)]
    fn from(val: Word) -> Self {
        val.0.cast_signed()
    }
}

impl From<Word> for Address {
    fn from(val: Word) -> Self {
        Self::from_const(val.into())
    }
}
