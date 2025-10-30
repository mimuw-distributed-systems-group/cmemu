use cmemu_common::Address;

pub const DISPLAY: &str = "ADCFIFOSTAT";
pub const OFFSET: u32 = 0x14;
/// 0x400c9014
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
pub const RESET_VALUE: u32 = 0x00000001;
pub const RESET_MASK: u32 = 0xffffffff;
/// FIFO overflow flag.
///
///
///
/// 0: FIFO has not overflowed.
///
/// 1: FIFO has overflowed, this flag is sticky until you flush the FIFO.
///
///
///
/// When the flag is set, the ADC FIFO write pointer is static. It is not possible to add more samples to the ADC FIFO. Flush FIFO to clear the flag.
pub mod OVERFLOW {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 4..=4;
    pub const BIT_MASK: u32 = 0x00000010;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// FIFO underflow flag.
///
///
///
/// 0: FIFO has not underflowed.
///
/// 1: FIFO has underflowed, this flag is sticky until you flush the FIFO.
///
///
///
/// When the flag is set, the ADC FIFO read pointer is static. Read returns the previous sample that was read. Flush FIFO to clear the flag.
pub mod UNDERFLOW {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 3..=3;
    pub const BIT_MASK: u32 = 0x00000008;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// FIFO full flag.
///
///
///
/// 0: FIFO is not full, there is less than 4 samples in the FIFO.
///
/// 1: FIFO is full, there are 4 samples in the FIFO.
///
///
///
/// When the flag is set, it is not possible to add more samples to the ADC FIFO. An attempt to add samples sets the OVERFLOW flag.
pub mod FULL {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 2..=2;
    pub const BIT_MASK: u32 = 0x00000004;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// FIFO almost full flag.
///
///
///
/// 0: There are less than 3 samples in the FIFO, or the FIFO is full. The FULL flag is also asserted in the latter case.
///
/// 1: There are 3 samples in the FIFO, there is room for one more sample.
pub mod ALMOST_FULL {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 1..=1;
    pub const BIT_MASK: u32 = 0x00000002;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// FIFO empty flag.
///
///
///
/// 0: FIFO contains one or more samples.
///
/// 1: FIFO is empty.
///
///
///
/// When the flag is set, read returns the previous sample that was read and sets the UNDERFLOW flag.
pub mod EMPTY {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 0..=0;
    pub const BIT_MASK: u32 = 0x00000001;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = false;
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
        pub EMPTY: B1,
        pub ALMOST_FULL: B1,
        pub FULL: B1,
        pub UNDERFLOW: B1,
        pub OVERFLOW: B1,
        pub reserved_5_32: B27,
    }

    impl HwRegister for Register {
        const RESERVED_BITS_MASK: u32 = 0xffffffe0;
        const READ_ONLY_BITS_MASK: u32 = 0x0000001f;
        const WRITE_ONLY_BITS_MASK: u32 = 0x00000000;

        fn read(&self) -> u32 {
            u32::from(self.content)
        }

        fn mutate(&mut self, word: u32) {
            let old_val: u32 = self.read();
            let mut new_val: u32 = word;

            // Check if modifies reserved bits
            if old_val & Self::RESERVED_BITS_MASK != new_val & Self::RESERVED_BITS_MASK {
                warn!(target: "cc2650_constants::AUX_ANAIF::ADCFIFOSTAT", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::AUX_ANAIF::ADCFIFOSTAT",
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
