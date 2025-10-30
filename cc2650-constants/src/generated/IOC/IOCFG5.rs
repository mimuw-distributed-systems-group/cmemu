use cmemu_common::Address;

pub const DISPLAY: &str = "IOCFG5";
pub const OFFSET: u32 = 0x14;
/// 0x40081014
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
pub const RESET_VALUE: u32 = 0x00006000;
pub const RESET_MASK: u32 = 0xffffffff;
/// 0: Input hysteresis disable
///
/// 1: Input hysteresis enable
pub mod HYST_EN {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 30..=30;
    pub const BIT_MASK: u32 = 0x40000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// 0: Input disabled
///
/// 1: Input enabled
///
///
///
/// Note: If IO is configured for AUX  ie. PORT_ID = 0x08, the enable will be ignored.
pub mod IE {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 29..=29;
    pub const BIT_MASK: u32 = 0x20000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08:
///
///
///
/// 00: No wake-up
///
/// 01: No wake-up
///
/// 10: Wakes up from shutdown if this pad is going low.
///
/// 11: Wakes up from shutdown if this pad is going high.
///
///
///
/// If IO is configured for AON peripheral signals or AUX  ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not.
///
///
///
/// 00, 01: Wakeup disabled
///
/// 10, 11: Wakeup enabled
///
///
///
/// Polarity is controlled from AON registers.
///
///
///
/// Note:When the MSB is set, the IOC will deactivate the output enable for the DIO.
pub mod WU_CFG {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 27..=28;
    pub const BIT_MASK: u32 = 0x18000000;
    pub const BIT_WIDTH: u8 = 2;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// IO Mode
///
/// N/A for IO configured for AON periph. signals and AUX  ie. PORT_ID 0x01-0x08
///
/// AUX has its own open_source/drain configuration.
///
///
///
/// 0x2: Reserved. Undefined behavior.
///
/// 0x3: Reserved. Undefined behavior.
pub mod IOMODE {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 24..=26;
    pub const BIT_MASK: u32 = 0x07000000;
    pub const BIT_WIDTH: u8 = 3;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// Open Source
        ///
        /// Inverted input / output
        pub const OPENSRC_INV: u32 = 7;
        /// Open Source
        ///
        /// Normal input / output
        pub const OPENSRC: u32 = 6;
        /// Open Drain
        ///
        /// Inverted input / output
        pub const OPENDR_INV: u32 = 5;
        /// Open Drain,
        ///
        /// Normal input / output
        pub const OPENDR: u32 = 4;
        /// Inverted input / ouput
        pub const INV: u32 = 1;
        /// Normal input / output
        pub const NORMAL: u32 = 0;
    }
}
/// 0: No interrupt generation
///
/// 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)
pub mod EDGE_IRQ_EN {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 18..=18;
    pub const BIT_MASK: u32 = 0x00040000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Enable generation of edge detection events on this IO
pub mod EDGE_DET {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 16..=17;
    pub const BIT_MASK: u32 = 0x00030000;
    pub const BIT_WIDTH: u8 = 2;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// Positive and negative edge detection
        pub const BOTH: u32 = 3;
        /// Positive edge detection
        pub const POS: u32 = 2;
        /// Negative edge detection
        pub const NEG: u32 = 1;
        /// No edge detection
        pub const NONE: u32 = 0;
    }
}
/// Pull control
pub mod PULL_CTL {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 13..=14;
    pub const BIT_MASK: u32 = 0x00006000;
    pub const BIT_WIDTH: u8 = 2;
    pub const RESET_VALUE: u32 = 0x3;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// No pull
        pub const DIS: u32 = 3;
        /// Pull up
        pub const UP: u32 = 2;
        /// Pull down
        pub const DWN: u32 = 1;
    }
}
/// 0: Normal slew rate
///
/// 1: Enables reduced slew rate in output driver.
pub mod SLEW_RED {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 12..=12;
    pub const BIT_MASK: u32 = 0x00001000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Selects IO current mode of this IO.
pub mod IOCURR {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 10..=11;
    pub const BIT_MASK: u32 = 0x00000c00;
    pub const BIT_WIDTH: u8 = 2;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// Extended-Current (EC) mode: Min 8 mA for double drive strength IOs (min 4 mA for normal IOs) when IOSTR is set to AUTO
        pub const _4_8MA: u32 = 2;
        /// High-Current (HC) mode: Min 4 mA when IOSTR is set to AUTO
        pub const _4MA: u32 = 1;
        /// Low-Current (LC) mode: Min 2 mA when IOSTR is set to AUTO
        pub const _2MA: u32 = 0;
    }
}
/// Select source for drive strength control of this IO.
///
/// This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR
pub mod IOSTR {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 8..=9;
    pub const BIT_MASK: u32 = 0x00000300;
    pub const BIT_WIDTH: u8 = 2;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// Maximum drive strength, controlled by AON_IOC:IOSTRMAX (min 2 mA @1.8V with default values)
        pub const MAX: u32 = 3;
        /// Medium drive strength, controlled by AON_IOC:IOSTRMED (min 2 mA @2.5V with default values)
        pub const MED: u32 = 2;
        /// Minimum drive strength, controlled by AON_IOC:IOSTRMIN (min 2 mA @3.3V with default values)
        pub const MIN: u32 = 1;
        /// Automatic drive strength, controlled by AON BATMON based on battery voltage. (min 2 mA @VDDS)
        pub const AUTO: u32 = 0;
    }
}
/// Selects usage for DIO5
pub mod PORT_ID {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 0..=5;
    pub const BIT_MASK: u32 = 0x0000003f;
    pub const BIT_WIDTH: u8 = 6;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// RF Core SMI Command Link In
        pub const RFC_SMI_CL_IN: u32 = 56;
        /// RF Core SMI Command Link Out
        pub const RFC_SMI_CL_OUT: u32 = 55;
        /// RF Core SMI Data Link In
        pub const RFC_SMI_DL_IN: u32 = 54;
        /// RF Core SMI Data Link Out
        pub const RFC_SMI_DL_OUT: u32 = 53;
        /// RF Core Data In 1
        pub const RFC_GPI1: u32 = 52;
        /// RF Core Data In 0
        pub const RFC_GPI0: u32 = 51;
        /// RF Core Data Out 3
        pub const RFC_GPO3: u32 = 50;
        /// RF Core Data Out 2
        pub const RFC_GPO2: u32 = 49;
        /// RF Core Data Out 1
        pub const RFC_GPO1: u32 = 48;
        /// RF Core Data Out 0
        pub const RFC_GPO0: u32 = 47;
        /// RF Core Trace
        pub const RFC_TRC: u32 = 46;
        /// I2S MCLK
        pub const I2S_MCLK: u32 = 41;
        /// I2S BCLK
        pub const I2S_BCLK: u32 = 40;
        /// I2S WCLK
        pub const I2S_WCLK: u32 = 39;
        /// I2S Data 1
        pub const I2S_AD1: u32 = 38;
        /// I2S Data 0
        pub const I2S_AD0: u32 = 37;
        /// SSI1 CLK
        pub const SSI1_CLK: u32 = 36;
        /// SSI1 FSS
        pub const SSI1_FSS: u32 = 35;
        /// SSI1 TX
        pub const SSI1_TX: u32 = 34;
        /// SSI1 RX
        pub const SSI1_RX: u32 = 33;
        /// CPU SWV
        pub const CPU_SWV: u32 = 32;
        /// PORT EVENT 7
        ///
        /// Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc
        pub const PORT_EVENT7: u32 = 30;
        /// PORT EVENT 6
        ///
        /// Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc
        pub const PORT_EVENT6: u32 = 29;
        /// PORT EVENT 5
        ///
        /// Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc
        pub const PORT_EVENT5: u32 = 28;
        /// PORT EVENT 4
        ///
        /// Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc
        pub const PORT_EVENT4: u32 = 27;
        /// PORT EVENT 3
        ///
        /// Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc
        pub const PORT_EVENT3: u32 = 26;
        /// PORT EVENT 2
        ///
        /// Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc
        pub const PORT_EVENT2: u32 = 25;
        /// PORT EVENT 1
        ///
        /// Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc
        pub const PORT_EVENT1: u32 = 24;
        /// PORT EVENT 0
        ///
        /// Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc
        pub const PORT_EVENT0: u32 = 23;
        /// UART0 RTS
        pub const UART0_RTS: u32 = 18;
        /// UART0 CTS
        pub const UART0_CTS: u32 = 17;
        /// UART0 TX
        pub const UART0_TX: u32 = 16;
        /// UART0 RX
        pub const UART0_RX: u32 = 15;
        /// I2C Clock
        pub const I2C_MSSCL: u32 = 14;
        /// I2C Data
        pub const I2C_MSSDA: u32 = 13;
        /// SSI0 CLK
        pub const SSI0_CLK: u32 = 12;
        /// SSI0 FSS
        pub const SSI0_FSS: u32 = 11;
        /// SSI0 TX
        pub const SSI0_TX: u32 = 10;
        /// SSI0 RX
        pub const SSI0_RX: u32 = 9;
        /// AUX IO
        pub const AUX_IO: u32 = 8;
        /// AON 32 KHz clock (SCLK_LF)
        pub const AON_CLK32K: u32 = 7;
        /// General Purpose IO
        pub const GPIO: u32 = 0;
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
        pub PORT_ID: B6,
        pub reserved_6_8: B2,
        pub IOSTR: B2,
        pub IOCURR: B2,
        pub SLEW_RED: B1,
        pub PULL_CTL: B2,
        pub reserved_15_16: B1,
        pub EDGE_DET: B2,
        pub EDGE_IRQ_EN: B1,
        pub reserved_19_24: B5,
        pub IOMODE: B3,
        pub WU_CFG: B2,
        pub IE: B1,
        pub HYST_EN: B1,
        pub reserved_31_32: B1,
    }

    impl HwRegister for Register {
        const RESERVED_BITS_MASK: u32 = 0x80f880c0;
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
                warn!(target: "cc2650_constants::IOC::IOCFG5", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::IOC::IOCFG5",
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
