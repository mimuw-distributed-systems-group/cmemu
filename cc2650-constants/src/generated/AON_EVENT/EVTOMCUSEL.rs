use cmemu_common::Address;

pub const DISPLAY: &str = "EVTOMCUSEL";
pub const OFFSET: u32 = 0x8;
/// 0x40093008
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
pub const RESET_VALUE: u32 = 0x002b2b2b;
pub const RESET_MASK: u32 = 0xffffffff;
/// Event selector for AON_PROG2 event.
///
///
///
/// AON Event Source id# selecting event routed to EVENT as AON_PROG2 event.
pub mod AON_PROG2_EV {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 16..=21;
    pub const BIT_MASK: u32 = 0x003f0000;
    pub const BIT_WIDTH: u8 = 6;
    pub const RESET_VALUE: u32 = 0x2b;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// No event, always low
        pub const NONE: u32 = 63;
        /// Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX
        pub const AUX_COMPB_ASYNC_N: u32 = 56;
        /// Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX
        pub const AUX_COMPB_ASYNC: u32 = 55;
        /// BATMON voltage update event
        pub const BATMON_VOLT: u32 = 54;
        /// BATMON temperature update event
        pub const BATMON_TEMP: u32 = 53;
        /// AUX Timer 1 Event
        pub const AUX_TIMER1_EV: u32 = 52;
        /// AUX Timer 0 Event
        pub const AUX_TIMER0_EV: u32 = 51;
        /// TDC completed or timed out
        pub const AUX_TDC_DONE: u32 = 50;
        /// ADC conversion completed
        pub const AUX_ADC_DONE: u32 = 49;
        /// Comparator B triggered
        pub const AUX_COMPB: u32 = 48;
        /// Comparator A triggered
        pub const AUX_COMPA: u32 = 47;
        /// AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2
        pub const AUX_SWEV2: u32 = 46;
        /// AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1
        pub const AUX_SWEV1: u32 = 45;
        /// AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0
        pub const AUX_SWEV0: u32 = 44;
        /// JTAG generated event
        pub const JTAG: u32 = 43;
        /// RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)
        pub const RTC_UPD: u32 = 42;
        /// RTC combined delayed event
        pub const RTC_COMB_DLY: u32 = 41;
        /// RTC channel 2 - delayed event
        pub const RTC_CH2_DLY: u32 = 40;
        /// RTC channel 1 - delayed event
        pub const RTC_CH1_DLY: u32 = 39;
        /// RTC channel 0 - delayed event
        pub const RTC_CH0_DLY: u32 = 38;
        /// RTC channel 2 event
        pub const RTC_CH2: u32 = 37;
        /// RTC channel 1 event
        pub const RTC_CH1: u32 = 36;
        /// RTC channel 0 event
        pub const RTC_CH0: u32 = 35;
        /// Edge detect on any PAD
        pub const PAD: u32 = 32;
        /// Edge detect on PAD31
        pub const PAD31: u32 = 31;
        /// Edge detect on PAD30
        pub const PAD30: u32 = 30;
        /// Edge detect on PAD29
        pub const PAD29: u32 = 29;
        /// Edge detect on PAD28
        pub const PAD28: u32 = 28;
        /// Edge detect on PAD27
        pub const PAD27: u32 = 27;
        /// Edge detect on PAD26
        pub const PAD26: u32 = 26;
        /// Edge detect on PAD25
        pub const PAD25: u32 = 25;
        /// Edge detect on PAD24
        pub const PAD24: u32 = 24;
        /// Edge detect on PAD23
        pub const PAD23: u32 = 23;
        /// Edge detect on PAD22
        pub const PAD22: u32 = 22;
        /// Edge detect on PAD21
        pub const PAD21: u32 = 21;
        /// Edge detect on PAD20
        pub const PAD20: u32 = 20;
        /// Edge detect on PAD19
        pub const PAD19: u32 = 19;
        /// Edge detect on PAD18
        pub const PAD18: u32 = 18;
        /// Edge detect on PAD17
        pub const PAD17: u32 = 17;
        /// Edge detect on PAD16
        pub const PAD16: u32 = 16;
        /// Edge detect on PAD15
        pub const PAD15: u32 = 15;
        /// Edge detect on PAD14
        pub const PAD14: u32 = 14;
        /// Edge detect on PAD13
        pub const PAD13: u32 = 13;
        /// Edge detect on PAD12
        pub const PAD12: u32 = 12;
        /// Edge detect on PAD11
        pub const PAD11: u32 = 11;
        /// Edge detect on PAD10
        pub const PAD10: u32 = 10;
        /// Edge detect on PAD9
        pub const PAD9: u32 = 9;
        /// Edge detect on PAD8
        pub const PAD8: u32 = 8;
        /// Edge detect on PAD7
        pub const PAD7: u32 = 7;
        /// Edge detect on PAD6
        pub const PAD6: u32 = 6;
        /// Edge detect on PAD5
        pub const PAD5: u32 = 5;
        /// Edge detect on PAD4
        pub const PAD4: u32 = 4;
        /// Edge detect on PAD3
        pub const PAD3: u32 = 3;
        /// Edge detect on PAD2
        pub const PAD2: u32 = 2;
        /// Edge detect on PAD1
        pub const PAD1: u32 = 1;
        /// Edge detect on PAD0
        pub const PAD0: u32 = 0;
    }
}
/// Event selector for AON_PROG1 event.
///
///
///
/// AON Event Source id# selecting event routed to EVENT as AON_PROG1 event.
pub mod AON_PROG1_EV {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 8..=13;
    pub const BIT_MASK: u32 = 0x00003f00;
    pub const BIT_WIDTH: u8 = 6;
    pub const RESET_VALUE: u32 = 0x2b;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// No event, always low
        pub const NONE: u32 = 63;
        /// Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX
        pub const AUX_COMPB_ASYNC_N: u32 = 56;
        /// Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX
        pub const AUX_COMPB_ASYNC: u32 = 55;
        /// BATMON voltage update event
        pub const BATMON_VOLT: u32 = 54;
        /// BATMON temperature update event
        pub const BATMON_TEMP: u32 = 53;
        /// AUX Timer 1 Event
        pub const AUX_TIMER1_EV: u32 = 52;
        /// AUX Timer 0 Event
        pub const AUX_TIMER0_EV: u32 = 51;
        /// TDC completed or timed out
        pub const AUX_TDC_DONE: u32 = 50;
        /// ADC conversion completed
        pub const AUX_ADC_DONE: u32 = 49;
        /// Comparator B triggered
        pub const AUX_COMPB: u32 = 48;
        /// Comparator A triggered
        pub const AUX_COMPA: u32 = 47;
        /// AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2
        pub const AUX_SWEV2: u32 = 46;
        /// AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1
        pub const AUX_SWEV1: u32 = 45;
        /// AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0
        pub const AUX_SWEV0: u32 = 44;
        /// JTAG generated event
        pub const JTAG: u32 = 43;
        /// RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)
        pub const RTC_UPD: u32 = 42;
        /// RTC combined delayed event
        pub const RTC_COMB_DLY: u32 = 41;
        /// RTC channel 2 - delayed event
        pub const RTC_CH2_DLY: u32 = 40;
        /// RTC channel 1 - delayed event
        pub const RTC_CH1_DLY: u32 = 39;
        /// RTC channel 0 - delayed event
        pub const RTC_CH0_DLY: u32 = 38;
        /// RTC channel 2 event
        pub const RTC_CH2: u32 = 37;
        /// RTC channel 1 event
        pub const RTC_CH1: u32 = 36;
        /// RTC channel 0 event
        pub const RTC_CH0: u32 = 35;
        /// Edge detect on any PAD
        pub const PAD: u32 = 32;
        /// Edge detect on PAD31
        pub const PAD31: u32 = 31;
        /// Edge detect on PAD30
        pub const PAD30: u32 = 30;
        /// Edge detect on PAD29
        pub const PAD29: u32 = 29;
        /// Edge detect on PAD28
        pub const PAD28: u32 = 28;
        /// Edge detect on PAD27
        pub const PAD27: u32 = 27;
        /// Edge detect on PAD26
        pub const PAD26: u32 = 26;
        /// Edge detect on PAD25
        pub const PAD25: u32 = 25;
        /// Edge detect on PAD24
        pub const PAD24: u32 = 24;
        /// Edge detect on PAD23
        pub const PAD23: u32 = 23;
        /// Edge detect on PAD22
        pub const PAD22: u32 = 22;
        /// Edge detect on PAD21
        pub const PAD21: u32 = 21;
        /// Edge detect on PAD20
        pub const PAD20: u32 = 20;
        /// Edge detect on PAD19
        pub const PAD19: u32 = 19;
        /// Edge detect on PAD18
        pub const PAD18: u32 = 18;
        /// Edge detect on PAD17
        pub const PAD17: u32 = 17;
        /// Edge detect on PAD16
        pub const PAD16: u32 = 16;
        /// Edge detect on PAD15
        pub const PAD15: u32 = 15;
        /// Edge detect on PAD14
        pub const PAD14: u32 = 14;
        /// Edge detect on PAD13
        pub const PAD13: u32 = 13;
        /// Edge detect on PAD12
        pub const PAD12: u32 = 12;
        /// Edge detect on PAD11
        pub const PAD11: u32 = 11;
        /// Edge detect on PAD10
        pub const PAD10: u32 = 10;
        /// Edge detect on PAD9
        pub const PAD9: u32 = 9;
        /// Edge detect on PAD8
        pub const PAD8: u32 = 8;
        /// Edge detect on PAD7
        pub const PAD7: u32 = 7;
        /// Edge detect on PAD6
        pub const PAD6: u32 = 6;
        /// Edge detect on PAD5
        pub const PAD5: u32 = 5;
        /// Edge detect on PAD4
        pub const PAD4: u32 = 4;
        /// Edge detect on PAD3
        pub const PAD3: u32 = 3;
        /// Edge detect on PAD2
        pub const PAD2: u32 = 2;
        /// Edge detect on PAD1
        pub const PAD1: u32 = 1;
        /// Edge detect on PAD0
        pub const PAD0: u32 = 0;
    }
}
/// Event selector for AON_PROG0 event.
///
///
///
/// AON Event Source id# selecting event routed to EVENT as AON_PROG0 event.
pub mod AON_PROG0_EV {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 0..=5;
    pub const BIT_MASK: u32 = 0x0000003f;
    pub const BIT_WIDTH: u8 = 6;
    pub const RESET_VALUE: u32 = 0x2b;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// No event, always low
        pub const NONE: u32 = 63;
        /// Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX
        pub const AUX_COMPB_ASYNC_N: u32 = 56;
        /// Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX
        pub const AUX_COMPB_ASYNC: u32 = 55;
        /// BATMON voltage update event
        pub const BATMON_VOLT: u32 = 54;
        /// BATMON temperature update event
        pub const BATMON_TEMP: u32 = 53;
        /// AUX Timer 1 Event
        pub const AUX_TIMER1_EV: u32 = 52;
        /// AUX Timer 0 Event
        pub const AUX_TIMER0_EV: u32 = 51;
        /// TDC completed or timed out
        pub const AUX_TDC_DONE: u32 = 50;
        /// ADC conversion completed
        pub const AUX_ADC_DONE: u32 = 49;
        /// Comparator B triggered
        pub const AUX_COMPB: u32 = 48;
        /// Comparator A triggered
        pub const AUX_COMPA: u32 = 47;
        /// AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2
        pub const AUX_SWEV2: u32 = 46;
        /// AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1
        pub const AUX_SWEV1: u32 = 45;
        /// AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0
        pub const AUX_SWEV0: u32 = 44;
        /// JTAG generated event
        pub const JTAG: u32 = 43;
        /// RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)
        pub const RTC_UPD: u32 = 42;
        /// RTC combined delayed event
        pub const RTC_COMB_DLY: u32 = 41;
        /// RTC channel 2 - delayed event
        pub const RTC_CH2_DLY: u32 = 40;
        /// RTC channel 1 - delayed event
        pub const RTC_CH1_DLY: u32 = 39;
        /// RTC channel 0 - delayed event
        pub const RTC_CH0_DLY: u32 = 38;
        /// RTC channel 2 event
        pub const RTC_CH2: u32 = 37;
        /// RTC channel 1 event
        pub const RTC_CH1: u32 = 36;
        /// RTC channel 0 event
        pub const RTC_CH0: u32 = 35;
        /// Edge detect on any PAD
        pub const PAD: u32 = 32;
        /// Edge detect on PAD31
        pub const PAD31: u32 = 31;
        /// Edge detect on PAD30
        pub const PAD30: u32 = 30;
        /// Edge detect on PAD29
        pub const PAD29: u32 = 29;
        /// Edge detect on PAD28
        pub const PAD28: u32 = 28;
        /// Edge detect on PAD27
        pub const PAD27: u32 = 27;
        /// Edge detect on PAD26
        pub const PAD26: u32 = 26;
        /// Edge detect on PAD25
        pub const PAD25: u32 = 25;
        /// Edge detect on PAD24
        pub const PAD24: u32 = 24;
        /// Edge detect on PAD23
        pub const PAD23: u32 = 23;
        /// Edge detect on PAD22
        pub const PAD22: u32 = 22;
        /// Edge detect on PAD21
        pub const PAD21: u32 = 21;
        /// Edge detect on PAD20
        pub const PAD20: u32 = 20;
        /// Edge detect on PAD19
        pub const PAD19: u32 = 19;
        /// Edge detect on PAD18
        pub const PAD18: u32 = 18;
        /// Edge detect on PAD17
        pub const PAD17: u32 = 17;
        /// Edge detect on PAD16
        pub const PAD16: u32 = 16;
        /// Edge detect on PAD15
        pub const PAD15: u32 = 15;
        /// Edge detect on PAD14
        pub const PAD14: u32 = 14;
        /// Edge detect on PAD13
        pub const PAD13: u32 = 13;
        /// Edge detect on PAD12
        pub const PAD12: u32 = 12;
        /// Edge detect on PAD11
        pub const PAD11: u32 = 11;
        /// Edge detect on PAD10
        pub const PAD10: u32 = 10;
        /// Edge detect on PAD9
        pub const PAD9: u32 = 9;
        /// Edge detect on PAD8
        pub const PAD8: u32 = 8;
        /// Edge detect on PAD7
        pub const PAD7: u32 = 7;
        /// Edge detect on PAD6
        pub const PAD6: u32 = 6;
        /// Edge detect on PAD5
        pub const PAD5: u32 = 5;
        /// Edge detect on PAD4
        pub const PAD4: u32 = 4;
        /// Edge detect on PAD3
        pub const PAD3: u32 = 3;
        /// Edge detect on PAD2
        pub const PAD2: u32 = 2;
        /// Edge detect on PAD1
        pub const PAD1: u32 = 1;
        /// Edge detect on PAD0
        pub const PAD0: u32 = 0;
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
        pub AON_PROG0_EV: B6,
        pub reserved_6_8: B2,
        pub AON_PROG1_EV: B6,
        pub reserved_14_16: B2,
        pub AON_PROG2_EV: B6,
        pub reserved_22_32: B10,
    }

    impl HwRegister for Register {
        const RESERVED_BITS_MASK: u32 = 0xffc0c0c0;
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
                warn!(target: "cc2650_constants::AON_EVENT::EVTOMCUSEL", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::AON_EVENT::EVTOMCUSEL",
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
