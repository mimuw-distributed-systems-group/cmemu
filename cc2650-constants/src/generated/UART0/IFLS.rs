use cmemu_common::Address;

pub const DISPLAY: &str = "IFLS";
pub const OFFSET: u32 = 0x34;
/// 0x40001034
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
pub const RESET_VALUE: u32 = 0x00000012;
pub const RESET_MASK: u32 = 0xffffffff;
/// Receive interrupt FIFO level select:
///
/// This field sets the trigger points for the receive interrupt. Values 0b101-0b111 are reserved.
pub mod RXSEL {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 3..=5;
    pub const BIT_MASK: u32 = 0x00000038;
    pub const BIT_WIDTH: u8 = 3;
    pub const RESET_VALUE: u32 = 0x2;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// Receive FIFO becomes >= 7/8 full
        pub const _7_8: u32 = 4;
        /// Receive FIFO becomes >= 3/4 full
        pub const _6_8: u32 = 3;
        /// Receive FIFO becomes >= 1/2 full
        pub const _4_8: u32 = 2;
        /// Receive FIFO becomes >= 1/4 full
        pub const _2_8: u32 = 1;
        /// Receive FIFO becomes >= 1/8 full
        pub const _1_8: u32 = 0;
    }
}
/// Transmit interrupt FIFO level select:
///
/// This field sets the trigger points for the transmit interrupt. Values 0b101-0b111 are reserved.
pub mod TXSEL {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 0..=2;
    pub const BIT_MASK: u32 = 0x00000007;
    pub const BIT_WIDTH: u8 = 3;
    pub const RESET_VALUE: u32 = 0x2;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// Transmit FIFO becomes <= 7/8 full
        pub const _7_8: u32 = 4;
        /// Transmit FIFO becomes <= 3/4 full
        pub const _6_8: u32 = 3;
        /// Transmit FIFO becomes <= 1/2 full
        pub const _4_8: u32 = 2;
        /// Transmit FIFO becomes <= 1/4 full
        pub const _2_8: u32 = 1;
        /// Transmit FIFO becomes <= 1/8 full
        pub const _1_8: u32 = 0;
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
        pub TXSEL: B3,
        pub RXSEL: B3,
        pub reserved_6_32: B26,
    }

    impl HwRegister for Register {
        const RESERVED_BITS_MASK: u32 = 0xffffffc0;
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
                warn!(target: "cc2650_constants::UART0::IFLS", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::UART0::IFLS",
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
