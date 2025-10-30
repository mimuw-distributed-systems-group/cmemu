use cmemu_common::Address;

pub const DISPLAY: &str = "FBFALLBACK";
pub const OFFSET: u32 = 0x2040;
/// 0x40032040
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
pub const RESET_VALUE: u32 = 0x0505ffff;
pub const RESET_MASK: u32 = 0xffffffff;
/// Internal. Only to be used through TI provided API.
pub mod FSM_PWRSAV {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 24..=27;
    pub const BIT_MASK: u32 = 0x0f000000;
    pub const BIT_WIDTH: u8 = 4;
    pub const RESET_VALUE: u32 = 0x5;
    pub const WRITABLE: bool = true;
}
/// Internal. Only to be used through TI provided API.
pub mod REG_PWRSAV {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 16..=19;
    pub const BIT_MASK: u32 = 0x000f0000;
    pub const BIT_WIDTH: u8 = 4;
    pub const RESET_VALUE: u32 = 0x5;
    pub const WRITABLE: bool = true;
}
/// Internal. Only to be used through TI provided API.
pub mod BANKPWR7 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 14..=15;
    pub const BIT_MASK: u32 = 0x0000c000;
    pub const BIT_WIDTH: u8 = 2;
    pub const RESET_VALUE: u32 = 0x3;
    pub const WRITABLE: bool = true;
}
/// Internal. Only to be used through TI provided API.
pub mod BANKPWR6 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 12..=13;
    pub const BIT_MASK: u32 = 0x00003000;
    pub const BIT_WIDTH: u8 = 2;
    pub const RESET_VALUE: u32 = 0x3;
    pub const WRITABLE: bool = true;
}
/// Internal. Only to be used through TI provided API.
pub mod BANKPWR5 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 10..=11;
    pub const BIT_MASK: u32 = 0x00000c00;
    pub const BIT_WIDTH: u8 = 2;
    pub const RESET_VALUE: u32 = 0x3;
    pub const WRITABLE: bool = true;
}
/// Internal. Only to be used through TI provided API.
pub mod BANKPWR4 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 8..=9;
    pub const BIT_MASK: u32 = 0x00000300;
    pub const BIT_WIDTH: u8 = 2;
    pub const RESET_VALUE: u32 = 0x3;
    pub const WRITABLE: bool = true;
}
/// Internal. Only to be used through TI provided API.
pub mod BANKPWR3 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 6..=7;
    pub const BIT_MASK: u32 = 0x000000c0;
    pub const BIT_WIDTH: u8 = 2;
    pub const RESET_VALUE: u32 = 0x3;
    pub const WRITABLE: bool = true;
}
/// Internal. Only to be used through TI provided API.
pub mod BANKPWR2 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 4..=5;
    pub const BIT_MASK: u32 = 0x00000030;
    pub const BIT_WIDTH: u8 = 2;
    pub const RESET_VALUE: u32 = 0x3;
    pub const WRITABLE: bool = true;
}
/// Internal. Only to be used through TI provided API.
pub mod BANKPWR1 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 2..=3;
    pub const BIT_MASK: u32 = 0x0000000c;
    pub const BIT_WIDTH: u8 = 2;
    pub const RESET_VALUE: u32 = 0x3;
    pub const WRITABLE: bool = true;
}
/// Internal. Only to be used through TI provided API.
pub mod BANKPWR0 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 0..=1;
    pub const BIT_MASK: u32 = 0x00000003;
    pub const BIT_WIDTH: u8 = 2;
    pub const RESET_VALUE: u32 = 0x3;
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
        pub BANKPWR0: B2,
        pub BANKPWR1: B2,
        pub BANKPWR2: B2,
        pub BANKPWR3: B2,
        pub BANKPWR4: B2,
        pub BANKPWR5: B2,
        pub BANKPWR6: B2,
        pub BANKPWR7: B2,
        pub REG_PWRSAV: B4,
        pub reserved_20_24: B4,
        pub FSM_PWRSAV: B4,
        pub reserved_28_32: B4,
    }

    impl HwRegister for Register {
        const RESERVED_BITS_MASK: u32 = 0xf0f00000;
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
                warn!(target: "cc2650_constants::FLASH::FBFALLBACK", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::FLASH::FBFALLBACK",
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
