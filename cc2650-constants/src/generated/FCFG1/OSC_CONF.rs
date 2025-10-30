use cmemu_common::Address;

pub const DISPLAY: &str = "OSC_CONF";
pub const OFFSET: u32 = 0x38c;
/// 0x5000138c
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
/// Trim value for DDI_0_OSC:ADCDOUBLERNANOAMPCTL.ADC_SH_VBUF_EN.
pub mod ADC_SH_VBUF_EN {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 29..=29;
    pub const BIT_MASK: u32 = 0x20000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const WRITABLE: bool = false;
}
/// Trim value for DDI_0_OSC:ADCDOUBLERNANOAMPCTL.ADC_SH_MODE_EN.
pub mod ADC_SH_MODE_EN {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 28..=28;
    pub const BIT_MASK: u32 = 0x10000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const WRITABLE: bool = false;
}
/// Trim value for DDI_0_OSC:ATESTCTL.ATESTLF_RCOSCLF_IBIAS_TRIM.
pub mod ATESTLF_RCOSCLF_IBIAS_TRIM {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 27..=27;
    pub const BIT_MASK: u32 = 0x08000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const WRITABLE: bool = false;
}
/// Trim value for DDI_0_OSC:LFOSCCTL.XOSCLF_REGULATOR_TRIM.
pub mod XOSCLF_REGULATOR_TRIM {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 25..=26;
    pub const BIT_MASK: u32 = 0x06000000;
    pub const BIT_WIDTH: u8 = 2;
    pub const WRITABLE: bool = false;
}
/// Trim value for DDI_0_OSC:LFOSCCTL.XOSCLF_CMIRRWR_RATIO.
pub mod XOSCLF_CMIRRWR_RATIO {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 21..=24;
    pub const BIT_MASK: u32 = 0x01e00000;
    pub const BIT_WIDTH: u8 = 4;
    pub const WRITABLE: bool = false;
}
/// Trim value for DDI_0_OSC:CTL1.XOSC_HF_FAST_START.
pub mod XOSC_HF_FAST_START {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 19..=20;
    pub const BIT_MASK: u32 = 0x00180000;
    pub const BIT_WIDTH: u8 = 2;
    pub const WRITABLE: bool = false;
}
/// 0: XOSC_HF unavailable (may not be bonded out)
///
/// 1: XOSC_HF available (default)
pub mod XOSC_OPTION {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 18..=18;
    pub const BIT_MASK: u32 = 0x00040000;
    pub const BIT_WIDTH: u8 = 1;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod HPOSC_OPTION {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 17..=17;
    pub const BIT_MASK: u32 = 0x00020000;
    pub const BIT_WIDTH: u8 = 1;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod HPOSC_BIAS_HOLD_MODE_EN {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 16..=16;
    pub const BIT_MASK: u32 = 0x00010000;
    pub const BIT_WIDTH: u8 = 1;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod HPOSC_CURRMIRR_RATIO {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 12..=15;
    pub const BIT_MASK: u32 = 0x0000f000;
    pub const BIT_WIDTH: u8 = 4;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod HPOSC_BIAS_RES_SET {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 8..=11;
    pub const BIT_MASK: u32 = 0x00000f00;
    pub const BIT_WIDTH: u8 = 4;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod HPOSC_FILTER_EN {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 7..=7;
    pub const BIT_MASK: u32 = 0x00000080;
    pub const BIT_WIDTH: u8 = 1;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod HPOSC_BIAS_RECHARGE_DELAY {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 5..=6;
    pub const BIT_MASK: u32 = 0x00000060;
    pub const BIT_WIDTH: u8 = 2;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod HPOSC_SERIES_CAP {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 1..=2;
    pub const BIT_MASK: u32 = 0x00000006;
    pub const BIT_WIDTH: u8 = 2;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod HPOSC_DIV3_BYPASS {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 0..=0;
    pub const BIT_MASK: u32 = 0x00000001;
    pub const BIT_WIDTH: u8 = 1;
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
        pub HPOSC_DIV3_BYPASS: B1,
        pub HPOSC_SERIES_CAP: B2,
        pub reserved_3_5: B2,
        pub HPOSC_BIAS_RECHARGE_DELAY: B2,
        pub HPOSC_FILTER_EN: B1,
        pub HPOSC_BIAS_RES_SET: B4,
        pub HPOSC_CURRMIRR_RATIO: B4,
        pub HPOSC_BIAS_HOLD_MODE_EN: B1,
        pub HPOSC_OPTION: B1,
        pub XOSC_OPTION: B1,
        pub XOSC_HF_FAST_START: B2,
        pub XOSCLF_CMIRRWR_RATIO: B4,
        pub XOSCLF_REGULATOR_TRIM: B2,
        pub ATESTLF_RCOSCLF_IBIAS_TRIM: B1,
        pub ADC_SH_MODE_EN: B1,
        pub ADC_SH_VBUF_EN: B1,
        pub reserved_30_32: B2,
    }

    impl HwRegister for Register {
        const RESERVED_BITS_MASK: u32 = 0xc0000018;
        const READ_ONLY_BITS_MASK: u32 = 0x3fffffe7;
        const WRITE_ONLY_BITS_MASK: u32 = 0x00000000;

        fn read(&self) -> u32 {
            u32::from(self.content)
        }

        fn mutate(&mut self, word: u32) {
            let old_val: u32 = self.read();
            let mut new_val: u32 = word;

            // Check if modifies reserved bits
            if old_val & Self::RESERVED_BITS_MASK != new_val & Self::RESERVED_BITS_MASK {
                warn!(target: "cc2650_constants::FCFG1::OSC_CONF", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::FCFG1::OSC_CONF",
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
