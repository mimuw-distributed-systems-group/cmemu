use cmemu_common::Address;

pub const DISPLAY: &str = "T1CFG";
pub const OFFSET: u32 = 0x4;
/// 0x400c7004
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
pub const RESET_VALUE: u32 = 0x00000000;
pub const RESET_MASK: u32 = 0xffffffff;
/// Tick source polarity for Timer 1.
pub mod TICK_SRC_POL {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 13..=13;
    pub const BIT_MASK: u32 = 0x00002000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// Count on falling edges of TICK_SRC.
        pub const FALL: u32 = 1;
        /// Count on rising edges of TICK_SRC.
        pub const RISE: u32 = 0;
    }
}
/// Select Timer 1 tick source from the synchronous event bus.
pub mod TICK_SRC {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 8..=12;
    pub const BIT_MASK: u32 = 0x00001f00;
    pub const BIT_WIDTH: u8 = 5;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// AUX_EVCTL:EVSTAT1.ADC_IRQ
        pub const ADC_IRQ: u32 = 31;
        /// AUX_EVCTL:EVSTAT1.MCU_EV
        pub const MCU_EVENT: u32 = 30;
        /// AUX_EVCTL:EVSTAT1.ACLK_REF
        pub const ACLK_REF: u32 = 29;
        /// AUX_EVCTL:EVSTAT1.AUXIO15
        pub const AUXIO15: u32 = 28;
        /// AUX_EVCTL:EVSTAT1.AUXIO14
        pub const AUXIO14: u32 = 27;
        /// AUX_EVCTL:EVSTAT1.AUXIO13
        pub const AUXIO13: u32 = 26;
        /// AUX_EVCTL:EVSTAT1.AUXIO12
        pub const AUXIO12: u32 = 25;
        /// AUX_EVCTL:EVSTAT1.AUXIO11
        pub const AUXIO11: u32 = 24;
        /// AUX_EVCTL:EVSTAT1.AUXIO10
        pub const AUXIO10: u32 = 23;
        /// AUX_EVCTL:EVSTAT1.AUXIO9
        pub const AUXIO9: u32 = 22;
        /// AUX_EVCTL:EVSTAT1.AUXIO8
        pub const AUXIO8: u32 = 21;
        /// AUX_EVCTL:EVSTAT1.AUXIO7
        pub const AUXIO7: u32 = 20;
        /// AUX_EVCTL:EVSTAT1.AUXIO6
        pub const AUXIO6: u32 = 19;
        /// AUX_EVCTL:EVSTAT1.AUXIO5
        pub const AUXIO5: u32 = 18;
        /// AUX_EVCTL:EVSTAT1.AUXIO4
        pub const AUXIO4: u32 = 17;
        /// AUX_EVCTL:EVSTAT1.AUXIO3
        pub const AUXIO3: u32 = 16;
        /// AUX_EVCTL:EVSTAT0.AUXIO2
        pub const AUXIO2: u32 = 15;
        /// AUX_EVCTL:EVSTAT0.AUXIO1
        pub const AUXIO1: u32 = 14;
        /// AUX_EVCTL:EVSTAT0.AUXIO0
        pub const AUXIO0: u32 = 13;
        /// AUX_EVCTL:EVSTAT0.AON_PROG_WU
        pub const AON_PROG_WU: u32 = 12;
        /// AUX_EVCTL:EVSTAT0.AON_SW
        pub const AON_SW: u32 = 11;
        /// AUX_EVCTL:EVSTAT0.OBSMUX1
        pub const OBSMUX1: u32 = 10;
        /// AUX_EVCTL:EVSTAT0.OBSMUX0
        pub const OBSMUX0: u32 = 9;
        /// AON_RTC:SUBSEC.VALUE bit 19. AON_RTC:CTL.RTC_4KHZ_EN enables this event.
        pub const RTC_4KHZ: u32 = 8;
        /// AUX_EVCTL:EVSTAT0.ADC_DONE
        pub const ADC_DONE: u32 = 7;
        /// AUX_EVCTL:EVSTAT0.SMPH_AUTOTAKE_DONE
        pub const SMPH_AUTOTAKE_DONE: u32 = 6;
        /// AUX_EVCTL:EVSTAT0.TIMER0_EV
        pub const TIMER0_EV: u32 = 4;
        /// AUX_EVCTL:EVSTAT0.TDC_DONE
        pub const TDC_DONE: u32 = 3;
        /// AUX_EVCTL:EVSTAT0.AUX_COMPB
        pub const AUX_COMPB: u32 = 2;
        /// AUX_EVCTL:EVSTAT0.AUX_COMPA
        pub const AUX_COMPA: u32 = 1;
        /// AUX_EVCTL:EVSTAT0.AON_RTC_CH2
        pub const RTC_CH2_EV: u32 = 0;
    }
}
/// Prescaler division ratio is 2^PRE:
///
///
///
/// 0x0: Divide by 1.
///
/// 0x1: Divide by 2.
///
/// 0x2: Divide by 4.
///
/// ...
///
/// 0xF: Divide by 32,768.
pub mod PRE {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 4..=7;
    pub const BIT_MASK: u32 = 0x000000f0;
    pub const BIT_WIDTH: u8 = 4;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Timer 1 mode.
///
///
///
/// Configure source for Timer 1 prescaler.
pub mod MODE {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 1..=1;
    pub const BIT_MASK: u32 = 0x00000002;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// Use event set by TICK_SRC as source for prescaler.
        pub const TICK: u32 = 1;
        /// Use AUX clock as source for prescaler.
        pub const CLK: u32 = 0;
    }
}
/// Timer 1 reload mode.
pub mod RELOAD {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 0..=0;
    pub const BIT_MASK: u32 = 0x00000001;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// Continuous mode.
        ///
        ///
        ///
        /// Timer 1 restarts when the counter value becomes equal to or greater than ( T1TARGET.VALUE - 1).
        pub const CONT: u32 = 1;
        /// Manual mode.
        ///
        ///
        ///
        /// Timer 1 stops and T1CTL.EN becomes 0 when the counter value becomes equal to or greater than T1TARGET.VALUE.
        pub const MAN: u32 = 0;
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
        pub RELOAD: B1,
        pub MODE: B1,
        pub reserved_2_4: B2,
        pub PRE: B4,
        pub TICK_SRC: B5,
        pub TICK_SRC_POL: B1,
        pub reserved_14_32: B18,
    }

    impl HwRegister for Register {
        const RESERVED_BITS_MASK: u32 = 0xffffc00c;
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
                warn!(target: "cc2650_constants::AUX_TIMER::T1CFG", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::AUX_TIMER::T1CFG",
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
