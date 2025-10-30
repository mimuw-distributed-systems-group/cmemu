use cmemu_common::Address;
use core::ops::Range;

pub const BASE_ADDR: Address = Address::from_const(0x40028000);
pub const ADDR: Address = BASE_ADDR.offset(0x0);
pub const SIZE: u32 = 0x2000;
/// 0x40028000..0x4002a000
pub const ADDR_SPACE: Range<Address> = ADDR..ADDR.offset(SIZE);

/// Alarm Control
///
/// [TI-TRM-I] 16.7.1.8 ALARMCNT Register (Offset = 1Ch) [reset = FFh]
pub mod ALARMCNT;
/// Alarm Event
///
/// [TI-TRM-I] 16.7.1.11 ALARMMASK Register (Offset = 28h) [reset = 0h]
pub mod ALARMMASK;
/// Alarm Shutdown
///
/// [TI-TRM-I] 16.7.1.12 ALARMSTOP Register (Offset = 2Ch) [reset = 0h]
pub mod ALARMSTOP;
/// Configuration 0
///
/// [TI-TRM-I] 16.7.1.7 CFG0 Register (Offset = 18h) [reset = 0h]
pub mod CFG0;
/// Control
///
/// [TI-TRM-I] 16.7.1.6 CTL Register (Offset = 14h) [reset = 0h]
pub mod CTL;
/// FRO De-tune Bit
///
/// [TI-TRM-I] 16.7.1.10 FRODETUNE Register (Offset = 24h) [reset = 0h]
pub mod FRODETUNE;
/// FRO Enable
///
/// [TI-TRM-I] 16.7.1.9 FROEN Register (Offset = 20h) [reset = 00FFFFFFh]
pub mod FROEN;
/// TRNG Engine Options Information
///
/// [TI-TRM-I] 16.7.1.16 HWOPT Register (Offset = 78h) [reset = 600h]
pub mod HWOPT;
/// HW Version 0
///
/// EIP Number And Core Revision
///
/// [TI-TRM-I] 16.7.1.17 HWVER0 Register (Offset = 7Ch) [reset = 0200B44Bh]
pub mod HWVER0;
/// HW Version 1
///
/// TRNG Revision Number
///
/// [TI-TRM-I] 16.7.1.19 HWVER1 Register (Offset = 1FE0h) [reset = 20h]
pub mod HWVER1;
/// Interrupt Flag Clear
///
/// [TI-TRM-I] 16.7.1.5 IRQFLAGCLR Register (Offset = 10h) [reset = 0h]
pub mod IRQFLAGCLR;
/// Interrupt Mask
///
/// [TI-TRM-I] 16.7.1.4 IRQFLAGMASK Register (Offset = Ch) [reset = 0h]
pub mod IRQFLAGMASK;
/// Interrupt Status
///
/// [TI-TRM-I] 16.7.1.3 IRQFLAGSTAT Register (Offset = 8h) [reset = 0h]
pub mod IRQFLAGSTAT;
/// Interrupt Set
///
/// [TI-TRM-I] 16.7.1.20 IRQSET Register (Offset = 1FECh) [reset = 0h]
pub mod IRQSET;
/// Interrupt Status
///
/// [TI-TRM-I] 16.7.1.22 IRQSTAT Register (Offset = 1FF8h) [reset = 0h]
pub mod IRQSTAT;
/// Interrupt Status After Masking
///
/// [TI-TRM-I] 16.7.1.18 IRQSTATMASK Register (Offset = 1FD8h) [reset = 0h]
pub mod IRQSTATMASK;
/// LFSR Readout Value
///
/// [TI-TRM-I] 16.7.1.13 LFSR0 Register (Offset = 30h) [reset = 0h]
pub mod LFSR0;
/// LFSR Readout Value
///
/// [TI-TRM-I] 16.7.1.14 LFSR1 Register (Offset = 34h) [reset = 0h]
pub mod LFSR1;
/// LFSR Readout Value
///
/// [TI-TRM-I] 16.7.1.15 LFSR2 Register (Offset = 38h) [reset = 0h]
pub mod LFSR2;
/// Random Number Lower Word Readout Value
///
/// [TI-TRM-I] 16.7.1.1 OUT0 Register (Offset = 0h) [reset = 0h]
pub mod OUT0;
/// Random Number Upper Word Readout Value
///
/// [TI-TRM-I] 16.7.1.2 OUT1 Register (Offset = 4h) [reset = 0h]
pub mod OUT1;
/// SW Reset Control
///
/// [TI-TRM-I] 16.7.1.21 SWRESET Register (Offset = 1FF0h) [reset = 0h]
pub mod SWRESET;
