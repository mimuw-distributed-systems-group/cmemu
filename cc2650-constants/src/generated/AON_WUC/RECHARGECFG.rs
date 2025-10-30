use cmemu_common::Address;

pub const DISPLAY: &str = "RECHARGECFG";
pub const OFFSET: u32 = 0x30;
/// 0x40091030
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
pub const RESET_VALUE: u32 = 0x00000000;
pub const RESET_MASK: u32 = 0xffffffff;
/// Enable adaptive recharge
///
///
///
/// Note: Recharge can be turned completely of by setting MAX_PER_E=7 and MAX_PER_M=31 and this bitfield to 0
pub mod ADAPTIVE_EN {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 31..=31;
    pub const BIT_MASK: u32 = 0x80000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Gain factor for adaptive recharge algorithm
///
///
///
/// period_new=period * ( 1+/-(2^-C1+2^-C2) )
///
/// Valid values for C2 is 2 to 10
///
///
///
/// Note: Rounding may cause adaptive recharge not to start for very small values of both Gain and Initial period. Criteria for algorithm to start is MAX(PERIOD*2^-C1,PERIOD*2^-C2) >= 1
pub mod C2 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 20..=23;
    pub const BIT_MASK: u32 = 0x00f00000;
    pub const BIT_WIDTH: u8 = 4;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Gain factor for adaptive recharge algorithm
///
///
///
/// period_new=period * ( 1+/-(2^-C1+2^-C2) )
///
/// Valid values for C1 is 1 to 10
///
///
///
/// Note: Rounding may cause adaptive recharge not to start for very small values of both Gain and Initial period. Criteria for algorithm to start is MAX(PERIOD*2^-C1,PERIOD*2^-C2) >= 1
pub mod C1 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 16..=19;
    pub const BIT_MASK: u32 = 0x000f0000;
    pub const BIT_WIDTH: u8 = 4;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// This register defines the maximum period that the recharge algorithm can take, i.e. it defines  the maximum number of cycles between 2 recharges.
///
/// The maximum number of cycles is specified with a 5 bit mantissa and 3 bit exponent:
///
/// MAXCYCLES=(MAX_PER_M*16+15)*2^MAX_PER_E
///
/// This field sets the mantissa of MAXCYCLES
pub mod MAX_PER_M {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 11..=15;
    pub const BIT_MASK: u32 = 0x0000f800;
    pub const BIT_WIDTH: u8 = 5;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// This register defines the maximum period that the recharge algorithm can take, i.e. it defines  the maximum number of cycles between 2 recharges.
///
/// The maximum number of cycles is specified with a 5 bit mantissa and 3 bit exponent:
///
/// MAXCYCLES=(MAX_PER_M*16+15)*2^MAX_PER_E
///
/// This field sets the exponent MAXCYCLES
pub mod MAX_PER_E {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 8..=10;
    pub const BIT_MASK: u32 = 0x00000700;
    pub const BIT_WIDTH: u8 = 3;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Number of 32 KHz clocks between activation of recharge controller
///
/// For recharge algorithm, PERIOD is the initial period when entering powerdown mode. The adaptive recharge algorithm will not change this register
///
/// PERIOD will effectively be a 16 bit value coded in a 5 bit mantissa and 3 bit exponent:
///
/// This field sets the Mantissa of the Period.
///
/// PERIOD=(PER_M*16+15)*2^PER_E
pub mod PER_M {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 3..=7;
    pub const BIT_MASK: u32 = 0x000000f8;
    pub const BIT_WIDTH: u8 = 5;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Number of 32 KHz clocks between activation of recharge controller
///
/// For recharge algorithm, PERIOD is the initial period when entering powerdown mode. The adaptive recharge algorithm will not change this register
///
/// PERIOD will effectively be a 16 bit value coded in a 5 bit mantissa and 3 bit exponent:
///
/// This field sets the Exponent of the Period.  
///
/// PERIOD=(PER_M*16+15)*2^PER_E
pub mod PER_E {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 0..=2;
    pub const BIT_MASK: u32 = 0x00000007;
    pub const BIT_WIDTH: u8 = 3;
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
        pub PER_E: B3,
        pub PER_M: B5,
        pub MAX_PER_E: B3,
        pub MAX_PER_M: B5,
        pub C1: B4,
        pub C2: B4,
        pub reserved_24_31: B7,
        pub ADAPTIVE_EN: B1,
    }

    impl HwRegister for Register {
        const RESERVED_BITS_MASK: u32 = 0x7f000000;
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
                warn!(target: "cc2650_constants::AON_WUC::RECHARGECFG", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::AON_WUC::RECHARGECFG",
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
