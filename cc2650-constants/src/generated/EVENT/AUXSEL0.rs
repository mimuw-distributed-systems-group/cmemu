use cmemu_common::Address;

pub const DISPLAY: &str = "AUXSEL0";
pub const OFFSET: u32 = 0x700;
/// 0x40083700
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
pub const RESET_VALUE: u32 = 0x00000010;
pub const RESET_MASK: u32 = 0xffffffff;
/// Read/write selection value
///
///
///
/// Writing any other value than values defined by a ENUM may result in undefined behavior.
pub mod EV {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 0..=6;
    pub const BIT_MASK: u32 = 0x0000007f;
    pub const BIT_WIDTH: u8 = 7;
    pub const RESET_VALUE: u32 = 0x10;
    pub const WRITABLE: bool = true;
    pub const RESET_ENUM: self::Values = self::Values::GPT0A;
    pub use self::Values as V;
    use modular_bitfield::prelude::BitfieldSpecifier;
    use num_enum::IntoPrimitive;
    #[repr(u8)]
    #[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, BitfieldSpecifier)]
    #[bits = 7]
    #[allow(non_camel_case_types)]
    #[non_exhaustive]
    pub enum Values {
        /// Always asserted
        ALWAYS_ACTIVE = 121,
        /// GPT1B interrupt event, controlled by GPT1:TBMR
        GPT1B = 19,
        /// GPT1A interrupt event, controlled by GPT1:TAMR
        GPT1A = 18,
        /// GPT0B interrupt event, controlled by GPT0:TBMR
        GPT0B = 17,
        /// GPT0A interrupt event, controlled by GPT0:TAMR
        GPT0A = 16,
        /// GPT3B interrupt event, controlled by GPT3:TBMR
        GPT3B = 15,
        /// GPT3A interrupt event, controlled by GPT3:TAMR
        GPT3A = 14,
        /// GPT2B interrupt event, controlled by GPT2:TBMR
        GPT2B = 13,
        /// GPT2A interrupt event, controlled by GPT2:TAMR
        GPT2A = 12,
        /// Always inactive
        NONE = 0,
    }
    pub use self::Named as E;
    pub mod Named {
        /// Always asserted
        pub const ALWAYS_ACTIVE: u32 = 121;
        /// GPT1B interrupt event, controlled by GPT1:TBMR
        pub const GPT1B: u32 = 19;
        /// GPT1A interrupt event, controlled by GPT1:TAMR
        pub const GPT1A: u32 = 18;
        /// GPT0B interrupt event, controlled by GPT0:TBMR
        pub const GPT0B: u32 = 17;
        /// GPT0A interrupt event, controlled by GPT0:TAMR
        pub const GPT0A: u32 = 16;
        /// GPT3B interrupt event, controlled by GPT3:TBMR
        pub const GPT3B: u32 = 15;
        /// GPT3A interrupt event, controlled by GPT3:TAMR
        pub const GPT3A: u32 = 14;
        /// GPT2B interrupt event, controlled by GPT2:TBMR
        pub const GPT2B: u32 = 13;
        /// GPT2A interrupt event, controlled by GPT2:TAMR
        pub const GPT2A: u32 = 12;
        /// Always inactive
        pub const NONE: u32 = 0;
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
        pub EV: super::EV::V,
        pub reserved_7_32: B25,
    }

    impl HwRegister for Register {
        const RESERVED_BITS_MASK: u32 = 0xffffff80;
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
                warn!(target: "cc2650_constants::EVENT::AUXSEL0", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::EVENT::AUXSEL0",
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
