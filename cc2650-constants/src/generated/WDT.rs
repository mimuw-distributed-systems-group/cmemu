use cmemu_common::Address;
use core::ops::Range;

pub const BASE_ADDR: Address = Address::from_const(0x40080000);
pub const ADDR: Address = BASE_ADDR.offset(0x0);
pub const SIZE: u32 = 0x1000;
/// 0x40080000..0x40081000
pub const ADDR_SPACE: Range<Address> = ADDR..ADDR.offset(SIZE);

/// Control
///
/// [TI-TRM-I] 15.4.1.3 CTL Register (Offset = 8h) [reset = 0h]
pub mod CTL;
/// Interrupt Clear
///
/// [TI-TRM-I] 15.4.1.4 ICR Register (Offset = Ch) [reset = 0h]
pub mod ICR;
/// Interrupt Cause Test Mode
///
/// [TI-TRM-I] 15.4.1.8 INT_CAUS Register (Offset = 41Ch) [reset = 0h]
pub mod INT_CAUS;
/// Configuration
///
/// [TI-TRM-I] 15.4.1.1 LOAD Register (Offset = 0h) [reset = FFFFFFFFh]
pub mod LOAD;
/// Lock
///
/// [TI-TRM-I] 15.4.1.9 LOCK Register (Offset = C00h) [reset = 0h]
pub mod LOCK;
/// Masked Interrupt Status
///
/// [TI-TRM-I] 15.4.1.6 MIS Register (Offset = 14h) [reset = 0h]
pub mod MIS;
/// Raw Interrupt Status
///
/// [TI-TRM-I] 15.4.1.5 RIS Register (Offset = 10h) [reset = 0h]
pub mod RIS;
/// Test Mode
///
/// [TI-TRM-I] 15.4.1.7 TEST Register (Offset = 418h) [reset = 0h]
pub mod TEST;
/// Current Count Value
///
/// [TI-TRM-I] 15.4.1.2 VALUE Register (Offset = 4h) [reset = FFFFFFFFh]
pub mod VALUE;
