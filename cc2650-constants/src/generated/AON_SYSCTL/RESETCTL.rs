use cmemu_common::Address;

pub const DISPLAY: &str = "RESETCTL";
pub const OFFSET: u32 = 0x4;
/// 0x40090004
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
pub const RESET_VALUE: u32 = 0x000000e0;
pub const RESET_MASK: u32 = 0xffffffff;
/// Cold reset register. Writing 1 to this bitfield will reset the entire chip and cause boot code to run again.
///
///
///
/// 0: No effect
///
/// 1: Generate system reset. Appears as SYSRESET in RESET_SRC.
pub mod SYSRESET {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 31..=31;
    pub const BIT_MASK: u32 = 0x80000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Internal. Only to be used through TI provided API.
pub mod BOOT_DET_1_CLR {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 25..=25;
    pub const BIT_MASK: u32 = 0x02000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Internal. Only to be used through TI provided API.
pub mod BOOT_DET_0_CLR {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 24..=24;
    pub const BIT_MASK: u32 = 0x01000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Internal. Only to be used through TI provided API.
pub mod BOOT_DET_1_SET {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 17..=17;
    pub const BIT_MASK: u32 = 0x00020000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Internal. Only to be used through TI provided API.
pub mod BOOT_DET_0_SET {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 16..=16;
    pub const BIT_MASK: u32 = 0x00010000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// A Wakeup from SHUTDOWN on an IO event has occurred, or a wakeup from SHUTDOWN has occurred as a result of the debugger being attached.. (TCK pin being forced low)
///
///
///
/// Please refer to \[IOC:IOCFGn,.WU_CFG\] for configuring the IO's as wakeup sources.
///
///
///
/// 0: Wakeup occurred from cold reset or brown out as seen in RESET_SRC
///
/// 1: A wakeup has occurred from SHUTDOWN
///
///
///
/// Note: This flag can not be cleared and will therefor remain valid untill poweroff/reset
pub mod WU_FROM_SD {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 15..=15;
    pub const BIT_MASK: u32 = 0x00008000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// A wakeup from SHUTDOWN on an IO event has occurred
///
///
///
/// Please refer to \[IOC:IOCFGn,.WU_CFG\] for configuring the IO's as wakeup sources.
///
///
///
/// 0: The wakeup did not occur from SHUTDOWN on an IO event
///
/// 1: A wakeup from SHUTDOWN occurred from an IO event
///
///
///
/// The case where WU_FROM_SD is asserted but this bitfield is not asserted will only occur in a debug session. The boot code will not proceed with wakeup from SHUTDOWN procedure until this bitfield is asserted as well.
///
///
///
/// Note: This flag can not be cleared and will therefor remain valid untill poweroff/reset
pub mod GPIO_WU_FROM_SD {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 14..=14;
    pub const BIT_MASK: u32 = 0x00004000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod BOOT_DET_1 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 13..=13;
    pub const BIT_MASK: u32 = 0x00002000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Internal. Only to be used through TI provided API.
pub mod BOOT_DET_0 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 12..=12;
    pub const BIT_MASK: u32 = 0x00001000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Override of VDDS_LOSS_EN
///
///
///
/// 0: Brown out detect of VDDS is ignored, unless VDDS_LOSS_EN=1
///
/// 1: Brown out detect of VDDS generates system reset (regardless of  VDDS_LOSS_EN)
///
///
///
/// This bit can be locked
pub mod VDDS_LOSS_EN_OVR {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 11..=11;
    pub const BIT_MASK: u32 = 0x00000800;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Override of VDDR_LOSS_EN
///
///
///
/// 0: Brown out detect of VDDR is ignored, unless VDDR_LOSS_EN=1
///
/// 1: Brown out detect of VDDR generates system reset (regardless of  VDDR_LOSS_EN)
///
///
///
/// This bit can be locked
pub mod VDDR_LOSS_EN_OVR {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 10..=10;
    pub const BIT_MASK: u32 = 0x00000400;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Override of VDD_LOSS_EN
///
///
///
/// 0: Brown out detect of VDD is ignored, unless VDD_LOSS_EN=1
///
/// 1: Brown out detect of VDD generates system reset (regardless of  VDD_LOSS_EN)
///
///
///
/// This bit can be locked
pub mod VDD_LOSS_EN_OVR {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 9..=9;
    pub const BIT_MASK: u32 = 0x00000200;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Controls reset generation in case VDDS is lost
///
///
///
/// 0: Brown out detect of VDDS is ignored, unless VDDS_LOSS_EN_OVR=1
///
/// 1: Brown out detect of VDDS generates system reset
pub mod VDDS_LOSS_EN {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 7..=7;
    pub const BIT_MASK: u32 = 0x00000080;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = true;
}
/// Controls reset generation in case VDDR is lost
///
///
///
/// 0: Brown out detect of VDDR is ignored, unless VDDR_LOSS_EN_OVR=1
///
/// 1: Brown out detect of VDDR generates system reset
pub mod VDDR_LOSS_EN {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 6..=6;
    pub const BIT_MASK: u32 = 0x00000040;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = true;
}
/// Controls reset generation in case VDD is lost
///
///
///
/// 0: Brown out detect of VDD is ignored, unless VDD_LOSS_EN_OVR=1
///
/// 1: Brown out detect of VDD generates system reset
pub mod VDD_LOSS_EN {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 5..=5;
    pub const BIT_MASK: u32 = 0x00000020;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = true;
}
/// Controls reset generation in case SCLK_LF is lost.  (provided that clock loss detection is enabled by DDI_0_OSC:CTL0.CLK_LOSS_EN)
///
///
///
/// Note: Clock loss reset generation must be disabled before SCLK_LF clock source is changed in  DDI_0_OSC:CTL0.SCLK_LF_SRC_SEL and remain disabled untill the change is confirmed in DDI_0_OSC:STAT0.SCLK_LF_SRC. Failure to do so may result in a spurious system reset. Clock loss reset generation can be disabled through this bitfield or by clearing  DDI_0_OSC:CTL0.CLK_LOSS_EN
///
///  
///
/// 0: Clock loss is ignored
///
/// 1: Clock loss generates system reset
pub mod CLK_LOSS_EN {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 4..=4;
    pub const BIT_MASK: u32 = 0x00000010;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Shows the source of the last system reset:
///
/// Occurrence of one of the reset sources may trigger several other reset sources as essential parts of the system are undergoing reset. This field will report the root cause of the reset (not the other resets that are consequence of the system reset).
///
/// To support this feature the actual register is not captured before the reset source being released. If a new reset source is triggered, in a window of four  32 kHz periods after the previous has been released,  this register may indicate Power on reset as source.
pub mod RESET_SRC {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 1..=3;
    pub const BIT_MASK: u32 = 0x0000000e;
    pub const BIT_WIDTH: u8 = 3;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
    pub use self::Named as E;
    pub mod Named {
        /// Software reset via PRCM warm reset request
        pub const WARMRESET: u32 = 7;
        /// Software reset via SYSRESET register
        pub const SYSRESET: u32 = 6;
        /// Clock loss detect
        pub const CLK_LOSS: u32 = 5;
        /// Brown out detect on VDDR
        pub const VDDR_LOSS: u32 = 4;
        /// Brown out detect on VDD
        pub const VDD_LOSS: u32 = 3;
        /// Brown out detect on VDDS
        pub const VDDS_LOSS: u32 = 2;
        /// Reset pin
        pub const PIN_RESET: u32 = 1;
        /// Power on reset
        pub const PWR_ON: u32 = 0;
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
        pub reserved_0_1: B1,
        pub RESET_SRC: B3,
        pub CLK_LOSS_EN: B1,
        pub VDD_LOSS_EN: B1,
        pub VDDR_LOSS_EN: B1,
        pub VDDS_LOSS_EN: B1,
        pub reserved_8_9: B1,
        pub VDD_LOSS_EN_OVR: B1,
        pub VDDR_LOSS_EN_OVR: B1,
        pub VDDS_LOSS_EN_OVR: B1,
        pub BOOT_DET_0: B1,
        pub BOOT_DET_1: B1,
        pub GPIO_WU_FROM_SD: B1,
        pub WU_FROM_SD: B1,
        pub BOOT_DET_0_SET: B1,
        pub BOOT_DET_1_SET: B1,
        pub reserved_18_24: B6,
        pub BOOT_DET_0_CLR: B1,
        pub BOOT_DET_1_CLR: B1,
        pub reserved_26_31: B5,
        pub SYSRESET: B1,
    }

    impl HwRegister for Register {
        const RESERVED_BITS_MASK: u32 = 0x7cfc0101;
        const READ_ONLY_BITS_MASK: u32 = 0x0000f00e;
        const WRITE_ONLY_BITS_MASK: u32 = 0x80000000;

        fn read(&self) -> u32 {
            u32::from(self.content)
        }

        fn mutate(&mut self, word: u32) {
            let old_val: u32 = self.read();
            let mut new_val: u32 = word;

            // Check if modifies reserved bits
            if old_val & Self::RESERVED_BITS_MASK != new_val & Self::RESERVED_BITS_MASK {
                warn!(target: "cc2650_constants::AON_SYSCTL::RESETCTL", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::AON_SYSCTL::RESETCTL",
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
