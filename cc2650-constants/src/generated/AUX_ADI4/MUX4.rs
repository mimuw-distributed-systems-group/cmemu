use cmemu_common::Address;

pub const DISPLAY: &str = "MUX4";
pub const OFFSET: u32 = 0x7;
/// 0x400cb007
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 8;
pub const RESET_VALUE: u32 = 0x00;
pub const RESET_MASK: u32 = 0xffffffff;
/// Internal. Only to be used through TI provided API.
pub mod COMPA_REF {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 0..=7;
    pub const BIT_MASK: u32 = 0xff;
    pub const BIT_WIDTH: u8 = 8;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// Internal. Only to be used through TI provided API.
        pub const AUXIO0: u32 = 128;
        /// Internal. Only to be used through TI provided API.
        pub const AUXIO1: u32 = 64;
        /// Internal. Only to be used through TI provided API.
        pub const AUXIO2: u32 = 32;
        /// Internal. Only to be used through TI provided API.
        pub const AUXIO3: u32 = 16;
        /// Internal. Only to be used through TI provided API.
        pub const AUXIO4: u32 = 8;
        /// Internal. Only to be used through TI provided API.
        pub const AUXIO5: u32 = 4;
        /// Internal. Only to be used through TI provided API.
        pub const AUXIO6: u32 = 2;
        /// Internal. Only to be used through TI provided API.
        pub const AUXIO7: u32 = 1;
        /// Internal. Only to be used through TI provided API.
        pub const NC: u32 = 0;
    }
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
        pub COMPA_REF: B8,
        pub reserved_8_32: B24,
    }

    impl HwRegister for Register {
        const RESERVED_BITS_MASK: u32 = 0xffffff00;
        const READ_ONLY_BITS_MASK: u32 = 0x00000000;
        const WRITE_ONLY_BITS_MASK: u32 = 0x00000000;

        fn read(&self) -> u32 {
            u32::from(self.content)
        }

        fn mutate(&mut self, word: u32) {
            let old_val: u32 = self.read();
            let mut new_val: u32 = word;

            // Check if modifies reserved bits
            if old_val & Self::RESERVED_BITS_MASK != new_val & Self::RESERVED_BITS_MASK {
                warn!(target: "cc2650_constants::AUX_ADI4::MUX4", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::AUX_ADI4::MUX4",
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
