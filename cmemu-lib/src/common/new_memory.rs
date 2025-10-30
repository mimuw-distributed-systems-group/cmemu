use crate::bridge_ports;
use crate::common::new_ahb::databus::DataBus;
use crate::common::new_ahb::ports::{AHBPortConfig, AHBSlavePortOutput};
use crate::common::new_ahb::signals::Size;
use crate::common::new_ahb::slave_driver::WriteMode;
use crate::common::new_ahb::slave_driver::faking_slave_driver::{
    FakingHandler, FakingSlaveInterface, WaitstatesOrErr,
};
use crate::engine::{
    Context, DisableableComponent, SeqFlopMemoryBank, Subcomponent, TickComponent,
    TickComponentExtra,
};
use crate::utils::IfExpr;
use cmemu_common::Address;
#[cfg(feature = "poison-unitialized")]
use fixedbitset::FixedBitSet;
use std::ops::Range;
use thiserror::Error;

pub(crate) trait MemoryConfiguration: AHBPortConfig {
    const IS_WRITABLE: bool;
    const ADDRESS_SPACE: Range<Address>;
    const BUS_WIDTH: Size;
    const WAIT_STATES: u8;
}

#[derive(Error, Debug)]
pub enum MemoryError {
    #[error("The provided address range if invalid for this component.")]
    InvalidAddressRangeError,
}

// TODO(matrach): Consider generalizing to any transfer data type

type Iface<SC> = FakingSlaveInterface<FakingSC<SC>, Memory<SC>>;

struct MemoryBackend {
    mem: Vec<u8>,
    #[cfg(feature = "poison-unitialized")]
    initialized: FixedBitSet,
}

#[derive(Subcomponent, TickComponent, TickComponentExtra, DisableableComponent)]
pub(crate) struct Memory<SC>
where
    SC: Subcomponent<Member = Self>,
    Self: MemoryConfiguration + AHBSlavePortOutput<Component = SC::Component, Data = DataBus>,
{
    #[subcomponent(FakingSC)]
    slave_driver: Iface<SC>,

    // func: (where to write, data source) verified to be correct
    #[flop]
    memory: SeqFlopMemoryBank<MemoryBackend, (Range<usize>, DataBus)>,
}

// TODO: make auto naming work better and use @auto_configuration back
impl<SC> AHBPortConfig for Iface<SC>
where
    SC: Subcomponent<Member = Memory<SC>>,
    Memory<SC>: MemoryConfiguration + AHBSlavePortOutput<Component = SC::Component, Data = DataBus>,
{
    type Data = <Memory<SC> as AHBPortConfig>::Data;
    type Component = <Memory<SC> as AHBPortConfig>::Component;
    // we implemented tag here
    const TAG: &'static str = <Memory<SC> as AHBPortConfig>::TAG;
}
bridge_ports!(<SC> @slave Memory<SC> => @slave Iface<SC> where
    SC: Subcomponent<Member = Memory<SC>>,
    Memory<SC>: MemoryConfiguration + AHBSlavePortOutput<Component = SC::Component, Data = DataBus>,
);

impl<SC> Memory<SC>
where
    SC: Subcomponent<Member = Self>,
    Self: MemoryConfiguration + AHBSlavePortOutput<Component = SC::Component, Data = DataBus>,
{
    fn new(memory: MemoryBackend) -> Self {
        #[cfg(feature = "poison-unitialized")]
        debug_assert!(
            memory.mem.len() == memory.initialized.len(),
            "Memory size doesn't match initialization metadata"
        );
        debug_assert!(
            memory.mem.len() == Self::memory_size(),
            "Memory size does not match configuration"
        );
        Self {
            slave_driver: Default::default(),
            memory: SeqFlopMemoryBank::new(memory),
        }
    }

    pub(crate) fn new_with_contents(memory: &[u8]) -> Self {
        let mut mem = vec![0; Self::memory_size()];
        mem[..memory.len()].copy_from_slice(memory);

        #[cfg(feature = "poison-unitialized")]
        let mut initialized = FixedBitSet::with_capacity(Self::memory_size());

        #[cfg(feature = "poison-unitialized")]
        initialized.insert_range(..memory.len());
        Memory::new(MemoryBackend {
            mem,
            #[cfg(feature = "poison-unitialized")]
            initialized,
        })
    }

    pub(crate) fn new_zeroed() -> Self {
        let mem = vec![0; Self::memory_size()];
        Memory::new(MemoryBackend {
            mem,
            #[cfg(feature = "poison-unitialized")]
            initialized: FixedBitSet::with_capacity(Self::memory_size()),
        })
    }

    pub(crate) fn run_driver(slave: &mut SC::Component, ctx: &mut Context) {
        Iface::<SC>::run_driver(slave, ctx);
    }

    pub(crate) fn tock(slave: &mut SC::Component, ctx: &mut Context) {
        Iface::<SC>::tock(slave, ctx);
    }

    pub(crate) fn write_memory(
        &mut self,
        start_address: Address,
        data: &[u8],
    ) -> Result<(), MemoryError> {
        let addr_range = start_address..start_address.offset(u32::try_from(data.len()).unwrap());
        if Address::is_range_covered(&Self::ADDRESS_SPACE, &addr_range) {
            let addr = start_address.offset_from(Self::ADDRESS_SPACE.start) as usize;
            let inner_range = addr..addr + data.len();
            let backend = self.memory.unsafe_as_mut();
            #[cfg(feature = "poison-unitialized")]
            backend.initialized.insert_range(inner_range.clone());
            backend.mem[inner_range].copy_from_slice(data);
            Ok(())
        } else {
            Err(MemoryError::InvalidAddressRangeError)
        }
    }

    pub(crate) fn read_memory(
        &self,
        start_address: Address,
        buffer: &mut [u8],
    ) -> Result<(), MemoryError> {
        let addr_range = start_address..start_address.offset(u32::try_from(buffer.len()).unwrap());
        if Address::is_range_covered(&Self::ADDRESS_SPACE, &addr_range) {
            let addr = start_address.offset_from(Self::ADDRESS_SPACE.start) as usize;
            buffer.copy_from_slice(&self.memory.mem[addr..addr + buffer.len()]);
            Ok(())
        } else {
            Err(MemoryError::InvalidAddressRangeError)
        }
    }

    pub(crate) fn memory_size() -> usize {
        Self::ADDRESS_SPACE
            .end
            .offset_from(Self::ADDRESS_SPACE.start) as usize
    }

    fn check_transfer(addr: Address, size: Size) -> Result<Range<usize>, &'static str> {
        let addr_range = addr..addr.offset(u32::try_from(size.bytes()).unwrap());

        Address::is_range_covered(&Self::ADDRESS_SPACE, &addr_range)
            .or_err("Address not in range")?;

        debug_assert!(
            size <= Self::BUS_WIDTH,
            "Transfer size too large for {} (see: [ARM-AHB] 3.4 - Transfer size :: Note)",
            <Self as AHBPortConfig>::TAG,
        );

        debug_assert!(
            size.is_addr_aligned(addr),
            "Address is not aligned, {:?} for {:?}",
            { addr },
            { size }
        );

        let offset = addr.offset_from(Self::ADDRESS_SPACE.start) as usize;
        Ok(offset..offset + size.bytes())
    }
}

impl<SC> FakingHandler for Memory<SC>
where
    SC: Subcomponent<Member = Self>,
    Self: MemoryConfiguration + AHBSlavePortOutput<Component = SC::Component, Data = DataBus>,
{
    const WRITE_MODE: WriteMode = WriteMode::Combinatorial;

    #[cfg_attr(not(feature = "poison-unitialized"), allow(unused_variables))]
    fn pre_read(
        slave: &mut Self::Component,
        _ctx: &mut Context,
        address: Address,
        size: Size,
    ) -> WaitstatesOrErr {
        #[cfg(feature = "poison-unitialized")]
        {
            let range = Self::check_transfer(address, size)?;
            if range.len()
                != SC::component_to_member(slave)
                    .memory
                    .initialized
                    .count_ones(range)
            {
                return Err("Reading from uninitialized memory!");
            }
        }

        Ok(<Self as MemoryConfiguration>::WAIT_STATES)
    }

    fn read(
        slave: &mut Self::Component,
        _ctx: &mut Context,
        address: Address,
        size: Size,
    ) -> Self::Data {
        let range =
            Self::check_transfer(address, size).expect("Final request not coherent with initial?");
        Self::Data::from_slice(&SC::component_to_member(slave).memory.mem[range])
    }

    fn pre_write(
        _slave: &mut Self::Component,
        _ctx: &mut Context,
        address: Address,
        size: Size,
    ) -> WaitstatesOrErr {
        <Self as MemoryConfiguration>::IS_WRITABLE.or_err("Memory marked as read only!")?;
        Self::check_transfer(address, size)?;
        Ok(<Self as MemoryConfiguration>::WAIT_STATES)
    }

    fn write(slave: &mut Self::Component, _ctx: &mut Context, address: Address, data: Self::Data) {
        let range = Self::check_transfer(address, data.size())
            .expect("Final request not coherent with initial?");
        SC::component_to_member_mut(slave).memory.mutate_next(
            (range, data),
            |mem, (range, data)| {
                #[cfg(feature = "poison-unitialized")]
                mem.initialized.insert_range(range.clone());
                data.write_into_slice(&mut mem.mem[range]);
            },
        );
    }
}

/// Requested invalid address range
#[derive(Debug)]
pub(crate) struct InvalidAddressError;
