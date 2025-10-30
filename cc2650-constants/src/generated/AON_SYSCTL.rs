use cmemu_common::Address;
use core::ops::Range;

pub const BASE_ADDR: Address = Address::from_const(0x40090000);
pub const ADDR: Address = BASE_ADDR.offset(0x0);
pub const SIZE: u32 = 0x400;
/// 0x40090000..0x40090400
pub const ADDR_SPACE: Range<Address> = ADDR..ADDR.offset(SIZE);

/// Power Management
///
///
///
/// This register controls bitfields for setting low level power management features such as selection of  regulator for VDDR supply and control of IO ring where certain segments can be enabled / disabled.
///
/// [TI-TRM-I] 6.8.2.2.1 PWRCTL Register (Offset = 0h) [reset = 0h]
pub mod PWRCTL;
/// Reset Management
///
///
///
/// This register contains bitfields releated to system reset such as reset source and reset request  and control of brown out resets.
///
/// [TI-TRM-I] 6.8.2.2.2 RESETCTL Register (Offset = 4h) [reset = E0h]
pub mod RESETCTL;
/// Sleep Mode
///
///
///
/// This register is used to unfreeze the IO pad ring after waking up from SHUTDOWN
///
/// [TI-TRM-I] 6.8.2.2.3 SLEEPCTL Register (Offset = 8h) [reset = 0h]
pub mod SLEEPCTL;
