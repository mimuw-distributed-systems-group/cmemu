use cmemu_common::Address;

pub const DISPLAY: &str = "CTL0";
pub const OFFSET: u32 = 0x0;
/// 0x400ca000
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
pub const RESET_VALUE: u32 = 0x00000000;
pub const RESET_MASK: u32 = 0xffffffff;
/// Set based on the accurate high frequency XTAL.
pub mod XTAL_IS_24M {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 31..=31;
    pub const BIT_MASK: u32 = 0x80000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// Internal. Only to be used through TI provided API.
        pub const _24M: u32 = 1;
        /// Internal. Only to be used through TI provided API.
        pub const _48M: u32 = 0;
    }
}
/// Internal. Only to be used through TI provided API.
pub mod BYPASS_XOSC_LF_CLK_QUAL {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 29..=29;
    pub const BIT_MASK: u32 = 0x20000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Internal. Only to be used through TI provided API.
pub mod BYPASS_RCOSC_LF_CLK_QUAL {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 28..=28;
    pub const BIT_MASK: u32 = 0x10000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Internal. Only to be used through TI provided API.
pub mod DOUBLER_START_DURATION {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 26..=27;
    pub const BIT_MASK: u32 = 0x0c000000;
    pub const BIT_WIDTH: u8 = 2;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Internal. Only to be used through TI provided API.
pub mod DOUBLER_RESET_DURATION {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 25..=25;
    pub const BIT_MASK: u32 = 0x02000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Internal. Only to be used through TI provided API.
pub mod FORCE_KICKSTART_EN {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 22..=22;
    pub const BIT_MASK: u32 = 0x00400000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// 0: Default - Switching of HF clock source is disabled .
///
/// 1: Allows switching of sclk_hf source.
///
///
///
/// Provided to prevent switching of the SCLK_HF source when running from flash (a long period during switching could corrupt flash). When sclk_hf  switching is disabled, a new source can be started when SCLK_HF_SRC_SEL is changed, but the switch will not occur until this bit is set.  This bit should be set to enable clock switching after STAT0.PENDINGSCLKHFSWITCHING indicates  the new HF clock is ready. When switching completes (also indicated by STAT0.PENDINGSCLKHFSWITCHING)  sclk_hf switching should be disabled to prevent flash corruption.  Switching should not be enabled when running from flash.
pub mod ALLOW_SCLK_HF_SWITCHING {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 16..=16;
    pub const BIT_MASK: u32 = 0x00010000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Internal. Only to be used through TI provided API.
pub mod HPOSC_MODE_EN {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 14..=14;
    pub const BIT_MASK: u32 = 0x00004000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Internal. Only to be used through TI provided API.
pub mod RCOSC_LF_TRIMMED {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 12..=12;
    pub const BIT_MASK: u32 = 0x00001000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Internal. Only to be used through TI provided API.
pub mod XOSC_HF_POWER_MODE {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 11..=11;
    pub const BIT_MASK: u32 = 0x00000800;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Bypass XOSC_LF and use the digital input clock from AON for the xosc_lf clock.
///
///
///
/// 0: Use 32kHz XOSC as xosc_lf clock source
///
/// 1: Use digital input (from AON) as xosc_lf clock source.
///
///
///
/// This bit will only have effect when SCLK_LF_SRC_SEL is selecting the xosc_lf as the sclk_lf source. The muxing performed by this bit is not glitch free. The following procedure must be followed when changing this field to avoid glitches on sclk_lf.
///
///
///
/// 1) Set SCLK_LF_SRC_SEL to select any source other than the xosc_lf clock source.
///
/// 2) Set or clear this bit to bypass or not bypass the xosc_lf.
///
/// 3) Set SCLK_LF_SRC_SEL to use xosc_lf.
///
///
///
/// It is recommended that either the rcosc_hf or xosc_hf (whichever is currently active) be selected as the source in step 1 above. This provides a faster clock change.
pub mod XOSC_LF_DIG_BYPASS {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 10..=10;
    pub const BIT_MASK: u32 = 0x00000400;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Enable clock loss detection and hence the indicators to system controller.  Checks both SCLK_HF and SCLK_LF clock loss indicators.
///
///
///
/// 0: Disable
///
/// 1: Enable
///
///
///
/// Clock loss detection must be disabled when changing the sclk_lf source.  STAT0.SCLK_LF_SRC can be polled to determine when a change to a new sclk_lf source has completed.
pub mod CLK_LOSS_EN {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 9..=9;
    pub const BIT_MASK: u32 = 0x00000200;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Source select for aclk_tdc.
///
///
///
/// 00: RCOSC_HF (48MHz)
///
/// 01: RCOSC_HF (24MHz)
///
/// 10: XOSC_HF (24MHz)
///
/// 11: Not used
pub mod ACLK_TDC_SRC_SEL {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 7..=8;
    pub const BIT_MASK: u32 = 0x00000180;
    pub const BIT_WIDTH: u8 = 2;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Source select for aclk_ref
///
///
///
/// 00: RCOSC_HF derived (31.25kHz)
///
/// 01: XOSC_HF derived (31.25kHz)
///
/// 10: RCOSC_LF (32kHz)
///
/// 11: XOSC_LF (32.768kHz)
pub mod ACLK_REF_SRC_SEL {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 5..=6;
    pub const BIT_MASK: u32 = 0x00000060;
    pub const BIT_WIDTH: u8 = 2;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.
pub mod SPARE4 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 4..=4;
    pub const BIT_MASK: u32 = 0x00000010;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Source select for sclk_lf
pub mod SCLK_LF_SRC_SEL {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 2..=3;
    pub const BIT_MASK: u32 = 0x0000000c;
    pub const BIT_WIDTH: u8 = 2;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// Low frequency XOSC
        pub const XOSCLF: u32 = 3;
        /// Low frequency RCOSC
        pub const RCOSCLF: u32 = 2;
        /// Low frequency clock derived from High Frequency XOSC
        pub const XOSCHFDLF: u32 = 1;
        /// Low frequency clock derived from High Frequency RCOSC
        pub const RCOSCHFDLF: u32 = 0;
    }
}
/// Internal. Only to be used through TI provided API.
pub mod SCLK_MF_SRC_SEL {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 1..=1;
    pub const BIT_MASK: u32 = 0x00000002;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// Medium frequency clock derived from high frequency XOSC.
        pub const XCOSCHFDMF: u32 = 1;
        /// Internal. Only to be used through TI provided API.
        pub const RCOSCHFDMF: u32 = 0;
    }
}
/// Source select for sclk_hf.  XOSC option is supported for test and debug only and should be used when the XOSC_HF is running.
pub mod SCLK_HF_SRC_SEL {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 0..=0;
    pub const BIT_MASK: u32 = 0x00000001;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// High frequency XOSC clk
        pub const XOSC: u32 = 1;
        /// High frequency RCOSC clock
        pub const RCOSC: u32 = 0;
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
        pub SCLK_HF_SRC_SEL: B1,
        pub SCLK_MF_SRC_SEL: B1,
        pub SCLK_LF_SRC_SEL: B2,
        pub SPARE4: B1,
        pub ACLK_REF_SRC_SEL: B2,
        pub ACLK_TDC_SRC_SEL: B2,
        pub CLK_LOSS_EN: B1,
        pub XOSC_LF_DIG_BYPASS: B1,
        pub XOSC_HF_POWER_MODE: B1,
        pub RCOSC_LF_TRIMMED: B1,
        pub reserved_13_14: B1,
        pub HPOSC_MODE_EN: B1,
        pub reserved_15_16: B1,
        pub ALLOW_SCLK_HF_SWITCHING: B1,
        pub reserved_17_22: B5,
        pub FORCE_KICKSTART_EN: B1,
        pub reserved_23_25: B2,
        pub DOUBLER_RESET_DURATION: B1,
        pub DOUBLER_START_DURATION: B2,
        pub BYPASS_RCOSC_LF_CLK_QUAL: B1,
        pub BYPASS_XOSC_LF_CLK_QUAL: B1,
        pub reserved_30_31: B1,
        pub XTAL_IS_24M: B1,
    }

    impl HwRegister for Register {
        const RESERVED_BITS_MASK: u32 = 0x41bea000;
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
                warn!(target: "cc2650_constants::AUX_DDI0_OSC::CTL0", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::AUX_DDI0_OSC::CTL0",
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
