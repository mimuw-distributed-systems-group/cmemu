use cmemu_common::Address;
use core::ops::Range;

pub const BASE_ADDR: Address = Address::from_const(0x40091000);
pub const ADDR: Address = BASE_ADDR.offset(0x0);
pub const SIZE: u32 = 0x1000;
/// 0x40091000..0x40092000
pub const ADDR_SPACE: Range<Address> = ADDR..ADDR.offset(SIZE);

/// AUX Configuration
///
///
///
/// This register contains power management related signals for the AUX domain.
///
/// [TI-TRM-I] 6.8.2.3.4 AUXCFG Register (Offset = Ch) [reset = 1h]
pub mod AUXCFG;
/// AUX Clock Management
///
///
///
/// This register contains bitfields that are relevant for setting up the clock to the AUX domain.
///
/// [TI-TRM-I] 6.8.2.3.2 AUXCLK Register (Offset = 4h) [reset = 1h]
pub mod AUXCLK;
/// AUX Control
///
///
///
/// This register contains events and control signals for the AUX domain.
///
/// [TI-TRM-I] 6.8.2.3.5 AUXCTL Register (Offset = 10h) [reset = 0h]
pub mod AUXCTL;
/// Control 0
///
///
///
/// This register contains various chip level control and debug bitfields.
///
/// [TI-TRM-I] 6.8.2.3.8 CTL0 Register (Offset = 20h) [reset = 0h]
pub mod CTL0;
/// Control 1
///
///
///
/// This register contains various chip level control and debug bitfields.
///
/// [TI-TRM-I] 6.8.2.3.9 CTL1 Register (Offset = 24h) [reset = 0h]
pub mod CTL1;
/// JTAG Configuration
///
///
///
/// This register contains control for configuration of the JTAG domain,- hereunder access permissions for each TAP.
///
/// [TI-TRM-I] 6.8.2.3.13 JTAGCFG Register (Offset = 40h) [reset = 100h]
pub mod JTAGCFG;
/// JTAG USERCODE
///
///
///
/// Boot code copies the JTAG USERCODE to this register from where it is forwarded to the debug subsystem.
///
/// [TI-TRM-I] 6.8.2.3.14 JTAGUSERCODE Register (Offset = 44h) [reset = 0B99A02Fh]
pub mod JTAGUSERCODE;
/// MCU Configuration
///
///
///
/// This register contains power management related bitfields for the MCU domain.
///
/// [TI-TRM-I] 6.8.2.3.3 MCUCFG Register (Offset = 8h) [reset = Fh]
pub mod MCUCFG;
/// MCU Clock Management
///
///
///
/// This register contains bitfields related to the MCU clock.
///
/// [TI-TRM-I] 6.8.2.3.1 MCUCLK Register (Offset = 0h) [reset = 0h]
pub mod MCUCLK;
/// Oscillator Configuration
///
///
///
/// This register sets the period for Amplitude compensation requests sent to the oscillator control system. The amplitude compensations is only applicable when XOSC_HF is running in low power mode.
///
/// [TI-TRM-I] 6.8.2.3.12 OSCCFG Register (Offset = 38h) [reset = 0h]
pub mod OSCCFG;
/// Power Status
///
///
///
/// This register is used to monitor various power management related signals in AON.  Most signals are for test, calibration and debug purpose only, and others can be used to detect that AUX or JTAG domains are powered up.
///
/// [TI-TRM-I] 6.8.2.3.6 PWRSTAT Register (Offset = 14h) [reset = 03800000h]
pub mod PWRSTAT;
/// Recharge Controller Configuration
///
///
///
/// This register sets all relevant patameters for controlling the recharge algorithm.
///
/// [TI-TRM-I] 6.8.2.3.10 RECHARGECFG Register (Offset = 30h) [reset = 0h]
pub mod RECHARGECFG;
/// Recharge Controller Status
///
///
///
/// This register controls various status registers which are updated during recharge.  The register is mostly intended for test and debug.
///
/// [TI-TRM-I] 6.8.2.3.11 RECHARGESTAT Register (Offset = 34h) [reset = 0h]
pub mod RECHARGESTAT;
/// Shutdown Control
///
///
///
/// This register contains bitfields required for entering shutdown mode
///
/// [TI-TRM-I] 6.8.2.3.7 SHUTDOWN Register (Offset = 18h) [reset = 0h]
pub mod SHUTDOWN;
