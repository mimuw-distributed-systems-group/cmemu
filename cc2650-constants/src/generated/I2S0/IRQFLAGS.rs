use cmemu_common::Address;

pub const DISPLAY: &str = "IRQFLAGS";
pub const OFFSET: u32 = 0x74;
/// 0x40021074
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
pub const RESET_VALUE: u32 = 0x00000000;
pub const RESET_MASK: u32 = 0xffffffff;
/// Set when condition for this bit field event occurs (auto cleared when input pointer is updated - AIFINPTRNEXT), see description of AIFINPTRNEXT register for details.
pub mod AIF_DMA_IN {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 5..=5;
    pub const BIT_MASK: u32 = 0x00000020;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Set when condition for this bit field event occurs (auto cleared when output pointer is updated - AIFOUTPTRNEXT), see description of AIFOUTPTRNEXT register for details
pub mod AIF_DMA_OUT {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 4..=4;
    pub const BIT_MASK: u32 = 0x00000010;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Set when the sample stamp generator does not detect a positive WCLK edge for  65535 clk periods. This signalizes that the internal or external BCLK and WCLK generator source has been disabled.
///
///
///
/// The bit is sticky and may only be cleared by software (by writing '1' to IRQCLR.WCLK_TIMEOUT).
pub mod WCLK_TIMEOUT {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 3..=3;
    pub const BIT_MASK: u32 = 0x00000008;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Set when a DMA operation is not completed in time (that is audio output buffer underflow, or audio input buffer overflow).
///
/// This error requires a complete restart since word synchronization has been lost. The bit is sticky and may only be cleared by software (by writing '1' to IRQCLR.BUS_ERR).
///
///
///
/// Note that DMA initiated transactions to illegal addresses will not trigger an interrupt. The response to such transactions is undefined.
pub mod BUS_ERR {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 2..=2;
    pub const BIT_MASK: u32 = 0x00000004;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Set when:
///
/// - An unexpected WCLK edge occurs during the data delay period of a phase.  Note unexpected WCLK edges during the word and idle periods of the phase are not detected.
///
/// -  In dual-phase mode, when two WCLK edges are less than 4 BCLK cycles apart.
///
/// -  In single-phase mode, when a WCLK pulse occurs before the last channel.
///
/// This error requires a complete restart since word synchronization has been lost. The bit is sticky and may only be cleared by software (by writing '1' to IRQCLR.WCLK_ERR).
pub mod WCLK_ERR {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 1..=1;
    pub const BIT_MASK: u32 = 0x00000002;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Set when AIFINPTRNEXT or AIFOUTPTRNEXT has not been loaded with the next block address in time.
///
/// This error requires a complete restart since word synchronization has been lost. The bit is sticky and may only be cleared by software (by writing '1' to IRQCLR.PTR_ERR).
pub mod PTR_ERR {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 0..=0;
    pub const BIT_MASK: u32 = 0x00000001;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
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
        pub PTR_ERR: B1,
        pub WCLK_ERR: B1,
        pub BUS_ERR: B1,
        pub WCLK_TIMEOUT: B1,
        pub AIF_DMA_OUT: B1,
        pub AIF_DMA_IN: B1,
        pub reserved_6_32: B26,
    }

    impl HwRegister for Register {
        const RESERVED_BITS_MASK: u32 = 0xffffffc0;
        const READ_ONLY_BITS_MASK: u32 = 0x0000003f;
        const WRITE_ONLY_BITS_MASK: u32 = 0x00000000;

        fn read(&self) -> u32 {
            u32::from(self.content)
        }

        fn mutate(&mut self, word: u32) {
            let old_val: u32 = self.read();
            let mut new_val: u32 = word;

            // Check if modifies reserved bits
            if old_val & Self::RESERVED_BITS_MASK != new_val & Self::RESERVED_BITS_MASK {
                warn!(target: "cc2650_constants::I2S0::IRQFLAGS", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::I2S0::IRQFLAGS",
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
