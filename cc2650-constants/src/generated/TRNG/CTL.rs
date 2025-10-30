use cmemu_common::Address;

pub const DISPLAY: &str = "CTL";
pub const OFFSET: u32 = 0x14;
/// 0x40028014
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
pub const RESET_VALUE: u32 = 0x00000000;
pub const RESET_MASK: u32 = 0xffffffff;
/// This field determines the number of samples (between 2^8 and 2^24) taken to gather entropy from the FROs during startup. If the written value of this field is zero, the number of samples is 2^24, otherwise the number of samples equals the written value times 2^8.
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
/// This field can only be modified while TRNG_EN is 0. If 1 an update will be ignored.
pub mod STARTUP_CYCLES {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 16..=31;
    pub const BIT_MASK: u32 = 0xffff0000;
    pub const BIT_WIDTH: u8 = 16;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// 0: Forces all TRNG logic back into the idle state immediately.
///
/// 1: Starts TRNG, gathering entropy from the FROs for the number of samples determined by STARTUP_CYCLES.
pub mod TRNG_EN {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 10..=10;
    pub const BIT_MASK: u32 = 0x00000400;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// 1: Remove XNOR feedback from the main LFSR, converting it into a normal shift register for the XOR-ed outputs of the FROs (shifting data in on the LSB side). A '1' also forces the LFSR to sample continuously.
///
///
///
/// This bit can only be set to '1' when TEST_MODE is also set to '1' and should not be used for other than test purposes
pub mod NO_LFSR_FB {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 2..=2;
    pub const BIT_MASK: u32 = 0x00000004;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// 1: Enables access to the TESTCNT and LFSR0/LFSR1/LFSR2 registers (the latter are automatically cleared before enabling access)  and keeps IRQFLAGSTAT.NEED_CLOCK at '1'.
///
///
///
/// This bit shall not be used unless you need to change the LFSR seed prior to creating a new random value. All other testing is done external to register control.
pub mod TEST_MODE {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 1..=1;
    pub const BIT_MASK: u32 = 0x00000002;
    pub const BIT_WIDTH: u8 = 1;
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
        pub reserved_0_1: B1,
        pub TEST_MODE: B1,
        pub NO_LFSR_FB: B1,
        pub reserved_3_10: B7,
        pub TRNG_EN: B1,
        pub reserved_11_16: B5,
        pub STARTUP_CYCLES: B16,
    }

    impl HwRegister for Register {
        const RESERVED_BITS_MASK: u32 = 0x0000fbf9;
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
                warn!(target: "cc2650_constants::TRNG::CTL", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::TRNG::CTL",
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
