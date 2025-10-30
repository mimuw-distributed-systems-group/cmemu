use cmemu_common::Address;
use core::ops::Range;

pub const BASE_ADDR: Address = Address::from_const(0x400cb000);
pub const ADDR: Address = BASE_ADDR.offset(0x0);
pub const SIZE: u32 = 0x200;
/// 0x400cb000..0x400cb200
pub const ADDR_SPACE: Range<Address> = ADDR..ADDR.offset(SIZE);

/// ADC Control 0
///
///
///
/// ADC Sample Control. Only to be used through TI provided API.
///
/// [TI-TRM-I] 17.7.1.8 ADC0 Register (Offset = 8h) [reset = 0h]
pub mod ADC0;
/// ADC Control 1
///
///
///
/// ADC Comparator Control. Only to be used through TI provided API.
///
/// [TI-TRM-I] 17.7.1.9 ADC1 Register (Offset = 9h) [reset = 0h]
pub mod ADC1;
/// ADC Reference 0
///
///
///
/// Control reference used by the ADC. Only to be used through TI provided API.
///
/// [TI-TRM-I] 17.7.1.10 ADCREF0 Register (Offset = Ah) [reset = 0h]
pub mod ADCREF0;
/// ADC Reference 1
///
///
///
/// Control reference used by the ADC. Only to be used through TI provided API.
///
/// [TI-TRM-I] 17.7.1.11 ADCREF1 Register (Offset = Bh) [reset = 0h]
pub mod ADCREF1;
/// Comparator
///
///
///
/// Control COMPA and COMPB comparators. Only to be used through TI provided API.
///
/// [TI-TRM-I] 17.7.1.6 COMP Register (Offset = 5h) [reset = 0h]
pub mod COMP;
/// Current Source
///
///
///
/// Strength and trim control for current source. Only to be used through TI provided API.
///
/// [TI-TRM-I] 17.7.1.5 ISRC Register (Offset = 4h) [reset = 0h]
pub mod ISRC;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 17.7.1.1 MUX0 Register (Offset = 0h) [reset = 0h]
pub mod MUX0;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 17.7.1.2 MUX1 Register (Offset = 1h) [reset = 0h]
pub mod MUX1;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 17.7.1.3 MUX2 Register (Offset = 2h) [reset = 0h]
pub mod MUX2;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 17.7.1.4 MUX3 Register (Offset = 3h) [reset = 0h]
pub mod MUX3;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 17.7.1.7 MUX4 Register (Offset = 7h) [reset = 0h]
pub mod MUX4;
