use crate::terminate_port;
// Right now, the muxing is done in the bus_matrix module.
use super::{DCodeSPort, ICodeSPort};

terminate_port!(@configured_slave_input ICodeSPort);

pub(super) type IDCodeSPort = DCodeSPort;
