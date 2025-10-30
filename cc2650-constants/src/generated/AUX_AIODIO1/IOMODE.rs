use cmemu_common::Address;

pub const DISPLAY: &str = "IOMODE";
pub const OFFSET: u32 = 0x4;
/// 0x400c2004
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
pub const RESET_VALUE: u32 = 0x00000000;
pub const RESET_MASK: u32 = 0xffffffff;
/// Select mode for AUXIO\[8i+7\].
pub mod IO7 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 14..=15;
    pub const BIT_MASK: u32 = 0x0000c000;
    pub const BIT_WIDTH: u8 = 2;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// Open-Source Mode:
        ///
        ///
        ///
        /// When GPIODOUT bit 7 is 0: AUXIO\[8i+7\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.
        ///
        ///
        ///
        /// When GPIODOUT bit 7 is 1: AUXIO\[8i+7\] is driven high.
        pub const OPEN_SOURCE: u32 = 3;
        /// Open-Drain Mode:
        ///
        ///
        ///
        /// When GPIODOUT bit 7 is 0: AUXIO\[8i+7\] is driven low.  
        ///
        ///
        ///
        /// When GPIODOUT bit 7 is 1: AUXIO\[8i+7\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.
        pub const OPEN_DRAIN: u32 = 2;
        /// Input Mode:
        ///
        ///
        ///
        /// When GPIODIE bit 7 is 0: AUXIO\[8i+7\] is enabled for analog signal transfer.
        ///
        ///
        ///
        /// When GPIODIE bit 7 is 1: AUXIO\[8i+7\] is enabled for digital input.
        pub const IN: u32 = 1;
        /// Output Mode:
        ///
        ///
        ///
        /// GPIODOUT bit 7 drives AUXIO\[8i+7\].
        pub const OUT: u32 = 0;
    }
}
/// Select mode for AUXIO\[8i+6\].
pub mod IO6 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 12..=13;
    pub const BIT_MASK: u32 = 0x00003000;
    pub const BIT_WIDTH: u8 = 2;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// Open-Source Mode:
        ///
        ///
        ///
        /// When GPIODOUT bit 6 is 0: AUXIO\[8i+6\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.
        ///
        ///
        ///
        /// When GPIODOUT bit 6 is 1: AUXIO\[8i+6\] is driven high.
        pub const OPEN_SOURCE: u32 = 3;
        /// Open-Drain Mode:
        ///
        ///
        ///
        /// When GPIODOUT bit 6 is 0: AUXIO\[8i+6\] is driven low.  
        ///
        ///
        ///
        /// When GPIODOUT bit 6 is 1: AUXIO\[8i+6\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.
        pub const OPEN_DRAIN: u32 = 2;
        /// Input Mode:
        ///
        ///
        ///
        /// When GPIODIE bit 6 is 0: AUXIO\[8i+6\] is enabled for analog signal transfer.
        ///
        ///
        ///
        /// When GPIODIE bit 6 is 1: AUXIO\[8i+6\] is enabled for digital input.
        pub const IN: u32 = 1;
        /// Output Mode:
        ///
        ///
        ///
        /// GPIODOUT bit 6 drives AUXIO\[8i+6\].
        pub const OUT: u32 = 0;
    }
}
/// Select mode for AUXIO\[8i+5\].
pub mod IO5 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 10..=11;
    pub const BIT_MASK: u32 = 0x00000c00;
    pub const BIT_WIDTH: u8 = 2;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// Open-Source Mode:
        ///
        ///
        ///
        /// When GPIODOUT bit 5 is 0: AUXIO\[8i+5\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.
        ///
        ///
        ///
        /// When GPIODOUT bit 5 is 1: AUXIO\[8i+5\] is driven high.
        pub const OPEN_SOURCE: u32 = 3;
        /// Open-Drain Mode:
        ///
        ///
        ///
        /// When GPIODOUT bit 5 is 0: AUXIO\[8i+5\] is driven low.  
        ///
        ///
        ///
        /// When GPIODOUT bit 5 is 1: AUXIO\[8i+5\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.
        pub const OPEN_DRAIN: u32 = 2;
        /// Input Mode:
        ///
        ///
        ///
        /// When GPIODIE bit 5 is 0: AUXIO\[8i+5\] is enabled for analog signal transfer.
        ///
        ///
        ///
        /// When GPIODIE bit 5 is 1: AUXIO\[8i+5\] is enabled for digital input.
        pub const IN: u32 = 1;
        /// Output Mode:
        ///
        ///
        ///
        /// GPIODOUT bit 5 drives AUXIO\[8i+5\].
        pub const OUT: u32 = 0;
    }
}
/// Select mode for AUXIO\[8i+4\].
pub mod IO4 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 8..=9;
    pub const BIT_MASK: u32 = 0x00000300;
    pub const BIT_WIDTH: u8 = 2;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// Open-Source Mode:
        ///
        ///
        ///
        /// When GPIODOUT bit 4 is 0: AUXIO\[8i+4\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.
        ///
        ///
        ///
        /// When GPIODOUT bit 4 is 1: AUXIO\[8i+4\] is driven high.
        pub const OPEN_SOURCE: u32 = 3;
        /// Open-Drain Mode:
        ///
        ///
        ///
        /// When GPIODOUT bit 4 is 0: AUXIO\[8i+4\] is driven low.  
        ///
        ///
        ///
        /// When GPIODOUT bit 4 is 1: AUXIO\[8i+4\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.
        pub const OPEN_DRAIN: u32 = 2;
        /// Input Mode:
        ///
        ///
        ///
        /// When GPIODIE bit 4 is 0: AUXIO\[8i+4\] is enabled for analog signal transfer.
        ///
        ///
        ///
        /// When GPIODIE bit 4 is 1: AUXIO\[8i+4\] is enabled for digital input.
        pub const IN: u32 = 1;
        /// Output Mode:
        ///
        ///
        ///
        /// GPIODOUT bit 4 drives AUXIO\[8i+4\].
        pub const OUT: u32 = 0;
    }
}
/// Select mode for AUXIO\[8i+3\].
pub mod IO3 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 6..=7;
    pub const BIT_MASK: u32 = 0x000000c0;
    pub const BIT_WIDTH: u8 = 2;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// Open-Source Mode:
        ///
        ///
        ///
        /// When GPIODOUT bit 3 is 0: AUXIO\[8i+3\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.
        ///
        ///
        ///
        /// When GPIODOUT bit 3 is 1: AUXIO\[8i+3\] is driven high.
        pub const OPEN_SOURCE: u32 = 3;
        /// Open-Drain Mode:
        ///
        ///
        ///
        /// When GPIODOUT bit 3 is 0: AUXIO\[8i+3\] is driven low.  
        ///
        ///
        ///
        /// When GPIODOUT bit 3 is 1: AUXIO\[8i+3\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.
        pub const OPEN_DRAIN: u32 = 2;
        /// Input Mode:
        ///
        ///
        ///
        /// When GPIODIE bit 3 is 0: AUXIO\[8i+3\] is enabled for analog signal transfer.
        ///
        ///
        ///
        /// When GPIODIE bit 3 is 1: AUXIO\[8i+3\] is enabled for digital input.
        pub const IN: u32 = 1;
        /// Output Mode:
        ///
        ///
        ///
        /// GPIODOUT bit 3 drives AUXIO\[8i+3\].
        pub const OUT: u32 = 0;
    }
}
/// Select mode for AUXIO\[8i+2\].
pub mod IO2 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 4..=5;
    pub const BIT_MASK: u32 = 0x00000030;
    pub const BIT_WIDTH: u8 = 2;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// Open-Source Mode:
        ///
        ///
        ///
        /// When GPIODOUT bit 2 is 0: AUXIO\[8i+2\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.
        ///
        ///
        ///
        /// When GPIODOUT bit 2 is 1: AUXIO\[8i+2\] is driven high.
        pub const OPEN_SOURCE: u32 = 3;
        /// Open-Drain Mode:
        ///
        ///
        ///
        /// When GPIODOUT bit 2 is 0: AUXIO\[8i+2\] is driven low.  
        ///
        ///
        ///
        /// When GPIODOUT bit 2 is 1: AUXIO\[8i+2\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.
        pub const OPEN_DRAIN: u32 = 2;
        /// Input Mode:
        ///
        ///
        ///
        /// When GPIODIE bit 2 is 0: AUXIO\[8i+2\] is enabled for analog signal transfer.
        ///
        ///
        ///
        /// When GPIODIE bit 2 is 1: AUXIO\[8i+2\] is enabled for digital input.
        pub const IN: u32 = 1;
        /// Output Mode:
        ///
        ///
        ///
        /// GPIODOUT bit 2 drives AUXIO\[8i+2\].
        pub const OUT: u32 = 0;
    }
}
/// Select mode for AUXIO\[8i+1\].
pub mod IO1 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 2..=3;
    pub const BIT_MASK: u32 = 0x0000000c;
    pub const BIT_WIDTH: u8 = 2;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// Open-Source Mode:
        ///
        ///
        ///
        /// When GPIODOUT bit 1 is 0: AUXIO\[8i+1\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.
        ///
        ///
        ///
        /// When GPIODOUT bit 1 is 1: AUXIO\[8i+1\] is driven high.
        pub const OPEN_SOURCE: u32 = 3;
        /// Open-Drain Mode:
        ///
        ///
        ///
        /// When GPIODOUT bit 1 is 0: AUXIO\[8i+1\] is driven low.  
        ///
        ///
        ///
        /// When GPIODOUT bit 1 is 1: AUXIO\[8i+1\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.
        pub const OPEN_DRAIN: u32 = 2;
        /// Input Mode:
        ///
        ///
        ///
        /// When GPIODIE bit 1 is 0: AUXIO\[8i+1\] is enabled for analog signal transfer.
        ///
        ///
        ///
        /// When GPIODIE bit 1 is 1: AUXIO\[8i+1\] is enabled for digital input.
        pub const IN: u32 = 1;
        /// Output Mode:
        ///
        ///
        ///
        /// GPIODOUT bit 1 drives AUXIO\[8i+1\].
        pub const OUT: u32 = 0;
    }
}
/// Select mode for AUXIO\[8i+0\].
pub mod IO0 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 0..=1;
    pub const BIT_MASK: u32 = 0x00000003;
    pub const BIT_WIDTH: u8 = 2;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// Open-Source Mode:
        ///
        ///
        ///
        /// When GPIODOUT bit 0 is 0: AUXIO\[8i+0\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.
        ///
        ///
        ///
        /// When GPIODOUT bit 0 is 1: AUXIO\[8i+0\] is driven high.
        pub const OPEN_SOURCE: u32 = 3;
        /// Open-Drain Mode:
        ///
        ///
        ///
        /// When GPIODOUT bit 0 is 0: AUXIO\[8i+0\] is driven low.  
        ///
        ///
        ///
        /// When GPIODOUT bit 0 is 1: AUXIO\[8i+0\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.
        pub const OPEN_DRAIN: u32 = 2;
        /// Input Mode:
        ///
        ///
        ///
        /// When GPIODIE bit 0 is 0: AUXIO\[8i+0\] is enabled for analog signal transfer.
        ///
        ///
        ///
        /// When GPIODIE bit 0 is 1: AUXIO\[8i+0\] is enabled for digital input.
        pub const IN: u32 = 1;
        /// Output Mode:
        ///
        ///
        ///
        /// GPIODOUT bit 0 drives AUXIO\[8i+0\].
        pub const OUT: u32 = 0;
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
        pub IO0: B2,
        pub IO1: B2,
        pub IO2: B2,
        pub IO3: B2,
        pub IO4: B2,
        pub IO5: B2,
        pub IO6: B2,
        pub IO7: B2,
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
                warn!(target: "cc2650_constants::AUX_AIODIO1::IOMODE", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::AUX_AIODIO1::IOMODE",
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
