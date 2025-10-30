use cmemu_common::Address;
use core::ops::Range;

pub const BASE_ADDR: Address = Address::from_const(0x40084000);
pub const ADDR: Address = BASE_ADDR.offset(0x0);
pub const SIZE: u32 = 0x1000;
/// 0x40084000..0x40085000
pub const ADDR_SPACE: Range<Address> = ADDR..ADDR.offset(SIZE);

/// MCU SEMAPHORE 0 ALIAS
///
/// [TI-TRM-I] undocumented
pub mod PEEK0;
/// MCU SEMAPHORE 1 ALIAS
///
/// [TI-TRM-I] undocumented
pub mod PEEK1;
/// MCU SEMAPHORE 10 ALIAS
///
/// [TI-TRM-I] undocumented
pub mod PEEK10;
/// MCU SEMAPHORE 11 ALIAS
///
/// [TI-TRM-I] undocumented
pub mod PEEK11;
/// MCU SEMAPHORE 12 ALIAS
///
/// [TI-TRM-I] undocumented
pub mod PEEK12;
/// MCU SEMAPHORE 13 ALIAS
///
/// [TI-TRM-I] undocumented
pub mod PEEK13;
/// MCU SEMAPHORE 14 ALIAS
///
/// [TI-TRM-I] undocumented
pub mod PEEK14;
/// MCU SEMAPHORE 15 ALIAS
///
/// [TI-TRM-I] undocumented
pub mod PEEK15;
/// MCU SEMAPHORE 16 ALIAS
///
/// [TI-TRM-I] undocumented
pub mod PEEK16;
/// MCU SEMAPHORE 17 ALIAS
///
/// [TI-TRM-I] undocumented
pub mod PEEK17;
/// MCU SEMAPHORE 18 ALIAS
///
/// [TI-TRM-I] undocumented
pub mod PEEK18;
/// MCU SEMAPHORE 19 ALIAS
///
/// [TI-TRM-I] undocumented
pub mod PEEK19;
/// MCU SEMAPHORE 2 ALIAS
///
/// [TI-TRM-I] undocumented
pub mod PEEK2;
/// MCU SEMAPHORE 20 ALIAS
///
/// [TI-TRM-I] undocumented
pub mod PEEK20;
/// MCU SEMAPHORE 21 ALIAS
///
/// [TI-TRM-I] undocumented
pub mod PEEK21;
/// MCU SEMAPHORE 22 ALIAS
///
/// [TI-TRM-I] undocumented
pub mod PEEK22;
/// MCU SEMAPHORE 23 ALIAS
///
/// [TI-TRM-I] undocumented
pub mod PEEK23;
/// MCU SEMAPHORE 24 ALIAS
///
/// [TI-TRM-I] undocumented
pub mod PEEK24;
/// MCU SEMAPHORE 25 ALIAS
///
/// [TI-TRM-I] undocumented
pub mod PEEK25;
/// MCU SEMAPHORE 26 ALIAS
///
/// [TI-TRM-I] undocumented
pub mod PEEK26;
/// MCU SEMAPHORE 27 ALIAS
///
/// [TI-TRM-I] undocumented
pub mod PEEK27;
/// MCU SEMAPHORE 28 ALIAS
///
/// [TI-TRM-I] undocumented
pub mod PEEK28;
/// MCU SEMAPHORE 29 ALIAS
///
/// [TI-TRM-I] undocumented
pub mod PEEK29;
/// MCU SEMAPHORE 3 ALIAS
///
/// [TI-TRM-I] undocumented
pub mod PEEK3;
/// MCU SEMAPHORE 30 ALIAS
///
/// [TI-TRM-I] undocumented
pub mod PEEK30;
/// MCU SEMAPHORE 31 ALIAS
///
/// [TI-TRM-I] undocumented
pub mod PEEK31;
/// MCU SEMAPHORE 4 ALIAS
///
/// [TI-TRM-I] undocumented
pub mod PEEK4;
/// MCU SEMAPHORE 5 ALIAS
///
/// [TI-TRM-I] undocumented
pub mod PEEK5;
/// MCU SEMAPHORE 6 ALIAS
///
/// [TI-TRM-I] undocumented
pub mod PEEK6;
/// MCU SEMAPHORE 7 ALIAS
///
/// [TI-TRM-I] undocumented
pub mod PEEK7;
/// MCU SEMAPHORE 8 ALIAS
///
/// [TI-TRM-I] undocumented
pub mod PEEK8;
/// MCU SEMAPHORE 9 ALIAS
///
/// [TI-TRM-I] undocumented
pub mod PEEK9;
/// MCU SEMAPHORE 0
///
/// [TI-TRM-I] undocumented
pub mod SMPH0;
/// MCU SEMAPHORE 1
///
/// [TI-TRM-I] undocumented
pub mod SMPH1;
/// MCU SEMAPHORE 10
///
/// [TI-TRM-I] undocumented
pub mod SMPH10;
/// MCU SEMAPHORE 11
///
/// [TI-TRM-I] undocumented
pub mod SMPH11;
/// MCU SEMAPHORE 12
///
/// [TI-TRM-I] undocumented
pub mod SMPH12;
/// MCU SEMAPHORE 13
///
/// [TI-TRM-I] undocumented
pub mod SMPH13;
/// MCU SEMAPHORE 14
///
/// [TI-TRM-I] undocumented
pub mod SMPH14;
/// MCU SEMAPHORE 15
///
/// [TI-TRM-I] undocumented
pub mod SMPH15;
/// MCU SEMAPHORE 16
///
/// [TI-TRM-I] undocumented
pub mod SMPH16;
/// MCU SEMAPHORE 17
///
/// [TI-TRM-I] undocumented
pub mod SMPH17;
/// MCU SEMAPHORE 18
///
/// [TI-TRM-I] undocumented
pub mod SMPH18;
/// MCU SEMAPHORE 19
///
/// [TI-TRM-I] undocumented
pub mod SMPH19;
/// MCU SEMAPHORE 2
///
/// [TI-TRM-I] undocumented
pub mod SMPH2;
/// MCU SEMAPHORE 20
///
/// [TI-TRM-I] undocumented
pub mod SMPH20;
/// MCU SEMAPHORE 21
///
/// [TI-TRM-I] undocumented
pub mod SMPH21;
/// MCU SEMAPHORE 22
///
/// [TI-TRM-I] undocumented
pub mod SMPH22;
/// MCU SEMAPHORE 23
///
/// [TI-TRM-I] undocumented
pub mod SMPH23;
/// MCU SEMAPHORE 24
///
/// [TI-TRM-I] undocumented
pub mod SMPH24;
/// MCU SEMAPHORE 25
///
/// [TI-TRM-I] undocumented
pub mod SMPH25;
/// MCU SEMAPHORE 26
///
/// [TI-TRM-I] undocumented
pub mod SMPH26;
/// MCU SEMAPHORE 27
///
/// [TI-TRM-I] undocumented
pub mod SMPH27;
/// MCU SEMAPHORE 28
///
/// [TI-TRM-I] undocumented
pub mod SMPH28;
/// MCU SEMAPHORE 29
///
/// [TI-TRM-I] undocumented
pub mod SMPH29;
/// MCU SEMAPHORE 3
///
/// [TI-TRM-I] undocumented
pub mod SMPH3;
/// MCU SEMAPHORE 30
///
/// [TI-TRM-I] undocumented
pub mod SMPH30;
/// MCU SEMAPHORE 31
///
/// [TI-TRM-I] undocumented
pub mod SMPH31;
/// MCU SEMAPHORE 4
///
/// [TI-TRM-I] undocumented
pub mod SMPH4;
/// MCU SEMAPHORE 5
///
/// [TI-TRM-I] undocumented
pub mod SMPH5;
/// MCU SEMAPHORE 6
///
/// [TI-TRM-I] undocumented
pub mod SMPH6;
/// MCU SEMAPHORE 7
///
/// [TI-TRM-I] undocumented
pub mod SMPH7;
/// MCU SEMAPHORE 8
///
/// [TI-TRM-I] undocumented
pub mod SMPH8;
/// MCU SEMAPHORE 9
///
/// [TI-TRM-I] undocumented
pub mod SMPH9;
