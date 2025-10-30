use cmemu_common::Address;
use core::ops::Range;

pub const BASE_ADDR: Address = Address::from_const(0xe0040000);
pub const ADDR: Address = BASE_ADDR.offset(0x0);
pub const SIZE: u32 = 0x1000;
/// 0xe0040000..0xe0041000
pub const ADDR_SPACE: Range<Address> = ADDR..ADDR.offset(SIZE);

/// Async Clock Prescaler
///
/// This register scales the baud rate of the asynchronous output.
///
/// [TI-TRM-I] 2.7.5.3 ACPR Register (Offset = 10h) [reset = 0h]
pub mod ACPR;
/// Claim Tag Clear
///
/// [TI-TRM-I] 2.7.5.11 CLAIMCLR Register (Offset = FA4h) [reset = 0h]
pub mod CLAIMCLR;
/// Claim Tag Mask
///
/// [TI-TRM-I] 2.7.5.8 CLAIMMASK Register (Offset = FA0h) [reset = Fh]
pub mod CLAIMMASK;
/// Claim Tag Set
///
/// [TI-TRM-I] 2.7.5.9 CLAIMSET Register (Offset = FA0h) [reset = Fh]
pub mod CLAIMSET;
/// Current Claim Tag
///
/// [TI-TRM-I] 2.7.5.10 CLAIMTAG Register (Offset = FA4h) [reset = 0h]
pub mod CLAIMTAG;
/// Current Sync Port Size
///
/// This register has the same format as SSPSR but only one bit can be set, and all others must be zero. Writing values with more than one bit set, or setting a bit that is not indicated as supported can cause Unpredictable behavior. On reset this defaults to the smallest possible port size, 1 bit.
///
/// [TI-TRM-I] 2.7.5.2 CSPSR Register (Offset = 4h) [reset = 1h]
pub mod CSPSR;
/// Device ID
///
/// [TI-TRM-I] 2.7.5.12 DEVID Register (Offset = FC8h) [reset = CA0h]
pub mod DEVID;
/// Formatter and Flush Control
///
/// When one of the two single wire output (SWO) modes is selected, ENFCONT enables the formatter to be bypassed. If the formatter is bypassed, only the ITM/DWT trace source (ATDATA2) passes through. The TPIU accepts and discards data that is presented on the ETM port (ATDATA1). This function is intended to be used when it is necessary to connect a device containing an ETM to a trace capture device that is only able to capture Serial Wire Output (SWO) data. Enabling or disabling the formatter causes momentary data corruption.
///
/// Note: If the selected pin protocol register (SPPR.PROTOCOL) is set to 0x00 (TracePort mode), this register always reads 0x102, because the formatter is automatically enabled. If one of the serial wire modes is then selected, the register reverts to its previously programmed value.
///
/// [TI-TRM-I] 2.7.5.6 FFCR Register (Offset = 304h) [reset = 102h]
pub mod FFCR;
/// Formatter and Flush Status
///
/// [TI-TRM-I] 2.7.5.5 FFSR Register (Offset = 300h) [reset = 8h]
pub mod FFSR;
/// Formatter Synchronization Counter
///
/// [TI-TRM-I] 2.7.5.7 FSCR Register (Offset = 308h) [reset = 0h]
pub mod FSCR;
/// Selected Pin Protocol
///
/// This register selects the protocol to be used for trace output.
///
/// Note: If this register is changed while trace data is being output, data corruption occurs.
///
/// [TI-TRM-I] 2.7.5.4 SPPR Register (Offset = F0h) [reset = 1h]
pub mod SPPR;
/// Supported Sync Port Sizes
///
/// This register represents a single port size that is supported on the device, that is, 4, 2 or 1. This is to ensure that tools do not attempt to select a port width that an attached TPA cannot capture.
///
/// [TI-TRM-I] 2.7.5.1 SSPSR Register (Offset = 0h) [reset = Bh]
pub mod SSPSR;
