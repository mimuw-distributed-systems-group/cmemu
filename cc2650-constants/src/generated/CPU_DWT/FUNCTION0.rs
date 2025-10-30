use cmemu_common::Address;

pub const DISPLAY: &str = "FUNCTION0";
pub const OFFSET: u32 = 0x28;
/// 0xe0001028
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
pub const RESET_VALUE: u32 = 0x00000000;
pub const RESET_MASK: u32 = 0xffffffff;
/// This bit is set when the comparator matches, and indicates that the operation defined by FUNCTION has occurred since this bit was last read. This bit is cleared on read.
pub mod MATCHED {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 24..=24;
    pub const BIT_MASK: u32 = 0x01000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// This bit is only available in comparator 0. When set, COMP0 will compare against the cycle counter (CYCCNT).
pub mod CYCMATCH {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 7..=7;
    pub const BIT_MASK: u32 = 0x00000080;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Emit range field. This bit permits emitting offset when range match occurs. PC sampling is not supported when emit range is enabled.
///
/// This field only applies for: FUNCTION = 1, 2, 3, 12, 13, 14, and 15.
pub mod EMITRANGE {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 5..=5;
    pub const BIT_MASK: u32 = 0x00000020;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Function settings.
///
///
///
/// 0x0: Disabled
///
/// 0x1: EMITRANGE = 0, sample and emit PC through ITM. EMITRANGE = 1, emit address offset through ITM
///
/// 0x2: EMITRANGE = 0, emit data through ITM on read and write. EMITRANGE = 1, emit data and address offset through ITM on read or write.
///
/// 0x3: EMITRANGE = 0, sample PC and data value through ITM on read or write. EMITRANGE = 1, emit address offset and data value through ITM on read or write.
///
/// 0x4: Watchpoint on PC match.
///
/// 0x5: Watchpoint on read.
///
/// 0x6: Watchpoint on write.
///
/// 0x7: Watchpoint on read or write.
///
/// 0x8: ETM trigger on PC match
///
/// 0x9: ETM trigger on read
///
/// 0xA: ETM trigger on write
///
/// 0xB: ETM trigger on read or write
///
/// 0xC: EMITRANGE = 0, sample data for read transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) for read transfers
///
/// 0xD: EMITRANGE = 0, sample data for write transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) for write transfers
///
/// 0xE: EMITRANGE = 0, sample PC + data for read transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) + data for read transfers
///
/// 0xF: EMITRANGE = 0, sample PC + data for write transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) + data for write transfers
///
///
///
/// Note 1: If the ETM is not fitted, then ETM trigger is not possible.
///
/// Note 2: Data value is only sampled for accesses that do not fault (MPU or bus fault). The PC is sampled irrespective of any faults. The PC is only sampled for the first address of a burst.
///
/// Note 3: PC match is not recommended for watchpoints because it stops after the instruction. It mainly guards and triggers the ETM.
pub mod FUNCTION {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 0..=3;
    pub const BIT_MASK: u32 = 0x0000000f;
    pub const BIT_WIDTH: u8 = 4;
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
        pub FUNCTION: B4,
        pub reserved_4_5: B1,
        pub EMITRANGE: B1,
        pub reserved_6_7: B1,
        pub CYCMATCH: B1,
        pub reserved_8_24: B16,
        pub MATCHED: B1,
        pub reserved_25_32: B7,
    }

    impl HwRegister for Register {
        const RESERVED_BITS_MASK: u32 = 0xfeffff50;
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
                warn!(target: "cc2650_constants::CPU_DWT::FUNCTION0", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::CPU_DWT::FUNCTION0",
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
