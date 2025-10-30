use cmemu_common::Address;

pub const DISPLAY: &str = "DHCSR";
pub const OFFSET: u32 = 0xdf0;
/// 0xe000edf0
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
/// Indicates that the core has been reset, or is now being reset, since the last time this bit was read. This a sticky bit that clears on read. So, reading twice and getting 1 then 0 means it was reset in the past. Reading twice and getting 1 both times means that it is being reset now (held in reset still).
///
/// When writing to this register, 0 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register.
pub mod S_RESET_ST {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 25..=25;
    pub const BIT_MASK: u32 = 0x02000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const WRITABLE: bool = true;
}
/// Indicates that an instruction has completed since last read. This is a sticky bit that clears on read. This determines if the core is stalled on a load/store or fetch.
///
/// When writing to this register, 0 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register.
pub mod S_RETIRE_ST {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 24..=24;
    pub const BIT_MASK: u32 = 0x01000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const WRITABLE: bool = true;
}
/// Reads as one if the core is running (not halted) and a lockup condition is present.
///
/// When writing to this register, 1 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register.
pub mod S_LOCKUP {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 19..=19;
    pub const BIT_MASK: u32 = 0x00080000;
    pub const BIT_WIDTH: u8 = 1;
    pub const WRITABLE: bool = true;
}
/// Indicates that the core is sleeping (WFI, WFE, or **SLEEP-ON-EXIT**). Must use C_HALT to gain control or wait for interrupt to wake-up.
///
/// When writing to this register, 1 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register.
pub mod S_SLEEP {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 18..=18;
    pub const BIT_MASK: u32 = 0x00040000;
    pub const BIT_WIDTH: u8 = 1;
    pub const WRITABLE: bool = true;
}
/// The core is in debug state when this bit is set.
///
/// When writing to this register, 1 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register.
pub mod S_HALT {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 17..=17;
    pub const BIT_MASK: u32 = 0x00020000;
    pub const BIT_WIDTH: u8 = 1;
    pub const WRITABLE: bool = true;
}
/// Register Read/Write on the Debug Core Register Selector register is available. Last transfer is complete.
///
/// When writing to this register, 1 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register.
pub mod S_REGRDY {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 16..=16;
    pub const BIT_MASK: u32 = 0x00010000;
    pub const BIT_WIDTH: u8 = 1;
    pub const WRITABLE: bool = true;
}
/// If the core is stalled on a load/store operation the stall ceases and the instruction is forced to complete. This enables Halting debug to gain control of the core. It can only be set if: C_DEBUGEN = 1 and C_HALT = 1. The core reads S_RETIRE_ST as 0. This indicates that no instruction has advanced. This prevents misuse. The bus state is Unpredictable when this is used. S_RETIRE_ST can detect core stalls on load/store operations.
pub mod C_SNAPSTALL {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 5..=5;
    pub const BIT_MASK: u32 = 0x00000020;
    pub const BIT_WIDTH: u8 = 1;
    pub const WRITABLE: bool = true;
}
/// Mask interrupts when stepping or running in halted debug. This masking does not affect NMI, fault exceptions and SVC caused by execution of the instructions. This bit must only be modified when the processor is halted (S_HALT == 1).  C_MASKINTS must be set or cleared before halt is released (i.e., the writes to set or clear C_MASKINTS and to set or clear C_HALT must be separate). Modifying C_MASKINTS while the system is running with halting debug support enabled (C_DEBUGEN = 1, S_HALT = 0) may cause unpredictable behavior.
pub mod C_MASKINTS {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 3..=3;
    pub const BIT_MASK: u32 = 0x00000008;
    pub const BIT_WIDTH: u8 = 1;
    pub const WRITABLE: bool = true;
}
/// Steps the core in halted debug. When C_DEBUGEN = 0, this bit has no effect. Must only be modified when the processor is halted (S_HALT == 1).
///
/// Modifying C_STEP while the system is running with halting debug support enabled (C_DEBUGEN = 1, S_HALT = 0) may cause unpredictable behavior.
pub mod C_STEP {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 2..=2;
    pub const BIT_MASK: u32 = 0x00000004;
    pub const BIT_WIDTH: u8 = 1;
    pub const WRITABLE: bool = true;
}
/// Halts the core. This bit is set automatically when the core Halts. For example Breakpoint. This bit clears on core reset.
pub mod C_HALT {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 1..=1;
    pub const BIT_MASK: u32 = 0x00000002;
    pub const BIT_WIDTH: u8 = 1;
    pub const WRITABLE: bool = true;
}
/// Enables debug. This can only be written by AHB-AP and not by the core. It is ignored when written by the core, which cannot set or clear it. The core must write a 1 to it when writing C_HALT to halt itself.
///
/// The values of C_HALT, C_STEP and C_MASKINTS are ignored by hardware when C_DEBUGEN = 0. The read values for C_HALT, C_STEP and C_MASKINTS fields will be unknown to software when C_DEBUGEN = 0.
pub mod C_DEBUGEN {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 0..=0;
    pub const BIT_MASK: u32 = 0x00000001;
    pub const BIT_WIDTH: u8 = 1;
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
        pub C_DEBUGEN: B1,
        pub C_HALT: B1,
        pub C_STEP: B1,
        pub C_MASKINTS: B1,
        pub reserved_4_5: B1,
        pub C_SNAPSTALL: B1,
        pub reserved_6_16: B10,
        pub S_REGRDY: B1,
        pub S_HALT: B1,
        pub S_SLEEP: B1,
        pub S_LOCKUP: B1,
        pub reserved_20_24: B4,
        pub S_RETIRE_ST: B1,
        pub S_RESET_ST: B1,
        pub reserved_26_32: B6,
    }

    impl HwRegister for Register {
        const RESERVED_BITS_MASK: u32 = 0xfcf0ffd0;
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
                warn!(target: "cc2650_constants::CPU_SCS::DHCSR", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::CPU_SCS::DHCSR",
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
