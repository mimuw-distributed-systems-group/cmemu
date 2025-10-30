use cmemu_common::Address;
use core::ops::Range;

pub const BASE_ADDR: Address = Address::from_const(0x40093000);
pub const ADDR: Address = BASE_ADDR.offset(0x0);
pub const SIZE: u32 = 0x400;
/// 0x40093000..0x40093400
pub const ADDR_SPACE: Range<Address> = ADDR..ADDR.offset(SIZE);

/// Wake-up Selector For AUX
///
///
///
/// This register contains pointers to 3 events which are routed to AON_WUC as wakeup sources for AUX. AON_WUC will start a wakeup sequence for the AUX domain when either of the 3 selected events are asserted. A wakeup sequence will guarantee that the AUX power switches are turned on, LDO resources are available and SCLK_HF is available and selected as clock source for AUX.
///
///
///
/// Note: It is recommended ( or required when AON_WUC:AUXCLK.PWR_DWN_SRC=NONE) to also setup a wakeup event here before AUX is requesting powerdown. ( AUX_WUC:PWRDWNREQ.REQ is asserted\] ) as it will speed up the wakeup procedure.
///
/// [TI-TRM-I] 4.7.1.2 AUXWUSEL Register (Offset = 4h) [reset = 003F3F3Fh]
pub mod AUXWUSEL;
/// Event Selector For MCU Event Fabric
///
///
///
/// This register contains pointers for 3 AON events that are routed to the MCU Event Fabric EVENT
///
/// [TI-TRM-I] 4.7.1.3 EVTOMCUSEL Register (Offset = 8h) [reset = 002B2B2Bh]
pub mod EVTOMCUSEL;
/// Wake-up Selector For MCU
///
///
///
/// This register contains pointers to 4 events which are routed to AON_WUC as wakeup sources for MCU. AON_WUC will start a wakeup sequence for the MCU domain when either of the 4 selected events are asserted. A wakeup sequence will guarantee that the MCU power switches are turned on, LDO resources are available and SCLK_HF is available and selected as clock source for MCU.
///
///
///
/// Note: It is recommended ( or required when AON_WUC:MCUCLK.PWR_DWN_SRC=NONE) to also setup a wakeup event here before MCU is requesting powerdown. ( PRCM requests uLDO, see conditions in PRCM:VDCTL.ULDO ) as it will speed up the wakeup procedure.
///
/// [TI-TRM-I] 4.7.1.1 MCUWUSEL Register (Offset = 0h) [reset = 3F3F3F3Fh]
pub mod MCUWUSEL;
/// RTC Capture Event Selector For AON_RTC
///
///
///
/// This register contains a pointer to select an AON event for RTC capture. Please refer to AON_RTC:CH1CAPT
///
/// [TI-TRM-I] 4.7.1.4 RTCSEL Register (Offset = Ch) [reset = 3Fh]
pub mod RTCSEL;
