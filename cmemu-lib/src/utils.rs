use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;

/// Make the expression `{}` printable by any means (with sane fallbacks).
///
/// Use macro `printable!(expr)` in a formatter context like `println!("{}", printable!(&&&&2));`
/// It will attempt to print the expression with the best type-information in has at the call-point,
/// while trying to dereference the expression.
/// In the case of no Display/Debug implementation, it will fall back to
/// printing the call expr and type name.
///
/// Read more about the technique here: <http://lukaskalbertodt.github.io/2019/12/05/generalized-autoref-based-specialization.html>
/// In short, the more `&` an impl has, the higher priority it has during auto-dereferencing.
#[macro_export]
macro_rules!
printable {
    ($e: expr) => {{
        use $crate::utils::Printerize as _;
        (&&&&&&&&&&$crate::utils::Printable(&$e, stringify!($e))).displayer()
    }};
}
#[doc(hidden)]
pub struct Printable<'a, T: ?Sized>(pub &'a T, pub &'static str);

/// Halper struct as we cannot implement Display on `&T`
#[doc(hidden)]
pub trait Printerize: Sized {
    fn displayer(self) -> Displayer<impl Fn(&mut Formatter) -> std::fmt::Result> {
        Displayer(|f| f.write_str("?cannot_print_this?"))
    }
}

impl<'a, T> Printerize for &Printable<'a, T> {
    #[allow(refining_impl_trait, reason = "Lifetime of temporary self")]
    fn displayer(self) -> Displayer<impl (Fn(&mut Formatter) -> std::fmt::Result) + use<'a, T>> {
        let x = self.1;
        Displayer(move |f| write!(f, "{}: {}", x, std::any::type_name::<T>()))
    }
}

impl<'a, T> Printerize for &&Printable<'a, T>
where
    T: Deref,
    <T as Deref>::Target: Deref + Sized,
    <<T as Deref>::Target as Deref>::Target: Debug,
{
    #[allow(refining_impl_trait, reason = "Lifetime of temporary self")]
    fn displayer(self) -> Displayer<impl (Fn(&mut Formatter) -> std::fmt::Result) + use<'a, T>> {
        let x = self.0;
        Displayer(move |f| write!(f, "{:?}", x.deref().deref()))
    }
}

impl<'a, T> Printerize for &&&Printable<'a, T>
where
    T: Deref,
    <T as Deref>::Target: Debug,
{
    #[allow(refining_impl_trait, reason = "Lifetime of temporary self")]
    fn displayer(self) -> Displayer<impl (Fn(&mut Formatter) -> std::fmt::Result) + use<'a, T>> {
        let x = self.0;
        Displayer(move |f| write!(f, "{:?}", &**x))
    }
}

impl<'a, T> Printerize for &&&&Printable<'a, T>
where
    T: Deref,
    <T as Deref>::Target: Display,
{
    #[allow(refining_impl_trait, reason = "Lifetime of temporary self")]
    fn displayer(self) -> Displayer<impl (Fn(&mut Formatter) -> std::fmt::Result) + use<'a, T>> {
        let x = self.0;
        Displayer(move |f| write!(f, "{:}", &**x))
    }
}

impl<'a, T: Debug> Printerize for &&&&&Printable<'a, T> {
    #[allow(refining_impl_trait, reason = "Lifetime of temporary self")]
    fn displayer(self) -> Displayer<impl (Fn(&mut Formatter) -> std::fmt::Result) + use<'a, T>> {
        let x = self.0;
        Displayer(move |f| write!(f, "{x:?}"))
    }
}

impl<'a, T: Display> Printerize for &&&&&&Printable<'a, T> {
    #[allow(refining_impl_trait, reason = "Lifetime of temporary self")]
    fn displayer(self) -> Displayer<impl (Fn(&mut Formatter) -> std::fmt::Result) + use<'a, T>> {
        let x = self.0;
        Displayer(move |f| write!(f, "{x}"))
    }
}

impl<'a> Printerize for &&&&&&&Printable<'a, str> {
    #[allow(refining_impl_trait, reason = "Lifetime of temporary self")]
    fn displayer(self) -> Displayer<impl (Fn(&mut Formatter) -> std::fmt::Result) + use<'a>> {
        let x = self.0;
        Displayer(move |f| f.write_str(x))
    }
}

// var_print
impl<T: Display> Printerize for Option<T> {
    fn displayer(self) -> Displayer<impl Fn(&mut Formatter) -> std::fmt::Result> {
        Displayer(move |f| {
            if let Some(val) = self.as_ref() {
                write!(f, "{val}")
            } else {
                f.write_str("None")
            }
        })
    }
}

/// A delayed displayer that could hold a closure calling a display trait
#[allow(dead_code)] // used by behind some random gates
#[repr(transparent)]
pub struct Displayer<F: Fn(&mut Formatter) -> std::fmt::Result>(pub F);
impl<F: Fn(&mut Formatter) -> std::fmt::Result> Display for Displayer<F> {
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0(f)
    }
}

pub enum VarDisplay<A: Display, B: Display>
where
    Self: Display,
{
    A(A),
    B(B),
}

impl<A: Display, B: Display> Display for VarDisplay<A, B> {
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::A(a) => Display::fmt(a, f),
            Self::B(b) => Display::fmt(b, f),
        }
    }
}

pub trait DisplayOr<T = Self>
where
    T: Display,
{
    fn display_or<D>(&self, default: impl FnOnce() -> D) -> VarDisplay<&T, D>
    where
        D: Display;

    #[allow(dead_code)]
    fn display_or_default<D>(&self, default: D) -> VarDisplay<&T, D>
    where
        D: Display,
    {
        self.display_or(|| default)
    }
}

impl<T: Display> DisplayOr<T> for Option<T> {
    #[inline(always)]
    fn display_or<D>(&self, default: impl FnOnce() -> D) -> VarDisplay<&T, D>
    where
        D: Display,
    {
        if let Some(v) = self {
            VarDisplay::A(v)
        } else {
            VarDisplay::B(default())
        }
    }
}

#[allow(dead_code)]
pub fn dife_lazy<C: IfExpr, T: Display, F: Display>(
    cond: C,
    t: impl FnOnce() -> T,
    f: impl FnOnce() -> F,
) -> VarDisplay<T, F> {
    if cond.ife(true, false) {
        VarDisplay::A(t())
    } else {
        VarDisplay::B(f())
    }
}

#[allow(dead_code)]
pub fn dife<C: IfExpr, T: Display, F: Display>(cond: C, t: T, f: F) -> VarDisplay<T, F> {
    ife(cond, VarDisplay::A(t), VarDisplay::B(f))
}

// Implication testing code
pub trait Wrapper: Sized {
    type Unwrapped;

    #[allow(dead_code)]
    fn wrap(x: Self::Unwrapped) -> Self;
}

#[cfg_attr(not(debug_assertions), allow(unused))]
pub trait Implies<U, T>: Wrapper
where
    U: Wrapper,
    T: Wrapper,
{
    fn expect_implies_then<F>(self, other: U, f: F) -> T
    where
        F: FnOnce(Self::Unwrapped, <U as Wrapper>::Unwrapped) -> <T as Wrapper>::Unwrapped,
    {
        self.implies_then(other, f, |_| panic!("Implication violated True => False"))
    }

    fn implies_then<F, G>(self, other: U, f: F, fail: G) -> T
    where
        F: FnOnce(Self::Unwrapped, <U as Wrapper>::Unwrapped) -> <T as Wrapper>::Unwrapped,
        G: FnOnce(Self::Unwrapped) -> T;
}

impl<T> Wrapper for Option<T> {
    type Unwrapped = T;
    fn wrap(x: Self::Unwrapped) -> Self {
        Some(x)
    }
}

impl Wrapper for bool {
    type Unwrapped = ();
    fn wrap((): Self::Unwrapped) -> Self {
        true
    }
}

impl Wrapper for () {
    type Unwrapped = ();
    fn wrap((): Self::Unwrapped) -> Self {}
}

impl<T> Implies<bool, Option<T>> for bool
where
    Option<T>: Wrapper<Unwrapped = T>,
{
    fn implies_then<F, G>(self, other: bool, f: F, fail: G) -> Option<T>
    where
        F: FnOnce((), ()) -> T,
        G: FnOnce(()) -> Option<T>,
    {
        self.then(|| other.then(|| f((), ())))
            .and_then(|o| o.or_else(|| fail(())))
    }
}

impl<S, U, T> Implies<Option<U>, Option<T>> for Option<S>
where
    S: Debug,
    Self: Wrapper<Unwrapped = S>,
    Option<U>: Wrapper<Unwrapped = U>,
    Option<T>: Wrapper<Unwrapped = T>,
{
    fn expect_implies_then<F>(self, other: Option<U>, f: F) -> Option<T>
    where
        F: FnOnce(S, U) -> T,
    {
        self.implies_then(other, f, |s| panic!("Implication violated {s:?} => None"))
    }

    fn implies_then<F, G>(self, other: Option<U>, f: F, fail: G) -> Option<T>
    where
        F: FnOnce(S, U) -> T,
        G: FnOnce(S) -> Option<T>,
    {
        if let Some(s) = self {
            if let Some(o) = other {
                Some(f(s, o))
            } else {
                fail(s)
            }
        } else {
            None
        }
    }
}

pub trait IfExpr {
    fn ife<T>(&self, t: T, f: T) -> T {
        if self.as_bool() { t } else { f }
    }
    fn as_bool(&self) -> bool;
    fn or_err<E>(&self, e: E) -> Result<&Self, E> {
        self.ife(Ok(self), Err(e))
    }
    fn implies<T: IfExpr>(&self, other: T) -> bool {
        self.ife(other.as_bool(), true)
    }
}

impl IfExpr for bool {
    fn as_bool(&self) -> bool {
        *self
    }
}

impl<X> IfExpr for Option<X> {
    fn as_bool(&self) -> bool {
        self.is_some()
    }
}

impl<X> IfExpr for &Option<X> {
    fn as_bool(&self) -> bool {
        self.is_some()
    }
}

impl<X, Y> IfExpr for Result<X, Y> {
    fn as_bool(&self) -> bool {
        self.is_ok()
    }
}

pub fn ife<I, T>(cond: I, t: T, f: T) -> T
where
    I: IfExpr,
{
    cond.ife(t, f)
}

/// Downcast generic 'static values/references that is optimized out in monomorphisation + opt-level>0
#[macro_export]
macro_rules! static_downcast {
    ($expr:expr, { _ => $res:expr $(,)?}) => {
        $res
    };
    ($expr:expr, { $(,)?} ) => {
        ()
    };
    ($expr:ident, { $pat:ty => $res:expr $(, $($rest:tt)*)? }) => {
        if let Some($expr) = $crate::static_downcast!($expr, $pat) {
            $res
        } else {
            $crate::static_downcast!($expr, { $($($rest)*)?})
        }
    };
    // special case to not lose type of ident (and make consistent api)
    ($expr:ident, { $pat:pat_param => $res:expr $(, $($rest:tt)*)? }) => {
        if let Some($expr @ $pat) = $crate::static_downcast!($expr) {
            $res
        } else {
            $crate::static_downcast!($expr, { $($($rest)*)? })
        }
    };
    ($expr:expr, { $pat:pat_param => $res:expr $(, $($rest:tt)*)? }) => {
        if let Some($pat) = $crate::static_downcast!($expr) {
            $res
        } else {
            $crate::static_downcast!($expr, { $($($rest)*)? })
        }
    };
    ($expr:expr, $typ:ty) => {
        (&$expr as &dyn std::any::Any).downcast_ref::<$typ>()
    };
    ($expr:expr) => {
        (&$expr as &dyn std::any::Any).downcast_ref::<_>()
    };
}

/// A function shortcut to pass instead of writing `|r| *r` repeatedly.
pub fn deref<T>(r: T) -> <T as Deref>::Target
where
    T: Deref,
    <T as Deref>::Target: Sized + Copy,
{
    *r
}

/// Sometimes the order of dispatch cannot be assumed.
/// Use this macro to test both ordering of two blocks of code.
#[macro_export]
macro_rules! mix_blocks {
    {$reorder:expr, $first:stmt, $second:stmt} => {{
        if $reorder {$second $first} else {$first $second}
    }};
}
