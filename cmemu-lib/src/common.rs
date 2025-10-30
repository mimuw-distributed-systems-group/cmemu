//! Commonly used internal data types & submodules as well as public types
//! used in cmemu-lib API ("component / emulator API").

// data types
pub mod bitstring;
mod shift;

// utils
#[macro_use]
pub(crate) mod utils;
#[macro_use]
pub(crate) mod new_ahb;
pub(crate) mod new_memory;
pub(crate) mod pending;
// `pub` because it needs to be tested.

// exports
// TODO: maybe CoreRegisterID would be a better name for an export?
pub use crate::component::core::{CoreCoupledRegisterId, RegisterID, SpecialPurposeRegisterId};
pub use crate::component::rfc::command::CcaReq;
pub use crate::component::rfc::{ModemImpl, ModemInterface, ModemOp};
pub use crate::component::semi_hosting::RequestedExit;
pub mod cmemu_hosting {
    pub use crate::component::semi_hosting::{
        OS_DATA_ABI_VER, OS_DATA_ARGC, OS_DATA_ARGV, OS_DATA_ARRAYS, OS_DATA_BASE, OS_DATA_ENVIRON,
        OS_DATA_RANGE,
    };
}
pub use crate::component::uart_lite::UARTLiteInterface;
pub use bitstring::{Bitstring, BitstringUtils, Word};
pub(crate) use shift::{SRType, Shift};

// reexports from cmemu-common
pub use cmemu_common::Address;
