use cmemu_common::Address;

pub const DISPLAY: &str = "STAT";
pub const OFFSET: u32 = 0x4;
/// 0x400c4004
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
pub const RESET_VALUE: u32 = 0x00000006;
pub const RESET_MASK: u32 = 0xffffffff;
/// TDC measurement saturation flag.
///
///
///
/// 0: Conversion has not saturated.
///
/// 1: Conversion stopped due to saturation.
///
///
///
/// This field is cleared when a new measurement is started or when CLR_RESULT is written to CTL.CMD.
pub mod SAT {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 7..=7;
    pub const BIT_MASK: u32 = 0x00000080;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// TDC measurement complete flag.
///
///
///
/// 0: TDC measurement has not yet completed.
///
/// 1: TDC measurement has completed.
///
///
///
/// This field clears when a new TDC measurement starts or when you write CLR_RESULT to CTL.CMD.
pub mod DONE {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 6..=6;
    pub const BIT_MASK: u32 = 0x00000040;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// TDC state machine status.
pub mod STATE {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 0..=5;
    pub const BIT_MASK: u32 = 0x0000003f;
    pub const BIT_WIDTH: u8 = 6;
    pub const RESET_VALUE: u32 = 0x6;
    pub const WRITABLE: bool = false;
    pub use self::Named as E;
    pub mod Named {
        /// Current state is TDC_FORCESTOP.
        ///
        /// You wrote ABORT to CTL.CMD to abort the TDC measurement.
        pub const FORCE_STOP: u32 = 46;
        /// Current state is TDC_WAIT_STARTFALL.
        ///
        /// The fast-counter circuit waits for a falling edge on the start event.
        pub const START_FALL: u32 = 30;
        /// Current state is TDC_STATE_WAIT_CLRCNT_DONE.
        ///
        /// The state machine waits for fast-counter circuit to finish reset.
        pub const WAIT_CLR_CNT_DONE: u32 = 22;
        /// Current state is TDC_STATE_POR.
        ///
        /// This is the reset state.
        pub const POR: u32 = 15;
        /// Current state is TDC_STATE_GETRESULTS.
        ///
        /// The state machine copies the counter value from the fast-counter circuit.
        pub const GET_RESULT: u32 = 14;
        /// Current state is TDC_STATE_WAIT_STOPCNTDOWN.
        ///
        /// The fast-counter circuit looks for the stop condition. It will ignore a number of stop events configured in TRIGCNTLOAD.CNT.
        pub const WAIT_STOP_CNTDWN: u32 = 12;
        /// Current state is TDC_STATE_WAIT_STOP.
        ///
        /// The state machine waits for the fast-counter circuit to stop.
        pub const WAIT_STOP: u32 = 8;
        /// Current state is TDC_STATE_CLRCNT. The fast-counter circuit is reset.
        pub const CLR_CNT: u32 = 7;
        /// Current state is TDC_STATE_IDLE.
        ///
        /// This is the default state after reset and abortion. State will change when you write CTL.CMD to either RUN_SYNC_START or RUN.
        pub const IDLE: u32 = 6;
        /// Current state is TDC_STATE_WAIT_STARTSTOPCNTEN.
        ///
        /// The fast-counter circuit looks for the start condition. The state machine waits for the fast-counter to increment.
        pub const WAIT_START_STOP_CNT_EN: u32 = 4;
        /// Current state is TDC_STATE_WAIT_START.
        ///
        /// The fast-counter circuit looks for the start condition. The state machine waits for the fast-counter to increment.
        pub const WAIT_START: u32 = 0;
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
        pub STATE: B6,
        pub DONE: B1,
        pub SAT: B1,
        pub reserved_8_32: B24,
    }

    impl HwRegister for Register {
        const RESERVED_BITS_MASK: u32 = 0xffffff00;
        const READ_ONLY_BITS_MASK: u32 = 0x000000ff;
        const WRITE_ONLY_BITS_MASK: u32 = 0x00000000;

        fn read(&self) -> u32 {
            u32::from(self.content)
        }

        fn mutate(&mut self, word: u32) {
            let old_val: u32 = self.read();
            let mut new_val: u32 = word;

            // Check if modifies reserved bits
            if old_val & Self::RESERVED_BITS_MASK != new_val & Self::RESERVED_BITS_MASK {
                warn!(target: "cc2650_constants::AUX_TDCIF::STAT", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::AUX_TDCIF::STAT",
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
