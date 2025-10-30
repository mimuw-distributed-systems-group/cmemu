use cmemu_common::Address;

pub const DISPLAY: &str = "FCFG_WRAPPER";
pub const OFFSET: u32 = 0x2404;
/// 0x40032404
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
pub const RESET_VALUE: u32 = 0x50009007;
pub const RESET_MASK: u32 = 0xffffffff;
/// Internal. Only to be used through TI provided API.
pub mod FAMILY_TYPE {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 24..=31;
    pub const BIT_MASK: u32 = 0xff000000;
    pub const BIT_WIDTH: u8 = 8;
    pub const RESET_VALUE: u32 = 0x50;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod MEM_MAP {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 20..=20;
    pub const BIT_MASK: u32 = 0x00100000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod CPU2 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 16..=19;
    pub const BIT_MASK: u32 = 0x000f0000;
    pub const BIT_WIDTH: u8 = 4;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod EE_IN_MAIN {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 12..=15;
    pub const BIT_MASK: u32 = 0x0000f000;
    pub const BIT_WIDTH: u8 = 4;
    pub const RESET_VALUE: u32 = 0x9;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod ROM {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 11..=11;
    pub const BIT_MASK: u32 = 0x00000800;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod IFLUSH {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 10..=10;
    pub const BIT_MASK: u32 = 0x00000400;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod SIL3 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 9..=9;
    pub const BIT_MASK: u32 = 0x00000200;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod ECCA {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 8..=8;
    pub const BIT_MASK: u32 = 0x00000100;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod AUTO_SUSP {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 6..=7;
    pub const BIT_MASK: u32 = 0x000000c0;
    pub const BIT_WIDTH: u8 = 2;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod UERR {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 4..=5;
    pub const BIT_MASK: u32 = 0x00000030;
    pub const BIT_WIDTH: u8 = 2;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod CPU_TYPE1 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 0..=3;
    pub const BIT_MASK: u32 = 0x0000000f;
    pub const BIT_WIDTH: u8 = 4;
    pub const RESET_VALUE: u32 = 0x7;
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
        pub CPU_TYPE1: B4,
        pub UERR: B2,
        pub AUTO_SUSP: B2,
        pub ECCA: B1,
        pub SIL3: B1,
        pub IFLUSH: B1,
        pub ROM: B1,
        pub EE_IN_MAIN: B4,
        pub CPU2: B4,
        pub MEM_MAP: B1,
        pub reserved_21_24: B3,
        pub FAMILY_TYPE: B8,
    }

    impl HwRegister for Register {
        const RESERVED_BITS_MASK: u32 = 0x00e00000;
        const READ_ONLY_BITS_MASK: u32 = 0xff1fffff;
        const WRITE_ONLY_BITS_MASK: u32 = 0x00000000;

        fn read(&self) -> u32 {
            u32::from(self.content)
        }

        fn mutate(&mut self, word: u32) {
            let old_val: u32 = self.read();
            let mut new_val: u32 = word;

            // Check if modifies reserved bits
            if old_val & Self::RESERVED_BITS_MASK != new_val & Self::RESERVED_BITS_MASK {
                warn!(target: "cc2650_constants::FLASH::FCFG_WRAPPER", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::FLASH::FCFG_WRAPPER",
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
