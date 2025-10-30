use cmemu_common::Address;

pub const DISPLAY: &str = "GPT3BCAPTSEL";
pub const OFFSET: u32 = 0x604;
/// 0x40083604
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
pub const RESET_VALUE: u32 = 0x0000005c;
pub const RESET_MASK: u32 = 0xffffffff;
/// Read/write selection value
///
///
///
/// Writing any other value than values defined by a ENUM may result in undefined behavior.
pub mod EV {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 0..=6;
    pub const BIT_MASK: u32 = 0x0000007f;
    pub const BIT_WIDTH: u8 = 7;
    pub const RESET_VALUE: u32 = 0x5c;
    pub const WRITABLE: bool = true;
    pub const RESET_ENUM: self::Values = self::Values::PORT_EVENT7;
    pub use self::Values as V;
    use modular_bitfield::prelude::BitfieldSpecifier;
    use num_enum::IntoPrimitive;
    #[repr(u8)]
    #[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, BitfieldSpecifier)]
    #[bits = 7]
    #[allow(non_camel_case_types)]
    #[non_exhaustive]
    pub enum Values {
        /// Always asserted
        ALWAYS_ACTIVE = 121,
        /// RTC periodic event controlled by AON_RTC:CTL.RTC_UPD_EN
        AON_RTC_UPD = 119,
        /// AUX ADC interrupt event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.ADC_IRQ. Status flags are found here AUX_EVCTL:EVTOMCUFLAGS
        AUX_ADC_IRQ = 115,
        /// Loopback of OBSMUX0 through AUX, corresponds to AUX_EVCTL:EVTOMCUFLAGS.OBSMUX0
        AUX_OBSMUX0 = 114,
        /// AUX ADC FIFO watermark event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.ADC_FIFO_ALMOST_FULL
        AUX_ADC_FIFO_ALMOST_FULL = 113,
        /// AUX ADC done, corresponds to AUX_EVCTL:EVTOMCUFLAGS.ADC_DONE
        AUX_ADC_DONE = 112,
        /// Autotake event from AUX semaphore, configured by AUX_SMPH:AUTOTAKE
        AUX_SMPH_AUTOTAKE_DONE = 111,
        /// AUX timer 1 event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.TIMER1_EV
        AUX_TIMER1_EV = 110,
        /// AUX timer 0 event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.TIMER0_EV
        AUX_TIMER0_EV = 109,
        /// AUX TDC measurement done event, corresponds to the flag AUX_EVCTL:EVTOMCUFLAGS.TDC_DONE and the AUX_TDC status AUX_TDC:STAT.DONE
        AUX_TDC_DONE = 108,
        /// AUX Compare B event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_COMPB
        AUX_COMPB = 107,
        /// AUX Compare A event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_COMPA
        AUX_COMPA = 106,
        /// AON wakeup event, corresponds flags are here AUX_EVCTL:EVTOMCUFLAGS.AON_WU_EV
        AUX_AON_WU_EV = 105,
        /// Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT7 wil be routed here.
        PORT_EVENT7 = 92,
        /// Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT6 wil be routed here.
        PORT_EVENT6 = 91,
        /// GPT3B compare event. Configured by GPT3:TBMR.TCACT
        GPT3B_CMP = 68,
        /// GPT3A compare event. Configured by GPT3:TAMR.TCACT
        GPT3A_CMP = 67,
        /// GPT2B compare event. Configured by GPT2:TBMR.TCACT
        GPT2B_CMP = 66,
        /// GPT2A compare event. Configured by GPT2:TAMR.TCACT
        GPT2A_CMP = 65,
        /// GPT1B compare event. Configured by GPT1:TBMR.TCACT
        GPT1B_CMP = 64,
        /// GPT1A compare event. Configured by GPT1:TAMR.TCACT
        GPT1A_CMP = 63,
        /// GPT0B compare event. Configured by GPT0:TBMR.TCACT
        GPT0B_CMP = 62,
        /// GPT0A compare event. Configured by GPT0:TAMR.TCACT
        GPT0A_CMP = 61,
        /// UART0 combined interrupt, interrupt flags are found here UART0:MIS
        UART0_COMB = 36,
        /// SSI1 combined interrupt, interrupt flags are found here SSI1:MIS
        SSI1_COMB = 35,
        /// SSI0 combined interrupt, interrupt flags are found here SSI0:MIS
        SSI0_COMB = 34,
        /// Combined Interrupt for CPE Generated events. Corresponding flags are here RFC_DBELL:RFCPEIFG. Only interrupts selected with CPE1 in RFC_DBELL:RFCPEIFG can trigger a RFC_CPE_1 event
        RFC_CPE_1 = 30,
        /// Combined Interrupt for CPE Generated events. Corresponding flags are here RFC_DBELL:RFCPEIFG. Only interrupts selected with CPE0 in RFC_DBELL:RFCPEIFG can trigger a RFC_CPE_0 event
        RFC_CPE_0 = 27,
        /// Combined RFC hardware interrupt, corresponding flag is here RFC_DBELL:RFHWIFG
        RFC_HW_COMB = 26,
        /// RFC Doorbell Command Acknowledgement Interrupt, equvialent to RFC_DBELL:RFACKIFG.ACKFLAG
        RFC_CMD_ACK = 25,
        /// FLASH controller error event,  the status flags are FLASH:FEDACSTAT.FSM_DONE and FLASH:FEDACSTAT.RVF_INT
        FLASH = 21,
        /// AUX combined event, the corresponding flag register is here AUX_EVCTL:EVTOMCUFLAGS
        AUX_COMB = 11,
        /// Event from AON_RTC, controlled by the AON_RTC:CTL.COMB_EV_MASK setting
        AON_RTC_COMB = 7,
        /// Edge detect event from IOC. Configureded by the IOC:IOCFGn.EDGE_IRQ_EN and  IOC:IOCFGn.EDGE_DET settings
        AON_GPIO_EDGE = 4,
        /// Always inactive
        NONE = 0,
    }
    pub use self::Named as E;
    pub mod Named {
        /// Always asserted
        pub const ALWAYS_ACTIVE: u32 = 121;
        /// RTC periodic event controlled by AON_RTC:CTL.RTC_UPD_EN
        pub const AON_RTC_UPD: u32 = 119;
        /// AUX ADC interrupt event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.ADC_IRQ. Status flags are found here AUX_EVCTL:EVTOMCUFLAGS
        pub const AUX_ADC_IRQ: u32 = 115;
        /// Loopback of OBSMUX0 through AUX, corresponds to AUX_EVCTL:EVTOMCUFLAGS.OBSMUX0
        pub const AUX_OBSMUX0: u32 = 114;
        /// AUX ADC FIFO watermark event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.ADC_FIFO_ALMOST_FULL
        pub const AUX_ADC_FIFO_ALMOST_FULL: u32 = 113;
        /// AUX ADC done, corresponds to AUX_EVCTL:EVTOMCUFLAGS.ADC_DONE
        pub const AUX_ADC_DONE: u32 = 112;
        /// Autotake event from AUX semaphore, configured by AUX_SMPH:AUTOTAKE
        pub const AUX_SMPH_AUTOTAKE_DONE: u32 = 111;
        /// AUX timer 1 event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.TIMER1_EV
        pub const AUX_TIMER1_EV: u32 = 110;
        /// AUX timer 0 event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.TIMER0_EV
        pub const AUX_TIMER0_EV: u32 = 109;
        /// AUX TDC measurement done event, corresponds to the flag AUX_EVCTL:EVTOMCUFLAGS.TDC_DONE and the AUX_TDC status AUX_TDC:STAT.DONE
        pub const AUX_TDC_DONE: u32 = 108;
        /// AUX Compare B event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_COMPB
        pub const AUX_COMPB: u32 = 107;
        /// AUX Compare A event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_COMPA
        pub const AUX_COMPA: u32 = 106;
        /// AON wakeup event, corresponds flags are here AUX_EVCTL:EVTOMCUFLAGS.AON_WU_EV
        pub const AUX_AON_WU_EV: u32 = 105;
        /// Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT7 wil be routed here.
        pub const PORT_EVENT7: u32 = 92;
        /// Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT6 wil be routed here.
        pub const PORT_EVENT6: u32 = 91;
        /// GPT3B compare event. Configured by GPT3:TBMR.TCACT
        pub const GPT3B_CMP: u32 = 68;
        /// GPT3A compare event. Configured by GPT3:TAMR.TCACT
        pub const GPT3A_CMP: u32 = 67;
        /// GPT2B compare event. Configured by GPT2:TBMR.TCACT
        pub const GPT2B_CMP: u32 = 66;
        /// GPT2A compare event. Configured by GPT2:TAMR.TCACT
        pub const GPT2A_CMP: u32 = 65;
        /// GPT1B compare event. Configured by GPT1:TBMR.TCACT
        pub const GPT1B_CMP: u32 = 64;
        /// GPT1A compare event. Configured by GPT1:TAMR.TCACT
        pub const GPT1A_CMP: u32 = 63;
        /// GPT0B compare event. Configured by GPT0:TBMR.TCACT
        pub const GPT0B_CMP: u32 = 62;
        /// GPT0A compare event. Configured by GPT0:TAMR.TCACT
        pub const GPT0A_CMP: u32 = 61;
        /// UART0 combined interrupt, interrupt flags are found here UART0:MIS
        pub const UART0_COMB: u32 = 36;
        /// SSI1 combined interrupt, interrupt flags are found here SSI1:MIS
        pub const SSI1_COMB: u32 = 35;
        /// SSI0 combined interrupt, interrupt flags are found here SSI0:MIS
        pub const SSI0_COMB: u32 = 34;
        /// Combined Interrupt for CPE Generated events. Corresponding flags are here RFC_DBELL:RFCPEIFG. Only interrupts selected with CPE1 in RFC_DBELL:RFCPEIFG can trigger a RFC_CPE_1 event
        pub const RFC_CPE_1: u32 = 30;
        /// Combined Interrupt for CPE Generated events. Corresponding flags are here RFC_DBELL:RFCPEIFG. Only interrupts selected with CPE0 in RFC_DBELL:RFCPEIFG can trigger a RFC_CPE_0 event
        pub const RFC_CPE_0: u32 = 27;
        /// Combined RFC hardware interrupt, corresponding flag is here RFC_DBELL:RFHWIFG
        pub const RFC_HW_COMB: u32 = 26;
        /// RFC Doorbell Command Acknowledgement Interrupt, equvialent to RFC_DBELL:RFACKIFG.ACKFLAG
        pub const RFC_CMD_ACK: u32 = 25;
        /// FLASH controller error event,  the status flags are FLASH:FEDACSTAT.FSM_DONE and FLASH:FEDACSTAT.RVF_INT
        pub const FLASH: u32 = 21;
        /// AUX combined event, the corresponding flag register is here AUX_EVCTL:EVTOMCUFLAGS
        pub const AUX_COMB: u32 = 11;
        /// Event from AON_RTC, controlled by the AON_RTC:CTL.COMB_EV_MASK setting
        pub const AON_RTC_COMB: u32 = 7;
        /// Edge detect event from IOC. Configureded by the IOC:IOCFGn.EDGE_IRQ_EN and  IOC:IOCFGn.EDGE_DET settings
        pub const AON_GPIO_EDGE: u32 = 4;
        /// Always inactive
        pub const NONE: u32 = 0;
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
        pub EV: super::EV::V,
        pub reserved_7_32: B25,
    }

    impl HwRegister for Register {
        const RESERVED_BITS_MASK: u32 = 0xffffff80;
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
                warn!(target: "cc2650_constants::EVENT::GPT3BCAPTSEL", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::EVENT::GPT3BCAPTSEL",
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
