use cmemu_common::Address;

pub const DISPLAY: &str = "KEYREADAREA";
pub const OFFSET: u32 = 0x40c;
/// 0x4002440c
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
pub const RESET_VALUE: u32 = 0x00000008;
pub const RESET_MASK: u32 = 0xffffffff;
/// Key store operation busy status flag (read only)
///
///
///
/// 0: operation is completed.
///
/// 1: operation is not completed and the key store is busy.
pub mod BUSY {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 31..=31;
    pub const BIT_MASK: u32 = 0x80000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Selects the area of the key store RAM from where the key needs to be read that will be written to the AES engine.
///
///
///
/// Only RAM areas that contain valid written keys can be selected.
pub mod RAM_AREA {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 0..=3;
    pub const BIT_MASK: u32 = 0x0000000f;
    pub const BIT_WIDTH: u8 = 4;
    pub const RESET_VALUE: u32 = 0x8;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// No RAM
        pub const NO_RAM: u32 = 8;
        /// RAM Area 7
        pub const RAM_AREA7: u32 = 7;
        /// RAM Area 6
        pub const RAM_AREA6: u32 = 6;
        /// RAM Area 5
        pub const RAM_AREA5: u32 = 5;
        /// RAM Area 4
        pub const RAM_AREA4: u32 = 4;
        /// RAM Area 3
        pub const RAM_AREA3: u32 = 3;
        /// RAM Area 2
        pub const RAM_AREA2: u32 = 2;
        /// RAM Area 1
        pub const RAM_AREA1: u32 = 1;
        /// RAM Area 0
        pub const RAM_AREA0: u32 = 0;
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
        pub RAM_AREA: B4,
        pub reserved_4_31: B27,
        pub BUSY: B1,
    }

    impl HwRegister for Register {
        const RESERVED_BITS_MASK: u32 = 0x7ffffff0;
        const READ_ONLY_BITS_MASK: u32 = 0x80000000;
        const WRITE_ONLY_BITS_MASK: u32 = 0x00000000;

        fn read(&self) -> u32 {
            u32::from(self.content)
        }

        fn mutate(&mut self, word: u32) {
            let old_val: u32 = self.read();
            let mut new_val: u32 = word;

            // Check if modifies reserved bits
            if old_val & Self::RESERVED_BITS_MASK != new_val & Self::RESERVED_BITS_MASK {
                warn!(target: "cc2650_constants::CRYPTO::KEYREADAREA", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::CRYPTO::KEYREADAREA",
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
