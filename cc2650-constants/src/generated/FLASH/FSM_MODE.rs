use cmemu_common::Address;

pub const DISPLAY: &str = "FSM_MODE";
pub const OFFSET: u32 = 0x225c;
/// 0x4003225c
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
pub const RESET_VALUE: u32 = 0x00000000;
pub const RESET_MASK: u32 = 0xffffffff;
/// Internal. Only to be used through TI provided API.
pub mod RDV_SUBMODE {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 18..=19;
    pub const BIT_MASK: u32 = 0x000c0000;
    pub const BIT_WIDTH: u8 = 2;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod PGM_SUBMODE {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 16..=17;
    pub const BIT_MASK: u32 = 0x00030000;
    pub const BIT_WIDTH: u8 = 2;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod ERA_SUBMODE {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 14..=15;
    pub const BIT_MASK: u32 = 0x0000c000;
    pub const BIT_WIDTH: u8 = 2;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod SUBMODE {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 12..=13;
    pub const BIT_MASK: u32 = 0x00003000;
    pub const BIT_WIDTH: u8 = 2;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod SAV_PGM_CMD {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 9..=11;
    pub const BIT_MASK: u32 = 0x00000e00;
    pub const BIT_WIDTH: u8 = 3;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod SAV_ERA_MODE {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 6..=8;
    pub const BIT_MASK: u32 = 0x000001c0;
    pub const BIT_WIDTH: u8 = 3;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod MODE {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 3..=5;
    pub const BIT_MASK: u32 = 0x00000038;
    pub const BIT_WIDTH: u8 = 3;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod CMD {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 0..=2;
    pub const BIT_MASK: u32 = 0x00000007;
    pub const BIT_WIDTH: u8 = 3;
    pub const RESET_VALUE: u32 = 0x0;
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
        pub CMD: B3,
        pub MODE: B3,
        pub SAV_ERA_MODE: B3,
        pub SAV_PGM_CMD: B3,
        pub SUBMODE: B2,
        pub ERA_SUBMODE: B2,
        pub PGM_SUBMODE: B2,
        pub RDV_SUBMODE: B2,
        pub reserved_20_32: B12,
    }

    impl HwRegister for Register {
        const RESERVED_BITS_MASK: u32 = 0xfff00000;
        const READ_ONLY_BITS_MASK: u32 = 0x000fffff;
        const WRITE_ONLY_BITS_MASK: u32 = 0x00000000;

        fn read(&self) -> u32 {
            u32::from(self.content)
        }

        fn mutate(&mut self, word: u32) {
            let old_val: u32 = self.read();
            let mut new_val: u32 = word;

            // Check if modifies reserved bits
            if old_val & Self::RESERVED_BITS_MASK != new_val & Self::RESERVED_BITS_MASK {
                warn!(target: "cc2650_constants::FLASH::FSM_MODE", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::FLASH::FSM_MODE",
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
