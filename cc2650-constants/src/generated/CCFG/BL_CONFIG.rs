use cmemu_common::Address;

pub const DISPLAY: &str = "BL_CONFIG";
pub const OFFSET: u32 = 0xfd8;
/// 0x50003fd8
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
pub const RESET_VALUE: u32 = 0xc5ffffff;
pub const RESET_MASK: u32 = 0xffffffff;
/// Bootloader enable. Boot loader can be accessed if IMAGE_VALID_CONF.IMAGE_VALID is non-zero or BL_ENABLE is enabled (and conditions for boot loader backdoor are met).
///
/// 0xC5: Boot loader is enabled.
///
/// Any other value: Boot loader is disabled.
pub mod BOOTLOADER_ENABLE {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 24..=31;
    pub const BIT_MASK: u32 = 0xff000000;
    pub const BIT_WIDTH: u8 = 8;
    pub const RESET_VALUE: u32 = 0xc5;
    pub const WRITABLE: bool = false;
}
/// Sets the active level of the selected DIO number BL_PIN_NUMBER if boot loader backdoor is enabled by the BL_ENABLE field.
///
/// 0: Active low.
///
/// 1: Active high.
pub mod BL_LEVEL {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 16..=16;
    pub const BIT_MASK: u32 = 0x00010000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = false;
}
/// DIO number that is level checked if the boot loader backdoor is enabled by the BL_ENABLE field.
pub mod BL_PIN_NUMBER {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 8..=15;
    pub const BIT_MASK: u32 = 0x0000ff00;
    pub const BIT_WIDTH: u8 = 8;
    pub const RESET_VALUE: u32 = 0xff;
    pub const WRITABLE: bool = false;
}
/// Enables the boot loader backdoor.
///
/// 0xC5: Boot loader backdoor is enabled.
///
/// Any other value: Boot loader backdoor is disabled.
///
///
///
/// NOTE! Boot loader must be enabled (see BOOTLOADER_ENABLE) if boot loader backdoor is enabled.
pub mod BL_ENABLE {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 0..=7;
    pub const BIT_MASK: u32 = 0x000000ff;
    pub const BIT_WIDTH: u8 = 8;
    pub const RESET_VALUE: u32 = 0xff;
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
        pub BL_ENABLE: B8,
        pub BL_PIN_NUMBER: B8,
        pub BL_LEVEL: B1,
        pub reserved_17_24: B7,
        pub BOOTLOADER_ENABLE: B8,
    }

    impl HwRegister for Register {
        const RESERVED_BITS_MASK: u32 = 0x00fe0000;
        const READ_ONLY_BITS_MASK: u32 = 0xff01ffff;
        const WRITE_ONLY_BITS_MASK: u32 = 0x00000000;

        fn read(&self) -> u32 {
            u32::from(self.content)
        }

        fn mutate(&mut self, word: u32) {
            let old_val: u32 = self.read();
            let mut new_val: u32 = word;

            // Check if modifies reserved bits
            if old_val & Self::RESERVED_BITS_MASK != new_val & Self::RESERVED_BITS_MASK {
                warn!(target: "cc2650_constants::CCFG::BL_CONFIG", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::CCFG::BL_CONFIG",
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
