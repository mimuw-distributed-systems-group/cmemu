use cmemu_common::Address;

pub const DISPLAY: &str = "FUNCTION1";
pub const OFFSET: u32 = 0x38;
/// 0xe0001038
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
pub const RESET_VALUE: u32 = 0x00000200;
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
/// Identity of a second linked address comparator for data value matching when DATAVMATCH == 1 and LNK1ENA == 1.
pub mod DATAVADDR1 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 16..=19;
    pub const BIT_MASK: u32 = 0x000f0000;
    pub const BIT_WIDTH: u8 = 4;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Identity of a linked address comparator for data value matching when DATAVMATCH == 1.
pub mod DATAVADDR0 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 12..=15;
    pub const BIT_MASK: u32 = 0x0000f000;
    pub const BIT_WIDTH: u8 = 4;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Defines the size of the data in the COMP1 register that is to be matched:
///
///
///
/// 0x0: Byte
///
/// 0x1: Halfword
///
/// 0x2: Word
///
/// 0x3: Unpredictable.
pub mod DATAVSIZE {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 10..=11;
    pub const BIT_MASK: u32 = 0x00000c00;
    pub const BIT_WIDTH: u8 = 2;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Read only bit-field only supported in comparator 1.
///
///
///
/// 0: DATAVADDR1 not supported
///
/// 1: DATAVADDR1 supported (enabled)
pub mod LNK1ENA {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 9..=9;
    pub const BIT_MASK: u32 = 0x00000200;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = false;
}
/// Data match feature:
///
///
///
/// 0: Perform address comparison
///
/// 1: Perform data value compare. The comparators given by DATAVADDR0 and DATAVADDR1 provide the address for the data comparison. The FUNCTION setting for the comparators given by DATAVADDR0 and DATAVADDR1 are overridden and those comparators only provide the address match for the data comparison.
///
///
///
/// This bit is only available in comparator 1.
pub mod DATAVMATCH {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 8..=8;
    pub const BIT_MASK: u32 = 0x00000100;
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
/// Function settings:
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
/// Note 3: FUNCTION is overridden for comparators given by DATAVADDR0 and DATAVADDR1 if DATAVMATCH is also set. The comparators given by DATAVADDR0 and DATAVADDR1 can then only perform address comparator matches for comparator 1 data matches.
///
/// Note 4: If the data matching functionality is not included during implementation it is not possible to set DATAVADDR0, DATAVADDR1, or DATAVMATCH. This means that the data matching functionality is not available in the implementation. Test the availability of data matching by writing and reading DATAVMATCH. If it is not settable then data matching is unavailable.
///
/// Note 5: PC match is not recommended for watchpoints because it stops after the instruction. It mainly guards and triggers the ETM.
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
        pub reserved_6_8: B2,
        pub DATAVMATCH: B1,
        pub LNK1ENA: B1,
        pub DATAVSIZE: B2,
        pub DATAVADDR0: B4,
        pub DATAVADDR1: B4,
        pub reserved_20_24: B4,
        pub MATCHED: B1,
        pub reserved_25_32: B7,
    }

    impl HwRegister for Register {
        const RESERVED_BITS_MASK: u32 = 0xfef000d0;
        const READ_ONLY_BITS_MASK: u32 = 0x00000200;
        const WRITE_ONLY_BITS_MASK: u32 = 0x00000000;

        fn read(&self) -> u32 {
            u32::from(self.content)
        }

        fn mutate(&mut self, word: u32) {
            let old_val: u32 = self.read();
            let mut new_val: u32 = word;

            // Check if modifies reserved bits
            if old_val & Self::RESERVED_BITS_MASK != new_val & Self::RESERVED_BITS_MASK {
                warn!(target: "cc2650_constants::CPU_DWT::FUNCTION1", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::CPU_DWT::FUNCTION1",
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
