use cmemu_common::Address;
use core::ops::Range;

pub const BASE_ADDR: Address = Address::from_const(0x40081000);
pub const ADDR: Address = BASE_ADDR.offset(0x0);
pub const SIZE: u32 = 0x1000;
/// 0x40081000..0x40082000
pub const ADDR_SPACE: Range<Address> = ADDR..ADDR.offset(SIZE);

/// Configuration of DIO0
///
/// [TI-TRM-I] 11.11.3.1 IOCFG0 Register (Offset = 0h) [reset = 6000h]
pub mod IOCFG0;
/// Configuration of DIO1
///
/// [TI-TRM-I] 11.11.3.2 IOCFG1 Register (Offset = 4h) [reset = 6000h]
pub mod IOCFG1;
/// Configuration of DIO10
///
/// [TI-TRM-I] 11.11.3.11 IOCFG10 Register (Offset = 28h) [reset = 6000h]
pub mod IOCFG10;
/// Configuration of DIO11
///
/// [TI-TRM-I] 11.11.3.12 IOCFG11 Register (Offset = 2Ch) [reset = 6000h]
pub mod IOCFG11;
/// Configuration of DIO12
///
/// [TI-TRM-I] 11.11.3.13 IOCFG12 Register (Offset = 30h) [reset = 6000h]
pub mod IOCFG12;
/// Configuration of DIO13
///
/// [TI-TRM-I] 11.11.3.14 IOCFG13 Register (Offset = 34h) [reset = 6000h]
pub mod IOCFG13;
/// Configuration of DIO14
///
/// [TI-TRM-I] 11.11.3.15 IOCFG14 Register (Offset = 38h) [reset = 6000h]
pub mod IOCFG14;
/// Configuration of DIO15
///
/// [TI-TRM-I] 11.11.3.16 IOCFG15 Register (Offset = 3Ch) [reset = 6000h]
pub mod IOCFG15;
/// Configuration of DIO16
///
/// [TI-TRM-I] 11.11.3.17 IOCFG16 Register (Offset = 40h) [reset = 00086000h]
pub mod IOCFG16;
/// Configuration of DIO17
///
/// [TI-TRM-I] 11.11.3.18 IOCFG17 Register (Offset = 44h) [reset = 00106000h]
pub mod IOCFG17;
/// Configuration of DIO18
///
/// [TI-TRM-I] 11.11.3.19 IOCFG18 Register (Offset = 48h) [reset = 6000h]
pub mod IOCFG18;
/// Configuration of DIO19
///
/// [TI-TRM-I] 11.11.3.20 IOCFG19 Register (Offset = 4Ch) [reset = 6000h]
pub mod IOCFG19;
/// Configuration of DIO2
///
/// [TI-TRM-I] 11.11.3.3 IOCFG2 Register (Offset = 8h) [reset = 6000h]
pub mod IOCFG2;
/// Configuration of DIO20
///
/// [TI-TRM-I] 11.11.3.21 IOCFG20 Register (Offset = 50h) [reset = 6000h]
pub mod IOCFG20;
/// Configuration of DIO21
///
/// [TI-TRM-I] 11.11.3.22 IOCFG21 Register (Offset = 54h) [reset = 6000h]
pub mod IOCFG21;
/// Configuration of DIO22
///
/// [TI-TRM-I] 11.11.3.23 IOCFG22 Register (Offset = 58h) [reset = 6000h]
pub mod IOCFG22;
/// Configuration of DIO23
///
/// [TI-TRM-I] 11.11.3.24 IOCFG23 Register (Offset = 5Ch) [reset = 6000h]
pub mod IOCFG23;
/// Configuration of DIO24
///
/// [TI-TRM-I] 11.11.3.25 IOCFG24 Register (Offset = 60h) [reset = 6000h]
pub mod IOCFG24;
/// Configuration of DIO25
///
/// [TI-TRM-I] 11.11.3.26 IOCFG25 Register (Offset = 64h) [reset = 6000h]
pub mod IOCFG25;
/// Configuration of DIO26
///
/// [TI-TRM-I] 11.11.3.27 IOCFG26 Register (Offset = 68h) [reset = 6000h]
pub mod IOCFG26;
/// Configuration of DIO27
///
/// [TI-TRM-I] 11.11.3.28 IOCFG27 Register (Offset = 6Ch) [reset = 6000h]
pub mod IOCFG27;
/// Configuration of DIO28
///
/// [TI-TRM-I] 11.11.3.29 IOCFG28 Register (Offset = 70h) [reset = 6000h]
pub mod IOCFG28;
/// Configuration of DIO29
///
/// [TI-TRM-I] 11.11.3.30 IOCFG29 Register (Offset = 74h) [reset = 6000h]
pub mod IOCFG29;
/// Configuration of DIO3
///
/// [TI-TRM-I] 11.11.3.4 IOCFG3 Register (Offset = Ch) [reset = 6000h]
pub mod IOCFG3;
/// Configuration of DIO30
///
/// [TI-TRM-I] 11.11.3.31 IOCFG30 Register (Offset = 78h) [reset = 6000h]
pub mod IOCFG30;
/// Configuration of DIO31
///
/// [TI-TRM-I] 11.11.3.32 IOCFG31 Register (Offset = 7Ch) [reset = 6000h]
pub mod IOCFG31;
/// Configuration of DIO4
///
/// [TI-TRM-I] 11.11.3.5 IOCFG4 Register (Offset = 10h) [reset = 6000h]
pub mod IOCFG4;
/// Configuration of DIO5
///
/// [TI-TRM-I] 11.11.3.6 IOCFG5 Register (Offset = 14h) [reset = 6000h]
pub mod IOCFG5;
/// Configuration of DIO6
///
/// [TI-TRM-I] 11.11.3.7 IOCFG6 Register (Offset = 18h) [reset = 6000h]
pub mod IOCFG6;
/// Configuration of DIO7
///
/// [TI-TRM-I] 11.11.3.8 IOCFG7 Register (Offset = 1Ch) [reset = 6000h]
pub mod IOCFG7;
/// Configuration of DIO8
///
/// [TI-TRM-I] 11.11.3.9 IOCFG8 Register (Offset = 20h) [reset = 6000h]
pub mod IOCFG8;
/// Configuration of DIO9
///
/// [TI-TRM-I] 11.11.3.10 IOCFG9 Register (Offset = 24h) [reset = 6000h]
pub mod IOCFG9;
