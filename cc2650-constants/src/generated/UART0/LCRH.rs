use cmemu_common::Address;

pub const DISPLAY: &str = "LCRH";
pub const OFFSET: u32 = 0x2c;
/// 0x4000102c
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
pub const RESET_VALUE: u32 = 0x00000000;
pub const RESET_MASK: u32 = 0xffffffff;
/// UART Stick Parity Select:
///
///
///
/// 0: Stick parity is disabled
///
/// 1: The parity bit is transmitted and checked as invert of EPS field (i.e. the parity bit is transmitted and checked as 1 when EPS = 0).
///
///
///
/// This bit has no effect when PEN disables parity checking and generation.
pub mod SPS {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 7..=7;
    pub const BIT_MASK: u32 = 0x00000080;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// UART Word Length:
///
/// These bits indicate the number of data bits transmitted or received in a frame.
pub mod WLEN {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 5..=6;
    pub const BIT_MASK: u32 = 0x00000060;
    pub const BIT_WIDTH: u8 = 2;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// Word Length 8 bits
        pub const _8: u32 = 3;
        /// Word Length 7 bits
        pub const _7: u32 = 2;
        /// Word Length 6 bits
        pub const _6: u32 = 1;
        /// Word Length 5 bits
        pub const _5: u32 = 0;
    }
}
/// UART Enable FIFOs
pub mod FEN {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 4..=4;
    pub const BIT_MASK: u32 = 0x00000010;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// Transmit and receive FIFO buffers are enabled (FIFO mode)
        pub const EN: u32 = 1;
        /// FIFOs are disabled (character mode) that is, the FIFOs become 1-byte-deep holding registers.
        pub const DIS: u32 = 0;
    }
}
/// UART Two Stop Bits Select:
///
/// If this bit is set to 1, two stop bits are transmitted at the end of the frame. The receive logic does not check for two stop bits being received.
pub mod STP2 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 3..=3;
    pub const BIT_MASK: u32 = 0x00000008;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// UART Even Parity Select
pub mod EPS {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 2..=2;
    pub const BIT_MASK: u32 = 0x00000004;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// Even parity: The UART generates or checks for an even number of 1s in the data and parity bits.
        pub const EVEN: u32 = 1;
        /// Odd parity: The UART generates or checks for an odd number of 1s in the data and parity bits.
        pub const ODD: u32 = 0;
    }
}
/// UART Parity Enable
///
/// This bit controls generation and checking of parity bit.
pub mod PEN {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 1..=1;
    pub const BIT_MASK: u32 = 0x00000002;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// Parity checking and generation is enabled.
        pub const EN: u32 = 1;
        /// Parity is disabled and no parity bit is added to the data frame
        pub const DIS: u32 = 0;
    }
}
/// UART Send Break
///
/// If this bit is set to 1, a low-level is continually output on the UARTTXD output pin, after completing transmission of the current character. For the proper execution of the break command, the
///
/// software must set this bit for at least two complete frames. For normal use, this bit must be cleared to 0.
pub mod BRK {
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
        pub BRK: B1,
        pub PEN: B1,
        pub EPS: B1,
        pub STP2: B1,
        pub FEN: B1,
        pub WLEN: B2,
        pub SPS: B1,
        pub reserved_8_32: B24,
    }

    impl HwRegister for Register {
        const RESERVED_BITS_MASK: u32 = 0xffffff00;
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
                warn!(target: "cc2650_constants::UART0::LCRH", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::UART0::LCRH",
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
