use cmemu_common::Address;
use core::ops::Range;

pub const BASE_ADDR: Address = Address::from_const(0x40020000);
pub const ADDR: Address = BASE_ADDR.offset(0x0);
pub const SIZE: u32 = 0x1000;
/// 0x40020000..0x40021000
pub const ADDR_SPACE: Range<Address> = ADDR..ADDR.offset(SIZE);

/// Channel Alternate Control Data Base Pointer
///
/// [TI-TRM-I] 12.5.1.4 ALTCTRL Register (Offset = Ch) [reset = 200h]
pub mod ALTCTRL;
/// Configuration
///
/// [TI-TRM-I] 12.5.1.2 CFG Register (Offset = 4h) [reset = 0h]
pub mod CFG;
/// Channel Clear UseBurst
///
/// [TI-TRM-I] 12.5.1.8 CLEARBURST Register (Offset = 1Ch) [reset = 0h]
pub mod CLEARBURST;
/// Clear Channel Enable
///
/// [TI-TRM-I] 12.5.1.12 CLEARCHANNELEN Register (Offset = 2Ch) [reset = 0h]
pub mod CLEARCHANNELEN;
/// Channel Clear Primary-Alternate
///
/// [TI-TRM-I] 12.5.1.14 CLEARCHNLPRIALT Register (Offset = 34h) [reset = 0h]
pub mod CLEARCHNLPRIALT;
/// Clear Channel Priority
///
/// [TI-TRM-I] 12.5.1.16 CLEARCHNLPRIORITY Register (Offset = 3Ch) [reset = 0h]
pub mod CLEARCHNLPRIORITY;
/// Clear Channel Request Mask
///
/// [TI-TRM-I] 12.5.1.10 CLEARREQMASK Register (Offset = 24h) [reset = 0h]
pub mod CLEARREQMASK;
/// Channel Control Data Base Pointer
///
/// [TI-TRM-I] 12.5.1.3 CTRL Register (Offset = 8h) [reset = 0h]
pub mod CTRL;
/// Channel Request Done Mask
///
/// [TI-TRM-I] 12.5.1.19 DONEMASK Register (Offset = 520h) [reset = 0h]
pub mod DONEMASK;
/// Error Status and Clear
///
/// [TI-TRM-I] 12.5.1.17 ERROR Register (Offset = 4Ch) [reset = 0h]
pub mod ERROR;
/// Channel Request Done
///
/// [TI-TRM-I] 12.5.1.18 REQDONE Register (Offset = 504h) [reset = 0h]
pub mod REQDONE;
/// Channel Set UseBurst
///
/// [TI-TRM-I] 12.5.1.7 SETBURST Register (Offset = 18h) [reset = 0h]
pub mod SETBURST;
/// Set Channel Enable
///
/// [TI-TRM-I] 12.5.1.11 SETCHANNELEN Register (Offset = 28h) [reset = 0h]
pub mod SETCHANNELEN;
/// Channel Set Primary-Alternate
///
/// [TI-TRM-I] 12.5.1.13 SETCHNLPRIALT Register (Offset = 30h) [reset = 0h]
pub mod SETCHNLPRIALT;
/// Set Channel Priority
///
/// [TI-TRM-I] 12.5.1.15 SETCHNLPRIORITY Register (Offset = 38h) [reset = 0h]
pub mod SETCHNLPRIORITY;
/// Channel Set Request Mask
///
/// [TI-TRM-I] 12.5.1.9 SETREQMASK Register (Offset = 20h) [reset = 0h]
pub mod SETREQMASK;
/// Channel Software Request
///
/// [TI-TRM-I] 12.5.1.6 SOFTREQ Register (Offset = 14h) [reset = 0h]
pub mod SOFTREQ;
/// Status
///
/// [TI-TRM-I] 12.5.1.1 STATUS Register (Offset = 0h) [reset = 001F0000h]
pub mod STATUS;
/// Channel Wait On Request Status
///
/// [TI-TRM-I] 12.5.1.5 WAITONREQ Register (Offset = 10h) [reset = FFFF1EFFh]
pub mod WAITONREQ;
