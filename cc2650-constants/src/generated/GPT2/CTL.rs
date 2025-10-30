use cmemu_common::Address;

pub const DISPLAY: &str = "CTL";
pub const OFFSET: u32 = 0xc;
/// 0x4001200c
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
pub const RESET_VALUE: u32 = 0x00000000;
pub const RESET_MASK: u32 = 0xffffffff;
/// GPT Timer B PWM Output Level
///
///
///
/// 0: Output is unaffected.
///
/// 1: Output is inverted.
pub mod TBPWML {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 14..=14;
    pub const BIT_MASK: u32 = 0x00004000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// Inverted
        pub const INVERTED: u32 = 1;
        /// Not inverted
        pub const NORMAL: u32 = 0;
    }
}
/// GPT Timer B Event Mode
///
///
///
/// The values in this register are defined as follows:
///
/// Value Description
///
/// 0x0 Positive edge
///
/// 0x1 Negative edge
///
/// 0x2 Reserved
///
/// 0x3 Both edges
///
/// Note: If PWM output inversion is enabled, edge detection interrupt
///
/// behavior is reversed. Thus, if a positive-edge interrupt trigger
///
/// has been set and the PWM inversion generates a postive
///
/// edge, no event-trigger interrupt asserts. Instead, the interrupt
///
/// is generated on the negative edge of the PWM signal.
pub mod TBEVENT {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 10..=11;
    pub const BIT_MASK: u32 = 0x00000c00;
    pub const BIT_WIDTH: u8 = 2;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// Both edges
        pub const BOTH: u32 = 3;
        /// Negative edge
        pub const NEG: u32 = 1;
        /// Positive edge
        pub const POS: u32 = 0;
    }
}
/// GPT Timer B Stall Enable
pub mod TBSTALL {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 9..=9;
    pub const BIT_MASK: u32 = 0x00000200;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// Timer B freezes counting while the processor is halted by the debugger.
        pub const EN: u32 = 1;
        /// Timer B continues counting while the processor is halted by the debugger.
        pub const DIS: u32 = 0;
    }
}
/// GPT Timer B Enable
pub mod TBEN {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 8..=8;
    pub const BIT_MASK: u32 = 0x00000100;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// Timer B is enabled and begins counting or the capture logic is enabled based on CFG register.
        pub const EN: u32 = 1;
        /// Timer B is disabled.
        pub const DIS: u32 = 0;
    }
}
/// GPT Timer A PWM Output Level
pub mod TAPWML {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 6..=6;
    pub const BIT_MASK: u32 = 0x00000040;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// Inverted
        pub const INVERTED: u32 = 1;
        /// Not inverted
        pub const NORMAL: u32 = 0;
    }
}
/// GPT Timer A Event Mode
///
///
///
/// The values in this register are defined as follows:
///
/// Value Description
///
/// 0x0 Positive edge
///
/// 0x1 Negative edge
///
/// 0x2 Reserved
///
/// 0x3 Both edges
///
/// Note: If PWM output inversion is enabled, edge detection interrupt
///
/// behavior is reversed. Thus, if a positive-edge interrupt trigger
///
/// has been set and the PWM inversion generates a postive
///
/// edge, no event-trigger interrupt asserts. Instead, the interrupt
///
/// is generated on the negative edge of the PWM signal.
pub mod TAEVENT {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 2..=3;
    pub const BIT_MASK: u32 = 0x0000000c;
    pub const BIT_WIDTH: u8 = 2;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// Both edges
        pub const BOTH: u32 = 3;
        /// Negative edge
        pub const NEG: u32 = 1;
        /// Positive edge
        pub const POS: u32 = 0;
    }
}
/// GPT Timer A Stall Enable
pub mod TASTALL {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 1..=1;
    pub const BIT_MASK: u32 = 0x00000002;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// Timer A freezes counting while the processor is halted by the debugger.
        pub const EN: u32 = 1;
        /// Timer A continues counting while the processor is halted by the debugger.
        pub const DIS: u32 = 0;
    }
}
/// GPT Timer A Enable
pub mod TAEN {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 0..=0;
    pub const BIT_MASK: u32 = 0x00000001;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// Timer A is enabled and begins counting or the capture logic is enabled based on the CFG register.
        pub const EN: u32 = 1;
        /// Timer A is disabled.
        pub const DIS: u32 = 0;
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
        pub TAEN: B1,
        pub TASTALL: B1,
        pub TAEVENT: B2,
        pub reserved_4_6: B2,
        pub TAPWML: B1,
        pub reserved_7_8: B1,
        pub TBEN: B1,
        pub TBSTALL: B1,
        pub TBEVENT: B2,
        pub reserved_12_14: B2,
        pub TBPWML: B1,
        pub reserved_15_32: B17,
    }

    impl HwRegister for Register {
        const RESERVED_BITS_MASK: u32 = 0xffffb0b0;
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
                warn!(target: "cc2650_constants::GPT2::CTL", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::GPT2::CTL",
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
