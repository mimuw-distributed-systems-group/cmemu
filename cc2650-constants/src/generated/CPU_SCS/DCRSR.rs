use cmemu_common::Address;

pub const DISPLAY: &str = "DCRSR";
pub const OFFSET: u32 = 0xdf4;
/// 0xe000edf4
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
/// 1: Write
///
/// 0: Read
pub mod REGWNR {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 16..=16;
    pub const BIT_MASK: u32 = 0x00010000;
    pub const BIT_WIDTH: u8 = 1;
    pub const WRITABLE: bool = true;
}
/// Register select
///
///
///
/// 0x00: R0
///
/// 0x01: R1
///
/// 0x02: R2
///
/// 0x03: R3
///
/// 0x04: R4
///
/// 0x05: R5
///
/// 0x06: R6
///
/// 0x07: R7
///
/// 0x08: R8
///
/// 0x09: R9
///
/// 0x0A: R10
///
/// 0x0B: R11
///
/// 0x0C: R12
///
/// 0x0D: Current SP
///
/// 0x0E: LR
///
/// 0x0F: DebugReturnAddress
///
/// 0x10: XPSR/flags, execution state information, and exception number
///
/// 0x11: MSP (Main SP)
///
/// 0x12: PSP (Process SP)
///
/// 0x14: CONTROL<<24 | FAULTMASK<<16 | BASEPRI<<8 | PRIMASK
pub mod REGSEL {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 0..=4;
    pub const BIT_MASK: u32 = 0x0000001f;
    pub const BIT_WIDTH: u8 = 5;
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
        pub REGSEL: B5,
        pub reserved_5_16: B11,
        pub REGWNR: B1,
        pub reserved_17_32: B15,
    }

    impl HwRegister for Register {
        const RESERVED_BITS_MASK: u32 = 0xfffeffe0;
        const READ_ONLY_BITS_MASK: u32 = 0x00000000;
        const WRITE_ONLY_BITS_MASK: u32 = 0x0001001f;

        fn read(&self) -> u32 {
            u32::from(self.content)
        }

        fn mutate(&mut self, word: u32) {
            let old_val: u32 = self.read();
            let mut new_val: u32 = word;

            // Check if modifies reserved bits
            if old_val & Self::RESERVED_BITS_MASK != new_val & Self::RESERVED_BITS_MASK {
                warn!(target: "cc2650_constants::CPU_SCS::DCRSR", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::CPU_SCS::DCRSR",
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
