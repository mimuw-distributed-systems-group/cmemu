use cmemu_common::Address;

pub const DISPLAY: &str = "NVIC_IABR0";
pub const OFFSET: u32 = 0x300;
/// 0xe000e300
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
pub const RESET_VALUE: u32 = 0x00000000;
pub const RESET_MASK: u32 = 0xffffffff;
/// Reading 0 from this bit implies that interrupt line 31 is not active. Reading 1 from this bit implies that the interrupt line 31 is active (See EVENT:CPUIRQSEL31.EV for details).
pub mod ACTIVE31 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 31..=31;
    pub const BIT_MASK: u32 = 0x80000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Reading 0 from this bit implies that interrupt line 30 is not active. Reading 1 from this bit implies that the interrupt line 30 is active (See EVENT:CPUIRQSEL30.EV for details).
pub mod ACTIVE30 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 30..=30;
    pub const BIT_MASK: u32 = 0x40000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Reading 0 from this bit implies that interrupt line 29 is not active. Reading 1 from this bit implies that the interrupt line 29 is active (See EVENT:CPUIRQSEL29.EV for details).
pub mod ACTIVE29 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 29..=29;
    pub const BIT_MASK: u32 = 0x20000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Reading 0 from this bit implies that interrupt line 28 is not active. Reading 1 from this bit implies that the interrupt line 28 is active (See EVENT:CPUIRQSEL28.EV for details).
pub mod ACTIVE28 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 28..=28;
    pub const BIT_MASK: u32 = 0x10000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Reading 0 from this bit implies that interrupt line 27 is not active. Reading 1 from this bit implies that the interrupt line 27 is active (See EVENT:CPUIRQSEL27.EV for details).
pub mod ACTIVE27 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 27..=27;
    pub const BIT_MASK: u32 = 0x08000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Reading 0 from this bit implies that interrupt line 26 is not active. Reading 1 from this bit implies that the interrupt line 26 is active (See EVENT:CPUIRQSEL26.EV for details).
pub mod ACTIVE26 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 26..=26;
    pub const BIT_MASK: u32 = 0x04000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Reading 0 from this bit implies that interrupt line 25 is not active. Reading 1 from this bit implies that the interrupt line 25 is active (See EVENT:CPUIRQSEL25.EV for details).
pub mod ACTIVE25 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 25..=25;
    pub const BIT_MASK: u32 = 0x02000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Reading 0 from this bit implies that interrupt line 24 is not active. Reading 1 from this bit implies that the interrupt line 24 is active (See EVENT:CPUIRQSEL24.EV for details).
pub mod ACTIVE24 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 24..=24;
    pub const BIT_MASK: u32 = 0x01000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Reading 0 from this bit implies that interrupt line 23 is not active. Reading 1 from this bit implies that the interrupt line 23 is active (See EVENT:CPUIRQSEL23.EV for details).
pub mod ACTIVE23 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 23..=23;
    pub const BIT_MASK: u32 = 0x00800000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Reading 0 from this bit implies that interrupt line 22 is not active. Reading 1 from this bit implies that the interrupt line 22 is active (See EVENT:CPUIRQSEL22.EV for details).
pub mod ACTIVE22 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 22..=22;
    pub const BIT_MASK: u32 = 0x00400000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Reading 0 from this bit implies that interrupt line 21 is not active. Reading 1 from this bit implies that the interrupt line 21 is active (See EVENT:CPUIRQSEL21.EV for details).
pub mod ACTIVE21 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 21..=21;
    pub const BIT_MASK: u32 = 0x00200000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Reading 0 from this bit implies that interrupt line 20 is not active. Reading 1 from this bit implies that the interrupt line 20 is active (See EVENT:CPUIRQSEL20.EV for details).
pub mod ACTIVE20 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 20..=20;
    pub const BIT_MASK: u32 = 0x00100000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Reading 0 from this bit implies that interrupt line 19 is not active. Reading 1 from this bit implies that the interrupt line 19 is active (See EVENT:CPUIRQSEL19.EV for details).
pub mod ACTIVE19 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 19..=19;
    pub const BIT_MASK: u32 = 0x00080000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Reading 0 from this bit implies that interrupt line 18 is not active. Reading 1 from this bit implies that the interrupt line 18 is active (See EVENT:CPUIRQSEL18.EV for details).
pub mod ACTIVE18 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 18..=18;
    pub const BIT_MASK: u32 = 0x00040000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Reading 0 from this bit implies that interrupt line 17 is not active. Reading 1 from this bit implies that the interrupt line 17 is active (See EVENT:CPUIRQSEL17.EV for details).
pub mod ACTIVE17 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 17..=17;
    pub const BIT_MASK: u32 = 0x00020000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Reading 0 from this bit implies that interrupt line 16 is not active. Reading 1 from this bit implies that the interrupt line 16 is active (See EVENT:CPUIRQSEL16.EV for details).
pub mod ACTIVE16 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 16..=16;
    pub const BIT_MASK: u32 = 0x00010000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Reading 0 from this bit implies that interrupt line 15 is not active. Reading 1 from this bit implies that the interrupt line 15 is active (See EVENT:CPUIRQSEL15.EV for details).
pub mod ACTIVE15 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 15..=15;
    pub const BIT_MASK: u32 = 0x00008000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Reading 0 from this bit implies that interrupt line 14 is not active. Reading 1 from this bit implies that the interrupt line 14 is active (See EVENT:CPUIRQSEL14.EV for details).
pub mod ACTIVE14 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 14..=14;
    pub const BIT_MASK: u32 = 0x00004000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Reading 0 from this bit implies that interrupt line 13 is not active. Reading 1 from this bit implies that the interrupt line 13 is active (See EVENT:CPUIRQSEL13.EV for details).
pub mod ACTIVE13 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 13..=13;
    pub const BIT_MASK: u32 = 0x00002000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Reading 0 from this bit implies that interrupt line 12 is not active. Reading 1 from this bit implies that the interrupt line 12 is active (See EVENT:CPUIRQSEL12.EV for details).
pub mod ACTIVE12 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 12..=12;
    pub const BIT_MASK: u32 = 0x00001000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Reading 0 from this bit implies that interrupt line 11 is not active. Reading 1 from this bit implies that the interrupt line 11 is active (See EVENT:CPUIRQSEL11.EV for details).
pub mod ACTIVE11 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 11..=11;
    pub const BIT_MASK: u32 = 0x00000800;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Reading 0 from this bit implies that interrupt line 10 is not active. Reading 1 from this bit implies that the interrupt line 10 is active (See EVENT:CPUIRQSEL10.EV for details).
pub mod ACTIVE10 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 10..=10;
    pub const BIT_MASK: u32 = 0x00000400;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Reading 0 from this bit implies that interrupt line 9 is not active. Reading 1 from this bit implies that the interrupt line 9 is active (See EVENT:CPUIRQSEL9.EV for details).
pub mod ACTIVE9 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 9..=9;
    pub const BIT_MASK: u32 = 0x00000200;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Reading 0 from this bit implies that interrupt line 8 is not active. Reading 1 from this bit implies that the interrupt line 8 is active (See EVENT:CPUIRQSEL8.EV for details).
pub mod ACTIVE8 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 8..=8;
    pub const BIT_MASK: u32 = 0x00000100;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Reading 0 from this bit implies that interrupt line 7 is not active. Reading 1 from this bit implies that the interrupt line 7 is active (See EVENT:CPUIRQSEL7.EV for details).
pub mod ACTIVE7 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 7..=7;
    pub const BIT_MASK: u32 = 0x00000080;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Reading 0 from this bit implies that interrupt line 6 is not active. Reading 1 from this bit implies that the interrupt line 6 is active (See EVENT:CPUIRQSEL6.EV for details).
pub mod ACTIVE6 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 6..=6;
    pub const BIT_MASK: u32 = 0x00000040;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Reading 0 from this bit implies that interrupt line 5 is not active. Reading 1 from this bit implies that the interrupt line 5 is active (See EVENT:CPUIRQSEL5.EV for details).
pub mod ACTIVE5 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 5..=5;
    pub const BIT_MASK: u32 = 0x00000020;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Reading 0 from this bit implies that interrupt line 4 is not active. Reading 1 from this bit implies that the interrupt line 4 is active (See EVENT:CPUIRQSEL4.EV for details).
pub mod ACTIVE4 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 4..=4;
    pub const BIT_MASK: u32 = 0x00000010;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Reading 0 from this bit implies that interrupt line 3 is not active. Reading 1 from this bit implies that the interrupt line 3 is active (See EVENT:CPUIRQSEL3.EV for details).
pub mod ACTIVE3 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 3..=3;
    pub const BIT_MASK: u32 = 0x00000008;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Reading 0 from this bit implies that interrupt line 2 is not active. Reading 1 from this bit implies that the interrupt line 2 is active (See EVENT:CPUIRQSEL2.EV for details).
pub mod ACTIVE2 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 2..=2;
    pub const BIT_MASK: u32 = 0x00000004;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Reading 0 from this bit implies that interrupt line 1 is not active. Reading 1 from this bit implies that the interrupt line 1 is active (See EVENT:CPUIRQSEL1.EV for details).
pub mod ACTIVE1 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 1..=1;
    pub const BIT_MASK: u32 = 0x00000002;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Reading 0 from this bit implies that interrupt line 0 is not active. Reading 1 from this bit implies that the interrupt line 0 is active (See EVENT:CPUIRQSEL0.EV for details).
pub mod ACTIVE0 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 0..=0;
    pub const BIT_MASK: u32 = 0x00000001;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
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
        pub ACTIVE0: B1,
        pub ACTIVE1: B1,
        pub ACTIVE2: B1,
        pub ACTIVE3: B1,
        pub ACTIVE4: B1,
        pub ACTIVE5: B1,
        pub ACTIVE6: B1,
        pub ACTIVE7: B1,
        pub ACTIVE8: B1,
        pub ACTIVE9: B1,
        pub ACTIVE10: B1,
        pub ACTIVE11: B1,
        pub ACTIVE12: B1,
        pub ACTIVE13: B1,
        pub ACTIVE14: B1,
        pub ACTIVE15: B1,
        pub ACTIVE16: B1,
        pub ACTIVE17: B1,
        pub ACTIVE18: B1,
        pub ACTIVE19: B1,
        pub ACTIVE20: B1,
        pub ACTIVE21: B1,
        pub ACTIVE22: B1,
        pub ACTIVE23: B1,
        pub ACTIVE24: B1,
        pub ACTIVE25: B1,
        pub ACTIVE26: B1,
        pub ACTIVE27: B1,
        pub ACTIVE28: B1,
        pub ACTIVE29: B1,
        pub ACTIVE30: B1,
        pub ACTIVE31: B1,
    }

    impl HwRegister for Register {
        const RESERVED_BITS_MASK: u32 = 0x00000000;
        const READ_ONLY_BITS_MASK: u32 = 0xffffffff;
        const WRITE_ONLY_BITS_MASK: u32 = 0x00000000;

        fn read(&self) -> u32 {
            u32::from(self.content)
        }

        fn mutate(&mut self, word: u32) {
            let old_val: u32 = self.read();
            let mut new_val: u32 = word;

            // Check if modifies reserved bits
            if old_val & Self::RESERVED_BITS_MASK != new_val & Self::RESERVED_BITS_MASK {
                warn!(target: "cc2650_constants::CPU_SCS::NVIC_IABR0", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::CPU_SCS::NVIC_IABR0",
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
