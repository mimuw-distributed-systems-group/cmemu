use cmemu_common::Address;

pub const DISPLAY: &str = "USER_ID";
pub const OFFSET: u32 = 0x294;
/// 0x50001294
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
/// Field used to distinguish revisions of the device.
pub mod PG_REV {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 28..=31;
    pub const BIT_MASK: u32 = 0xf0000000;
    pub const BIT_WIDTH: u8 = 4;
    pub const WRITABLE: bool = false;
}
/// Version number.
///
///
///
/// 0x0: Bits \[25:12\] of this register has the stated meaning.
///
///
///
/// Any other setting indicate a different encoding of these bits.
pub mod VER {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 26..=27;
    pub const BIT_MASK: u32 = 0x0c000000;
    pub const BIT_WIDTH: u8 = 2;
    pub const WRITABLE: bool = false;
}
/// Sequence.
///
///
///
/// Used to differentiate between marketing/orderable product where other fields of USER_ID is the same (temp range, flash size, voltage range etc)
pub mod SEQUENCE {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 19..=22;
    pub const BIT_MASK: u32 = 0x00780000;
    pub const BIT_WIDTH: u8 = 4;
    pub const WRITABLE: bool = false;
}
/// Package type.
///
///
///
/// 0x0: 4x4mm QFN (RHB) package
///
/// 0x1: 5x5mm QFN (RSM) package
///
/// 0x2: 7x7mm QFN (RGZ) package
///
/// 0x3: Wafer sale package (naked die)
///
/// 0x4: 2.7x2.7mm WCSP (YFV)
///
/// 0x5: 7x7mm QFN package with Wettable Flanks
///
///
///
/// Other values are reserved for future use.
///
/// Packages available for a specific device are shown in the device datasheet.
pub mod PKG {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 16..=18;
    pub const BIT_MASK: u32 = 0x00070000;
    pub const BIT_WIDTH: u8 = 3;
    pub const WRITABLE: bool = false;
}
/// Protocols supported.
///
///
///
/// 0x1: BLE
///
/// 0x2: RF4CE
///
/// 0x4: Zigbee/6lowpan
///
/// 0x8: Proprietary
///
///
///
/// More than one protocol can be supported on same device - values above are then combined.
pub mod PROTOCOL {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 12..=15;
    pub const BIT_MASK: u32 = 0x0000f000;
    pub const BIT_WIDTH: u8 = 4;
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
        pub reserved_0_12: B12,
        pub PROTOCOL: B4,
        pub PKG: B3,
        pub SEQUENCE: B4,
        pub reserved_23_26: B3,
        pub VER: B2,
        pub PG_REV: B4,
    }

    impl HwRegister for Register {
        const RESERVED_BITS_MASK: u32 = 0x03800fff;
        const READ_ONLY_BITS_MASK: u32 = 0xfc7ff000;
        const WRITE_ONLY_BITS_MASK: u32 = 0x00000000;

        fn read(&self) -> u32 {
            u32::from(self.content)
        }

        fn mutate(&mut self, word: u32) {
            let old_val: u32 = self.read();
            let mut new_val: u32 = word;

            // Check if modifies reserved bits
            if old_val & Self::RESERVED_BITS_MASK != new_val & Self::RESERVED_BITS_MASK {
                warn!(target: "cc2650_constants::FCFG1::USER_ID", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::FCFG1::USER_ID",
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
