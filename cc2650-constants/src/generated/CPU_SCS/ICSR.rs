use cmemu_common::Address;

pub const DISPLAY: &str = "ICSR";
pub const OFFSET: u32 = 0xd04;
/// 0xe000ed04
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
/// Set pending NMI bit. Setting this bit pends and activates an NMI. Because NMI is the highest-priority interrupt, it takes effect as soon as it registers.
///
///
///
/// 0: No action
///
/// 1: Set pending NMI
pub mod NMIPENDSET {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 31..=31;
    pub const BIT_MASK: u32 = 0x80000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const WRITABLE: bool = true;
}
/// Set pending pendSV bit.
///
///
///
/// 0: No action
///
/// 1: Set pending PendSV
pub mod PENDSVSET {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 28..=28;
    pub const BIT_MASK: u32 = 0x10000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const WRITABLE: bool = true;
}
/// Clear pending pendSV bit
///
///
///
/// 0: No action
///
/// 1: Clear pending pendSV
pub mod PENDSVCLR {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 27..=27;
    pub const BIT_MASK: u32 = 0x08000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const WRITABLE: bool = true;
}
/// Set a pending SysTick bit.
///
///
///
/// 0: No action
///
/// 1: Set pending SysTick
pub mod PENDSTSET {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 26..=26;
    pub const BIT_MASK: u32 = 0x04000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const WRITABLE: bool = true;
}
/// Clear pending SysTick bit
///
///
///
/// 0: No action
///
/// 1: Clear pending SysTick
pub mod PENDSTCLR {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 25..=25;
    pub const BIT_MASK: u32 = 0x02000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const WRITABLE: bool = true;
}
/// This field can only be used at debug time. It indicates that a pending interrupt is to be taken in the next running cycle. If DHCSR.C_MASKINTS= 0, the interrupt is serviced.
///
///
///
/// 0: A pending exception is not serviced.
///
/// 1: A pending exception is serviced on exit from the debug halt state
pub mod ISRPREEMPT {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 23..=23;
    pub const BIT_MASK: u32 = 0x00800000;
    pub const BIT_WIDTH: u8 = 1;
    pub const WRITABLE: bool = false;
}
/// Interrupt pending flag. Excludes NMI and faults.
///
///
///
/// 0x0: Interrupt not pending
///
/// 0x1: Interrupt pending
pub mod ISRPENDING {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 22..=22;
    pub const BIT_MASK: u32 = 0x00400000;
    pub const BIT_WIDTH: u8 = 1;
    pub const WRITABLE: bool = false;
}
/// Pending ISR number field. This field contains the interrupt number of the highest priority pending ISR.
pub mod VECTPENDING {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 12..=17;
    pub const BIT_MASK: u32 = 0x0003f000;
    pub const BIT_WIDTH: u8 = 6;
    pub const WRITABLE: bool = false;
}
/// Indicates whether there are preempted active exceptions:
///
///
///
/// 0: There are preempted active exceptions to execute
///
/// 1: There are no active exceptions, or the currently-executing exception is the only active exception.
pub mod RETTOBASE {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 11..=11;
    pub const BIT_MASK: u32 = 0x00000800;
    pub const BIT_WIDTH: u8 = 1;
    pub const WRITABLE: bool = false;
}
/// Active ISR number field. Reset clears this field.
pub mod VECTACTIVE {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 0..=8;
    pub const BIT_MASK: u32 = 0x000001ff;
    pub const BIT_WIDTH: u8 = 9;
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
        pub VECTACTIVE: B9,
        pub reserved_9_11: B2,
        pub RETTOBASE: B1,
        pub VECTPENDING: B6,
        pub reserved_18_22: B4,
        pub ISRPENDING: B1,
        pub ISRPREEMPT: B1,
        pub reserved_24_25: B1,
        pub PENDSTCLR: B1,
        pub PENDSTSET: B1,
        pub PENDSVCLR: B1,
        pub PENDSVSET: B1,
        pub reserved_29_31: B2,
        pub NMIPENDSET: B1,
    }

    impl HwRegister for Register {
        const RESERVED_BITS_MASK: u32 = 0x613c0600;
        const READ_ONLY_BITS_MASK: u32 = 0x00c3f9ff;
        const WRITE_ONLY_BITS_MASK: u32 = 0x0a000000;

        fn read(&self) -> u32 {
            u32::from(self.content)
        }

        fn mutate(&mut self, word: u32) {
            let old_val: u32 = self.read();
            let mut new_val: u32 = word;

            // Check if modifies reserved bits
            if old_val & Self::RESERVED_BITS_MASK != new_val & Self::RESERVED_BITS_MASK {
                warn!(target: "cc2650_constants::CPU_SCS::ICSR", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::CPU_SCS::ICSR",
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
