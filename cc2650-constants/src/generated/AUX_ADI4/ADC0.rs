use cmemu_common::Address;

pub const DISPLAY: &str = "ADC0";
pub const OFFSET: u32 = 0x8;
/// 0x400cb008
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 8;
pub const RESET_VALUE: u32 = 0x00;
pub const RESET_MASK: u32 = 0xffffffff;
/// ADC Sampling mode:
///
///
///
/// 0: Synchronous mode
///
/// 1: Asynchronous mode
///
///
///
/// The ADC does a sample-and-hold before conversion. In synchronous mode the sampling starts when the ADC clock detects a rising edge on the trigger signal. Jitter/uncertainty will be inferred in the detection if the trigger signal originates from a domain that is asynchronous to the ADC clock. SMPL_CYCLE_EXP  determines the the duration of sampling.
///
/// Conversion starts immediately after sampling ends.
///
///
///
/// In asynchronous mode the sampling is continuous when enabled. Sampling ends and conversion starts immediately with the rising edge of the trigger signal. Sampling restarts when the conversion has finished.
///
/// Asynchronous mode is useful when it is important to avoid jitter in the sampling instant of an externally driven signal
pub mod SMPL_MODE {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 7..=7;
    pub const BIT_MASK: u32 = 0x80;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Controls the sampling duration before conversion when the ADC is operated in synchronous mode (SMPL_MODE = 0). The setting has no effect in asynchronous mode. The sampling duration is given as 2^(SMPL_CYCLE_EXP + 1) / 6 us.
pub mod SMPL_CYCLE_EXP {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 3..=6;
    pub const BIT_MASK: u32 = 0x78;
    pub const BIT_WIDTH: u8 = 4;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// 65536x 6 MHz clock periods = 10.9ms
        pub const _10P9_MS: u32 = 15;
        /// 32768x 6 MHz clock periods = 5.46ms
        pub const _5P46_MS: u32 = 14;
        /// 16384x 6 MHz clock periods = 2.73ms
        pub const _2P73_MS: u32 = 13;
        /// 8192x 6 MHz clock periods = 1.37ms
        pub const _1P37_MS: u32 = 12;
        /// 4096x 6 MHz clock periods = 682us
        pub const _682_US: u32 = 11;
        /// 2048x 6 MHz clock periods = 341us
        pub const _341_US: u32 = 10;
        /// 1024x 6 MHz clock periods = 170us
        pub const _170_US: u32 = 9;
        /// 512x 6 MHz clock periods = 85.3us
        pub const _85P3_US: u32 = 8;
        /// 256x 6 MHz clock periods = 42.6us
        pub const _42P6_US: u32 = 7;
        /// 128x 6 MHz clock periods = 21.3us
        pub const _21P3_US: u32 = 6;
        /// 64x 6 MHz clock periods = 10.6us
        pub const _10P6_US: u32 = 5;
        /// 32x 6 MHz clock periods = 5.3us
        pub const _5P3_US: u32 = 4;
        /// 16x 6 MHz clock periods = 2.7us
        pub const _2P7_US: u32 = 3;
    }
}
/// Reset ADC digital subchip, active low. ADC must be reset every time it is reconfigured.
///
///
///
/// 0: Reset
///
/// 1: Normal operation
pub mod RESET_N {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 1..=1;
    pub const BIT_MASK: u32 = 0x02;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// ADC Enable
///
///
///
/// 0: Disable
///
/// 1: Enable
pub mod EN {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 0..=0;
    pub const BIT_MASK: u32 = 0x01;
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
        pub EN: B1,
        pub RESET_N: B1,
        pub reserved_2_3: B1,
        pub SMPL_CYCLE_EXP: B4,
        pub SMPL_MODE: B1,
        pub reserved_8_32: B24,
    }

    impl HwRegister for Register {
        const RESERVED_BITS_MASK: u32 = 0xffffff04;
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
                warn!(target: "cc2650_constants::AUX_ADI4::ADC0", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::AUX_ADI4::ADC0",
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
