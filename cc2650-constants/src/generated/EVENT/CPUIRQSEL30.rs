use cmemu_common::Address;

pub const DISPLAY: &str = "CPUIRQSEL30";
pub const OFFSET: u32 = 0x78;
/// 0x40083078
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
pub const RESET_VALUE: u32 = 0x00000000;
pub const RESET_MASK: u32 = 0xffffffff;
/// Read/write selection value
///
///
///
/// Writing any other value than values defined by a ENUM may result in undefined behavior.
pub mod EV {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 0..=6;
    pub const BIT_MASK: u32 = 0x0000007f;
    pub const BIT_WIDTH: u8 = 7;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub const RESET_ENUM: self::Values = self::Values::NONE;
    pub use self::Values as V;
    use modular_bitfield::prelude::BitfieldSpecifier;
    use num_enum::IntoPrimitive;
    #[repr(u8)]
    #[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, BitfieldSpecifier)]
    #[bits = 7]
    #[allow(non_camel_case_types)]
    #[non_exhaustive]
    pub enum Values {
        /// Always asserted
        ALWAYS_ACTIVE = 121,
        /// RTC periodic event controlled by AON_RTC:CTL.RTC_UPD_EN
        AON_RTC_UPD = 119,
        /// Loopback of OBSMUX0 through AUX, corresponds to AUX_EVCTL:EVTOMCUFLAGS.OBSMUX0
        AUX_OBSMUX0 = 114,
        /// AUX ADC FIFO watermark event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.ADC_FIFO_ALMOST_FULL
        AUX_ADC_FIFO_ALMOST_FULL = 113,
        /// AUX ADC done, corresponds to AUX_EVCTL:EVTOMCUFLAGS.ADC_DONE
        AUX_ADC_DONE = 112,
        /// Autotake event from AUX semaphore, configured by AUX_SMPH:AUTOTAKE
        AUX_SMPH_AUTOTAKE_DONE = 111,
        /// AUX timer 1 event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.TIMER1_EV
        AUX_TIMER1_EV = 110,
        /// AUX timer 0 event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.TIMER0_EV
        AUX_TIMER0_EV = 109,
        /// AUX TDC measurement done event, corresponds to the flag AUX_EVCTL:EVTOMCUFLAGS.TDC_DONE and the AUX_TDC status AUX_TDC:STAT.DONE
        AUX_TDC_DONE = 108,
        /// AUX Compare B event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_COMPB
        AUX_COMPB = 107,
        /// AON wakeup event, corresponds flags are here AUX_EVCTL:EVTOMCUFLAGS.AON_WU_EV
        AUX_AON_WU_EV = 105,
        /// CRYPTO DMA input done event, the correspondingg flag is CRYPTO:IRQSTAT.DMA_IN_DONE. Controlled by CRYPTO:IRQEN.DMA_IN_DONE
        CRYPTO_DMA_DONE_IRQ = 94,
        /// DMA done for software tiggered UDMA channel 18, see UDMA0:SOFTREQ
        DMA_CH18_DONE = 22,
        /// DMA done for software tiggered UDMA channel 0, see UDMA0:SOFTREQ
        DMA_CH0_DONE = 20,
        /// AUX Software event 0, AUX_EVCTL:SWEVSET.SWEV0
        AON_AUX_SWEV0 = 10,
        /// Interrupt event from I2S
        I2S_IRQ = 8,
        /// AON programmable event 2. Event selected by AON_EVENT MCU event selector, AON_EVENT:EVTOMCUSEL.AON_PROG2_EV
        AON_PROG2 = 3,
        /// AON programmable event 1. Event selected by AON_EVENT MCU event selector, AON_EVENT:EVTOMCUSEL.AON_PROG1_EV
        AON_PROG1 = 2,
        /// Always inactive
        NONE = 0,
    }
    pub use self::Named as E;
    pub mod Named {
        /// Always asserted
        pub const ALWAYS_ACTIVE: u32 = 121;
        /// RTC periodic event controlled by AON_RTC:CTL.RTC_UPD_EN
        pub const AON_RTC_UPD: u32 = 119;
        /// Loopback of OBSMUX0 through AUX, corresponds to AUX_EVCTL:EVTOMCUFLAGS.OBSMUX0
        pub const AUX_OBSMUX0: u32 = 114;
        /// AUX ADC FIFO watermark event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.ADC_FIFO_ALMOST_FULL
        pub const AUX_ADC_FIFO_ALMOST_FULL: u32 = 113;
        /// AUX ADC done, corresponds to AUX_EVCTL:EVTOMCUFLAGS.ADC_DONE
        pub const AUX_ADC_DONE: u32 = 112;
        /// Autotake event from AUX semaphore, configured by AUX_SMPH:AUTOTAKE
        pub const AUX_SMPH_AUTOTAKE_DONE: u32 = 111;
        /// AUX timer 1 event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.TIMER1_EV
        pub const AUX_TIMER1_EV: u32 = 110;
        /// AUX timer 0 event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.TIMER0_EV
        pub const AUX_TIMER0_EV: u32 = 109;
        /// AUX TDC measurement done event, corresponds to the flag AUX_EVCTL:EVTOMCUFLAGS.TDC_DONE and the AUX_TDC status AUX_TDC:STAT.DONE
        pub const AUX_TDC_DONE: u32 = 108;
        /// AUX Compare B event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_COMPB
        pub const AUX_COMPB: u32 = 107;
        /// AON wakeup event, corresponds flags are here AUX_EVCTL:EVTOMCUFLAGS.AON_WU_EV
        pub const AUX_AON_WU_EV: u32 = 105;
        /// CRYPTO DMA input done event, the correspondingg flag is CRYPTO:IRQSTAT.DMA_IN_DONE. Controlled by CRYPTO:IRQEN.DMA_IN_DONE
        pub const CRYPTO_DMA_DONE_IRQ: u32 = 94;
        /// DMA done for software tiggered UDMA channel 18, see UDMA0:SOFTREQ
        pub const DMA_CH18_DONE: u32 = 22;
        /// DMA done for software tiggered UDMA channel 0, see UDMA0:SOFTREQ
        pub const DMA_CH0_DONE: u32 = 20;
        /// AUX Software event 0, AUX_EVCTL:SWEVSET.SWEV0
        pub const AON_AUX_SWEV0: u32 = 10;
        /// Interrupt event from I2S
        pub const I2S_IRQ: u32 = 8;
        /// AON programmable event 2. Event selected by AON_EVENT MCU event selector, AON_EVENT:EVTOMCUSEL.AON_PROG2_EV
        pub const AON_PROG2: u32 = 3;
        /// AON programmable event 1. Event selected by AON_EVENT MCU event selector, AON_EVENT:EVTOMCUSEL.AON_PROG1_EV
        pub const AON_PROG1: u32 = 2;
        /// Always inactive
        pub const NONE: u32 = 0;
    }
}

pub use HwRegisterImpl::Register;

pub mod HwRegisterImpl {
    #![allow(
        clippy::cast_lossless,
        clippy::identity_op,
        clippy::must_use_candidate,
        clippy::new_without_default,
        clippy::no_effect,
        clippy::no_effect_underscore_binding,
        clippy::return_self_not_must_use,
        unused_braces
    )]
    use cmemu_common::HwRegister;
    use log::warn;
    use modular_bitfield::prelude::*;

    #[derive(Clone, Copy, Debug)]
    pub struct Register {
        content: Bitfields,
    }

    #[repr(u32)]
    #[bitfield]
    #[derive(Clone, Copy, Debug)]
    pub struct Bitfields {
        pub EV: super::EV::V,
        pub reserved_7_32: B25,
    }

    impl HwRegister for Register {
        const RESERVED_BITS_MASK: u32 = 0xffffff80;
        const READ_ONLY_BITS_MASK: u32 = 0x00000000;
        const WRITE_ONLY_BITS_MASK: u32 = 0x00000000;

        fn read(&self) -> u32 {
            u32::from(self.content)
        }

        fn mutate(&mut self, word: u32) {
            let old_val: u32 = self.read();
            let mut new_val: u32 = word;

            // Check if modifies reserved bits
            if old_val & Self::RESERVED_BITS_MASK != new_val & Self::RESERVED_BITS_MASK {
                warn!(target: "cc2650_constants::EVENT::CPUIRQSEL30", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::EVENT::CPUIRQSEL30",
                    "Changing read only bits of {}, write to read only bits is ignored",
                    super::DISPLAY
                );
                // replace read only bits in `val` with original value in `self.0`
                new_val =
                    (new_val & !Self::READ_ONLY_BITS_MASK) | (old_val & Self::READ_ONLY_BITS_MASK);
            }
            self.content = Bitfields::from(new_val);
        }
    }

    impl Register {
        pub fn new() -> Self {
            Self {
                content: Bitfields::from(super::RESET_VALUE),
            }
        }

        pub fn bitfields(self) -> Bitfields {
            self.content
        }

        pub fn mut_bitfields(&mut self) -> &mut Bitfields {
            &mut self.content
        }

        pub fn mutate_copy(&self, mutator: fn(Bitfields) -> Bitfields) -> Self {
            Self {
                content: mutator(self.content),
            }
        }
    }

    impl From<u32> for Register {
        fn from(item: u32) -> Self {
            Self {
                content: Bitfields::from(item),
            }
        }
    }

    impl From<Register> for u32 {
        fn from(item: Register) -> Self {
            Self::from(item.content)
        }
    }
}
