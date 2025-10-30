use cmemu_common::Address;

pub const DISPLAY: &str = "NVIC_ISER0";
pub const OFFSET: u32 = 0x100;
/// 0xe000e100
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
pub const RESET_VALUE: u32 = 0x00000000;
pub const RESET_MASK: u32 = 0xffffffff;
/// Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 31 (See EVENT:CPUIRQSEL31.EV for details). Reading the bit returns its current enable state.
pub mod SETENA31 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 31..=31;
    pub const BIT_MASK: u32 = 0x80000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 30 (See EVENT:CPUIRQSEL30.EV for details). Reading the bit returns its current enable state.
pub mod SETENA30 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 30..=30;
    pub const BIT_MASK: u32 = 0x40000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 29 (See EVENT:CPUIRQSEL29.EV for details). Reading the bit returns its current enable state.
pub mod SETENA29 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 29..=29;
    pub const BIT_MASK: u32 = 0x20000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 28 (See EVENT:CPUIRQSEL28.EV for details). Reading the bit returns its current enable state.
pub mod SETENA28 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 28..=28;
    pub const BIT_MASK: u32 = 0x10000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 27 (See EVENT:CPUIRQSEL27.EV for details). Reading the bit returns its current enable state.
pub mod SETENA27 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 27..=27;
    pub const BIT_MASK: u32 = 0x08000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 26 (See EVENT:CPUIRQSEL26.EV for details). Reading the bit returns its current enable state.
pub mod SETENA26 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 26..=26;
    pub const BIT_MASK: u32 = 0x04000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 25 (See EVENT:CPUIRQSEL25.EV for details). Reading the bit returns its current enable state.
pub mod SETENA25 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 25..=25;
    pub const BIT_MASK: u32 = 0x02000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 24 (See EVENT:CPUIRQSEL24.EV for details). Reading the bit returns its current enable state.
pub mod SETENA24 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 24..=24;
    pub const BIT_MASK: u32 = 0x01000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 23 (See EVENT:CPUIRQSEL23.EV for details). Reading the bit returns its current enable state.
pub mod SETENA23 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 23..=23;
    pub const BIT_MASK: u32 = 0x00800000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 22 (See EVENT:CPUIRQSEL22.EV for details). Reading the bit returns its current enable state.
pub mod SETENA22 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 22..=22;
    pub const BIT_MASK: u32 = 0x00400000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 21 (See EVENT:CPUIRQSEL21.EV for details). Reading the bit returns its current enable state.
pub mod SETENA21 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 21..=21;
    pub const BIT_MASK: u32 = 0x00200000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 20 (See EVENT:CPUIRQSEL20.EV for details). Reading the bit returns its current enable state.
pub mod SETENA20 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 20..=20;
    pub const BIT_MASK: u32 = 0x00100000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 19 (See EVENT:CPUIRQSEL19.EV for details). Reading the bit returns its current enable state.
pub mod SETENA19 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 19..=19;
    pub const BIT_MASK: u32 = 0x00080000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 18 (See EVENT:CPUIRQSEL18.EV for details). Reading the bit returns its current enable state.
pub mod SETENA18 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 18..=18;
    pub const BIT_MASK: u32 = 0x00040000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 17 (See EVENT:CPUIRQSEL17.EV for details). Reading the bit returns its current enable state.
pub mod SETENA17 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 17..=17;
    pub const BIT_MASK: u32 = 0x00020000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 16 (See EVENT:CPUIRQSEL16.EV for details). Reading the bit returns its current enable state.
pub mod SETENA16 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 16..=16;
    pub const BIT_MASK: u32 = 0x00010000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 15 (See EVENT:CPUIRQSEL15.EV for details). Reading the bit returns its current enable state.
pub mod SETENA15 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 15..=15;
    pub const BIT_MASK: u32 = 0x00008000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 14 (See EVENT:CPUIRQSEL14.EV for details). Reading the bit returns its current enable state.
pub mod SETENA14 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 14..=14;
    pub const BIT_MASK: u32 = 0x00004000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 13 (See EVENT:CPUIRQSEL13.EV for details). Reading the bit returns its current enable state.
pub mod SETENA13 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 13..=13;
    pub const BIT_MASK: u32 = 0x00002000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 12 (See EVENT:CPUIRQSEL12.EV for details). Reading the bit returns its current enable state.
pub mod SETENA12 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 12..=12;
    pub const BIT_MASK: u32 = 0x00001000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 11 (See EVENT:CPUIRQSEL11.EV for details). Reading the bit returns its current enable state.
pub mod SETENA11 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 11..=11;
    pub const BIT_MASK: u32 = 0x00000800;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 10 (See EVENT:CPUIRQSEL10.EV for details). Reading the bit returns its current enable state.
pub mod SETENA10 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 10..=10;
    pub const BIT_MASK: u32 = 0x00000400;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 9 (See EVENT:CPUIRQSEL9.EV for details). Reading the bit returns its current enable state.
pub mod SETENA9 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 9..=9;
    pub const BIT_MASK: u32 = 0x00000200;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 8 (See EVENT:CPUIRQSEL8.EV for details). Reading the bit returns its current enable state.
pub mod SETENA8 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 8..=8;
    pub const BIT_MASK: u32 = 0x00000100;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 7 (See EVENT:CPUIRQSEL7.EV for details). Reading the bit returns its current enable state.
pub mod SETENA7 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 7..=7;
    pub const BIT_MASK: u32 = 0x00000080;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 6 (See EVENT:CPUIRQSEL6.EV for details). Reading the bit returns its current enable state.
pub mod SETENA6 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 6..=6;
    pub const BIT_MASK: u32 = 0x00000040;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 5 (See EVENT:CPUIRQSEL5.EV for details). Reading the bit returns its current enable state.
pub mod SETENA5 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 5..=5;
    pub const BIT_MASK: u32 = 0x00000020;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 4 (See EVENT:CPUIRQSEL4.EV for details). Reading the bit returns its current enable state.
pub mod SETENA4 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 4..=4;
    pub const BIT_MASK: u32 = 0x00000010;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 3 (See EVENT:CPUIRQSEL3.EV for details). Reading the bit returns its current enable state.
pub mod SETENA3 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 3..=3;
    pub const BIT_MASK: u32 = 0x00000008;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 2 (See EVENT:CPUIRQSEL2.EV for details). Reading the bit returns its current enable state.
pub mod SETENA2 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 2..=2;
    pub const BIT_MASK: u32 = 0x00000004;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 1 (See EVENT:CPUIRQSEL1.EV for details). Reading the bit returns its current enable state.
pub mod SETENA1 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 1..=1;
    pub const BIT_MASK: u32 = 0x00000002;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 0 (See EVENT:CPUIRQSEL0.EV for details). Reading the bit returns its current enable state.
pub mod SETENA0 {
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
        pub SETENA0: B1,
        pub SETENA1: B1,
        pub SETENA2: B1,
        pub SETENA3: B1,
        pub SETENA4: B1,
        pub SETENA5: B1,
        pub SETENA6: B1,
        pub SETENA7: B1,
        pub SETENA8: B1,
        pub SETENA9: B1,
        pub SETENA10: B1,
        pub SETENA11: B1,
        pub SETENA12: B1,
        pub SETENA13: B1,
        pub SETENA14: B1,
        pub SETENA15: B1,
        pub SETENA16: B1,
        pub SETENA17: B1,
        pub SETENA18: B1,
        pub SETENA19: B1,
        pub SETENA20: B1,
        pub SETENA21: B1,
        pub SETENA22: B1,
        pub SETENA23: B1,
        pub SETENA24: B1,
        pub SETENA25: B1,
        pub SETENA26: B1,
        pub SETENA27: B1,
        pub SETENA28: B1,
        pub SETENA29: B1,
        pub SETENA30: B1,
        pub SETENA31: B1,
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
                warn!(target: "cc2650_constants::CPU_SCS::NVIC_ISER0", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::CPU_SCS::NVIC_ISER0",
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
