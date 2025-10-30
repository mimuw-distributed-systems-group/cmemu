use cmemu_common::Address;

pub const DISPLAY: &str = "SHDW_OSC_BIAS_LDO_TRIM";
pub const OFFSET: u32 = 0x138;
/// 0x50001138
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
/// Internal. Only to be used through TI provided API.
pub mod SET_RCOSC_HF_COARSE_RESISTOR {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 27..=28;
    pub const BIT_MASK: u32 = 0x18000000;
    pub const BIT_WIDTH: u8 = 2;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod TRIMMAG {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 23..=26;
    pub const BIT_MASK: u32 = 0x07800000;
    pub const BIT_WIDTH: u8 = 4;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod TRIMIREF {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 18..=22;
    pub const BIT_MASK: u32 = 0x007c0000;
    pub const BIT_WIDTH: u8 = 5;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod ITRIM_DIG_LDO {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 16..=17;
    pub const BIT_MASK: u32 = 0x00030000;
    pub const BIT_WIDTH: u8 = 2;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod VTRIM_DIG {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 12..=15;
    pub const BIT_MASK: u32 = 0x0000f000;
    pub const BIT_WIDTH: u8 = 4;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod VTRIM_COARSE {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 8..=11;
    pub const BIT_MASK: u32 = 0x00000f00;
    pub const BIT_WIDTH: u8 = 4;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod RCOSCHF_CTRIM {
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
        pub RCOSCHF_CTRIM: B8,
        pub VTRIM_COARSE: B4,
        pub VTRIM_DIG: B4,
        pub ITRIM_DIG_LDO: B2,
        pub TRIMIREF: B5,
        pub TRIMMAG: B4,
        pub SET_RCOSC_HF_COARSE_RESISTOR: B2,
        pub reserved_29_32: B3,
    }

    impl HwRegister for Register {
        const RESERVED_BITS_MASK: u32 = 0xe0000000;
        const READ_ONLY_BITS_MASK: u32 = 0x1fffffff;
        const WRITE_ONLY_BITS_MASK: u32 = 0x00000000;

        fn read(&self) -> u32 {
            u32::from(self.content)
        }

        fn mutate(&mut self, word: u32) {
            let old_val: u32 = self.read();
            let mut new_val: u32 = word;

            // Check if modifies reserved bits
            if old_val & Self::RESERVED_BITS_MASK != new_val & Self::RESERVED_BITS_MASK {
                warn!(target: "cc2650_constants::FCFG1::SHDW_OSC_BIAS_LDO_TRIM", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::FCFG1::SHDW_OSC_BIAS_LDO_TRIM",
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
