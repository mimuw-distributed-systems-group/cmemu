use cmemu_common::Address;

pub const DISPLAY: &str = "STAT2";
pub const OFFSET: u32 = 0x3c;
/// 0x400ca03c
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
pub const RESET_VALUE: u32 = 0x00000000;
pub const RESET_MASK: u32 = 0xffffffff;
/// DC Bias read by RADC during SAR mode
///
/// The value is an unsigned integer. It is used for debug only.
pub mod ADC_DCBIAS {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 26..=31;
    pub const BIT_MASK: u32 = 0xfc000000;
    pub const BIT_WIDTH: u8 = 6;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Indication of threshold is met for hpm_ramp1
pub mod HPM_RAMP1_THMET {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 25..=25;
    pub const BIT_MASK: u32 = 0x02000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Indication of threshold is met for hpm_ramp2
pub mod HPM_RAMP2_THMET {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 24..=24;
    pub const BIT_MASK: u32 = 0x01000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Indication of threshold is met for hpm_ramp3
pub mod HPM_RAMP3_THMET {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 23..=23;
    pub const BIT_MASK: u32 = 0x00800000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// xosc_hf amplitude compensation FSM
///
///
///
/// This is identical to STAT1.RAMPSTATE. See that description for encoding.
pub mod RAMPSTATE {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 12..=15;
    pub const BIT_MASK: u32 = 0x0000f000;
    pub const BIT_WIDTH: u8 = 4;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// ampcomp_req
pub mod AMPCOMP_REQ {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 3..=3;
    pub const BIT_MASK: u32 = 0x00000008;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// amplitude of xosc_hf is within the required threshold (set by DDI). Not used for anything just for debug/status
pub mod XOSC_HF_AMPGOOD {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 2..=2;
    pub const BIT_MASK: u32 = 0x00000004;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// frequency of xosc_hf is good to use for the digital clocks
pub mod XOSC_HF_FREQGOOD {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 1..=1;
    pub const BIT_MASK: u32 = 0x00000002;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// frequency of xosc_hf is within +/- 20 ppm and xosc_hf is good for radio operations. Used for SW to start synthesizer.
pub mod XOSC_HF_RF_FREQGOOD {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 0..=0;
    pub const BIT_MASK: u32 = 0x00000001;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
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
        pub XOSC_HF_RF_FREQGOOD: B1,
        pub XOSC_HF_FREQGOOD: B1,
        pub XOSC_HF_AMPGOOD: B1,
        pub AMPCOMP_REQ: B1,
        pub reserved_4_12: B8,
        pub RAMPSTATE: B4,
        pub reserved_16_23: B7,
        pub HPM_RAMP3_THMET: B1,
        pub HPM_RAMP2_THMET: B1,
        pub HPM_RAMP1_THMET: B1,
        pub ADC_DCBIAS: B6,
    }

    impl HwRegister for Register {
        const RESERVED_BITS_MASK: u32 = 0x007f0ff0;
        const READ_ONLY_BITS_MASK: u32 = 0xff80f00f;
        const WRITE_ONLY_BITS_MASK: u32 = 0x00000000;

        fn read(&self) -> u32 {
            u32::from(self.content)
        }

        fn mutate(&mut self, word: u32) {
            let old_val: u32 = self.read();
            let mut new_val: u32 = word;

            // Check if modifies reserved bits
            if old_val & Self::RESERVED_BITS_MASK != new_val & Self::RESERVED_BITS_MASK {
                warn!(target: "cc2650_constants::AUX_DDI0_OSC::STAT2", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::AUX_DDI0_OSC::STAT2",
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
