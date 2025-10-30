use cmemu_common::Address;

pub const DISPLAY: &str = "MISC_OTP_DATA";
pub const OFFSET: u32 = 0x320;
/// 0x50001320
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
/// Internal. Only to be used through TI provided API.
pub mod RCOSC_HF_ITUNE {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 28..=31;
    pub const BIT_MASK: u32 = 0xf0000000;
    pub const BIT_WIDTH: u8 = 4;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod RCOSC_HF_CRIM {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 20..=27;
    pub const BIT_MASK: u32 = 0x0ff00000;
    pub const BIT_WIDTH: u8 = 8;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod PER_M {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 15..=19;
    pub const BIT_MASK: u32 = 0x000f8000;
    pub const BIT_WIDTH: u8 = 5;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod PER_E {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 12..=14;
    pub const BIT_MASK: u32 = 0x00007000;
    pub const BIT_WIDTH: u8 = 3;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod PO_TAIL_RES_TRIM {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 8..=11;
    pub const BIT_MASK: u32 = 0x00000f00;
    pub const BIT_WIDTH: u8 = 4;
    pub const WRITABLE: bool = false;
}
/// The revision of the test program used in the production process when FCFG1 was programmed.
///
/// Value migth change without warning.
pub mod TEST_PROGRAM_REV {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 0..=7;
    pub const BIT_MASK: u32 = 0x000000ff;
    pub const BIT_WIDTH: u8 = 8;
    pub const WRITABLE: bool = false;
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
        pub TEST_PROGRAM_REV: B8,
        pub PO_TAIL_RES_TRIM: B4,
        pub PER_E: B3,
        pub PER_M: B5,
        pub RCOSC_HF_CRIM: B8,
        pub RCOSC_HF_ITUNE: B4,
    }

    impl HwRegister for Register {
        const RESERVED_BITS_MASK: u32 = 0x00000000;
        const READ_ONLY_BITS_MASK: u32 = 0xffffffff;
        const WRITE_ONLY_BITS_MASK: u32 = 0x00000000;

        fn read(&self) -> u32 {
            u32::from(self.content)
        }

        fn mutate(&mut self, word: u32) {
            let old_val: u32 = self.read();
            let mut new_val: u32 = word;

            // Check if modifies reserved bits
            if old_val & Self::RESERVED_BITS_MASK != new_val & Self::RESERVED_BITS_MASK {
                warn!(target: "cc2650_constants::FCFG1::MISC_OTP_DATA", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::FCFG1::MISC_OTP_DATA",
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
