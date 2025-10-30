use cmemu_common::Address;
use core::ops::Range;

pub const BASE_ADDR: Address = Address::from_const(0x40021000);
pub const ADDR: Address = BASE_ADDR.offset(0x0);
pub const SIZE: u32 = 0x1000;
/// 0x40021000..0x40022000
pub const ADDR_SPACE: Range<Address> = ADDR..ADDR.offset(SIZE);

/// Pin Direction
///
/// [TI-TRM-I] 22.10.1.3 AIFDIRCFG Register (Offset = 8h) [reset = 0h]
pub mod AIFDIRCFG;
/// DMA Buffer Size Configuration
///
/// [TI-TRM-I] 22.10.1.2 AIFDMACFG Register (Offset = 4h) [reset = 0h]
pub mod AIFDMACFG;
/// Serial Interface Format Configuration
///
/// [TI-TRM-I] 22.10.1.4 AIFFMTCFG Register (Offset = Ch) [reset = 170h]
pub mod AIFFMTCFG;
/// DMA Input Buffer Current Pointer
///
/// [TI-TRM-I] 22.10.1.9 AIFINPTR Register (Offset = 24h) [reset = 0h]
pub mod AIFINPTR;
/// DMA Input Buffer Next Pointer
///
/// [TI-TRM-I] 22.10.1.8 AIFINPTRNEXT Register (Offset = 20h) [reset = 0h]
pub mod AIFINPTRNEXT;
/// DMA Output Buffer Current Pointer
///
/// [TI-TRM-I] 22.10.1.11 AIFOUTPTR Register (Offset = 2Ch) [reset = 0h]
pub mod AIFOUTPTR;
/// DMA Output Buffer Next Pointer
///
/// [TI-TRM-I] 22.10.1.10 AIFOUTPTRNEXT Register (Offset = 28h) [reset = 0h]
pub mod AIFOUTPTRNEXT;
/// Audio Interface PWM Debug Value
///
/// [TI-TRM-I] 22.10.1.7 AIFPWMVALUE Register (Offset = 1Ch) [reset = 0h]
pub mod AIFPWMVALUE;
/// WCLK Source Selection
///
/// [TI-TRM-I] 22.10.1.1 AIFWCLKSRC Register (Offset = 0h) [reset = 0h]
pub mod AIFWCLKSRC;
/// Word Selection Bit Mask for Pin 0
///
/// [TI-TRM-I] 22.10.1.5 AIFWMASK0 Register (Offset = 10h) [reset = 3h]
pub mod AIFWMASK0;
/// Word Selection Bit Mask for Pin 1
///
/// [TI-TRM-I] 22.10.1.6 AIFWMASK1 Register (Offset = 14h) [reset = 3h]
pub mod AIFWMASK1;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod AIFWMASK2;
/// Interrupt Clear Register
///
/// [TI-TRM-I] 22.10.1.29 IRQCLR Register (Offset = 7Ch) [reset = 0h]
pub mod IRQCLR;
/// Raw Interrupt Status Register
///
/// [TI-TRM-I] 22.10.1.27 IRQFLAGS Register (Offset = 74h) [reset = 0h]
pub mod IRQFLAGS;
/// Interrupt Mask Register
///
///
///
/// Selects mask states of the flags in IRQFLAGS that contribute to the I2S_IRQ event.
///
/// [TI-TRM-I] 22.10.1.26 IRQMASK Register (Offset = 70h) [reset = 0h]
pub mod IRQMASK;
/// Interrupt Set Register
///
/// [TI-TRM-I] 22.10.1.28 IRQSET Register (Offset = 78h) [reset = 0h]
pub mod IRQSET;
/// Samplestamp Generator Control Register
///
/// [TI-TRM-I] 22.10.1.12 STMPCTL Register (Offset = 34h) [reset = 0h]
pub mod STMPCTL;
/// WCLK Counter Trigger Value for Input Pins
///
/// [TI-TRM-I] 22.10.1.17 STMPINTRIG Register (Offset = 48h) [reset = 0h]
pub mod STMPINTRIG;
/// WCLK Counter Trigger Value for Output Pins
///
/// [TI-TRM-I] 22.10.1.18 STMPOUTTRIG Register (Offset = 4Ch) [reset = 0h]
pub mod STMPOUTTRIG;
/// WCLK Counter Add Operation
///
/// [TI-TRM-I] 22.10.1.20 STMPWADD Register (Offset = 54h) [reset = 0h]
pub mod STMPWADD;
/// Current Value of WCNT
///
/// [TI-TRM-I] 22.10.1.22 STMPWCNT Register (Offset = 5Ch) [reset = 0h]
pub mod STMPWCNT;
/// Captured WCLK Counter Value, Capture Channel 0
///
/// [TI-TRM-I] 22.10.1.15 STMPWCNTCAPT0 Register (Offset = 40h) [reset = 0h]
pub mod STMPWCNTCAPT0;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 22.10.1.25 STMPWCNTCAPT1 Register (Offset = 68h) [reset = 0h]
pub mod STMPWCNTCAPT1;
/// WCLK Counter Period Value
///
/// [TI-TRM-I] 22.10.1.16 STMPWPER Register (Offset = 44h) [reset = 0h]
pub mod STMPWPER;
/// WCLK Counter Set Operation
///
/// [TI-TRM-I] 22.10.1.19 STMPWSET Register (Offset = 50h) [reset = 0h]
pub mod STMPWSET;
/// Current Value of XCNT
///
/// [TI-TRM-I] 22.10.1.23 STMPXCNT Register (Offset = 60h) [reset = 0h]
pub mod STMPXCNT;
/// Captured XOSC Counter Value, Capture Channel 0
///
/// [TI-TRM-I] 22.10.1.13 STMPXCNTCAPT0 Register (Offset = 38h) [reset = 0h]
pub mod STMPXCNTCAPT0;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 22.10.1.24 STMPXCNTCAPT1 Register (Offset = 64h) [reset = 0h]
pub mod STMPXCNTCAPT1;
/// XOSC Period Value
///
/// [TI-TRM-I] 22.10.1.14 STMPXPER Register (Offset = 3Ch) [reset = 0h]
pub mod STMPXPER;
/// XOSC Minimum Period Value
///
/// Minimum Value of STMPXPER
///
/// [TI-TRM-I] 22.10.1.21 STMPXPERMIN Register (Offset = 58h) [reset = FFFFh]
pub mod STMPXPERMIN;
