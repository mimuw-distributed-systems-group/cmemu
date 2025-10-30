use cmemu_common::Address;
use core::ops::Range;

pub const BASE_ADDR: Address = Address::from_const(0x400c8000);
pub const ADDR: Address = BASE_ADDR.offset(0x0);
pub const SIZE: u32 = 0x1000;
/// 0x400c8000..0x400c9000
pub const ADDR_SPACE: Range<Address> = ADDR..ADDR.offset(SIZE);

/// Auto Take
///
///
///
/// Sticky Request for Single Semaphore.
///
/// [TI-TRM-I] 17.7.4.9 AUTOTAKE Register (Offset = 20h) [reset = 0h]
pub mod AUTOTAKE;
/// Semaphore 0
///
/// [TI-TRM-I] 17.7.4.1 SMPH0 Register (Offset = 0h) [reset = 1h]
pub mod SMPH0;
/// Semaphore 1
///
/// [TI-TRM-I] 17.7.4.2 SMPH1 Register (Offset = 4h) [reset = 1h]
pub mod SMPH1;
/// Semaphore 2
///
/// [TI-TRM-I] 17.7.4.3 SMPH2 Register (Offset = 8h) [reset = 1h]
pub mod SMPH2;
/// Semaphore 3
///
/// [TI-TRM-I] 17.7.4.4 SMPH3 Register (Offset = Ch) [reset = 1h]
pub mod SMPH3;
/// Semaphore 4
///
/// [TI-TRM-I] 17.7.4.5 SMPH4 Register (Offset = 10h) [reset = 1h]
pub mod SMPH4;
/// Semaphore 5
///
/// [TI-TRM-I] 17.7.4.6 SMPH5 Register (Offset = 14h) [reset = 1h]
pub mod SMPH5;
/// Semaphore 6
///
/// [TI-TRM-I] 17.7.4.7 SMPH6 Register (Offset = 18h) [reset = 1h]
pub mod SMPH6;
/// Semaphore 7
///
/// [TI-TRM-I] 17.7.4.8 SMPH7 Register (Offset = 1Ch) [reset = 1h]
pub mod SMPH7;
