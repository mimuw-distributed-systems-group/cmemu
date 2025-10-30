use cmemu_common::Address;
use core::ops::Range;

pub const BASE_ADDR: Address = Address::from_const(0x40041000);
pub const ADDR: Address = BASE_ADDR.offset(0x0);
pub const SIZE: u32 = 0x40;
/// 0x40041000..0x40041040
pub const ADDR_SPACE: Range<Address> = ADDR..ADDR.offset(SIZE);

/// Doorbell Command Register
///
/// [TI-TRM-I] 23.8.2.1 CMDR Register (Offset = 0h) [reset = 0h]
pub mod CMDR;
/// Doorbell Command Status Register
///
/// [TI-TRM-I] 23.8.2.2 CMDSTA Register (Offset = 4h) [reset = 0h]
pub mod CMDSTA;
/// Doorbell Command Acknowledgement Interrupt Flag
///
/// [TI-TRM-I] 23.8.2.8 RFACKIFG Register (Offset = 1Ch) [reset = 0h]
pub mod RFACKIFG;
/// Interrupt Enable For Command and Packet Engine Generated Interrupts
///
/// [TI-TRM-I] 23.8.2.6 RFCPEIEN Register (Offset = 14h) [reset = FFFFFFFFh]
pub mod RFCPEIEN;
/// Interrupt Flags For Command and Packet Engine Generated Interrupts
///
/// [TI-TRM-I] 23.8.2.5 RFCPEIFG Register (Offset = 10h) [reset = 0h]
pub mod RFCPEIFG;
/// Interrupt Vector Selection For Command and Packet Engine Generated Interrupts
///
/// [TI-TRM-I] 23.8.2.7 RFCPEISL Register (Offset = 18h) [reset = FFFF0000h]
pub mod RFCPEISL;
/// Interrupt Enable For RF Hardware Modules
///
/// [TI-TRM-I] 23.8.2.4 RFHWIEN Register (Offset = Ch) [reset = 0h]
pub mod RFHWIEN;
/// Interrupt Flags From RF Hardware Modules
///
/// [TI-TRM-I] 23.8.2.3 RFHWIFG Register (Offset = 8h) [reset = 0h]
pub mod RFHWIFG;
/// RF Core General Purpose Output Control
///
/// [TI-TRM-I] 23.8.2.9 SYSGPOCTL Register (Offset = 20h) [reset = 0h]
pub mod SYSGPOCTL;
