use cmemu_common::Address;

pub const DISPLAY: &str = "DMAHWVER";
pub const OFFSET: u32 = 0xfc;
/// 0x400240fc
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
pub const RESET_VALUE: u32 = 0x01012ed1;
pub const RESET_MASK: u32 = 0xffffffff;
/// Major version number
pub mod HW_MAJOR_VER {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 24..=27;
    pub const BIT_MASK: u32 = 0x0f000000;
    pub const BIT_WIDTH: u8 = 4;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = false;
}
/// Minor version number
pub mod HW_MINOR_VER {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 20..=23;
    pub const BIT_MASK: u32 = 0x00f00000;
    pub const BIT_WIDTH: u8 = 4;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Patch level.
pub mod HW_PATCH_LVL {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 16..=19;
    pub const BIT_MASK: u32 = 0x000f0000;
    pub const BIT_WIDTH: u8 = 4;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = false;
}
/// Bit-by-bit complement of the VER_NUM field bits.
pub mod VER_NUM_COMPL {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 8..=15;
    pub const BIT_MASK: u32 = 0x0000ff00;
    pub const BIT_WIDTH: u8 = 8;
    pub const RESET_VALUE: u32 = 0x2e;
    pub const WRITABLE: bool = false;
}
/// Version number of the DMA Controller (209)
pub mod VER_NUM {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 0..=7;
    pub const BIT_MASK: u32 = 0x000000ff;
    pub const BIT_WIDTH: u8 = 8;
    pub const RESET_VALUE: u32 = 0xd1;
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
        pub VER_NUM: B8,
        pub VER_NUM_COMPL: B8,
        pub HW_PATCH_LVL: B4,
        pub HW_MINOR_VER: B4,
        pub HW_MAJOR_VER: B4,
        pub reserved_28_32: B4,
    }

    impl HwRegister for Register {
        const RESERVED_BITS_MASK: u32 = 0xf0000000;
        const READ_ONLY_BITS_MASK: u32 = 0x0fffffff;
        const WRITE_ONLY_BITS_MASK: u32 = 0x00000000;

        fn read(&self) -> u32 {
            u32::from(self.content)
        }

        fn mutate(&mut self, word: u32) {
            let old_val: u32 = self.read();
            let mut new_val: u32 = word;

            // Check if modifies reserved bits
            if old_val & Self::RESERVED_BITS_MASK != new_val & Self::RESERVED_BITS_MASK {
                warn!(target: "cc2650_constants::CRYPTO::DMAHWVER", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::CRYPTO::DMAHWVER",
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
