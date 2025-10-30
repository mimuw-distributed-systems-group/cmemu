use cmemu_common::Address;

pub const DISPLAY: &str = "CR0";
pub const OFFSET: u32 = 0x0;
/// 0x40000000
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
pub const RESET_VALUE: u32 = 0x00000000;
pub const RESET_MASK: u32 = 0xffffffff;
/// Serial clock rate:
///
/// This is used to generate the transmit and receive bit rate of the SSI. The bit rate is
///
/// (SSI's clock frequency)/((SCR+1)*CPSR.CPSDVSR).
///
/// SCR is a value from 0-255.
pub mod SCR {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 8..=15;
    pub const BIT_MASK: u32 = 0x0000ff00;
    pub const BIT_WIDTH: u8 = 8;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// CLKOUT phase (Motorola SPI frame format only)
///
/// This bit selects the clock edge that captures data and enables it to change state. It
///
/// has the most impact on the first bit transmitted by either permitting or not permitting a clock transition before the first data capture edge.
pub mod SPH {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 7..=7;
    pub const BIT_MASK: u32 = 0x00000080;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// Data is captured on the second clock edge transition.
        pub const _2ND_CLK_EDGE: u32 = 1;
        /// Data is captured on the first clock edge transition.
        pub const _1ST_CLK_EDGE: u32 = 0;
    }
}
/// CLKOUT polarity (Motorola SPI frame format only)
pub mod SPO {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 6..=6;
    pub const BIT_MASK: u32 = 0x00000040;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// SSI produces a steady state HIGH value on the CLKOUT pin when data is not being transferred.
        pub const HIGH: u32 = 1;
        /// SSI produces a steady state LOW value on the
        ///
        /// CLKOUT pin when data is not being transferred.
        pub const LOW: u32 = 0;
    }
}
/// Frame format.
///
/// The supported frame formats are Motorola SPI, TI synchronous serial and National Microwire.
///
/// Value 0'b11 is reserved and shall not be used.
pub mod FRF {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 4..=5;
    pub const BIT_MASK: u32 = 0x00000030;
    pub const BIT_WIDTH: u8 = 2;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// National Microwire frame format
        pub const NATIONAL_MICROWIRE: u32 = 2;
        /// TI synchronous serial frame format
        pub const TI_SYNC_SERIAL: u32 = 1;
        /// Motorola SPI frame format
        pub const MOTOROLA_SPI: u32 = 0;
    }
}
/// Data Size Select.
///
/// Values 0b0000, 0b0001, 0b0010 are reserved and shall not be used.
pub mod DSS {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 0..=3;
    pub const BIT_MASK: u32 = 0x0000000f;
    pub const BIT_WIDTH: u8 = 4;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// 16-bit data
        pub const _16_BIT: u32 = 15;
        /// 15-bit data
        pub const _15_BIT: u32 = 14;
        /// 14-bit data
        pub const _14_BIT: u32 = 13;
        /// 13-bit data
        pub const _13_BIT: u32 = 12;
        /// 12-bit data
        pub const _12_BIT: u32 = 11;
        /// 11-bit data
        pub const _11_BIT: u32 = 10;
        /// 10-bit data
        pub const _10_BIT: u32 = 9;
        /// 9-bit data
        pub const _9_BIT: u32 = 8;
        /// 8-bit data
        pub const _8_BIT: u32 = 7;
        /// 7-bit data
        pub const _7_BIT: u32 = 6;
        /// 6-bit data
        pub const _6_BIT: u32 = 5;
        /// 5-bit data
        pub const _5_BIT: u32 = 4;
        /// 4-bit data
        pub const _4_BIT: u32 = 3;
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
        pub DSS: B4,
        pub FRF: B2,
        pub SPO: B1,
        pub SPH: B1,
        pub SCR: B8,
        pub reserved_16_32: B16,
    }

    impl HwRegister for Register {
        const RESERVED_BITS_MASK: u32 = 0xffff0000;
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
                warn!(target: "cc2650_constants::SSI0::CR0", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::SSI0::CR0",
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
