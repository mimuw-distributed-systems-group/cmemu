use cmemu_common::Address;
use core::ops::Range;

pub const BASE_ADDR: Address = Address::from_const(0x400c1000);
pub const ADDR: Address = BASE_ADDR.offset(0x0);
pub const SIZE: u32 = 0x1000;
/// 0x400c1000..0x400c2000
pub const ADDR_SPACE: Range<Address> = ADDR..ADDR.offset(SIZE);

/// General Purpose Input Output Digital Input Enable
///
///
///
/// This register controls input buffers for AUXIO that are controlled by instance i of AUX_AIODIO.  Hence, in formulas below i = 0 for AUX_AIODIO0 and I = 1 for AUX_AIODIO1.
///
/// [TI-TRM-I] 17.7.2.7 GPIODIE Register (Offset = 18h) [reset = 0h]
pub mod GPIODIE;
/// General Purpose Input Output Data In
///
///
///
/// This register provides synchronized input data for AUXIO  that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and I = 1 for AUX_AIODIO1.
///
/// [TI-TRM-I] 17.7.2.3 GPIODIN Register (Offset = 8h) [reset = 0h]
pub mod GPIODIN;
/// General Purpose Input Output Data Out
///
///
///
/// The output data register is used to set data on AUXIO that are controlled by instance i of AUX_AIODIO.  Hence, in formulas below i = 0 for AUX_AIODIO0 and  i = 1 for AUX_AIODIO1
///
/// [TI-TRM-I] 17.7.2.1 GPIODOUT Register (Offset = 0h) [reset = 0h]
pub mod GPIODOUT;
/// General Purpose Input Output Data Out Clear
///
///
///
/// Clear bits in GPIODOUT instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and i = 1 for AUX_AIODIO1.
///
/// [TI-TRM-I] 17.7.2.5 GPIODOUTCLR Register (Offset = 10h) [reset = 0h]
pub mod GPIODOUTCLR;
/// General Purpose Input Output Data Out Set
///
///
///
/// Set bits in GPIODOUT in instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and  i = 1 for AUX_AIODIO1.
///
/// [TI-TRM-I] 17.7.2.4 GPIODOUTSET Register (Offset = Ch) [reset = 0h]
pub mod GPIODOUTSET;
/// General Purpose Input Output Data Out Toggle
///
///
///
/// Toggle bits in GPIODOUT in instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and i = 1 for AUX_AIODIO1.
///
/// [TI-TRM-I] 17.7.2.6 GPIODOUTTGL Register (Offset = 14h) [reset = 0h]
pub mod GPIODOUTTGL;
/// Input Output Mode
///
///
///
/// This register controls pull-up, pull-down, and output mode for AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and i = 1 for AUX_AIODIO1
///
/// [TI-TRM-I] 17.7.2.2 IOMODE Register (Offset = 4h) [reset = 0h]
pub mod IOMODE;
