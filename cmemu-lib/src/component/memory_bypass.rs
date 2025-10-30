//! The dirty method for fetching data from various chip memories
use crate::common::new_ahb::databus::DataBus;
use crate::common::new_ahb::ports::AHBSlavePortOutput;
use crate::common::new_memory::{Memory, MemoryConfiguration};
use crate::component::rfc::RfcMemoryBypassReceiver;
use crate::engine::{Context, Subcomponent};
use crate::proxy::RfcProxy;
use cmemu_common::Address;

#[derive(Clone, Copy, Debug)]
pub(crate) enum MemoryBypassReceiver {
    Rfc(RfcMemoryBypassReceiver),
}

impl<SC> Memory<SC>
where
    SC: Subcomponent<Member = Self>,
    Self: MemoryConfiguration + AHBSlavePortOutput<Component = SC::Component, Data = DataBus>,
{
    pub(crate) fn request_memory_read_bypass(
        &self,
        ctx: &mut Context,
        start_address: Address,
        mut buffer: Vec<u8>,
        size: usize,
        receiver: MemoryBypassReceiver,
    ) {
        debug_assert!(
            size <= buffer.capacity(),
            "failing to fit {size} bytes for {receiver:?} in {buffer:?}"
        );
        buffer.resize(size, 0);
        self.read_memory(start_address, buffer.as_mut_slice())
            .expect("Tried to read outside of memory address space.");
        // There are no other receivers than Rfc right now.
        match receiver {
            MemoryBypassReceiver::Rfc(subreceiver) => {
                RfcProxy.receive_memory_bypass(ctx, buffer, start_address, subreceiver);
            }
        }
    }

    pub(crate) fn request_memory_write_bypass(
        &mut self,
        _ctx: &mut Context,
        start_address: Address,
        data: DataBus,
    ) {
        let (mut buf, buf_size) = DataBus::make_slice(data.size());
        data.write_into_slice(&mut buf[..buf_size]);
        let buf = &buf[..buf_size];

        self.write_memory(start_address, buf).unwrap();
    }
}
