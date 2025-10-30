/// [`Pending`] is a utility type representing value that may be set by various code paths.
///
/// It is like [`Option`], but has strongly typed variants for expecting a value, not expecting it,
/// having it set, or having a value moved out.
/// Therefore, it is like [`crate::engine::flop::BufferFlop`], but it is not a flop,
/// i.e., it may be used for stuff in a single cycle, where a value may be set in one of two messages.
use std::mem;
use std::ops::Deref;

#[derive(PartialEq, Debug, Default)]
pub(crate) enum Pending<T> {
    /// Not set, not waiting for data
    #[default]
    Unset,
    /// There should be data, but it was not provided yet
    Waiting,
    /// The data was provided
    Ready(T),
    /// The data was provided, but was already moved out by a reader.
    Consumed,
}

#[allow(dead_code)]
impl<T> Pending<T> {
    fn expecting(does: bool) -> Pending<T> {
        if does {
            Pending::Waiting
        } else {
            Pending::Unset
        }
    }
    pub(crate) fn needs_data(&self) -> bool {
        matches!(self, Pending::Waiting)
    }
    pub(crate) fn is_ready(&self) -> bool {
        !matches!(self, Pending::Waiting)
    }
    pub(crate) fn is_set(&self) -> bool {
        !matches!(self, Pending::Unset | Pending::Waiting)
    }
    pub(crate) fn clear(&mut self) {
        *self = Pending::Unset;
    }
    pub(crate) fn clear_set_expecting(&mut self, expect: bool) {
        *self = Self::expecting(expect);
    }
    pub(crate) fn supply(&mut self, t: T) {
        debug_assert!(
            matches!(self, Pending::Waiting),
            "Supplied value while not waiting for it!"
        );
        *self = Pending::Ready(t);
    }
    pub(crate) fn take(&mut self) -> T {
        // perform checks
        let _ = &*self;
        mem::replace(self, Pending::Consumed).unwrap()
    }
    pub(crate) fn unwrap(self) -> T {
        if let Pending::Ready(t) = self {
            t
        } else {
            panic!("Unwrapping not ready future!")
        }
    }
    pub(crate) fn ref_or<'a>(&'a self, default: &'a T) -> &'a T {
        if self.is_set() { self } else { default }
    }
    pub(crate) fn or(&self, default: T) -> T
    where
        T: Copy,
    {
        if self.is_set() { **self } else { default }
    }

    pub(crate) fn maybe(&self) -> Option<&T> {
        self.is_set().then(|| &**self)
    }
    pub(crate) fn is_set_and(&self, f: impl FnOnce(&T) -> bool) -> bool {
        self.maybe().is_some_and(f)
    }
}

impl<T> Deref for Pending<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Pending::Ready(t) => t,
            Pending::Unset | Pending::Waiting => {
                panic!("Trying to dereference future without data!")
            }
            Pending::Consumed => panic!("The data was already consumed from the future!"),
        }
    }
}
