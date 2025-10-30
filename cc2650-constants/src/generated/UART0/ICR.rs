use cmemu_common::Address;

pub const DISPLAY: &str = "ICR";
pub const OFFSET: u32 = 0x44;
/// 0x40001044
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
/// Overrun error interrupt clear:
///
/// Writing 1 to this field clears the overrun error interrupt (RIS.OERIS). Writing 0 has no effect.
pub mod OEIC {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 10..=10;
    pub const BIT_MASK: u32 = 0x00000400;
    pub const BIT_WIDTH: u8 = 1;
    pub const WRITABLE: bool = true;
}
/// Break error interrupt clear:
///
/// Writing 1 to this field clears the break error interrupt (RIS.BERIS). Writing 0 has no effect.
pub mod BEIC {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 9..=9;
    pub const BIT_MASK: u32 = 0x00000200;
    pub const BIT_WIDTH: u8 = 1;
    pub const WRITABLE: bool = true;
}
/// Parity error interrupt clear:
///
/// Writing 1 to this field clears the parity error interrupt (RIS.PERIS). Writing 0 has no effect.
pub mod PEIC {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 8..=8;
    pub const BIT_MASK: u32 = 0x00000100;
    pub const BIT_WIDTH: u8 = 1;
    pub const WRITABLE: bool = true;
}
/// Framing error interrupt clear:
///
/// Writing 1 to this field clears the framing error interrupt (RIS.FERIS). Writing 0 has no effect.
pub mod FEIC {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 7..=7;
    pub const BIT_MASK: u32 = 0x00000080;
    pub const BIT_WIDTH: u8 = 1;
    pub const WRITABLE: bool = true;
}
/// Receive timeout interrupt clear:
///
/// Writing 1 to this field clears the receive timeout interrupt (RIS.RTRIS). Writing 0 has no effect.
pub mod RTIC {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 6..=6;
    pub const BIT_MASK: u32 = 0x00000040;
    pub const BIT_WIDTH: u8 = 1;
    pub const WRITABLE: bool = true;
}
/// Transmit interrupt clear:
///
/// Writing 1 to this field clears the transmit interrupt (RIS.TXRIS). Writing 0 has no effect.
pub mod TXIC {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 5..=5;
    pub const BIT_MASK: u32 = 0x00000020;
    pub const BIT_WIDTH: u8 = 1;
    pub const WRITABLE: bool = true;
}
/// Receive interrupt clear:
///
/// Writing 1 to this field clears the receive interrupt (RIS.RXRIS). Writing 0 has no effect.
pub mod RXIC {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 4..=4;
    pub const BIT_MASK: u32 = 0x00000010;
    pub const BIT_WIDTH: u8 = 1;
    pub const WRITABLE: bool = true;
}
/// Clear to Send (CTS) modem interrupt clear:
///
/// Writing 1 to this field clears the clear to send interrupt (RIS.CTSRMIS). Writing 0 has no effect.
pub mod CTSMIC {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 1..=1;
    pub const BIT_MASK: u32 = 0x00000002;
    pub const BIT_WIDTH: u8 = 1;
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
        pub reserved_0_1: B1,
        pub CTSMIC: B1,
        pub reserved_2_4: B2,
        pub RXIC: B1,
        pub TXIC: B1,
        pub RTIC: B1,
        pub FEIC: B1,
        pub PEIC: B1,
        pub BEIC: B1,
        pub OEIC: B1,
        pub reserved_11_32: B21,
    }

    impl HwRegister for Register {
        const RESERVED_BITS_MASK: u32 = 0xfffff80d;
        const READ_ONLY_BITS_MASK: u32 = 0x00000000;
        const WRITE_ONLY_BITS_MASK: u32 = 0x000007f2;

        fn read(&self) -> u32 {
            u32::from(self.content)
        }

        fn mutate(&mut self, word: u32) {
            let old_val: u32 = self.read();
            let mut new_val: u32 = word;

            // Check if modifies reserved bits
            if old_val & Self::RESERVED_BITS_MASK != new_val & Self::RESERVED_BITS_MASK {
                warn!(target: "cc2650_constants::UART0::ICR", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::UART0::ICR",
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
