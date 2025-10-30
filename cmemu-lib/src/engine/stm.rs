use std::fmt::Debug;

// Seems like a good idea to mark this kind of enum.
#[allow(
    dead_code,
    reason = "This is a marker with possible future derive impl."
)]
pub(crate) trait StateMachine: Debug {}

#[macro_export]
macro_rules! move_state_machine {
    // The syntax is weird, but Assignment Expression is Expr = Expr, but in macros only => comma and ; are allowed after :expr.
    ($this:ident.$field:ident => $from:path => $to:path) => {
        #[cfg(not(test))]
        $crate::engine::move_state_machine!($this.$field => $from => $to,);
        #[cfg(test)]
        $crate::engine::move_state_machine!($this.$field => $from => $to, ": in {}", $crate::printable!($this));
    };
    ($lvalue:expr => $from:path => $to:path $(, $fmt:literal $(,$fargs:expr)*)? $(,)?) => {
        assert!(
            matches!($lvalue, $from),
                concat!("State machine {} expected to be in {:?} state, but was in {:?}", $(" ", $fmt)?),
                stringify!($lvalue),
                $from,
                $lvalue,
                $($($fargs),*)?
        );
        $lvalue = $to;
    };
    ($lvalue:expr => $type:path = { $($from:pat $(if $ifexpr:expr)? => $to:expr),* $(,)? }  $(, $fmt:literal $(,$fargs:expr)*)? $(,)?) => {{
        use $type ::*;
        $lvalue = match $lvalue {
            $(
            $from $(if $ifexpr)? => $to
            ,)*
            _ => panic!(
                    concat!("State machine {} expected to be in any of {} state, but was in {:?}", $(" ", $fmt)?),
                    stringify!($lvalue),
                    stringify!( $($from)|* ),
                    $lvalue,
                    $($($fargs),*)?
                )
        };
    }};
}

pub(crate) use move_state_machine;

#[macro_export]
macro_rules! debug_move_state_machine {
    ($($tokens:tt)*) => {
        #[cfg(debug_assertions)]
        $crate::engine::move_state_machine!($($tokens)*);
    }
}
pub(crate) use debug_move_state_machine;

#[cfg(debug_assertions)]
#[allow(
    dead_code,
    reason = "This is a marker with possible future derive impl."
)]
pub(crate) trait TransitionValidator {
    /// Asserts that the proposed state transition is valid in terms of a specific protocol.
    /// Panics
    /// ------
    /// If the transition violates the basic assumptions of the protocol.
    /// Recoverable error handling should be done separately.
    fn assert_is_valid_transition(&self, next: &Self);
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! impl_state_machine_validator {
    ($type:path, { $($from:pat => $to:pat),* $(,)? } ) => {
use $type ::*;
impl $crate::engine::TransitionValidator for $type where $type : $crate::engine::StateMachine {

    fn assert_is_valid_transition(&self, next: &Self) {
        match (self, next) {
            $(
            ($from, $to) => (),
            )*
            _ => panic!(
                    "Transition from {:?} to {:?} is invalid for {}!",
                    self, next,
                    stringify!( $type ),
                )
        };
    }
}
    };
}
