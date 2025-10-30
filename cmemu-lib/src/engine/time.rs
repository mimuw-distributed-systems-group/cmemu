use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

pub const PICOS_IN_SECOND: u64 = 1_000_000_000_000;

macro_rules! def_time_type {
    ($name:ident) => {
        #[derive(Clone, Copy, Default, PartialOrd, Ord, PartialEq, Eq, Hash)]
        pub struct $name {
            picoseconds: u64,
        }

        // Impl inspired by: https://doc.rust-lang.org/std/time/struct.Duration.html.
        impl $name {
            #[must_use]
            pub const fn from_picos(picoseconds: u64) -> Self {
                Self { picoseconds }
            }
            #[must_use]
            pub const fn as_picos(self) -> u64 {
                self.picoseconds
            }
            pub const ZERO: Self = Self::from_picos(0);
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                if f.alternate() {
                    let units = [
                        (24 * 3_600 * PICOS_IN_SECOND, "days"),
                        (3_600 * PICOS_IN_SECOND, "h"),
                        (PICOS_IN_SECOND, "s"),
                        (1_000_000_000, "ms"),
                        (1_000_000, "us"),
                        (1_000, "ns"),
                    ];
                    for (unit_divider, unit_name) in units {
                        if self.picoseconds > unit_divider {
                            #[allow(clippy::cast_precision_loss)]
                            return write!(
                                f,
                                "{}({:.4}{})",
                                std::stringify!($name),
                                self.picoseconds as f64 / unit_divider as f64,
                                unit_name
                            );
                        }
                    }
                }
                write!(f, "{}({}ps)", std::stringify!($name), &self.picoseconds)
            }
        }

        // TODO: derive it too and handle division by 0 properly
        impl Div for $name {
            type Output = u64;

            fn div(self, rhs: Self) -> u64 {
                self.picoseconds / rhs.picoseconds
            }
        }
    };
}

def_time_type!(Timepoint);
def_time_type!(Duration);

macro_rules! define_time_op {
    ($lhs:ty [$trait_name:ident :: $fn_name:ident] $rhs:tt => $output:ty;
        $checked_op:ident using $impl_ch_op:ident,
        $wrapping_op:ident using $impl_wr_op:ident,
        $saturating_op:ident using $impl_sat_op:ident
    ) => {
        impl $trait_name<$rhs> for $lhs {
            type Output = $output;
            fn $fn_name(self, rhs: $rhs) -> Self::Output {
                let lhs: u64 = self.as_picos();
                let rhs: u64 = _helper_as_picos!(rhs: $rhs);
                let result: u64 = $trait_name::$fn_name(lhs, rhs);
                <$output>::from_picos(result)
            }
        }

        impl $lhs {
            #[must_use]
            pub const fn $checked_op(self, rhs: $rhs) -> Option<$output> {
                let lhs: u64 = self.as_picos();
                let rhs: u64 = _helper_as_picos!(rhs: $rhs);
                if let Some(result) = lhs.$impl_ch_op(rhs) {
                    Some(<$output>::from_picos(result))
                } else {
                    None
                }
            }

            #[must_use]
            pub const fn $wrapping_op(self, rhs: $rhs) -> $output {
                let lhs: u64 = self.as_picos();
                let rhs: u64 = _helper_as_picos!(rhs: $rhs);
                let result: u64 = lhs.$impl_wr_op(rhs);
                <$output>::from_picos(result)
            }

            #[must_use]
            pub const fn $saturating_op(self, rhs: $rhs) -> $output {
                let lhs: u64 = self.as_picos();
                let rhs: u64 = _helper_as_picos!(rhs: $rhs);
                let result: u64 = lhs.$impl_sat_op(rhs);
                <$output>::from_picos(result)
            }
        }
    };
    ($lhs:ty [$trait_name:ident :: $fn_name:ident] $rhs:tt) => {
        impl $trait_name<$rhs> for $lhs {
            fn $fn_name(&mut self, rhs: $rhs) {
                let mut lhs: u64 = self.as_picos();
                let rhs: u64 = _helper_as_picos!(rhs: $rhs);
                $trait_name::$fn_name(&mut lhs, rhs);
                *self = Self::from_picos(lhs);
            }
        }
    };
}

macro_rules! _helper_as_picos {
    ($rhs_id:ident : Timepoint) => {
        $rhs_id.as_picos()
    };
    ($rhs_id:ident : Duration) => {
        $rhs_id.as_picos()
    };
    ($rhs_id:ident : u64) => {
        $rhs_id
    };
}

define_time_op!(Timepoint [Add::add] Duration => Timepoint;
    checked_add_duration using checked_add, wrapping_add_duration using wrapping_add, saturating_add_duration using saturating_add);
define_time_op!(Timepoint [Sub::sub] Duration => Timepoint;
    checked_sub_duration using checked_sub, wrapping_sub_duration using wrapping_sub, saturating_sub_duration using saturating_sub);
define_time_op!(Timepoint [Sub::sub] Timepoint => Duration;
    checked_sub_timepoint using checked_sub, wrapping_sub_timepoint using wrapping_sub, saturating_sub_timepoint using saturating_sub);
define_time_op!(Timepoint [AddAssign::add_assign] Duration);
define_time_op!(Timepoint [SubAssign::sub_assign] Duration);

define_time_op!(Duration [Add::add] Duration => Duration;
    checked_add_duration using checked_add, wrapping_add_duration using wrapping_add, saturating_add_duration using saturating_add);
define_time_op!(Duration [Sub::sub] Duration => Duration;
    checked_sub_duration using checked_sub, wrapping_sub_duration using wrapping_sub, saturating_sub_duration using saturating_sub);
define_time_op!(Duration [Mul::mul] u64 => Duration;
    checked_mul_u64 using checked_mul, wrapping_mul_u64 using wrapping_mul, saturating_mul_u64 using saturating_mul);
define_time_op!(Duration [Div::div] u64 => Duration;
    checked_div_u64 using checked_div, wrapping_div_u64 using wrapping_div, saturating_div_u64 using saturating_div);

define_time_op!(Duration [AddAssign::add_assign] Duration);
define_time_op!(Duration [SubAssign::sub_assign] Duration);
define_time_op!(Duration [MulAssign::mul_assign] u64);
define_time_op!(Duration [DivAssign::div_assign] u64);

impl Duration {
    pub const ONE_SECOND: Duration = Duration::from_picos(PICOS_IN_SECOND);
    /// Minimal non-zero delay
    pub const MIN_DELAY: Duration = Duration::from_picos(1);
    pub const MAX_DELAY: Duration = Duration::from_picos(u64::MAX);
    /// A very large duration such as if it looks like an infinity!
    pub const LOOKS_SATURATED: Duration = Duration::from_picos(u64::MAX - u32::MAX as u64 - 1);

    pub fn into_std(self) -> std::time::Duration {
        std::time::Duration::from_nanos(self.as_picos() / 1_000)
    }
}
