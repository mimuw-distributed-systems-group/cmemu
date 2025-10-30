use cmemu_common::Address;

pub const DISPLAY: &str = "RIS";
pub const OFFSET: u32 = 0x1c;
/// 0x4001001c
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
pub const RESET_VALUE: u32 = 0x00000000;
pub const RESET_MASK: u32 = 0xffffffff;
/// GPT Timer B DMA Done Raw Interrupt Status
///
///
///
/// 0: Transfer has not completed
///
/// 1: Transfer has completed
pub mod DMABRIS {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 13..=13;
    pub const BIT_MASK: u32 = 0x00002000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// GPT Timer B Match Raw  Interrupt
///
///
///
/// 0:  The match value has not been reached
///
/// 1:  The match value is reached.
///
///
///
/// TBMR.TBMIE is set, and the match values in TBMATCHR and optionally TBPMR have been reached when configured in one-shot or periodic mode.
pub mod TBMRIS {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 11..=11;
    pub const BIT_MASK: u32 = 0x00000800;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// GPT Timer B Capture Mode Event Raw Interrupt
///
///
///
/// 0:  The event has not occured.
///
/// 1:  The event has occured.
///
///
///
/// This interrupt asserts when the subtimer is configured in Input Edge-Time mode
pub mod CBERIS {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 10..=10;
    pub const BIT_MASK: u32 = 0x00000400;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// GPT Timer B Capture Mode Match Raw Interrupt
///
///
///
/// 0:  The capture mode match for Timer B has not occurred.
///
/// 1:  A capture mode match has occurred for Timer B. This interrupt
///
/// asserts when the values in the TBR and TBPR
///
/// match the values in the TBMATCHR and TBPMR
///
/// when configured in Input Edge-Time mode.
///
///
///
/// This bit is cleared by writing a 1 to the ICLR.CBMCINT bit.
pub mod CBMRIS {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 9..=9;
    pub const BIT_MASK: u32 = 0x00000200;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// GPT Timer B Time-out Raw Interrupt
///
///
///
/// 0:  Timer B has not timed out
///
/// 1:  Timer B has timed out.
///
///
///
/// This interrupt is asserted when a one-shot or periodic mode timer reaches its count limit. The count limit is 0 or the value loaded into TBILR, depending on the count direction.
pub mod TBTORIS {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 8..=8;
    pub const BIT_MASK: u32 = 0x00000100;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// GPT Timer A DMA Done Raw Interrupt Status
///
///
///
/// 0: Transfer has not completed
///
/// 1: Transfer has completed
pub mod DMAARIS {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 5..=5;
    pub const BIT_MASK: u32 = 0x00000020;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// GPT Timer A Match Raw  Interrupt
///
///
///
/// 0:  The match value has not been reached
///
/// 1:  The match value is reached.
///
///
///
/// TAMR.TAMIE is set, and the match values in TAMATCHR and optionally TAPMR have been reached when configured in one-shot or periodic mode.
pub mod TAMRIS {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 4..=4;
    pub const BIT_MASK: u32 = 0x00000010;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// GPT Timer A Capture Mode Event Raw Interrupt
///
///
///
/// 0:  The event has not occured.
///
/// 1:  The event has occured.
///
///
///
/// This interrupt asserts when the subtimer is configured in Input Edge-Time mode
pub mod CAERIS {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 2..=2;
    pub const BIT_MASK: u32 = 0x00000004;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// GPT Timer A Capture Mode Match Raw Interrupt
///
///
///
/// 0:  The capture mode match for Timer A has not occurred.
///
/// 1:  A capture mode match has occurred for Timer A. This interrupt
///
/// asserts when the values in the TAR and TAPR
///
/// match the values in the TAMATCHR and TAPMR
///
/// when configured in Input Edge-Time mode.
///
///
///
/// This bit is cleared by writing a 1 to the ICLR.CAMCINT bit.
pub mod CAMRIS {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 1..=1;
    pub const BIT_MASK: u32 = 0x00000002;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// GPT Timer A Time-out Raw Interrupt
///
///
///
/// 0:  Timer A has not timed out
///
/// 1:  Timer A has timed out.
///
///
///
/// This interrupt is asserted when a one-shot or periodic mode timer reaches its count limit. The count limit is 0 or the value loaded into TAILR, depending on the count direction.
pub mod TATORIS {
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
        pub TATORIS: B1,
        pub CAMRIS: B1,
        pub CAERIS: B1,
        pub reserved_3_4: B1,
        pub TAMRIS: B1,
        pub DMAARIS: B1,
        pub reserved_6_8: B2,
        pub TBTORIS: B1,
        pub CBMRIS: B1,
        pub CBERIS: B1,
        pub TBMRIS: B1,
        pub reserved_12_13: B1,
        pub DMABRIS: B1,
        pub reserved_14_32: B18,
    }

    impl HwRegister for Register {
        const RESERVED_BITS_MASK: u32 = 0xffffd0c8;
        const READ_ONLY_BITS_MASK: u32 = 0x00002f37;
        const WRITE_ONLY_BITS_MASK: u32 = 0x00000000;

        fn read(&self) -> u32 {
            u32::from(self.content)
        }

        fn mutate(&mut self, word: u32) {
            let old_val: u32 = self.read();
            let mut new_val: u32 = word;

            // Check if modifies reserved bits
            if old_val & Self::RESERVED_BITS_MASK != new_val & Self::RESERVED_BITS_MASK {
                warn!(target: "cc2650_constants::GPT0::RIS", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::GPT0::RIS",
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
