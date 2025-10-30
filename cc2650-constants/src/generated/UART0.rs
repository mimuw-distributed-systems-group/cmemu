use cmemu_common::Address;
use core::ops::Range;

pub const BASE_ADDR: Address = Address::from_const(0x40001000);
pub const ADDR: Address = BASE_ADDR.offset(0x0);
pub const SIZE: u32 = 0x1000;
/// 0x40001000..0x40002000
pub const ADDR_SPACE: Range<Address> = ADDR..ADDR.offset(SIZE);

/// Control
///
/// [TI-TRM-I] 19.8.1.8 CTL Register (Offset = 30h) [reset = 300h]
pub mod CTL;
/// DMA Control
///
/// [TI-TRM-I] 19.8.1.14 DMACTL Register (Offset = 48h) [reset = 0h]
pub mod DMACTL;
/// Data
///
/// For words to be transmitted:
///
///   - if the FIFOs are enabled (LCRH.FEN = 1), data written to this location is pushed onto the transmit FIFO
///
///   - if the FIFOs are not enabled (LCRH.FEN = 0), data is stored in the transmitter holding register (the bottom word of the transmit FIFO).
///
/// The write operation initiates transmission from the UART. The data is prefixed with a start bit, appended with the appropriate parity bit (if parity is enabled), and a stop bit.
///
/// The resultant word is then transmitted.
///
/// For received words:
///
///   - if the FIFOs are enabled (LCRH.FEN = 1), the data byte and the 4-bit status (break, frame, parity, and overrun) is pushed onto the 12-bit wide receive FIFO
///
///   - if the FIFOs are not enabled (LCRH.FEN = 0), the data byte and status are stored in the receiving holding register (the bottom word of the receive FIFO).
///
/// The received data byte is read by performing reads from this register along with the corresponding status information. The status information can also be read by a read of the RSR register.
///
/// [TI-TRM-I] 19.8.1.1 DR Register (Offset = 0h) [reset = X]
pub mod DR;
/// Error Clear
///
/// This register is mapped to the same address as RSR register.  Reads from this address are associated with RSR register and return the receive status. Writes to this address are associated with ECR register and clear the receive status flags (framing, parity, break, and overrun errors).
///
/// [TI-TRM-I] 19.8.1.3 ECR Register (Offset = 4h) [reset = 0h]
pub mod ECR;
/// Fractional Baud-Rate Divisor
///
/// If this register is modified while trasmission or reception is on-going, the baudrate will not be updated until transmission or reception of the current character is complete.
///
/// [TI-TRM-I] 19.8.1.6 FBRD Register (Offset = 28h) [reset = 0h]
pub mod FBRD;
/// Flag
///
/// Reads from this register return the UART flags.
///
/// [TI-TRM-I] 19.8.1.4 FR Register (Offset = 18h) [reset = X]
pub mod FR;
/// Integer Baud-Rate Divisor
///
/// If this register is modified while trasmission or reception is on-going, the baudrate will not be updated until transmission or reception of the current character is complete.
///
/// [TI-TRM-I] 19.8.1.5 IBRD Register (Offset = 24h) [reset = 0h]
pub mod IBRD;
/// Interrupt Clear
///
/// On a write of 1, the corresponding interrupt is cleared. A write of 0 has no effect.
///
/// [TI-TRM-I] 19.8.1.13 ICR Register (Offset = 44h) [reset = X]
pub mod ICR;
/// Interrupt FIFO Level Select
///
/// [TI-TRM-I] 19.8.1.9 IFLS Register (Offset = 34h) [reset = 12h]
pub mod IFLS;
/// Interrupt Mask Set/Clear
///
/// [TI-TRM-I] 19.8.1.10 IMSC Register (Offset = 38h) [reset = 0h]
pub mod IMSC;
/// Line Control
///
/// [TI-TRM-I] 19.8.1.7 LCRH Register (Offset = 2Ch) [reset = 0h]
pub mod LCRH;
/// Masked Interrupt Status
///
/// [TI-TRM-I] 19.8.1.12 MIS Register (Offset = 40h) [reset = 0h]
pub mod MIS;
/// Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.
///
/// [TI-TRM-I] undocumented
pub mod RESERVED0;
/// Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.
///
/// [TI-TRM-I] undocumented
pub mod RESERVED1;
/// Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.
///
/// [TI-TRM-I] undocumented
pub mod RESERVED2;
/// Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.
///
/// [TI-TRM-I] undocumented
pub mod RESERVED3;
/// Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.
///
/// [TI-TRM-I] undocumented
pub mod RESERVED4;
/// Raw Interrupt Status
///
/// [TI-TRM-I] 19.8.1.11 RIS Register (Offset = 3Ch) [reset = X]
pub mod RIS;
/// Status
///
/// This register is mapped to the same address as ECR register.  Reads from this address are associated with RSR register and return the receive status. Writes to this address are associated with ECR register and clear the receive status flags (framing, parity, break, and overrun errors).
///
/// If the status is read from this register, then the status information for break, framing and parity corresponds to the data character read from the Data Register, DR prior to reading the RSR. The status information for overrun is set immediately when an overrun condition occurs.
///
/// [TI-TRM-I] 19.8.1.2 RSR Register (Offset = 4h) [reset = 0h]
pub mod RSR;
