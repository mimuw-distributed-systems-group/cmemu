use cmemu_common::Address;
use core::ops::Range;

pub const BASE_ADDR: Address = Address::from_const(0x40022000);
pub const ADDR: Address = BASE_ADDR.offset(0x0);
pub const SIZE: u32 = 0x400;
/// 0x40022000..0x40022400
pub const ADDR_SPACE: Range<Address> = ADDR..ADDR.offset(SIZE);

/// Data Input from DIO 0 to 31
///
/// [TI-TRM-I] 11.11.2.13 DIN31_0 Register (Offset = C0h) [reset = 0h]
pub mod DIN31_0;
/// Data Output Enable for DIO 0 to 31
///
/// [TI-TRM-I] 11.11.2.14 DOE31_0 Register (Offset = D0h) [reset = 0h]
pub mod DOE31_0;
/// Data Out 8 to 11
///
///
///
/// Alias register for byte access to each bit in DOUT31_0
///
/// [TI-TRM-I] 11.11.2.3 DOUT11_8 Register (Offset = 8h) [reset = 0h]
pub mod DOUT11_8;
/// Data Out 12 to 15
///
///
///
/// Alias register for byte access to each bit in DOUT31_0
///
/// [TI-TRM-I] 11.11.2.4 DOUT15_12 Register (Offset = Ch) [reset = 0h]
pub mod DOUT15_12;
/// Data Out 16 to 19
///
///
///
/// Alias register for byte access to each bit in DOUT31_0
///
/// [TI-TRM-I] 11.11.2.5 DOUT19_16 Register (Offset = 10h) [reset = 0h]
pub mod DOUT19_16;
/// Data Out 20 to 23
///
///
///
/// Alias register for byte access to each bit in DOUT31_0
///
/// [TI-TRM-I] 11.11.2.6 DOUT23_20 Register (Offset = 14h) [reset = 0h]
pub mod DOUT23_20;
/// Data Out 24 to 27
///
///
///
/// Alias register for byte access to each bit in DOUT31_0
///
/// [TI-TRM-I] 11.11.2.7 DOUT27_24 Register (Offset = 18h) [reset = 0h]
pub mod DOUT27_24;
/// Data Output for DIO 0 to 31
///
/// [TI-TRM-I] 11.11.2.9 DOUT31_0 Register (Offset = 80h) [reset = 0h]
pub mod DOUT31_0;
/// Data Out 28 to 31
///
///
///
/// Alias register for byte access to each bit in DOUT31_0
///
/// [TI-TRM-I] 11.11.2.8 DOUT31_28 Register (Offset = 1Ch) [reset = 0h]
pub mod DOUT31_28;
/// Data Out 0 to 3
///
///
///
/// Alias register for byte access to each bit in DOUT31_0
///
/// [TI-TRM-I] 11.11.2.1 DOUT3_0 Register (Offset = 0h) [reset = 0h]
pub mod DOUT3_0;
/// Data Out 4 to 7
///
///
///
/// Alias register for byte access to each bit in DOUT31_0
///
/// [TI-TRM-I] 11.11.2.2 DOUT7_4 Register (Offset = 4h) [reset = 0h]
pub mod DOUT7_4;
/// Data Out Clear
///
///
///
/// Writing 1 to a bit position clears the corresponding bit in the DOUT31_0 register
///
/// [TI-TRM-I] 11.11.2.11 DOUTCLR31_0 Register (Offset = A0h) [reset = 0h]
pub mod DOUTCLR31_0;
/// Data Out Set
///
///
///
/// Writing 1 to a bit position sets the corresponding bit in the DOUT31_0 register
///
/// [TI-TRM-I] 11.11.2.10 DOUTSET31_0 Register (Offset = 90h) [reset = 0h]
pub mod DOUTSET31_0;
/// Data Out Toggle
///
///
///
/// Writing 1 to a bit position will invert the corresponding DIO output.
///
/// [TI-TRM-I] 11.11.2.12 DOUTTGL31_0 Register (Offset = B0h) [reset = 0h]
pub mod DOUTTGL31_0;
/// Event Register for DIO 0 to 31
///
///
///
/// Reading  this registers will return 1 for triggered event and 0 for non-triggered events.
///
/// Writing a 1 to a bit field will clear the event.
///
///
///
/// The configuration of events is done inside MCU IOC, e.g. events for DIO #0 is configured in IOC:IOCFG0.EDGE_DET and IOC:IOCFG0.EDGE_IRQ_EN.
///
/// [TI-TRM-I] 11.11.2.15 EVFLAGS31_0 Register (Offset = E0h) [reset = 0h]
pub mod EVFLAGS31_0;
