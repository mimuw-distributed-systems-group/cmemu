use cmemu_common::Address;
use core::ops::Range;

pub const BASE_ADDR: Address = Address::from_const(0x400c7000);
pub const ADDR: Address = BASE_ADDR.offset(0x0);
pub const SIZE: u32 = 0x1000;
/// 0x400c7000..0x400c8000
pub const ADDR_SPACE: Range<Address> = ADDR..ADDR.offset(SIZE);

/// Timer 0 Configuration
///
/// [TI-TRM-I] 17.7.6.1 T0CFG Register (Offset = 0h) [reset = 0h]
pub mod T0CFG;
/// Timer 0 Control
///
/// [TI-TRM-I] 17.7.6.3 T0CTL Register (Offset = 8h) [reset = 0h]
pub mod T0CTL;
/// Timer 0 Target
///
/// [TI-TRM-I] 17.7.6.4 T0TARGET Register (Offset = Ch) [reset = 0h]
pub mod T0TARGET;
/// Timer 1 Configuration
///
/// [TI-TRM-I] 17.7.6.2 T1CFG Register (Offset = 4h) [reset = 0h]
pub mod T1CFG;
/// Timer 1 Control
///
/// [TI-TRM-I] 17.7.6.6 T1CTL Register (Offset = 14h) [reset = 0h]
pub mod T1CTL;
/// Timer 1 Target
///
///
///
/// Timer 1 counter target value
///
/// [TI-TRM-I] 17.7.6.5 T1TARGET Register (Offset = 10h) [reset = 0h]
pub mod T1TARGET;
