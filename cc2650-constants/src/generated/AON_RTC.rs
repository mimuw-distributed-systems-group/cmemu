use cmemu_common::Address;
use core::ops::Range;

pub const BASE_ADDR: Address = Address::from_const(0x40092000);
pub const ADDR: Address = BASE_ADDR.offset(0x0);
pub const SIZE: u32 = 0x400;
/// 0x40092000..0x40092400
pub const ADDR_SPACE: Range<Address> = ADDR..ADDR.offset(SIZE);

/// Channel 0 Compare Value
///
/// [TI-TRM-I] 14.4.1.7 CH0CMP Register (Offset = 18h) [reset = 0h]
pub mod CH0CMP;
/// Channel 1 Capture Value
///
///
///
/// If CHCTL.CH1_EN = 1and CHCTL.CH1_CAPT_EN = 1, capture occurs on each rising edge of the event selected in AON_EVENT:RTCSEL.
///
/// [TI-TRM-I] 14.4.1.11 CH1CAPT Register (Offset = 28h) [reset = 0h]
pub mod CH1CAPT;
/// Channel 1 Compare Value
///
/// [TI-TRM-I] 14.4.1.8 CH1CMP Register (Offset = 1Ch) [reset = 0h]
pub mod CH1CMP;
/// Channel 2 Compare Value
///
/// [TI-TRM-I] 14.4.1.9 CH2CMP Register (Offset = 20h) [reset = 0h]
pub mod CH2CMP;
/// Channel 2 Compare Value Auto-increment
///
///
///
/// This register is primarily used to generate periodical wake-up for the AUX_SCE module, through the \[AUX_EVCTL.EVSTAT0.AON_RTC\] event.
///
/// [TI-TRM-I] 14.4.1.10 CH2CMPINC Register (Offset = 24h) [reset = 0h]
pub mod CH2CMPINC;
/// Channel Configuration
///
/// [TI-TRM-I] 14.4.1.6 CHCTL Register (Offset = 14h) [reset = 0h]
pub mod CHCTL;
/// Control
///
///
///
/// This register contains various  bitfields for configuration of RTC
///
/// [TI-TRM-I] 14.4.1.1 CTL Register (Offset = 0h) [reset = 0h]
pub mod CTL;
/// Event Flags, RTC Status
///
///
///
/// This register contains event flags from the 3 RTC channels. Each flag will be cleared when writing a '1' to the corresponding bitfield.
///
/// [TI-TRM-I] 14.4.1.2 EVFLAGS Register (Offset = 4h) [reset = 0h]
pub mod EVFLAGS;
/// Second Counter Value, Integer Part
///
/// [TI-TRM-I] 14.4.1.3 SEC Register (Offset = 8h) [reset = 0h]
pub mod SEC;
/// Second Counter Value, Fractional Part
///
/// [TI-TRM-I] 14.4.1.4 SUBSEC Register (Offset = Ch) [reset = 0h]
pub mod SUBSEC;
/// Subseconds Increment
///
/// Value added to SUBSEC.VALUE on every SCLK_LFclock cycle.
///
/// [TI-TRM-I] 14.4.1.5 SUBSECINC Register (Offset = 10h) [reset = 00800000h]
pub mod SUBSECINC;
/// AON Synchronization
///
///
///
/// This register is used for synchronizing between MCU and entire AON domain.
///
/// [TI-TRM-I] 14.4.1.12 SYNC Register (Offset = 2Ch) [reset = 0h]
pub mod SYNC;
