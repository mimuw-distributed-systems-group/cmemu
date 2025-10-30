use cmemu_common::Address;
use core::ops::Range;

pub const BASE_ADDR: Address = Address::from_const(0xe000e000);
pub const ADDR: Address = BASE_ADDR.offset(0x0);
pub const SIZE: u32 = 0x1000;
/// 0xe000e000..0xe000f000
pub const ADDR_SPACE: Range<Address> = ADDR..ADDR.offset(SIZE);

/// Auxiliary Control
///
/// This register is used to disable certain aspects of functionality within the processor
///
/// [TI-TRM-I] 2.7.4.2 ACTLR Register (Offset = 8h) [reset = 0h]
pub mod ACTLR;
/// Auxiliary Fault Status
///
/// This register is used to determine additional system fault information to software. Single-cycle high level on an auxiliary faults is latched as one. The bit can only be cleared by writing a one to the corresponding bit. Auxiliary fault inputs to the CPU are tied to 0.
///
/// [TI-TRM-I] 2.7.4.41 AFSR Register (Offset = D3Ch) [reset = 0h]
pub mod AFSR;
/// Application Interrupt/Reset Control
///
/// This register is used to determine data endianness, clear all active state information for debug or to recover from a hard failure, execute a system reset, alter the priority grouping position (binary point).
///
/// [TI-TRM-I] 2.7.4.29 AIRCR Register (Offset = D0Ch) [reset = FA050000h]
pub mod AIRCR;
/// Bus Fault Address
///
/// This register is used to read the address of the location that generated a Bus Fault.
///
/// [TI-TRM-I] 2.7.4.40 BFAR Register (Offset = D38h) [reset = X]
pub mod BFAR;
/// Configuration Control
///
/// This register is used to enable NMI, HardFault and FAULTMASK to ignore bus fault, trap divide by zero and unaligned accesses, enable user access to the Software Trigger Interrupt Register (STIR), control entry to Thread Mode.
///
/// [TI-TRM-I] 2.7.4.31 CCR Register (Offset = D14h) [reset = 200h]
pub mod CCR;
/// Configurable Fault Status
///
/// This register is used to obtain information about local faults. These registers include three subsections: The first byte is Memory Manage Fault Status Register (MMFSR). The second byte is Bus Fault Status Register (BFSR). The higher half-word is Usage Fault Status Register (UFSR). The flags in these registers indicate the causes of local faults. Multiple flags can be set if more than one fault occurs. These register are read/write-clear. This means that they can be read normally, but writing a 1 to any bit clears that bit.
///
/// The CFSR is byte accessible. CFSR or its subregisters can be accessed as follows:
///
/// The following accesses are possible to the CFSR register:
///
/// - access the complete register with a word access to 0xE000ED28.
///
/// - access the MMFSR with a byte access to 0xE000ED28
///
/// - access the MMFSR and BFSR with a halfword access to 0xE000ED28
///
/// - access the BFSR with a byte access to 0xE000ED29
///
/// - access the UFSR with a halfword access to 0xE000ED2A.
///
/// [TI-TRM-I] 2.7.4.36 CFSR Register (Offset = D28h) [reset = 0h]
pub mod CFSR;
/// Coprocessor Access Control
///
/// This register specifies the access privileges for coprocessors.
///
/// [TI-TRM-I] 2.7.4.55 CPACR Register (Offset = D88h) [reset = 0h]
pub mod CPACR;
/// CPUID Base
///
/// This register determines the ID number of the processor core, the version number of the processor core and the implementation details of the processor core.
///
/// [TI-TRM-I] 2.7.4.26 CPUID Register (Offset = D00h) [reset = 412FC231h]
pub mod CPUID;
/// Debug Core Register Data
///
/// [TI-TRM-I] 2.7.4.58 DCRDR Register (Offset = DF8h) [reset = X]
pub mod DCRDR;
/// Deubg Core Register Selector
///
/// The purpose of this register is to select the processor register to transfer data to or from. This write-only register generates a handshake to the core to transfer data to or from Debug Core Register Data Register and the selected register. Until this core transaction is complete, DHCSR.S_REGRDY is 0. Note that writes to this register in any size but word are Unpredictable.
///
/// Note that PSR registers are fully accessible this way, whereas some read as 0 when using MRS instructions. Note that all bits can be written, but some combinations cause a fault when execution is resumed.
///
/// [TI-TRM-I] 2.7.4.57 DCRSR Register (Offset = DF4h) [reset = X]
pub mod DCRSR;
/// Debug Exception and Monitor Control
///
/// The purpose of this register is vector catching and debug monitor control.  This register manages exception behavior under debug. Vector catching is only available to halting debug. The upper halfword is for monitor controls and the lower halfword is for halting exception support. This register is not reset on a system reset. This register is reset by a power-on reset. The fields MON_EN, MON_PEND, MON_STEP and MON_REQ are always cleared on a core reset. The debug monitor is enabled by software in the reset handler or later, or by the **AHB-AP** port. Vector catching is semi-synchronous. When a matching event is seen, a Halt is requested. Because the processor can only halt on an instruction boundary, it must wait until the next instruction boundary. As a result, it stops on the first instruction of the exception handler. However, two special cases exist when a vector catch has triggered: 1. If a fault is taken during a vector read or stack push error the halt occurs on the corresponding fault handler for the vector error or stack push. 2. If a late arriving interrupt detected during a vector read or stack push error it is not taken. That is, an implementation that supports the late arrival optimization must suppress it in this case.
///
/// [TI-TRM-I] 2.7.4.59 DEMCR Register (Offset = DFCh) [reset = 0h]
pub mod DEMCR;
/// Debug Fault Status
///
/// This register is used to monitor external debug requests, vector catches, data watchpoint match, BKPT instruction execution, halt requests. Multiple flags in the Debug Fault Status Register can be set when multiple fault conditions occur. The register is read/write clear. This means that it can be read normally. Writing a 1 to a bit clears that bit. Note that these bits are not set unless the event is caught. This means that it causes a stop of some sort. If halting debug is enabled, these events stop the processor into debug. If debug is disabled and the debug monitor is enabled, then this becomes a debug monitor handler call, if priority permits. If debug and the monitor are both disabled, some of these events are Hard Faults, and some are ignored.
///
/// [TI-TRM-I] 2.7.4.38 DFSR Register (Offset = D30h) [reset = 0h]
pub mod DFSR;
/// Debug Halting Control and Status
///
/// The purpose of this register is to provide status information about the state of the processor, enable core debug, halt and step the processor. For writes, 0xA05F must be written to higher half-word of this register, otherwise the write operation is ignored and no bits are written into the register. If not enabled for Halting mode, C_DEBUGEN = 1, all other fields are disabled. This register is not reset on a core reset. It is reset by a power-on reset. However, C_HALT always clears on a core reset. To halt on a reset, the following bits must be enabled: DEMCR.VC_CORERESET and C_DEBUGEN. Note that writes to this register in any size other than word are unpredictable. It is acceptable to read in any size, and it can be used to avoid or intentionally change a sticky bit.
///
///
///
/// Behavior of the system when writing to this register while CPU is halted (i.e. C_DEBUGEN = 1 and S_HALT= 1):
///
/// C_HALT=0, C_STEP=0, C_MASKINTS=0               Exit Debug state and start instruction execution. Exceptions activate according to the exception configuration rules.
///
/// C_HALT=0, C_STEP=0, C_MASKINTS=1               Exit Debug state and start instruction execution. PendSV, SysTick and external configurable interrupts are disabled, otherwise exceptions activate according to standard configuration rules.
///
/// C_HALT=0, C_STEP=1, C_MASKINTS=0               Exit Debug state, step an instruction and halt. Exceptions activate according to the exception configuration rules.
///
/// C_HALT=0, C_STEP=1, C_MASKINTS=1               Exit Debug state, step an instruction and halt. PendSV, SysTick and external configurable interrupts are disabled, otherwise exceptions activate according to standard configuration rules.
///
/// C_HALT=1, C_STEP=x, C_MASKINTS=x               Remain in Debug state
///
/// [TI-TRM-I] 2.7.4.56 DHCSR Register (Offset = DF0h) [reset = X]
pub mod DHCSR;
/// Hard Fault Status
///
/// This register is used to obtain information about events that activate the Hard Fault handler. This register is a write-clear register. This means that writing a 1 to a bit clears that bit.
///
/// [TI-TRM-I] 2.7.4.37 HFSR Register (Offset = D2Ch) [reset = 0h]
pub mod HFSR;
/// Interrupt Control State
///
/// This register is used to set a pending Non-Maskable Interrupt (NMI), set or clear a pending SVC, set or clear a pending SysTick, check for pending exceptions, check the vector number of the highest priority pended exception, and check the vector number of the active exception.
///
/// [TI-TRM-I] 2.7.4.27 ICSR Register (Offset = D04h) [reset = X]
pub mod ICSR;
/// Interrupt Control Type
///
/// Read this register to see the number of interrupt lines that the NVIC supports.
///
/// [TI-TRM-I] 2.7.4.1 ICTR Register (Offset = 4h) [reset = 1h]
pub mod ICTR;
/// Auxiliary Feature 0
///
/// This register provides some freedom for implementation defined features to be registered. Not used in Cortex-M.
///
/// [TI-TRM-I] 2.7.4.45 ID_AFR0 Register (Offset = D4Ch) [reset = 0h]
pub mod ID_AFR0;
/// Debug Feature 0
///
/// This register provides a high level view of the debug system. Further details are provided in the debug infrastructure itself.
///
/// [TI-TRM-I] 2.7.4.44 ID_DFR0 Register (Offset = D48h) [reset = 00100000h]
pub mod ID_DFR0;
/// ISA Feature 0
///
/// Information on the instruction set attributes register
///
/// [TI-TRM-I] 2.7.4.50 ID_ISAR0 Register (Offset = D60h) [reset = 01101110h]
pub mod ID_ISAR0;
/// ISA Feature 1
///
/// Information on the instruction set attributes register
///
/// [TI-TRM-I] 2.7.4.51 ID_ISAR1 Register (Offset = D64h) [reset = 02111000h]
pub mod ID_ISAR1;
/// ISA Feature 2
///
/// Information on the instruction set attributes register
///
/// [TI-TRM-I] 2.7.4.52 ID_ISAR2 Register (Offset = D68h) [reset = 21112231h]
pub mod ID_ISAR2;
/// ISA Feature 3
///
/// Information on the instruction set attributes register
///
/// [TI-TRM-I] 2.7.4.53 ID_ISAR3 Register (Offset = D6Ch) [reset = 01111110h]
pub mod ID_ISAR3;
/// ISA Feature 4
///
/// Information on the instruction set attributes register
///
/// [TI-TRM-I] 2.7.4.54 ID_ISAR4 Register (Offset = D70h) [reset = 01310132h]
pub mod ID_ISAR4;
/// Memory Model Feature 0
///
/// General information on the memory model and memory management support.
///
/// [TI-TRM-I] 2.7.4.46 ID_MMFR0 Register (Offset = D50h) [reset = 00100030h]
pub mod ID_MMFR0;
/// Memory Model Feature 1
///
/// General information on the memory model and memory management support.
///
/// [TI-TRM-I] 2.7.4.47 ID_MMFR1 Register (Offset = D54h) [reset = 0h]
pub mod ID_MMFR1;
/// Memory Model Feature 2
///
/// General information on the memory model and memory management support.
///
/// [TI-TRM-I] 2.7.4.48 ID_MMFR2 Register (Offset = D58h) [reset = 01000000h]
pub mod ID_MMFR2;
/// Memory Model Feature 3
///
/// General information on the memory model and memory management support.
///
/// [TI-TRM-I] 2.7.4.49 ID_MMFR3 Register (Offset = D5Ch) [reset = 0h]
pub mod ID_MMFR3;
/// Processor Feature 0
///
/// [TI-TRM-I] 2.7.4.42 ID_PFR0 Register (Offset = D40h) [reset = 30h]
pub mod ID_PFR0;
/// Processor Feature 1
///
/// [TI-TRM-I] 2.7.4.43 ID_PFR1 Register (Offset = D44h) [reset = 200h]
pub mod ID_PFR1;
/// Mem Manage Fault Address
///
/// This register is used to read the address of the location that caused a Memory Manage Fault.
///
/// [TI-TRM-I] 2.7.4.39 MMFAR Register (Offset = D34h) [reset = X]
pub mod MMFAR;
/// Irq 0 to 31 Active Bit
///
/// This register is used to determine which interrupts are active. Each flag in the register corresponds to one interrupt.
///
/// [TI-TRM-I] 2.7.4.15 NVIC_IABR0 Register (Offset = 300h) [reset = 0h]
pub mod NVIC_IABR0;
/// Irq 32 to 63 Active Bit
///
/// This register is used to determine which interrupts are active. Each flag in the register corresponds to one interrupt.
///
/// [TI-TRM-I] 2.7.4.16 NVIC_IABR1 Register (Offset = 304h) [reset = 0h]
pub mod NVIC_IABR1;
/// Irq 0 to 31 Clear Enable
///
/// This register is used to disable interrupts and determine which interrupts are currently enabled.
///
/// [TI-TRM-I] 2.7.4.9 NVIC_ICER0 Register (Offset = 180h) [reset = 0h]
pub mod NVIC_ICER0;
/// Irq 32 to 63 Clear Enable
///
/// This register is used to disable interrupts and determine which interrupts are currently enabled.
///
/// [TI-TRM-I] 2.7.4.10 NVIC_ICER1 Register (Offset = 184h) [reset = 0h]
pub mod NVIC_ICER1;
/// Irq 0 to 31 Clear Pending
///
/// This register is used to clear pending interrupts and determine which interrupts are currently pending.
///
/// [TI-TRM-I] 2.7.4.13 NVIC_ICPR0 Register (Offset = 280h) [reset = 0h]
pub mod NVIC_ICPR0;
/// Irq 32 to 63 Clear Pending
///
/// This register is used to clear pending interrupts and determine which interrupts are currently pending.
///
/// [TI-TRM-I] 2.7.4.14 NVIC_ICPR1 Register (Offset = 284h) [reset = 0h]
pub mod NVIC_ICPR1;
/// Irq 0 to 3 Priority
///
/// This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP.
///
/// [TI-TRM-I] 2.7.4.17 NVIC_IPR0 Register (Offset = 400h) [reset = 0h]
pub mod NVIC_IPR0;
/// Irq 4 to 7 Priority
///
/// This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP.
///
/// [TI-TRM-I] 2.7.4.18 NVIC_IPR1 Register (Offset = 404h) [reset = 0h]
pub mod NVIC_IPR1;
/// Irq 8 to 11 Priority
///
/// This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP.
///
/// [TI-TRM-I] 2.7.4.19 NVIC_IPR2 Register (Offset = 408h) [reset = 0h]
pub mod NVIC_IPR2;
/// Irq 12 to 15 Priority
///
/// This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP.
///
/// [TI-TRM-I] 2.7.4.20 NVIC_IPR3 Register (Offset = 40Ch) [reset = 0h]
pub mod NVIC_IPR3;
/// Irq 16 to 19 Priority
///
/// This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP.
///
/// [TI-TRM-I] 2.7.4.21 NVIC_IPR4 Register (Offset = 410h) [reset = 0h]
pub mod NVIC_IPR4;
/// Irq 20 to 23 Priority
///
/// This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP.
///
/// [TI-TRM-I] 2.7.4.22 NVIC_IPR5 Register (Offset = 414h) [reset = 0h]
pub mod NVIC_IPR5;
/// Irq 24 to 27 Priority
///
/// This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP.
///
/// [TI-TRM-I] 2.7.4.23 NVIC_IPR6 Register (Offset = 418h) [reset = 0h]
pub mod NVIC_IPR6;
/// Irq 28 to 31 Priority
///
/// This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP.
///
/// [TI-TRM-I] 2.7.4.24 NVIC_IPR7 Register (Offset = 41Ch) [reset = 0h]
pub mod NVIC_IPR7;
/// Irq 32 to 35 Priority
///
/// This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP.
///
/// [TI-TRM-I] 2.7.4.25 NVIC_IPR8 Register (Offset = 420h) [reset = 0h]
pub mod NVIC_IPR8;
/// Irq 0 to 31 Set Enable
///
/// This register is used to enable interrupts and determine which interrupts are currently enabled.
///
/// [TI-TRM-I] 2.7.4.7 NVIC_ISER0 Register (Offset = 100h) [reset = 0h]
pub mod NVIC_ISER0;
/// Irq 32 to 63 Set Enable
///
/// This register is used to enable interrupts and determine which interrupts are currently enabled.
///
/// [TI-TRM-I] 2.7.4.8 NVIC_ISER1 Register (Offset = 104h) [reset = 0h]
pub mod NVIC_ISER1;
/// Irq 0 to 31 Set Pending
///
/// This register is used to force interrupts into the pending state and determine which interrupts are currently pending.
///
/// [TI-TRM-I] 2.7.4.11 NVIC_ISPR0 Register (Offset = 200h) [reset = 0h]
pub mod NVIC_ISPR0;
/// Irq 32 to 63 Set Pending
///
/// This register is used to force interrupts into the pending state and determine which interrupts are currently pending.
///
/// [TI-TRM-I] 2.7.4.12 NVIC_ISPR1 Register (Offset = 204h) [reset = 0h]
pub mod NVIC_ISPR1;
/// Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.
///
/// [TI-TRM-I] undocumented
pub mod RESERVED0;
/// Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.
///
/// [TI-TRM-I] undocumented
pub mod RESERVED000;
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
/// Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.
///
/// [TI-TRM-I] undocumented
pub mod RESERVED5;
/// Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.
///
/// [TI-TRM-I] undocumented
pub mod RESERVED6;
/// System Control
///
/// This register is used for power-management functions, i.e., signaling to the system when the processor can enter a low power state, controlling how the processor enters and exits low power states.
///
/// [TI-TRM-I] 2.7.4.30 SCR Register (Offset = D10h) [reset = 0h]
pub mod SCR;
/// System Handler Control and State
///
/// This register is used to enable or disable the system handlers, determine the pending status of bus fault, mem manage fault, and SVC, determine the active status of the system handlers. If a fault condition occurs while its fault handler is disabled, the fault escalates to a Hard Fault.
///
/// [TI-TRM-I] 2.7.4.35 SHCSR Register (Offset = D24h) [reset = 0h]
pub mod SHCSR;
/// System Handlers 4-7 Priority
///
/// This register is used to prioritize the following system handlers: Memory manage, Bus fault, and Usage fault. System Handlers are a special class of exception handler that can have their priority set to any of the priority levels. Most can be masked on (enabled) or off (disabled). When disabled, the fault is always treated as a Hard Fault.
///
/// [TI-TRM-I] 2.7.4.32 SHPR1 Register (Offset = D18h) [reset = 0h]
pub mod SHPR1;
/// System Handlers 8-11 Priority
///
/// This register is used to prioritize the SVC handler. System Handlers are a special class of exception handler that can have their priority set to any of the priority levels. Most can be masked on (enabled) or off (disabled). When disabled, the fault is always treated as a Hard Fault.
///
/// [TI-TRM-I] 2.7.4.33 SHPR2 Register (Offset = D1Ch) [reset = 0h]
pub mod SHPR2;
/// System Handlers 12-15 Priority
///
/// This register is used to prioritize the following system handlers: SysTick, PendSV and Debug Monitor. System Handlers are a special class of exception handler that can have their priority set to any of the priority levels. Most can be masked on (enabled) or off (disabled). When disabled, the fault is always treated as a Hard Fault.
///
/// [TI-TRM-I] 2.7.4.34 SHPR3 Register (Offset = D20h) [reset = 0h]
pub mod SHPR3;
/// SysTick Calibration Value
///
/// Used to enable software to scale to any required speed using divide and multiply.
///
/// [TI-TRM-I] 2.7.4.6 STCR Register (Offset = 1Ch) [reset = C0075300h]
pub mod STCR;
/// SysTick Control and Status
///
/// This register enables the SysTick features and returns status flags related to SysTick.
///
/// [TI-TRM-I] 2.7.4.3 STCSR Register (Offset = 10h) [reset = 4h]
pub mod STCSR;
/// SysTick Current Value
///
/// Read from this register returns the current value of SysTick counter. Writing to this register resets the SysTick counter (as well as  STCSR.COUNTFLAG).
///
/// [TI-TRM-I] 2.7.4.5 STCVR Register (Offset = 18h) [reset = X]
pub mod STCVR;
/// Software Trigger Interrupt
///
/// [TI-TRM-I] 2.7.4.60 STIR Register (Offset = F00h) [reset = X]
pub mod STIR;
/// SysTick Reload Value
///
/// This register is used to specify the start value to load into the current value register STCVR.CURRENT when the counter reaches 0. It can be any value between 1 and 0x00FFFFFF. A start value of 0 is possible, but has no effect because the SysTick interrupt and STCSR.COUNTFLAG are activated when counting from 1 to 0.
///
/// [TI-TRM-I] 2.7.4.4 STRVR Register (Offset = 14h) [reset = X]
pub mod STRVR;
/// Vector Table Offset
///
/// This register is used to relocated the vector table base address. The vector table base offset determines the offset from the bottom of the memory map. The two most significant bits and the seven least significant bits of the vector table base offset must be 0. The portion of vector table base offset that is allowed to change is TBLOFF.
///
/// [TI-TRM-I] 2.7.4.28 VTOR Register (Offset = D08h) [reset = 0h]
pub mod VTOR;
