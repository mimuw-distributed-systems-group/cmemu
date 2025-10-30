use cmemu_common::Address;

pub const DISPLAY: &str = "AUXCLK";
pub const OFFSET: u32 = 0x4;
/// 0x40091004
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
pub const RESET_VALUE: u32 = 0x00000001;
pub const RESET_MASK: u32 = 0xffffffff;
/// When AUX requests powerdown with SCLK_HF as source, then WUC will switch over to this clock source during powerdown, and automatically switch back to SCLK_HF when AUX system is back in active mode
pub mod PWR_DWN_SRC {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 11..=12;
    pub const BIT_MASK: u32 = 0x00001800;
    pub const BIT_WIDTH: u8 = 2;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// Use SCLK_LF in Powerdown
        pub const SCLK_LF: u32 = 1;
        /// No clock in Powerdown
        pub const NONE: u32 = 0;
    }
}
/// Select the AUX clock divider for SCLK_HF
///
///
///
/// NB: It is not supported to change the AUX clock divider while SCLK_HF is active source for AUX
pub mod SCLK_HF_DIV {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 8..=10;
    pub const BIT_MASK: u32 = 0x00000700;
    pub const BIT_WIDTH: u8 = 3;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// Divide by 256
        pub const DIV256: u32 = 7;
        /// Divide by 128
        pub const DIV128: u32 = 6;
        /// Divide by 64
        pub const DIV64: u32 = 5;
        /// Divide by 32
        pub const DIV32: u32 = 4;
        /// Divide by 16
        pub const DIV16: u32 = 3;
        /// Divide by 8
        pub const DIV8: u32 = 2;
        /// Divide by 4
        pub const DIV4: u32 = 1;
        /// Divide by 2
        pub const DIV2: u32 = 0;
    }
}
/// Selects the clock source for AUX:
///
///
///
/// NB: Switching the clock source is guaranteed to be glitchless
pub mod SRC {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 0..=2;
    pub const BIT_MASK: u32 = 0x00000007;
    pub const BIT_WIDTH: u8 = 3;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// LF Clock (SCLK_LF)
        pub const SCLK_LF: u32 = 4;
        /// HF Clock (SCLK_HF)
        pub const SCLK_HF: u32 = 1;
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
        pub SRC: B3,
        pub reserved_3_8: B5,
        pub SCLK_HF_DIV: B3,
        pub PWR_DWN_SRC: B2,
        pub reserved_13_32: B19,
    }

    impl HwRegister for Register {
        const RESERVED_BITS_MASK: u32 = 0xffffe0f8;
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
                warn!(target: "cc2650_constants::AON_WUC::AUXCLK", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::AON_WUC::AUXCLK",
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
