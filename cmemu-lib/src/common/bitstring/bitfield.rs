#![allow(clippy::elidable_lifetime_names)]

use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

pub trait ExpandedBitfield: Sized {
    // Note: The Clone is here right now only to mean "plain old struct",
    // so we don't need ``unsafe`` to write ExpandedViewMut without Option's overhead
    /// An expanded version of a bitfield struct is a native (unpacked) Rust struct
    type Expanded: Sized + Debug + Clone;

    /// Expand the bitfield into a directly accessible struct, that represents a copy.
    #[must_use]
    fn expand_copy(&self) -> Self::Expanded;

    /// Update the bitfield from information stored in the expanded version
    ///
    /// If the expanded version is not covering some bits, these must not be modified.
    /// If the expanded version has aliased bit ranges, the behavior is not specified here.
    fn update_from(&mut self, expanded: Self::Expanded);

    /// "In-place" mutate the bitfield by mutating the expanded version.
    ///
    /// Note: the mutation is transactional -- that is, the actual change occurs when this
    /// function returns (and doesn't if the function doesn't return, including unwinding).
    #[inline(always)]
    fn mutate(&mut self, f: impl FnOnce(&mut Self::Expanded)) {
        // This version implicitly doesn't handle unwinding to limit inlining performance penalty...
        // we call it "transactional mutation" - it should compile to native mutation after opts.
        let mut expanded = self.expand_copy();
        f(&mut expanded);
        self.update_from(expanded);

        // Correct version:
        // let mut expanded_view = self.expanded_mut();
        // f(&mut *expanded_view);
    }

    /// Expand the bitfield into a directly accessible **view** struct.
    ///
    /// The view cannot become desynchronized with the bitfield, because it borrows ``&self``.
    #[inline(always)]
    #[must_use]
    fn expanded(&self) -> ExpandedView<'_, Self> {
        ExpandedView {
            expanded: self.expand_copy(),
            original: PhantomData, // we care only about capturing the reference's lifetime
        }
    }

    /// Expand the bitfield into a directly accessible **mutable view** struct.
    ///
    /// The view borrows ``&mut self`` to exclude concurrent accesses to the backing bitfield,
    /// and updates it in ``Drop``.
    #[inline(always)]
    #[must_use]
    fn expanded_mut(&mut self) -> ExpandedViewMut<'_, Self> {
        ExpandedViewMut {
            expanded: self.expand_copy(),
            original: self,
        }
    }
}

#[derive(Debug)]
pub struct ExpandedView<'a, B>
where
    B: ExpandedBitfield,
{
    expanded: <B as ExpandedBitfield>::Expanded,
    original: PhantomData<&'a B>,
}

impl<'a, B> Deref for ExpandedView<'a, B>
where
    B: ExpandedBitfield,
{
    type Target = <B as ExpandedBitfield>::Expanded;
    fn deref(&self) -> &Self::Target {
        &self.expanded
    }
}

// We need two structs, because Drop cannot be specialized!
#[derive(Debug)]
pub struct ExpandedViewMut<'a, B>
where
    B: ExpandedBitfield,
{
    expanded: <B as ExpandedBitfield>::Expanded,
    original: &'a mut B,
}

impl<'a, B> Deref for ExpandedViewMut<'a, B>
where
    B: ExpandedBitfield,
{
    type Target = <B as ExpandedBitfield>::Expanded;
    fn deref(&self) -> &Self::Target {
        &self.expanded
    }
}
impl<'a, B> DerefMut for ExpandedViewMut<'a, B>
where
    B: ExpandedBitfield,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.expanded
    }
}

impl<'a, B> Drop for ExpandedViewMut<'a, B>
where
    B: ExpandedBitfield,
{
    #[inline(always)]
    fn drop(&mut self) {
        self.original.update_from(self.expanded.clone());
    }
}

/// Wrapper macro for making strongly typed bitfields using our bitstring primitives.
///
/// The macro will create a new-type wrapper over `Bitstring![N]` with methods for extraction/mutation
/// of the bitfields using `bitstring_extract`/`bitstring_substitute` macros under the hood.
/// Moreover, our bitfields offer *expanded views* to work on the bitfield using native
/// unpacked Rust struct.
/// The macro automatically derives a `Debug` implementation.
///
/// # Example
/// Here is an example with syntax for reference:
///
/// ```ignore
/// crate::bitfield! {
/// #[derive(Clone, Copy)]
/// pub struct Test[32] {
///     /// Comment, attribute
///     XTR[31:30]: 2 bits,
///     Bt[15:15]: 1 bits,
///     pub a[4:2]: 3 bits,
/// }}
/// ```
///
/// Will generate a ``pub struct Test(Bitfield![32])`` base structure and expanded version of:
/// ```ignore
/// pub struct TestExpanded {
///     XTR: Bitstring![2],
///     Bt: Bitstring![1],
///     a: Bitstring![3],
/// }
/// ```
///
/// The ``Test`` structure will have methods for each field with names including the field:
/// ```ignore
/// impl Test {
///     fn FIELD(&self) -> Bitstring![N];
///     fn set_FIELD(&mut self, val: Bitstring![N]);
///     fn with_FIELD(self, val: Bitstring![N]) -> Self;
/// }
/// ```
///
/// Moreover, single-bit fields have methods working on bools:
/// ```ignore
/// impl Test {
///     fn get_FIELD_bit(&self) -> bool;
///     fn with_FIELD_bit(self, val: bool) -> Self;
/// }
/// ```
///
/// The expanded views use [`ExpandedBitfield`] for lifetime-bounded views which guarantee
/// consistency with the backing bitfield. You can write, for instance:
/// ```ignore
/// fn do_stuff(t: &mut Test) {
///     let exp = t.expanded_mut();
///     exp.Bt = true.into();
///     // t.set_a(C_111); -- won't borrow check!
///     exp.a = C_111;
/// }
/// ```
/// Check [`ExpandedBitfield::expanded`], [`ExpandedBitfield::expanded_mut`], [`ExpandedBitfield::mutate`],
/// and [`ExpandedBitfield::expand_copy`] for details.
#[macro_export]
macro_rules! bitfield {
    {
        $(#[$attr:meta])* $vis:vis struct $name:ident[$width:tt $((raw $raw_vis:vis))?] {
        $(
        $(#[$field_attr:meta])*
        $fvis:vis $field:ident [$hi:literal : $lo:literal] : $N:literal bits
        ),+ $(,)?
    }} => {
        $(#[$attr])*
        #[repr(transparent)]
        $vis struct $name($($raw_vis)? $crate::Bitstring![$width]);

        paste::paste! {
        #[allow(non_snake_case, unused)]
        impl $name {
            $(
                $(#[$field_attr])*
                #[inline(always)]
                #[must_use]
                $fvis fn $field(&self) -> $crate::Bitstring![$N] {
                    $crate::bitstring_extract!((self.0) <$hi: $lo> | $N bits)
                }

                #[inline(always)]
                $fvis fn [<set_$field>](&mut self, bits: $crate::Bitstring![$N])  {
                    let mut ident = self.0;
                    $crate::bitstring_substitute!(ident <$hi: $lo> = bits);
                    self.0 = ident;
                }

                #[inline(always)]
                #[must_use]
                $fvis fn [<with_$field>](self, bits: $crate::Bitstring![$N]) -> Self
                where Self: Copy {
                    let mut ident = self.0;
                    $crate::bitstring_substitute!(ident <$hi: $lo> = bits);
                    Self(ident)
                }

                #[allow(unused_lifetimes, reason = "see comment below this")]
                #[inline(always)]
                #[must_use]
                $fvis fn [<with_ $field _bit>](self, bit: bool) -> Self
                // That's an ugly hack to make trivial bounds to make this method present only for N==1
                where for<'a> $crate::Bitstring![$N]: From<bool>  {
                    use $crate::common::bitstring::BitstringUtils;
                    #[allow(clippy::eq_op)]
                    const { assert!($hi == $lo); }
                    Self(self.0.with_bit_set($hi, bit))
                }

                #[allow(unused_lifetimes, reason = "see comment below this")]
                #[inline(always)]
                #[must_use]
                $fvis fn [<get_ $field _bit>](&self) -> bool
                // That's an ugly hack to make trivial bounds to make this method present only for N==1
                where for<'a> $crate::Bitstring![$N]: From<bool>  {
                    use $crate::common::bitstring::BitstringUtils;
                    #[allow(clippy::eq_op)]
                    const { assert!($hi == $lo); }
                    self.0.get_bit($hi)
                }
            )+
        }
        impl $crate::common::bitstring::bitfield::ExpandedBitfield for $name {
            type Expanded = [<$name Expanded>];

            #[inline(always)]
            fn expand_copy(&self) -> [<$name Expanded>] {
                [<$name Expanded>] {
                    $(
                    $field: Self::$field(&self),
                    )+
                }
            }

            #[inline(always)]
            fn update_from(&mut self, expanded: Self::Expanded) {
                $(
                self.[<set_$field>](expanded.$field);
                )+
            }

        }
        #[allow(non_snake_case, unused)]
        #[derive(Debug, Clone)]
        $vis struct [<$name Expanded>]  {
            $(
            $fvis $field: $crate::Bitstring![$N],
            )+
        }

        impl std::fmt::Debug for $name
        {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{} {{", stringify!($name))?;
                if f.alternate() {f.write_str("\n")?}
                write!(f, " #raw#: ")?;
                std::fmt::Debug::fmt(&self.0, f)?;
                if f.alternate() {f.write_str("\n")?}
                write!(f, " #whole#: {:#b},", self.0)?;
                if f.alternate() {f.write_str("\n")?}
                $(
                write!(f, " {}: {:#b},", stringify!($field), self.$field())?;
                if f.alternate() {f.write_str("\n")?}
                )+
                write!(f, "}}")
            }
        }

        }
    }
}
