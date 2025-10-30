use cmemu_common::Address;
use core::ops::Range;

pub const BASE_ADDR: Address = Address::from_const(0x40043000);
pub const ADDR: Address = BASE_ADDR.offset(0x0);
pub const SIZE: u32 = 0x100;
/// 0x40043000..0x40043100
pub const ADDR_SPACE: Range<Address> = ADDR..ADDR.offset(SIZE);

/// Timer Channel 0 Capture/Compare Register
///
/// [TI-TRM-I] 23.8.1.2 RATCH0VAL Register (Offset = 80h) [reset = 0h]
pub mod RATCH0VAL;
/// Timer Channel 1 Capture/Compare Register
///
/// [TI-TRM-I] 23.8.1.3 RATCH1VAL Register (Offset = 84h) [reset = 0h]
pub mod RATCH1VAL;
/// Timer Channel 2 Capture/Compare Register
///
/// [TI-TRM-I] 23.8.1.4 RATCH2VAL Register (Offset = 88h) [reset = 0h]
pub mod RATCH2VAL;
/// Timer Channel 3 Capture/Compare Register
///
/// [TI-TRM-I] 23.8.1.5 RATCH3VAL Register (Offset = 8Ch) [reset = 0h]
pub mod RATCH3VAL;
/// Timer Channel 4 Capture/Compare Register
///
/// [TI-TRM-I] 23.8.1.6 RATCH4VAL Register (Offset = 90h) [reset = 0h]
pub mod RATCH4VAL;
/// Timer Channel 5 Capture/Compare Register
///
/// [TI-TRM-I] 23.8.1.7 RATCH5VAL Register (Offset = 94h) [reset = 0h]
pub mod RATCH5VAL;
/// Timer Channel 6 Capture/Compare Register
///
/// [TI-TRM-I] 23.8.1.8 RATCH6VAL Register (Offset = 98h) [reset = 0h]
pub mod RATCH6VAL;
/// Timer Channel 7 Capture/Compare Register
///
/// [TI-TRM-I] 23.8.1.9 RATCH7VAL Register (Offset = 9Ch) [reset = 0h]
pub mod RATCH7VAL;
/// Radio Timer Counter Value
///
/// [TI-TRM-I] 23.8.1.1 RATCNT Register (Offset = 4h) [reset = 0h]
pub mod RATCNT;
