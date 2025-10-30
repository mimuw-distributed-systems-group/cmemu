use cmemu_common::Address;
use core::ops::Range;

pub const BASE_ADDR: Address = Address::from_const(0x40002000);
pub const ADDR: Address = BASE_ADDR.offset(0x0);
pub const SIZE: u32 = 0x1000;
/// 0x40002000..0x40003000
pub const ADDR_SPACE: Range<Address> = ADDR..ADDR.offset(SIZE);

/// Master Configuration
///
/// This register configures the mode (Master or Slave) and sets the interface for test mode loopback.
///
/// [TI-TRM-I] 21.5.1.18 MCR Register (Offset = 820h) [reset = 0h]
pub mod MCR;
/// Master Control
///
///
///
/// This register accesses status bits when read and control bits when written. When read, the status register indicates the state of the I2C bus controller as stated in MSTAT. When written, the control register configures the I2C controller operation.
///
///
///
/// To generate a single transmit cycle, the I2C Master Slave Address (MSA) register is written with the desired address, the MSA.RS bit is cleared, and this register is written with
///
/// * ACK=X (0 or 1),
///
/// * STOP=1,
///
/// * START=1,
///
/// * RUN=1
///
/// to perform the operation and stop.
///
/// When the operation is completed (or aborted due an error), an interrupt becomes active and the data may be read from the MDR register.
///
/// [TI-TRM-I] 21.5.1.11 MCTRL Register (Offset = 804h) [reset = 0h]
pub mod MCTRL;
/// Master Data
///
/// This register contains the data to be transmitted when in the Master Transmit state and the data received when in the Master Receive state.
///
/// [TI-TRM-I] 21.5.1.12 MDR Register (Offset = 808h) [reset = 0h]
pub mod MDR;
/// Master Interrupt Clear
///
/// This register clears the raw and masked interrupt.
///
/// [TI-TRM-I] 21.5.1.17 MICR Register (Offset = 81Ch) [reset = 0h]
pub mod MICR;
/// Master Interrupt Mask
///
/// This register controls whether a raw interrupt is promoted to a controller interrupt.
///
/// [TI-TRM-I] 21.5.1.14 MIMR Register (Offset = 810h) [reset = 0h]
pub mod MIMR;
/// Master Masked Interrupt Status
///
/// This register show which interrupt is active (based on result from MRIS and MIMR).
///
/// [TI-TRM-I] 21.5.1.16 MMIS Register (Offset = 818h) [reset = 0h]
pub mod MMIS;
/// Master Raw Interrupt Status
///
/// This register show the unmasked interrupt status.
///
/// [TI-TRM-I] 21.5.1.15 MRIS Register (Offset = 814h) [reset = 0h]
pub mod MRIS;
/// Master Salve Address
///
/// This register contains seven address bits of the slave to be accessed by the master (a6-a0), and an RS bit determining if the next operation is a receive or transmit.
///
/// [TI-TRM-I] 21.5.1.9 MSA Register (Offset = 800h) [reset = 0h]
pub mod MSA;
/// Master Status
///
/// [TI-TRM-I] 21.5.1.10 MSTAT Register (Offset = 804h) [reset = 20h]
pub mod MSTAT;
/// I2C Master Timer Period
///
/// This register specifies the period of the SCL clock.
///
/// [TI-TRM-I] 21.5.1.13 MTPR Register (Offset = 80Ch) [reset = 1h]
pub mod MTPR;
/// Slave Control
///
/// Note: This register shares address with SSTAT, meaning that this register functions as a control register when written, and a status register when read.
///
/// [TI-TRM-I] 21.5.1.3 SCTL Register (Offset = 4h) [reset = 0h]
pub mod SCTL;
/// Slave Data
///
/// This register contains the data to be transmitted when in the Slave Transmit state, and the data received when in the Slave Receive state.
///
/// [TI-TRM-I] 21.5.1.4 SDR Register (Offset = 8h) [reset = 0h]
pub mod SDR;
/// Slave Interrupt Clear
///
/// This register clears the raw interrupt SRIS.
///
/// [TI-TRM-I] 21.5.1.8 SICR Register (Offset = 18h) [reset = 0h]
pub mod SICR;
/// Slave Interrupt Mask
///
/// This register controls whether a raw interrupt is promoted to a controller interrupt.
///
/// [TI-TRM-I] 21.5.1.5 SIMR Register (Offset = Ch) [reset = 0h]
pub mod SIMR;
/// Slave Masked Interrupt Status
///
/// This register show which interrupt is active (based on result from SRIS and SIMR).
///
/// [TI-TRM-I] 21.5.1.7 SMIS Register (Offset = 14h) [reset = 0h]
pub mod SMIS;
/// Slave Own Address
///
/// This register consists of seven address bits that identify this I2C device on the I2C bus.
///
/// [TI-TRM-I] 21.5.1.1 SOAR Register (Offset = 0h) [reset = 0h]
pub mod SOAR;
/// Slave Raw Interrupt Status
///
/// This register shows the unmasked interrupt status.
///
/// [TI-TRM-I] 21.5.1.6 SRIS Register (Offset = 10h) [reset = 0h]
pub mod SRIS;
/// Slave Status
///
/// Note: This register shares address with SCTL, meaning that this register functions as a control register when written, and a status register when read.
///
/// [TI-TRM-I] 21.5.1.2 SSTAT Register (Offset = 4h) [reset = 0h]
pub mod SSTAT;
