use cmemu_common::Address;

pub const DISPLAY: &str = "CLKLOADCTL";
pub const OFFSET: u32 = 0x28;
/// 0x40082028
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
pub const RESET_VALUE: u32 = 0x00000002;
pub const RESET_MASK: u32 = 0xffffffff;
/// Status of LOAD.
///
/// Will be cleared to 0 when any of the registers requiring a LOAD is written to, and be set to 1 when a LOAD is done.
///
/// Note that writing no change to a register will result in the LOAD_DONE being cleared.
///
///
///
/// 0 : One or more registers have been write accessed after last LOAD
///
/// 1 : No registers are write accessed after last LOAD
pub mod LOAD_DONE {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 1..=1;
    pub const BIT_MASK: u32 = 0x00000002;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = false;
}
/// 0: No action
///
/// 1: Load settings to CLKCTRL. Bit is HW cleared.
///
///
///
/// Multiple changes to settings may be done before LOAD is written once so all changes takes place at the same time. LOAD can also be done after single setting updates.
///
///
///
/// Registers that needs to be followed by LOAD before settings being applied are:
///
/// - RFCCLKG
///
/// - VIMSCLKG
///
/// - SECDMACLKGR
///
/// - SECDMACLKGS
///
/// - SECDMACLKGDS
///
/// - GPIOCLKGR
///
/// - GPIOCLKGS
///
/// - GPIOCLKGDS
///
/// - GPTCLKGR
///
/// - GPTCLKGS
///
/// - GPTCLKGDS
///
/// - GPTCLKDIV
///
/// - I2CCLKGR
///
/// - I2CCLKGS
///
/// - I2CCLKGDS
///
/// - SSICLKGR
///
/// - SSICLKGS
///
/// - SSICLKGDS
///
/// - UARTCLKGR
///
/// - UARTCLKGS
///
/// - UARTCLKGDS
///
/// - I2SCLKGR
///
/// - I2SCLKGS
///
/// - I2SCLKGDS
///
/// - I2SBCLKSEL
///
/// - I2SCLKCTL
///
/// - I2SMCLKDIV
///
/// - I2SBCLKDIV
///
/// - I2SWCLKDIV
pub mod LOAD {
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
        pub LOAD: B1,
        pub LOAD_DONE: B1,
        pub reserved_2_32: B30,
    }

    impl HwRegister for Register {
        const RESERVED_BITS_MASK: u32 = 0xfffffffc;
        const READ_ONLY_BITS_MASK: u32 = 0x00000002;
        const WRITE_ONLY_BITS_MASK: u32 = 0x00000001;

        fn read(&self) -> u32 {
            u32::from(self.content)
        }

        fn mutate(&mut self, word: u32) {
            let old_val: u32 = self.read();
            let mut new_val: u32 = word;

            // Check if modifies reserved bits
            if old_val & Self::RESERVED_BITS_MASK != new_val & Self::RESERVED_BITS_MASK {
                warn!(target: "cc2650_constants::PRCM::CLKLOADCTL", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::PRCM::CLKLOADCTL",
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
