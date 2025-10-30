use core::ops::{Range, RangeBounds, RangeInclusive};
use std::collections::Bound;
use std::fmt;

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Address(u32);

impl Address {
    #[must_use]
    pub const fn from_const(constant: u32) -> Self {
        Self(constant)
    }

    #[must_use]
    pub const fn to_const(self) -> u32 {
        self.0
    }

    #[must_use]
    pub const fn offset(self, count: u32) -> Self {
        Self(self.0.wrapping_add(count))
    }

    #[must_use]
    pub const fn wrapping_sub(self, count: u32) -> Self {
        Self(self.0.wrapping_sub(count))
    }

    #[must_use]
    pub const fn offset_from(self, other: Self) -> u32 {
        self.0.wrapping_sub(other.0)
    }

    #[must_use]
    pub const fn aligned_down_to_2_bytes(self) -> Self {
        Self(self.0 & (!0b1))
    }

    #[must_use]
    pub const fn aligned_down_to_4_bytes(self) -> Self {
        Self(self.0 & (!0b11))
    }

    #[must_use]
    pub const fn masked(self, mask: u32) -> Self {
        Self(self.0 & mask)
    }

    #[must_use]
    pub const fn aligned_down_to_8_bytes(self) -> Self {
        Self(self.0 & (!0b111))
    }

    #[must_use]
    pub const fn is_aligned_to_4_bytes(self) -> bool {
        self.0.trailing_zeros() >= 2
    }

    #[must_use]
    pub const fn is_aligned_to_8_bytes(self) -> bool {
        self.0.trailing_zeros() >= 3
    }

    /// Make address range (exclusive: ``from..(from+len)``).
    ///
    /// # Panics
    /// Panics on overflow. Use `range_inclusive_from_len` or make an unbounded range
    /// if you need to indicate a range up to the end of the address space.
    #[must_use]
    pub const fn range_from_len(from: u32, len: u32) -> Range<Self> {
        Self(from)..Self(from.checked_add(len).unwrap())
    }

    /// Make inclusive address range (exclusive: ``from..=(from+len)``).
    ///
    /// # Panics
    /// Panics on empty range (`len == 0`).
    #[must_use]
    pub const fn range_inclusive_from_len(from: u32, len: u32) -> RangeInclusive<Self> {
        assert!(len > 0);
        Self(from)..=Self(from.saturating_add(len.saturating_sub(1)))
    }
}

// Range operations
impl Address {
    #[must_use]
    pub const fn is_in_range(self, range: &Range<Self>) -> bool {
        range.start.0 <= self.0 && self.0 < range.end.0
    }

    #[must_use]
    pub const fn is_in_range_inclusive(self, range: &RangeInclusive<Self>) -> bool {
        range.start().0 <= self.0 && self.0 <= range.end().0
    }

    // Const fn cannot be generic yet, update once it is stabilized.
    #[must_use]
    pub fn is_in_bounds(self, bounds: &impl RangeBounds<Self>) -> bool {
        Address::is_range_covered(bounds, &(self..=self))
    }

    /// Const versions of `is_in_bounds`.
    ///
    /// Prefer using `xbounds` and `is_subrange` macros.
    #[must_use]
    pub const fn is_in_bounds_const(self, bounds: (Bound<&Self>, Bound<&Self>)) -> bool {
        Address::is_range_covered_const(bounds, (Bound::Included(&self), Bound::Included(&self)))
    }

    /// Does `sub_range` is fully inside `container`, assuming discrete addresses.
    #[must_use]
    pub fn is_range_covered(
        container: &impl RangeBounds<Address>,
        sub_range: &impl RangeBounds<Address>,
    ) -> bool {
        Self::is_range_covered_const(
            (container.start_bound(), container.end_bound()),
            (sub_range.start_bound(), sub_range.end_bound()),
        )
    }

    /// Does `sub_range` is fully inside `container`, assuming discrete addresses?
    ///
    /// Note: use `is_range_covered` once const traits are stable
    #[must_use]
    pub const fn is_range_covered_const(
        container: (Bound<&Self>, Bound<&Self>),
        sub_range: (Bound<&Self>, Bound<&Self>),
    ) -> bool {
        use std::ops::Bound::{Excluded, Included, Unbounded};
        macro_rules! c {
            ($a:expr) => {
                $a.to_const()
            };
        }
        (match (container.0, sub_range.0) {
            (Unbounded | Included(Address(u32::MIN)), _) => true,
            (_, Unbounded) => false,
            (Included(&x), Included(&y)) | (Excluded(&x), Excluded(&y)) => c!(x) <= c!(y),
            (Excluded(&x), Included(&y)) => c!(x) < c!(y),
            (Included(&x), Excluded(&y)) => {
                c!(x) <= c!(y) || (c!(y) < c!(x) && x.offset_from(y) == 1)
            }
        }) && (match (container.1, sub_range.1) {
            (Unbounded | Included(Address(u32::MAX)), _) => true,
            (_, Unbounded) => false,
            (Included(&x), Included(&y)) | (Excluded(&x), Excluded(&y)) => c!(x) >= c!(y),
            (Excluded(&x), Included(&y)) => c!(x) > c!(y),
            (Included(&x), Excluded(&y)) => {
                c!(x) >= c!(y) || (c!(y) > c!(x) && y.offset_from(x) == 1)
            }
        })
    }
}

impl From<u32> for Address {
    fn from(val: u32) -> Self {
        Self(val)
    }
}

impl From<Address> for u32 {
    fn from(val: Address) -> u32 {
        val.0
    }
}

impl fmt::Debug for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:08x}", self.0)
    }
}

/// Shift range `a..b` to `(a+count)..(b+count)`
///
/// # Panics
/// Panics on overflow.
#[must_use]
pub const fn offset_range(range: Range<Address>, count: u32) -> Range<Address> {
    assert!(
        range.end.to_const().checked_add(count).is_some(),
        "Range offset wrapping"
    );
    range.start.offset(count)..range.end.offset(count)
}

pub const EMPTY_RANGE: Range<Address> = Address(0)..Address(0);
#[allow(clippy::exhaustive_structs)]
#[derive(Debug)]
pub struct RangeUnion<'a, T1, T2>(pub &'a T1, pub &'a T2);
#[allow(clippy::exhaustive_structs)]
#[derive(Debug)]
pub struct RangeIntersection<'a, T1, T2>(pub &'a T1, pub &'a T2);

// This is a bit hacky utility, to make is_in_range const check work in macros and be variadic by range type,
// Therefore, we cannot use RangeBounds' method
// This module is public to export macro helpers.
pub mod const_hax {
    use super::{Address, RangeIntersection, RangeUnion};
    use core::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};
    use std::collections::Bound;
    use std::ops::Bound::{Excluded, Included, Unbounded};

    /// Const dispatch - aka fake trait
    #[doc(hidden)]
    #[allow(
        clippy::exhaustive_structs,
        missing_debug_implementations,
        reason = "This is internal"
    )]
    pub struct XBounds<'a, T>(pub &'a T);
    impl XBounds<'_, Range<Address>> {
        #[must_use]
        pub const fn bounds(&self) -> (Bound<&Address>, Bound<&Address>) {
            (Included(&self.0.start), Excluded(&self.0.end))
        }
    }
    impl XBounds<'_, RangeInclusive<Address>> {
        #[must_use]
        pub const fn bounds(&self) -> (Bound<&Address>, Bound<&Address>) {
            (Included(self.0.start()), Included(self.0.end()))
        }
    }
    impl XBounds<'_, RangeFrom<Address>> {
        #[must_use]
        pub const fn bounds(&self) -> (Bound<&Address>, Bound<&Address>) {
            (Included(&self.0.start), Unbounded)
        }
    }
    impl XBounds<'_, RangeTo<Address>> {
        #[must_use]
        pub const fn bounds(&self) -> (Bound<&Address>, Bound<&Address>) {
            (Unbounded, Excluded(&self.0.end))
        }
    }
    impl XBounds<'_, RangeToInclusive<Address>> {
        #[must_use]
        pub const fn bounds(&self) -> (Bound<&Address>, Bound<&Address>) {
            (Unbounded, Included(&self.0.end))
        }
    }
    impl XBounds<'_, RangeFull> {
        #[must_use]
        pub const fn bounds(&self) -> (Bound<&Address>, Bound<&Address>) {
            (Unbounded, Unbounded)
        }
    }

    #[macro_export]
    macro_rules! xbounds {
        ($r:expr) => {
            $crate::address::const_hax::XBounds(&$r).bounds()
        };
    }

    // Previously T was bounded by RangeBounds and there was a possibility to make it cleaner
    // in the future releases of Rust with better const-traits.
    // If it is needed, then revert the SetUnion change.
    #[doc(hidden)]
    #[allow(
        clippy::exhaustive_structs,
        missing_debug_implementations,
        reason = "This is internal"
    )]
    pub struct RangeChecker<'a, T>(pub Address, pub &'a T);

    impl RangeChecker<'_, Address> {
        #[must_use]
        pub const fn check(self) -> bool {
            self.0.0 == self.1.0
        }
    }
    macro_rules! range_checker_from_bounds {
    // Unrolling over list
    ([$($t:path),* $(,)?]) => {
        $(
        range_checker_from_bounds!{$t}
        )*
    };
    ($t:path) => {
        impl RangeChecker<'_, $t> {
            #[must_use]
            pub const fn check(self) -> bool {
                self.0.is_in_bounds_const($crate::xbounds!(*self.1))
            }
        }
    };
}

    range_checker_from_bounds! {[
        Range<Address>, RangeInclusive<Address>,
        RangeFrom<Address>, RangeTo<Address>, RangeToInclusive<Address>, RangeFull,
    ]}

    macro_rules! range_checker_product {
        ($t1:path, $t2:path) => {
            impl<'a> RangeChecker<'a, RangeUnion<'a, $t1, $t2>> {
                #[must_use]
                pub const fn check(self) -> bool {
                    RangeChecker::<'a>(self.0, self.1.0).check()
                        || RangeChecker::<'a>(self.0, self.1.1).check()
                }
            }
            impl<'a> RangeChecker<'a, RangeIntersection<'a, $t1, $t2>> {
                #[must_use]
                pub const fn check(self) -> bool {
                    RangeChecker::<'a>(self.0, self.1.0).check()
                        && RangeChecker::<'a>(self.0, self.1.1).check()
                }
            }
        };
    }

    // TODO(matrach): move it to some more generic place
    macro_rules! product {
    (@leaf [$cb:tt $($t:path) *] $t2:path, $t3:path) => {
        $cb!{$($t,)* $t2, $t3}
    };
    (@leaf $cb:tt $t1:path, $t2:path) => {
        $cb!{$t1, $t2}
    };
    // Fully unrolled one layer -- if x3, then second is still a list
    (@leaf $cb:tt $t1:path, [ $($t2:tt),*]) => {
        product!{@mult [$cb $t1] [$($t2),*], [$($t2),*]}
    };
    // Unrolled left, unrolling over right
    (@partial $cb:tt $t1:path, [$($t2:path),*] ) => {
        $(
        product!{@leaf $cb $t1, $t2}
        )*
    };
    // Unrolling over left, keep right
    (@mult $cb:tt [ $($t1:path),*], $t2:tt ) => {
        $(
        product!{@partial $cb $t1, $t2}
        )*
    };
    // Usage: macro{ callback, [ list, of, idents] }
    // Duplicate the task
    ($cb:tt, $t:tt) => {
        product!{@mult $cb $t, $t}
    };
}

    product! {range_checker_product, [
    Range<Address>, RangeInclusive<Address>, RangeFrom<Address>, RangeTo<Address>, RangeToInclusive<Address>,
    RangeUnion<'_, Range<Address>, Range<Address>>
    ]}
}

/// Const-traits replacement to check if any type of a range is a subrange of another.
#[macro_export]
macro_rules! is_subrange {
    ($container:expr, $sub_range:expr) => {
        $crate::Address::is_range_covered_const(
            $crate::xbounds!($container),
            $crate::xbounds!($sub_range),
        )
    };
}

#[macro_export]
macro_rules! static_assert_is_subrange {
    ($container:expr, $sub_range:expr) => {
        const _: () = assert!($crate::is_subrange!($container, $sub_range));
    };
}

// Macros don't allow partial match building...
// The hard part is handling _ as well as any paths/idents
// See https://stackoverflow.com/questions/47467568/recursive-macro-to-parse-match-arms-in-rust
// and https://danielkeep.github.io/tlborm/book/pat-incremental-tt-munchers.html
#[macro_export]
macro_rules! address_match_range {
    // Finally we dump the accumulated case arms
    (@pat $addr:expr, {$($arms:tt)*}, $(,)*) => {
        match $addr {
            $($arms)*
        }
    };
    // Accumulate the optional, last wildcard arm
    (@pat $addr:expr, {$($arms:tt)*}, _ => $default:expr $(,)*) => {
        $crate::address_match_range!(@pat $addr, {
            $($arms)*
            _ => $default
        },)
    };
    // Accumulate any constant-path arm "MOD::CONST => {2+2}"
    // or "MOD::CONST :if guard =>"
    // The : is required, because ``if`` cannot follow ``path`` and ``path`` cannot be used as expr.
    (@pat $addr:expr, {$($arms:tt)*}, $range:path $(:if $guard:expr)? => $value:expr, $(,)* $($tail:tt)*) => {
        $crate::address_match_range!(@pat $addr, {
            $($arms)*
            _ if $crate::address::const_hax::RangeChecker($addr, &$range).check() $(&& $guard)? => $value,
        }, $($tail)*)
    };
    {$addr:expr,  $($body:tt)* } => {{
        // Cannot write match $addr { expand_further!(...) } here,
        // since it is not in rust grammar
        $crate::address_match_range! { @pat $addr, {}, $($body)*, }
    }};
}

// We don't need to distinguish _ from path, so we don't need recursion
// It was impossible previously since we cannot simply match ($range:pat => $value:expr)
// since once it is parsed, we cannot inspect it to check for _.
// Also, we cannot leave the LL(0) parser wonder if the next token should be parsed as path or matched.
// The only alternative is to introduce some unnatural separator after expr: only ; and => is allowed.
// An alternative way was to capture a chunk of tt-s until a comma, but it also is possible
// only by recursion (thus we trade the limit on number of arms for a limit on number of tokens in an arm).
#[macro_export]
macro_rules! address_match_range_exhaustive {
    {$addr:expr,  $($range:path  $(:if $guard:expr)? => $value:expr),* $(,)*} => {
        match $addr {
            $(
                _ if $crate::address::const_hax::RangeChecker($addr, &$range).check() $(&& $guard)? => $value,
            )*
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::address::EMPTY_RANGE;
    use crate::{Address, xbounds};
    use std::ops::{Range, RangeFull};

    // assert_eq is not const!
    //noinspection RsAssertEqual
    #[test]
    const fn check_overflows() {
        // There should be no panics here!
        let a0 = Address::from_const(0);
        let a1 = Address::from_const(1);
        let last = Address::from_const(u32::MAX);

        assert!(last.offset(2).to_const() == a1.to_const());
        assert!(a1.offset_from(a0) == 1);
        assert!(a0.offset_from(a1) == u32::MAX);
    }

    #[test]
    const fn check_alignment() {
        let a = Address::from_const(16 + 4 + 1);
        assert!(!a.is_aligned_to_4_bytes());
        assert!(!a.is_aligned_to_8_bytes());

        let al2 = a.aligned_down_to_2_bytes();
        assert!(al2.is_aligned_to_4_bytes());
        assert!(!al2.is_aligned_to_8_bytes());

        let al8 = a.aligned_down_to_8_bytes();
        assert!(al8.is_aligned_to_4_bytes());
        assert!(al8.is_aligned_to_8_bytes());

        assert!(a.masked(!7).is_aligned_to_8_bytes());
    }

    #[test]
    const fn check_address_space_end() {
        let last = Address::from_const(u32::MAX);
        let range = Address::range_inclusive_from_len(42, u32::MAX - 1);
        assert!(last.is_in_bounds_const(xbounds!(range)));
    }

    #[test]
    #[should_panic(expected = "unwrap")]
    const fn check_address_space_end_exclusive() {
        let _last = Address::from_const(u32::MAX);
        let _range = Address::range_from_len(42, u32::MAX - 1);
        // assert!(last.is_in_bounds_const(xbounds!(range)));
    }

    #[test]
    const fn check_membership() {
        let a0 = Address::from_const(0);
        let a1 = Address::from_const(1);
        let last = Address::from_const(u32::MAX);

        assert!(a0.is_in_bounds_const(xbounds!(a0..a1)));
        assert!(!a1.is_in_bounds_const(xbounds!(a0..a1)));

        assert!(!a0.is_in_bounds_const(xbounds!(a0..a0)));
        assert!(a1.is_in_bounds_const(xbounds!(a0..=a1)));

        assert!(a0.is_in_bounds_const(xbounds!(..)));
        assert!(last.is_in_bounds_const(xbounds!(..)));
        assert!(last.is_in_bounds_const(xbounds!(a1..)));

        assert!(a0.is_in_bounds_const(xbounds!(..a1)));
        assert!(!a1.is_in_bounds_const(xbounds!(..a1)));

        assert!(!a1.is_in_bounds_const(xbounds!(EMPTY_RANGE)));

        assert!(!last.is_in_bounds_const(xbounds!(a1..last)));
        assert!(last.is_in_bounds_const(xbounds!(a1..=last)));
    }

    #[test]
    const fn check_subrange() {
        let a0 = Address::from_const(0);
        let a1 = Address::from_const(1);
        let last = Address::from_const(u32::MAX);

        static_assert_is_subrange!(.., EMPTY_RANGE);
        assert!(is_subrange!(.., EMPTY_RANGE));
        assert!(is_subrange!(.., ..));

        // Full ranges in discrete space
        assert!(is_subrange!(.., ..=last));
        assert!(is_subrange!(..=last, ..));
        assert!(is_subrange!(a0.., ..));
        assert!(is_subrange!(a0..=last, ..));
        assert!(!is_subrange!(a1.., ..));
        assert!(!is_subrange!(a0..last, ..));

        assert!(is_subrange!(a0..=a1, a0..a1));
        assert!(is_subrange!(a0..a1, a0..a1));
        assert!(!is_subrange!(a0..a1, a0..=a1));
        assert!(is_subrange!(a0..a1, a0..=a0));
        assert!(!is_subrange!(a1..a1, a1..=a1));

        // ops on empty ranges
        assert!(is_subrange!(a1..a1, a1..a1));
        assert!(is_subrange!(a1..a1, a1..=a0));
        assert!(is_subrange!(..a0, a0..a0));
    }

    //noinspection RsAssertEqual
    #[test]
    const fn check_match_ergonomics() {
        const RANGE11: Range<Address> = Address::range_from_len(11, 13);
        const fn map(a: Address) -> u32 {
            address_match_range!(a,
                EMPTY_RANGE => 0,
                RANGE11 => 11,
                RangeFull :if a.to_const() == 33 => 33,
                _ => 44,
            )
        }

        const fn map2(a: Address) -> Option<u32> {
            address_match_range_exhaustive!(a,
                RANGE11 => Some(11),
                RangeFull => None,
            )
        }

        assert!(map(Address::from_const(13)) == 11);
        assert!(map(Address::from_const(33)) == 33);
        assert!(map(Address::from_const(123)) == 44);

        assert!(map2(Address::from_const(13)).is_some());
        assert!(map2(Address::from_const(33)).is_none());
    }
}
