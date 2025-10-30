use cmemu_common::Address;

pub const DISPLAY: &str = "SYSGPOCTL";
pub const OFFSET: u32 = 0x20;
/// 0x40041020
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
pub const RESET_VALUE: u32 = 0x00000000;
pub const RESET_MASK: u32 = 0xffffffff;
/// RF Core GPO control bit 3. Selects which signal to output on the RF Core GPO line 3.
pub mod GPOCTL3 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 12..=15;
    pub const BIT_MASK: u32 = 0x0000f000;
    pub const BIT_WIDTH: u8 = 4;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// RAT GPO line 3
        pub const RATGPO3: u32 = 15;
        /// RAT GPO line 2
        pub const RATGPO2: u32 = 14;
        /// RAT GPO line 1
        pub const RATGPO1: u32 = 13;
        /// RAT GPO line 0
        pub const RATGPO0: u32 = 12;
        /// RFE GPO line 3
        pub const RFEGPO3: u32 = 11;
        /// RFE GPO line 2
        pub const RFEGPO2: u32 = 10;
        /// RFE GPO line 1
        pub const RFEGPO1: u32 = 9;
        /// RFE GPO line 0
        pub const RFEGPO0: u32 = 8;
        /// MCE GPO line 3
        pub const MCEGPO3: u32 = 7;
        /// MCE GPO line 2
        pub const MCEGPO2: u32 = 6;
        /// MCE GPO line 1
        pub const MCEGPO1: u32 = 5;
        /// MCE GPO line 0
        pub const MCEGPO0: u32 = 4;
        /// CPE GPO line 3
        pub const CPEGPO3: u32 = 3;
        /// CPE GPO line 2
        pub const CPEGPO2: u32 = 2;
        /// CPE GPO line 1
        pub const CPEGPO1: u32 = 1;
        /// CPE GPO line 0
        pub const CPEGPO0: u32 = 0;
    }
}
/// RF Core GPO control bit 2. Selects which signal to output on the RF Core GPO line 2.
pub mod GPOCTL2 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 8..=11;
    pub const BIT_MASK: u32 = 0x00000f00;
    pub const BIT_WIDTH: u8 = 4;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// RAT GPO line 3
        pub const RATGPO3: u32 = 15;
        /// RAT GPO line 2
        pub const RATGPO2: u32 = 14;
        /// RAT GPO line 1
        pub const RATGPO1: u32 = 13;
        /// RAT GPO line 0
        pub const RATGPO0: u32 = 12;
        /// RFE GPO line 3
        pub const RFEGPO3: u32 = 11;
        /// RFE GPO line 2
        pub const RFEGPO2: u32 = 10;
        /// RFE GPO line 1
        pub const RFEGPO1: u32 = 9;
        /// RFE GPO line 0
        pub const RFEGPO0: u32 = 8;
        /// MCE GPO line 3
        pub const MCEGPO3: u32 = 7;
        /// MCE GPO line 2
        pub const MCEGPO2: u32 = 6;
        /// MCE GPO line 1
        pub const MCEGPO1: u32 = 5;
        /// MCE GPO line 0
        pub const MCEGPO0: u32 = 4;
        /// CPE GPO line 3
        pub const CPEGPO3: u32 = 3;
        /// CPE GPO line 2
        pub const CPEGPO2: u32 = 2;
        /// CPE GPO line 1
        pub const CPEGPO1: u32 = 1;
        /// CPE GPO line 0
        pub const CPEGPO0: u32 = 0;
    }
}
/// RF Core GPO control bit 1. Selects which signal to output on the RF Core GPO line 1.
pub mod GPOCTL1 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 4..=7;
    pub const BIT_MASK: u32 = 0x000000f0;
    pub const BIT_WIDTH: u8 = 4;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// RAT GPO line 3
        pub const RATGPO3: u32 = 15;
        /// RAT GPO line 2
        pub const RATGPO2: u32 = 14;
        /// RAT GPO line 1
        pub const RATGPO1: u32 = 13;
        /// RAT GPO line 0
        pub const RATGPO0: u32 = 12;
        /// RFE GPO line 3
        pub const RFEGPO3: u32 = 11;
        /// RFE GPO line 2
        pub const RFEGPO2: u32 = 10;
        /// RFE GPO line 1
        pub const RFEGPO1: u32 = 9;
        /// RFE GPO line 0
        pub const RFEGPO0: u32 = 8;
        /// MCE GPO line 3
        pub const MCEGPO3: u32 = 7;
        /// MCE GPO line 2
        pub const MCEGPO2: u32 = 6;
        /// MCE GPO line 1
        pub const MCEGPO1: u32 = 5;
        /// MCE GPO line 0
        pub const MCEGPO0: u32 = 4;
        /// CPE GPO line 3
        pub const CPEGPO3: u32 = 3;
        /// CPE GPO line 2
        pub const CPEGPO2: u32 = 2;
        /// CPE GPO line 1
        pub const CPEGPO1: u32 = 1;
        /// CPE GPO line 0
        pub const CPEGPO0: u32 = 0;
    }
}
/// RF Core GPO control bit 0. Selects which signal to output on the RF Core GPO line 0.
pub mod GPOCTL0 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 0..=3;
    pub const BIT_MASK: u32 = 0x0000000f;
    pub const BIT_WIDTH: u8 = 4;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// RAT GPO line 3
        pub const RATGPO3: u32 = 15;
        /// RAT GPO line 2
        pub const RATGPO2: u32 = 14;
        /// RAT GPO line 1
        pub const RATGPO1: u32 = 13;
        /// RAT GPO line 0
        pub const RATGPO0: u32 = 12;
        /// RFE GPO line 3
        pub const RFEGPO3: u32 = 11;
        /// RFE GPO line 2
        pub const RFEGPO2: u32 = 10;
        /// RFE GPO line 1
        pub const RFEGPO1: u32 = 9;
        /// RFE GPO line 0
        pub const RFEGPO0: u32 = 8;
        /// MCE GPO line 3
        pub const MCEGPO3: u32 = 7;
        /// MCE GPO line 2
        pub const MCEGPO2: u32 = 6;
        /// MCE GPO line 1
        pub const MCEGPO1: u32 = 5;
        /// MCE GPO line 0
        pub const MCEGPO0: u32 = 4;
        /// CPE GPO line 3
        pub const CPEGPO3: u32 = 3;
        /// CPE GPO line 2
        pub const CPEGPO2: u32 = 2;
        /// CPE GPO line 1
        pub const CPEGPO1: u32 = 1;
        /// CPE GPO line 0
        pub const CPEGPO0: u32 = 0;
    }
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
        pub GPOCTL0: B4,
        pub GPOCTL1: B4,
        pub GPOCTL2: B4,
        pub GPOCTL3: B4,
        pub reserved_16_32: B16,
    }

    impl HwRegister for Register {
        const RESERVED_BITS_MASK: u32 = 0xffff0000;
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
                warn!(target: "cc2650_constants::RFC_DBELL::SYSGPOCTL", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::RFC_DBELL::SYSGPOCTL",
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
