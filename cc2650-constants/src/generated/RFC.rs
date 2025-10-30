use cmemu_common::Address;
use core::ops::Range;
pub const START_ADDR: Address = PWR::ADDR;
pub use super::RFC_DBELL as DBELL;
pub use super::RFC_PWR as PWR;
pub use super::RFC_RAT as RAT;
pub const ADDR_SPACE: Range<Address> = START_ADDR..RAT::ADDR_SPACE.end;
