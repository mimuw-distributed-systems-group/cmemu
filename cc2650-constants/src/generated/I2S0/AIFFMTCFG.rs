use cmemu_common::Address;

pub const DISPLAY: &str = "AIFFMTCFG";
pub const OFFSET: u32 = 0xc;
/// 0x4002100c
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
pub const RESET_VALUE: u32 = 0x00000170;
pub const RESET_MASK: u32 = 0xffffffff;
/// The number of BCLK periods between a WCLK edge and MSB of the first word in a phase:
///
///
///
/// 0x00: LJF and DSP format
///
/// 0x01: I2S and DSP format
///
/// 0x02: RJF format
///
/// ...
///
/// 0xFF: RJF format
///
///
///
/// Note: When 0, MSB of the next word will be output in the idle period between LSB of the previous word and the start of the next word. Otherwise logical 0 will be output until the data delay has expired.
pub mod DATA_DELAY {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 8..=15;
    pub const BIT_MASK: u32 = 0x0000ff00;
    pub const BIT_WIDTH: u8 = 8;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = true;
}
/// The size of each word stored to or loaded from memory:
pub mod MEM_LEN_24 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 7..=7;
    pub const BIT_MASK: u32 = 0x00000080;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// 24-bit (one 8 bit and one 16 bit locked access per sample)
        pub const _24BIT: u32 = 1;
        /// 16-bit (one 16 bit access per sample)
        pub const _16BIT: u32 = 0;
    }
}
/// On the serial audio interface, data (and wclk) is sampled and clocked out on opposite edges of BCLK.
pub mod SMPL_EDGE {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 6..=6;
    pub const BIT_MASK: u32 = 0x00000040;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// Data is sampled on the positive edge and clocked out on the negative edge.
        pub const POS: u32 = 1;
        /// Data is sampled on the negative edge and clocked out on the positive edge.
        pub const NEG: u32 = 0;
    }
}
/// Selects dual- or single-phase format.
///
///
///
/// 0: Single-phase: DSP format
///
/// 1: Dual-phase: I2S, LJF and RJF formats
pub mod DUAL_PHASE {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 5..=5;
    pub const BIT_MASK: u32 = 0x00000020;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = true;
}
/// Number of bits per word (8-24):
///
/// In single-phase format, this is the exact number of bits per word.
///
/// In dual-phase format, this is the maximum number of bits per word.
///
///
///
/// Values below 8 and above 24 give undefined behavior. Data written to memory is always aligned to 16 or 24 bits as defined by MEM_LEN_24. Bit widths that differ from this alignment will either be truncated or zero padded.
pub mod WORD_LEN {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 0..=4;
    pub const BIT_MASK: u32 = 0x0000001f;
    pub const BIT_WIDTH: u8 = 5;
    pub const RESET_VALUE: u32 = 0x10;
    pub const WRITABLE: bool = true;
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
        pub WORD_LEN: B5,
        pub DUAL_PHASE: B1,
        pub SMPL_EDGE: B1,
        pub MEM_LEN_24: B1,
        pub DATA_DELAY: B8,
        pub reserved_16_32: B16,
    }

    impl HwRegister for Register {
        const RESERVED_BITS_MASK: u32 = 0xffff0000;
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
                warn!(target: "cc2650_constants::I2S0::AIFFMTCFG", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::I2S0::AIFFMTCFG",
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
