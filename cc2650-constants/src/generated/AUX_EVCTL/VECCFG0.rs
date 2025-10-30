use cmemu_common::Address;

pub const DISPLAY: &str = "VECCFG0";
pub const OFFSET: u32 = 0x0;
/// 0x400c5000
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
pub const RESET_VALUE: u32 = 0x00000000;
pub const RESET_MASK: u32 = 0xffffffff;
/// Vector 1 trigger event polarity.
///
///
///
/// To manually trigger vector 1 execution:
///
/// - AUX_SCE must sleep.
///
/// - Set VEC1_EV to a known static value.
///
/// - Toggle VEC1_POL twice.
pub mod VEC1_POL {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 14..=14;
    pub const BIT_MASK: u32 = 0x00004000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// Falling edge triggers vector 1 execution.
        pub const FALL: u32 = 1;
        /// Rising edge triggers vector 1 execution.
        pub const RISE: u32 = 0;
    }
}
/// Vector 1 trigger enable.
///
///
///
/// When enabled, VEC1_EV event with VEC1_POL polarity triggers a jump to vector # 1 when AUX_SCE sleeps.
///
///
///
/// Lower vectors (0) have priority.
pub mod VEC1_EN {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 13..=13;
    pub const BIT_MASK: u32 = 0x00002000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// Enable vector 1 trigger.
        pub const EN: u32 = 1;
        /// Disable vector 1 trigger.
        pub const DIS: u32 = 0;
    }
}
/// Select vector 1 trigger source event.
pub mod VEC1_EV {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 8..=12;
    pub const BIT_MASK: u32 = 0x00001f00;
    pub const BIT_WIDTH: u8 = 5;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// EVSTAT1.ADC_IRQ
        pub const ADC_IRQ: u32 = 31;
        /// EVSTAT1.MCU_EV
        pub const MCU_EV: u32 = 30;
        /// EVSTAT1.ACLK_REF
        pub const ACLK_REF: u32 = 29;
        /// EVSTAT1.AUXIO15
        pub const AUXIO15: u32 = 28;
        /// EVSTAT1.AUXIO14
        pub const AUXIO14: u32 = 27;
        /// EVSTAT1.AUXIO13
        pub const AUXIO13: u32 = 26;
        /// EVSTAT1.AUXIO12
        pub const AUXIO12: u32 = 25;
        /// EVSTAT1.AUXIO11
        pub const AUXIO11: u32 = 24;
        /// EVSTAT1.AUXIO10
        pub const AUXIO10: u32 = 23;
        /// EVSTAT1.AUXIO9
        pub const AUXIO9: u32 = 22;
        /// EVSTAT1.AUXIO8
        pub const AUXIO8: u32 = 21;
        /// EVSTAT1.AUXIO7
        pub const AUXIO7: u32 = 20;
        /// EVSTAT1.AUXIO6
        pub const AUXIO6: u32 = 19;
        /// EVSTAT1.AUXIO5
        pub const AUXIO5: u32 = 18;
        /// EVSTAT1.AUXIO4
        pub const AUXIO4: u32 = 17;
        /// EVSTAT1.AUXIO3
        pub const AUXIO3: u32 = 16;
        /// EVSTAT0.AUXIO2
        pub const AUXIO2: u32 = 15;
        /// EVSTAT0.AUXIO1
        pub const AUXIO1: u32 = 14;
        /// EVSTAT0.AUXIO0
        pub const AUXIO0: u32 = 13;
        /// EVSTAT0.AON_PROG_WU
        pub const AON_PROG_WU: u32 = 12;
        /// EVSTAT0.AON_SW
        pub const AON_SW: u32 = 11;
        /// EVSTAT0.OBSMUX1
        pub const OBSMUX1: u32 = 10;
        /// EVSTAT0.OBSMUX0
        pub const OBSMUX0: u32 = 9;
        /// EVSTAT0.ADC_FIFO_ALMOST_FULL
        pub const ADC_FIFO_ALMOST_FULL: u32 = 8;
        /// EVSTAT0.ADC_DONE
        pub const ADC_DONE: u32 = 7;
        /// EVSTAT0.SMPH_AUTOTAKE_DONE
        pub const SMPH_AUTOTAKE_DONE: u32 = 6;
        /// EVSTAT0.TIMER1_EV
        pub const TIMER1_EV: u32 = 5;
        /// EVSTAT0.TIMER0_EV
        pub const TIMER0_EV: u32 = 4;
        /// EVSTAT0.TDC_DONE
        pub const TDC_DONE: u32 = 3;
        /// EVSTAT0.AUX_COMPB
        pub const AUX_COMPB: u32 = 2;
        /// EVSTAT0.AUX_COMPA
        pub const AUX_COMPA: u32 = 1;
        /// EVSTAT0.AON_RTC_CH2
        pub const AON_RTC_CH2: u32 = 0;
    }
}
/// Vector 0 trigger event polarity.
///
///
///
/// To manually trigger vector 0 execution:
///
/// - AUX_SCE must sleep.
///
/// - Set VEC0_EV to a known static value.
///
/// - Toggle VEC0_POL twice.
pub mod VEC0_POL {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 6..=6;
    pub const BIT_MASK: u32 = 0x00000040;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// Falling edge triggers vector 0 execution.
        pub const FALL: u32 = 1;
        /// Rising edge triggers vector 0 execution.
        pub const RISE: u32 = 0;
    }
}
/// Vector 0 trigger enable.
///
///
///
/// When enabled, VEC0_EV event with VEC0_POL polarity triggers a jump to vector # 0 when AUX_SCE sleeps.
pub mod VEC0_EN {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 5..=5;
    pub const BIT_MASK: u32 = 0x00000020;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// Enable vector 0 trigger.
        pub const EN: u32 = 1;
        /// Disable vector 0 trigger.
        pub const DIS: u32 = 0;
    }
}
/// Select vector 0 trigger source event.
pub mod VEC0_EV {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 0..=4;
    pub const BIT_MASK: u32 = 0x0000001f;
    pub const BIT_WIDTH: u8 = 5;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// EVSTAT1.ADC_IRQ
        pub const ADC_IRQ: u32 = 31;
        /// EVSTAT1.MCU_EV
        pub const MCU_EV: u32 = 30;
        /// EVSTAT1.ACLK_REF
        pub const ACLK_REF: u32 = 29;
        /// EVSTAT1.AUXIO15
        pub const AUXIO15: u32 = 28;
        /// EVSTAT1.AUXIO14
        pub const AUXIO14: u32 = 27;
        /// EVSTAT1.AUXIO13
        pub const AUXIO13: u32 = 26;
        /// EVSTAT1.AUXIO12
        pub const AUXIO12: u32 = 25;
        /// EVSTAT1.AUXIO11
        pub const AUXIO11: u32 = 24;
        /// EVSTAT1.AUXIO10
        pub const AUXIO10: u32 = 23;
        /// EVSTAT1.AUXIO9
        pub const AUXIO9: u32 = 22;
        /// EVSTAT1.AUXIO8
        pub const AUXIO8: u32 = 21;
        /// EVSTAT1.AUXIO7
        pub const AUXIO7: u32 = 20;
        /// EVSTAT1.AUXIO6
        pub const AUXIO6: u32 = 19;
        /// EVSTAT1.AUXIO5
        pub const AUXIO5: u32 = 18;
        /// EVSTAT1.AUXIO4
        pub const AUXIO4: u32 = 17;
        /// EVSTAT1.AUXIO3
        pub const AUXIO3: u32 = 16;
        /// EVSTAT0.AUXIO2
        pub const AUXIO2: u32 = 15;
        /// EVSTAT0.AUXIO1
        pub const AUXIO1: u32 = 14;
        /// EVSTAT0.AUXIO0
        pub const AUXIO0: u32 = 13;
        /// EVSTAT0.AON_PROG_WU
        pub const AON_PROG_WU: u32 = 12;
        /// EVSTAT0.AON_SW
        pub const AON_SW: u32 = 11;
        /// EVSTAT0.OBSMUX1
        pub const OBSMUX1: u32 = 10;
        /// EVSTAT0.OBSMUX0
        pub const OBSMUX0: u32 = 9;
        /// EVSTAT0.ADC_FIFO_ALMOST_FULL
        pub const ADC_FIFO_ALMOST_FULL: u32 = 8;
        /// EVSTAT0.ADC_DONE
        pub const ADC_DONE: u32 = 7;
        /// EVSTAT0.SMPH_AUTOTAKE_DONE
        pub const SMPH_AUTOTAKE_DONE: u32 = 6;
        /// EVSTAT0.TIMER1_EV
        pub const TIMER1_EV: u32 = 5;
        /// EVSTAT0.TIMER0_EV
        pub const TIMER0_EV: u32 = 4;
        /// EVSTAT0.TDC_DONE
        pub const TDC_DONE: u32 = 3;
        /// EVSTAT0.AUX_COMPB
        pub const AUX_COMPB: u32 = 2;
        /// EVSTAT0.AUX_COMPA
        pub const AUX_COMPA: u32 = 1;
        /// EVSTAT0.AON_RTC_CH2
        pub const AON_RTC_CH2: u32 = 0;
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
        pub VEC0_EV: B5,
        pub VEC0_EN: B1,
        pub VEC0_POL: B1,
        pub reserved_7_8: B1,
        pub VEC1_EV: B5,
        pub VEC1_EN: B1,
        pub VEC1_POL: B1,
        pub reserved_15_32: B17,
    }

    impl HwRegister for Register {
        const RESERVED_BITS_MASK: u32 = 0xffff8080;
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
                warn!(target: "cc2650_constants::AUX_EVCTL::VECCFG0", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::AUX_EVCTL::VECCFG0",
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
