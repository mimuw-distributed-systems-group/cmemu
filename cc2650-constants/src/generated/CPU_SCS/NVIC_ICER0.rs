use cmemu_common::Address;

pub const DISPLAY: &str = "NVIC_ICER0";
pub const OFFSET: u32 = 0x180;
/// 0xe000e180
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
pub const RESET_VALUE: u32 = 0x00000000;
pub const RESET_MASK: u32 = 0xffffffff;
/// Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number  31 (See EVENT:CPUIRQSEL31.EV for details). Reading the bit returns its current enable state.
pub mod CLRENA31 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 31..=31;
    pub const BIT_MASK: u32 = 0x80000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number  30 (See EVENT:CPUIRQSEL30.EV for details). Reading the bit returns its current enable state.
pub mod CLRENA30 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 30..=30;
    pub const BIT_MASK: u32 = 0x40000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number  29 (See EVENT:CPUIRQSEL29.EV for details). Reading the bit returns its current enable state.
pub mod CLRENA29 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 29..=29;
    pub const BIT_MASK: u32 = 0x20000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number  28 (See EVENT:CPUIRQSEL28.EV for details). Reading the bit returns its current enable state.
pub mod CLRENA28 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 28..=28;
    pub const BIT_MASK: u32 = 0x10000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number  27 (See EVENT:CPUIRQSEL27.EV for details). Reading the bit returns its current enable state.
pub mod CLRENA27 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 27..=27;
    pub const BIT_MASK: u32 = 0x08000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number  26 (See EVENT:CPUIRQSEL26.EV for details). Reading the bit returns its current enable state.
pub mod CLRENA26 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 26..=26;
    pub const BIT_MASK: u32 = 0x04000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number  25 (See EVENT:CPUIRQSEL25.EV for details). Reading the bit returns its current enable state.
pub mod CLRENA25 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 25..=25;
    pub const BIT_MASK: u32 = 0x02000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number  24 (See EVENT:CPUIRQSEL24.EV for details). Reading the bit returns its current enable state.
pub mod CLRENA24 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 24..=24;
    pub const BIT_MASK: u32 = 0x01000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number  23 (See EVENT:CPUIRQSEL23.EV for details). Reading the bit returns its current enable state.
pub mod CLRENA23 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 23..=23;
    pub const BIT_MASK: u32 = 0x00800000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number  22 (See EVENT:CPUIRQSEL22.EV for details). Reading the bit returns its current enable state.
pub mod CLRENA22 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 22..=22;
    pub const BIT_MASK: u32 = 0x00400000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number  21 (See EVENT:CPUIRQSEL21.EV for details). Reading the bit returns its current enable state.
pub mod CLRENA21 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 21..=21;
    pub const BIT_MASK: u32 = 0x00200000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number  20 (See EVENT:CPUIRQSEL20.EV for details). Reading the bit returns its current enable state.
pub mod CLRENA20 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 20..=20;
    pub const BIT_MASK: u32 = 0x00100000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number  19 (See EVENT:CPUIRQSEL19.EV for details). Reading the bit returns its current enable state.
pub mod CLRENA19 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 19..=19;
    pub const BIT_MASK: u32 = 0x00080000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number  18 (See EVENT:CPUIRQSEL18.EV for details). Reading the bit returns its current enable state.
pub mod CLRENA18 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 18..=18;
    pub const BIT_MASK: u32 = 0x00040000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number  17 (See EVENT:CPUIRQSEL17.EV for details). Reading the bit returns its current enable state.
pub mod CLRENA17 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 17..=17;
    pub const BIT_MASK: u32 = 0x00020000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number  16 (See EVENT:CPUIRQSEL16.EV for details). Reading the bit returns its current enable state.
pub mod CLRENA16 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 16..=16;
    pub const BIT_MASK: u32 = 0x00010000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number  15 (See EVENT:CPUIRQSEL15.EV for details). Reading the bit returns its current enable state.
pub mod CLRENA15 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 15..=15;
    pub const BIT_MASK: u32 = 0x00008000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number  14 (See EVENT:CPUIRQSEL14.EV for details). Reading the bit returns its current enable state.
pub mod CLRENA14 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 14..=14;
    pub const BIT_MASK: u32 = 0x00004000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number  13 (See EVENT:CPUIRQSEL13.EV for details). Reading the bit returns its current enable state.
pub mod CLRENA13 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 13..=13;
    pub const BIT_MASK: u32 = 0x00002000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number  12 (See EVENT:CPUIRQSEL12.EV for details). Reading the bit returns its current enable state.
pub mod CLRENA12 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 12..=12;
    pub const BIT_MASK: u32 = 0x00001000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number  11 (See EVENT:CPUIRQSEL11.EV for details). Reading the bit returns its current enable state.
pub mod CLRENA11 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 11..=11;
    pub const BIT_MASK: u32 = 0x00000800;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number  10 (See EVENT:CPUIRQSEL10.EV for details). Reading the bit returns its current enable state.
pub mod CLRENA10 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 10..=10;
    pub const BIT_MASK: u32 = 0x00000400;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number  9 (See EVENT:CPUIRQSEL9.EV for details). Reading the bit returns its current enable state.
pub mod CLRENA9 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 9..=9;
    pub const BIT_MASK: u32 = 0x00000200;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number  8 (See EVENT:CPUIRQSEL8.EV for details). Reading the bit returns its current enable state.
pub mod CLRENA8 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 8..=8;
    pub const BIT_MASK: u32 = 0x00000100;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number  7 (See EVENT:CPUIRQSEL7.EV for details). Reading the bit returns its current enable state.
pub mod CLRENA7 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 7..=7;
    pub const BIT_MASK: u32 = 0x00000080;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number  6 (See EVENT:CPUIRQSEL6.EV for details). Reading the bit returns its current enable state.
pub mod CLRENA6 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 6..=6;
    pub const BIT_MASK: u32 = 0x00000040;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number  5 (See EVENT:CPUIRQSEL5.EV for details). Reading the bit returns its current enable state.
pub mod CLRENA5 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 5..=5;
    pub const BIT_MASK: u32 = 0x00000020;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number  4 (See EVENT:CPUIRQSEL4.EV for details). Reading the bit returns its current enable state.
pub mod CLRENA4 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 4..=4;
    pub const BIT_MASK: u32 = 0x00000010;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number  3 (See EVENT:CPUIRQSEL3.EV for details). Reading the bit returns its current enable state.
pub mod CLRENA3 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 3..=3;
    pub const BIT_MASK: u32 = 0x00000008;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number  2 (See EVENT:CPUIRQSEL2.EV for details). Reading the bit returns its current enable state.
pub mod CLRENA2 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 2..=2;
    pub const BIT_MASK: u32 = 0x00000004;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number  1 (See EVENT:CPUIRQSEL1.EV for details). Reading the bit returns its current enable state.
pub mod CLRENA1 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 1..=1;
    pub const BIT_MASK: u32 = 0x00000002;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number  0 (See EVENT:CPUIRQSEL0.EV for details). Reading the bit returns its current enable state.
pub mod CLRENA0 {
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
        pub CLRENA0: B1,
        pub CLRENA1: B1,
        pub CLRENA2: B1,
        pub CLRENA3: B1,
        pub CLRENA4: B1,
        pub CLRENA5: B1,
        pub CLRENA6: B1,
        pub CLRENA7: B1,
        pub CLRENA8: B1,
        pub CLRENA9: B1,
        pub CLRENA10: B1,
        pub CLRENA11: B1,
        pub CLRENA12: B1,
        pub CLRENA13: B1,
        pub CLRENA14: B1,
        pub CLRENA15: B1,
        pub CLRENA16: B1,
        pub CLRENA17: B1,
        pub CLRENA18: B1,
        pub CLRENA19: B1,
        pub CLRENA20: B1,
        pub CLRENA21: B1,
        pub CLRENA22: B1,
        pub CLRENA23: B1,
        pub CLRENA24: B1,
        pub CLRENA25: B1,
        pub CLRENA26: B1,
        pub CLRENA27: B1,
        pub CLRENA28: B1,
        pub CLRENA29: B1,
        pub CLRENA30: B1,
        pub CLRENA31: B1,
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
                warn!(target: "cc2650_constants::CPU_SCS::NVIC_ICER0", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::CPU_SCS::NVIC_ICER0",
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
