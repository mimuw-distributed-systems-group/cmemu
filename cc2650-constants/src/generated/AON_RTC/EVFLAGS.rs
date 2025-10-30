use cmemu_common::Address;

pub const DISPLAY: &str = "EVFLAGS";
pub const OFFSET: u32 = 0x4;
/// 0x40092004
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
pub const RESET_VALUE: u32 = 0x00000000;
pub const RESET_MASK: u32 = 0xffffffff;
/// Channel 2 event flag, set when CHCTL.CH2_EN = 1 and the RTC value matches or passes the CH2CMP value.
///
///
///
/// An event will be scheduled to occur as soon as possible when writing to CH2CMP provided that the channel is enabled and the new value matches any time between next  RTC value and 1 second in the past
///
///
///
/// Writing 1 clears this flag. Note that a new event can not occur on this channel in first 2 SCLK_LF cycles after a clearance.
///
///
///
/// AUX_SCE can read the flag through AUX_WUC:WUEVFLAGS.AON_RTC_CH2 and clear it using AUX_WUC:WUEVCLR.AON_RTC_CH2.
pub mod CH2 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 16..=16;
    pub const BIT_MASK: u32 = 0x00010000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Channel 1 event flag, set when CHCTL.CH1_EN = 1 and one of the following:
///
/// - CHCTL.CH1_CAPT_EN = 0 and the RTC value matches or passes the CH1CMP value.
///
/// - CHCTL.CH1_CAPT_EN = 1 and capture occurs.
///
///
///
/// An event will be scheduled to occur as soon as possible when writing to CH1CMP provided that the channel is enabled, in compare mode and the new value matches any time between next  RTC value and 1 second in the past.
///
///
///
/// Writing 1 clears this flag. Note that a new event can not occur on this channel in first 2 SCLK_LF cycles after a clearance.
pub mod CH1 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 8..=8;
    pub const BIT_MASK: u32 = 0x00000100;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Channel 0 event flag, set when CHCTL.CH0_EN = 1 and the RTC value matches or passes the CH0CMP value.
///
///
///
/// An event will be scheduled to occur as soon as possible when writing to CH0CMP provided that the channels is enabled and the new value matches any time between next  RTC value and 1 second in the past.
///
///
///
/// Writing 1 clears this flag. Note that a new event can not occur on this channel in first 2 SCLK_LF cycles after a clearance.
pub mod CH0 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 0..=0;
    pub const BIT_MASK: u32 = 0x00000001;
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
        pub CH0: B1,
        pub reserved_1_8: B7,
        pub CH1: B1,
        pub reserved_9_16: B7,
        pub CH2: B1,
        pub reserved_17_32: B15,
    }

    impl HwRegister for Register {
        const RESERVED_BITS_MASK: u32 = 0xfffefefe;
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
                warn!(target: "cc2650_constants::AON_RTC::EVFLAGS", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::AON_RTC::EVFLAGS",
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
