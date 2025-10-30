use cmemu_common::Address;

pub const DISPLAY: &str = "SIZE_AND_DIS_FLAGS";
pub const OFFSET: u32 = 0xfb0;
/// 0x50003fb0
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
pub const RESET_VALUE: u32 = 0xffffffff;
pub const RESET_MASK: u32 = 0xffffffff;
/// Total size of CCFG in bytes.
pub mod SIZE_OF_CCFG {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 16..=31;
    pub const BIT_MASK: u32 = 0xffff0000;
    pub const BIT_WIDTH: u8 = 16;
    pub const RESET_VALUE: u32 = 0xffff;
    pub const WRITABLE: bool = false;
}
/// Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior.
pub mod DISABLE_FLAGS {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 4..=15;
    pub const BIT_MASK: u32 = 0x0000fff0;
    pub const BIT_WIDTH: u8 = 12;
    pub const RESET_VALUE: u32 = 0xfff;
    pub const WRITABLE: bool = false;
}
/// Disable TCXO.
///
/// 0: TCXO functionality enabled.
///
/// 1: TCXO functionality disabled.
///
/// Note:
///
/// An external TCXO is required if DIS_TCXO = 0.
pub mod DIS_TCXO {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 3..=3;
    pub const BIT_MASK: u32 = 0x00000008;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = false;
}
/// Disable GPRAM (or use the 8K VIMS RAM as CACHE RAM).
///
/// 0: GPRAM is enabled and hence CACHE disabled.
///
/// 1: GPRAM is disabled and instead CACHE is enabled (default).
///
/// Notes:
///
/// - Disabling CACHE will reduce CPU execution speed (up to 60%).
///
/// - GPRAM is 8 K-bytes in size and located at 0x11000000-0x11001FFF if enabled.
///
/// See:
///
/// VIMS:CTL.MODE
pub mod DIS_GPRAM {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 2..=2;
    pub const BIT_MASK: u32 = 0x00000004;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = false;
}
/// Disable alternate DC/DC settings.
///
/// 0: Enable alternate DC/DC settings.
///
/// 1: Disable alternate DC/DC settings.
///
/// See:
///
/// MODE_CONF_1.ALT_DCDC_VMIN
///
/// MODE_CONF_1.ALT_DCDC_DITHER_EN
///
/// MODE_CONF_1.ALT_DCDC_IPEAK
///
///
///
/// NOTE! The DriverLib function SysCtrl_DCDC_VoltageConditionalControl() must be called regularly to apply this field (handled automatically if using TI RTOS!).
pub mod DIS_ALT_DCDC_SETTING {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 1..=1;
    pub const BIT_MASK: u32 = 0x00000002;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = false;
}
/// Disable XOSC override functionality.
///
/// 0: Enable XOSC override functionality.
///
/// 1: Disable XOSC override functionality.
///
/// See:
///
/// MODE_CONF_1.DELTA_IBIAS_INIT
///
/// MODE_CONF_1.DELTA_IBIAS_OFFSET
///
/// MODE_CONF_1.XOSC_MAX_START
pub mod DIS_XOSC_OVR {
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
        pub DIS_XOSC_OVR: B1,
        pub DIS_ALT_DCDC_SETTING: B1,
        pub DIS_GPRAM: B1,
        pub DIS_TCXO: B1,
        pub DISABLE_FLAGS: B12,
        pub SIZE_OF_CCFG: B16,
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
                warn!(target: "cc2650_constants::CCFG::SIZE_AND_DIS_FLAGS", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::CCFG::SIZE_AND_DIS_FLAGS",
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
