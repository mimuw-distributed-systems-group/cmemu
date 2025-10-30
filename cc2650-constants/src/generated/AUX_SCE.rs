use cmemu_common::Address;
use core::ops::Range;

pub const BASE_ADDR: Address = Address::from_const(0x400e1000);
pub const ADDR: Address = BASE_ADDR.offset(0x0);
pub const SIZE: u32 = 0x1000;
/// 0x400e1000..0x400e2000
pub const ADDR_SPACE: Range<Address> = ADDR..ADDR.offset(SIZE);

/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod CPUSTAT;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod CTL;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod FETCHSTAT;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod LOOPADDR;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod LOOPCNT;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod REG1_0;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod REG3_2;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod REG5_4;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod REG7_6;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod WUSTAT;
