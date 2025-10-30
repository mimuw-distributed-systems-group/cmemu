use cmemu_common::Address;

pub const DISPLAY: &str = "STAT1";
pub const OFFSET: u32 = 0x38;
/// 0x400ca038
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
pub const RESET_VALUE: u32 = 0x00000000;
pub const RESET_MASK: u32 = 0xffffffff;
/// AMPCOMP FSM State
pub mod RAMPSTATE {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 28..=31;
    pub const BIT_MASK: u32 = 0xf0000000;
    pub const BIT_WIDTH: u8 = 4;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
    pub use self::Named as E;
    pub mod Named {
        /// FAST_START_SETTLE
        pub const FAST_START_SETTLE: u32 = 14;
        /// FAST_START
        pub const FAST_START: u32 = 13;
        /// DUMMY_TO_INIT_1
        pub const DUMMY_TO_INIT_1: u32 = 12;
        /// IDAC_DECREMENT_WITH_MEASURE
        pub const IDAC_DEC_W_MEASURE: u32 = 11;
        /// IBIAS_INCREMENT
        pub const IBIAS_INC: u32 = 10;
        /// LPM_UPDATE
        pub const LPM_UPDATE: u32 = 9;
        /// IBIAS_DECREMENT_WITH_MEASURE
        pub const IBIAS_DEC_W_MEASURE: u32 = 8;
        /// IBIAS_CAP_UPDATE
        pub const IBIAS_CAP_UPDATE: u32 = 7;
        /// IDAC_INCREMENT
        pub const IDAC_INCREMENT: u32 = 6;
        /// HPM_UPDATE
        pub const HPM_UPDATE: u32 = 5;
        /// HPM_RAMP3
        pub const HPM_RAMP3: u32 = 4;
        /// HPM_RAMP2
        pub const HPM_RAMP2: u32 = 3;
        /// HPM_RAMP1
        pub const HPM_RAMP1: u32 = 2;
        /// INITIALIZATION
        pub const INITIALIZATION: u32 = 1;
        /// RESET
        pub const RESET: u32 = 0;
    }
}
/// OSC amplitude during HPM_UPDATE state.
///
/// When amplitude compensation of XOSC_HF is enabled in high performance mode, this value is the amplitude of the crystal oscillations measured by the on-chip oscillator ADC, divided by 15 mV.  For example, a value of 0x20 would indicate that the amplitude of the crystal is approximately 480 mV.  To enable amplitude compensation, AON_WUC OSCCFG must be set to a non-zero value.
pub mod HPM_UPDATE_AMP {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 22..=27;
    pub const BIT_MASK: u32 = 0x0fc00000;
    pub const BIT_WIDTH: u8 = 6;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// OSC amplitude during LPM_UPDATE state
///
/// When amplitude compensation of XOSC_HF is enabled in low power mode, this value is the amplitude of the crystal oscillations measured by the on-chip oscillator ADC, divided by 15 mV.  For example, a value of 0x20 would indicate that the amplitude of the crystal is approximately 480 mV.  To enable amplitude compensation, AON_WUC OSCCFG must be set to a non-zero value.
pub mod LPM_UPDATE_AMP {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 16..=21;
    pub const BIT_MASK: u32 = 0x003f0000;
    pub const BIT_WIDTH: u8 = 6;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// force_rcosc_hf
pub mod FORCE_RCOSC_HF {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 15..=15;
    pub const BIT_MASK: u32 = 0x00008000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// SCLK_HF_EN
pub mod SCLK_HF_EN {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 14..=14;
    pub const BIT_MASK: u32 = 0x00004000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// SCLK_MF_EN
pub mod SCLK_MF_EN {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 13..=13;
    pub const BIT_MASK: u32 = 0x00002000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// ACLK_ADC_EN
pub mod ACLK_ADC_EN {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 12..=12;
    pub const BIT_MASK: u32 = 0x00001000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// ACLK_TDC_EN
pub mod ACLK_TDC_EN {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 11..=11;
    pub const BIT_MASK: u32 = 0x00000800;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// ACLK_REF_EN
pub mod ACLK_REF_EN {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 10..=10;
    pub const BIT_MASK: u32 = 0x00000400;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// CLK_CHP_EN
pub mod CLK_CHP_EN {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 9..=9;
    pub const BIT_MASK: u32 = 0x00000200;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// CLK_DCDC_EN
pub mod CLK_DCDC_EN {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 8..=8;
    pub const BIT_MASK: u32 = 0x00000100;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// SCLK_HF_GOOD
pub mod SCLK_HF_GOOD {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 7..=7;
    pub const BIT_MASK: u32 = 0x00000080;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// SCLK_MF_GOOD
pub mod SCLK_MF_GOOD {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 6..=6;
    pub const BIT_MASK: u32 = 0x00000040;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// SCLK_LF_GOOD
pub mod SCLK_LF_GOOD {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 5..=5;
    pub const BIT_MASK: u32 = 0x00000020;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// ACLK_ADC_GOOD
pub mod ACLK_ADC_GOOD {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 4..=4;
    pub const BIT_MASK: u32 = 0x00000010;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// ACLK_TDC_GOOD
pub mod ACLK_TDC_GOOD {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 3..=3;
    pub const BIT_MASK: u32 = 0x00000008;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// ACLK_REF_GOOD
pub mod ACLK_REF_GOOD {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 2..=2;
    pub const BIT_MASK: u32 = 0x00000004;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// CLK_CHP_GOOD
pub mod CLK_CHP_GOOD {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 1..=1;
    pub const BIT_MASK: u32 = 0x00000002;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// CLK_DCDC_GOOD
pub mod CLK_DCDC_GOOD {
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
        pub CLK_DCDC_GOOD: B1,
        pub CLK_CHP_GOOD: B1,
        pub ACLK_REF_GOOD: B1,
        pub ACLK_TDC_GOOD: B1,
        pub ACLK_ADC_GOOD: B1,
        pub SCLK_LF_GOOD: B1,
        pub SCLK_MF_GOOD: B1,
        pub SCLK_HF_GOOD: B1,
        pub CLK_DCDC_EN: B1,
        pub CLK_CHP_EN: B1,
        pub ACLK_REF_EN: B1,
        pub ACLK_TDC_EN: B1,
        pub ACLK_ADC_EN: B1,
        pub SCLK_MF_EN: B1,
        pub SCLK_HF_EN: B1,
        pub FORCE_RCOSC_HF: B1,
        pub LPM_UPDATE_AMP: B6,
        pub HPM_UPDATE_AMP: B6,
        pub RAMPSTATE: B4,
    }

    impl HwRegister for Register {
        const RESERVED_BITS_MASK: u32 = 0x00000000;
        const READ_ONLY_BITS_MASK: u32 = 0xffffffff;
        const WRITE_ONLY_BITS_MASK: u32 = 0x00000000;

        fn read(&self) -> u32 {
            u32::from(self.content)
        }

        fn mutate(&mut self, word: u32) {
            let old_val: u32 = self.read();
            let mut new_val: u32 = word;

            // Check if modifies reserved bits
            if old_val & Self::RESERVED_BITS_MASK != new_val & Self::RESERVED_BITS_MASK {
                warn!(target: "cc2650_constants::AUX_DDI0_OSC::STAT1", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::AUX_DDI0_OSC::STAT1",
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
