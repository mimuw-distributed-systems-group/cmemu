use std::ops::{Deref, DerefMut};

use enum_map::{EnumArray, EnumMap};

use crate::engine::Subcomponent;

pub(crate) struct SubcomponentProxyMut<'a, SC>
where
    SC: Subcomponent,
{
    component: &'a mut SC::Component,
}

impl<'a, SC> SubcomponentProxyMut<'a, SC>
where
    SC: Subcomponent,
{
    // trait bounds other than `Sized` on const fn parameters are unstable
    pub(crate) fn component(&self) -> &SC::Component {
        self.component
    }
    pub(crate) fn component_mut(&mut self) -> &mut SC::Component {
        self.component
    }
    pub(crate) fn this(&self) -> &SC::Member {
        SC::component_to_member(self.component)
    }
    pub(crate) fn this_mut(&mut self) -> &mut SC::Member {
        SC::component_to_member_mut(self.component)
    }

    #[allow(dead_code)]
    pub(crate) fn as_proxy<'b, OtherSC>(&'a mut self) -> SubcomponentProxyMut<'b, OtherSC>
    where
        OtherSC: Subcomponent<Component = SC::Component>,
        'a: 'b,
    {
        SubcomponentProxyMut::<'b, OtherSC> {
            component: &mut *self.component,
        }
    }
}

impl<SC> Deref for SubcomponentProxyMut<'_, SC>
where
    SC: Subcomponent,
{
    type Target = SC::Member;

    fn deref(&self) -> &Self::Target {
        self.this()
    }
}
impl<SC> DerefMut for SubcomponentProxyMut<'_, SC>
where
    SC: Subcomponent,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.this_mut()
    }
}

impl<'a, SC> From<&'a mut SC::Component> for SubcomponentProxyMut<'a, SC>
where
    SC: Subcomponent,
{
    fn from(component: &'a mut SC::Component) -> Self {
        Self { component }
    }
}

#[cfg(all(feature = "paranoid", not(feature = "no_paranoid_override")))]
#[macro_export]
macro_rules! paranoid {
    ($cb:ident, $($tt:tt)*) => {
        panic!($($tt)*);
    }
}
#[cfg(any(not(feature = "paranoid"), feature = "no_paranoid_override"))]
#[macro_export]
macro_rules! paranoid {
    ($cb:ident, $($tt:tt)*) => {
        ::log::$cb!($($tt)*);
    }
}

pub(crate) fn iter_enum<E: EnumArray<()>>() -> impl Iterator<Item = E> {
    EnumMap::<E, ()>::default().into_iter().map(|(k, ())| k)
}
pub(crate) fn iter_enum_t<E, T>() -> impl Iterator<Item = E>
where
    E: EnumArray<T>,
    T: Copy + Default,
{
    EnumMap::<E, T>::default().into_iter().map(|(k, _)| k)
}

/// This is a workaround for people using the same method for `read_for_filler` and read with side effects
///
/// You can always ``deref`` it, but there is ``get_mut() -> Option<_>`` instead of ``deref_mut``.
/// Rust 1.84 has no concept of higher-order-over-mutability like ``for<'a>`` for lifetimes.
pub(crate) enum MaybeMut<'a, T> {
    Ref(&'a T),
    Mut(&'a mut T),
}

impl<T> Deref for MaybeMut<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        match self {
            Self::Ref(t) => t,
            Self::Mut(t) => t,
        }
    }
}

impl<C> MaybeMut<'_, C> {
    pub(crate) fn get_mut(&mut self) -> Option<&mut C> {
        match self {
            MaybeMut::Ref(_) => None,
            MaybeMut::Mut(t) => Some(t),
        }
    }

    /// Project ``MaybeMut<Component>`` into a borrowed ``MaybeMut<Subcomponent>``.
    pub(crate) fn project<S>(&mut self) -> MaybeMut<'_, S>
    where
        S: Subcomponent<Component = C, Member = S>,
    {
        match self {
            Self::Ref(t) => MaybeMut::Ref(S::component_to_member(t)),
            Self::Mut(t) => MaybeMut::Mut(S::component_to_member_mut(t)),
        }
    }
}

/// Convert a marker-type into a value.
///
/// This trait is like `std::From<M>`, but it is unsuitable for marker types, since
/// it wants a value:
/// - never-enum style markers cannot create a value,
/// - unit-struct style markers cannot be created from thin air in generic context
///   (without implementing `Default` for them, which leads to ugly code).
pub trait FromMarker<M> {
    fn from_marker() -> Self;
    const MARKER_NAME: &'static str = "?";
}
