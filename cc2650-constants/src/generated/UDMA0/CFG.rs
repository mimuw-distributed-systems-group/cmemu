use cmemu_common::Address;

pub const DISPLAY: &str = "CFG";
pub const OFFSET: u32 = 0x4;
/// 0x40020004
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
pub const RESET_VALUE: u32 = 0x00000000;
pub const RESET_MASK: u32 = 0xffffffff;
/// Sets the AHB-Lite bus protocol protection state by controlling the AHB signal HProt\[3:1\] as follows:
///
///
///
/// Bit \[7\] Controls HProt\[3\] to indicate if a cacheable access is occurring.
///
/// Bit \[6\] Controls HProt\[2\] to indicate if a bufferable access is occurring.
///
/// Bit \[5\] Controls HProt\[1\] to indicate if a privileged access is occurring.
///
///
///
/// When bit \[n\] = 1 then the corresponding HProt bit is high.
///
/// When bit \[n\] = 0 then the corresponding HProt bit is low.
///
///
///
/// This field controls HProt\[3:1\] signal for all transactions initiated by uDMA except two transactions below:
///
/// - the read from the address indicated by source address pointer
///
/// - the write to the address indicated by destination address pointer
///
/// HProt\[3:1\] for these two exceptions can be controlled by dedicated fields in the channel configutation descriptor.
pub mod PRTOCTRL {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 5..=7;
    pub const BIT_MASK: u32 = 0x000000e0;
    pub const BIT_WIDTH: u8 = 3;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Enables the controller:
///
///
///
/// 0: Disables the controller
///
/// 1: Enables the controller
pub mod MASTERENABLE {
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
        pub MASTERENABLE: B1,
        pub reserved_1_5: B4,
        pub PRTOCTRL: B3,
        pub reserved_8_32: B24,
    }

    impl HwRegister for Register {
        const RESERVED_BITS_MASK: u32 = 0xffffff1e;
        const READ_ONLY_BITS_MASK: u32 = 0x00000000;
        const WRITE_ONLY_BITS_MASK: u32 = 0x000000e1;

        fn read(&self) -> u32 {
            u32::from(self.content)
        }

        fn mutate(&mut self, word: u32) {
            let old_val: u32 = self.read();
            let mut new_val: u32 = word;

            // Check if modifies reserved bits
            if old_val & Self::RESERVED_BITS_MASK != new_val & Self::RESERVED_BITS_MASK {
                warn!(target: "cc2650_constants::UDMA0::CFG", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::UDMA0::CFG",
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
