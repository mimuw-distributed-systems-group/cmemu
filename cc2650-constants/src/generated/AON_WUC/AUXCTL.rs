use cmemu_common::Address;

pub const DISPLAY: &str = "AUXCTL";
pub const OFFSET: u32 = 0x10;
/// 0x40091010
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
pub const RESET_VALUE: u32 = 0x00000000;
pub const RESET_MASK: u32 = 0xffffffff;
/// Reset request for AUX. Writing 1 to this register will assert reset to AUX. The reset will be held until the bit is cleared again.
///
///
///
/// 0: AUX reset pin will be deasserted
///
/// 1: AUX reset pin will be asserted
pub mod RESET_REQ {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 31..=31;
    pub const BIT_MASK: u32 = 0x80000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Enables (1) or disables (0) AUX_SCE execution. AUX_SCE execution will begin when AUX Domain is powered and either this or AUX_SCE:CTL.CLK_EN is set.
///
///
///
/// Setting this bit will assure that AUX_SCE execution starts as soon as AUX power domain is woken up. (  AUX_SCE:CTL.CLK_EN will be reset to 0 if AUX power domain has been off)
///
///
///
/// 0: AUX_SCE execution will be disabled if AUX_SCE:CTL.CLK_EN is 0
///
/// 1: AUX_SCE execution is enabled.
pub mod SCE_RUN_EN {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 2..=2;
    pub const BIT_MASK: u32 = 0x00000004;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Writing 1 sets the software event to the AUX domain, which can be read through AUX_WUC:WUEVFLAGS.AON_SW.
///
///
///
/// This event is normally cleared by AUX_SCE through the AUX_WUC:WUEVCLR.AON_SW. It can also be cleared by writing 0 to this register.
///
///
///
/// Reading 0 means that there is no outstanding software event for AUX.
///
///
///
/// Note that it can take up to 1,5 SCLK_LF clock cycles to clear the event from AUX.
pub mod SWEV {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 1..=1;
    pub const BIT_MASK: u32 = 0x00000002;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Forces the AUX domain into active mode, overriding the requests from AUX_WUC:PWROFFREQ, AUX_WUC:PWRDWNREQ and AUX_WUC:MCUBUSCTL.
///
/// Note that an ongoing AUX_WUC:PWROFFREQ will complete before this bit will set the AUX domain into active mode.
///
///
///
/// MCU must set this bit in order to access the AUX peripherals.
///
/// The AUX domain status can be read from PWRSTAT.AUX_PD_ON
///
///
///
/// 0: AUX is allowed to Power Off, Power Down or Disconnect.
///
/// 1: AUX  Power OFF, Power Down or Disconnect requests will be overruled
pub mod AUX_FORCE_ON {
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
        pub AUX_FORCE_ON: B1,
        pub SWEV: B1,
        pub SCE_RUN_EN: B1,
        pub reserved_3_31: B28,
        pub RESET_REQ: B1,
    }

    impl HwRegister for Register {
        const RESERVED_BITS_MASK: u32 = 0x7ffffff8;
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
                warn!(target: "cc2650_constants::AON_WUC::AUXCTL", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::AON_WUC::AUXCTL",
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
