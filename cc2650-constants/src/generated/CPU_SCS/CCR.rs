use cmemu_common::Address;

pub const DISPLAY: &str = "CCR";
pub const OFFSET: u32 = 0xd14;
/// 0xe000ed14
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
pub const RESET_VALUE: u32 = 0x00000200;
pub const RESET_MASK: u32 = 0xffffffff;
/// Stack alignment bit.
///
///
///
/// 0: Only 4-byte alignment is guaranteed for the SP used prior to the exception on exception entry.
///
/// 1: On exception entry, the SP used prior to the exception is adjusted to be 8-byte aligned and the context to restore it is saved. The SP is restored on the associated exception return.
pub mod STKALIGN {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 9..=9;
    pub const BIT_MASK: u32 = 0x00000200;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = true;
}
/// Enables handlers with priority -1 or -2 to ignore data BusFaults caused by load and store instructions. This applies to the HardFault, NMI, and FAULTMASK escalated handlers:
///
///
///
/// 0: Data BusFaults caused by load and store instructions cause a lock-up
///
/// 1: Data BusFaults caused by load and store instructions are ignored.
///
///
///
/// Set this bit to 1 only when the handler and its data are in absolutely safe memory. The normal use
///
/// of this bit is to probe system devices and bridges to detect problems.
pub mod BFHFNMIGN {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 8..=8;
    pub const BIT_MASK: u32 = 0x00000100;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Enables faulting or halting when the processor executes an SDIV or UDIV instruction with a divisor of 0:
///
///
///
/// 0: Do not trap divide by 0. In this mode, a divide by zero returns a quotient of 0.
///
/// 1: Trap divide by 0. The relevant Usage Fault Status Register bit is CFSR.DIVBYZERO.
pub mod DIV_0_TRP {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 4..=4;
    pub const BIT_MASK: u32 = 0x00000010;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Enables unaligned access traps:
///
///
///
/// 0: Do not trap unaligned halfword and word accesses
///
/// 1: Trap unaligned halfword and word accesses. The relevant Usage Fault Status Register bit is CFSR.UNALIGNED.
///
///
///
/// If this bit is set to 1, an unaligned access generates a UsageFault.
///
/// Unaligned LDM, STM, LDRD, and STRD instructions always fault regardless of the value in UNALIGN_TRP.
pub mod UNALIGN_TRP {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 3..=3;
    pub const BIT_MASK: u32 = 0x00000008;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Enables unprivileged software access to STIR:
///
///
///
/// 0: User code is not allowed to write to the Software Trigger Interrupt register (STIR).
///
/// 1: User code can write the Software Trigger Interrupt register (STIR) to trigger (pend) a Main exception, which is associated with the Main stack pointer.
pub mod USERSETMPEND {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 1..=1;
    pub const BIT_MASK: u32 = 0x00000002;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Indicates how the processor enters Thread mode:
///
///
///
/// 0: Processor can enter Thread mode only when no exception is active.
///
/// 1: Processor can enter Thread mode from any level using the appropriate return value (EXC_RETURN).
///
///
///
/// Exception returns occur when one of the following instructions loads a value of 0xFXXXXXXX into the PC while in Handler mode:
///
/// - POP/LDM which includes loading the PC.
///
/// - LDR with PC as a destination.
///
/// - BX with any register.
///
/// The value written to the PC is intercepted and is referred to as the EXC_RETURN value.
pub mod NONBASETHREDENA {
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
        pub NONBASETHREDENA: B1,
        pub USERSETMPEND: B1,
        pub reserved_2_3: B1,
        pub UNALIGN_TRP: B1,
        pub DIV_0_TRP: B1,
        pub reserved_5_8: B3,
        pub BFHFNMIGN: B1,
        pub STKALIGN: B1,
        pub reserved_10_32: B22,
    }

    impl HwRegister for Register {
        const RESERVED_BITS_MASK: u32 = 0xfffffce4;
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
                warn!(target: "cc2650_constants::CPU_SCS::CCR", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::CPU_SCS::CCR",
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
