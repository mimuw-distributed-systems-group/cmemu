use cmemu_common::Address;
use core::ops::Range;

pub const BASE_ADDR: Address = Address::from_const(0x400c9000);
pub const ADDR: Address = BASE_ADDR.offset(0x0);
pub const SIZE: u32 = 0x1000;
/// 0x400c9000..0x400ca000
pub const ADDR_SPACE: Range<Address> = ADDR..ADDR.offset(SIZE);

/// ADC Control
///
///
///
/// Configuration of ADI_4_AUX:ADC0.SMPL_MODE decides if the ADC trigger starts sampling or conversion.
///
/// [TI-TRM-I] 17.7.8.1 ADCCTL Register (Offset = 10h) [reset = 0h]
pub mod ADCCTL;
/// ADC FIFO
///
/// [TI-TRM-I] 17.7.8.3 ADCFIFO Register (Offset = 18h) [reset = 0h]
pub mod ADCFIFO;
/// ADC FIFO Status
///
///
///
/// FIFO can hold up to four ADC samples.
///
/// [TI-TRM-I] 17.7.8.2 ADCFIFOSTAT Register (Offset = 14h) [reset = 1h]
pub mod ADCFIFOSTAT;
/// ADC Trigger
///
/// [TI-TRM-I] 17.7.8.4 ADCTRIG Register (Offset = 1Ch) [reset = 0h]
pub mod ADCTRIG;
/// Current Source Control
///
/// [TI-TRM-I] 17.7.8.5 ISRCCTL Register (Offset = 20h) [reset = 1h]
pub mod ISRCCTL;
