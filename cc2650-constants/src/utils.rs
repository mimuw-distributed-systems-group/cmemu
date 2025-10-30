use crate::SYSBUS_PERIPH_ADDR_SPACE;
use cmemu_common::Address;
use std::ops::Range;

const SYSBUS_UNBUFF_OFFSET: u32 = 0x2000_0000;
const SYSBUS_PERIPH_UNBUFFERED_RANGE: Range<Address> =
    SYSBUS_PERIPH_ADDR_SPACE.start.offset(SYSBUS_UNBUFF_OFFSET)
        ..SYSBUS_PERIPH_ADDR_SPACE.end.offset(SYSBUS_UNBUFF_OFFSET);

/// Returns unbuffered alias of given address.
///
/// # Panics
///
/// Panics if the address is outside [`SYSBUS_PERIPH_ADDR_SPACE`].
#[must_use]
pub const fn unbuffered_alias(addr: Address) -> Address {
    match addr {
        _ if addr.is_in_range(&SYSBUS_PERIPH_ADDR_SPACE) => addr.offset(0x2000_0000),
        _ => panic!("The address in not in unbuffered range!"),
    }
}

pub const fn is_unbuffered_alias(addr: Address) -> Option<Address> {
    match addr {
        _ if addr.is_in_range(&SYSBUS_PERIPH_UNBUFFERED_RANGE) => {
            Some(addr.offset(!SYSBUS_UNBUFF_OFFSET + 1))
        }
        _ => None,
    }
}

/// Helper struct to allow basic manipulation of `Address` in contexts such as match arms,
/// where macros and method calls are not available.
///
/// ```rust
/// use cmemu_common::Address;
/// use cc2650_constants::{AddressExt as A, SRAM};
///
/// let result = match Address::from_const(0x2000_0001) {
///     SRAM::BASE_ADDR => "SRAM",
///     A::<{ SRAM::BASE_ADDR.to_const() }>::BYTE_2ND => "SRAM+1",
///     _ => "",
/// };
/// assert_eq!(result, "SRAM+1");
/// ```
// TODO: simplify the fake u32 when adt_const_params comes to stable
#[allow(clippy::exhaustive_structs, missing_debug_implementations)]
pub struct AddressExt<const ADDRESS: u32>;

impl<const ADDRESS: u32> AddressExt<ADDRESS> {
    pub const ITSELF: Address = Address::from_const(ADDRESS);
    pub const BYTE_1ST: Address = Address::from_const(ADDRESS);
    pub const BYTE_2ND: Address = Address::from_const(ADDRESS + 1);
    pub const BYTE_3RD: Address = Address::from_const(ADDRESS + 2);
    pub const BYTE_4TH: Address = Address::from_const(ADDRESS + 3);
    pub const UNBUFFERED: Address = unbuffered_alias(Address::from_const(ADDRESS));
    pub const UNBUF_A: u32 = unbuffered_alias(Address::from_const(ADDRESS)).to_const();
}
