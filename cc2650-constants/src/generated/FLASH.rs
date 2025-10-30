use cmemu_common::Address;
use core::ops::Range;

pub const BASE_ADDR: Address = Address::from_const(0x40030000);
pub const ADDR: Address = BASE_ADDR.offset(0x0);
pub const SIZE: u32 = 0x4000;
/// 0x40030000..0x40034000
pub const ADDR_SPACE: Range<Address> = ADDR..ADDR.offset(SIZE);

/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.13 ACC Register (Offset = 1018h) [reset = 0h]
pub mod ACC;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.14 BOUNDARY Register (Offset = 101Ch) [reset = 0h]
pub mod BOUNDARY;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.2 CFG Register (Offset = 24h) [reset = 0h]
pub mod CFG;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.10 DATALOWER Register (Offset = 100Ch) [reset = 0h]
pub mod DATALOWER;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.9 DATAUPPER Register (Offset = 1008h) [reset = 0h]
pub mod DATAUPPER;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod EEPROM_CFG;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.7 EFUSE Register (Offset = 1000h) [reset = 0h]
pub mod EFUSE;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.8 EFUSEADDR Register (Offset = 1004h) [reset = 0h]
pub mod EFUSEADDR;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.11 EFUSECFG Register (Offset = 1010h) [reset = 1h]
pub mod EFUSECFG;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.19 EFUSECRA Register (Offset = 1030h) [reset = 0h]
pub mod EFUSECRA;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.22 EFUSEERROR Register (Offset = 103Ch) [reset = 0h]
pub mod EFUSEERROR;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.15 EFUSEFLAG Register (Offset = 1020h) [reset = 0h]
pub mod EFUSEFLAG;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.16 EFUSEKEY Register (Offset = 1024h) [reset = 0h]
pub mod EFUSEKEY;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.18 EFUSEPINS Register (Offset = 102Ch) [reset = X]
pub mod EFUSEPINS;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.21 EFUSEPROGRAM Register (Offset = 1038h) [reset = 0h]
pub mod EFUSEPROGRAM;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.20 EFUSEREAD Register (Offset = 1034h) [reset = 0h]
pub mod EFUSEREAD;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.17 EFUSERELEASE Register (Offset = 1028h) [reset = X]
pub mod EFUSERELEASE;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.12 EFUSESTAT Register (Offset = 1014h) [reset = 1h]
pub mod EFUSESTAT;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.57 FADDR Register (Offset = 2110h) [reset = 0h]
pub mod FADDR;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.34 FBAC Register (Offset = 203Ch) [reset = Fh]
pub mod FBAC;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.33 FBBUSY Register (Offset = 2038h) [reset = FEh]
pub mod FBBUSY;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.35 FBFALLBACK Register (Offset = 2040h) [reset = 0505FFFFh]
pub mod FBFALLBACK;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.55 FBMODE Register (Offset = 2108h) [reset = 0h]
pub mod FBMODE;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.36 FBPRDY Register (Offset = 2044h) [reset = 00FF00FEh]
pub mod FBPRDY;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.31 FBPROT Register (Offset = 2030h) [reset = 0h]
pub mod FBPROT;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.32 FBSE Register (Offset = 2034h) [reset = 0h]
pub mod FBSE;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.53 FBSTROBES Register (Offset = 2100h) [reset = 104h]
pub mod FBSTROBES;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.123 FCFG_B0_SSIZE0 Register (Offset = 2430h) [reset = 00200004h]
pub mod FCFG_B0_SSIZE0;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod FCFG_B0_SSIZE1;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod FCFG_B0_SSIZE2;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod FCFG_B0_SSIZE3;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.115 FCFG_B0_START Register (Offset = 2410h) [reset = 02000000h]
pub mod FCFG_B0_START;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod FCFG_B1_SSIZE0;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod FCFG_B1_SSIZE1;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod FCFG_B1_SSIZE2;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod FCFG_B1_SSIZE3;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.116 FCFG_B1_START Register (Offset = 2414h) [reset = 0h]
pub mod FCFG_B1_START;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod FCFG_B2_SSIZE0;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod FCFG_B2_SSIZE1;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod FCFG_B2_SSIZE2;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod FCFG_B2_SSIZE3;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.117 FCFG_B2_START Register (Offset = 2418h) [reset = 0h]
pub mod FCFG_B2_START;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod FCFG_B3_SSIZE0;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod FCFG_B3_SSIZE1;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod FCFG_B3_SSIZE2;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod FCFG_B3_SSIZE3;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.118 FCFG_B3_START Register (Offset = 241Ch) [reset = 0h]
pub mod FCFG_B3_START;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod FCFG_B4_SSIZE0;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod FCFG_B4_SSIZE1;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod FCFG_B4_SSIZE2;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod FCFG_B4_SSIZE3;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.119 FCFG_B4_START Register (Offset = 2420h) [reset = 0h]
pub mod FCFG_B4_START;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod FCFG_B5_SSIZE0;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod FCFG_B5_SSIZE1;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod FCFG_B5_SSIZE2;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod FCFG_B5_SSIZE3;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.120 FCFG_B5_START Register (Offset = 2424h) [reset = 0h]
pub mod FCFG_B5_START;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod FCFG_B6_SSIZE0;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod FCFG_B6_SSIZE1;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod FCFG_B6_SSIZE2;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod FCFG_B6_SSIZE3;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.121 FCFG_B6_START Register (Offset = 2428h) [reset = 0h]
pub mod FCFG_B6_START;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod FCFG_B7_SSIZE0;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod FCFG_B7_SSIZE1;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod FCFG_B7_SSIZE2;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod FCFG_B7_SSIZE3;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.122 FCFG_B7_START Register (Offset = 242Ch) [reset = 0h]
pub mod FCFG_B7_START;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.112 FCFG_BANK Register (Offset = 2400h) [reset = 401h]
pub mod FCFG_BANK;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.114 FCFG_BNK_TYPE Register (Offset = 2408h) [reset = 3h]
pub mod FCFG_BNK_TYPE;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.113 FCFG_WRAPPER Register (Offset = 2404h) [reset = 50009007h]
pub mod FCFG_WRAPPER;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod FCLKTRIM;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod FCOR_ERR_ADD;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod FCOR_ERR_CNT;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod FCOR_ERR_POS;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod FDIAGCTL;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.29 FEDACCTL1 Register (Offset = 2008h) [reset = 0h]
pub mod FEDACCTL1;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod FEDACCTL2;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod FEDACSDIS;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod FEDACSDIS2;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.30 FEDACSTAT Register (Offset = 201Ch) [reset = 0h]
pub mod FEDACSTAT;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.49 FEFUSECTL Register (Offset = 209Ch) [reset = 0701010Ah]
pub mod FEFUSECTL;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.51 FEFUSEDATA Register (Offset = 20A4h) [reset = 0h]
pub mod FEFUSEDATA;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.50 FEFUSESTAT Register (Offset = 20A0h) [reset = 0h]
pub mod FEFUSESTAT;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod FEMU_ADDR;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod FEMU_DLSW;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod FEMU_DMSW;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod FEMU_ECC;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.4 FLASH_SIZE Register (Offset = 2Ch) [reset = 0h]
pub mod FLASH_SIZE;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.41 FLOCK Register (Offset = 2064h) [reset = 55AAh]
pub mod FLOCK;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.39 FMAC Register (Offset = 2050h) [reset = 0h]
pub mod FMAC;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.102 FMC_REV_ID Register (Offset = 22A8h) [reset = X]
pub mod FMC_REV_ID;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.40 FMSTAT Register (Offset = 2054h) [reset = 0h]
pub mod FMSTAT;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.37 FPAC1 Register (Offset = 2048h) [reset = 02082081h]
pub mod FPAC1;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.38 FPAC2 Register (Offset = 204Ch) [reset = 0h]
pub mod FPAC2;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod FPAR_OVR;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod FPMTCTL;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod FPRIM_ADD_TAG;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.54 FPSTROBES Register (Offset = 2104h) [reset = 103h]
pub mod FPSTROBES;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod FRAW_DATAH;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod FRAW_DATAL;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod FRAW_ECC;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.27 FRDCTL Register (Offset = 2000h) [reset = 200h]
pub mod FRDCTL;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod FREDU_ADD_TAG;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.52 FSEQPMP Register (Offset = 20A8h) [reset = 85080000h]
pub mod FSEQPMP;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.99 FSM_ACC_EP Register (Offset = 2290h) [reset = 0h]
pub mod FSM_ACC_EP;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.98 FSM_ACC_PP Register (Offset = 228Ch) [reset = 0h]
pub mod FSM_ACC_PP;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.100 FSM_ADDR Register (Offset = 22A0h) [reset = 0h]
pub mod FSM_ADDR;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.108 FSM_BSLE0 Register (Offset = 22E0h) [reset = 0h]
pub mod FSM_BSLE0;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.109 FSM_BSLE1 Register (Offset = 22E4h) [reset = 0h]
pub mod FSM_BSLE1;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.110 FSM_BSLP0 Register (Offset = 22F0h) [reset = 0h]
pub mod FSM_BSLP0;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.111 FSM_BSLP1 Register (Offset = 22F4h) [reset = 0h]
pub mod FSM_BSLP1;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.72 FSM_CMD Register (Offset = 220Ch) [reset = 0h]
pub mod FSM_CMD;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.76 FSM_CMP_VSU Register (Offset = 221Ch) [reset = 0h]
pub mod FSM_CMP_VSU;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.94 FSM_EC_STEP_HEIGHT Register (Offset = 2278h) [reset = 0h]
pub mod FSM_EC_STEP_HEIGHT;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.89 FSM_ERA Register (Offset = 2264h) [reset = 0h]
pub mod FSM_ERA;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.80 FSM_ERA_OH Register (Offset = 222Ch) [reset = 1h]
pub mod FSM_ERA_OH;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.91 FSM_ERA_PUL Register (Offset = 226Ch) [reset = 00040BB8h]
pub mod FSM_ERA_PUL;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.84 FSM_ERA_PW Register (Offset = 2244h) [reset = 0h]
pub mod FSM_ERA_PW;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.103 FSM_ERR_ADDR Register (Offset = 22ACh) [reset = 0h]
pub mod FSM_ERR_ADDR;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.105 FSM_EXECUTE Register (Offset = 22B4h) [reset = 000A000Ah]
pub mod FSM_EXECUTE;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.77 FSM_EX_VAL Register (Offset = 2220h) [reset = 301h]
pub mod FSM_EX_VAL;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.96 FSM_FLES Register (Offset = 2280h) [reset = 0h]
pub mod FSM_FLES;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.69 FSM_GLBCTL Register (Offset = 2200h) [reset = 1h]
pub mod FSM_GLBCTL;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.87 FSM_MODE Register (Offset = 225Ch) [reset = 0h]
pub mod FSM_MODE;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.73 FSM_PE_OSU Register (Offset = 2210h) [reset = 0h]
pub mod FSM_PE_OSU;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.82 FSM_PE_VH Register (Offset = 2234h) [reset = 100h]
pub mod FSM_PE_VH;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.75 FSM_PE_VSU Register (Offset = 2218h) [reset = 0h]
pub mod FSM_PE_VSU;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.88 FSM_PGM Register (Offset = 2260h) [reset = 0h]
pub mod FSM_PGM;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.104 FSM_PGM_MAXPUL Register (Offset = 22B0h) [reset = 0h]
pub mod FSM_PGM_MAXPUL;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.90 FSM_PRG_PUL Register (Offset = 2268h) [reset = 00040032h]
pub mod FSM_PRG_PUL;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.83 FSM_PRG_PW Register (Offset = 2240h) [reset = 0h]
pub mod FSM_PRG_PW;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.93 FSM_PUL_CNTR Register (Offset = 2274h) [reset = 0h]
pub mod FSM_PUL_CNTR;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.79 FSM_P_OH Register (Offset = 2228h) [reset = 100h]
pub mod FSM_P_OH;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.78 FSM_RD_H Register (Offset = 2224h) [reset = 5Ah]
pub mod FSM_RD_H;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.85 FSM_SAV_ERA_PUL Register (Offset = 2254h) [reset = 0h]
pub mod FSM_SAV_ERA_PUL;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.81 FSM_SAV_PPUL Register (Offset = 2230h) [reset = 0h]
pub mod FSM_SAV_PPUL;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.101 FSM_SECTOR Register (Offset = 22A4h) [reset = FFFF0000h]
pub mod FSM_SECTOR;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.106 FSM_SECTOR1 Register (Offset = 22C0h) [reset = FFFFFFFFh]
pub mod FSM_SECTOR1;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.107 FSM_SECTOR2 Register (Offset = 22C4h) [reset = 0h]
pub mod FSM_SECTOR2;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.71 FSM_STAT Register (Offset = 2208h) [reset = 4h]
pub mod FSM_STAT;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.70 FSM_STATE Register (Offset = 2204h) [reset = C00h]
pub mod FSM_STATE;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.92 FSM_STEP_SIZE Register (Offset = 2270h) [reset = 0h]
pub mod FSM_STEP_SIZE;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.95 FSM_ST_MACHINE Register (Offset = 227Ch) [reset = 00800500h]
pub mod FSM_ST_MACHINE;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.86 FSM_TIMER Register (Offset = 2258h) [reset = 0h]
pub mod FSM_TIMER;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.74 FSM_VSTAT Register (Offset = 2214h) [reset = 3000h]
pub mod FSM_VSTAT;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.97 FSM_WR_ENA Register (Offset = 2288h) [reset = 2h]
pub mod FSM_WR_ENA;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.28 FSPRD Register (Offset = 2004h) [reset = 0h]
pub mod FSPRD;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.68 FSWSTAT Register (Offset = 2144h) [reset = 1h]
pub mod FSWSTAT;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.56 FTCR Register (Offset = 210Ch) [reset = 0h]
pub mod FTCR;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.58 FTCTL Register (Offset = 211Ch) [reset = 0h]
pub mod FTCTL;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod FUNC_ERR_ADD;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.43 FVHVCT1 Register (Offset = 2084h) [reset = 00840088h]
pub mod FVHVCT1;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.44 FVHVCT2 Register (Offset = 2088h) [reset = 00A20000h]
pub mod FVHVCT2;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.45 FVHVCT3 Register (Offset = 208Ch) [reset = 000F0000h]
pub mod FVHVCT3;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.46 FVNVCT Register (Offset = 2090h) [reset = 800h]
pub mod FVNVCT;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.42 FVREADCT Register (Offset = 2080h) [reset = 8h]
pub mod FVREADCT;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.47 FVSLP Register (Offset = 2094h) [reset = 8000h]
pub mod FVSLP;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.48 FVWLCT Register (Offset = 2098h) [reset = 8h]
pub mod FVWLCT;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.6 FWFLAG Register (Offset = 40h) [reset = 0h]
pub mod FWFLAG;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.5 FWLOCK Register (Offset = 3Ch) [reset = 0h]
pub mod FWLOCK;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.59 FWPWRITE0 Register (Offset = 2120h) [reset = FFFFFFFFh]
pub mod FWPWRITE0;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.60 FWPWRITE1 Register (Offset = 2124h) [reset = FFFFFFFFh]
pub mod FWPWRITE1;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.61 FWPWRITE2 Register (Offset = 2128h) [reset = FFFFFFFFh]
pub mod FWPWRITE2;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.62 FWPWRITE3 Register (Offset = 212Ch) [reset = FFFFFFFFh]
pub mod FWPWRITE3;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.63 FWPWRITE4 Register (Offset = 2130h) [reset = FFFFFFFFh]
pub mod FWPWRITE4;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.64 FWPWRITE5 Register (Offset = 2134h) [reset = FFFFFFFFh]
pub mod FWPWRITE5;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.65 FWPWRITE6 Register (Offset = 2138h) [reset = FFFFFFFFh]
pub mod FWPWRITE6;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.66 FWPWRITE7 Register (Offset = 213Ch) [reset = FFFFFFFFh]
pub mod FWPWRITE7;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.67 FWPWRITE_ECC Register (Offset = 2140h) [reset = FFFFFFFFh]
pub mod FWPWRITE_ECC;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod PBISTCTL;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] undocumented
pub mod ROM_TEST;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.25 SELFTESTCYC Register (Offset = 1048h) [reset = 0h]
pub mod SELFTESTCYC;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.26 SELFTESTSIGN Register (Offset = 104Ch) [reset = 0h]
pub mod SELFTESTSIGN;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.23 SINGLEBIT Register (Offset = 1040h) [reset = 0h]
pub mod SINGLEBIT;
/// FMC and Efuse Status
///
/// [TI-TRM-I] 7.9.1.1 STAT Register (Offset = 1Ch) [reset = 0h]
pub mod STAT;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.3 SYSCODE_START Register (Offset = 28h) [reset = 0h]
pub mod SYSCODE_START;
/// Internal. Only to be used through TI provided API.
///
/// [TI-TRM-I] 7.9.1.24 TWOBIT Register (Offset = 1044h) [reset = 0h]
pub mod TWOBIT;
