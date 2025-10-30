use cmemu_common::Address;

pub const DISPLAY: &str = "SATCFG";
pub const OFFSET: u32 = 0xc;
/// 0x400c400c
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
pub const RESET_VALUE: u32 = 0x0000000f;
pub const RESET_MASK: u32 = 0xffffffff;
/// Saturation limit.
///
///
///
/// The flag STAT.SAT is set when the TDC counter saturates.
///
///
///
/// Values not enumerated are not supported
pub mod LIMIT {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 0..=3;
    pub const BIT_MASK: u32 = 0x0000000f;
    pub const BIT_WIDTH: u8 = 4;
    pub const RESET_VALUE: u32 = 0xf;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// Result bit 24: TDC conversion saturates and stops when RESULT.VALUE\[24\] is set.
        pub const R24: u32 = 15;
        /// Result bit 23: TDC conversion saturates and stops when RESULT.VALUE\[23\] is set.
        pub const R23: u32 = 14;
        /// Result bit 22: TDC conversion saturates and stops when RESULT.VALUE\[22\] is set.
        pub const R22: u32 = 13;
        /// Result bit 21: TDC conversion saturates and stops when RESULT.VALUE\[21\] is set.
        pub const R21: u32 = 12;
        /// Result bit 20: TDC conversion saturates and stops when RESULT.VALUE\[20\] is set.
        pub const R20: u32 = 11;
        /// Result bit 19: TDC conversion saturates and stops when RESULT.VALUE\[19\] is set.
        pub const R19: u32 = 10;
        /// Result bit 18: TDC conversion saturates and stops when RESULT.VALUE\[18\] is set.
        pub const R18: u32 = 9;
        /// Result bit 17: TDC conversion saturates and stops when RESULT.VALUE\[17\] is set.
        pub const R17: u32 = 8;
        /// Result bit 16: TDC conversion saturates and stops when RESULT.VALUE\[16\] is set.
        pub const R16: u32 = 7;
        /// Result bit 15: TDC conversion saturates and stops when RESULT.VALUE\[15\] is set.
        pub const R15: u32 = 6;
        /// Result bit 14: TDC conversion saturates and stops when RESULT.VALUE\[14\] is set.
        pub const R14: u32 = 5;
        /// Result bit 13: TDC conversion saturates and stops when RESULT.VALUE\[13\] is set.
        pub const R13: u32 = 4;
        /// Result bit 12: TDC conversion saturates and stops when RESULT.VALUE\[12\] is set.
        pub const R12: u32 = 3;
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
        pub LIMIT: B4,
        pub reserved_4_32: B28,
    }

    impl HwRegister for Register {
        const RESERVED_BITS_MASK: u32 = 0xfffffff0;
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
                warn!(target: "cc2650_constants::AUX_TDCIF::SATCFG", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::AUX_TDCIF::SATCFG",
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
