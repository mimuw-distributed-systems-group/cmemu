//! Memory: AHB(-Lite) subsystem
//!
//! Check the `README.md` for an in-depth description of the subsystem.
pub(crate) mod signals;

pub(crate) mod cdl;
pub(crate) mod output_stage;
#[macro_use]
pub(crate) mod decoder;
pub(crate) mod arbiter;
pub(crate) mod databus;
pub(crate) mod input_stage;
pub(crate) mod interconnect;
pub(crate) mod line_buffer;
pub(crate) mod master_driver;
pub(crate) mod ports;
pub(crate) mod slave_driver;
pub(crate) mod state_track;
#[cfg(test)]
mod test;
pub(crate) mod vlan;
pub(crate) mod write_buffer;

pub(crate) use databus::DataBus;
pub(crate) use ports::AHBPortConfig;
pub(crate) use signals::{MasterToSlaveWires, Size, SlaveToMasterWires};
