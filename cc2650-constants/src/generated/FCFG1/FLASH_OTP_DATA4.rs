use cmemu_common::Address;

pub const DISPLAY: &str = "FLASH_OTP_DATA4";
pub const OFFSET: u32 = 0x308;
/// 0x50001308
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
pub const RESET_VALUE: u32 = 0x98989f9f;
pub const RESET_MASK: u32 = 0xffffffff;
/// Internal. Only to be used through TI provided API.
pub mod STANDBY_MODE_SEL_INT_WRT {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 31..=31;
    pub const BIT_MASK: u32 = 0x80000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod STANDBY_PW_SEL_INT_WRT {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 29..=30;
    pub const BIT_MASK: u32 = 0x60000000;
    pub const BIT_WIDTH: u8 = 2;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod DIS_STANDBY_INT_WRT {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 28..=28;
    pub const BIT_MASK: u32 = 0x10000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod DIS_IDLE_INT_WRT {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 27..=27;
    pub const BIT_MASK: u32 = 0x08000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod VIN_AT_X_INT_WRT {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 24..=26;
    pub const BIT_MASK: u32 = 0x07000000;
    pub const BIT_WIDTH: u8 = 3;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod STANDBY_MODE_SEL_EXT_WRT {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 23..=23;
    pub const BIT_MASK: u32 = 0x00800000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod STANDBY_PW_SEL_EXT_WRT {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 21..=22;
    pub const BIT_MASK: u32 = 0x00600000;
    pub const BIT_WIDTH: u8 = 2;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod DIS_STANDBY_EXT_WRT {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 20..=20;
    pub const BIT_MASK: u32 = 0x00100000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod DIS_IDLE_EXT_WRT {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 19..=19;
    pub const BIT_MASK: u32 = 0x00080000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod VIN_AT_X_EXT_WRT {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 16..=18;
    pub const BIT_MASK: u32 = 0x00070000;
    pub const BIT_WIDTH: u8 = 3;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod STANDBY_MODE_SEL_INT_RD {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 15..=15;
    pub const BIT_MASK: u32 = 0x00008000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod STANDBY_PW_SEL_INT_RD {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 13..=14;
    pub const BIT_MASK: u32 = 0x00006000;
    pub const BIT_WIDTH: u8 = 2;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod DIS_STANDBY_INT_RD {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 12..=12;
    pub const BIT_MASK: u32 = 0x00001000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod DIS_IDLE_INT_RD {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 11..=11;
    pub const BIT_MASK: u32 = 0x00000800;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod VIN_AT_X_INT_RD {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 8..=10;
    pub const BIT_MASK: u32 = 0x00000700;
    pub const BIT_WIDTH: u8 = 3;
    pub const RESET_VALUE: u32 = 0x7;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod STANDBY_MODE_SEL_EXT_RD {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 7..=7;
    pub const BIT_MASK: u32 = 0x00000080;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod STANDBY_PW_SEL_EXT_RD {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 5..=6;
    pub const BIT_MASK: u32 = 0x00000060;
    pub const BIT_WIDTH: u8 = 2;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod DIS_STANDBY_EXT_RD {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 4..=4;
    pub const BIT_MASK: u32 = 0x00000010;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod DIS_IDLE_EXT_RD {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 3..=3;
    pub const BIT_MASK: u32 = 0x00000008;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod VIN_AT_X_EXT_RD {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 0..=2;
    pub const BIT_MASK: u32 = 0x00000007;
    pub const BIT_WIDTH: u8 = 3;
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
        pub VIN_AT_X_EXT_RD: B3,
        pub DIS_IDLE_EXT_RD: B1,
        pub DIS_STANDBY_EXT_RD: B1,
        pub STANDBY_PW_SEL_EXT_RD: B2,
        pub STANDBY_MODE_SEL_EXT_RD: B1,
        pub VIN_AT_X_INT_RD: B3,
        pub DIS_IDLE_INT_RD: B1,
        pub DIS_STANDBY_INT_RD: B1,
        pub STANDBY_PW_SEL_INT_RD: B2,
        pub STANDBY_MODE_SEL_INT_RD: B1,
        pub VIN_AT_X_EXT_WRT: B3,
        pub DIS_IDLE_EXT_WRT: B1,
        pub DIS_STANDBY_EXT_WRT: B1,
        pub STANDBY_PW_SEL_EXT_WRT: B2,
        pub STANDBY_MODE_SEL_EXT_WRT: B1,
        pub VIN_AT_X_INT_WRT: B3,
        pub DIS_IDLE_INT_WRT: B1,
        pub DIS_STANDBY_INT_WRT: B1,
        pub STANDBY_PW_SEL_INT_WRT: B2,
        pub STANDBY_MODE_SEL_INT_WRT: B1,
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
                warn!(target: "cc2650_constants::FCFG1::FLASH_OTP_DATA4", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::FCFG1::FLASH_OTP_DATA4",
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
