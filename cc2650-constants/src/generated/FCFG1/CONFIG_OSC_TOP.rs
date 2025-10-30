use cmemu_common::Address;

pub const DISPLAY: &str = "CONFIG_OSC_TOP";
pub const OFFSET: u32 = 0x350;
/// 0x50001350
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
/// Internal. Only to be used through TI provided API.
pub mod XOSC_HF_ROW_Q12 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 26..=29;
    pub const BIT_MASK: u32 = 0x3c000000;
    pub const BIT_WIDTH: u8 = 4;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod XOSC_HF_COLUMN_Q12 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 10..=25;
    pub const BIT_MASK: u32 = 0x03fffc00;
    pub const BIT_WIDTH: u8 = 16;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod RCOSCLF_CTUNE_TRIM {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 2..=9;
    pub const BIT_MASK: u32 = 0x000003fc;
    pub const BIT_WIDTH: u8 = 8;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod RCOSCLF_RTUNE_TRIM {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 0..=1;
    pub const BIT_MASK: u32 = 0x00000003;
    pub const BIT_WIDTH: u8 = 2;
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
        pub RCOSCLF_RTUNE_TRIM: B2,
        pub RCOSCLF_CTUNE_TRIM: B8,
        pub XOSC_HF_COLUMN_Q12: B16,
        pub XOSC_HF_ROW_Q12: B4,
        pub reserved_30_32: B2,
    }

    impl HwRegister for Register {
        const RESERVED_BITS_MASK: u32 = 0xc0000000;
        const READ_ONLY_BITS_MASK: u32 = 0x3fffffff;
        const WRITE_ONLY_BITS_MASK: u32 = 0x00000000;

        fn read(&self) -> u32 {
            u32::from(self.content)
        }

        fn mutate(&mut self, word: u32) {
            let old_val: u32 = self.read();
            let mut new_val: u32 = word;

            // Check if modifies reserved bits
            if old_val & Self::RESERVED_BITS_MASK != new_val & Self::RESERVED_BITS_MASK {
                warn!(target: "cc2650_constants::FCFG1::CONFIG_OSC_TOP", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::FCFG1::CONFIG_OSC_TOP",
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
