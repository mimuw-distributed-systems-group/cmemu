use cmemu_common::Address;
use core::ops::Range;

pub const BASE_ADDR: Address = Address::from_const(0x40011000);
pub const ADDR: Address = BASE_ADDR.offset(0x0);
pub const SIZE: u32 = 0x1000;
/// 0x40011000..0x40012000
pub const ADDR_SPACE: Range<Address> = ADDR..ADDR.offset(SIZE);

/// Combined CCP Output
///
/// This register is used to logically AND CCP output pairs for each timer
///
/// [TI-TRM-I] 13.5.1.28 ANDCCP Register (Offset = FB4h) [reset = 0h]
pub mod ANDCCP;
/// Configuration
///
/// [TI-TRM-I] 13.5.1.1 CFG Register (Offset = 0h) [reset = 0h]
pub mod CFG;
/// Control
///
/// [TI-TRM-I] 13.5.1.4 CTL Register (Offset = Ch) [reset = 0h]
pub mod CTL;
/// DMA Event
///
/// This register allows software to enable/disable GPT DMA trigger events.
///
/// [TI-TRM-I] 13.5.1.26 DMAEV Register (Offset = 6Ch) [reset = 0h]
pub mod DMAEV;
/// Interrupt Clear
///
/// This register is used to clear status bits in the RIS and MIS registers
///
/// [TI-TRM-I] 13.5.1.9 ICLR Register (Offset = 24h) [reset = 0h]
pub mod ICLR;
/// Interrupt Mask
///
/// This register is used to enable the interrupts.
///
/// Associated registers:
///
/// RIS, MIS, ICLR
///
/// [TI-TRM-I] 13.5.1.6 IMR Register (Offset = 18h) [reset = 0h]
pub mod IMR;
/// Masked Interrupt Status
///
/// Values are result of bitwise AND operation between RIS and IMR
///
/// Assosciated clear register: ICLR
///
/// [TI-TRM-I] 13.5.1.8 MIS Register (Offset = 20h) [reset = 0h]
pub mod MIS;
/// Raw Interrupt Status
///
/// Associated registers:
///
/// IMR, MIS, ICLR
///
/// [TI-TRM-I] 13.5.1.7 RIS Register (Offset = 1Ch) [reset = 0h]
pub mod RIS;
/// Synch Register
///
/// [TI-TRM-I] 13.5.1.5 SYNC Register (Offset = 10h) [reset = 0h]
pub mod SYNC;
/// Timer A Interval Load  Register
///
/// [TI-TRM-I] 13.5.1.10 TAILR Register (Offset = 28h) [reset = FFFFFFFFh]
pub mod TAILR;
/// Timer A Match Register
///
///
///
/// Interrupts can be generated when the timer value is equal to the value in this register in one-shot or periodic mode.
///
///
///
/// In Edge-Count mode, this register along with TAILR, determines how many edge events are counted.
///
/// The total number of edge events counted is equal to the value in TAILR minus this value.
///
///
///
/// Note that in edge-count mode, when executing an up-count, the value of TAPR and TAILR must be greater than the value of TAPMR and this register.
///
///
///
/// In PWM mode, this value along with TAILR, determines the duty cycle of the output PWM signal.
///
///
///
/// When a 16/32-bit GPT is configured to one of the 32-bit modes, TAMATCHR appears as a 32-bit register. (The upper 16-bits correspond to the contents TBMATCHR).
///
///
///
/// In a 16-bit mode, the upper 16 bits of this register read as 0s and have no effect on the state of TBMATCHR.
///
///
///
/// Note : This register is updated internally (takes effect) based on TAMR.TAMRSU
///
/// [TI-TRM-I] 13.5.1.12 TAMATCHR Register (Offset = 30h) [reset = FFFFFFFFh]
pub mod TAMATCHR;
/// Timer A Mode
///
/// [TI-TRM-I] 13.5.1.2 TAMR Register (Offset = 4h) [reset = 0h]
pub mod TAMR;
/// Timer A Pre-scale Match
///
/// This register allows software to extend the range of the TAMATCHR when used individually.
///
/// [TI-TRM-I] 13.5.1.16 TAPMR Register (Offset = 40h) [reset = 0h]
pub mod TAPMR;
/// Timer A Pre-scale
///
/// This register allows software to extend the range of the timers when they are used individually.
///
/// When in one-shot or periodic down count modes, this register acts as a true prescaler for the timer counter.
///
/// When acting as a true prescaler, the prescaler counts down to 0 before the value in TAR and TAV registers are incremented.
///
/// In all other individual/split modes, this register is a linear extension of the upper range of the timer counter, holding bits 23:16 in the 16-bit modes of the 16/32-bit GPT.
///
/// [TI-TRM-I] 13.5.1.14 TAPR Register (Offset = 38h) [reset = 0h]
pub mod TAPR;
/// Timer A Pre-scale Snap-shot
///
///
///
/// Based on the value in the register field TAMR.TAILD, this register is updated with the value from TAPR register either on the next cycle or on the next timeout.
///
///
///
///
///
/// This register shows the current value of the Timer A pre-scaler in the 16-bit mode.
///
/// [TI-TRM-I] 13.5.1.22 TAPS Register (Offset = 5Ch) [reset = 0h]
pub mod TAPS;
/// Timer A Pre-scale Value
///
/// This register shows the current value of the Timer A free running pre-scaler in the 16-bit mode.
///
/// [TI-TRM-I] 13.5.1.24 TAPV Register (Offset = 64h) [reset = 0h]
pub mod TAPV;
/// Timer A Register
///
/// This register shows the current value of the Timer A counter in all cases except for Input Edge Count and Time modes. In the Input Edge Count mode, this register contains the number of edges that
///
/// have occurred. In the Input Edge Time mode, this register contains the time at which the last edge event took place.
///
///
///
/// When a GPT is configured to one of the 32-bit modes, this register appears as a 32-bit register (the upper 16-bits correspond to the contents of the Timer B (TBR) register). In
///
/// the16-bit Input Edge Count, Input Edge Time, and PWM modes, bits 15:0 contain the value of the counter and bits 23:16 contain the value of the prescaler, which is the upper 8 bits of the count. Bits
///
/// 31:24 always read as 0. To read the value of the prescaler in 16-bit One-Shot and Periodic modes, read bits \[23:16\] in the TAV register. To read the value of the prescalar in periodic snapshot
///
/// mode, read the Timer A Prescale Snapshot (TAPS) register.
///
/// [TI-TRM-I] 13.5.1.18 TAR Register (Offset = 48h) [reset = FFFFFFFFh]
pub mod TAR;
/// Timer A Value
///
/// When read, this register shows the current, free-running value of Timer A in all modes. Softwarecan use this value to determine the time elapsed between an interrupt and the ISR entry when using
///
/// the snapshot feature with the periodic operating mode. When written, the value written into this register is loaded into the TAR register on the next clock cycle.
///
///
///
/// When a 16/32-bit GPTM is configured to one of the 32-bit modes, this register appears as a 32-bit register (the upper 16-bits correspond to the contents of the GPTM Timer B Value (TBV) register). In a 16-bit mode, bits 15:0 contain the value of the counter and bits 23:16 contain the current, free-running value of the prescaler, which is the upper 8 bits of the count in Input Edge Count, Input Edge Time, PWM and one-shot or periodic up count modes. In one-shot or periodic
///
/// down count modes, the prescaler stored in 23:16 is a true prescaler, meaning bits 23:16 count down before decrementing the value in bits 15:0. The prescaler in bits 31:24 always reads as 0.
///
/// [TI-TRM-I] 13.5.1.20 TAV Register (Offset = 50h) [reset = FFFFFFFFh]
pub mod TAV;
/// Timer B Interval Load  Register
///
/// [TI-TRM-I] 13.5.1.11 TBILR Register (Offset = 2Ch) [reset = FFFFh]
pub mod TBILR;
/// Timer B Match Register
///
///
///
///  When a GPT is configured to one of the 32-bit modes, the contents of bits 15:0 in this register are loaded into the upper 16 bits of  TAMATCHR.
///
/// Reads from this register return the current match value of Timer B and writes are ignored.
///
/// In a 16-bit mode, bits 15:0 are used for the match value. Bits 31:16 are reserved in both cases.
///
///
///
/// Note : This register is updated internally (takes effect) based on TBMR.TBMRSU
///
/// [TI-TRM-I] 13.5.1.13 TBMATCHR Register (Offset = 34h) [reset = FFFFh]
pub mod TBMATCHR;
/// Timer B Mode
///
/// [TI-TRM-I] 13.5.1.3 TBMR Register (Offset = 8h) [reset = 0h]
pub mod TBMR;
/// Timer B Pre-scale Match
///
/// This register allows software to extend the range of the TBMATCHR when used individually.
///
/// [TI-TRM-I] 13.5.1.17 TBPMR Register (Offset = 44h) [reset = 0h]
pub mod TBPMR;
/// Timer B Pre-scale
///
/// This register allows software to extend the range of the timers when they are used individually.
///
/// When in one-shot or periodic down count modes, this register acts as a true prescaler for the timer counter.
///
/// When acting as a true prescaler, the prescaler counts down to 0 before the value in TBR and TBV registers are incremented.
///
/// In all other individual/split modes, this register is a linear extension of the upper range of the timer counter, holding bits 23:16 in the 16-bit modes of the 16/32-bit GPT.
///
/// [TI-TRM-I] 13.5.1.15 TBPR Register (Offset = 3Ch) [reset = 0h]
pub mod TBPR;
/// Timer B Pre-scale Snap-shot
///
///
///
/// Based on the value in the register field TBMR.TBILD, this register is updated with the value from TBPR register either on the next cycle or on the next timeout.
///
///
///
/// This register shows the current value of the Timer B pre-scaler in the 16-bit mode.
///
/// [TI-TRM-I] 13.5.1.23 TBPS Register (Offset = 60h) [reset = 0h]
pub mod TBPS;
/// Timer B Pre-scale Value
///
/// This register shows the current value of the Timer B free running pre-scaler in the 16-bit mode.
///
/// [TI-TRM-I] 13.5.1.25 TBPV Register (Offset = 68h) [reset = 0h]
pub mod TBPV;
/// Timer B Register
///
/// This register shows the current value of the Timer B counter in all cases except for Input Edge Count and Time modes. In the Input Edge Count mode, this register contains the number of edges that
///
/// have occurred. In the Input Edge Time mode, this register contains the time at which the last edge event took place.
///
///
///
/// When a GPTM is configured to one of the 32-bit modes, the contents of bits 15:0 in this register are loaded into the upper 16 bits of the TAR register. Reads from this register return the current
///
/// value of Timer B. In a 16-bit mode, bits 15:0 contain the value of the counter and bits 23:16 contain the value of the prescaler in Input Edge Count, Input Edge Time, and PWM modes, which is the
///
/// upper 8 bits of the count. Bits 31:24 always read as 0. To read the value of the prescaler in 16-bit One-Shot and Periodic modes, read bits \[23:16\] in the TBV register. To read the value of the
///
/// prescalar in periodic snapshot mode, read the Timer B Prescale Snapshot (TBPS) register.
///
/// [TI-TRM-I] 13.5.1.19 TBR Register (Offset = 4Ch) [reset = FFFFh]
pub mod TBR;
/// Timer B Value
///
/// When read, this register shows the current, free-running value of Timer B in all modes. Software can use this value to determine the time elapsed between an interrupt and the ISR entry. When
///
/// written, the value written into this register is loaded into the TBR register on the next clock cycle.
///
///
///
/// When a 16/32-bit GPTM is configured to one of the 32-bit modes, the contents of bits 15:0 in this register are loaded into the upper 16 bits of the TAV register. Reads from this register return
///
/// the current free-running value of Timer B. In a 16-bit mode, bits 15:0 contain the value of the counter and bits 23:16 contain the current, free-running value of the prescaler, which is the upper 8 bits of
///
/// the count in Input Edge Count, Input Edge Time, PWM and one-shot or periodic up count modes.
///
/// In one-shot or periodic down count modes, the prescaler stored in 23:16 is a true prescaler, meaning bits 23:16 count down before decrementing the value in bits 15:0. The prescaler in bits 31:24 always reads as 0.
///
/// [TI-TRM-I] 13.5.1.21 TBV Register (Offset = 54h) [reset = FFFFh]
pub mod TBV;
/// Peripheral Version
///
/// This register provides information regarding the GPT version
///
/// [TI-TRM-I] 13.5.1.27 VERSION Register (Offset = FB0h) [reset = 400h]
pub mod VERSION;
