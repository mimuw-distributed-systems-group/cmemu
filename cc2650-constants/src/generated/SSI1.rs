use cmemu_common::Address;
use core::ops::Range;

pub const BASE_ADDR: Address = Address::from_const(0x40008000);
pub const ADDR: Address = BASE_ADDR.offset(0x0);
pub const SIZE: u32 = 0x1000;
/// 0x40008000..0x40009000
pub const ADDR_SPACE: Range<Address> = ADDR..ADDR.offset(SIZE);

/// Clock Prescale
///
/// [TI-TRM-I] 20.7.1.5 CPSR Register (Offset = 10h) [reset = 0h]
pub mod CPSR;
/// Control 0
///
/// [TI-TRM-I] 20.7.1.1 CR0 Register (Offset = 0h) [reset = 0h]
pub mod CR0;
/// Control 1
///
/// [TI-TRM-I] 20.7.1.2 CR1 Register (Offset = 4h) [reset = 0h]
pub mod CR1;
/// DMA Control
///
/// [TI-TRM-I] 20.7.1.10 DMACR Register (Offset = 24h) [reset = 0h]
pub mod DMACR;
/// Data
///
/// 16-bits wide data register:
///
/// When read, the entry in the receive FIFO, pointed to by the current FIFO read pointer, is accessed. As data values are removed by the  receive logic from the incoming data frame, they are placed into the entry in the receive FIFO, pointed to by the current FIFO write pointer.
///
/// When written, the entry in the transmit FIFO, pointed to by the write pointer, is written to. Data values are removed from the transmit FIFO one value at a time by the transmit logic. It is loaded into the transmit serial shifter, then serially shifted out onto the TXD output pin at the programmed bit rate.
///
/// When a data size of less than 16 bits is selected, the user must right-justify data written to the transmit FIFO. The transmit logic ignores the unused bits. Received data less than 16 bits is automatically right-justified in the receive buffer.
///
/// [TI-TRM-I] 20.7.1.3 DR Register (Offset = 8h) [reset = X]
pub mod DR;
/// Interrupt Clear
///
/// On a write of 1, the corresponding interrupt is cleared. A write of 0 has no effect.
///
/// [TI-TRM-I] 20.7.1.9 ICR Register (Offset = 20h) [reset = 0h]
pub mod ICR;
/// Interrupt Mask Set and Clear
///
/// [TI-TRM-I] 20.7.1.6 IMSC Register (Offset = 14h) [reset = 0h]
pub mod IMSC;
/// Masked Interrupt Status
///
/// [TI-TRM-I] 20.7.1.8 MIS Register (Offset = 1Ch) [reset = 0h]
pub mod MIS;
/// Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.
///
/// [TI-TRM-I] undocumented
pub mod RESERVED1;
/// Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.
///
/// [TI-TRM-I] undocumented
pub mod RESERVED2;
/// Raw Interrupt Status
///
/// [TI-TRM-I] 20.7.1.7 RIS Register (Offset = 18h) [reset = 8h]
pub mod RIS;
/// Status
///
/// [TI-TRM-I] 20.7.1.4 SR Register (Offset = Ch) [reset = 3h]
pub mod SR;
