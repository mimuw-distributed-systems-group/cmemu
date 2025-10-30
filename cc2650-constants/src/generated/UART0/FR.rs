use cmemu_common::Address;

pub const DISPLAY: &str = "FR";
pub const OFFSET: u32 = 0x18;
/// 0x40001018
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
/// UART Transmit FIFO Empty:
///
/// The meaning of this bit depends on the state of LCRH.FEN .
///
///   - If the FIFO is disabled, this bit is set when the transmit holding register is empty.
///
///   - If the FIFO is enabled, this bit is set when the transmit FIFO is empty.
///
/// This bit does not indicate if there is data in the transmit shift register.
pub mod TXFE {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 7..=7;
    pub const BIT_MASK: u32 = 0x00000080;
    pub const BIT_WIDTH: u8 = 1;
    pub const WRITABLE: bool = false;
}
/// UART Receive FIFO Full:
///
/// The meaning of this bit depends on the state of LCRH.FEN.
///
///   - If the FIFO is disabled, this bit is set when the receive holding register is full.
///
///   - If the FIFO is enabled, this bit is set when the receive FIFO is full.
pub mod RXFF {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 6..=6;
    pub const BIT_MASK: u32 = 0x00000040;
    pub const BIT_WIDTH: u8 = 1;
    pub const WRITABLE: bool = false;
}
/// UART Transmit FIFO Full:
///
/// Transmit FIFO full. The meaning of this bit depends on the state of LCRH.FEN.
///
///   - If the FIFO is disabled, this bit is set when the transmit holding register is full.
///
///   - If the FIFO is enabled, this bit is set when the transmit FIFO is full.
pub mod TXFF {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 5..=5;
    pub const BIT_MASK: u32 = 0x00000020;
    pub const BIT_WIDTH: u8 = 1;
    pub const WRITABLE: bool = false;
}
/// UART Receive FIFO Empty:
///
/// Receive FIFO empty. The meaning of this bit depends on the state of LCRH.FEN.
///
///   - If the FIFO is disabled, this bit is set when the receive holding register is empty.
///
///   - If the FIFO is enabled, this bit is set when the receive FIFO is empty.
pub mod RXFE {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 4..=4;
    pub const BIT_MASK: u32 = 0x00000010;
    pub const BIT_WIDTH: u8 = 1;
    pub const WRITABLE: bool = false;
}
/// UART Busy:
///
/// If this bit is set to 1, the UART is busy transmitting data. This bit remains set until the complete byte, including all the stop bits, has been sent from the shift register.
///
/// This bit is set as soon as the transmit FIFO becomes non-empty, regardless of whether the UART is enabled or not.
pub mod BUSY {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 3..=3;
    pub const BIT_MASK: u32 = 0x00000008;
    pub const BIT_WIDTH: u8 = 1;
    pub const WRITABLE: bool = false;
}
/// Clear To Send:
///
/// This bit is the complement of the active-low UART CTS input pin.
///
/// That is, the bit is 1 when CTS input pin is LOW.
pub mod CTS {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 0..=0;
    pub const BIT_MASK: u32 = 0x00000001;
    pub const BIT_WIDTH: u8 = 1;
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
        pub CTS: B1,
        pub reserved_1_3: B2,
        pub BUSY: B1,
        pub RXFE: B1,
        pub TXFF: B1,
        pub RXFF: B1,
        pub TXFE: B1,
        pub reserved_8_32: B24,
    }

    impl HwRegister for Register {
        const RESERVED_BITS_MASK: u32 = 0xffffff06;
        const READ_ONLY_BITS_MASK: u32 = 0x000000f9;
        const WRITE_ONLY_BITS_MASK: u32 = 0x00000000;

        fn read(&self) -> u32 {
            u32::from(self.content)
        }

        fn mutate(&mut self, word: u32) {
            let old_val: u32 = self.read();
            let mut new_val: u32 = word;

            // Check if modifies reserved bits
            if old_val & Self::RESERVED_BITS_MASK != new_val & Self::RESERVED_BITS_MASK {
                warn!(target: "cc2650_constants::UART0::FR", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::UART0::FR",
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
