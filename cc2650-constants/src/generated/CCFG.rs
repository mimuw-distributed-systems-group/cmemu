use cmemu_common::Address;
use core::ops::Range;

pub const BASE_ADDR: Address = Address::from_const(0x50003000);
pub const ADDR: Address = BASE_ADDR.offset(0x0);
pub const SIZE: u32 = 0x1000;
/// 0x50003000..0x50004000
pub const ADDR_SPACE: Range<Address> = ADDR..ADDR.offset(SIZE);

/// Bootloader Configuration
///
/// Configures the functionality of the ROM boot loader.
///
/// If both the boot loader is enabled by the BOOTLOADER_ENABLE field and the boot loader backdoor is enabled by the BL_ENABLE field it is possible to force entry of the ROM boot loader even if a valid image is present in flash.
///
/// [TI-TRM-I] 9.1.1.13 BL_CONFIG Register (Offset = FD8h) [reset = C5FFFFFFh]
pub mod BL_CONFIG;
/// Protect Sectors 96-127
///
/// Each bit write protects one flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect. Not in use by CC26x0 and CC13x0.
///
/// [TI-TRM-I] 9.1.1.22 CCFG_PROT_127_96 Register (Offset = FFCh) [reset = FFFFFFFFh]
pub mod CCFG_PROT_127_96;
/// Protect Sectors 0-31
///
/// Each bit write protects one 4KB flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect.
///
/// [TI-TRM-I] 9.1.1.19 CCFG_PROT_31_0 Register (Offset = FF0h) [reset = FFFFFFFFh]
pub mod CCFG_PROT_31_0;
/// Protect Sectors 32-63
///
/// Each bit write protects one 4KB flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect. Not in use by CC26x0 and CC13x0.
///
/// [TI-TRM-I] 9.1.1.20 CCFG_PROT_63_32 Register (Offset = FF4h) [reset = FFFFFFFFh]
pub mod CCFG_PROT_63_32;
/// Protect Sectors 64-95
///
/// Each bit write protects one flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect. Not in use by CC26x0 and CC13x0.
///
/// [TI-TRM-I] 9.1.1.21 CCFG_PROT_95_64 Register (Offset = FF8h) [reset = FFFFFFFFh]
pub mod CCFG_PROT_95_64;
/// Test Access Points Enable 0
///
/// [TI-TRM-I] 9.1.1.16 CCFG_TAP_DAP_0 Register (Offset = FE4h) [reset = FFC5C5C5h]
pub mod CCFG_TAP_DAP_0;
/// Test Access Points Enable 1
///
/// [TI-TRM-I] 9.1.1.17 CCFG_TAP_DAP_1 Register (Offset = FE8h) [reset = FFC5C5C5h]
pub mod CCFG_TAP_DAP_1;
/// TI Options
///
/// [TI-TRM-I] 9.1.1.15 CCFG_TI_OPTIONS Register (Offset = FE0h) [reset = FFFFFFC5h]
pub mod CCFG_TI_OPTIONS;
/// Erase Configuration
///
/// [TI-TRM-I] 9.1.1.14 ERASE_CONF Register (Offset = FDCh) [reset = FFFFFFFFh]
pub mod ERASE_CONF;
/// Extern LF clock configuration
///
/// [TI-TRM-I] 9.1.1.1 EXT_LF_CLK Register (Offset = FA8h) [reset = FFFFFFFFh]
pub mod EXT_LF_CLK;
/// Frequency Offset
///
/// [TI-TRM-I] 9.1.1.8 FREQ_OFFSET Register (Offset = FC4h) [reset = FFFFFFFFh]
pub mod FREQ_OFFSET;
/// IEEE BLE Address 0
///
/// [TI-TRM-I] 9.1.1.11 IEEE_BLE_0 Register (Offset = FD0h) [reset = FFFFFFFFh]
pub mod IEEE_BLE_0;
/// IEEE BLE Address 1
///
/// [TI-TRM-I] 9.1.1.12 IEEE_BLE_1 Register (Offset = FD4h) [reset = FFFFFFFFh]
pub mod IEEE_BLE_1;
/// IEEE MAC Address 0
///
/// [TI-TRM-I] 9.1.1.9 IEEE_MAC_0 Register (Offset = FC8h) [reset = FFFFFFFFh]
pub mod IEEE_MAC_0;
/// IEEE MAC Address 1
///
/// [TI-TRM-I] 9.1.1.10 IEEE_MAC_1 Register (Offset = FCCh) [reset = FFFFFFFFh]
pub mod IEEE_MAC_1;
/// Image Valid
///
/// [TI-TRM-I] 9.1.1.18 IMAGE_VALID_CONF Register (Offset = FECh) [reset = FFFFFFFFh]
pub mod IMAGE_VALID_CONF;
/// Mode Configuration 0
///
/// [TI-TRM-I] 9.1.1.4 MODE_CONF Register (Offset = FB4h) [reset = FFFFFFFFh]
pub mod MODE_CONF;
/// Mode Configuration 1
///
/// [TI-TRM-I] 9.1.1.2 MODE_CONF_1 Register (Offset = FACh) [reset = FFFBFFFFh]
pub mod MODE_CONF_1;
/// Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.
///
/// [TI-TRM-I] undocumented
pub mod RESERVED_0;
/// Real Time Clock Offset
///
/// Enabled by MODE_CONF.RTC_COMP.
///
/// [TI-TRM-I] 9.1.1.7 RTC_OFFSET Register (Offset = FC0h) [reset = FFFFFFFFh]
pub mod RTC_OFFSET;
/// CCFG Size and Disable Flags
///
/// [TI-TRM-I] 9.1.1.3 SIZE_AND_DIS_FLAGS Register (Offset = FB0h) [reset = FFFFFFFFh]
pub mod SIZE_AND_DIS_FLAGS;
/// Voltage Load 0
///
/// Enabled by MODE_CONF.VDDR_EXT_LOAD.
///
/// [TI-TRM-I] 9.1.1.5 VOLT_LOAD_0 Register (Offset = FB8h) [reset = FFFFFFFFh]
pub mod VOLT_LOAD_0;
/// Voltage Load 1
///
/// Enabled by MODE_CONF.VDDR_EXT_LOAD.
///
/// [TI-TRM-I] 9.1.1.6 VOLT_LOAD_1 Register (Offset = FBCh) [reset = FFFFFFFFh]
pub mod VOLT_LOAD_1;
