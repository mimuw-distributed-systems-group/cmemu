use cmemu_common::Address;
use core::ops::Range;

pub const BASE_ADDR: Address = Address::from_const(0xe0001000);
pub const ADDR: Address = BASE_ADDR.offset(0x0);
pub const SIZE: u32 = 0x1000;
/// 0xe0001000..0xe0002000
pub const ADDR_SPACE: Range<Address> = ADDR..ADDR.offset(SIZE);

/// Comparator 0
///
/// This register is used to write the reference value for comparator 0.
///
/// [TI-TRM-I] 2.7.1.9 COMP0 Register (Offset = 20h) [reset = X]
pub mod COMP0;
/// Comparator 1
///
/// This register is used to write the reference value for comparator 1.
///
/// [TI-TRM-I] 2.7.1.12 COMP1 Register (Offset = 30h) [reset = X]
pub mod COMP1;
/// Comparator 2
///
/// This register is used to write the reference value for comparator 2.
///
/// [TI-TRM-I] 2.7.1.15 COMP2 Register (Offset = 40h) [reset = X]
pub mod COMP2;
/// Comparator 3
///
/// This register is used to write the reference value for comparator 3.
///
/// [TI-TRM-I] 2.7.1.18 COMP3 Register (Offset = 50h) [reset = X]
pub mod COMP3;
/// CPI Count
///
/// This register is used to count the total number of instruction cycles beyond the first cycle.
///
/// [TI-TRM-I] 2.7.1.3 CPICNT Register (Offset = 8h) [reset = X]
pub mod CPICNT;
/// Control
///
/// Use the DWT Control Register to enable the DWT unit.
///
/// [TI-TRM-I] 2.7.1.1 CTRL Register (Offset = 0h) [reset = 40000000h]
pub mod CTRL;
/// Current PC Sampler Cycle Count
///
/// This register is used to count the number of core cycles. This counter can measure elapsed execution time. This is a free-running counter (this counter will not advance in power modes where free-running clock to CPU stops). The counter has three functions:
///
///
///
/// 1: When CTRL.PCSAMPLEENA = 1, the PC is sampled and emitted when the selected tapped bit changes value (0 to 1 or 1 to 0) and any post-scalar value counts to 0.
///
/// 2: When CTRL.CYCEVTENA = 1 , (and CTRL.PCSAMPLEENA = 0), an event is emitted when the selected tapped bit changes value (0 to 1 or 1 to 0) and any post-scalar value counts to 0.
///
/// 3: Applications and debuggers can use the counter to measure elapsed execution time. By subtracting a start and an end time, an application can measure time between in-core clocks (other than when Halted in debug). This is valid to 2^32 core clock cycles (for example, almost 89.5 seconds at 48MHz).
///
/// [TI-TRM-I] 2.7.1.2 CYCCNT Register (Offset = 4h) [reset = 0h]
pub mod CYCCNT;
/// Exception Overhead Count
///
/// This register is used to count the total cycles spent in interrupt processing.
///
/// [TI-TRM-I] 2.7.1.4 EXCCNT Register (Offset = Ch) [reset = X]
pub mod EXCCNT;
/// Fold Count
///
/// This register is used to count the total number of folded instructions. The counter increments on each instruction which takes 0 cycles.
///
/// [TI-TRM-I] 2.7.1.7 FOLDCNT Register (Offset = 18h) [reset = X]
pub mod FOLDCNT;
/// Function 0
///
/// Use the DWT Function Registers 0 to control the operation of the comparator 0. This comparator can:
///
/// 1. Match against either the PC or the data address. This is controlled by CYCMATCH. This function is only available for comparator 0 (COMP0).
///
/// 2. Emit data or PC couples, trigger the ETM, or generate a watchpoint depending on the operation defined by FUNCTION.
///
/// [TI-TRM-I] 2.7.1.11 FUNCTION0 Register (Offset = 28h) [reset = 0h]
pub mod FUNCTION0;
/// Function 1
///
/// Use the DWT Function Registers 1 to control the operation of the comparator 1. This comparator can:
///
/// 1. Perform data value comparisons if associated address comparators have performed an address match. This function is only available for comparator 1 (COMP1).
///
/// 2. Emit data or PC couples, trigger the ETM, or generate a watchpoint depending on the operation defined by FUNCTION.
///
/// [TI-TRM-I] 2.7.1.14 FUNCTION1 Register (Offset = 38h) [reset = 200h]
pub mod FUNCTION1;
/// Function 2
///
/// Use the DWT Function Registers 2 to control the operation of the comparator 2. This comparator can emit data or PC couples, trigger the ETM, or generate a watchpoint depending on the operation defined by FUNCTION.
///
/// [TI-TRM-I] 2.7.1.17 FUNCTION2 Register (Offset = 48h) [reset = 0h]
pub mod FUNCTION2;
/// Function 3
///
/// Use the DWT Function Registers 3 to control the operation of the comparator 3. This comparator can emit data or PC couples, trigger the ETM, or generate a watchpoint depending on the operation defined by FUNCTION.
///
/// [TI-TRM-I] 2.7.1.20 FUNCTION3 Register (Offset = 58h) [reset = 0h]
pub mod FUNCTION3;
/// LSU Count
///
/// This register is used to count the total number of cycles during which the processor is processing an LSU operation beyond the first cycle.
///
/// [TI-TRM-I] 2.7.1.6 LSUCNT Register (Offset = 14h) [reset = X]
pub mod LSUCNT;
/// Mask 0
///
/// Use the DWT Mask Registers 0 to apply a mask to data addresses when matching against COMP0.
///
/// [TI-TRM-I] 2.7.1.10 MASK0 Register (Offset = 24h) [reset = X]
pub mod MASK0;
/// Mask 1
///
/// Use the DWT Mask Registers 1 to apply a mask to data addresses when matching against COMP1.
///
/// [TI-TRM-I] 2.7.1.13 MASK1 Register (Offset = 34h) [reset = X]
pub mod MASK1;
/// Mask 2
///
/// Use the DWT Mask Registers 2 to apply a mask to data addresses when matching against COMP2.
///
/// [TI-TRM-I] 2.7.1.16 MASK2 Register (Offset = 44h) [reset = X]
pub mod MASK2;
/// Mask 3
///
/// Use the DWT Mask Registers 3 to apply a mask to data addresses when matching against COMP3.
///
/// [TI-TRM-I] 2.7.1.19 MASK3 Register (Offset = 54h) [reset = X]
pub mod MASK3;
/// Program Counter Sample
///
/// This register is used to enable coarse-grained software profiling using a debug agent, without changing the currently executing code. If the core is not in debug state, the value returned is the instruction address of a recently executed instruction. If the core is in debug state, the value returned is 0xFFFFFFFF.
///
/// [TI-TRM-I] 2.7.1.8 PCSR Register (Offset = 1Ch) [reset = X]
pub mod PCSR;
/// Sleep Count
///
/// This register is used to count the total number of cycles during which the processor is sleeping.
///
/// [TI-TRM-I] 2.7.1.5 SLEEPCNT Register (Offset = 10h) [reset = X]
pub mod SLEEPCNT;
