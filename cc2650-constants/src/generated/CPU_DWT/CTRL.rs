use cmemu_common::Address;

pub const DISPLAY: &str = "CTRL";
pub const OFFSET: u32 = 0x0;
/// 0xe0001000
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
pub const RESET_VALUE: u32 = 0x40000000;
pub const RESET_MASK: u32 = 0xffffffff;
/// When set, CYCCNT is not supported.
pub mod NOCYCCNT {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 25..=25;
    pub const BIT_MASK: u32 = 0x02000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// When set, FOLDCNT, LSUCNT, SLEEPCNT, EXCCNT, and CPICNT are not supported.
pub mod NOPRFCNT {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 24..=24;
    pub const BIT_MASK: u32 = 0x01000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Enables Cycle count event. Emits an event when the POSTCNT counter triggers it. See CYCTAP and POSTPRESET for details. This event is only emitted if PCSAMPLEENA is disabled. PCSAMPLEENA overrides the setting of this bit.
///
///
///
/// 0: Cycle count events disabled
///
/// 1: Cycle count events enabled
pub mod CYCEVTENA {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 22..=22;
    pub const BIT_MASK: u32 = 0x00400000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Enables Folded instruction count event. Emits an event when FOLDCNT overflows (every 256 cycles of folded instructions). A folded instruction is one that does not incur even one cycle to execute. For example, an IT instruction is folded away and so does not use up one cycle.
///
///
///
/// 0: Folded instruction count events disabled.
///
/// 1: Folded instruction count events enabled.
pub mod FOLDEVTENA {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 21..=21;
    pub const BIT_MASK: u32 = 0x00200000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Enables LSU count event. Emits an event when LSUCNT overflows (every 256 cycles of LSU operation). LSU counts include all LSU costs after the initial cycle for the instruction.
///
///
///
/// 0: LSU count events disabled.
///
/// 1: LSU count events enabled.
pub mod LSUEVTENA {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 20..=20;
    pub const BIT_MASK: u32 = 0x00100000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Enables Sleep count event. Emits an event when SLEEPCNT overflows (every 256 cycles that the processor is sleeping).
///
///
///
/// 0: Sleep count events disabled.
///
/// 1: Sleep count events enabled.
pub mod SLEEPEVTENA {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 19..=19;
    pub const BIT_MASK: u32 = 0x00080000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Enables Interrupt overhead event. Emits an event when EXCCNT overflows (every 256 cycles of interrupt overhead).
///
///
///
/// 0x0: Interrupt overhead event disabled.
///
/// 0x1: Interrupt overhead event enabled.
pub mod EXCEVTENA {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 18..=18;
    pub const BIT_MASK: u32 = 0x00040000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Enables CPI count event. Emits an event when CPICNT overflows (every 256 cycles of multi-cycle instructions).
///
///
///
/// 0: CPI counter events disabled.
///
/// 1: CPI counter events enabled.
pub mod CPIEVTENA {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 17..=17;
    pub const BIT_MASK: u32 = 0x00020000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Enables Interrupt event tracing.
///
///
///
/// 0: Interrupt event trace disabled.
///
/// 1: Interrupt event trace enabled.
pub mod EXCTRCENA {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 16..=16;
    pub const BIT_MASK: u32 = 0x00010000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Enables PC Sampling event. A PC sample event is emitted when the POSTCNT counter triggers it. See CYCTAP and POSTPRESET for details. Enabling this bit overrides CYCEVTENA.
///
///
///
/// 0: PC Sampling event disabled.
///
/// 1: Sampling event enabled.
pub mod PCSAMPLEENA {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 12..=12;
    pub const BIT_MASK: u32 = 0x00001000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Selects a synchronization packet rate. CYCCNTENA and CPU_ITM:TCR.SYNCENA must also be enabled for this feature.
///
/// Synchronization packets (if enabled) are generated on tap transitions (0 to1 or 1 to 0).
pub mod SYNCTAP {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 10..=11;
    pub const BIT_MASK: u32 = 0x00000c00;
    pub const BIT_WIDTH: u8 = 2;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// Tap at bit 28 of CYCCNT
        pub const BIT28: u32 = 3;
        /// Tap at bit 26 of CYCCNT
        pub const BIT26: u32 = 2;
        /// Tap at bit 24 of CYCCNT
        pub const BIT24: u32 = 1;
        /// Disabled. No synchronization packets
        pub const DIS: u32 = 0;
    }
}
/// Selects a tap on CYCCNT. These are spaced at bits \[6\] and \[10\]. When the selected bit in CYCCNT changes from 0 to 1 or 1 to 0, it emits into the POSTCNT, post-scalar counter. That counter then counts down. On a bit change when post-scalar is 0, it triggers an event for PC sampling or cycle count event (see details in CYCEVTENA).
pub mod CYCTAP {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 9..=9;
    pub const BIT_MASK: u32 = 0x00000200;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// Selects bit \[10\] to tap
        pub const BIT10: u32 = 1;
        /// Selects bit \[6\] to tap
        pub const BIT6: u32 = 0;
    }
}
/// Post-scalar counter for CYCTAP. When the selected tapped bit changes from 0 to 1 or 1 to 0, the post scalar counter is down-counted when not 0. If 0, it triggers an event for PCSAMPLEENA or CYCEVTENA use. It also reloads with the value from POSTPRESET.
pub mod POSTCNT {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 5..=8;
    pub const BIT_MASK: u32 = 0x000001e0;
    pub const BIT_WIDTH: u8 = 4;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Reload value for post-scalar counter POSTCNT. When 0, events are triggered on each tap change (a power of 2). If this field has a non-0 value, it forms a count-down value, to be reloaded into POSTCNT each time it reaches 0. For example, a value 1 in this register means an event is formed every other tap change.
pub mod POSTPRESET {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 1..=4;
    pub const BIT_MASK: u32 = 0x0000001e;
    pub const BIT_WIDTH: u8 = 4;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Enable CYCCNT, allowing it to increment and generate synchronization and count events. If NOCYCCNT = 1, this bit reads zero and ignore writes.
pub mod CYCCNTENA {
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
        pub CYCCNTENA: B1,
        pub POSTPRESET: B4,
        pub POSTCNT: B4,
        pub CYCTAP: B1,
        pub SYNCTAP: B2,
        pub PCSAMPLEENA: B1,
        pub reserved_13_16: B3,
        pub EXCTRCENA: B1,
        pub CPIEVTENA: B1,
        pub EXCEVTENA: B1,
        pub SLEEPEVTENA: B1,
        pub LSUEVTENA: B1,
        pub FOLDEVTENA: B1,
        pub CYCEVTENA: B1,
        pub reserved_23_24: B1,
        pub NOPRFCNT: B1,
        pub NOCYCCNT: B1,
        pub reserved_26_32: B6,
    }

    impl HwRegister for Register {
        const RESERVED_BITS_MASK: u32 = 0xfc80e000;
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
                warn!(target: "cc2650_constants::CPU_DWT::CTRL", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::CPU_DWT::CTRL",
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
