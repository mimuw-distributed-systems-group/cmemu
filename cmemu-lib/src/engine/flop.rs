use owo_colors::OwoColorize;
#[cfg(debug_assertions)]
use std::cell::Cell;
use std::marker::PhantomData;
use std::ops::Deref;
use std::{fmt, mem};

/// In digital electronics, flip flops (registers) behaves like memory cells
/// that changes theirs value on clock ticks (tick = next cycle begins).
/// This structure emulates them.
///
/// You can dereference a flop to read value set in previous cycle.
/// You can also set value that will be available in the next cycle.
///
/// Moreover, in debug build, this structure contains extra checks to make
/// sure the flop is used as intended: every set value should be either
/// used XOR ignored in the following cycle.
///
/// ## Combinational flops (`CombFlop`) vs sequential flops (`SeqFlop`)
///
/// Most likely `SeqFlop` is what you need.
///
/// **Important note**: every instance of `CombFlop` **must** have a comment
///                     explaining why it must be `CombFlop` (containing
///                     dependencies on events from other components).
///
/// Name is inspired by combinational and sequential circuits.
/// When component ticks, it should set some values for the next cycle.
/// If a value (flop) depends only on values from the component itself,
/// the component can calculate and set the value once.
/// However, some value can depend on events from other components.
/// If we know that given event arrives at most once, we can set
/// the value at most once.
/// Otherwise, we might need to overwrite the flop multiple times.
/// For this very specific case, there is `CombFlop` that lifts
/// the check of setting a value at most once that is present in `SeqFlop`.
///
/// ## Difference with `FlopMemoryBank`
/// `Flop` is meant for temporary variables that must be used in a cycle later
/// after setting the value, then the value is erased.
/// `FlopMemoryBank`, on the other hand, is a memory - it always has a value.
pub(crate) struct Flop<T, M> {
    // note: big fields earlier, small fields later - save memory on padding
    // TODO: it would be way efficient if we did the flipping globally instead of locally.
    // That is, we could use the offset matching parity of the current cycle.
    // Such approach would allow a lot of optimizations of the machine code.
    cur: Option<T>,
    next: Option<T>,
    keep_value: bool,
    type_marker: PhantomData<M>,
    #[cfg(debug_assertions)]
    was_read: Cell<bool>,
    #[cfg(debug_assertions)]
    was_ignored: Cell<bool>,
}

pub(crate) struct SeqFlopMarker;
pub(crate) struct BufferFlopMarker;
pub(crate) struct CombFlopMarker;
pub(crate) struct LatchFlopMarker;

pub(crate) type SeqFlop<T> = Flop<T, SeqFlopMarker>;
pub(crate) type CombFlop<T> = Flop<T, CombFlopMarker>;
pub(crate) type LatchFlop<T> = Flop<T, LatchFlopMarker>;

/// A helper type that check valid order of usage and remembers previous cycle data
pub(crate) type BufferFlop<T> = Flop<T, BufferFlopMarker>;

impl<T, M> Flop<T, M> {
    pub(crate) fn new() -> Self {
        Self {
            cur: None,
            next: None,
            keep_value: false,
            type_marker: PhantomData,
            #[cfg(debug_assertions)]
            was_read: Cell::new(false),
            #[cfg(debug_assertions)]
            was_ignored: Cell::new(false),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_from(value: T) -> Self {
        Self {
            cur: None,
            next: Some(value),
            keep_value: false,
            type_marker: PhantomData,
            #[cfg(debug_assertions)]
            was_read: Cell::new(false),
            #[cfg(debug_assertions)]
            was_ignored: Cell::new(false),
        }
    }

    #[track_caller]
    pub(crate) fn is_set(&self) -> bool {
        #[cfg(debug_assertions)]
        assert!(
            self.cur.is_some() || (self.cur.is_none() && !self.was_read.get()),
            "Flop value was taken."
        );
        self.cur.is_some()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.cur.is_none() && self.next.is_none()
    }

    #[cfg_attr(not(feature = "cycle-debug-logger"), allow(dead_code))]
    #[track_caller]
    pub(crate) fn ignore(&mut self) {
        let _ = self; // unused in release build
        #[cfg(debug_assertions)]
        {
            assert!(!self.was_read.get(), "Ignoring value of already used flop.");
            self.was_ignored.set(true);
        }
    }

    // Note: we don't want to provide as_option() method since it could be
    // misused this way: `as_option().is_some()` without actually reading
    // the value - and we want to be able to track explicit intentional reads.
    // Providing `map_or` method is sound: we are actually interested in
    // the value if it is present.
    #[track_caller]
    pub(crate) fn map_or<'a, U, F>(&'a self, default: U, f: F) -> U
    where
        F: FnOnce(&'a T) -> U,
    {
        if self.is_set() { f(&**self) } else { default }
    }

    // This is like the previous, but implement the common use-case mirroring Rust trends.
    #[allow(dead_code)]
    #[track_caller]
    pub(crate) fn map_option<'a, U, F>(&'a self, f: F) -> Option<U>
    where
        F: FnOnce(&'a T) -> U,
    {
        self.map_or(None, |x| Some(f(x)))
    }
    // is_some_and will be stable soon
    #[track_caller]
    pub(crate) fn is_set_and<'a, F>(&'a self, f: F) -> bool
    where
        F: FnOnce(&'a T) -> bool,
    {
        self.map_or(false, f)
    }

    #[track_caller]
    pub(crate) fn deref_or<'a, 'b>(&'a self, default: &'b T) -> &'b T
    where
        'a: 'b,
    {
        if self.is_set() { self } else { default }
    }

    #[allow(dead_code)]
    #[track_caller]
    pub(crate) fn get_or(&self, default: T) -> T
    where
        T: Copy,
    {
        *self.deref_or(&default)
    }

    // Note: for better error messages, method takes & uses flop name
    //       in debug mode. The method itself is called from autogenerated code
    //       and the call stack does not help.
    pub(crate) fn tick(&mut self, #[cfg(debug_assertions)] my_name: &str) {
        #[cfg(debug_assertions)]
        {
            assert!(
                self.cur.is_none() || self.was_read.get() || self.was_ignored.get(),
                "Flop \"{my_name}\": value not used in previous cycle."
            );
            self.was_read.set(false);
            self.was_ignored.set(false);
        }
        if self.keep_value {
            debug_assert!(self.cur.is_some(), "Keeping value of unset flop");
            self.keep_value = false;
            self.next = None;
        } else {
            drop(mem::replace(&mut self.cur, self.next.take()));
        }
    }

    pub(crate) fn is_next_set(&self) -> bool {
        self.keep_value || self.next.is_some()
    }

    /// Peek the next value - use is discouraged unless for debug/special checks.
    #[track_caller]
    pub(crate) fn peek_next(&self) -> &T {
        assert!(self.is_next_set(), "Peeking flop without set next value.");
        if self.keep_value {
            self
        } else {
            self.next.as_ref().unwrap()
        }
    }

    /// Try to evaluate a func on the next value - use is discouraged unless for debug/special checks.
    #[track_caller]
    pub(crate) fn is_next_set_and<'a, F>(&'a self, f: F) -> bool
    where
        F: FnOnce(&'a T) -> bool,
    {
        if self.is_next_set() {
            f(self.peek_next())
        } else {
            false
        }
    }

    #[track_caller]
    pub(crate) fn take(&mut self) -> T {
        #[cfg(debug_assertions)]
        {
            // if the data was taken, was_read is true, but is_some false
            assert!(
                self.cur.is_some() || (self.cur.is_none() && !self.was_read.get()),
                "Flop value was taken."
            );
        }
        // This will handle panics in case of not setting values etc.
        let _ = self.deref().deref();
        self.cur.take().unwrap()
    }

    #[track_caller]
    pub(crate) fn try_take(&mut self) -> Option<T> {
        self.is_set().then(|| self.take())
    }
}

macro_rules! impl_flop_next_for {
    ($marker_type:ty) => {
        impl<T> Flop<T, $marker_type> {
            #[track_caller]
            pub(crate) fn set_next(&mut self, value: T) {
                assert!(!self.is_next_set(), "Setting flop that is already set.");
                self.next = Some(value);
            }

            /// This works like an alias on ``flop.set_next(*flop.clone())``,
            /// but without requiring that ``T: clone``.
            #[cfg_attr(not(feature = "cycle-debug-logger"), allow(dead_code))]
            #[track_caller]
            pub(crate) fn keep_current_as_next(&mut self) {
                #[cfg(debug_assertions)]
                {
                    assert!(!self.was_ignored.get(), "Using value of ignored flop.");
                    self.was_read.set(true);
                }
                assert!(!self.is_next_set(), "Setting flop that is already set.");
                assert!(
                    self.is_set(),
                    "Keeping current value of flop that is not set."
                );
                self.keep_value = true;
            }
        }
    };
}

impl_flop_next_for!(SeqFlopMarker);
impl_flop_next_for!(BufferFlopMarker);

#[allow(dead_code)]
impl<T> Flop<T, BufferFlopMarker> {
    pub(crate) fn allow_skip(&self) {
        let _ = self.try_prev_cycle();
    }
    // Alternative nomenclature
    pub(crate) fn get_this_cycle(&self) -> &T {
        self.peek_next()
    }

    pub(crate) fn set_this_cycle(&mut self, val: T) {
        self.set_next(val);
    }

    pub(crate) fn has_this_cycle(&self) -> bool {
        self.is_next_set()
    }

    #[track_caller]
    pub(crate) fn try_this_cycle(&self) -> Option<&T> {
        self.has_this_cycle().then(|| self.peek_next())
    }

    #[track_caller]
    pub(crate) fn get_prev_cycle(&self) -> &T {
        self
    }

    pub(crate) fn try_prev_cycle(&self) -> Option<&T> {
        self.has_prev_cycle().then(|| &**self)
    }

    pub(crate) fn has_prev_cycle(&self) -> bool {
        self.is_set()
    }
}

impl<T> Flop<T, CombFlopMarker> {
    /// Set next value. If one exists, it is overriden.
    #[track_caller]
    pub(crate) fn set_next(&mut self, value: T) {
        self.next = Some(value);
        self.keep_value = false;
    }

    /// Get a mutable reference to the value stored as next.
    /// Panics if not present.
    #[track_caller]
    pub(crate) fn get_next_mut(&mut self) -> &mut T {
        debug_assert!(
            self.is_next_set() && !self.keep_value,
            "No next value to get reference to!"
        );
        self.next.as_mut().expect("Next data not set")
    }

    /// Get a mutable reference to the value stored as next,
    /// possibly initializing it with ``T::default()`` if not present.
    #[track_caller]
    pub(crate) fn next_builder(&mut self) -> &mut T
    where
        T: Default,
    {
        if !self.is_next_set() {
            self.set_next(T::default());
        }
        self.get_next_mut()
    }

    /// Set (or override) next value unless the present one come from ``keep_current_as_next`` family.
    #[track_caller]
    pub(crate) fn set_next_if_not_latching(&mut self, value: T) {
        if self.keep_value {
            return;
        }
        self.next = Some(value);
    }

    /// Set next value only if not present.
    #[track_caller]
    pub(crate) fn set_default_next(&mut self, value: T) {
        if !self.is_next_set() {
            self.next = Some(value);
            self.keep_value = false;
        }
    }

    #[track_caller]
    pub(crate) fn default_keep_current_as_next(&mut self)
    where
        T: Clone,
    {
        // NOTE: the interplay with defaults and set_next_if_not_latching is complex
        // and we have too few data points to encode the situation
        if !self.is_next_set() && self.is_set() {
            self.set_default_next(self.deref().deref().clone());
        }
    }

    /// This works like an alias on ``flop.set_next(*flop.clone())``,
    /// but without requiring that ``T: clone``.
    #[allow(dead_code)]
    #[track_caller]
    pub(crate) fn keep_current_as_next(&mut self) {
        #[cfg(debug_assertions)]
        {
            assert!(!self.was_ignored.get(), "Using value of ignored flop.");
            self.was_read.set(true);
            assert!(
                self.is_set(),
                "Keeping current value of flop that is not set."
            );
        }
        self.next = None;
        self.keep_value = true;
    }

    /// Clear the next value slot to the state like just after ``flop.tick()``.
    #[allow(dead_code)]
    #[track_caller]
    pub(crate) fn unset_next(&mut self) {
        self.next = None;
        self.keep_value = false;
    }
}

impl<T> Flop<T, LatchFlopMarker> {
    #[track_caller]
    pub(crate) fn set_next(&mut self, value: T) {
        // #[cfg(any(debug_assertions, feature = "paranoid"))]
        assert!(
            self.next.is_none(),
            "Setting latch flop proposal for a second time."
        );
        self.next = Some(value);
    }

    #[track_caller]
    pub(crate) fn keep_current_as_next(&mut self) {
        #[cfg(debug_assertions)]
        {
            assert!(!self.was_ignored.get(), "Using value of ignored flop.");
            self.was_read.set(true);
            assert!(
                self.is_set(),
                "Keeping current value of flop that is not set."
            );
        }
        self.keep_value = true;
    }

    // Alternative name
    #[allow(dead_code)]
    #[track_caller]
    pub(crate) fn latch(&mut self) {
        self.keep_current_as_next();
    }
}

impl<T, M> Default for Flop<T, M> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, M> Deref for Flop<T, M> {
    type Target = T;

    #[track_caller]
    fn deref(&self) -> &Self::Target {
        #[cfg(debug_assertions)]
        {
            assert!(!self.was_ignored.get(), "Using value of ignored flop.");
            self.was_read.set(true);
        }
        self.cur
            .as_ref()
            .expect("Trying to dereference not set flop.")
    }
}

impl<T, M> fmt::Debug for Flop<T, M>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let curr = &self.cur;
        let next = if self.keep_value {
            &self.cur
        } else {
            &self.next
        };

        if f.alternate() {
            write!(f, "Flop{{",)?;
            // TODO: implement this as a generic trait (we have display_or_default, but not debug_or_default)
            if let Some(val) = curr {
                write!(f, "{val:?}")?;
            } else {
                write!(f, "{}", "-".dimmed())?;
            }
            write!(f, " => ")?;
            if let Some(val) = next {
                write!(f, "{val:?}")?;
            } else {
                write!(f, "{}", "-".dimmed())?;
            }
            write!(f, "}}")
        } else {
            write!(f, "Flop {{ current: {curr:?}, next: {next:?} }}",)
        }
    }
}

/// Transparent latch either returns value from the previous cycle (if latched)
/// or is a pass-through otherwise.
#[allow(dead_code)]
pub(crate) struct TransparentLatch<T: Clone> {
    data: Option<T>,
    // Handles safety checks
    to_latch: CombFlop<bool>,
}

// TODO: This was once useful, maybe will be another time -- and developing it was nontrivial
#[allow(dead_code)]
impl<T: Clone> TransparentLatch<T> {
    pub(crate) fn new() -> Self {
        Self {
            data: None,
            to_latch: CombFlop::new_from(false),
        }
    }
    #[track_caller]
    pub(crate) fn pass(&mut self, data: T) -> T {
        self.pass_if(data, true)
    }
    #[track_caller]
    pub(crate) fn pass_if(&mut self, data: T, may_store: bool) -> T {
        if *self.to_latch {
            self.data.clone().expect("Data not found but set as latch!")
        } else {
            debug_assert!(
                self.data.is_none(),
                "Data passed twice a cycle through a latch!"
            );
            if may_store {
                self.data = Some(data.clone());
            } else {
                // higher priority
                // TODO: go back to something safe
                self.to_latch.keep_current_as_next();
            }
            data
        }
    }

    #[track_caller]
    pub(crate) fn is_passing(&self) -> bool {
        !*self.to_latch
    }
    #[track_caller]
    pub(crate) fn latch(&mut self) {
        // lower priority
        self.to_latch.set_next_if_not_latching(true);
    }

    pub(crate) fn tick(&mut self, #[cfg(debug_assertions)] my_name: &str) {
        self.to_latch.tick(
            #[cfg(debug_assertions)]
            my_name,
        );
        // default
        self.to_latch.set_next(false);
        if !*self.to_latch {
            self.data = None;
        } else {
            #[cfg(debug_assertions)]
            assert!(
                self.data.is_some(),
                "Transparent Latch {my_name} is set to latch, but had no data!"
            );
        }
    }
}

impl<T: Clone> Deref for TransparentLatch<T> {
    type Target = T;

    #[track_caller]
    fn deref(&self) -> &Self::Target {
        self.data.as_ref().expect("Buffer is not set")
    }
}

impl<T> fmt::Debug for TransparentLatch<T>
where
    T: fmt::Debug + Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TransparentLatch {{ buf: {:?}, will_latch:: {:?} }}",
            self.data, self.to_latch,
        )
    }
}

/// Flop that represents a memory - it always has a value.
/// It is meant to carry larger structures and allowing for just a single
/// write-port. For performance purposes, it mutates data
/// instead of overwriting it every cycle. To avoid unnecessary allocations,
/// no heap-allocating closures are allowed. Instead, one should "manually"
/// pass the extra data:
/// ```ignore
/// #[flop]
/// memory: SeqFlopMemoryBank<u32, (u8, u8)>,
/// // usage:
/// self.memory.mutate_next((mul, add), |val, (mul, add)| *val = (*val) * mul + add);
///
/// // for small types (more specifically: `FlopMemoryBank<T, T, M> where T: Copy`):
/// #[flop]
/// memory: SeqFlopMemoryBankSimple<u32>, // same as SeqFlopMemoryBank<u32, u32>
/// // usage:
/// self.memory.set_next((*self.memory) * mul + add);
/// ```
///
/// Similarly to the `Flop`, there are two flavours of the `FlopMemoryBank`:
/// `SeqFlopMemoryBank` and `CombFlopMemoryBank`. The difference in the semantics
/// is the same as in case of `Flop`.
///
/// Note: see `Flop` doc-string for the difference between it and this type.
pub(crate) struct FlopMemoryBank<T, D, M> {
    data: T,
    next_mutator: Option<(D, MutatorFunction<T, D>)>,
    type_marker: PhantomData<M>,
}

type MutatorFunction<T, D> = fn(&mut T, D);

pub(crate) type SeqFlopMemoryBank<T, D> = FlopMemoryBank<T, D, SeqFlopMarker>;
pub(crate) type CombFlopMemoryBank<T, D> = FlopMemoryBank<T, D, CombFlopMarker>;

impl<T, D, M> fmt::Debug for FlopMemoryBank<T, D, M>
where
    T: fmt::Debug,
    D: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FlopMemoryBank {{ current: {:?}, data_for_next: {:?} }}",
            self.data,
            self.next_mutator.as_ref().map(|(d, _)| d),
        )
    }
}
impl<T, D, M> FlopMemoryBank<T, D, M> {
    pub(crate) fn new(value: T) -> Self {
        Self {
            data: value,
            next_mutator: None,
            type_marker: PhantomData,
        }
    }

    // Note: for better error messages, method takes & uses flop name
    //       in debug mode. The method itself is called from autogenerated code
    //       and the call stack does not help.
    pub(crate) fn tick(&mut self, #[cfg(debug_assertions)] _my_name: &str) {
        if let Some((data, f)) = self.next_mutator.take() {
            f(&mut self.data, data);
        }
    }

    #[track_caller]
    #[allow(dead_code)]
    pub(crate) fn is_mutator_set(&self) -> bool {
        self.next_mutator.is_some()
    }

    #[track_caller]
    pub(crate) fn unsafe_as_mut(&mut self) -> &mut T {
        &mut self.data
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.next_mutator.is_none()
    }
}

impl<T, D> FlopMemoryBank<T, D, SeqFlopMarker> {
    #[track_caller]
    pub(crate) fn mutate_next(&mut self, data: D, f: MutatorFunction<T, D>) {
        debug_assert!(
            self.next_mutator.is_none(),
            "Setting flop mutator that is already set."
        );
        self.next_mutator = Some((data, f));
    }
}

impl<T, D> FlopMemoryBank<T, D, CombFlopMarker> {
    #[track_caller]
    pub(crate) fn mutate_next(&mut self, data: D, f: MutatorFunction<T, D>) {
        self.next_mutator = Some((data, f));
    }

    #[allow(dead_code)]
    #[track_caller]
    pub(crate) fn clear_next(&mut self) {
        self.next_mutator = None;
    }
}

macro_rules! impl_flop_memory_bank_for {
    ($marker_type:ty) => {
        impl<T> FlopMemoryBank<T, T, $marker_type>
        where
            // `FlopMemoryBank` is meant for relatively big data structs,
            // but might be also used with smaller ones, too.
            // Thus, for ergonomy, a `set_next` method is provided for `Copy`
            // types (they should be around 8 bytes).
            T: Copy,
        {
            #[allow(dead_code)]
            #[track_caller]
            pub(crate) fn set_next(&mut self, new_data: T) {
                self.mutate_next(new_data, |data, new_data| *data = new_data);
            }
        }
    };
}

impl_flop_memory_bank_for!(SeqFlopMarker);
impl_flop_memory_bank_for!(CombFlopMarker);

impl<T, D, M> Deref for FlopMemoryBank<T, D, M> {
    type Target = T;

    #[track_caller]
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

/// The `Register` type is an optimized version of `FlopMemoryBank` for simple (small) values.
/// In contrast to the former, the compiler can optimize `tick` method, because there is
/// no user-supplied function pointer.
/// It is designed to be used with small types (that are copy) or move types that change very rarely.
pub(crate) struct Register<T, M> {
    data: T,
    next: Option<T>,
    type_marker: PhantomData<M>,
}
pub(crate) type SeqFlopMemoryBankSimple<T> = Register<T, SeqFlopMarker>;
pub(crate) type SeqRegister<T> = Register<T, SeqFlopMarker>;
/// The use is discouraged.
pub(crate) type SeqFlopMemoryBankSimpleLarge<T> = Register<T, BufferFlopMarker>;
pub(crate) type CombFlopMemoryBankSimple<T> = Register<T, CombFlopMarker>;
pub(crate) type CombRegister<T> = Register<T, CombFlopMarker>;

impl<T, M> fmt::Debug for Register<T, M>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Register {{ current: {:?}, next: {:?} }}",
            self.data, self.next,
        )
    }
}
impl<T, M> Register<T, M> {
    pub(crate) fn new(value: T) -> Self {
        Self {
            data: value,
            next: None,
            type_marker: PhantomData,
        }
    }

    // Note: for better error messages, method takes & uses flop name
    //       in debug mode. The method itself is called from autogenerated code
    //       and the call stack does not help.
    pub(crate) fn tick(&mut self, #[cfg(debug_assertions)] _my_name: &str) {
        if let Some(data) = self.next.take() {
            self.data = data;
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.next.is_none()
    }

    #[track_caller]
    pub(crate) fn peek_next(&self) -> Option<&T> {
        self.next.as_ref()
    }
}

// `Register` is meant for small types (that impl `Copy`).
// (they should be around 8 bytes).
impl<T: Copy> Register<T, SeqFlopMarker> {
    #[track_caller]
    pub(crate) fn set_next(&mut self, data: T)
    where
        T: Copy,
    {
        debug_assert!(
            self.next.is_none(),
            "Setting register mutator that is already set."
        );
        self.next = Some(data);
    }
}

// Consider moving this to a more suitable place
impl<T: cmemu_common::HwRegister + Copy> SeqRegister<T> {
    #[track_caller]
    pub(crate) fn set_next_mutated_reg(&mut self, data: u32) -> T {
        let mut next = self.data;
        next.mutate(data);
        self.set_next(next);
        next
    }
}

impl<T: cmemu_common::HwRegister + Copy> CombRegister<T> {
    #[track_caller]
    pub(crate) fn set_next_mutated_reg(&mut self, data: u32) -> T {
        let mut next = self.data;
        next.mutate(data);
        self.set_next(next);
        next
    }
}

impl<T: Copy> Register<T, CombFlopMarker> {
    #[track_caller]
    pub(crate) fn set_next(&mut self, data: T) {
        self.next = Some(data);
    }

    #[allow(dead_code)]
    #[track_caller]
    pub(crate) fn clear_next(&mut self) {
        self.next = None;
    }

    /// Get a mutable reference to the value stored as next,
    /// possibly initializing it with the current value if not present.
    #[track_caller]
    pub(crate) fn next_builder(&mut self) -> &mut T {
        self.next.get_or_insert(self.data)
    }

    #[track_caller]
    pub(crate) fn unsafe_as_mut(&mut self) -> &mut T {
        &mut self.data
    }
}

impl<T> Register<T, BufferFlopMarker> {
    #[track_caller]
    pub(crate) fn set_next(&mut self, data: T) {
        debug_assert!(
            self.next.is_none(),
            "Setting register mutator that is already set."
        );
        self.next = Some(data);
    }
}

impl<T, M> Deref for Register<T, M> {
    type Target = T;

    #[track_caller]
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

#[cfg(test)]
mod test {
    use super::TransparentLatch;

    #[cfg(debug_assertions)]
    #[test]
    fn latch_usage() {
        let mut latch = TransparentLatch::<u32>::new();

        // nothing happens
        latch.tick("L");
        latch.tick("L");
        latch.tick("L");

        assert_eq!(latch.pass(42), 42);
        assert_eq!(*latch, 42);

        latch.tick("L");

        assert_eq!(latch.pass(17), 17);
        latch.latch();
        assert_eq!(*latch, 17);

        latch.tick("L");
        assert_eq!(latch.pass(42), 17);
        latch.latch();
        assert_eq!(*latch, 17);

        latch.tick("L");
        latch.latch();
        assert_eq!(latch.pass(42), 17);

        latch.tick("L");
        assert_eq!(latch.pass(42), 17);

        latch.tick("L");
        latch.latch();
        assert_eq!(latch.pass(42), 42);

        latch.tick("L");
        // Usage of set latch
        assert_eq!(latch.pass(19), 42);

        // No-op
        latch.tick("L");
        latch.tick("L");
        latch.tick("L");
        assert_eq!(latch.pass(17), 17);
    }

    #[test]
    #[should_panic]
    fn latch_double_pass_unset() {
        let mut latch = TransparentLatch::<u32>::new();
        assert_eq!(latch.pass(42), 42);
        assert_eq!(latch.pass(42), 42);
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn latch_double_pass_set() {
        let mut latch = TransparentLatch::<u32>::new();
        assert_eq!(latch.pass(42), 42);
        latch.latch();
        latch.tick("L");

        assert_eq!(latch.pass(17), 42);
        assert_eq!(latch.pass(17), 42);
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn unused_latched_value() {
        let mut latch = TransparentLatch::<u32>::new();
        assert_eq!(latch.pass(42), 42);
        latch.latch();
        latch.tick("L");
        latch.tick("L");
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn deref_no_data() {
        let mut latch = TransparentLatch::<u32>::new();
        assert_eq!(latch.pass(42), 42);
        latch.tick("L");
        let _ = *latch;
    }
}
