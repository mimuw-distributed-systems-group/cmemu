use cmemu_common::Address;

pub const DISPLAY: &str = "CFG0";
pub const OFFSET: u32 = 0x18;
/// 0x40028018
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
pub const RESET_VALUE: u32 = 0x00000000;
pub const RESET_MASK: u32 = 0xffffffff;
/// This field determines the maximum number of samples (between 2^8 and 2^24) taken to re-generate entropy from the FROs after reading out a 64 bits random number. If the written value of this field is zero, the number of samples is 2^24, otherwise the number of samples equals the written value times 2^8.
///
///
///
/// 0x0000: 2^24 samples
///
/// 0x0001: 1*2^8 samples
///
/// 0x0002: 2*2^8 samples
///
/// 0x0003: 3*2^8 samples
///
/// ...
///
/// 0x8000: 32768*2^8 samples
///
/// 0xC000: 49152*2^8 samples
///
/// ...
///
/// 0xFFFF: 65535*2^8 samples
///
///
///
/// This field can only be modified while CTL.TRNG_EN is 0.
pub mod MAX_REFILL_CYCLES {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 16..=31;
    pub const BIT_MASK: u32 = 0xffff0000;
    pub const BIT_WIDTH: u8 = 16;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// This field directly controls the number of clock cycles between samples taken from the FROs. Default value 0 indicates that samples are taken every clock cycle,
///
/// maximum value 0xF takes one sample every 16 clock cycles.
///
/// This field must be set to a value such that the slowest FRO (even under worst-case
///
/// conditions) has a cycle time less than twice the sample period.
///
///
///
/// This field can only be modified while CTL.TRNG_EN is '0'.
pub mod SMPL_DIV {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 8..=11;
    pub const BIT_MASK: u32 = 0x00000f00;
    pub const BIT_WIDTH: u8 = 4;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// This field determines the minimum number of samples (between 2^6 and 2^14) taken to re-generate entropy from the FROs after reading out a 64 bits random number. If the value of this field is zero, the number of samples is fixed to the value determined by the MAX_REFILL_CYCLES field, otherwise the minimum number of samples equals the written value times 64 (which can be up to 2^14). To ensure same entropy in all generated random numbers the value 0 should be used. Then MAX_REFILL_CYCLES controls the minimum refill interval. The number of samples defined here cannot be higher than the number defined by the 'max_refill_cycles' field (i.e. that field takes precedence). No random value will be created if min refill > max refill.
///
///
///
/// This field can only be modified while CTL.TRNG_EN = 0.
///
///
///
/// 0x00: Minimum samples = MAX_REFILL_CYCLES (all numbers have same entropy)
///
/// 0x01: 1*2^6 samples
///
/// 0x02: 2*2^6 samples
///
/// ...
///
/// 0xFF: 255*2^6 samples
pub mod MIN_REFILL_CYCLES {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 0..=7;
    pub const BIT_MASK: u32 = 0x000000ff;
    pub const BIT_WIDTH: u8 = 8;
    pub const RESET_VALUE: u32 = 0x0;
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
        pub MIN_REFILL_CYCLES: B8,
        pub SMPL_DIV: B4,
        pub reserved_12_16: B4,
        pub MAX_REFILL_CYCLES: B16,
    }

    impl HwRegister for Register {
        const RESERVED_BITS_MASK: u32 = 0x0000f000;
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
                warn!(target: "cc2650_constants::TRNG::CFG0", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::TRNG::CFG0",
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
