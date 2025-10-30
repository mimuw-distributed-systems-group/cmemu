use cmemu_common::Address;
use core::ops::Range;

pub const BASE_ADDR: Address = Address::from_const(0x40082000);
pub const ADDR: Address = BASE_ADDR.offset(0x0);
pub const SIZE: u32 = 0x1000;
/// 0x40082000..0x40083000
pub const ADDR_SPACE: Range<Address> = ADDR..ADDR.offset(SIZE);

/// Load PRCM Settings To CLKCTRL Power Domain
///
/// [TI-TRM-I] 6.8.2.4.5 CLKLOADCTL Register (Offset = 28h) [reset = 2h]
pub mod CLKLOADCTL;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 6.8.2.4.29 CPUCLKDIV Register (Offset = B8h) [reset = 0h]
pub mod CPUCLKDIV;
/// GPIO Clock Gate For Deep Sleep Mode
///
/// [TI-TRM-I] 6.8.2.4.13 GPIOCLKGDS Register (Offset = 50h) [reset = 0h]
pub mod GPIOCLKGDS;
/// GPIO Clock Gate For Run Mode
///
/// [TI-TRM-I] 6.8.2.4.11 GPIOCLKGR Register (Offset = 48h) [reset = 0h]
pub mod GPIOCLKGR;
/// GPIO Clock Gate For Sleep Mode
///
/// [TI-TRM-I] 6.8.2.4.12 GPIOCLKGS Register (Offset = 4Ch) [reset = 0h]
pub mod GPIOCLKGS;
/// GPT Scalar
///
/// [TI-TRM-I] 6.8.2.4.31 GPTCLKDIV Register (Offset = CCh) [reset = 0h]
pub mod GPTCLKDIV;
/// GPT Clock Gate For Deep Sleep Mode
///
/// [TI-TRM-I] 6.8.2.4.16 GPTCLKGDS Register (Offset = 5Ch) [reset = 0h]
pub mod GPTCLKGDS;
/// GPT Clock Gate For Run Mode
///
/// [TI-TRM-I] 6.8.2.4.14 GPTCLKGR Register (Offset = 54h) [reset = 0h]
pub mod GPTCLKGR;
/// GPT Clock Gate For Sleep Mode
///
/// [TI-TRM-I] 6.8.2.4.15 GPTCLKGS Register (Offset = 58h) [reset = 0h]
pub mod GPTCLKGS;
/// I2C Clock Gate For Deep Sleep Mode
///
/// [TI-TRM-I] 6.8.2.4.19 I2CCLKGDS Register (Offset = 68h) [reset = 0h]
pub mod I2CCLKGDS;
/// I2C Clock Gate For Run Mode
///
/// [TI-TRM-I] 6.8.2.4.17 I2CCLKGR Register (Offset = 60h) [reset = 0h]
pub mod I2CCLKGR;
/// I2C Clock Gate For Sleep Mode
///
/// [TI-TRM-I] 6.8.2.4.18 I2CCLKGS Register (Offset = 64h) [reset = 0h]
pub mod I2CCLKGS;
/// BCLK Division Ratio
///
/// [TI-TRM-I] 6.8.2.4.34 I2SBCLKDIV Register (Offset = D8h) [reset = 0h]
pub mod I2SBCLKDIV;
/// I2S Clock Control
///
/// [TI-TRM-I] 6.8.2.4.30 I2SBCLKSEL Register (Offset = C8h) [reset = 0h]
pub mod I2SBCLKSEL;
/// I2S Clock Control
///
/// [TI-TRM-I] 6.8.2.4.32 I2SCLKCTL Register (Offset = D0h) [reset = 0h]
pub mod I2SCLKCTL;
/// I2S Clock Gate For Deep Sleep Mode
///
/// [TI-TRM-I] 6.8.2.4.28 I2SCLKGDS Register (Offset = 8Ch) [reset = 0h]
pub mod I2SCLKGDS;
/// I2S Clock Gate For Run Mode
///
/// [TI-TRM-I] 6.8.2.4.26 I2SCLKGR Register (Offset = 84h) [reset = 0h]
pub mod I2SCLKGR;
/// I2S Clock Gate For Sleep Mode
///
/// [TI-TRM-I] 6.8.2.4.27 I2SCLKGS Register (Offset = 88h) [reset = 0h]
pub mod I2SCLKGS;
/// MCLK Division Ratio
///
/// [TI-TRM-I] 6.8.2.4.33 I2SMCLKDIV Register (Offset = D4h) [reset = 0h]
pub mod I2SMCLKDIV;
/// WCLK Division Ratio
///
/// [TI-TRM-I] 6.8.2.4.35 I2SWCLKDIV Register (Offset = DCh) [reset = 0h]
pub mod I2SWCLKDIV;
/// Infrastructure Clock Division Factor For DeepSleep Mode
///
/// [TI-TRM-I] 6.8.2.4.3 INFRCLKDIVDS Register (Offset = 8h) [reset = 0h]
pub mod INFRCLKDIVDS;
/// Infrastructure Clock Division Factor For Run Mode
///
/// [TI-TRM-I] 6.8.2.4.1 INFRCLKDIVR Register (Offset = 0h) [reset = 0h]
pub mod INFRCLKDIVR;
/// Infrastructure Clock Division Factor For Sleep Mode
///
/// [TI-TRM-I] 6.8.2.4.2 INFRCLKDIVS Register (Offset = 4h) [reset = 0h]
pub mod INFRCLKDIVS;
/// Power Domain Control
///
/// [TI-TRM-I] 6.8.2.4.38 PDCTL0 Register (Offset = 12Ch) [reset = 0h]
pub mod PDCTL0;
/// PERIPH Power Domain Control
///
/// [TI-TRM-I] 6.8.2.4.41 PDCTL0PERIPH Register (Offset = 138h) [reset = 0h]
pub mod PDCTL0PERIPH;
/// RFC Power Domain Control
///
/// [TI-TRM-I] 6.8.2.4.39 PDCTL0RFC Register (Offset = 130h) [reset = 0h]
pub mod PDCTL0RFC;
/// SERIAL Power Domain Control
///
/// [TI-TRM-I] 6.8.2.4.40 PDCTL0SERIAL Register (Offset = 134h) [reset = 0h]
pub mod PDCTL0SERIAL;
/// Power Domain Control
///
/// [TI-TRM-I] 6.8.2.4.46 PDCTL1 Register (Offset = 17Ch) [reset = Ah]
pub mod PDCTL1;
/// CPU Power Domain Direct Control
///
/// [TI-TRM-I] 6.8.2.4.47 PDCTL1CPU Register (Offset = 184h) [reset = 1h]
pub mod PDCTL1CPU;
/// RFC Power Domain Direct Control
///
/// [TI-TRM-I] 6.8.2.4.48 PDCTL1RFC Register (Offset = 188h) [reset = 0h]
pub mod PDCTL1RFC;
/// VIMS Mode Direct Control
///
/// [TI-TRM-I] 6.8.2.4.49 PDCTL1VIMS Register (Offset = 18Ch) [reset = 1h]
pub mod PDCTL1VIMS;
/// Power Domain Status
///
/// [TI-TRM-I] 6.8.2.4.42 PDSTAT0 Register (Offset = 140h) [reset = 0h]
pub mod PDSTAT0;
/// PERIPH Power Domain Status
///
/// [TI-TRM-I] 6.8.2.4.45 PDSTAT0PERIPH Register (Offset = 14Ch) [reset = 0h]
pub mod PDSTAT0PERIPH;
/// RFC Power Domain Status
///
/// [TI-TRM-I] 6.8.2.4.43 PDSTAT0RFC Register (Offset = 144h) [reset = 0h]
pub mod PDSTAT0RFC;
/// SERIAL Power Domain Status
///
/// [TI-TRM-I] 6.8.2.4.44 PDSTAT0SERIAL Register (Offset = 148h) [reset = 0h]
pub mod PDSTAT0SERIAL;
/// Power Manager Status
///
/// [TI-TRM-I] 6.8.2.4.50 PDSTAT1 Register (Offset = 194h) [reset = 1Ah]
pub mod PDSTAT1;
/// BUS Power Domain Direct Read Status
///
/// [TI-TRM-I] 6.8.2.4.51 PDSTAT1BUS Register (Offset = 198h) [reset = 1h]
pub mod PDSTAT1BUS;
/// CPU Power Domain Direct Read Status
///
/// [TI-TRM-I] 6.8.2.4.53 PDSTAT1CPU Register (Offset = 1A0h) [reset = 1h]
pub mod PDSTAT1CPU;
/// RFC Power Domain Direct Read Status
///
/// [TI-TRM-I] 6.8.2.4.52 PDSTAT1RFC Register (Offset = 19Ch) [reset = 0h]
pub mod PDSTAT1RFC;
/// VIMS Mode Direct Read Status
///
/// [TI-TRM-I] 6.8.2.4.54 PDSTAT1VIMS Register (Offset = 1A4h) [reset = 1h]
pub mod PDSTAT1VIMS;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod PERBUSDMACLKDIV;
/// Power Profiler Register
///
/// [TI-TRM-I] 6.8.2.4.58 PWRPROFSTAT Register (Offset = 1E0h) [reset = 1h]
pub mod PWRPROFSTAT;
/// Memory Retention Control
///
/// [TI-TRM-I] 6.8.2.4.59 RAMRETEN Register (Offset = 224h) [reset = 3h]
pub mod RAMRETEN;
/// Control To RFC
///
/// [TI-TRM-I] 6.8.2.4.55 RFCBITS Register (Offset = 1CCh) [reset = 0h]
pub mod RFCBITS;
/// RFC Clock Gate
///
/// [TI-TRM-I] 6.8.2.4.6 RFCCLKG Register (Offset = 2Ch) [reset = 1h]
pub mod RFCCLKG;
/// Allowed RFC Modes
///
/// [TI-TRM-I] 6.8.2.4.57 RFCMODEHWOPT Register (Offset = 1D4h) [reset = 0h]
pub mod RFCMODEHWOPT;
/// Selected RFC Mode
///
/// [TI-TRM-I] 6.8.2.4.56 RFCMODESEL Register (Offset = 1D0h) [reset = 0h]
pub mod RFCMODESEL;
/// TRNG, CRYPTO And UDMA Clock Gate For Deep Sleep Mode
///
/// [TI-TRM-I] 6.8.2.4.10 SECDMACLKGDS Register (Offset = 44h) [reset = 0h]
pub mod SECDMACLKGDS;
/// TRNG, CRYPTO And UDMA Clock Gate For Run Mode
///
/// [TI-TRM-I] 6.8.2.4.8 SECDMACLKGR Register (Offset = 3Ch) [reset = 0h]
pub mod SECDMACLKGR;
/// TRNG, CRYPTO And UDMA Clock Gate For Sleep Mode
///
/// [TI-TRM-I] 6.8.2.4.9 SECDMACLKGS Register (Offset = 40h) [reset = 0h]
pub mod SECDMACLKGS;
/// SSI Clock Gate For Deep Sleep Mode
///
/// [TI-TRM-I] 6.8.2.4.25 SSICLKGDS Register (Offset = 80h) [reset = 0h]
pub mod SSICLKGDS;
/// SSI Clock Gate For Run Mode
///
/// [TI-TRM-I] 6.8.2.4.23 SSICLKGR Register (Offset = 78h) [reset = 0h]
pub mod SSICLKGR;
/// SSI Clock Gate For Sleep Mode
///
/// [TI-TRM-I] 6.8.2.4.24 SSICLKGS Register (Offset = 7Ch) [reset = 0h]
pub mod SSICLKGS;
/// SW Initiated Resets
///
/// [TI-TRM-I] 6.8.2.4.36 SWRESET Register (Offset = 10Ch) [reset = 0h]
pub mod SWRESET;
/// UART Clock Gate For Deep Sleep Mode
///
/// [TI-TRM-I] 6.8.2.4.22 UARTCLKGDS Register (Offset = 74h) [reset = 0h]
pub mod UARTCLKGDS;
/// UART Clock Gate For Run Mode
///
/// [TI-TRM-I] 6.8.2.4.20 UARTCLKGR Register (Offset = 6Ch) [reset = 0h]
pub mod UARTCLKGR;
/// UART Clock Gate For Sleep Mode
///
/// [TI-TRM-I] 6.8.2.4.21 UARTCLKGS Register (Offset = 70h) [reset = 0h]
pub mod UARTCLKGS;
/// MCU Voltage Domain Control
///
/// [TI-TRM-I] 6.8.2.4.4 VDCTL Register (Offset = Ch) [reset = 0h]
pub mod VDCTL;
/// VIMS Clock Gate
///
/// [TI-TRM-I] 6.8.2.4.7 VIMSCLKG Register (Offset = 30h) [reset = 3h]
pub mod VIMSCLKG;
/// WARM Reset Control And Status
///
/// [TI-TRM-I] 6.8.2.4.37 WARMRESET Register (Offset = 110h) [reset = 0h]
pub mod WARMRESET;
