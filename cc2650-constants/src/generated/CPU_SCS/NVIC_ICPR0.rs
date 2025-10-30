use cmemu_common::Address;

pub const DISPLAY: &str = "NVIC_ICPR0";
pub const OFFSET: u32 = 0x280;
/// 0xe000e280
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
pub const RESET_VALUE: u32 = 0x00000000;
pub const RESET_MASK: u32 = 0xffffffff;
/// Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 31 (See EVENT:CPUIRQSEL31.EV for details). Reading the bit returns its current state.
pub mod CLRPEND31 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 31..=31;
    pub const BIT_MASK: u32 = 0x80000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 30 (See EVENT:CPUIRQSEL30.EV for details). Reading the bit returns its current state.
pub mod CLRPEND30 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 30..=30;
    pub const BIT_MASK: u32 = 0x40000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 29 (See EVENT:CPUIRQSEL29.EV for details). Reading the bit returns its current state.
pub mod CLRPEND29 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 29..=29;
    pub const BIT_MASK: u32 = 0x20000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 28 (See EVENT:CPUIRQSEL28.EV for details). Reading the bit returns its current state.
pub mod CLRPEND28 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 28..=28;
    pub const BIT_MASK: u32 = 0x10000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 27 (See EVENT:CPUIRQSEL27.EV for details). Reading the bit returns its current state.
pub mod CLRPEND27 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 27..=27;
    pub const BIT_MASK: u32 = 0x08000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 26 (See EVENT:CPUIRQSEL26.EV for details). Reading the bit returns its current state.
pub mod CLRPEND26 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 26..=26;
    pub const BIT_MASK: u32 = 0x04000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 25 (See EVENT:CPUIRQSEL25.EV for details). Reading the bit returns its current state.
pub mod CLRPEND25 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 25..=25;
    pub const BIT_MASK: u32 = 0x02000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 24 (See EVENT:CPUIRQSEL24.EV for details). Reading the bit returns its current state.
pub mod CLRPEND24 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 24..=24;
    pub const BIT_MASK: u32 = 0x01000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 23 (See EVENT:CPUIRQSEL23.EV for details). Reading the bit returns its current state.
pub mod CLRPEND23 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 23..=23;
    pub const BIT_MASK: u32 = 0x00800000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 22 (See EVENT:CPUIRQSEL22.EV for details). Reading the bit returns its current state.
pub mod CLRPEND22 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 22..=22;
    pub const BIT_MASK: u32 = 0x00400000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 21 (See EVENT:CPUIRQSEL21.EV for details). Reading the bit returns its current state.
pub mod CLRPEND21 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 21..=21;
    pub const BIT_MASK: u32 = 0x00200000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 20 (See EVENT:CPUIRQSEL20.EV for details). Reading the bit returns its current state.
pub mod CLRPEND20 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 20..=20;
    pub const BIT_MASK: u32 = 0x00100000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 19 (See EVENT:CPUIRQSEL19.EV for details). Reading the bit returns its current state.
pub mod CLRPEND19 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 19..=19;
    pub const BIT_MASK: u32 = 0x00080000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 18 (See EVENT:CPUIRQSEL18.EV for details). Reading the bit returns its current state.
pub mod CLRPEND18 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 18..=18;
    pub const BIT_MASK: u32 = 0x00040000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 17 (See EVENT:CPUIRQSEL17.EV for details). Reading the bit returns its current state.
pub mod CLRPEND17 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 17..=17;
    pub const BIT_MASK: u32 = 0x00020000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 16 (See EVENT:CPUIRQSEL16.EV for details). Reading the bit returns its current state.
pub mod CLRPEND16 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 16..=16;
    pub const BIT_MASK: u32 = 0x00010000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 15 (See EVENT:CPUIRQSEL15.EV for details). Reading the bit returns its current state.
pub mod CLRPEND15 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 15..=15;
    pub const BIT_MASK: u32 = 0x00008000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 14 (See EVENT:CPUIRQSEL14.EV for details). Reading the bit returns its current state.
pub mod CLRPEND14 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 14..=14;
    pub const BIT_MASK: u32 = 0x00004000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 13 (See EVENT:CPUIRQSEL13.EV for details). Reading the bit returns its current state.
pub mod CLRPEND13 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 13..=13;
    pub const BIT_MASK: u32 = 0x00002000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 12 (See EVENT:CPUIRQSEL12.EV for details). Reading the bit returns its current state.
pub mod CLRPEND12 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 12..=12;
    pub const BIT_MASK: u32 = 0x00001000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 11 (See EVENT:CPUIRQSEL11.EV for details). Reading the bit returns its current state.
pub mod CLRPEND11 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 11..=11;
    pub const BIT_MASK: u32 = 0x00000800;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 10 (See EVENT:CPUIRQSEL10.EV for details). Reading the bit returns its current state.
pub mod CLRPEND10 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 10..=10;
    pub const BIT_MASK: u32 = 0x00000400;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 9 (See EVENT:CPUIRQSEL9.EV for details). Reading the bit returns its current state.
pub mod CLRPEND9 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 9..=9;
    pub const BIT_MASK: u32 = 0x00000200;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 8 (See EVENT:CPUIRQSEL8.EV for details). Reading the bit returns its current state.
pub mod CLRPEND8 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 8..=8;
    pub const BIT_MASK: u32 = 0x00000100;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 7 (See EVENT:CPUIRQSEL7.EV for details). Reading the bit returns its current state.
pub mod CLRPEND7 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 7..=7;
    pub const BIT_MASK: u32 = 0x00000080;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 6 (See EVENT:CPUIRQSEL6.EV for details). Reading the bit returns its current state.
pub mod CLRPEND6 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 6..=6;
    pub const BIT_MASK: u32 = 0x00000040;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 5 (See EVENT:CPUIRQSEL5.EV for details). Reading the bit returns its current state.
pub mod CLRPEND5 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 5..=5;
    pub const BIT_MASK: u32 = 0x00000020;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 4 (See EVENT:CPUIRQSEL4.EV for details). Reading the bit returns its current state.
pub mod CLRPEND4 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 4..=4;
    pub const BIT_MASK: u32 = 0x00000010;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 3 (See EVENT:CPUIRQSEL3.EV for details). Reading the bit returns its current state.
pub mod CLRPEND3 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 3..=3;
    pub const BIT_MASK: u32 = 0x00000008;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 2 (See EVENT:CPUIRQSEL2.EV for details). Reading the bit returns its current state.
pub mod CLRPEND2 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 2..=2;
    pub const BIT_MASK: u32 = 0x00000004;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 1 (See EVENT:CPUIRQSEL1.EV for details). Reading the bit returns its current state.
pub mod CLRPEND1 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 1..=1;
    pub const BIT_MASK: u32 = 0x00000002;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 0 (See EVENT:CPUIRQSEL0.EV for details). Reading the bit returns its current state.
pub mod CLRPEND0 {
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
        pub CLRPEND0: B1,
        pub CLRPEND1: B1,
        pub CLRPEND2: B1,
        pub CLRPEND3: B1,
        pub CLRPEND4: B1,
        pub CLRPEND5: B1,
        pub CLRPEND6: B1,
        pub CLRPEND7: B1,
        pub CLRPEND8: B1,
        pub CLRPEND9: B1,
        pub CLRPEND10: B1,
        pub CLRPEND11: B1,
        pub CLRPEND12: B1,
        pub CLRPEND13: B1,
        pub CLRPEND14: B1,
        pub CLRPEND15: B1,
        pub CLRPEND16: B1,
        pub CLRPEND17: B1,
        pub CLRPEND18: B1,
        pub CLRPEND19: B1,
        pub CLRPEND20: B1,
        pub CLRPEND21: B1,
        pub CLRPEND22: B1,
        pub CLRPEND23: B1,
        pub CLRPEND24: B1,
        pub CLRPEND25: B1,
        pub CLRPEND26: B1,
        pub CLRPEND27: B1,
        pub CLRPEND28: B1,
        pub CLRPEND29: B1,
        pub CLRPEND30: B1,
        pub CLRPEND31: B1,
    }

    impl HwRegister for Register {
        const RESERVED_BITS_MASK: u32 = 0x00000000;
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
                warn!(target: "cc2650_constants::CPU_SCS::NVIC_ICPR0", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::CPU_SCS::NVIC_ICPR0",
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
