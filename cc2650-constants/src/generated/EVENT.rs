use cmemu_common::Address;
use core::ops::Range;

pub const BASE_ADDR: Address = Address::from_const(0x40083000);
pub const ADDR: Address = BASE_ADDR.offset(0x0);
pub const SIZE: u32 = 0x1000;
/// 0x40083000..0x40084000
pub const ADDR_SPACE: Range<Address> = ADDR..ADDR.offset(SIZE);

/// Output Selection for AUX Subscriber 0
///
/// [TI-TRM-I] 4.7.2.92 AUXSEL0 Register (Offset = 700h) [reset = 10h]
pub mod AUXSEL0;
/// Output Selection for NMI Subscriber 0
///
/// [TI-TRM-I] 4.7.2.93 CM3NMISEL0 Register (Offset = 800h) [reset = 63h]
pub mod CM3NMISEL0;
/// Output Selection for CPU Interrupt 0
///
/// [TI-TRM-I] 4.7.2.1 CPUIRQSEL0 Register (Offset = 0h) [reset = 4h]
pub mod CPUIRQSEL0;
/// Output Selection for CPU Interrupt 1
///
/// [TI-TRM-I] 4.7.2.2 CPUIRQSEL1 Register (Offset = 4h) [reset = 9h]
pub mod CPUIRQSEL1;
/// Output Selection for CPU Interrupt 10
///
/// [TI-TRM-I] 4.7.2.11 CPUIRQSEL10 Register (Offset = 28h) [reset = 1Ah]
pub mod CPUIRQSEL10;
/// Output Selection for CPU Interrupt 11
///
/// [TI-TRM-I] 4.7.2.12 CPUIRQSEL11 Register (Offset = 2Ch) [reset = 19h]
pub mod CPUIRQSEL11;
/// Output Selection for CPU Interrupt 12
///
/// [TI-TRM-I] 4.7.2.13 CPUIRQSEL12 Register (Offset = 30h) [reset = 8h]
pub mod CPUIRQSEL12;
/// Output Selection for CPU Interrupt 13
///
/// [TI-TRM-I] 4.7.2.14 CPUIRQSEL13 Register (Offset = 34h) [reset = 1Dh]
pub mod CPUIRQSEL13;
/// Output Selection for CPU Interrupt 14
///
/// [TI-TRM-I] 4.7.2.15 CPUIRQSEL14 Register (Offset = 38h) [reset = 18h]
pub mod CPUIRQSEL14;
/// Output Selection for CPU Interrupt 15
///
/// [TI-TRM-I] 4.7.2.16 CPUIRQSEL15 Register (Offset = 3Ch) [reset = 10h]
pub mod CPUIRQSEL15;
/// Output Selection for CPU Interrupt 16
///
/// [TI-TRM-I] 4.7.2.17 CPUIRQSEL16 Register (Offset = 40h) [reset = 11h]
pub mod CPUIRQSEL16;
/// Output Selection for CPU Interrupt 17
///
/// [TI-TRM-I] 4.7.2.18 CPUIRQSEL17 Register (Offset = 44h) [reset = 12h]
pub mod CPUIRQSEL17;
/// Output Selection for CPU Interrupt 18
///
/// [TI-TRM-I] 4.7.2.19 CPUIRQSEL18 Register (Offset = 48h) [reset = 13h]
pub mod CPUIRQSEL18;
/// Output Selection for CPU Interrupt 19
///
/// [TI-TRM-I] 4.7.2.20 CPUIRQSEL19 Register (Offset = 4Ch) [reset = Ch]
pub mod CPUIRQSEL19;
/// Output Selection for CPU Interrupt 2
///
/// [TI-TRM-I] 4.7.2.3 CPUIRQSEL2 Register (Offset = 8h) [reset = 1Eh]
pub mod CPUIRQSEL2;
/// Output Selection for CPU Interrupt 20
///
/// [TI-TRM-I] 4.7.2.21 CPUIRQSEL20 Register (Offset = 50h) [reset = Dh]
pub mod CPUIRQSEL20;
/// Output Selection for CPU Interrupt 21
///
/// [TI-TRM-I] 4.7.2.22 CPUIRQSEL21 Register (Offset = 54h) [reset = Eh]
pub mod CPUIRQSEL21;
/// Output Selection for CPU Interrupt 22
///
/// [TI-TRM-I] 4.7.2.23 CPUIRQSEL22 Register (Offset = 58h) [reset = Fh]
pub mod CPUIRQSEL22;
/// Output Selection for CPU Interrupt 23
///
/// [TI-TRM-I] 4.7.2.24 CPUIRQSEL23 Register (Offset = 5Ch) [reset = 5Dh]
pub mod CPUIRQSEL23;
/// Output Selection for CPU Interrupt 24
///
/// [TI-TRM-I] 4.7.2.25 CPUIRQSEL24 Register (Offset = 60h) [reset = 27h]
pub mod CPUIRQSEL24;
/// Output Selection for CPU Interrupt 25
///
/// [TI-TRM-I] 4.7.2.26 CPUIRQSEL25 Register (Offset = 64h) [reset = 26h]
pub mod CPUIRQSEL25;
/// Output Selection for CPU Interrupt 26
///
/// [TI-TRM-I] 4.7.2.27 CPUIRQSEL26 Register (Offset = 68h) [reset = 15h]
pub mod CPUIRQSEL26;
/// Output Selection for CPU Interrupt 27
///
/// [TI-TRM-I] 4.7.2.28 CPUIRQSEL27 Register (Offset = 6Ch) [reset = 64h]
pub mod CPUIRQSEL27;
/// Output Selection for CPU Interrupt 28
///
/// [TI-TRM-I] 4.7.2.29 CPUIRQSEL28 Register (Offset = 70h) [reset = Bh]
pub mod CPUIRQSEL28;
/// Output Selection for CPU Interrupt 29
///
/// [TI-TRM-I] 4.7.2.30 CPUIRQSEL29 Register (Offset = 74h) [reset = 1h]
pub mod CPUIRQSEL29;
/// Output Selection for CPU Interrupt 3
///
/// [TI-TRM-I] 4.7.2.4 CPUIRQSEL3 Register (Offset = Ch) [reset = 38h]
pub mod CPUIRQSEL3;
/// Output Selection for CPU Interrupt 30
///
/// [TI-TRM-I] 4.7.2.31 CPUIRQSEL30 Register (Offset = 78h) [reset = 0h]
pub mod CPUIRQSEL30;
/// Output Selection for CPU Interrupt 31
///
/// [TI-TRM-I] 4.7.2.32 CPUIRQSEL31 Register (Offset = 7Ch) [reset = 6Ah]
pub mod CPUIRQSEL31;
/// Output Selection for CPU Interrupt 32
///
/// [TI-TRM-I] 4.7.2.33 CPUIRQSEL32 Register (Offset = 80h) [reset = 73h]
pub mod CPUIRQSEL32;
/// Output Selection for CPU Interrupt 33
///
/// [TI-TRM-I] 4.7.2.34 CPUIRQSEL33 Register (Offset = 84h) [reset = 68h]
pub mod CPUIRQSEL33;
/// Output Selection for CPU Interrupt 4
///
/// [TI-TRM-I] 4.7.2.5 CPUIRQSEL4 Register (Offset = 10h) [reset = 7h]
pub mod CPUIRQSEL4;
/// Output Selection for CPU Interrupt 5
///
/// [TI-TRM-I] 4.7.2.6 CPUIRQSEL5 Register (Offset = 14h) [reset = 24h]
pub mod CPUIRQSEL5;
/// Output Selection for CPU Interrupt 6
///
/// [TI-TRM-I] 4.7.2.7 CPUIRQSEL6 Register (Offset = 18h) [reset = 1Ch]
pub mod CPUIRQSEL6;
/// Output Selection for CPU Interrupt 7
///
/// [TI-TRM-I] 4.7.2.8 CPUIRQSEL7 Register (Offset = 1Ch) [reset = 22h]
pub mod CPUIRQSEL7;
/// Output Selection for CPU Interrupt 8
///
/// [TI-TRM-I] 4.7.2.9 CPUIRQSEL8 Register (Offset = 20h) [reset = 23h]
pub mod CPUIRQSEL8;
/// Output Selection for CPU Interrupt 9
///
/// [TI-TRM-I] 4.7.2.10 CPUIRQSEL9 Register (Offset = 24h) [reset = 1Bh]
pub mod CPUIRQSEL9;
/// Output Selection for FRZ Subscriber
///
/// The halted debug signal is passed to peripherals such as the General Purpose Timer, Sensor Controller with Digital and Analog Peripherals (AUX), Radio, and RTC. When the system CPU halts, the connected peripherals that have freeze enabled also halt. The programmable output can be set to static values of 0 or 1, and can also be set to pass the halted signal.
///
/// [TI-TRM-I] 4.7.2.95 FRZSEL0 Register (Offset = A00h) [reset = 78h]
pub mod FRZSEL0;
/// Output Selection for GPT0 0
///
/// [TI-TRM-I] 4.7.2.45 GPT0ACAPTSEL Register (Offset = 200h) [reset = 55h]
pub mod GPT0ACAPTSEL;
/// Output Selection for GPT0 1
///
/// [TI-TRM-I] 4.7.2.46 GPT0BCAPTSEL Register (Offset = 204h) [reset = 56h]
pub mod GPT0BCAPTSEL;
/// Output Selection for GPT1 0
///
/// [TI-TRM-I] 4.7.2.47 GPT1ACAPTSEL Register (Offset = 300h) [reset = 57h]
pub mod GPT1ACAPTSEL;
/// Output Selection for GPT1 1
///
/// [TI-TRM-I] 4.7.2.48 GPT1BCAPTSEL Register (Offset = 304h) [reset = 58h]
pub mod GPT1BCAPTSEL;
/// Output Selection for GPT2 0
///
/// [TI-TRM-I] 4.7.2.49 GPT2ACAPTSEL Register (Offset = 400h) [reset = 59h]
pub mod GPT2ACAPTSEL;
/// Output Selection for GPT2 1
///
/// [TI-TRM-I] 4.7.2.50 GPT2BCAPTSEL Register (Offset = 404h) [reset = 5Ah]
pub mod GPT2BCAPTSEL;
/// Output Selection for GPT3 0
///
/// [TI-TRM-I] 4.7.2.90 GPT3ACAPTSEL Register (Offset = 600h) [reset = 5Bh]
pub mod GPT3ACAPTSEL;
/// Output Selection for GPT3 1
///
/// [TI-TRM-I] 4.7.2.91 GPT3BCAPTSEL Register (Offset = 604h) [reset = 5Ch]
pub mod GPT3BCAPTSEL;
/// Output Selection for I2S Subscriber 0
///
/// [TI-TRM-I] 4.7.2.94 I2SSTMPSEL0 Register (Offset = 900h) [reset = 5Fh]
pub mod I2SSTMPSEL0;
/// Output Selection for RFC Event 0
///
/// [TI-TRM-I] 4.7.2.35 RFCSEL0 Register (Offset = 100h) [reset = 3Dh]
pub mod RFCSEL0;
/// Output Selection for RFC Event 1
///
/// [TI-TRM-I] 4.7.2.36 RFCSEL1 Register (Offset = 104h) [reset = 3Eh]
pub mod RFCSEL1;
/// Output Selection for RFC Event 2
///
/// [TI-TRM-I] 4.7.2.37 RFCSEL2 Register (Offset = 108h) [reset = 3Fh]
pub mod RFCSEL2;
/// Output Selection for RFC Event 3
///
/// [TI-TRM-I] 4.7.2.38 RFCSEL3 Register (Offset = 10Ch) [reset = 40h]
pub mod RFCSEL3;
/// Output Selection for RFC Event 4
///
/// [TI-TRM-I] 4.7.2.39 RFCSEL4 Register (Offset = 110h) [reset = 41h]
pub mod RFCSEL4;
/// Output Selection for RFC Event 5
///
/// [TI-TRM-I] 4.7.2.40 RFCSEL5 Register (Offset = 114h) [reset = 42h]
pub mod RFCSEL5;
/// Output Selection for RFC Event 6
///
/// [TI-TRM-I] 4.7.2.41 RFCSEL6 Register (Offset = 118h) [reset = 43h]
pub mod RFCSEL6;
/// Output Selection for RFC Event 7
///
/// [TI-TRM-I] 4.7.2.42 RFCSEL7 Register (Offset = 11Ch) [reset = 44h]
pub mod RFCSEL7;
/// Output Selection for RFC Event 8
///
/// [TI-TRM-I] 4.7.2.43 RFCSEL8 Register (Offset = 120h) [reset = 77h]
pub mod RFCSEL8;
/// Output Selection for RFC Event 9
///
/// [TI-TRM-I] 4.7.2.44 RFCSEL9 Register (Offset = 124h) [reset = 2h]
pub mod RFCSEL9;
/// Set or Clear Software Events
///
/// [TI-TRM-I] 4.7.2.96 SWEV Register (Offset = F00h) [reset = 0h]
pub mod SWEV;
/// Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.
///
/// [TI-TRM-I] undocumented
pub mod UDMACH0BSEL;
/// Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.
///
/// [TI-TRM-I] undocumented
pub mod UDMACH0SSEL;
/// Output Selection for DMA Channel 10 REQ
///
///
///
/// DMA_DONE for the corresponding DMA channel is available as interrupt on GPT0 as GPT0:RIS.DMABRIS
///
/// [TI-TRM-I] 4.7.2.70 UDMACH10BSEL Register (Offset = 554h) [reset = 4Eh]
pub mod UDMACH10BSEL;
/// Output Selection for DMA Channel 10 SREQ
///
///
///
/// DMA_DONE for the corresponding DMA channel is available as interrupt on GPT0 as GPT0:RIS.DMABRIS
///
/// [TI-TRM-I] 4.7.2.69 UDMACH10SSEL Register (Offset = 550h) [reset = 46h]
pub mod UDMACH10SSEL;
/// Output Selection for DMA Channel 11 REQ
///
///
///
/// DMA_DONE for the corresponding DMA channel is available as interrupt on GPT1 as GPT1:RIS.DMAARIS
///
/// [TI-TRM-I] 4.7.2.72 UDMACH11BSEL Register (Offset = 55Ch) [reset = 4Fh]
pub mod UDMACH11BSEL;
/// Output Selection for DMA Channel 11 SREQ
///
///
///
/// DMA_DONE for the corresponding DMA channel is available as interrupt on GPT1 as GPT1:RIS.DMAARIS
///
/// [TI-TRM-I] 4.7.2.71 UDMACH11SSEL Register (Offset = 558h) [reset = 47h]
pub mod UDMACH11SSEL;
/// Output Selection for DMA Channel 12 REQ
///
///
///
/// DMA_DONE for the corresponding DMA channel is available as interrupt on GPT1 as GPT1:RIS.DMABRIS
///
/// [TI-TRM-I] 4.7.2.74 UDMACH12BSEL Register (Offset = 564h) [reset = 50h]
pub mod UDMACH12BSEL;
/// Output Selection for DMA Channel 12 SREQ
///
///
///
/// DMA_DONE for the corresponding DMA channel is available as interrupt on GPT1 as GPT1:RIS.DMABRIS
///
/// [TI-TRM-I] 4.7.2.73 UDMACH12SSEL Register (Offset = 560h) [reset = 48h]
pub mod UDMACH12SSEL;
/// Output Selection for DMA Channel 13 REQ
///
/// [TI-TRM-I] 4.7.2.75 UDMACH13BSEL Register (Offset = 56Ch) [reset = 3h]
pub mod UDMACH13BSEL;
/// Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.
///
/// [TI-TRM-I] undocumented
pub mod UDMACH13SSEL;
/// Output Selection for DMA Channel 14 REQ
///
/// [TI-TRM-I] 4.7.2.76 UDMACH14BSEL Register (Offset = 574h) [reset = 1h]
pub mod UDMACH14BSEL;
/// Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.
///
/// [TI-TRM-I] undocumented
pub mod UDMACH14SSEL;
/// Output Selection for DMA Channel 15 REQ
///
/// [TI-TRM-I] 4.7.2.77 UDMACH15BSEL Register (Offset = 57Ch) [reset = 7h]
pub mod UDMACH15BSEL;
/// Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.
///
/// [TI-TRM-I] undocumented
pub mod UDMACH15SSEL;
/// Output Selection for DMA Channel 16 REQ
///
/// [TI-TRM-I] 4.7.2.79 UDMACH16BSEL Register (Offset = 584h) [reset = 2Ch]
pub mod UDMACH16BSEL;
/// Output Selection for DMA Channel 16 SREQ
///
/// [TI-TRM-I] 4.7.2.78 UDMACH16SSEL Register (Offset = 580h) [reset = 2Dh]
pub mod UDMACH16SSEL;
/// Output Selection for DMA Channel 17 REQ
///
/// [TI-TRM-I] 4.7.2.81 UDMACH17BSEL Register (Offset = 58Ch) [reset = 2Eh]
pub mod UDMACH17BSEL;
/// Output Selection for DMA Channel 17 SREQ
///
/// [TI-TRM-I] 4.7.2.80 UDMACH17SSEL Register (Offset = 588h) [reset = 2Fh]
pub mod UDMACH17SSEL;
/// Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.
///
/// [TI-TRM-I] undocumented
pub mod UDMACH18BSEL;
/// Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.
///
/// [TI-TRM-I] undocumented
pub mod UDMACH18SSEL;
/// Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.
///
/// [TI-TRM-I] undocumented
pub mod UDMACH19BSEL;
/// Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.
///
/// [TI-TRM-I] undocumented
pub mod UDMACH19SSEL;
/// Output Selection for DMA Channel 1 REQ
///
/// [TI-TRM-I] 4.7.2.52 UDMACH1BSEL Register (Offset = 50Ch) [reset = 30h]
pub mod UDMACH1BSEL;
/// Output Selection for DMA Channel 1 SREQ
///
/// [TI-TRM-I] 4.7.2.51 UDMACH1SSEL Register (Offset = 508h) [reset = 31h]
pub mod UDMACH1SSEL;
/// Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.
///
/// [TI-TRM-I] undocumented
pub mod UDMACH20BSEL;
/// Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.
///
/// [TI-TRM-I] undocumented
pub mod UDMACH20SSEL;
/// Output Selection for DMA Channel 21 REQ
///
/// [TI-TRM-I] 4.7.2.83 UDMACH21BSEL Register (Offset = 5ACh) [reset = 64h]
pub mod UDMACH21BSEL;
/// Output Selection for DMA Channel 21 SREQ
///
/// [TI-TRM-I] 4.7.2.82 UDMACH21SSEL Register (Offset = 5A8h) [reset = 64h]
pub mod UDMACH21SSEL;
/// Output Selection for DMA Channel 22 REQ
///
/// [TI-TRM-I] 4.7.2.85 UDMACH22BSEL Register (Offset = 5B4h) [reset = 65h]
pub mod UDMACH22BSEL;
/// Output Selection for DMA Channel 22 SREQ
///
/// [TI-TRM-I] 4.7.2.84 UDMACH22SSEL Register (Offset = 5B0h) [reset = 65h]
pub mod UDMACH22SSEL;
/// Output Selection for DMA Channel 23 REQ
///
/// [TI-TRM-I] 4.7.2.87 UDMACH23BSEL Register (Offset = 5BCh) [reset = 66h]
pub mod UDMACH23BSEL;
/// Output Selection for DMA Channel 23 SREQ
///
/// [TI-TRM-I] 4.7.2.86 UDMACH23SSEL Register (Offset = 5B8h) [reset = 66h]
pub mod UDMACH23SSEL;
/// Output Selection for DMA Channel 24 REQ
///
/// [TI-TRM-I] 4.7.2.89 UDMACH24BSEL Register (Offset = 5C4h) [reset = 67h]
pub mod UDMACH24BSEL;
/// Output Selection for DMA Channel 24 SREQ
///
/// [TI-TRM-I] 4.7.2.88 UDMACH24SSEL Register (Offset = 5C0h) [reset = 67h]
pub mod UDMACH24SSEL;
/// Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.
///
/// [TI-TRM-I] undocumented
pub mod UDMACH25BSEL;
/// Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.
///
/// [TI-TRM-I] undocumented
pub mod UDMACH25SSEL;
/// Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.
///
/// [TI-TRM-I] undocumented
pub mod UDMACH26BSEL;
/// Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.
///
/// [TI-TRM-I] undocumented
pub mod UDMACH26SSEL;
/// Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.
///
/// [TI-TRM-I] undocumented
pub mod UDMACH27BSEL;
/// Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.
///
/// [TI-TRM-I] undocumented
pub mod UDMACH27SSEL;
/// Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.
///
/// [TI-TRM-I] undocumented
pub mod UDMACH28BSEL;
/// Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.
///
/// [TI-TRM-I] undocumented
pub mod UDMACH28SSEL;
/// Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.
///
/// [TI-TRM-I] undocumented
pub mod UDMACH29BSEL;
/// Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.
///
/// [TI-TRM-I] undocumented
pub mod UDMACH29SSEL;
/// Output Selection for DMA Channel 2 REQ
///
/// [TI-TRM-I] 4.7.2.54 UDMACH2BSEL Register (Offset = 514h) [reset = 32h]
pub mod UDMACH2BSEL;
/// Output Selection for DMA Channel 2 SREQ
///
/// [TI-TRM-I] 4.7.2.53 UDMACH2SSEL Register (Offset = 510h) [reset = 33h]
pub mod UDMACH2SSEL;
/// Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.
///
/// [TI-TRM-I] undocumented
pub mod UDMACH30BSEL;
/// Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.
///
/// [TI-TRM-I] undocumented
pub mod UDMACH30SSEL;
/// Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.
///
/// [TI-TRM-I] undocumented
pub mod UDMACH31BSEL;
/// Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.
///
/// [TI-TRM-I] undocumented
pub mod UDMACH31SSEL;
/// Output Selection for DMA Channel 3 REQ
///
/// [TI-TRM-I] 4.7.2.56 UDMACH3BSEL Register (Offset = 51Ch) [reset = 28h]
pub mod UDMACH3BSEL;
/// Output Selection for DMA Channel 3 SREQ
///
/// [TI-TRM-I] 4.7.2.55 UDMACH3SSEL Register (Offset = 518h) [reset = 29h]
pub mod UDMACH3SSEL;
/// Output Selection for DMA Channel 4 REQ
///
/// [TI-TRM-I] 4.7.2.58 UDMACH4BSEL Register (Offset = 524h) [reset = 2Ah]
pub mod UDMACH4BSEL;
/// Output Selection for DMA Channel 4 SREQ
///
/// [TI-TRM-I] 4.7.2.57 UDMACH4SSEL Register (Offset = 520h) [reset = 2Bh]
pub mod UDMACH4SSEL;
/// Output Selection for DMA Channel 5 REQ
///
/// [TI-TRM-I] 4.7.2.60 UDMACH5BSEL Register (Offset = 52Ch) [reset = 39h]
pub mod UDMACH5BSEL;
/// Output Selection for DMA Channel 5 SREQ
///
/// [TI-TRM-I] 4.7.2.59 UDMACH5SSEL Register (Offset = 528h) [reset = 3Ah]
pub mod UDMACH5SSEL;
/// Output Selection for DMA Channel 6 REQ
///
/// [TI-TRM-I] 4.7.2.62 UDMACH6BSEL Register (Offset = 534h) [reset = 3Bh]
pub mod UDMACH6BSEL;
/// Output Selection for DMA Channel 6 SREQ
///
/// [TI-TRM-I] 4.7.2.61 UDMACH6SSEL Register (Offset = 530h) [reset = 3Ch]
pub mod UDMACH6SSEL;
/// Output Selection for DMA Channel 7 REQ
///
/// [TI-TRM-I] 4.7.2.64 UDMACH7BSEL Register (Offset = 53Ch) [reset = 76h]
pub mod UDMACH7BSEL;
/// Output Selection for DMA Channel 7 SREQ
///
/// [TI-TRM-I] 4.7.2.63 UDMACH7SSEL Register (Offset = 538h) [reset = 75h]
pub mod UDMACH7SSEL;
/// Output Selection for DMA Channel 8 REQ
///
/// [TI-TRM-I] 4.7.2.66 UDMACH8BSEL Register (Offset = 544h) [reset = 74h]
pub mod UDMACH8BSEL;
/// Output Selection for DMA Channel 8 SREQ
///
///
///
/// Single request is ignored for this channel
///
/// [TI-TRM-I] 4.7.2.65 UDMACH8SSEL Register (Offset = 540h) [reset = 74h]
pub mod UDMACH8SSEL;
/// Output Selection for DMA Channel 9 REQ
///
///
///
/// DMA_DONE for the corresponding DMA channel is available as interrupt on GPT0 as GPT0:RIS.DMAARIS
///
/// [TI-TRM-I] 4.7.2.68 UDMACH9BSEL Register (Offset = 54Ch) [reset = 4Dh]
pub mod UDMACH9BSEL;
/// Output Selection for DMA Channel 9 SREQ
///
///
///
/// DMA_DONE for the corresponding DMA channel is available as interrupt on GPT0 as GPT0:RIS.DMAARIS
///
/// [TI-TRM-I] 4.7.2.67 UDMACH9SSEL Register (Offset = 548h) [reset = 45h]
pub mod UDMACH9SSEL;
