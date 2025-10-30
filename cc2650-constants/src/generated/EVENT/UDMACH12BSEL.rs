use cmemu_common::Address;

pub const DISPLAY: &str = "UDMACH12BSEL";
pub const OFFSET: u32 = 0x564;
/// 0x40083564
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
pub const RESET_VALUE: u32 = 0x00000050;
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
    pub const RESET_VALUE: u32 = 0x50;
    pub const WRITABLE: bool = true;
    pub const RESET_ENUM: self::Values = self::Values::GPT1B_DMABREQ;
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
        /// GPT3B DMA trigger event. Configured by GPT3:DMAEV
        GPT3B_DMABREQ = 84,
        /// GPT3A DMA trigger event. Configured by GPT3:DMAEV
        GPT3A_DMABREQ = 83,
        /// GPT2B DMA trigger event. Configured by GPT2:DMAEV
        GPT2B_DMABREQ = 82,
        /// GPT2A DMA trigger event. Configured by GPT2:DMAEV
        GPT2A_DMABREQ = 81,
        /// GPT1B DMA trigger event. Configured by GPT1:DMAEV
        GPT1B_DMABREQ = 80,
        /// GPT1A DMA trigger event. Configured by GPT1:DMAEV
        GPT1A_DMABREQ = 79,
        /// GPT0B DMA trigger event. Configured by GPT0:DMAEV
        GPT0B_DMABREQ = 78,
        /// GPT0A DMA trigger event. Configured by GPT0:DMAEV
        GPT0A_DMABREQ = 77,
        /// Always inactive
        NONE = 0,
    }
    pub use self::Named as E;
    pub mod Named {
        /// Always asserted
        pub const ALWAYS_ACTIVE: u32 = 121;
        /// GPT3B DMA trigger event. Configured by GPT3:DMAEV
        pub const GPT3B_DMABREQ: u32 = 84;
        /// GPT3A DMA trigger event. Configured by GPT3:DMAEV
        pub const GPT3A_DMABREQ: u32 = 83;
        /// GPT2B DMA trigger event. Configured by GPT2:DMAEV
        pub const GPT2B_DMABREQ: u32 = 82;
        /// GPT2A DMA trigger event. Configured by GPT2:DMAEV
        pub const GPT2A_DMABREQ: u32 = 81;
        /// GPT1B DMA trigger event. Configured by GPT1:DMAEV
        pub const GPT1B_DMABREQ: u32 = 80;
        /// GPT1A DMA trigger event. Configured by GPT1:DMAEV
        pub const GPT1A_DMABREQ: u32 = 79;
        /// GPT0B DMA trigger event. Configured by GPT0:DMAEV
        pub const GPT0B_DMABREQ: u32 = 78;
        /// GPT0A DMA trigger event. Configured by GPT0:DMAEV
        pub const GPT0A_DMABREQ: u32 = 77;
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
                warn!(target: "cc2650_constants::EVENT::UDMACH12BSEL", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::EVENT::UDMACH12BSEL",
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
