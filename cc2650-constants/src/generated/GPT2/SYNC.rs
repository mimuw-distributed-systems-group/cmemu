use cmemu_common::Address;

pub const DISPLAY: &str = "SYNC";
pub const OFFSET: u32 = 0x10;
/// 0x40012010
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
pub const RESET_VALUE: u32 = 0x00000000;
pub const RESET_MASK: u32 = 0xffffffff;
/// Synchronize GPT Timer 3.
pub mod SYNC3 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 6..=7;
    pub const BIT_MASK: u32 = 0x000000c0;
    pub const BIT_WIDTH: u8 = 2;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// A timeout event for both Timer A and Timer B of GPT3 is triggered
        pub const BOTH: u32 = 3;
        /// A timeout event for Timer B of GPT3 is triggered
        pub const TIMERB: u32 = 2;
        /// A timeout event for Timer A of GPT3 is triggered
        pub const TIMERA: u32 = 1;
        /// No Sync. GPT3 is not affected.
        pub const NOSYNC: u32 = 0;
    }
}
/// Synchronize GPT Timer 2.
pub mod SYNC2 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 4..=5;
    pub const BIT_MASK: u32 = 0x00000030;
    pub const BIT_WIDTH: u8 = 2;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// A timeout event for both Timer A and Timer B of GPT2 is triggered
        pub const BOTH: u32 = 3;
        /// A timeout event for Timer B of GPT2 is triggered
        pub const TIMERB: u32 = 2;
        /// A timeout event for Timer A of GPT2 is triggered
        pub const TIMERA: u32 = 1;
        /// No Sync. GPT2 is not affected.
        pub const NOSYNC: u32 = 0;
    }
}
/// Synchronize GPT Timer 1
pub mod SYNC1 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 2..=3;
    pub const BIT_MASK: u32 = 0x0000000c;
    pub const BIT_WIDTH: u8 = 2;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// A timeout event for both Timer A and Timer B of GPT1 is triggered
        pub const BOTH: u32 = 3;
        /// A timeout event for Timer B of GPT1 is triggered
        pub const TIMERB: u32 = 2;
        /// A timeout event for Timer A of GPT1 is triggered
        pub const TIMERA: u32 = 1;
        /// No Sync. GPT1 is not affected.
        pub const NOSYNC: u32 = 0;
    }
}
/// Synchronize GPT Timer 0
pub mod SYNC0 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 0..=1;
    pub const BIT_MASK: u32 = 0x00000003;
    pub const BIT_WIDTH: u8 = 2;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// A timeout event for both Timer A and Timer B of GPT0 is triggered
        pub const BOTH: u32 = 3;
        /// A timeout event for Timer B of GPT0 is triggered
        pub const TIMERB: u32 = 2;
        /// A timeout event for Timer A of GPT0 is triggered
        pub const TIMERA: u32 = 1;
        /// No Sync. GPT0 is not affected.
        pub const NOSYNC: u32 = 0;
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
        pub SYNC0: B2,
        pub SYNC1: B2,
        pub SYNC2: B2,
        pub SYNC3: B2,
        pub reserved_8_32: B24,
    }

    impl HwRegister for Register {
        const RESERVED_BITS_MASK: u32 = 0xffffff00;
        const READ_ONLY_BITS_MASK: u32 = 0x00000000;
        const WRITE_ONLY_BITS_MASK: u32 = 0x000000ff;

        fn read(&self) -> u32 {
            u32::from(self.content)
        }

        fn mutate(&mut self, word: u32) {
            let old_val: u32 = self.read();
            let mut new_val: u32 = word;

            // Check if modifies reserved bits
            if old_val & Self::RESERVED_BITS_MASK != new_val & Self::RESERVED_BITS_MASK {
                warn!(target: "cc2650_constants::GPT2::SYNC", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::GPT2::SYNC",
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
