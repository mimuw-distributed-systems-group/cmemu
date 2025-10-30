use cmemu_common::Address;

pub const DISPLAY: &str = "TAMR";
pub const OFFSET: u32 = 0x4;
/// 0x40011004
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
pub const RESET_VALUE: u32 = 0x00000000;
pub const RESET_MASK: u32 = 0xffffffff;
/// Timer Compare Action Select
pub mod TCACT {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 13..=15;
    pub const BIT_MASK: u32 = 0x0000e000;
    pub const BIT_WIDTH: u8 = 3;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// Clear CCP output pin immediately and set on Time-Out
        pub const CLRSET_ON_TO: u32 = 7;
        /// Set CCP output pin immediately and clear on Time-Out
        pub const SETCLR_ON_TO: u32 = 6;
        /// Clear CCP output pin immediately and toggle on Time-Out
        pub const CLRTOG_ON_TO: u32 = 5;
        /// Set CCP output pin immediately and toggle on Time-Out
        pub const SETTOG_ON_TO: u32 = 4;
        /// Set CCP output pin on Time-Out
        pub const SET_ON_TO: u32 = 3;
        /// Clear CCP output pin on Time-Out
        pub const CLR_ON_TO: u32 = 2;
        /// Toggle State on Time-Out
        pub const TOG_ON_TO: u32 = 1;
        /// Disable compare operations
        pub const DIS_CMP: u32 = 0;
    }
}
/// One-Shot/Periodic Interrupt Disable
pub mod TACINTD {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 12..=12;
    pub const BIT_MASK: u32 = 0x00001000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// Time-out interrupt are disabled
        pub const DIS_TO_INTR: u32 = 1;
        /// Time-out interrupt function as normal
        pub const EN_TO_INTR: u32 = 0;
    }
}
/// GPTM Timer A PWM Legacy Operation
///
///
///
/// 0  Legacy operation with CCP pin driven Low when the TAILR
///
/// register is reloaded after the timer reaches 0.
///
///
///
/// 1 CCP is driven High when the TAILR  register is reloaded after the timer reaches 0.
///
///
///
/// This bit is only valid in PWM mode.
pub mod TAPLO {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 11..=11;
    pub const BIT_MASK: u32 = 0x00000800;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// CCP output pin is set to 1 on time-out
        pub const CCP_ON_TO: u32 = 1;
        /// Legacy operation
        pub const LEGACY: u32 = 0;
    }
}
/// Timer A Match Register Update mode
///
///  
///
/// This bit defines when the TAMATCHR and TAPR registers are updated.
///
///
///
/// If the timer is disabled (CTL.TAEN = 0) when this bit is set, TAMATCHR and TAPR are updated when the timer is enabled.
///
/// If the timer is stalled (CTL.TASTALL = 1) when this bit is set, TAMATCHR and TAPR are updated according to the configuration of this bit.
pub mod TAMRSU {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 10..=10;
    pub const BIT_MASK: u32 = 0x00000400;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// Update TAMATCHR and TAPR, if used, on the next time-out.
        pub const TOUPDATE: u32 = 1;
        /// Update TAMATCHR and TAPR, if used, on the next cycle.
        pub const CYCLEUPDATE: u32 = 0;
    }
}
/// GPTM Timer A PWM Interrupt Enable
///
/// This bit enables interrupts in PWM mode on rising, falling, or both edges of the CCP output, as defined by the CTL.TAEVENT
///
/// In addition, when this bit is set and a capture event occurs, Timer A
///
/// automatically generates triggers to the DMA if the trigger capability is enabled by setting the CTL.TAOTE bit and the DMAEV.CAEDMAEN bit respectively.
///
///
///
/// 0 Capture event interrupt is disabled.
///
/// 1 Capture event interrupt is enabled.
///
/// This bit is only valid in PWM mode.
pub mod TAPWMIE {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 9..=9;
    pub const BIT_MASK: u32 = 0x00000200;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// Interrupt is enabled.  This bit is only valid in PWM mode.
        pub const EN: u32 = 1;
        /// Interrupt is disabled.
        pub const DIS: u32 = 0;
    }
}
/// GPT Timer A PWM Interval Load Write
pub mod TAILD {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 8..=8;
    pub const BIT_MASK: u32 = 0x00000100;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// Update the TAR register with the value in the TAILR register on the next timeout. If the prescaler is used, update the TAPS register with the value in the TAPR register on the next timeout.
        pub const TOUPDATE: u32 = 1;
        /// Update the TAR register with the value in the TAILR register on the next clock cycle. If the pre-scaler is used, update the TAPS register with the value in the TAPR register on the next clock cycle.
        pub const CYCLEUPDATE: u32 = 0;
    }
}
/// GPT Timer A Snap-Shot Mode
pub mod TASNAPS {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 7..=7;
    pub const BIT_MASK: u32 = 0x00000080;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// If Timer A is configured in the periodic mode, the actual free-running value of Timer A is loaded at the time-out event into the GPT Timer A (TAR) register.
        pub const EN: u32 = 1;
        /// Snap-shot mode is disabled.
        pub const DIS: u32 = 0;
    }
}
/// GPT Timer A Wait-On-Trigger
pub mod TAWOT {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 6..=6;
    pub const BIT_MASK: u32 = 0x00000040;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// If Timer A is enabled (CTL.TAEN = 1), Timer A does not begin counting until it receives a trigger from the timer in the previous position in the daisy chain. This bit must be clear for GPT Module 0, Timer A. This function is valid for one-shot, periodic, and PWM modes
        pub const WAIT: u32 = 1;
        /// Timer A begins counting as soon as it is enabled.
        pub const NOWAIT: u32 = 0;
    }
}
/// GPT Timer A Match Interrupt Enable
pub mod TAMIE {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 5..=5;
    pub const BIT_MASK: u32 = 0x00000020;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// An interrupt is generated when the match value in TAMATCHR is reached in the one-shot and periodic modes.
        pub const EN: u32 = 1;
        /// The match interrupt is disabled for match events. Additionally, output triggers on match events are prevented.
        pub const DIS: u32 = 0;
    }
}
/// GPT Timer A Count Direction
pub mod TACDIR {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 4..=4;
    pub const BIT_MASK: u32 = 0x00000010;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// The timer counts up. When counting up, the timer starts from a value of 0x0.
        pub const UP: u32 = 1;
        /// The timer counts down.
        pub const DOWN: u32 = 0;
    }
}
/// GPT Timer A Alternate Mode
///
///
///
/// Note: To enable PWM mode, you must also clear TACM and then configure TAMR field to 0x2.
pub mod TAAMS {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 3..=3;
    pub const BIT_MASK: u32 = 0x00000008;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// PWM mode is enabled
        pub const PWM: u32 = 1;
        /// Capture/Compare mode is enabled.
        pub const CAP_COMP: u32 = 0;
    }
}
/// GPT Timer A Capture Mode
pub mod TACM {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 2..=2;
    pub const BIT_MASK: u32 = 0x00000004;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// Edge-Time mode
        pub const EDGTIME: u32 = 1;
        /// Edge-Count mode
        pub const EDGCNT: u32 = 0;
    }
}
/// GPT Timer A Mode
///
///
///
/// 0x0 Reserved
///
/// 0x1 One-Shot Timer mode
///
/// 0x2 Periodic Timer mode
///
/// 0x3 Capture mode
///
/// The Timer mode is based on the timer configuration defined by bits 2:0 in the CFG register
pub mod TAMR {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 0..=1;
    pub const BIT_MASK: u32 = 0x00000003;
    pub const BIT_WIDTH: u8 = 2;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// Capture mode
        pub const CAPTURE: u32 = 3;
        /// Periodic Timer mode
        pub const PERIODIC: u32 = 2;
        /// One-Shot Timer mode
        pub const ONE_SHOT: u32 = 1;
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
        pub TAMR: B2,
        pub TACM: B1,
        pub TAAMS: B1,
        pub TACDIR: B1,
        pub TAMIE: B1,
        pub TAWOT: B1,
        pub TASNAPS: B1,
        pub TAILD: B1,
        pub TAPWMIE: B1,
        pub TAMRSU: B1,
        pub TAPLO: B1,
        pub TACINTD: B1,
        pub TCACT: B3,
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
                warn!(target: "cc2650_constants::GPT1::TAMR", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::GPT1::TAMR",
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
