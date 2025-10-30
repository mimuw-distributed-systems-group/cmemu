use cmemu_common::Address;

pub const DISPLAY: &str = "STATUS";
pub const OFFSET: u32 = 0x0;
/// 0x40020000
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
pub const RESET_VALUE: u32 = 0x001f0000;
pub const RESET_MASK: u32 = 0xffffffff;
/// 0x0: Controller does not include the integration test logic
///
/// 0x1: Controller includes the integration test logic
///
/// 0x2: Undefined
///
/// ...
///
/// 0xF: Undefined
pub mod TEST {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 28..=31;
    pub const BIT_MASK: u32 = 0xf0000000;
    pub const BIT_WIDTH: u8 = 4;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Register value returns number of available uDMA channels minus one. For example a read out value of:
///
///
///
/// 0x00: Show that the controller is configured to use 1 uDMA channel
///
/// 0x01: Shows that the controller is configured to use 2 uDMA channels
///
/// ...
///
/// 0x1F: Shows that the controller is configured to use 32 uDMA channels (32-1=31=0x1F)
pub mod TOTALCHANNELS {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 16..=20;
    pub const BIT_MASK: u32 = 0x001f0000;
    pub const BIT_WIDTH: u8 = 5;
    pub const RESET_VALUE: u32 = 0x1f;
    pub const WRITABLE: bool = false;
}
/// Current state of the control state machine. State can be one of the following:
///
///
///
/// 0x0: Idle
///
/// 0x1: Reading channel controller data
///
/// 0x2: Reading source data end pointer
///
/// 0x3: Reading destination data end pointer
///
/// 0x4: Reading source data
///
/// 0x5: Writing destination data
///
/// 0x6: Waiting for uDMA request to clear
///
/// 0x7: Writing channel controller data
///
/// 0x8: Stalled
///
/// 0x9: Done
///
/// 0xA: Peripheral scatter-gather transition
///
/// 0xB: Undefined
///
/// ...
///
/// 0xF: Undefined.
pub mod STATE {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 4..=7;
    pub const BIT_MASK: u32 = 0x000000f0;
    pub const BIT_WIDTH: u8 = 4;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Shows the enable status of the controller as configured by CFG.MASTERENABLE:
///
///
///
/// 0: Controller is disabled
///
/// 1: Controller is enabled
pub mod MASTERENABLE {
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
        pub MASTERENABLE: B1,
        pub reserved_1_4: B3,
        pub STATE: B4,
        pub reserved_8_16: B8,
        pub TOTALCHANNELS: B5,
        pub reserved_21_28: B7,
        pub TEST: B4,
    }

    impl HwRegister for Register {
        const RESERVED_BITS_MASK: u32 = 0x0fe0ff0e;
        const READ_ONLY_BITS_MASK: u32 = 0xf01f00f1;
        const WRITE_ONLY_BITS_MASK: u32 = 0x00000000;

        fn read(&self) -> u32 {
            u32::from(self.content)
        }

        fn mutate(&mut self, word: u32) {
            let old_val: u32 = self.read();
            let mut new_val: u32 = word;

            // Check if modifies reserved bits
            if old_val & Self::RESERVED_BITS_MASK != new_val & Self::RESERVED_BITS_MASK {
                warn!(target: "cc2650_constants::UDMA0::STATUS", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::UDMA0::STATUS",
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
