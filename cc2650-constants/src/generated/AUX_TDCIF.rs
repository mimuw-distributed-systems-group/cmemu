use cmemu_common::Address;
use core::ops::Range;

pub const BASE_ADDR: Address = Address::from_const(0x400c4000);
pub const ADDR: Address = BASE_ADDR.offset(0x0);
pub const SIZE: u32 = 0x1000;
/// 0x400c4000..0x400c5000
pub const ADDR_SPACE: Range<Address> = ADDR..ADDR.offset(SIZE);

/// Control
///
/// [TI-TRM-I] 17.7.5.1 CTL Register (Offset = 0h) [reset = 0h]
pub mod CTL;
/// Prescaler Counter
///
/// [TI-TRM-I] 17.7.5.10 PRECNT Register (Offset = 24h) [reset = 0h]
pub mod PRECNT;
/// Prescaler Control
///
///
///
/// The prescaler can be used to count events that are faster than the AUX clock frequency.
///
/// It can be used to:
///
/// - count pulses on a specified event from the asynchronous event bus.
///
/// - prescale a specified event from the asynchronous event bus.
///
///
///
/// To use the prescaler output as an event source in TDC measurements you must set both TRIGSRC.START_SRC and TRIGSRC.STOP_SRC to AUX_TDC_PRE.
///
///
///
/// It is recommended to use the prescaler when the signal frequency to measure exceeds 1/10th of the AUX clock frequency.
///
/// [TI-TRM-I] 17.7.5.9 PRECTL Register (Offset = 20h) [reset = 1Fh]
pub mod PRECTL;
/// Result
///
///
///
/// Result of last TDC conversion
///
/// [TI-TRM-I] 17.7.5.3 RESULT Register (Offset = 8h) [reset = 2h]
pub mod RESULT;
/// Saturation Configuration
///
/// [TI-TRM-I] 17.7.5.4 SATCFG Register (Offset = Ch) [reset = Fh]
pub mod SATCFG;
/// Status
///
/// [TI-TRM-I] 17.7.5.2 STAT Register (Offset = 4h) [reset = 6h]
pub mod STAT;
/// Trigger Counter
///
///
///
/// Stop-counter control and status.
///
/// [TI-TRM-I] 17.7.5.6 TRIGCNT Register (Offset = 14h) [reset = 0h]
pub mod TRIGCNT;
/// Trigger Counter Configuration
///
///
///
/// Stop-counter configuration.
///
/// [TI-TRM-I] 17.7.5.8 TRIGCNTCFG Register (Offset = 1Ch) [reset = 0h]
pub mod TRIGCNTCFG;
/// Trigger Counter Load
///
///
///
/// Stop-counter load.
///
/// [TI-TRM-I] 17.7.5.7 TRIGCNTLOAD Register (Offset = 18h) [reset = 0h]
pub mod TRIGCNTLOAD;
/// Trigger Source
///
///
///
/// Select source and polarity for TDC start and stop events. See the Technical Reference Manual for event timing requirements.
///
/// [TI-TRM-I] 17.7.5.5 TRIGSRC Register (Offset = 10h) [reset = 0h]
pub mod TRIGSRC;
