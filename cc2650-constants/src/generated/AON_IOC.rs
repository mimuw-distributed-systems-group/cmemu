use cmemu_common::Address;
use core::ops::Range;

pub const BASE_ADDR: Address = Address::from_const(0x40094000);
pub const ADDR: Address = BASE_ADDR.offset(0x0);
pub const SIZE: u32 = 0x400;
/// 0x40094000..0x40094400
pub const ADDR_SPACE: Range<Address> = ADDR..ADDR.offset(SIZE);

/// SCLK_LF External Output Control
///
/// [TI-TRM-I] 11.11.1.5 CLK32KCTL Register (Offset = 10h) [reset = 1h]
pub mod CLK32KCTL;
/// IO Latch Control
///
///
///
/// Controls transparency of all latches holding I/O or configuration state from the MCU IOC
///
/// [TI-TRM-I] 11.11.1.4 IOCLATCH Register (Offset = Ch) [reset = 1h]
pub mod IOCLATCH;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 11.11.1.3 IOSTRMAX Register (Offset = 8h) [reset = 5h]
pub mod IOSTRMAX;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 11.11.1.2 IOSTRMED Register (Offset = 4h) [reset = 6h]
pub mod IOSTRMED;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 11.11.1.1 IOSTRMIN Register (Offset = 0h) [reset = 3h]
pub mod IOSTRMIN;
