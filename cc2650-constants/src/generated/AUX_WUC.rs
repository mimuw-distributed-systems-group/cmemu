use cmemu_common::Address;
use core::ops::Range;

pub const BASE_ADDR: Address = Address::from_const(0x400c6000);
pub const ADDR: Address = BASE_ADDR.offset(0x0);
pub const SIZE: u32 = 0x1000;
/// 0x400c6000..0x400c7000
pub const ADDR_SPACE: Range<Address> = ADDR..ADDR.offset(SIZE);

/// ADC Clock Control
///
///
///
/// Controls the ADC internal clock
///
///
///
/// Note that the ADC command and data interface requires MODCLKEN0.ANAIF or MODCLKEN1.ANAIF also to be set
///
/// [TI-TRM-I] 17.7.7.9 ADCCLKCTL Register (Offset = 30h) [reset = 0h]
pub mod ADCCLKCTL;
/// AON Domain Control Status
///
///
///
/// Status of AUX domain control from AON_WUC.
///
/// [TI-TRM-I] 17.7.7.17 AONCTLSTAT Register (Offset = 50h) [reset = 0h]
pub mod AONCTLSTAT;
/// AUX Input Output Latch
///
///
///
/// Controls latching of signals between AUX_AIODIO0/AUX_AIODIO1 and AON_IOC.
///
/// [TI-TRM-I] 17.7.7.18 AUXIOLATCH Register (Offset = 54h) [reset = 0h]
pub mod AUXIOLATCH;
/// Low Frequency Clock Acknowledgment
///
/// [TI-TRM-I] 17.7.7.6 CLKLFACK Register (Offset = 14h) [reset = 0h]
pub mod CLKLFACK;
/// Low Frequency Clock Request
///
/// [TI-TRM-I] 17.7.7.5 CLKLFREQ Register (Offset = 10h) [reset = 0h]
pub mod CLKLFREQ;
/// MCU Bus Control
///
///
///
/// Controls the connection between the AUX domain bus and the MCU domain bus.
///
///
///
/// The buses must be disconnected to allow power-down or power-off of the AUX domain.
///
/// [TI-TRM-I] 17.7.7.15 MCUBUSCTL Register (Offset = 48h) [reset = 0h]
pub mod MCUBUSCTL;
/// MCU Bus Status
///
///
///
/// Indicates the connection state of the AUX domain and MCU domain buses.
///
///
///
/// Note that this register cannot be read from the MCU domain while disconnected, and is therefore only useful for the AUX_SCE.
///
/// [TI-TRM-I] 17.7.7.16 MCUBUSSTAT Register (Offset = 4Ch) [reset = 0h]
pub mod MCUBUSSTAT;
/// Module Clock Enable
///
///
///
/// Clock enable for each module in the AUX domain
///
///
///
/// For use by the system CPU
///
///
///
/// The settings in this register are OR'ed with the corresponding settings in MODCLKEN1. This allows the system CPU and AUX_SCE to request clocks independently. Settings take effect immediately.
///
/// [TI-TRM-I] 17.7.7.1 MODCLKEN0 Register (Offset = 0h) [reset = 0h]
pub mod MODCLKEN0;
/// Module Clock Enable 1
///
///
///
/// Clock enable for each module in the AUX domain, for use by the AUX_SCE. Settings take effect immediately.
///
///
///
/// The settings in this register are OR'ed with the corresponding settings in MODCLKEN0. This allows system CPU and AUX_SCE to request clocks independently.
///
/// [TI-TRM-I] 17.7.7.19 MODCLKEN1 Register (Offset = 5Ch) [reset = 0h]
pub mod MODCLKEN1;
/// Power Down Acknowledgment
///
/// [TI-TRM-I] 17.7.7.4 PWRDWNACK Register (Offset = Ch) [reset = 0h]
pub mod PWRDWNACK;
/// Power Down Request
///
///
///
/// Request from AUX for system to enter power down. When system is in power down there is limited current supply available and the clock source is set by AON_WUC:AUXCLK.PWR_DWN_SRC
///
/// [TI-TRM-I] 17.7.7.3 PWRDWNREQ Register (Offset = 8h) [reset = 0h]
pub mod PWRDWNREQ;
/// Power Off Request
///
///
///
/// Requests power off request for the AUX domain. When powered off, the power supply and clock is disabled. This may only be used when taking the entire device into shutdown mode (i.e. with full device reset when resuming operation).
///
///
///
/// Power off is prevented if AON_WUC:AUXCTL.AUX_FORCE_ON has been set, or if MCUBUSCTL.DISCONNECT_REQ has been cleared.
///
/// [TI-TRM-I] 17.7.7.2 PWROFFREQ Register (Offset = 4h) [reset = 0h]
pub mod PWROFFREQ;
/// Reference Clock Control
///
///
///
/// Controls the TDC reference clock source, which is to be compared against the TDC counter clock.
///
///
///
/// The source of this clock is controlled by OSC_DIG:CTL0.ACLK_REF_SRC_SEL.
///
/// [TI-TRM-I] 17.7.7.11 REFCLKCTL Register (Offset = 38h) [reset = 0h]
pub mod REFCLKCTL;
/// Real Time Counter Sub Second Increment 0
///
///
///
/// New value for the real-time counter (AON_RTC) sub-second increment value, part corresponding to AON_RTC:SUBSECINC bits 15:0.
///
///
///
/// After setting INC15_0 and RTCSUBSECINC1.INC23_16, the value is loaded into AON_RTC:SUBSECINC.VALUEINC by setting RTCSUBSECINCCTL.UPD_REQ.
///
/// [TI-TRM-I] 17.7.7.12 RTCSUBSECINC0 Register (Offset = 3Ch) [reset = 0h]
pub mod RTCSUBSECINC0;
/// Real Time Counter Sub Second Increment 1
///
///
///
/// New value for the real-time counter (AON_RTC) sub-second increment value, part corresponding to AON_RTC:SUBSECINC bits 23:16.
///
///
///
/// After setting RTCSUBSECINC0.INC15_0 and INC23_16, the value is loaded into AON_RTC:SUBSECINC.VALUEINC by setting RTCSUBSECINCCTL.UPD_REQ.
///
/// [TI-TRM-I] 17.7.7.13 RTCSUBSECINC1 Register (Offset = 40h) [reset = 0h]
pub mod RTCSUBSECINC1;
/// Real Time Counter Sub Second Increment Control
///
/// [TI-TRM-I] 17.7.7.14 RTCSUBSECINCCTL Register (Offset = 44h) [reset = 0h]
pub mod RTCSUBSECINCCTL;
/// TDC Clock Control
///
///
///
/// Controls the TDC counter clock source, which steps the TDC counter value
///
///
///
/// The source of this clock is controlled by OSC_DIG:CTL0.ACLK_TDC_SRC_SEL.
///
/// [TI-TRM-I] 17.7.7.10 TDCCLKCTL Register (Offset = 34h) [reset = 0h]
pub mod TDCCLKCTL;
/// Wake-up Event Clear
///
///
///
/// Clears wake-up events from the AON domain
///
/// [TI-TRM-I] 17.7.7.8 WUEVCLR Register (Offset = 2Ch) [reset = 0h]
pub mod WUEVCLR;
/// Wake-up Event Flags
///
///
///
/// Status of wake-up events from the AON domain
///
///
///
/// The event flags are cleared by setting the corresponding bits in WUEVCLR
///
/// [TI-TRM-I] 17.7.7.7 WUEVFLAGS Register (Offset = 28h) [reset = 0h]
pub mod WUEVFLAGS;
