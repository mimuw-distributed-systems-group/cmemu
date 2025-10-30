use cmemu_common::Address;
use core::ops::Range;

pub const BASE_ADDR: Address = Address::from_const(0x40040000);
pub const ADDR: Address = BASE_ADDR.offset(0x0);
pub const SIZE: u32 = 0x4;
/// 0x40040000..0x40040004
pub const ADDR_SPACE: Range<Address> = ADDR..ADDR.offset(SIZE);

/// RF Core Power Management and Clock Enable
///
/// [TI-TRM-I] 23.8.3.1 PWMCLKEN Register (Offset = 0h) [reset = 1h]
pub mod PWMCLKEN;
