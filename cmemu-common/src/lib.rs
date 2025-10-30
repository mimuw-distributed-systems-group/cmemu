//! A library containing the minimal subset of core types formerly in cmemu-lib,
//! which are necessary for crate splitting.
//!
//! Do not add more types here, except to break dependency cycles.

pub mod address;
pub mod hw_register;

pub use address::Address;
pub use hw_register::HwRegister;
