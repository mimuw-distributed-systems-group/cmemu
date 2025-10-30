use cmemu_common::Address;

pub const DISPLAY: &str = "CTL";
pub const OFFSET: u32 = 0x0;
/// 0x40092000
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
pub const RESET_VALUE: u32 = 0x00000000;
pub const RESET_MASK: u32 = 0xffffffff;
/// Eventmask selecting which delayed events that form the combined event.
pub mod COMB_EV_MASK {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 16..=18;
    pub const BIT_MASK: u32 = 0x00070000;
    pub const BIT_WIDTH: u8 = 3;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// Use Channel 2 delayed event in combined event
        pub const CH2: u32 = 4;
        /// Use Channel 1 delayed event in combined event
        pub const CH1: u32 = 2;
        /// Use Channel 0 delayed event in combined event
        pub const CH0: u32 = 1;
        /// No event is selected for combined event.
        pub const NONE: u32 = 0;
    }
}
/// Number of SCLK_LF clock cycles waited before generating delayed events. (Common setting for all RTC cannels)  the delayed event is delayed
pub mod EV_DELAY {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 8..=11;
    pub const BIT_MASK: u32 = 0x00000f00;
    pub const BIT_WIDTH: u8 = 4;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// Delay by 144 clock cycles
        pub const D144: u32 = 13;
        /// Delay by 128 clock cycles
        pub const D128: u32 = 12;
        /// Delay by 112 clock cycles
        pub const D112: u32 = 11;
        /// Delay by 96 clock cycles
        pub const D96: u32 = 10;
        /// Delay by 80 clock cycles
        pub const D80: u32 = 9;
        /// Delay by 64 clock cycles
        pub const D64: u32 = 8;
        /// Delay by 48 clock cycles
        pub const D48: u32 = 7;
        /// Delay by 32 clock cycles
        pub const D32: u32 = 6;
        /// Delay by 16 clock cycles
        pub const D16: u32 = 5;
        /// Delay by 8 clock cycles
        pub const D8: u32 = 4;
        /// Delay by 4 clock cycles
        pub const D4: u32 = 3;
        /// Delay by 2 clock cycles
        pub const D2: u32 = 2;
        /// Delay by 1 clock cycles
        pub const D1: u32 = 1;
        /// No delay on delayed event
        pub const D0: u32 = 0;
    }
}
/// RTC Counter reset.
///
///
///
/// Writing 1 to this bit will reset the RTC counter.
///
///
///
/// This bit is cleared when reset takes effect
pub mod RESET {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 7..=7;
    pub const BIT_MASK: u32 = 0x00000080;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// RTC_4KHZ is a 4 KHz reference output, tapped from  SUBSEC.VALUE  bit 19 which is used by AUX timer.
///
///
///
/// 0: RTC_4KHZ signal is forced to 0
///
/// 1: RTC_4KHZ is enabled ( provied that RTC is enabled EN)
pub mod RTC_4KHZ_EN {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 2..=2;
    pub const BIT_MASK: u32 = 0x00000004;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// RTC_UPD is a 16 KHz signal used to sync up the radio timer. The 16 Khz is SCLK_LF divided by 2
///
///
///
/// 0: RTC_UPD signal is forced to 0
///
/// 1: RTC_UPD signal is toggling @16 kHz
pub mod RTC_UPD_EN {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 1..=1;
    pub const BIT_MASK: u32 = 0x00000002;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Enable RTC counter
///
///
///
/// 0: Halted (frozen)
///
/// 1: Running
pub mod EN {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 0..=0;
    pub const BIT_MASK: u32 = 0x00000001;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
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
        pub EN: B1,
        pub RTC_UPD_EN: B1,
        pub RTC_4KHZ_EN: B1,
        pub reserved_3_7: B4,
        pub RESET: B1,
        pub EV_DELAY: B4,
        pub reserved_12_16: B4,
        pub COMB_EV_MASK: B3,
        pub reserved_19_32: B13,
    }

    impl HwRegister for Register {
        const RESERVED_BITS_MASK: u32 = 0xfff8f078;
        const READ_ONLY_BITS_MASK: u32 = 0x00000000;
        const WRITE_ONLY_BITS_MASK: u32 = 0x00000080;

        fn read(&self) -> u32 {
            u32::from(self.content)
        }

        fn mutate(&mut self, word: u32) {
            let old_val: u32 = self.read();
            let mut new_val: u32 = word;

            // Check if modifies reserved bits
            if old_val & Self::RESERVED_BITS_MASK != new_val & Self::RESERVED_BITS_MASK {
                warn!(target: "cc2650_constants::AON_RTC::CTL", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::AON_RTC::CTL",
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
