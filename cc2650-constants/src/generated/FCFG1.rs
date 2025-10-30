use cmemu_common::Address;
use core::ops::Range;

pub const BASE_ADDR: Address = Address::from_const(0x50001000);
pub const ADDR: Address = BASE_ADDR.offset(0x0);
pub const SIZE: u32 = 0x400;
/// 0x50001000..0x50001400
pub const ADDR_SPACE: Range<Address> = ADDR..ADDR.offset(SIZE);

/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 9.2.2.1.65 AMPCOMP_CTRL1 Register (Offset = 378h) [reset = FF183F47h]
pub mod AMPCOMP_CTRL1;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 9.2.2.1.63 AMPCOMP_TH1 Register (Offset = 370h) [reset = FF7B828Eh]
pub mod AMPCOMP_TH1;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 9.2.2.1.64 AMPCOMP_TH2 Register (Offset = 374h) [reset = 6B8B0303h]
pub mod AMPCOMP_TH2;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 9.2.2.1.42 ANA2_TRIM Register (Offset = 2B4h) [reset = X]
pub mod ANA2_TRIM;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 9.2.2.1.66 ANABYPASS_VALUE2 Register (Offset = 37Ch) [reset = FFFFC3FFh]
pub mod ANABYPASS_VALUE2;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 9.2.1.1.44 BAT_RC_LDO_TRIM Register (Offset = 2BCh) [reset = X]
pub mod BAT_RC_LDO_TRIM;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 9.2.2.1.71 CAP_TRIM Register (Offset = 394h) [reset = FFFFFFFFh]
pub mod CAP_TRIM;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 9.2.2.1.55 CONFIG_IF_ADC Register (Offset = 34Ch) [reset = X]
pub mod CONFIG_IF_ADC;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 9.2.2.1.67 CONFIG_MISC_ADC Register (Offset = 380h) [reset = X]
pub mod CONFIG_MISC_ADC;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 9.2.2.1.17 CONFIG_MISC_ADC_DIV10 Register (Offset = FCh) [reset = FFFFFFFFh]
pub mod CONFIG_MISC_ADC_DIV10;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 9.2.2.1.18 CONFIG_MISC_ADC_DIV12 Register (Offset = 100h) [reset = FFFFFFFFh]
pub mod CONFIG_MISC_ADC_DIV12;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 9.2.2.1.19 CONFIG_MISC_ADC_DIV15 Register (Offset = 104h) [reset = FFFFFFFFh]
pub mod CONFIG_MISC_ADC_DIV15;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 9.2.2.1.20 CONFIG_MISC_ADC_DIV30 Register (Offset = 108h) [reset = FFFFFFFFh]
pub mod CONFIG_MISC_ADC_DIV30;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 9.2.2.1.15 CONFIG_MISC_ADC_DIV5 Register (Offset = F4h) [reset = FFFFFFFFh]
pub mod CONFIG_MISC_ADC_DIV5;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 9.2.2.1.16 CONFIG_MISC_ADC_DIV6 Register (Offset = F8h) [reset = FFFFFFFFh]
pub mod CONFIG_MISC_ADC_DIV6;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 9.2.2.1.56 CONFIG_OSC_TOP Register (Offset = 350h) [reset = X]
pub mod CONFIG_OSC_TOP;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 9.2.2.1.57 CONFIG_RF_FRONTEND Register (Offset = 354h) [reset = X]
pub mod CONFIG_RF_FRONTEND;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 9.2.2.1.5 CONFIG_RF_FRONTEND_DIV10 Register (Offset = CCh) [reset = FFFFFFFFh]
pub mod CONFIG_RF_FRONTEND_DIV10;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 9.2.2.1.6 CONFIG_RF_FRONTEND_DIV12 Register (Offset = D0h) [reset = FFFFFFFFh]
pub mod CONFIG_RF_FRONTEND_DIV12;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 9.2.2.1.7 CONFIG_RF_FRONTEND_DIV15 Register (Offset = D4h) [reset = FFFFFFFFh]
pub mod CONFIG_RF_FRONTEND_DIV15;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 9.2.2.1.8 CONFIG_RF_FRONTEND_DIV30 Register (Offset = D8h) [reset = FFFFFFFFh]
pub mod CONFIG_RF_FRONTEND_DIV30;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 9.2.2.1.3 CONFIG_RF_FRONTEND_DIV5 Register (Offset = C4h) [reset = FFFFFFFFh]
pub mod CONFIG_RF_FRONTEND_DIV5;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 9.2.2.1.4 CONFIG_RF_FRONTEND_DIV6 Register (Offset = C8h) [reset = FFFFFFFFh]
pub mod CONFIG_RF_FRONTEND_DIV6;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 9.2.2.1.58 CONFIG_SYNTH Register (Offset = 358h) [reset = X]
pub mod CONFIG_SYNTH;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 9.2.2.1.11 CONFIG_SYNTH_DIV10 Register (Offset = E4h) [reset = FFFFFFFFh]
pub mod CONFIG_SYNTH_DIV10;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 9.2.2.1.12 CONFIG_SYNTH_DIV12 Register (Offset = E8h) [reset = FFFFFFFFh]
pub mod CONFIG_SYNTH_DIV12;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 9.2.2.1.13 CONFIG_SYNTH_DIV15 Register (Offset = ECh) [reset = FFFFFFFFh]
pub mod CONFIG_SYNTH_DIV15;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 9.2.2.1.14 CONFIG_SYNTH_DIV30 Register (Offset = F0h) [reset = FFFFFFFFh]
pub mod CONFIG_SYNTH_DIV30;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 9.2.2.1.9 CONFIG_SYNTH_DIV5 Register (Offset = DCh) [reset = FFFFFFFFh]
pub mod CONFIG_SYNTH_DIV5;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 9.2.2.1.10 CONFIG_SYNTH_DIV6 Register (Offset = E0h) [reset = FFFFFFFFh]
pub mod CONFIG_SYNTH_DIV6;
/// Factory Configuration (FCFG1) Revision
///
/// [TI-TRM-I] 9.2.2.1.52 FCFG1_REVISION Register (Offset = 31Ch) [reset = 25h]
pub mod FCFG1_REVISION;
/// Flash coordinate
///
/// [TI-TRM-I] 9.2.2.1.28 FLASH_COORDINATE Register (Offset = 16Ch) [reset = X]
pub mod FLASH_COORDINATE;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 9.2.2.1.30 FLASH_C_E_P_R Register (Offset = 174h) [reset = 0A0A2000h]
pub mod FLASH_C_E_P_R;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 9.2.2.1.32 FLASH_EH_SEQ Register (Offset = 17Ch) [reset = 0200F000h]
pub mod FLASH_EH_SEQ;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 9.2.2.1.36 FLASH_ERA_PW Register (Offset = 18Ch) [reset = FA0h]
pub mod FLASH_ERA_PW;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 9.2.2.1.29 FLASH_E_P Register (Offset = 170h) [reset = 17331A33h]
pub mod FLASH_E_P;
/// Flash number
///
/// [TI-TRM-I] 9.2.2.1.27 FLASH_NUMBER Register (Offset = 164h) [reset = X]
pub mod FLASH_NUMBER;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 9.2.2.1.41 FLASH_OTP_DATA3 Register (Offset = 2B0h) [reset = X]
pub mod FLASH_OTP_DATA3;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 9.2.2.1.48 FLASH_OTP_DATA4 Register (Offset = 308h) [reset = 98989F9Fh]
pub mod FLASH_OTP_DATA4;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 9.2.2.1.34 FLASH_PP Register (Offset = 184h) [reset = X]
pub mod FLASH_PP;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 9.2.2.1.35 FLASH_PROG_EP Register (Offset = 188h) [reset = 0FA00010h]
pub mod FLASH_PROG_EP;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 9.2.2.1.31 FLASH_P_R_PV Register (Offset = 178h) [reset = 026E0200h]
pub mod FLASH_P_R_PV;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 9.2.2.1.39 FLASH_V Register (Offset = 198h) [reset = X]
pub mod FLASH_V;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 9.2.2.1.37 FLASH_VHV Register (Offset = 190h) [reset = X]
pub mod FLASH_VHV;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 9.2.2.1.33 FLASH_VHV_E Register (Offset = 180h) [reset = 1h]
pub mod FLASH_VHV_E;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 9.2.2.1.38 FLASH_VHV_PV Register (Offset = 194h) [reset = X]
pub mod FLASH_VHV_PV;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 9.2.2.1.70 FREQ_OFFSET Register (Offset = 390h) [reset = X]
pub mod FREQ_OFFSET;
/// IcePick Device Identification
///
/// Reading this register and the USER_ID register is the only support way of identifying a device.
///
/// [TI-TRM-I] 9.2.2.1.51 ICEPICK_DEVICE_ID Register (Offset = 318h) [reset = BB99A02Fh]
pub mod ICEPICK_DEVICE_ID;
/// IO Configuration
///
/// [TI-TRM-I] 9.2.2.1.54 IOCONF Register (Offset = 344h) [reset = X]
pub mod IOCONF;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 9.2.2.1.43 LDO_TRIM Register (Offset = 2B8h) [reset = X]
pub mod LDO_TRIM;
/// MAC IEEE 802.15.4 Address 0
///
/// [TI-TRM-I] 9.2.2.1.46 MAC_15_4_0 Register (Offset = 2F0h) [reset = X]
pub mod MAC_15_4_0;
/// MAC IEEE 802.15.4 Address 1
///
/// [TI-TRM-I] 9.2.2.1.47 MAC_15_4_1 Register (Offset = 2F4h) [reset = X]
pub mod MAC_15_4_1;
/// MAC BLE Address 0
///
/// [TI-TRM-I] 9.2.2.1.44 MAC_BLE_0 Register (Offset = 2E8h) [reset = X]
pub mod MAC_BLE_0;
/// MAC BLE Address 1
///
/// [TI-TRM-I] 9.2.2.1.45 MAC_BLE_1 Register (Offset = 2ECh) [reset = X]
pub mod MAC_BLE_1;
/// Misc configurations
///
/// [TI-TRM-I] 9.2.2.1.1 MISC_CONF_1 Register (Offset = A0h) [reset = X]
pub mod MISC_CONF_1;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 9.2.2.1.2 MISC_CONF_2 Register (Offset = A4h) [reset = X]
pub mod MISC_CONF_2;
/// Misc OTP Data
///
/// [TI-TRM-I] 9.2.2.1.53 MISC_OTP_DATA Register (Offset = 320h) [reset = X]
pub mod MISC_OTP_DATA;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 9.2.2.1.72 MISC_OTP_DATA_1 Register (Offset = 398h) [reset = E00403F8h]
pub mod MISC_OTP_DATA_1;
/// Miscellaneous Trim  Parameters
///
/// [TI-TRM-I] 9.2.2.1.49 MISC_TRIM Register (Offset = 30Ch) [reset = FFFFFF33h]
pub mod MISC_TRIM;
/// OSC Configuration
///
/// [TI-TRM-I] 9.2.2.1.69 OSC_CONF Register (Offset = 38Ch) [reset = X]
pub mod OSC_CONF;
/// Power Down Current Control 110C
///
/// [TI-TRM-I] 9.2.2.1.79 PWD_CURR_110C Register (Offset = 3B4h) [reset = 789E706Bh]
pub mod PWD_CURR_110C;
/// Power Down Current Control 125C
///
/// [TI-TRM-I] 9.2.2.1.80 PWD_CURR_125C Register (Offset = 3B8h) [reset = ADE1809Ah]
pub mod PWD_CURR_125C;
/// Power Down Current Control 20C
///
/// [TI-TRM-I] 9.2.2.1.73 PWD_CURR_20C Register (Offset = 39Ch) [reset = 080BA608h]
pub mod PWD_CURR_20C;
/// Power Down Current Control 35C
///
/// [TI-TRM-I] 9.2.2.1.74 PWD_CURR_35C Register (Offset = 3A0h) [reset = 0C10A50Ah]
pub mod PWD_CURR_35C;
/// Power Down Current Control 50C
///
/// [TI-TRM-I] 9.2.2.1.75 PWD_CURR_50C Register (Offset = 3A4h) [reset = 1218A20Dh]
pub mod PWD_CURR_50C;
/// Power Down Current Control 65C
///
/// [TI-TRM-I] 9.2.2.1.76 PWD_CURR_65C Register (Offset = 3A8h) [reset = 1C259C14h]
pub mod PWD_CURR_65C;
/// Power Down Current Control 80C
///
/// [TI-TRM-I] 9.2.2.1.77 PWD_CURR_80C Register (Offset = 3ACh) [reset = 2E3B9021h]
pub mod PWD_CURR_80C;
/// Power Down Current Control 95C
///
/// [TI-TRM-I] 9.2.2.1.78 PWD_CURR_95C Register (Offset = 3B0h) [reset = 4C627A3Bh]
pub mod PWD_CURR_95C;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 9.2.2.1.50 RCOSC_HF_TEMPCOMP Register (Offset = 310h) [reset = 3h]
pub mod RCOSC_HF_TEMPCOMP;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 9.2.2.1.26 SHDW_ANA_TRIM Register (Offset = 13Ch) [reset = X]
pub mod SHDW_ANA_TRIM;
/// Shadow of DIE_ID_0 register in eFuse
///
/// [TI-TRM-I] 9.2.2.1.21 SHDW_DIE_ID_0 Register (Offset = 118h) [reset = X]
pub mod SHDW_DIE_ID_0;
/// Shadow of DIE_ID_1 register in eFuse
///
/// [TI-TRM-I] 9.2.2.1.22 SHDW_DIE_ID_1 Register (Offset = 11Ch) [reset = X]
pub mod SHDW_DIE_ID_1;
/// Shadow of DIE_ID_2 register in eFuse
///
/// [TI-TRM-I] 9.2.2.1.23 SHDW_DIE_ID_2 Register (Offset = 120h) [reset = X]
pub mod SHDW_DIE_ID_2;
/// Shadow of DIE_ID_3 register in eFuse
///
/// [TI-TRM-I] 9.2.2.1.24 SHDW_DIE_ID_3 Register (Offset = 124h) [reset = X]
pub mod SHDW_DIE_ID_3;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 9.2.2.1.25 SHDW_OSC_BIAS_LDO_TRIM Register (Offset = 138h) [reset = X]
pub mod SHDW_OSC_BIAS_LDO_TRIM;
/// AUX_ADC Gain in Absolute Reference Mode
///
/// [TI-TRM-I] 9.2.2.1.59 SOC_ADC_ABS_GAIN Register (Offset = 35Ch) [reset = X]
pub mod SOC_ADC_ABS_GAIN;
/// AUX_ADC Temperature Offsets in Absolute Reference Mode
///
/// [TI-TRM-I] 9.2.2.1.61 SOC_ADC_OFFSET_INT Register (Offset = 368h) [reset = X]
pub mod SOC_ADC_OFFSET_INT;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 9.2.2.1.62 SOC_ADC_REF_TRIM_AND_OFFSET_EXT Register (Offset = 36Ch) [reset = X]
pub mod SOC_ADC_REF_TRIM_AND_OFFSET_EXT;
/// AUX_ADC Gain in Relative Reference Mode
///
/// [TI-TRM-I] 9.2.2.1.60 SOC_ADC_REL_GAIN Register (Offset = 360h) [reset = X]
pub mod SOC_ADC_REL_GAIN;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 9.2.1.1.52 TRIM_CAL_REVISION Register (Offset = 314h) [reset = X]
pub mod TRIM_CAL_REVISION;
/// User Identification.
///
/// Reading this register and the ICEPICK_DEVICE_ID register is the only support way of identifying a device.
///
/// The value of this register will be written to AON_WUC:JTAGUSERCODE by boot FW while in safezone.
///
/// [TI-TRM-I] 9.2.2.1.40 USER_ID Register (Offset = 294h) [reset = X]
pub mod USER_ID;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 9.2.2.1.68 VOLT_TRIM Register (Offset = 388h) [reset = X]
pub mod VOLT_TRIM;
