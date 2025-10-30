// THIS FILE IS GENERATED AUTOMATICALLY
// see mm319369/svd in playground

// Allow uppercase register names etc to appear in comments.
#![allow(clippy::doc_markdown)]
// We tried but cannot fix this.
#![allow(clippy::too_long_first_doc_paragraph)]
// There are some modules in SVD that only have one register named like the module.
#![allow(clippy::module_inception)]
// This heuristic fails a lot on all-uppercase module names.
#![allow(clippy::module_name_repetitions)]
// Ignore address literals without underscore in the middle
#![allow(clippy::unreadable_literal)]
// TODO: The documentation chunks come from TI's Code Composer Studio / Uniflash,
//       and are possible under an incompatibile license.
//       Possibly we need to reference BSD-3 clause of driverlib - these comments can be published.

//! SimpleLink(TM) multi-protocol CC2650 wireless MCU for 2.4 GHz applications
pub const SOC_NAME: &str = "CC2650F128";
pub const VENDOR: &str = "Texas Instruments";
pub const VENDOR_ID: &str = "TI";
pub const ADDRESS_BITS: u8 = 8;
pub const DEFAULT_REG_WIDTH: u8 = 32;

pub mod AON;
pub mod AUX;
pub mod CPU;
pub mod RFC;

pub mod interrupts;

/// Always On (AON) Battery And Temperature MONitor (BATMON) residing in the AON domain  Note: This module only supports 32 bit Read/Write access from MCU.
///
/// Range: 0x40095000 - 0x40095400
///
/// [TI-TRM-I] 18.3.1 AON_BATMON Registers
pub mod AON_BATMON;
/// This module configures the event fabric located in the AON domain.
///
///
///
/// Note: This module is only supporting 32 bit ReadWrite access from MCU
///
/// Range: 0x40093000 - 0x40093400
///
/// [TI-TRM-I] 4.7.1 AON_EVENT Registers
pub mod AON_EVENT;
/// Always On (AON) IO Controller  - controls IO operation when the MCU IO Controller (IOC) is powered off and resides in the AON domain.  Note: This module only supports 32 bit Read/Write access from MCU.
///
/// Range: 0x40094000 - 0x40094400
///
/// [TI-TRM-I] 11.11.1 AON_IOC Registers
pub mod AON_IOC;
/// This component control the Real Time Clock residing in AON
///
///
///
/// Note: This module is only supporting 32 bit ReadWrite access.
///
/// Range: 0x40092000 - 0x40092400
///
/// [TI-TRM-I] 14.4.1 AON_RTC Registers
pub mod AON_RTC;
/// This component controls AON_SYSCTL, which is the device's system controller.
///
///
///
/// Note: This module is only supporting 32 bit ReadWrite access from MCU
///
/// Range: 0x40090000 - 0x40090400
///
/// [TI-TRM-I] 6.8.2.2 AON_SYSCTL Registers
pub mod AON_SYSCTL;
/// This component control the Wakeup controller residing in the AON domain.
///
///
///
/// Note: This module is only supporting 32 bit ReadWrite access from MCU
///
/// Range: 0x40091000 - 0x40092000
///
/// [TI-TRM-I] 6.8.2.3 AON_WUC Registers
pub mod AON_WUC;
/// Configuration registers controlling analog peripherals of AUX. Registers Fields should be considered static unless otherwise noted (as dynamic)
///
/// Range: 0x400cb000 - 0x400cb200
///
/// [TI-TRM-I] 17.7.1 ADI_4_AUX Registers
pub mod AUX_ADI4;
/// AUX Analog/Digital Input Output Controller
///
/// Range: 0x400c1000 - 0x400c2000
///
/// [TI-TRM-I] 17.7.2 AUX_AIODIO Registers
pub mod AUX_AIODIO0;
/// AUX Analog/Digital Input Output Controller
///
/// Range: 0x400c2000 - 0x400c3000
///
/// [TI-TRM-I] 17.7.2 AUX_AIODIO Registers
pub mod AUX_AIODIO1;
/// AUX Analog Peripheral Control Module
///
/// Range: 0x400c9000 - 0x400ca000
///
/// [TI-TRM-I] 17.7.8 AUX_ANAIF Registers
pub mod AUX_ANAIF;
/// This is the DDI for the digital block that controls all the analog clock oscillators  (OSC_DIG) and performs qualification of the clocks generated.
///
/// Range: 0x400ca000 - 0x400cb000
///
/// [TI-TRM-I] 6.8.2.1 DDI_0_OSC Registers
pub mod AUX_DDI0_OSC;
/// AUX Event Controller
///
/// Range: 0x400c5000 - 0x400c6000
///
/// [TI-TRM-I] 17.7.3 AUX_EVCTL Registers
pub mod AUX_EVCTL;
/// AUX Sensor Control Engine Control Module
///
/// Range: 0x400e1000 - 0x400e2000
///
/// [TI-TRM-I] undocumented
pub mod AUX_SCE;
/// AUX Semaphore Controller
///
/// Range: 0x400c8000 - 0x400c9000
///
/// [TI-TRM-I] 17.7.4 AUX_SMPH Registers
pub mod AUX_SMPH;
/// AUX Time To Digital Converter
///
/// Range: 0x400c4000 - 0x400c5000
///
/// [TI-TRM-I] 17.7.5 AUX_TDC Registers
pub mod AUX_TDCIF;
/// AUX Timer
///
/// Range: 0x400c7000 - 0x400c8000
///
/// [TI-TRM-I] 17.7.6 AUX_TIMER Registers
pub mod AUX_TIMER;
/// AUX Wake-up controller
///
/// Range: 0x400c6000 - 0x400c7000
///
/// [TI-TRM-I] 17.7.7 AUX_WUC Registers
pub mod AUX_WUC;
/// Customer configuration area (CCFG)
///
/// Range: 0x50003000 - 0x50004000
///
/// [TI-TRM-I] 9.1.1 CCFG Registers
pub mod CCFG;
/// Cortex-M's Data watchpoint and Trace (DWT)
///
/// Range: 0xe0001000 - 0xe0002000
///
/// [TI-TRM-I] 2.7.1 CPU_DWT Registers
pub mod CPU_DWT;
/// Cortex-M's Flash Patch and Breakpoint (FPB)
///
/// Range: 0xe0002000 - 0xe0003000
///
/// [TI-TRM-I] 2.7.2 CPU_FPB Registers
pub mod CPU_FPB;
/// Cortex-M's Instrumentation Trace Macrocell (ITM)
///
/// Range: 0xe0000000 - 0xe0001000
///
/// [TI-TRM-I] 2.7.3 CPU_ITM Registers
pub mod CPU_ITM;
/// Cortex-M's System Control Space (SCS)
///
/// Range: 0xe000e000 - 0xe000f000
///
/// [TI-TRM-I] 2.7.4 CPU_SCS Registers
pub mod CPU_SCS;
/// Cortex-M's TI proprietary registers
///
/// Range: 0xe00fe000 - 0xe00ff000
///
/// [TI-TRM-I] undocumented
pub mod CPU_TIPROP;
/// Cortex-M3's Trace Port Interface Unit (TPIU)
///
/// Range: 0xe0040000 - 0xe0041000
///
/// [TI-TRM-I] 2.7.5 CPU_TPIU Registers
pub mod CPU_TPIU;
/// Crypto core with DMA capability and local key storage
///
/// Range: 0x40024000 - 0x40024800
///
/// [TI-TRM-I] 10.9.1 CRYPTO Registers
pub mod CRYPTO;
/// Event Fabric Component Definition
///
/// Range: 0x40083000 - 0x40084000
///
/// [TI-TRM-I] 4.7.2 EVENT Registers
pub mod EVENT;
/// Factory configuration area (FCFG1)
///
/// Range: 0x50001000 - 0x50001400
///
/// [TI-TRM-I] 9.2.2.1 FCFG1 Registers
pub mod FCFG1;
/// Flash sub-system registers, includes the Flash Memory Controller (FMC), flash read path, and an integrated Efuse controller and EFUSEROM.
///
/// Range: 0x40030000 - 0x40034000
///
/// [TI-TRM-I] 7.9.1 FLASH Registers
pub mod FLASH;
/// MCU GPIO - I/F for controlling and reading IO status and IO event status
///
/// Range: 0x40022000 - 0x40022400
///
/// [TI-TRM-I] 11.11.2 GPIO Registers
pub mod GPIO;
/// General Purpose Timer.
///
/// Range: 0x40010000 - 0x40011000
///
/// [TI-TRM-I] 13.5.1 GPT Registers
pub mod GPT0;
/// General Purpose Timer.
///
/// Range: 0x40011000 - 0x40012000
///
/// [TI-TRM-I] 13.5.1 GPT Registers
pub mod GPT1;
/// General Purpose Timer.
///
/// Range: 0x40012000 - 0x40013000
///
/// [TI-TRM-I] 13.5.1 GPT Registers
pub mod GPT2;
/// General Purpose Timer.
///
/// Range: 0x40013000 - 0x40014000
///
/// [TI-TRM-I] 13.5.1 GPT Registers
pub mod GPT3;
/// I2CMaster/Slave Serial Controler
///
/// Range: 0x40002000 - 0x40003000
///
/// [TI-TRM-I] 21.5.1 I2C Registers
pub mod I2C0;
/// I2S Audio DMA module supporting formats I2S, LJF, RJF and DSP
///
/// Range: 0x40021000 - 0x40022000
///
/// [TI-TRM-I] 22.10.1 I2S Registers
pub mod I2S0;
/// IO Controller (IOC) - configures all the DIOs and resides in the MCU domain.
///
/// Range: 0x40081000 - 0x40082000
///
/// [TI-TRM-I] 11.11.3 IOC Registers
pub mod IOC;
/// Power, Reset and Clock Management
///
/// Range: 0x40082000 - 0x40083000
///
/// [TI-TRM-I] 6.8.2.4 PRCM Registers
pub mod PRCM;
/// RF Core Doorbell
///
/// Range: 0x40041000 - 0x40041040
///
/// [TI-TRM-I] 23.8.2 RFC_DBELL Registers
pub mod RFC_DBELL;
/// RF Core Power Management
///
/// Range: 0x40040000 - 0x40040004
///
/// [TI-TRM-I] 23.8.3 RFC_PWR Registers
pub mod RFC_PWR;
/// RF Core Radio Timer
///
/// Range: 0x40043000 - 0x40043100
///
/// [TI-TRM-I] 23.8.1 RFC_RAT Registers
pub mod RFC_RAT;
/// MCU Semaphore Module
///
///
///
/// This module provides 32 binary semaphores. The state of a binary semaphore is either taken or available.
///
///
///
/// A semaphore does not implement any ownership attribute. Still, a semaphore can be used to handle mutual exclusion scenarios.
///
/// Range: 0x40084000 - 0x40085000
///
/// [TI-TRM-I] undocumented
pub mod SMPH;
/// Synchronous Serial Interface with master and slave capabilities
///
/// Range: 0x40000000 - 0x40001000
///
/// [TI-TRM-I] 20.7.1 SSI Registers
pub mod SSI0;
/// Synchronous Serial Interface with master and slave capabilities
///
/// Range: 0x40008000 - 0x40009000
///
/// [TI-TRM-I] 20.7.1 SSI Registers
pub mod SSI1;
/// True Random Number Generator
///
/// Range: 0x40028000 - 0x4002a000
///
/// [TI-TRM-I] 16.7.1 TRNG Registers
pub mod TRNG;
/// Universal Asynchronous Receiver/Transmitter (UART) interface
///
/// Range: 0x40001000 - 0x40002000
///
/// [TI-TRM-I] 19.8.1 UART Registers
pub mod UART0;
/// ARM Micro Direct Memory Access Controller
///
/// Range: 0x40020000 - 0x40021000
///
/// [TI-TRM-I] 12.5.1 UDMA Registers
pub mod UDMA0;
/// Versatile Instruction Memory System
///
/// Controls memory access to the Flash and encapsulates the following instruction memories:
///
/// - Boot ROM
///
/// - Cache / GPRAM
///
/// Range: 0x40034000 - 0x40034400
///
/// [TI-TRM-I] 7.9.2 VIMS Registers
pub mod VIMS;
/// Watchdog Timer
///
/// Range: 0x40080000 - 0x40081000
///
/// [TI-TRM-I] 15.4.1 WDT Registers
pub mod WDT;
