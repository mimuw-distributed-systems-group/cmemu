use cmemu_common::Address;

pub const DISPLAY: &str = "SHDW_ANA_TRIM";
pub const OFFSET: u32 = 0x13c;
/// 0x5000113c
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
/// Internal. Only to be used through TI provided API.
pub mod BOD_BANDGAP_TRIM_CNF {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 25..=26;
    pub const BIT_MASK: u32 = 0x06000000;
    pub const BIT_WIDTH: u8 = 2;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod VDDR_ENABLE_PG1 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 24..=24;
    pub const BIT_MASK: u32 = 0x01000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod VDDR_OK_HYS {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 23..=23;
    pub const BIT_MASK: u32 = 0x00800000;
    pub const BIT_WIDTH: u8 = 1;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod IPTAT_TRIM {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 21..=22;
    pub const BIT_MASK: u32 = 0x00600000;
    pub const BIT_WIDTH: u8 = 2;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod VDDR_TRIM {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 16..=20;
    pub const BIT_MASK: u32 = 0x001f0000;
    pub const BIT_WIDTH: u8 = 5;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod TRIMBOD_INTMODE {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 11..=15;
    pub const BIT_MASK: u32 = 0x0000f800;
    pub const BIT_WIDTH: u8 = 5;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod TRIMBOD_EXTMODE {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 6..=10;
    pub const BIT_MASK: u32 = 0x000007c0;
    pub const BIT_WIDTH: u8 = 5;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod TRIMTEMP {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 0..=5;
    pub const BIT_MASK: u32 = 0x0000003f;
    pub const BIT_WIDTH: u8 = 6;
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
        pub TRIMTEMP: B6,
        pub TRIMBOD_EXTMODE: B5,
        pub TRIMBOD_INTMODE: B5,
        pub VDDR_TRIM: B5,
        pub IPTAT_TRIM: B2,
        pub VDDR_OK_HYS: B1,
        pub VDDR_ENABLE_PG1: B1,
        pub BOD_BANDGAP_TRIM_CNF: B2,
        pub reserved_27_32: B5,
    }

    impl HwRegister for Register {
        const RESERVED_BITS_MASK: u32 = 0xf8000000;
        const READ_ONLY_BITS_MASK: u32 = 0x07ffffff;
        const WRITE_ONLY_BITS_MASK: u32 = 0x00000000;

        fn read(&self) -> u32 {
            u32::from(self.content)
        }

        fn mutate(&mut self, word: u32) {
            let old_val: u32 = self.read();
            let mut new_val: u32 = word;

            // Check if modifies reserved bits
            if old_val & Self::RESERVED_BITS_MASK != new_val & Self::RESERVED_BITS_MASK {
                warn!(target: "cc2650_constants::FCFG1::SHDW_ANA_TRIM", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::FCFG1::SHDW_ANA_TRIM",
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
