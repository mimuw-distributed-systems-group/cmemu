use cmemu_common::Address;

pub const DISPLAY: &str = "MODE_CONF";
pub const OFFSET: u32 = 0xfb4;
/// 0x50003fb4
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
pub const RESET_VALUE: u32 = 0xffffffff;
pub const RESET_MASK: u32 = 0xffffffff;
/// Signed delta value to apply to the
///
/// VDDR_TRIM_SLEEP target, minus one. See FCFG1:VOLT_TRIM.VDDR_TRIM_SLEEP_H.
///
/// 0x8 (-8) : Delta = -7
///
/// ...
///
/// 0xF (-1) : Delta = 0
///
/// 0x0 (0) : Delta = +1
///
/// ...
///
/// 0x7 (7) : Delta = +8
pub mod VDDR_TRIM_SLEEP_DELTA {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 28..=31;
    pub const BIT_MASK: u32 = 0xf0000000;
    pub const BIT_WIDTH: u8 = 4;
    pub const RESET_VALUE: u32 = 0xf;
    pub const WRITABLE: bool = false;
}
/// DC/DC during recharge in powerdown.
///
/// 0: Use the DC/DC during recharge in powerdown.
///
/// 1: Do not use the DC/DC during recharge in powerdown (default).
///
///
///
/// NOTE! The DriverLib function SysCtrl_DCDC_VoltageConditionalControl() must be called regularly to apply this field (handled automatically if using TI RTOS!).
pub mod DCDC_RECHARGE {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 27..=27;
    pub const BIT_MASK: u32 = 0x08000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = false;
}
/// DC/DC in active mode.
///
/// 0: Use the DC/DC during active mode.
///
/// 1: Do not use the DC/DC during active mode (default).
///
///
///
/// NOTE! The DriverLib function SysCtrl_DCDC_VoltageConditionalControl() must be called regularly to apply this field (handled automatically if using TI RTOS!).
pub mod DCDC_ACTIVE {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 26..=26;
    pub const BIT_MASK: u32 = 0x04000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = false;
}
/// Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior.
pub mod VDDR_EXT_LOAD {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 25..=25;
    pub const BIT_MASK: u32 = 0x02000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = false;
}
/// VDDS BOD level.
///
/// 0: VDDS BOD level is 2.0 V (necessary for maximum PA output power on CC13x0).
///
/// 1: VDDS BOD level is 1.8 V (or 1.7 V for external regulator mode) (default).
pub mod VDDS_BOD_LEVEL {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 24..=24;
    pub const BIT_MASK: u32 = 0x01000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = false;
}
/// Select source for SCLK_LF.
pub mod SCLK_LF_OPTION {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 22..=23;
    pub const BIT_MASK: u32 = 0x00c00000;
    pub const BIT_WIDTH: u8 = 2;
    pub const RESET_VALUE: u32 = 0x3;
    pub const WRITABLE: bool = false;
    pub use self::Named as E;
    pub mod Named {
        /// Low frequency RCOSC (default)
        pub const RCOSC_LF: u32 = 3;
        /// 32.768kHz low frequency XOSC
        pub const XOSC_LF: u32 = 2;
        /// External low frequency clock on DIO defined by EXT_LF_CLK.DIO. The RTC tick speed AON_RTC:SUBSECINC is updated to EXT_LF_CLK.RTC_INCREMENT (done in the trimDevice() xxWare boot function). External clock must always be running when the chip is in standby for VDDR recharge timing.
        pub const EXTERNAL_LF: u32 = 1;
        /// 31.25kHz clock derived from 24MHz XOSC (dividing by 768 in HW). The RTC tick speed \[AON_RTC.SUBSECINC.*\] is updated to 0x8637BD, corresponding to a 31.25kHz clock (done in the trimDevice() xxWare boot function). Standby power mode is not supported when using this clock source.
        pub const XOSC_HF_DLF: u32 = 0;
    }
}
/// 0x1: VDDR_TRIM_SLEEP_DELTA is not temperature compensated
///
/// 0x0: RTOS/driver temperature compensates VDDR_TRIM_SLEEP_DELTA every time standby mode is entered. This improves low-temperature RCOSC_LF frequency stability in standby mode.
///
///
///
/// When temperature compensation is performed, the delta is calculates this way:
///
/// Delta = max (delta, min(8, floor(62-temp)/8))
///
/// Here, delta is given by VDDR_TRIM_SLEEP_DELTA, and temp is the current temperature in degrees C.
pub mod VDDR_TRIM_SLEEP_TC {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 21..=21;
    pub const BIT_MASK: u32 = 0x00200000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = false;
}
/// Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior.
pub mod RTC_COMP {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 20..=20;
    pub const BIT_MASK: u32 = 0x00100000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = false;
}
/// Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior.
pub mod XOSC_FREQ {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 18..=19;
    pub const BIT_MASK: u32 = 0x000c0000;
    pub const BIT_WIDTH: u8 = 2;
    pub const RESET_VALUE: u32 = 0x3;
    pub const WRITABLE: bool = false;
    pub use self::Named as E;
    pub mod Named {
        /// 24 MHz XOSC_HF
        pub const _24M: u32 = 3;
        /// 48 MHz XOSC_HF
        pub const _48M: u32 = 2;
        /// HPOSC
        pub const HPOSC: u32 = 1;
    }
}
/// Enable modification (delta) to XOSC cap-array. Value specified in XOSC_CAPARRAY_DELTA.
///
/// 0: Apply cap-array delta
///
/// 1: Do not apply cap-array delta (default)
pub mod XOSC_CAP_MOD {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 17..=17;
    pub const BIT_MASK: u32 = 0x00020000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = false;
}
/// Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior.
pub mod HF_COMP {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 16..=16;
    pub const BIT_MASK: u32 = 0x00010000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = false;
}
/// Signed 8-bit value, directly modifying trimmed XOSC cap-array step value. Enabled by XOSC_CAP_MOD.
pub mod XOSC_CAPARRAY_DELTA {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 8..=15;
    pub const BIT_MASK: u32 = 0x0000ff00;
    pub const BIT_WIDTH: u8 = 8;
    pub const RESET_VALUE: u32 = 0xff;
    pub const WRITABLE: bool = false;
}
/// Unsigned 8-bit integer, representing the minimum decoupling capacitance (worst case) on VDDR, in units of 100nF. This should take into account capacitor tolerance and voltage dependent capacitance variation. This bit affects the recharge period calculation when going into powerdown or standby.
///
///  
///
/// NOTE! If using the following functions this field must be configured (used by TI RTOS):
///
/// SysCtrlSetRechargeBeforePowerDown() SysCtrlAdjustRechargeAfterPowerDown()
pub mod VDDR_CAP {
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
        pub VDDR_CAP: B8,
        pub XOSC_CAPARRAY_DELTA: B8,
        pub HF_COMP: B1,
        pub XOSC_CAP_MOD: B1,
        pub XOSC_FREQ: B2,
        pub RTC_COMP: B1,
        pub VDDR_TRIM_SLEEP_TC: B1,
        pub SCLK_LF_OPTION: B2,
        pub VDDS_BOD_LEVEL: B1,
        pub VDDR_EXT_LOAD: B1,
        pub DCDC_ACTIVE: B1,
        pub DCDC_RECHARGE: B1,
        pub VDDR_TRIM_SLEEP_DELTA: B4,
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
                warn!(target: "cc2650_constants::CCFG::MODE_CONF", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::CCFG::MODE_CONF",
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
