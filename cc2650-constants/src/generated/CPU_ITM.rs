use cmemu_common::Address;
use core::ops::Range;

pub const BASE_ADDR: Address = Address::from_const(0xe0000000);
pub const ADDR: Address = BASE_ADDR.offset(0x0);
pub const SIZE: u32 = 0x1000;
/// 0xe0000000..0xe0001000
pub const ADDR_SPACE: Range<Address> = ADDR..ADDR.offset(SIZE);

/// Lock Access
///
/// This register is used to prevent write accesses to the Control Registers: TER, TPR and TCR.
///
/// [TI-TRM-I] 2.7.3.36 LAR Register (Offset = FB0h) [reset = 0h]
pub mod LAR;
/// Lock Status
///
/// Use this register to enable write accesses to the Control Register.
///
/// [TI-TRM-I] 2.7.3.37 LSR Register (Offset = FB4h) [reset = 3h]
pub mod LSR;
/// Stimulus Port 0
///
/// [TI-TRM-I] 2.7.3.1 STIM0 Register (Offset = 0h) [reset = X]
pub mod STIM0;
/// Stimulus Port 1
///
/// [TI-TRM-I] 2.7.3.2 STIM1 Register (Offset = 4h) [reset = X]
pub mod STIM1;
/// Stimulus Port 10
///
/// [TI-TRM-I] 2.7.3.11 STIM10 Register (Offset = 28h) [reset = X]
pub mod STIM10;
/// Stimulus Port 11
///
/// [TI-TRM-I] 2.7.3.12 STIM11 Register (Offset = 2Ch) [reset = X]
pub mod STIM11;
/// Stimulus Port 12
///
/// [TI-TRM-I] 2.7.3.13 STIM12 Register (Offset = 30h) [reset = X]
pub mod STIM12;
/// Stimulus Port 13
///
/// [TI-TRM-I] 2.7.3.14 STIM13 Register (Offset = 34h) [reset = X]
pub mod STIM13;
/// Stimulus Port 14
///
/// [TI-TRM-I] 2.7.3.15 STIM14 Register (Offset = 38h) [reset = X]
pub mod STIM14;
/// Stimulus Port 15
///
/// [TI-TRM-I] 2.7.3.16 STIM15 Register (Offset = 3Ch) [reset = X]
pub mod STIM15;
/// Stimulus Port 16
///
/// [TI-TRM-I] 2.7.3.17 STIM16 Register (Offset = 40h) [reset = X]
pub mod STIM16;
/// Stimulus Port 17
///
/// [TI-TRM-I] 2.7.3.18 STIM17 Register (Offset = 44h) [reset = X]
pub mod STIM17;
/// Stimulus Port 18
///
/// [TI-TRM-I] 2.7.3.19 STIM18 Register (Offset = 48h) [reset = X]
pub mod STIM18;
/// Stimulus Port 19
///
/// [TI-TRM-I] 2.7.3.20 STIM19 Register (Offset = 4Ch) [reset = X]
pub mod STIM19;
/// Stimulus Port 2
///
/// [TI-TRM-I] 2.7.3.3 STIM2 Register (Offset = 8h) [reset = X]
pub mod STIM2;
/// Stimulus Port 20
///
/// [TI-TRM-I] 2.7.3.21 STIM20 Register (Offset = 50h) [reset = X]
pub mod STIM20;
/// Stimulus Port 21
///
/// [TI-TRM-I] 2.7.3.22 STIM21 Register (Offset = 54h) [reset = X]
pub mod STIM21;
/// Stimulus Port 22
///
/// [TI-TRM-I] 2.7.3.23 STIM22 Register (Offset = 58h) [reset = X]
pub mod STIM22;
/// Stimulus Port 23
///
/// [TI-TRM-I] 2.7.3.24 STIM23 Register (Offset = 5Ch) [reset = X]
pub mod STIM23;
/// Stimulus Port 24
///
/// [TI-TRM-I] 2.7.3.25 STIM24 Register (Offset = 60h) [reset = X]
pub mod STIM24;
/// Stimulus Port 25
///
/// [TI-TRM-I] 2.7.3.26 STIM25 Register (Offset = 64h) [reset = X]
pub mod STIM25;
/// Stimulus Port 26
///
/// [TI-TRM-I] 2.7.3.27 STIM26 Register (Offset = 68h) [reset = X]
pub mod STIM26;
/// Stimulus Port 27
///
/// [TI-TRM-I] 2.7.3.28 STIM27 Register (Offset = 6Ch) [reset = X]
pub mod STIM27;
/// Stimulus Port 28
///
/// [TI-TRM-I] 2.7.3.29 STIM28 Register (Offset = 70h) [reset = X]
pub mod STIM28;
/// Stimulus Port 29
///
/// [TI-TRM-I] 2.7.3.30 STIM29 Register (Offset = 74h) [reset = X]
pub mod STIM29;
/// Stimulus Port 3
///
/// [TI-TRM-I] 2.7.3.4 STIM3 Register (Offset = Ch) [reset = X]
pub mod STIM3;
/// Stimulus Port 30
///
/// [TI-TRM-I] 2.7.3.31 STIM30 Register (Offset = 78h) [reset = X]
pub mod STIM30;
/// Stimulus Port 31
///
/// [TI-TRM-I] 2.7.3.32 STIM31 Register (Offset = 7Ch) [reset = X]
pub mod STIM31;
/// Stimulus Port 4
///
/// [TI-TRM-I] 2.7.3.5 STIM4 Register (Offset = 10h) [reset = X]
pub mod STIM4;
/// Stimulus Port 5
///
/// [TI-TRM-I] 2.7.3.6 STIM5 Register (Offset = 14h) [reset = X]
pub mod STIM5;
/// Stimulus Port 6
///
/// [TI-TRM-I] 2.7.3.7 STIM6 Register (Offset = 18h) [reset = X]
pub mod STIM6;
/// Stimulus Port 7
///
/// [TI-TRM-I] 2.7.3.8 STIM7 Register (Offset = 1Ch) [reset = X]
pub mod STIM7;
/// Stimulus Port 8
///
/// [TI-TRM-I] 2.7.3.9 STIM8 Register (Offset = 20h) [reset = X]
pub mod STIM8;
/// Stimulus Port 9
///
/// [TI-TRM-I] 2.7.3.10 STIM9 Register (Offset = 24h) [reset = X]
pub mod STIM9;
/// Trace Control
///
/// Use this register to configure and control ITM transfers. This register can only be written in privilege mode. DWT is not enabled in the ITM block. However, DWT stimulus entry into the FIFO is controlled by DWTENA. If DWT requires timestamping, the TSENA bit must be set.
///
/// [TI-TRM-I] 2.7.3.35 TCR Register (Offset = E80h) [reset = 0h]
pub mod TCR;
/// Trace Enable
///
/// Use the Trace Enable Register to generate trace data by writing to the corresponding stimulus port. Note: Privileged writes are accepted to this register if TCR.ITMENA is set. User writes are accepted to this register if TCR.ITMENA is set and the appropriate privilege mask is cleared. Privileged access to the stimulus ports enables an RTOS kernel to guarantee instrumentation slots or bandwidth as required.
///
/// [TI-TRM-I] 2.7.3.33 TER Register (Offset = E00h) [reset = 0h]
pub mod TER;
/// Trace Privilege
///
/// This register is used to enable an operating system to control which stimulus ports are accessible by user code. This register can only be used in privileged mode.
///
/// [TI-TRM-I] 2.7.3.34 TPR Register (Offset = E40h) [reset = 0h]
pub mod TPR;
