use cmemu_common::Address;
use core::ops::Range;

pub const BASE_ADDR: Address = Address::from_const(0x400ca000);
pub const ADDR: Address = BASE_ADDR.offset(0x0);
pub const SIZE: u32 = 0x1000;
/// 0x400ca000..0x400cb000
pub const ADDR_SPACE: Range<Address> = ADDR..ADDR.offset(SIZE);

/// ADC Doubler Nanoamp Control
///
/// [TI-TRM-I] 6.8.2.1.10 ADCDOUBLERNANOAMPCTL Register (Offset = 24h) [reset = 0h]
pub mod ADCDOUBLERNANOAMPCTL;
/// Amplitude Compensation Control
///
/// [TI-TRM-I] 6.8.2.1.4 AMPCOMPCTL Register (Offset = Ch) [reset = 0h]
pub mod AMPCOMPCTL;
/// Amplitude Compensation Threshold 1
///
/// This register contains threshold values for amplitude compensation algorithm
///
/// [TI-TRM-I] 6.8.2.1.5 AMPCOMPTH1 Register (Offset = 10h) [reset = 0h]
pub mod AMPCOMPTH1;
/// Amplitude Compensation Threshold 2
///
/// This register contains threshold values for amplitude compensation algorithm.
///
/// [TI-TRM-I] 6.8.2.1.6 AMPCOMPTH2 Register (Offset = 14h) [reset = 0h]
pub mod AMPCOMPTH2;
/// Analog Bypass Values 1
///
/// [TI-TRM-I] 6.8.2.1.7 ANABYPASSVAL1 Register (Offset = 18h) [reset = 0h]
pub mod ANABYPASSVAL1;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 6.8.2.1.8 ANABYPASSVAL2 Register (Offset = 1Ch) [reset = 0h]
pub mod ANABYPASSVAL2;
/// Analog Test Control
///
/// [TI-TRM-I] 6.8.2.1.9 ATESTCTL Register (Offset = 20h) [reset = 0h]
pub mod ATESTCTL;
/// Control 0
///
/// Controls clock source selects
///
/// [TI-TRM-I] 6.8.2.1.1 CTL0 Register (Offset = 0h) [reset = 0h]
pub mod CTL0;
/// Control 1
///
/// This register contains OSC_DIG configuration
///
/// [TI-TRM-I] 6.8.2.1.2 CTL1 Register (Offset = 4h) [reset = 0h]
pub mod CTL1;
/// Low Frequency Oscillator Control
///
/// [TI-TRM-I] 6.8.2.1.12 LFOSCCTL Register (Offset = 2Ch) [reset = 0h]
pub mod LFOSCCTL;
/// RADC External Configuration
///
/// [TI-TRM-I] 6.8.2.1.3 RADCEXTCFG Register (Offset = 8h) [reset = 0h]
pub mod RADCEXTCFG;
/// RCOSCHF Control
///
/// [TI-TRM-I] 6.8.2.1.13 RCOSCHFCTL Register (Offset = 30h) [reset = 0h]
pub mod RCOSCHFCTL;
/// Status 0
///
/// This register contains status signals from OSC_DIG
///
/// [TI-TRM-I] 6.8.2.1.14 STAT0 Register (Offset = 34h) [reset = 0h]
pub mod STAT0;
/// Status 1
///
/// This register contains status signals from OSC_DIG
///
/// [TI-TRM-I] 6.8.2.1.15 STAT1 Register (Offset = 38h) [reset = 0h]
pub mod STAT1;
/// Status 2
///
/// This register contains status signals from AMPCOMP FSM
///
/// [TI-TRM-I] 6.8.2.1.16 STAT2 Register (Offset = 3Ch) [reset = 0h]
pub mod STAT2;
/// XOSCHF Control
///
/// [TI-TRM-I] 6.8.2.1.11 XOSCHFCTL Register (Offset = 28h) [reset = 0h]
pub mod XOSCHFCTL;
