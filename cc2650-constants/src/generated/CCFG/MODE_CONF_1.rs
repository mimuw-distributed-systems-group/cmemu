use cmemu_common::Address;

pub const DISPLAY: &str = "MODE_CONF_1";
pub const OFFSET: u32 = 0xfac;
/// 0x50003fac
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
pub const RESET_VALUE: u32 = 0xfffbffff;
pub const RESET_MASK: u32 = 0xffffffff;
/// Minimum voltage for when DC/DC should be used if alternate DC/DC setting is enabled (SIZE_AND_DIS_FLAGS.DIS_ALT_DCDC_SETTING=0).
///
/// Voltage = (28 + ALT_DCDC_VMIN) / 16.
///
/// 0: 1.75V
///
/// 1: 1.8125V
///
/// ...
///
/// 14: 2.625V
///
/// 15: 2.6875V
///
///
///
/// NOTE! The DriverLib function SysCtrl_DCDC_VoltageConditionalControl() must be called regularly to apply this field (handled automatically if using TI RTOS!).
pub mod ALT_DCDC_VMIN {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 20..=23;
    pub const BIT_MASK: u32 = 0x00f00000;
    pub const BIT_WIDTH: u8 = 4;
    pub const RESET_VALUE: u32 = 0xf;
    pub const WRITABLE: bool = false;
}
/// Enable DC/DC dithering if alternate DC/DC setting is enabled (SIZE_AND_DIS_FLAGS.DIS_ALT_DCDC_SETTING=0).
///
/// 0: Dither disable
///
/// 1: Dither enable
pub mod ALT_DCDC_DITHER_EN {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 19..=19;
    pub const BIT_MASK: u32 = 0x00080000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = false;
}
/// Inductor peak current if alternate DC/DC setting is enabled (SIZE_AND_DIS_FLAGS.DIS_ALT_DCDC_SETTING=0). Assuming 10uH external inductor!
///
/// Peak current = 31 + ( 4 * ALT_DCDC_IPEAK ) :
///
/// 0: 31mA (min)
///
/// ...
///
/// 4: 47mA
///
/// ...
///
/// 7: 59mA (max)
pub mod ALT_DCDC_IPEAK {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 16..=18;
    pub const BIT_MASK: u32 = 0x00070000;
    pub const BIT_WIDTH: u8 = 3;
    pub const RESET_VALUE: u32 = 0x3;
    pub const WRITABLE: bool = false;
}
/// Signed delta value for IBIAS_INIT. Delta value only applies if SIZE_AND_DIS_FLAGS.DIS_XOSC_OVR=0.
///
/// See FCFG1:AMPCOMP_CTRL1.IBIAS_INIT
pub mod DELTA_IBIAS_INIT {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 12..=15;
    pub const BIT_MASK: u32 = 0x0000f000;
    pub const BIT_WIDTH: u8 = 4;
    pub const RESET_VALUE: u32 = 0xf;
    pub const WRITABLE: bool = false;
}
/// Signed delta value for IBIAS_OFFSET. Delta value only applies if SIZE_AND_DIS_FLAGS.DIS_XOSC_OVR=0.
///
/// See FCFG1:AMPCOMP_CTRL1.IBIAS_OFFSET
pub mod DELTA_IBIAS_OFFSET {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 8..=11;
    pub const BIT_MASK: u32 = 0x00000f00;
    pub const BIT_WIDTH: u8 = 4;
    pub const RESET_VALUE: u32 = 0xf;
    pub const WRITABLE: bool = false;
}
/// Unsigned value of maximum XOSC startup time (worst case) in units of 100us. Value only applies if SIZE_AND_DIS_FLAGS.DIS_XOSC_OVR=0.
pub mod XOSC_MAX_START {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 0..=7;
    pub const BIT_MASK: u32 = 0x000000ff;
    pub const BIT_WIDTH: u8 = 8;
    pub const RESET_VALUE: u32 = 0xff;
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
        pub XOSC_MAX_START: B8,
        pub DELTA_IBIAS_OFFSET: B4,
        pub DELTA_IBIAS_INIT: B4,
        pub ALT_DCDC_IPEAK: B3,
        pub ALT_DCDC_DITHER_EN: B1,
        pub ALT_DCDC_VMIN: B4,
        pub reserved_24_32: B8,
    }

    impl HwRegister for Register {
        const RESERVED_BITS_MASK: u32 = 0xff000000;
        const READ_ONLY_BITS_MASK: u32 = 0x00ffffff;
        const WRITE_ONLY_BITS_MASK: u32 = 0x00000000;

        fn read(&self) -> u32 {
            u32::from(self.content)
        }

        fn mutate(&mut self, word: u32) {
            let old_val: u32 = self.read();
            let mut new_val: u32 = word;

            // Check if modifies reserved bits
            if old_val & Self::RESERVED_BITS_MASK != new_val & Self::RESERVED_BITS_MASK {
                warn!(target: "cc2650_constants::CCFG::MODE_CONF_1", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::CCFG::MODE_CONF_1",
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
