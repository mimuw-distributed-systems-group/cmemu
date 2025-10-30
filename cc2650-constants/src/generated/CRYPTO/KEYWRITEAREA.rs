use cmemu_common::Address;

pub const DISPLAY: &str = "KEYWRITEAREA";
pub const OFFSET: u32 = 0x400;
/// 0x40024400
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
pub const RESET_VALUE: u32 = 0x00000000;
pub const RESET_MASK: u32 = 0xffffffff;
/// Represents an area of 128 bits.
///
/// Select the key store RAM area(s) where the key(s) needs to be written.
///
///
///
/// Writing to multiple RAM locations is only possible when the selected RAM areas are sequential.
pub mod RAM_AREA7 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 7..=7;
    pub const BIT_MASK: u32 = 0x00000080;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// This RAM area is selected to be written
        pub const SEL: u32 = 1;
        /// This RAM area is not selected to be written
        pub const NOT_SEL: u32 = 0;
    }
}
/// Represents an area of 128 bits.
///
/// Select the key store RAM area(s) where the key(s) needs to be written.
///
///
///
/// Writing to multiple RAM locations is only possible when the selected RAM areas are sequential.
pub mod RAM_AREA6 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 6..=6;
    pub const BIT_MASK: u32 = 0x00000040;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// This RAM area is selected to be written
        pub const SEL: u32 = 1;
        /// This RAM area is not selected to be written
        pub const NOT_SEL: u32 = 0;
    }
}
/// Represents an area of 128 bits.
///
/// Select the key store RAM area(s) where the key(s) needs to be written.
///
///
///
/// Writing to multiple RAM locations is only possible when the selected RAM areas are sequential.
pub mod RAM_AREA5 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 5..=5;
    pub const BIT_MASK: u32 = 0x00000020;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// This RAM area is selected to be written
        pub const SEL: u32 = 1;
        /// This RAM area is not selected to be written
        pub const NOT_SEL: u32 = 0;
    }
}
/// Represents an area of 128 bits.
///
/// Select the key store RAM area(s) where the key(s) needs to be written.
///
///
///
/// Writing to multiple RAM locations is only possible when the selected RAM areas are sequential.
pub mod RAM_AREA4 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 4..=4;
    pub const BIT_MASK: u32 = 0x00000010;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// This RAM area is selected to be written
        pub const SEL: u32 = 1;
        /// This RAM area is not selected to be written
        pub const NOT_SEL: u32 = 0;
    }
}
/// Represents an area of 128 bits.
///
/// Select the key store RAM area(s) where the key(s) needs to be written.
///
///
///
/// Writing to multiple RAM locations is only possible when the selected RAM areas are sequential.
pub mod RAM_AREA3 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 3..=3;
    pub const BIT_MASK: u32 = 0x00000008;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// This RAM area is selected to be written
        pub const SEL: u32 = 1;
        /// This RAM area is not selected to be written
        pub const NOT_SEL: u32 = 0;
    }
}
/// Represents an area of 128 bits.
///
/// Select the key store RAM area(s) where the key(s) needs to be written.
///
///
///
/// Writing to multiple RAM locations is only possible when the selected RAM areas are sequential.
pub mod RAM_AREA2 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 2..=2;
    pub const BIT_MASK: u32 = 0x00000004;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// This RAM area is selected to be written
        pub const SEL: u32 = 1;
        /// This RAM area is not selected to be written
        pub const NOT_SEL: u32 = 0;
    }
}
/// Represents an area of 128 bits.
///
/// Select the key store RAM area(s) where the key(s) needs to be written.
///
///
///
/// Writing to multiple RAM locations is only possible when the selected RAM areas are sequential.
pub mod RAM_AREA1 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 1..=1;
    pub const BIT_MASK: u32 = 0x00000002;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// This RAM area is selected to be written
        pub const SEL: u32 = 1;
        /// This RAM area is not selected to be written
        pub const NOT_SEL: u32 = 0;
    }
}
/// Represents an area of 128 bits.
///
/// Select the key store RAM area(s) where the key(s) needs to be written.
///
///
///
/// Writing to multiple RAM locations is only possible when the selected RAM areas are sequential.
pub mod RAM_AREA0 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 0..=0;
    pub const BIT_MASK: u32 = 0x00000001;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// This RAM area is selected to be written
        pub const SEL: u32 = 1;
        /// This RAM area is not selected to be written
        pub const NOT_SEL: u32 = 0;
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
        pub RAM_AREA0: B1,
        pub RAM_AREA1: B1,
        pub RAM_AREA2: B1,
        pub RAM_AREA3: B1,
        pub RAM_AREA4: B1,
        pub RAM_AREA5: B1,
        pub RAM_AREA6: B1,
        pub RAM_AREA7: B1,
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
                warn!(target: "cc2650_constants::CRYPTO::KEYWRITEAREA", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::CRYPTO::KEYWRITEAREA",
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
