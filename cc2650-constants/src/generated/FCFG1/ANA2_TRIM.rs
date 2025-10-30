use cmemu_common::Address;

pub const DISPLAY: &str = "ANA2_TRIM";
pub const OFFSET: u32 = 0x2b4;
/// 0x500012b4
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
/// Internal. Only to be used through TI provided API.
pub mod RCOSCHFCTRIMFRACT_EN {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 31..=31;
    pub const BIT_MASK: u32 = 0x80000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod RCOSCHFCTRIMFRACT {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 26..=30;
    pub const BIT_MASK: u32 = 0x7c000000;
    pub const BIT_WIDTH: u8 = 5;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod SET_RCOSC_HF_FINE_RESISTOR {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 23..=24;
    pub const BIT_MASK: u32 = 0x01800000;
    pub const BIT_WIDTH: u8 = 2;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod ATESTLF_UDIGLDO_IBIAS_TRIM {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 22..=22;
    pub const BIT_MASK: u32 = 0x00400000;
    pub const BIT_WIDTH: u8 = 1;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod NANOAMP_RES_TRIM {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 16..=21;
    pub const BIT_MASK: u32 = 0x003f0000;
    pub const BIT_WIDTH: u8 = 6;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod DITHER_EN {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 11..=11;
    pub const BIT_MASK: u32 = 0x00000800;
    pub const BIT_WIDTH: u8 = 1;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod DCDC_IPEAK {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 8..=10;
    pub const BIT_MASK: u32 = 0x00000700;
    pub const BIT_WIDTH: u8 = 3;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod DEAD_TIME_TRIM {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 6..=7;
    pub const BIT_MASK: u32 = 0x000000c0;
    pub const BIT_WIDTH: u8 = 2;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod DCDC_LOW_EN_SEL {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 3..=5;
    pub const BIT_MASK: u32 = 0x00000038;
    pub const BIT_WIDTH: u8 = 3;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod DCDC_HIGH_EN_SEL {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 0..=2;
    pub const BIT_MASK: u32 = 0x00000007;
    pub const BIT_WIDTH: u8 = 3;
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
        pub DCDC_HIGH_EN_SEL: B3,
        pub DCDC_LOW_EN_SEL: B3,
        pub DEAD_TIME_TRIM: B2,
        pub DCDC_IPEAK: B3,
        pub DITHER_EN: B1,
        pub reserved_12_16: B4,
        pub NANOAMP_RES_TRIM: B6,
        pub ATESTLF_UDIGLDO_IBIAS_TRIM: B1,
        pub SET_RCOSC_HF_FINE_RESISTOR: B2,
        pub reserved_25_26: B1,
        pub RCOSCHFCTRIMFRACT: B5,
        pub RCOSCHFCTRIMFRACT_EN: B1,
    }

    impl HwRegister for Register {
        const RESERVED_BITS_MASK: u32 = 0x0200f000;
        const READ_ONLY_BITS_MASK: u32 = 0xfdff0fff;
        const WRITE_ONLY_BITS_MASK: u32 = 0x00000000;

        fn read(&self) -> u32 {
            u32::from(self.content)
        }

        fn mutate(&mut self, word: u32) {
            let old_val: u32 = self.read();
            let mut new_val: u32 = word;

            // Check if modifies reserved bits
            if old_val & Self::RESERVED_BITS_MASK != new_val & Self::RESERVED_BITS_MASK {
                warn!(target: "cc2650_constants::FCFG1::ANA2_TRIM", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::FCFG1::ANA2_TRIM",
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
