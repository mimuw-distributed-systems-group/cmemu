use cmemu_common::Address;
use core::ops::Range;

pub const BASE_ADDR: Address = Address::from_const(0x40095000);
pub const ADDR: Address = BASE_ADDR.offset(0x0);
pub const SIZE: u32 = 0x400;
/// 0x40095000..0x40095400
pub const ADDR_SPACE: Range<Address> = ADDR..ADDR.offset(SIZE);

/// Last Measured Battery Voltage
///
///
///
/// This register may be read while BATUPD.STAT = 1
///
/// [TI-TRM-I] 18.3.1.10 BAT Register (Offset = 28h) [reset = 0h]
pub mod BAT;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 18.3.1.6 BATMONP0 Register (Offset = 18h) [reset = 0h]
pub mod BATMONP0;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 18.3.1.7 BATMONP1 Register (Offset = 1Ch) [reset = 0h]
pub mod BATMONP1;
/// Battery Update
///
///
///
/// Indicates BAT Updates
///
/// [TI-TRM-I] 18.3.1.11 BATUPD Register (Offset = 2Ch) [reset = 0h]
pub mod BATUPD;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 18.3.1.1 CTL Register (Offset = 0h) [reset = 0h]
pub mod CTL;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 18.3.1.9 FLASHPUMPP0 Register (Offset = 24h) [reset = 0h]
pub mod FLASHPUMPP0;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 18.3.1.8 IOSTRP0 Register (Offset = 20h) [reset = 28h]
pub mod IOSTRP0;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 18.3.1.2 MEASCFG Register (Offset = 4h) [reset = 0h]
pub mod MEASCFG;
/// Temperature
///
///
///
/// Last Measured Temperature in Degrees Celsius
///
///
///
/// This register may be read while TEMPUPD.STAT = 1.
///
/// [TI-TRM-I] 18.3.1.12 TEMP Register (Offset = 30h) [reset = 0h]
pub mod TEMP;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 18.3.1.3 TEMPP0 Register (Offset = Ch) [reset = 0h]
pub mod TEMPP0;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 18.3.1.4 TEMPP1 Register (Offset = 10h) [reset = 0h]
pub mod TEMPP1;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 18.3.1.5 TEMPP2 Register (Offset = 14h) [reset = 0h]
pub mod TEMPP2;
/// Temperature Update
///
///
///
/// Indicates TEMP Updates
///
/// [TI-TRM-I] 18.3.1.13 TEMPUPD Register (Offset = 34h) [reset = 0h]
pub mod TEMPUPD;
