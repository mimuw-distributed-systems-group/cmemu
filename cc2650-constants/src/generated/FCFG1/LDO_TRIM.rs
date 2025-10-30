use cmemu_common::Address;

pub const DISPLAY: &str = "LDO_TRIM";
pub const OFFSET: u32 = 0x2b8;
/// 0x500012b8
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
/// Internal. Only to be used through TI provided API.
pub mod VDDR_TRIM_SLEEP {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 24..=28;
    pub const BIT_MASK: u32 = 0x1f000000;
    pub const BIT_WIDTH: u8 = 5;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod GLDO_CURSRC {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 16..=18;
    pub const BIT_MASK: u32 = 0x00070000;
    pub const BIT_WIDTH: u8 = 3;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod ITRIM_DIGLDO_LOAD {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 11..=12;
    pub const BIT_MASK: u32 = 0x00001800;
    pub const BIT_WIDTH: u8 = 2;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod ITRIM_UDIGLDO {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 8..=10;
    pub const BIT_MASK: u32 = 0x00000700;
    pub const BIT_WIDTH: u8 = 3;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod VTRIM_DELTA {
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
        pub VTRIM_DELTA: B3,
        pub reserved_3_8: B5,
        pub ITRIM_UDIGLDO: B3,
        pub ITRIM_DIGLDO_LOAD: B2,
        pub reserved_13_16: B3,
        pub GLDO_CURSRC: B3,
        pub reserved_19_24: B5,
        pub VDDR_TRIM_SLEEP: B5,
        pub reserved_29_32: B3,
    }

    impl HwRegister for Register {
        const RESERVED_BITS_MASK: u32 = 0xe0f8e0f8;
        const READ_ONLY_BITS_MASK: u32 = 0x1f071f07;
        const WRITE_ONLY_BITS_MASK: u32 = 0x00000000;

        fn read(&self) -> u32 {
            u32::from(self.content)
        }

        fn mutate(&mut self, word: u32) {
            let old_val: u32 = self.read();
            let mut new_val: u32 = word;

            // Check if modifies reserved bits
            if old_val & Self::RESERVED_BITS_MASK != new_val & Self::RESERVED_BITS_MASK {
                warn!(target: "cc2650_constants::FCFG1::LDO_TRIM", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::FCFG1::LDO_TRIM",
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
