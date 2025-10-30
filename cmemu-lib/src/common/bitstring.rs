//! Module containing `Bitstring` datatype and operations on it.
//!
//! [ARM-ARM] A2.2.1 -- Integer arithmetic
//! [ARM-ARM] D6.5 -- Operators and built-in functions

use std::fmt;
use std::fmt::{Binary, Debug, Formatter};
use std::marker::PhantomData;
use std::mem::size_of;
use std::ops::{Add, BitAnd, BitOr, BitXor, Not, Sub};

pub mod bitfield;
mod word;

pub use word::Word;

pub const MAX_BITS: usize = 32;

// ============================================================================
// Type definition
// ============================================================================

/// Represents data type defined in [ARM-ARM] A2.2.
///
/// Valid data types are `Bitstring![1]` through `Bitstring![32]` (aka `Word`).
/// Part of operations is available through `BitstringUtils` trait
/// and some are macros (due to implementation details).
///
/// `N`: bits representation: u8, u16 or u32
/// `T`: marker type "[bool; N]"
/// `PartialEq`, `Eq`, `PartialOrd`, `Ord`: [ARM-ARM] D6.5.4 Comparisons
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Bitstring<N, T>(N, PhantomData<T>);

// Note(asserts):
//     normally we use `debug_assert`s, however there's an exception:
//     asserts on size are optimized away in all* of our use cases
//     (passing a constant via inlined [static] call),
//     and in other future cases (if they will exist), we want an assertion to crash,
//     since it's a programmer error, not a runtime one; the runtime cost
//     shouldn't be high assuming the other calls are rare, but in case
//     they are, we can change the `assert`s back to `debug_assert`s
//     \* valid in the time of writing this comment

// Note(clippy-workaround):
//     Clippy seems bugged, so it needs larger scope for `#[allow()]`,
//     and rustc 1.47 is bugged too, so this workaround can't be enclosed in a macro
//     (https://github.com/rust-lang/rust/issues/78892).
//     Lambda call should be optimized away, anyway.

// Note(clippy-repetitions):
//     As of April 2022 Clippy nightly complains about bitstring_ macros' names
//     for containing parent module name in their names.
//     Those macros are exported from the crate and it would be awkward to have
//     a macro exported that's called just concat.
//     Funnily enough it only complains about 4 macros and not the others.

// ============================================================================
// Basic operations (regular methods)
// ============================================================================

// helper trait
// doc-strings can be found below at the definition
// allow: for readability (mostly as trait bound, also for "use crate::common::BitstringUtils")
#[allow(clippy::module_name_repetitions)]
pub trait BitstringUtils<N>: Sized + Copy {
    #[must_use]
    fn mask() -> N;
    #[must_use]
    fn bits_width() -> usize;
    #[must_use]
    fn bits_width_u32() -> u32;
    #[must_use]
    fn check_invariant(self) -> bool;

    #[must_use]
    fn get_bit(self, i: u32) -> bool;
    #[must_use]
    fn with_bit_set(self, i: u32, val: bool) -> Self;

    #[must_use]
    fn is_zero(self) -> bool;
    #[must_use]
    fn is_ones(self) -> bool;
    #[must_use]
    fn is_zero_bit(self) -> bool;
    #[must_use]
    fn bit_count(self) -> u32;
    #[must_use]
    fn highest_set_bit(self) -> i32;
    #[must_use]
    fn count_leading_zero_bits(self) -> u32;

    #[must_use]
    fn lsl_c(self, shift: u32) -> (Self, bool);
    #[must_use]
    fn lsl(self, shift: u32) -> Self;
    #[must_use]
    fn lsr_c(self, shift: u32) -> (Self, bool);
    #[must_use]
    fn lsr(self, shift: u32) -> Self;
    #[must_use]
    fn asr_c(self, shift: u32) -> (Self, bool);
    #[must_use]
    fn asr(self, shift: u32) -> Self;
    #[must_use]
    fn ror_c(self, shift: u32) -> (Self, bool);
    #[must_use]
    fn ror(self, shift: u32) -> Self;
    #[must_use]
    fn rrx_c(self, carry_in: bool) -> (Self, bool);
    #[must_use]
    fn rrx(self, carry_in: bool) -> Self;
}

macro_rules! __bitstring_impl_utils {
    ($ty:ty) => {
        impl<T> BitstringUtils<$ty> for Bitstring<$ty, T>
        where
            u32: From<Self>,
            Self: TryFrom<$ty> + Copy,
            <Self as TryFrom<$ty>>::Error: fmt::Debug,
        {
            /// Implementation detail (helper).
            ///
            /// For `Bitstring![N]` returns `2**N - 1` (`N` ones in binary)
            #[inline(always)]
            fn mask() -> $ty {
                let n = Self::bits_width();
                assert!(n <= MAX_BITS); // see: ctrl+f "Note(asserts)" in this file
                let mask = (1_u64 << n) - 1;
                mask.try_into().unwrap()
            }

            /// Implementation detail (helper).
            ///
            /// For `Bitstring![N]` returns the `N`.
            #[inline(always)]
            fn bits_width() -> usize {
                // see: `ensure_bool_array_marker_size()`
                size_of::<T>()
            }

            /// Implementation detail (helper).
            ///
            /// For `Bitstring![N]` returns the `N`, but as u32.
            #[inline(always)]
            fn bits_width_u32() -> u32 {
                u32::try_from(Self::bits_width()).unwrap()
            }

            /// Implementation detail (helper).
            ///
            /// Should always return true.
            ///
            /// Internal invariant:
            ///     "Non-existing highest bits" are 0s, i.e. `Bitstring![5]` is based on `u8`
            ///     and bits 0..=4 are used, but bits 5..=7 are always 0s.
            ///     Note: as an optimization we could apply the mask only when converting to `u8/u16/u32`,
            ///           but we rarely use type different than `Word` anyway
            ///           (compiler optimizes away applying the mask).
            ///           Other problem: reimplementation of operations (currently, they respect the invariant)
            #[inline(always)]
            fn check_invariant(self) -> bool {
                self.0 & !Self::mask() == 0
            }

            #[inline(always)]
            fn get_bit(self, i: u32) -> bool {
                let n = Self::bits_width_u32();
                assert!(i < n); // see: ctrl+f "Note(asserts)" in this file
                ((self.0 >> i) & 1) == 1
            }

            #[inline(always)]
            fn with_bit_set(self, i: u32, val: bool) -> Self {
                debug_assert!(self.check_invariant());
                let n = Self::bits_width_u32();
                assert!(i < n); // see: ctrl+f "Note(asserts)" in this file

                if val {
                    Self::try_from(self.0 | (1 << i)).unwrap()
                } else {
                    // note: there's no need to use the mask: self.0's "non existing high bits" are 0s anyway
                    Self::try_from(self.0 & !(1 << i)).unwrap()
                }
            }

            /// [ARM-ARM] D6.5.3
            #[inline(always)]
            fn is_zero(self) -> bool {
                debug_assert!(self.check_invariant());
                self.0 == 0
            }

            /// [ARM-ARM] D6.5.3
            #[inline(always)]
            fn is_ones(self) -> bool {
                self.bit_count() == Self::bits_width_u32()
            }

            /// [ARM-ARM] D6.5.3
            ///
            /// Note: should return `Bitstring![1]`, however `bool` is more convenient for us
            #[inline(always)]
            fn is_zero_bit(self) -> bool {
                self.is_zero()
            }

            /// [ARM-ARM] D6.5.3
            fn bit_count(self) -> u32 {
                debug_assert!(self.check_invariant());
                self.0.count_ones()
            }

            /// [ARM-ARM] D6.5.3
            fn highest_set_bit(self) -> i32 {
                debug_assert!(self.check_invariant());
                let x: u32 = self.into();
                let r = 32 - x.leading_zeros();
                debug_assert!(r <= Self::bits_width_u32());
                i32::try_from(r - 1).unwrap()
            }

            /// [ARM-ARM] D6.5.3
            fn count_leading_zero_bits(self) -> u32 {
                debug_assert!(self.check_invariant());

                let v: Word = self.zero_extend::<u32, [bool; 32]>();
                let x: u32 = v.into();
                let r: u32 = Self::bits_width_u32() - (32 - x.leading_zeros());
                debug_assert!(r <= Self::bits_width_u32());
                r
            }

            /// [ARM-ARM] A2.2.1
            #[allow(unused_comparisons)] // to be complete with docs
            fn lsl_c(self, shift: u32) -> (Self, bool) {
                let n = Self::bits_width();
                debug_assert!(n <= 32); // assumption for this implementation to be correct

                assert!(shift > 0); // see: ctrl+f "Note(asserts)" in this file
                let extended_x = u64::from(self.0).unbounded_shl(shift);
                #[allow(clippy::cast_possible_truncation, reason = "we wanto to truncate")]
                let result = Self::try_from((extended_x as $ty) & Self::mask()).unwrap();
                let carry_out = (extended_x >> n) & 0b1 == 0b1;
                (result, carry_out)
            }

            /// [ARM-ARM] A2.2.1
            #[allow(unused_comparisons)] // to be complete with docs
            fn lsl(self, shift: u32) -> Self {
                assert!(shift >= 0); // to be complete with docs; always true for `u32`, will be optimized out
                if shift == 0 {
                    self
                } else {
                    self.lsl_c(shift).0
                }
            }

            /// [ARM-ARM] A2.2.1
            fn lsr_c(self, shift: u32) -> (Self, bool) {
                let n = Self::bits_width();
                debug_assert!(n <= 32); // assumption for this implementation to be correct

                assert!(shift > 0); // see: ctrl+f "Note(asserts)" in this file
                let extended_x_c = u64::from(self.0).unbounded_shr(shift - 1);
                #[allow(clippy::cast_possible_truncation, reason = "not possible, actually")]
                let result = Self::try_from((extended_x_c >> 1) as $ty & Self::mask()).unwrap();
                let carry_out = extended_x_c & 0b1 == 0b1;
                (result, carry_out)
            }

            /// [ARM-ARM] A2.2.1
            #[allow(unused_comparisons)] // to be complete with docs
            fn lsr(self, shift: u32) -> Self {
                assert!(shift >= 0); // to be complete with docs; always true for `u32`, will be optimized out
                if shift == 0 {
                    self
                } else {
                    self.lsr_c(shift).0
                }
            }

            /// [ARM-ARM] A2.2.1
            fn asr_c(self, shift: u32) -> (Self, bool) {
                let n = Self::bits_width();
                debug_assert!(n <= 32); // assumption for this implementation to be correct

                assert!(shift > 0); // see: ctrl+f "Note(asserts)" in this file
                // types applied for `sign_extend` are same as in `Bitstring![32]` definition
                let extended_x_c = i64::from(self.sign_extend::<u32, [bool; 32]>().0.cast_signed())
                    .unbounded_shr(shift - 1);
                #[allow(clippy::cast_possible_truncation, reason = "okay, we're sign ext")]
                let result =
                    Self::try_from((extended_x_c >> 1).cast_unsigned() as $ty & Self::mask())
                        .unwrap();
                let carry_out = extended_x_c & 0b1 == 0b1;
                (result, carry_out)
            }

            /// [ARM-ARM] A2.2.1
            #[allow(unused_comparisons)] // to be complete with docs
            fn asr(self, shift: u32) -> Self {
                assert!(shift >= 0); // to be complete with docs; always true for `u32`, will be optimized out
                if shift == 0 {
                    self
                } else {
                    self.asr_c(shift).0
                }
            }

            /// [ARM-ARM] A2.2.1
            fn ror_c(self, shift: u32) -> (Self, bool) {
                let n = Self::bits_width_u32();

                assert!(shift != 0); // see: ctrl+f "Note(asserts)" in this file
                let m = shift % n; // should be optimized away for static calls
                let result = self.lsr(m) | self.lsl(n - m);
                let carry_out = result.get_bit(n - 1);
                (result, carry_out)
            }

            /// [ARM-ARM] A2.2.1
            fn ror(self, shift: u32) -> Self {
                if shift == 0 {
                    self
                } else {
                    self.ror_c(shift).0
                }
            }

            /// [ARM-ARM] A2.2.1
            fn rrx_c(self, carry_in: bool) -> (Self, bool) {
                let n = Self::bits_width_u32();

                let (low, carry_out) = self.lsr_c(1);
                let result = low.with_bit_set(n - 1, carry_in);
                (result, carry_out)
            }

            /// [ARM-ARM] A2.2.1
            fn rrx(self, carry_in: bool) -> Self {
                self.rrx_c(carry_in).0
            }
        }
    };
}

__bitstring_impl_utils!(u8);
__bitstring_impl_utils!(u16);
__bitstring_impl_utils!(u32);

impl<N, T> Bitstring<N, T> {
    /// [ARM-ARM] D6.5.3
    ///
    /// # Panics
    /// When extending to fewer bits.
    #[must_use]
    pub fn zero_extend<N2, T2>(self) -> Bitstring<N2, T2>
    where
        Self: BitstringUtils<N>,
        Bitstring<N2, T2>: TryFrom<N> + BitstringUtils<N2>,
        <Bitstring<N2, T2> as TryFrom<N>>::Error: Debug,
    {
        debug_assert!(self.check_invariant());
        let n1 = Self::bits_width();
        let n2 = Bitstring::<N2, T2>::bits_width();
        assert!(n1 <= n2, "extend to fewer bits"); // see: ctrl+f "Note(asserts)" in this file
        Bitstring::try_from(self.0).unwrap()
    }

    /// [ARM-ARM] D6.5.3
    ///
    /// # Panics
    /// When extending to fewer bits.
    #[must_use]
    pub fn sign_extend<N2, T2>(self) -> Bitstring<N2, T2>
    where
        N: Copy,
        Self: BitstringUtils<N> + Copy,
        Bitstring<N2, T2>: TryFrom<N2> + BitstringUtils<N2>,
        <Bitstring<N2, T2> as TryFrom<N2>>::Error: Debug,
        N2: From<N> + BitXor<Output = N2> + BitOr<Output = N2>,
    {
        debug_assert!(self.check_invariant());
        let n1 = Self::bits_width_u32();
        let n2 = Bitstring::<N2, T2>::bits_width_u32();
        assert!(n1 <= n2, "extend to fewer bits"); // see: ctrl+f "Note(asserts)" in this file
        let val = N2::from(self.0);
        let top_bit = self.get_bit(n1 - 1);
        if top_bit {
            let mask1 = N2::from(Self::mask());
            let mask2 = Bitstring::<N2, T2>::mask();
            let diff_ones = mask1 ^ mask2;
            let val = val | diff_ones;
            Bitstring::try_from(val).unwrap()
        } else {
            Bitstring::try_from(val).unwrap()
        }
    }
}

/// According to the [docs](size_of()), we can imply `size_of::<[bool; N]> == N`.
/// This is assumed in the implementation, so let's have a check to ensure
/// this assumption holds true. (See: `bits_width()`)
#[test]
#[allow(clippy::cognitive_complexity, clippy::tests_outside_test_module)]
fn ensure_bool_array_marker_size() {
    // helper macro to shorten the code
    macro_rules! check {
        ($($size:tt),* $(,)?) => {
            $( assert_eq!(size_of::<[bool; $size]>(), $size); )*
            $( assert_eq!(<crate::Bitstring![$size]>::bits_width(), $size); )*
        };
    }
    check![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28, 29, 30, 31, 32,
    ];
}

// ============================================================================
// Trait operations (operators)
// ============================================================================

impl<N, T> Binary for Bitstring<N, T>
where
    N: Copy,
    Self: BitstringUtils<N>,
    u32: From<N>,
{
    /// Binary formatting will always print all digits
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // Note: updating formatting params is unstable
        let val = u32::from(self.0);
        let width = Self::bits_width();
        if !f.alternate()
            && !f.sign_aware_zero_pad()
            && f.align().is_none()
            && f.width().is_none_or(|w| w <= width)
        {
            // do we really need a fast path?
            write!(f, "{val:0width$b}")
        } else {
            f.pad_integral(true, "0b", &format!("{val:0width$b}"))
        }
    }
}

/// [ARM-ARM] D6.5.4
impl<N, T> Add for Bitstring<N, T>
where
    // note: we need `u32` for `wrapping_add`
    u32: From<N>,
    Self: TryFrom<u32> + BitstringUtils<N>,
    <Self as TryFrom<u32>>::Error: Debug,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let (lhs, rhs) = (u32::from(self.0), u32::from(rhs.0));
        let val = lhs.wrapping_add(rhs);
        #[allow(clippy::suspicious_arithmetic_impl)]
        let val = val & u32::from(Self::mask());
        Bitstring::try_from(val).unwrap()
    }
}

/// [ARM-ARM] D6.5.4
impl<N, T> Sub for Bitstring<N, T>
where
    // note: we need `u32` for `wrapping_sub`
    u32: From<N>,
    Self: TryFrom<u32> + BitstringUtils<N>,
    <Self as TryFrom<u32>>::Error: Debug,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let (lhs, rhs) = (u32::from(self.0), u32::from(rhs.0));
        let val = lhs.wrapping_sub(rhs);
        #[allow(clippy::suspicious_arithmetic_impl)]
        let val = val & u32::from(Self::mask());
        Bitstring::try_from(val).unwrap()
    }
}

/// [ARM-ARM] D6.5.3
impl<N, T> BitAnd for Bitstring<N, T>
where
    N: BitAnd + From<<N as BitAnd>::Output>,
    Self: TryFrom<N> + BitstringUtils<N>,
    <Self as TryFrom<N>>::Error: Debug,
{
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        debug_assert!(self.check_invariant() && rhs.check_invariant());
        let val = N::from(self.0 & rhs.0);
        Bitstring::try_from(val).unwrap()
    }
}

/// [ARM-ARM] D6.5.3
impl<N, T> BitOr for Bitstring<N, T>
where
    N: BitOr + From<<N as BitOr>::Output>,
    Self: TryFrom<N> + BitstringUtils<N>,
    <Self as TryFrom<N>>::Error: Debug,
{
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        debug_assert!(self.check_invariant() && rhs.check_invariant());
        let val = N::from(self.0 | rhs.0);
        Bitstring::try_from(val).unwrap()
    }
}

/// [ARM-ARM] D6.5.3
impl<N, T> BitXor for Bitstring<N, T>
where
    N: BitXor + From<<N as BitXor>::Output>,
    Self: TryFrom<N> + BitstringUtils<N>,
    <Self as TryFrom<N>>::Error: Debug,
{
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self {
        debug_assert!(self.check_invariant() && rhs.check_invariant());
        let val = N::from(self.0 ^ rhs.0);
        Bitstring::try_from(val).unwrap()
    }
}

/// [ARM-ARM] D6.5.3
impl<N, T> Not for Bitstring<N, T>
where
    N: Not<Output = N> + BitAnd<Output = N>,
    Self: TryFrom<N> + BitstringUtils<N>,
    <Self as TryFrom<N>>::Error: Debug,
{
    type Output = Self;

    fn not(self) -> Self {
        let val = !self.0;
        let val = val & Self::mask();
        Bitstring::try_from(val).unwrap()
    }
}

// ============================================================================
// Macros: `Bistring![N]` type, extract and concat operations (+helpers)
// ============================================================================

#[doc(hidden)]
#[macro_export]
// See: ctrl+f "Note(clippy-repetitions)" in this file.
#[allow(clippy::module_name_repetitions)]
macro_rules! __bitstring_width_to_num_type {
    [1] => {u8};
    [2] => {u8};
    [3] => {u8};
    [4] => {u8};
    [5] => {u8};
    [6] => {u8};
    [7] => {u8};
    [8] => {u8};
    [9] => {u16};
    [10] => {u16};
    [11] => {u16};
    [12] => {u16};
    [13] => {u16};
    [14] => {u16};
    [15] => {u16};
    [16] => {u16};
    [17] => {u32};
    [18] => {u32};
    [19] => {u32};
    [20] => {u32};
    [21] => {u32};
    [22] => {u32};
    [23] => {u32};
    [24] => {u32};
    [25] => {u32};
    [26] => {u32};
    [27] => {u32};
    [28] => {u32};
    [29] => {u32};
    [30] => {u32};
    [31] => {u32};
    [32] => {u32};
}

/// `Bitstring![N]` generates a valid type for a `Bitstring` of `N` bits (`1 <= N <= 32`).
/// Example usage in `bitstring_extract!` and `bitstring_concat!`.
#[macro_export]
macro_rules! Bitstring {
    [$N:tt] => {
        $crate::common::Bitstring::<$crate::__bitstring_width_to_num_type![$N], [bool; $N]>
    };
}

/// Extract "subbitstring".
/// Syntax: `bitstring_extract!(ident<high_index:low_index> | n bits)`.
/// Alternative: `bitstring_extract!((expr)<high_index:low_index> | n bits)`.
///
/// ```
/// use cmemu_lib::{Bitstring, bitstring_extract};
///
/// let imm5 = <Bitstring![5]>::try_from(0b11011_u32).unwrap();
/// let imm2 = bitstring_extract!(imm5<3:2> | 2 bits);
/// let imm2_eq = <Bitstring![2]>::try_from(0b10_u32).unwrap();
/// assert_eq!(imm2, imm2_eq);
/// ```
#[macro_export]
// See: ctrl+f "Note(clippy-repetitions)" in this file.
#[allow(clippy::module_name_repetitions)]
macro_rules! bitstring_extract {
    (($bs:expr) < $hi:literal : $lo:literal > | $N:tt bits) => {{
        // See: ctrl+f "Note(clippy-workaround)" in this file.
        #[allow(clippy::items_after_statements)] // for better readability
        #[allow(clippy::eq_op, reason = "compile-time asserts")]
        let clippy_allow_workaround = || {
            #[allow(dead_code)] // It's not dead, though.
            const NEW_N_: usize = $hi - $lo + 1;
            const {
                assert!($lo <= $hi, "Lower index must not be greater than upper index.");
                assert!(NEW_N_ == $N, "Specified bit range and its size does not match.");
                // Assumption: bitstring has at most 32 bits.
                // Reason: 1_u32 << 32 is undefined behavior, so we use 1_u64.
                assert!($N <= $crate::common::bitstring::MAX_BITS, "Specified size is greater than possible maximum.");
            }
            type OutBitString = $crate::Bitstring![$N];
            let bs: $crate::common::Bitstring<_, _> = $bs;
            // Checks type and returns size. We cannot simply call bits_with having only `bs`.
            #[inline]
            fn helper<Num, T>(bs: $crate::common::Bitstring<Num, T>) -> usize
            where
                $crate::common::Bitstring<Num, T>: $crate::common::BitstringUtils<Num>,
            {
                debug_assert!($crate::common::BitstringUtils::check_invariant(bs));
                <$crate::common::Bitstring<Num, T> as $crate::common::BitstringUtils<Num>>::bits_width()
            }
            let n_ = helper(bs);

            // See: ctrl+f "Note(asserts)" in this file.
            assert!(
                $hi < n_,
                "Bitstring![{}]: can't extract bits <{}:{}>",
                n_,
                $hi,
                $lo,
            );

            let val_ = u32::from(bs) >> $lo;
            let mask_ = u32::from(<OutBitString as $crate::common::BitstringUtils<_>>::mask());
            <OutBitString as std::convert::TryFrom<_>>::try_from(val_ & mask_).unwrap()
        };
        clippy_allow_workaround()
    }};
    (($bs:expr) < $idx:literal > | 1 bits) => {{
        bitstring_extract!(($bs)<$idx : $idx> | 1 bits)
    }};
    ($bs:ident < $hi:literal : $lo:literal > | $N:tt bits) => {{
        bitstring_extract!(($bs)<$hi:$lo> | $N bits)
    }};
    ($bs:ident < $idx:literal > | 1 bits) => {{
        bitstring_extract!($bs<$idx : $idx> | 1 bits)
    }};
}

/// Substitues some bits with another.
/// Syntax: `bitstring_substitute!(ident<high_index:low_index> = expr(new, bits))`.
///
/// ```
/// use cmemu_lib::{Bitstring, bitstring_substitute};
///
/// let mut imm5 = <Bitstring![5]>::try_from(0b11011_u32).unwrap();
/// let imm3 = <Bitstring![3]>::try_from(0b011_u32).unwrap();
/// bitstring_substitute!(imm5<3:1> = imm3);
/// let imm5_eq = <Bitstring![5]>::try_from(0b10111_u32).unwrap();
/// assert_eq!(imm5, imm5_eq);
/// ```
#[macro_export]
// See: ctrl+f "Note(clippy-repetitions)" in this file.
#[allow(clippy::module_name_repetitions)]
macro_rules! bitstring_substitute {
    ($bs:ident < $hi:literal : $lo:literal > = $new_bits:expr) => {{
        // See: ctrl+f "Note(clippy-workaround)" in this file.
        #[allow(clippy::items_after_statements)] // for better readability
        #[allow(clippy::eq_op, reason = "compile-time asserts")]
        let mut clippy_allow_workaround = || {
            let substituted_bits_ = $hi - $lo + 1;
            const { assert!($lo <= $hi, "Lower index must not be greater than upper index."); }
            // Checks type and returns size and mask.
            #[inline]
            fn helper<Num, T>(bs: $crate::common::Bitstring<Num, T>) -> (usize, Num)
            where
                $crate::common::Bitstring<Num, T>: $crate::common::BitstringUtils<Num>,
            {
                debug_assert!($crate::common::BitstringUtils::check_invariant(bs));
                (
                    <$crate::common::Bitstring<Num, T> as $crate::common::BitstringUtils<Num>>::bits_width(),
                    <$crate::common::Bitstring<Num, T> as $crate::common::BitstringUtils<Num>>::mask(),
                )
            }
            let (n1_, _) = helper($bs);
            let (n2_, mask2_) = helper($new_bits);
            // See: ctrl+f "Note(asserts)" in this file.
            assert!(
                $hi < n1_,
                "Bitstring![{}]: can't substitute bits <{}:{}>",
                n1_,
                $hi,
                $lo,
            );
            // See: ctrl+f "Note(asserts)" in this file.
            assert_eq!(
                substituted_bits_, n2_,
                "Bitstring!: tried to substitute {} bits with {} bits",
                substituted_bits_, n2_
            );

            let bs_val_ = u32::from($bs);
            let clearing_mask_ = !(u32::from(mask2_) << $lo);
            let new_bits_moved_ = u32::from($new_bits) << $lo;
            let new_val_ = (bs_val_ & clearing_mask_) | new_bits_moved_;
            $bs = std::convert::TryInto::try_into(new_val_).unwrap();
        };
        clippy_allow_workaround()
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __bistring_concat_helper {
    () => {
        0_u32
    };
    ( $val:expr, $_size:expr; $($val_tail:expr, $size_tail:expr;)* ) => {
        ($val.unbounded_shl((0 $(+ $size_tail)*)))
        | $crate::__bistring_concat_helper!($($val_tail, $size_tail;)*)
    };
}

/// Concatenates bitstrings.
/// Syntax: `bitstring_concat!(ident : path : some::constant::path | n bits)`.
///
/// ```
/// use cmemu_lib::{Bitstring, bitstring_concat};
///
/// let imm3 = <Bitstring![3]>::try_from(0b101_u32).unwrap();
/// let imm2 = <Bitstring![2]>::try_from(0b10_u32).unwrap();
/// let imm7 = bitstring_concat!(imm2 : imm3 : imm2 | 7 bits);
/// let imm7_eq = <Bitstring![7]>::try_from(0b1010110_u32).unwrap();
/// assert_eq!(imm7, imm7_eq);
/// ```
#[macro_export]
// See: ctrl+f "Note(clippy-repetitions)" in this file.
#[allow(clippy::module_name_repetitions)]
macro_rules! bitstring_concat {
    ( $($bs:path):+ | $N:tt bits) => {{
        // See: ctrl+f "Note(clippy-workaround)" in this file.
        #[allow(clippy::items_after_statements, clippy::eq_op)] // for better readability
        let clippy_allow_workaround = || {
            // Assumption: bitstring has at most 32 bits.
            const {
                assert!($N <= $crate::common::bitstring::MAX_BITS,
                        "Specified size is greater than possible maximum.");
            }
            type OutBitString = $crate::Bitstring![$N];
            // Checks type and returns size. We cannot simply call bits_with having only `bs`.
            #[inline]
            fn helper<Num, T>(bs: $crate::common::Bitstring<Num, T>) -> u32
            where
                $crate::common::Bitstring<Num, T>: $crate::common::BitstringUtils<Num>,
            {
                debug_assert!($crate::common::BitstringUtils::check_invariant(bs));
                <$crate::common::Bitstring::<Num, T> as $crate::common::BitstringUtils::<Num>>::bits_width_u32()
            }
            let calculated_n_ = 0 $(+ helper($bs))+;
            // See: ctrl+f "Note(asserts)" in this file.
            assert_eq!(calculated_n_, $N, "got {} bits, but declared as {} bits", calculated_n_, $N);
            let val_ = $crate::__bistring_concat_helper!( $( u32::from($bs), helper($bs); )+ );
            let mask_ = u32::from(<OutBitString as $crate::common::BitstringUtils<_>>::mask());
            <OutBitString as std::convert::TryFrom<_>>::try_from(val_ & mask_).unwrap()
        };
        clippy_allow_workaround()
    }};
}

// ============================================================================
// Conversions between u{8,16,32} and `Bitstring![N]`
// ============================================================================

// internal macro to use right here
macro_rules! __bitstring_impl_conversions {
    ([From]<[$ty:ty]> for Bitstring[$N:tt]) => {
        impl From<$ty> for $crate::Bitstring![$N] {
            fn from(value: $ty) -> Self {
                let value: crate::__bitstring_width_to_num_type![$N] = value.into();
                Bitstring(value, PhantomData)
            }
        }
    };
    ([Into]<[$ty:ty]> for Bitstring[$N:tt]) => {
        // "`Into`", however `From` is preferred
        impl From<$crate::Bitstring![$N]> for $ty {
            fn from(value: $crate::Bitstring![$N]) -> Self {
                value.0.into()
            }
        }
    };
    ([from_const]<[$ty:ty]> for Bitstring[$N:tt]) => {
        impl $crate::Bitstring![$N] {
            #[inline]
            pub const fn from_const(value: $ty) -> Self {
                Self(value, PhantomData)
            }
        }
    };
    ([TryFrom]<[$ty:ty]> for Bitstring[$N:tt]) => {
        impl TryFrom<$ty> for $crate::Bitstring![$N] {
            type Error = &'static str;

            fn try_from(value: $ty) -> Result<Self, Self::Error> {
                if u64::from(value) < (1_u64 << $N) {
                    let value: crate::__bitstring_width_to_num_type![$N] =
                        value.try_into().unwrap();
                    Ok(Bitstring(value, PhantomData))
                } else {
                    Err("passed value has too many bits")
                }
            }
        }
    };
    ([TryInto]<[$ty:ty]> for Bitstring[$N:tt]) => {
        // "`TryInto`", however `TryFrom` is preferred
        impl TryFrom<$crate::Bitstring![$N]> for $ty {
            type Error = <$ty as TryFrom<crate::__bitstring_width_to_num_type![$N]>>::Error;

            fn try_from(value: $crate::Bitstring![$N]) -> Result<Self, Self::Error> {
                debug_assert!(value.check_invariant());
                value.0.try_into()
            }
        }
    };
    ([$trait:ident]<[$ty:ty]> for Bitstring[ $($N:tt),+] ) => {
        $( __bitstring_impl_conversions! {[$trait]<[$ty]> for Bitstring[$N]} )*
    };
    ([$trait:ident]< [$($ty:ty),+] > for Bitstring $Ns:tt ) => {
        $( __bitstring_impl_conversions! {[$trait]<[$ty]> for Bitstring $Ns} )*
    };
    ([$($trait:ident),+]< $Ts:tt > for Bitstring $Ns:tt ) => {
        $( __bitstring_impl_conversions! {[$trait]< $Ts > for Bitstring $Ns} )*
    };

    // ========================================================================

    ([From]<Bitstring[$Nt:tt]> for Bitstring[$N:tt]) => {
        impl From<$crate::Bitstring![$Nt]> for $crate::Bitstring![$N] {
            fn from(value: $crate::Bitstring![$Nt]) -> Self {
                debug_assert!(value.check_invariant());
                let n_from = <$crate::Bitstring![$Nt]>::bits_width();
                let n_into = <$crate::Bitstring![$N]>::bits_width();
                // see: ctrl+f "Note(asserts)" in this file
                assert!(n_from <= n_into, "This conversion shouldn't have been generated");
                Bitstring::try_from(value.0).unwrap()
            }
        }
    };
    ([TryInto]<Bitstring[$Nt:tt]> for Bitstring[$N:tt]) => {
        // "`TryInto`", however `TryFrom` is preferred
        impl TryFrom<$crate::Bitstring![$N]> for $crate::Bitstring![$Nt] {
            type Error = <Self as TryFrom<crate::__bitstring_width_to_num_type![$N]>>::Error;

            fn try_from(value: $crate::Bitstring![$N]) -> Result<Self, Self::Error> {
                debug_assert!(value.check_invariant());
                value.0.try_into()
            }
        }
    };
    ([$trait:ident]< Bitstring[$Nt:tt] > for Bitstring[ $($N:tt),+] ) => {
        $( __bitstring_impl_conversions! {[$trait]< Bitstring[$Nt] > for Bitstring[$N]} )*
    };
    ([$($trait:ident),+]< Bitstring[$Nt:tt] > for Bitstring $Ns:tt ) => {
        $( __bitstring_impl_conversions! {[$trait]< Bitstring[$Nt] > for Bitstring $Ns} )*
    };
    (Bitstring [(cycle-shift-left) $Ts:tt $N_head:tt for $N_next:tt $(, $N_tail:tt)*]) => {
        __bitstring_impl_conversions! {$Ts< Bitstring[$N_head] > for Bitstring [$N_next $(, $N_tail)*]}
        __bitstring_impl_conversions! {Bitstring [(cycle-shift-left) $Ts $N_next for $($N_tail),*]}
    };
    (Bitstring [(cycle-shift-left) $Ts:tt $N_head:tt for]) => {};
}

// order: (bitstring[N], [u8, u16, u32])
// for each bitstring[N] and u{8,16,32} there are (try_)from and (try_)into
__bitstring_impl_conversions! {[TryFrom, Into]<[u8, u16, u32]> for Bitstring[1, 2, 3, 4, 5, 6, 7]}
__bitstring_impl_conversions! {[From, from_const, Into]<[u8]> for Bitstring[8]}
__bitstring_impl_conversions! {[TryFrom, Into]<[u16, u32]> for Bitstring[8]}
__bitstring_impl_conversions! {[From, TryInto]<[u8]> for Bitstring[9, 10, 11, 12, 13, 14, 15]}
__bitstring_impl_conversions! {[TryFrom, Into]<[u16, u32]> for Bitstring[9, 10, 11, 12, 13, 14, 15]}
__bitstring_impl_conversions! {[From, TryInto]<[u8]> for Bitstring[16]}
__bitstring_impl_conversions! {[From, from_const, Into]<[u16]> for Bitstring[16]}
__bitstring_impl_conversions! {[TryFrom, Into]<[u32]> for Bitstring[16]}
__bitstring_impl_conversions! {[From, TryInto]<[u8, u16]> for Bitstring[17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31]}
__bitstring_impl_conversions! {[TryFrom, Into]<[u32]> for Bitstring[17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31]}
__bitstring_impl_conversions! {[From, TryInto]<[u8, u16]> for Bitstring[32]}
__bitstring_impl_conversions! {[From, from_const, Into]<[u32]> for Bitstring[32]}

// conversions between bitstrings
// Note: From<Bitstring[N1]> for Bitstring[N2] <=> Into<Bitstring[N2]> for Bitstring[N1]
__bitstring_impl_conversions! {Bitstring[(cycle-shift-left)
    [From, TryInto] 1 for 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
    17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
]}

// extra conversions for byte arrays ==========================================
macro_rules! __bitstring_impl_byte_array_conversions {
    (Bitstring[$N:tt] as $arr:ty) => {
        impl Bitstring![$N] {
            pub const fn from_le_bytes(b: $arr) -> Self {
                Self::from_const(<$crate::__bitstring_width_to_num_type![$N]>::from_le_bytes(
                    b,
                ))
            }

            pub const fn to_le_bytes(self) -> $arr {
                self.0.to_le_bytes()
            }
        }
    };
}

__bitstring_impl_byte_array_conversions! {Bitstring[8] as [u8; 1]}
__bitstring_impl_byte_array_conversions! {Bitstring[16] as [u8; 2]}
__bitstring_impl_byte_array_conversions! {Bitstring[32] as [u8; 4]}

// Utility for BitString[1] <-> bool interoperability
impl From<bool> for Bitstring![1] {
    fn from(b: bool) -> Self {
        constants::C_0.with_bit_set(0, b)
    }
}

impl From<Bitstring![1]> for bool {
    fn from(b: Bitstring![1]) -> Self {
        b.is_ones()
    }
}

impl Bitstring![1] {
    pub fn is_set(&self) -> bool {
        self.get_bit(0)
    }

    pub fn is_cleared(&self) -> bool {
        !self.is_set()
    }

    #[must_use]
    pub fn with_set(&self) -> Self {
        self.with_bit_set(0, true)
    }

    #[must_use]
    pub fn with_cleared(&self) -> Self {
        self.with_bit_set(0, false)
    }
}

// ============================================================================
// Constants
// ============================================================================

/// Often used constants (in [ARM-ARM])
pub mod constants {
    use super::{Bitstring, PhantomData};

    pub const C_0: crate::Bitstring![1] = Bitstring(0b0, PhantomData);
    pub const C_1: crate::Bitstring![1] = Bitstring(0b1, PhantomData);
    pub const C_00: crate::Bitstring![2] = Bitstring(0b00, PhantomData);
    pub const C_01: crate::Bitstring![2] = Bitstring(0b01, PhantomData);
    pub const C_10: crate::Bitstring![2] = Bitstring(0b10, PhantomData);
    pub const C_11: crate::Bitstring![2] = Bitstring(0b11, PhantomData);
    pub const C_000: crate::Bitstring![3] = Bitstring(0b000, PhantomData);
    pub const C_001: crate::Bitstring![3] = Bitstring(0b001, PhantomData);
    pub const C_010: crate::Bitstring![3] = Bitstring(0b010, PhantomData);
    pub const C_011: crate::Bitstring![3] = Bitstring(0b011, PhantomData);
    pub const C_100: crate::Bitstring![3] = Bitstring(0b100, PhantomData);
    pub const C_101: crate::Bitstring![3] = Bitstring(0b101, PhantomData);
    pub const C_110: crate::Bitstring![3] = Bitstring(0b110, PhantomData);
    pub const C_111: crate::Bitstring![3] = Bitstring(0b111, PhantomData);

    pub const C_0000: crate::Bitstring![4] = Bitstring(0b0000, PhantomData);
    pub const C_0001: crate::Bitstring![4] = Bitstring(0b0001, PhantomData);
    pub const C_0010: crate::Bitstring![4] = Bitstring(0b0010, PhantomData);
    pub const C_0011: crate::Bitstring![4] = Bitstring(0b0011, PhantomData);
    pub const C_0100: crate::Bitstring![4] = Bitstring(0b0100, PhantomData);
    pub const C_0101: crate::Bitstring![4] = Bitstring(0b0101, PhantomData);
    pub const C_0110: crate::Bitstring![4] = Bitstring(0b0110, PhantomData);
    pub const C_0111: crate::Bitstring![4] = Bitstring(0b0111, PhantomData);
    pub const C_1000: crate::Bitstring![4] = Bitstring(0b1000, PhantomData);
    pub const C_1001: crate::Bitstring![4] = Bitstring(0b1001, PhantomData);
    pub const C_1010: crate::Bitstring![4] = Bitstring(0b1010, PhantomData);
    pub const C_1011: crate::Bitstring![4] = Bitstring(0b1011, PhantomData);
    pub const C_1100: crate::Bitstring![4] = Bitstring(0b1100, PhantomData);
    pub const C_1101: crate::Bitstring![4] = Bitstring(0b1101, PhantomData);
    pub const C_1110: crate::Bitstring![4] = Bitstring(0b1110, PhantomData);
    pub const C_1111: crate::Bitstring![4] = Bitstring(0b1111, PhantomData);

    pub const C_0_0000: crate::Bitstring![5] = Bitstring(0b0_0000, PhantomData);
    pub const C_0_0001: crate::Bitstring![5] = Bitstring(0b0_0001, PhantomData);
    pub const C_0_0010: crate::Bitstring![5] = Bitstring(0b0_0010, PhantomData);
    pub const C_0_0011: crate::Bitstring![5] = Bitstring(0b0_0011, PhantomData);
    pub const C_0_0100: crate::Bitstring![5] = Bitstring(0b0_0100, PhantomData);
    pub const C_0_0101: crate::Bitstring![5] = Bitstring(0b0_0101, PhantomData);
    pub const C_0_0110: crate::Bitstring![5] = Bitstring(0b0_0110, PhantomData);
    pub const C_0_0111: crate::Bitstring![5] = Bitstring(0b0_0111, PhantomData);
    pub const C_0_1000: crate::Bitstring![5] = Bitstring(0b0_1000, PhantomData);
    pub const C_0_1001: crate::Bitstring![5] = Bitstring(0b0_1001, PhantomData);
    pub const C_0_1010: crate::Bitstring![5] = Bitstring(0b0_1010, PhantomData);
    pub const C_0_1011: crate::Bitstring![5] = Bitstring(0b0_1011, PhantomData);
    pub const C_0_1100: crate::Bitstring![5] = Bitstring(0b0_1100, PhantomData);
    pub const C_0_1101: crate::Bitstring![5] = Bitstring(0b0_1101, PhantomData);
    pub const C_0_1110: crate::Bitstring![5] = Bitstring(0b0_1110, PhantomData);
    pub const C_0_1111: crate::Bitstring![5] = Bitstring(0b0_1111, PhantomData);

    pub const C_1_0000: crate::Bitstring![5] = Bitstring(0b1_0000, PhantomData);
    pub const C_1_0001: crate::Bitstring![5] = Bitstring(0b1_0001, PhantomData);
    pub const C_1_0010: crate::Bitstring![5] = Bitstring(0b1_0010, PhantomData);
    pub const C_1_0011: crate::Bitstring![5] = Bitstring(0b1_0011, PhantomData);
    pub const C_1_0100: crate::Bitstring![5] = Bitstring(0b1_0100, PhantomData);
    pub const C_1_0101: crate::Bitstring![5] = Bitstring(0b1_0101, PhantomData);
    pub const C_1_0110: crate::Bitstring![5] = Bitstring(0b1_0110, PhantomData);
    pub const C_1_0111: crate::Bitstring![5] = Bitstring(0b1_0111, PhantomData);
    pub const C_1_1000: crate::Bitstring![5] = Bitstring(0b1_1000, PhantomData);
    pub const C_1_1001: crate::Bitstring![5] = Bitstring(0b1_1001, PhantomData);
    pub const C_1_1010: crate::Bitstring![5] = Bitstring(0b1_1010, PhantomData);
    pub const C_1_1011: crate::Bitstring![5] = Bitstring(0b1_1011, PhantomData);
    pub const C_1_1100: crate::Bitstring![5] = Bitstring(0b1_1100, PhantomData);
    pub const C_1_1101: crate::Bitstring![5] = Bitstring(0b1_1101, PhantomData);
    pub const C_1_1110: crate::Bitstring![5] = Bitstring(0b1_1110, PhantomData);
    pub const C_1_1111: crate::Bitstring![5] = Bitstring(0b1_1111, PhantomData);

    pub const C_00_0000: crate::Bitstring![6] = Bitstring(0b00_0000, PhantomData);
    pub const C_000_0000: crate::Bitstring![7] = Bitstring(0b000_0000, PhantomData);
    pub const C_0000_0000: crate::Bitstring![8] = Bitstring(0b0000_0000, PhantomData);
    pub const C_0000_0001: crate::Bitstring![8] = Bitstring(0b0000_0001, PhantomData);
    pub const C_0000_0010: crate::Bitstring![8] = Bitstring(0b0000_0010, PhantomData);
    pub const C_0000_0011: crate::Bitstring![8] = Bitstring(0b0000_0011, PhantomData);
    pub const C_0000_0100: crate::Bitstring![8] = Bitstring(0b0000_0100, PhantomData);
    pub const C_0000_0101: crate::Bitstring![8] = Bitstring(0b0000_0101, PhantomData);
    pub const C_0000_0110: crate::Bitstring![8] = Bitstring(0b0000_0110, PhantomData);
    pub const C_0000_0111: crate::Bitstring![8] = Bitstring(0b0000_0111, PhantomData);
    pub const C_0000_1000: crate::Bitstring![8] = Bitstring(0b0000_1000, PhantomData);
    pub const C_0000_1001: crate::Bitstring![8] = Bitstring(0b0000_1001, PhantomData);
    pub const C_0001_0000: crate::Bitstring![8] = Bitstring(0b0001_0000, PhantomData);
    pub const C_0001_0001: crate::Bitstring![8] = Bitstring(0b0001_0001, PhantomData);
    pub const C_0001_0010: crate::Bitstring![8] = Bitstring(0b0001_0010, PhantomData);
    pub const C_0001_0011: crate::Bitstring![8] = Bitstring(0b0001_0011, PhantomData);
    pub const C_0001_0100: crate::Bitstring![8] = Bitstring(0b0001_0100, PhantomData);

    pub const C_0_0000_0000: crate::Bitstring![9] = Bitstring(0b0_0000_0000, PhantomData);
}
