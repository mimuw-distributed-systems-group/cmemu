use cmemu_common::Address;
use core::ops::Range;

pub const BASE_ADDR: Address = Address::from_const(0xe0002000);
pub const ADDR: Address = BASE_ADDR.offset(0x0);
pub const SIZE: u32 = 0x1000;
/// 0xe0002000..0xe0003000
pub const ADDR_SPACE: Range<Address> = ADDR..ADDR.offset(SIZE);

/// Comparator 0
///
/// [TI-TRM-I] 2.7.2.3 COMP0 Register (Offset = 8h) [reset = 0h]
pub mod COMP0;
/// Comparator 1
///
/// [TI-TRM-I] 2.7.2.4 COMP1 Register (Offset = Ch) [reset = 0h]
pub mod COMP1;
/// Comparator 2
///
/// [TI-TRM-I] 2.7.2.5 COMP2 Register (Offset = 10h) [reset = 0h]
pub mod COMP2;
/// Comparator 3
///
/// [TI-TRM-I] 2.7.2.6 COMP3 Register (Offset = 14h) [reset = 0h]
pub mod COMP3;
/// Comparator 4
///
/// [TI-TRM-I] 2.7.2.7 COMP4 Register (Offset = 18h) [reset = 0h]
pub mod COMP4;
/// Comparator 5
///
/// [TI-TRM-I] 2.7.2.8 COMP5 Register (Offset = 1Ch) [reset = 0h]
pub mod COMP5;
/// Comparator 6
///
/// [TI-TRM-I] 2.7.2.9 COMP6 Register (Offset = 20h) [reset = 0h]
pub mod COMP6;
/// Comparator 7
///
/// [TI-TRM-I] 2.7.2.10 COMP7 Register (Offset = 24h) [reset = 0h]
pub mod COMP7;
/// Control
///
/// This register is used to enable the flash patch block.
///
/// [TI-TRM-I] 2.7.2.1 CTRL Register (Offset = 0h) [reset = 260h]
pub mod CTRL;
/// Remap
///
/// This register provides the remap base address location where a matched addresses are remapped. The three most significant bits and the five least significant bits of the remap base address are hard-coded to 3'b001 and 5'b00000 respectively. The remap base address must be in system space and is it required to be 8-word aligned, with one word allocated to each of the eight FPB comparators.
///
/// [TI-TRM-I] 2.7.2.2 REMAP Register (Offset = 4h) [reset = X]
pub mod REMAP;
