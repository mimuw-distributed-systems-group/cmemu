use cmemu_common::Address;

pub const DISPLAY: &str = "DMABUSCFG";
pub const OFFSET: u32 = 0x78;
/// 0x40024078
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
pub const RESET_VALUE: u32 = 0x00002400;
pub const RESET_MASK: u32 = 0xffffffff;
/// Maximum burst size that can be performed on the AHB bus
pub mod AHB_MST1_BURST_SIZE {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 12..=15;
    pub const BIT_MASK: u32 = 0x0000f000;
    pub const BIT_WIDTH: u8 = 4;
    pub const RESET_VALUE: u32 = 0x2;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// 64 bytes
        pub const _64_BYTE: u32 = 6;
        /// 32 bytes
        pub const _32_BYTE: u32 = 5;
        /// 16 bytes
        pub const _16_BYTE: u32 = 4;
        /// 8 bytes
        pub const _8_BYTE: u32 = 3;
        /// 4 bytes
        pub const _4_BYTE: u32 = 2;
    }
}
/// Idle transfer insertion between consecutive burst transfers on AHB
pub mod AHB_MST1_IDLE_EN {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 11..=11;
    pub const BIT_MASK: u32 = 0x00000800;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// Idle transfer insertion enabled
        pub const IDLE: u32 = 1;
        /// Do not insert idle transfers.
        pub const NO_IDLE: u32 = 0;
    }
}
/// Burst length type of AHB transfer
pub mod AHB_MST1_INCR_EN {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 10..=10;
    pub const BIT_MASK: u32 = 0x00000400;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// Fixed length bursts or single transfers
        pub const SPECIFIED: u32 = 1;
        /// Unspecified length burst transfers
        pub const UNSPECIFIED: u32 = 0;
    }
}
/// Locked transform on AHB
pub mod AHB_MST1_LOCK_EN {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 9..=9;
    pub const BIT_MASK: u32 = 0x00000200;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// Transfers are locked
        pub const LOCKED: u32 = 1;
        /// Transfers are not locked
        pub const NOT_LOCKED: u32 = 0;
    }
}
/// Endianess for the AHB master
pub mod AHB_MST1_BIGEND {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 8..=8;
    pub const BIT_MASK: u32 = 0x00000100;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// Big Endian
        pub const BIG_ENDIAN: u32 = 1;
        /// Little Endian
        pub const LITTLE_ENDIAN: u32 = 0;
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
        pub reserved_0_8: B8,
        pub AHB_MST1_BIGEND: B1,
        pub AHB_MST1_LOCK_EN: B1,
        pub AHB_MST1_INCR_EN: B1,
        pub AHB_MST1_IDLE_EN: B1,
        pub AHB_MST1_BURST_SIZE: B4,
        pub reserved_16_32: B16,
    }

    impl HwRegister for Register {
        const RESERVED_BITS_MASK: u32 = 0xffff00ff;
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
                warn!(target: "cc2650_constants::CRYPTO::DMABUSCFG", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::CRYPTO::DMABUSCFG",
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
