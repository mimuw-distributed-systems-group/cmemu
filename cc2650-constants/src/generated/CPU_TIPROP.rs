use cmemu_common::Address;
use core::ops::Range;

pub const BASE_ADDR: Address = Address::from_const(0xe00fe000);
pub const ADDR: Address = BASE_ADDR.offset(0x0);
pub const SIZE: u32 = 0x1000;
/// 0xe00fe000..0xe00ff000
pub const ADDR_SPACE: Range<Address> = ADDR..ADDR.offset(SIZE);

/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod DYN_CG;
/// Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.
///
/// [TI-TRM-I] undocumented
pub mod RESERVED000;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod TRACECLKMUX;
