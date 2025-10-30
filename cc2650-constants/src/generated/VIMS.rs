use cmemu_common::Address;
use core::ops::Range;

pub const BASE_ADDR: Address = Address::from_const(0x40034000);
pub const ADDR: Address = BASE_ADDR.offset(0x0);
pub const SIZE: u32 = 0x400;
/// 0x40034000..0x40034400
pub const ADDR_SPACE: Range<Address> = ADDR..ADDR.offset(SIZE);

/// Control
///
/// Configure VIMS mode and line buffer settings
///
/// [TI-TRM-I] 7.9.2.2 CTL Register (Offset = 4h) [reset = 0h]
pub mod CTL;
/// Status
///
/// Displays current VIMS mode and line buffer status
///
/// [TI-TRM-I] 7.9.2.1 STAT Register (Offset = 0h) [reset = 0h]
pub mod STAT;
