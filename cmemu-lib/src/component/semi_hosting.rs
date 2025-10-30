use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io::{ErrorKind, Read, Write, stderr, stdin, stdout};
use std::ops::Range;
use std::process::{ExitCode, Termination};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use num_enum::{IntoPrimitive, TryFromPrimitive};

use cc2650_constants::CoreMap::VENDOR_RANGE;
use cmemu_common::address_match_range_exhaustive;

use crate::bridge_ports;
use crate::common::Address;
use crate::common::new_ahb::databus::DataBus;
use crate::common::new_ahb::ports::AHBPortConfig;
use crate::common::new_ahb::signals::Size;
use crate::common::new_ahb::slave_driver::WriteMode;
use crate::common::new_ahb::slave_driver::faking_slave_driver::{
    FakingHandler, FakingIface, WaitstatesOrErr,
};
use crate::common::new_memory::{InvalidAddressError, Memory, MemoryConfiguration};
use crate::engine::{
    BufferFlop, Context, DisableableComponent, Subcomponent, TickComponent, TickComponentExtra,
};

use SpecialRegisters::*;

/// Exit requested by the guest.
///
/// This implementation is just because `ExitCode` from std doesn't allow getting the u8 back.
#[derive(Debug, Eq, PartialEq)]
pub struct RequestedExit(pub(crate) u8);

impl RequestedExit {
    pub fn code(&self) -> u8 {
        self.0
    }
}

impl Termination for RequestedExit {
    fn report(self) -> ExitCode {
        ExitCode::from(self.0)
    }
}

impl Display for RequestedExit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "guest exit code: {}", self.0)
    }
}

impl Error for RequestedExit {}

/// Address space: 64 kB at `0xFEED_0000`:
/// 0 – 4 kB: registers
/// 4 – 8 kB: stdout alias
/// 8 – 16 kB: stderr alias
/// unallocated
/// 32 – 64 kB: ROM memory to be initialized by a loader
pub const SEMI_HOSTING_BASE_ADDR: Address = Address::from_const(0xFEED_0000);
pub const SEMI_HOSTING_SIZE: u32 = 0x1_0000;
pub const SEMI_HOSTING_ADDR_SPACE: Range<Address> =
    SEMI_HOSTING_BASE_ADDR..SEMI_HOSTING_BASE_ADDR.offset(SEMI_HOSTING_SIZE);
cmemu_common::static_assert_is_subrange!(VENDOR_RANGE, SEMI_HOSTING_ADDR_SPACE);

// One could just memcopy a buffer here
pub const STDOUT_ALIAS_BASE: Address = SEMI_HOSTING_BASE_ADDR.offset(0x1000);
pub const STDOUT_ALIAS_RANGE: Range<Address> = STDOUT_ALIAS_BASE..STDOUT_ALIAS_BASE.offset(0x1000);
pub const STDERR_ALIAS_BASE: Address = SEMI_HOSTING_BASE_ADDR.offset(0x2000);
pub const STDERR_ALIAS_RANGE: Range<Address> = STDERR_ALIAS_BASE..STDERR_ALIAS_BASE.offset(0x1000);
cmemu_common::static_assert_is_subrange!(SEMI_HOSTING_ADDR_SPACE, STDOUT_ALIAS_RANGE);
cmemu_common::static_assert_is_subrange!(SEMI_HOSTING_ADDR_SPACE, STDERR_ALIAS_RANGE);

// Data for args and environment
pub const OS_DATA_BASE: Address = SEMI_HOSTING_BASE_ADDR.offset(0x8000);
pub const OS_DATA_RANGE: Range<Address> = OS_DATA_BASE..OS_DATA_BASE.offset(0x8000);
cmemu_common::static_assert_is_subrange!(SEMI_HOSTING_ADDR_SPACE, OS_DATA_RANGE);

// This should be a stable part for loader-apps interface.
pub const OS_DATA_ABI_VER: Address = OS_DATA_BASE.offset(0);
pub const OS_DATA_ARGC: Address = OS_DATA_BASE.offset(4);
pub const OS_DATA_ARGV: Address = OS_DATA_BASE.offset(8);
pub const OS_DATA_ENVIRON: Address = OS_DATA_BASE.offset(12);
// This is not really part of any interface, just to mark the first free element.
pub const OS_DATA_ARRAYS: Address = OS_DATA_BASE.offset(16);

#[derive(Debug, PartialEq, Clone, Copy, IntoPrimitive, TryFromPrimitive)]
#[repr(u16)]
pub enum SpecialRegisters {
    Panic = 0x0000,
    Exit = 0x0004,

    Stdout = 0x0010,
    Stderr = 0x0014,
    // Will hang emulator
    Stdin = 0x0018,
    StdinBytesOrEof = 0x001c,

    // 64 bit values are latched
    VirtualTimeL = 0x0020,
    VirtualTimeH = 0x0024,
    VirtualUSec = 0x0028,

    RealSec = 0x0030,
    RealNanoSec = 0x0034,
    RealFloat64 = 0x0038,
    RealFloat64H = 0x003c,

    UnixTimeSec = 0x0040,
    UnixTimeSecH = 0x0044,
    UnixTimeUSec = 0x0048,

    // OsData starts with argc, argv, environ,
    // but that's up to the app and loader, actually.
    OsData = 0x8000,
}

impl TryFrom<Address> for SpecialRegisters {
    type Error = &'static str;

    fn try_from(a: Address) -> Result<Self, Self::Error> {
        assert!(a.is_in_range(&SEMI_HOSTING_ADDR_SPACE));

        address_match_range_exhaustive!(a,
            STDOUT_ALIAS_RANGE => Ok(Stdout),
            STDERR_ALIAS_BASE => Ok(Stderr),
            OS_DATA_RANGE => Ok(OsData),
            SEMI_HOSTING_ADDR_SPACE => {
                let offset = u16::try_from(a.offset_from(SEMI_HOSTING_BASE_ADDR)).unwrap();
                Self::try_from(offset).map_err(|_x| "Invalid address")
            }
        )
    }
}

#[derive(Subcomponent, TickComponent, DisableableComponent)]
#[subcomponent_1to1]
pub(crate) struct SemiHosting {
    creation_time: Instant,
    #[subcomponent(DriverSC)]
    driver: BusDriver,

    #[subcomponent(pub(super) OsDataSC)]
    os_data: OsDataMemory,
    #[flop]
    systime: BufferFlop<SystemTime>,
}
type BusDriver = FakingIface<DriverSC, SemiHosting>;

impl MemoryConfiguration for OsDataMemory {
    const IS_WRITABLE: bool = false;
    const ADDRESS_SPACE: Range<Address> = OS_DATA_RANGE;
    const BUS_WIDTH: Size = Size::Word;
    const WAIT_STATES: u8 = 0;
}
// TODO: this type is exposed to put the routing inside sysbus, but it would be nicer to use a decoder here.
pub(super) type OsDataMemory = Memory<OsDataSC>;

impl TickComponentExtra for SemiHosting {
    fn tick_extra(&mut self) {
        // TODO: allow buffer flop to be skippable by default?
        self.systime.allow_skip();
    }
}

impl SemiHosting {
    pub(crate) fn new() -> Self {
        Self {
            creation_time: Instant::now(),
            driver: Default::default(),
            // TODO: should we worry for allocating oft unused 32K for each Emulator instance?
            //       We would need to modify ROM Memory impl to support lazy allocation...
            os_data: OsDataMemory::new_zeroed(),
            systime: Default::default(),
        }
    }

    pub(crate) fn tick(comp: &mut <Self as Subcomponent>::Component, ctx: &mut Context) {
        BusDriver::run_driver(comp, ctx);
        OsDataMemory::run_driver(comp, ctx);
    }

    pub(crate) fn tock(comp: &mut <Self as Subcomponent>::Component, ctx: &mut Context) {
        BusDriver::tock(comp, ctx);
        OsDataMemory::tock(comp, ctx);
    }

    /// Write to `OSData` memory
    pub(crate) fn write_memory(
        &mut self,
        start_address: Address,
        memory: &[u8],
    ) -> Result<(), InvalidAddressError> {
        self.os_data
            .write_memory(start_address, memory)
            .or(Err(InvalidAddressError))
    }

    /// Read from `OSData` memory
    pub(crate) fn read_memory(
        &self,
        start_address: Address,
        memory: &mut [u8],
    ) -> Result<(), InvalidAddressError> {
        self.os_data
            .read_memory(start_address, memory)
            .or(Err(InvalidAddressError))
    }
}

impl AHBPortConfig for SemiHosting {
    type Data = DataBus;
    type Component = <SemiHosting as Subcomponent>::Component;
    const TAG: &'static str = "SemiHosting";
}
bridge_ports!(@slave SemiHosting => @auto_configured @slave BusDriver);

impl AHBPortConfig for OsDataMemory {
    type Data = DataBus;
    type Component = <SemiHosting as Subcomponent>::Component;
    const TAG: &'static str = "SemiHosting:OSData";
}
// no bridge – connected directly at sysbus

impl FakingHandler for SemiHosting {
    const WRITE_MODE: WriteMode = WriteMode::Combinatorial;

    fn pre_read(
        _slave: &mut Self::Component,
        _ctx: &mut Context,
        address: Address,
        _size: Size,
    ) -> WaitstatesOrErr {
        assert!(address.is_aligned_to_4_bytes());
        let reg = SpecialRegisters::try_from(address)?;
        match reg {
            Panic | Exit | Stderr | Stdout => Err("Write only fields"),
            _ => Ok(0),
        }
    }

    fn read(
        slave: &mut Self::Component,
        ctx: &mut Context,
        address: Address,
        size: Size,
    ) -> Self::Data {
        let this = Self::component_to_member_mut(slave);

        match SpecialRegisters::try_from(address).expect("already verified?") {
            Stdin => {
                let (mut slice, s) = DataBus::make_slice(size);
                let slice = &mut slice[..s];
                stdin().read_exact(slice).expect("Stdin broken");
                DataBus::from_slice(slice)
            }
            StdinBytesOrEof => {
                let mut slice = [0u8; 1];
                let res: u32 = match stdin().read_exact(&mut slice) {
                    Ok(()) => u32::from(u8::from_le_bytes(slice)),
                    Err(ref e) if e.kind() == ErrorKind::UnexpectedEof => 0x100,
                    Err(_) => 0x200,
                };
                DataBus::Word(res)
            }
            VirtualTimeL | VirtualTimeH => {
                DataBus::Quad(ctx.event_queue().get_current_time().as_picos())
                    .extract_from_aligned(address, size)
            }
            #[allow(clippy::cast_possible_truncation, reason = "We want to trunc")]
            VirtualUSec => {
                DataBus::Word((ctx.event_queue().get_current_time().as_picos() / 1_000_000) as u32)
                    .extract_from_aligned(address, size)
            }
            #[allow(clippy::cast_possible_truncation, reason = "We want to trunc")]
            RealSec => DataBus::Word(this.creation_time.elapsed().as_secs() as u32)
                .extract_from_aligned(address, size),
            RealNanoSec => {
                todo!("Make it consistent!")
            }
            RealFloat64 | RealFloat64H => {
                todo!("Make it consistent!");
                // DataBus::from(this.creation_time.elapsed().as_secs_f64().to_le_bytes())
                //     .extract_from_aligned(address, size)
            }
            UnixTimeSec | UnixTimeSecH => {
                // Allow pipelined requests to see the same type value
                let systime = this.systime.try_take().unwrap_or_else(SystemTime::now);
                this.systime.set_this_cycle(systime);
                DataBus::Quad(systime.duration_since(UNIX_EPOCH).unwrap().as_secs())
                    .extract_from_aligned(address, size)
            }
            UnixTimeUSec => DataBus::Word(
                this.systime
                    .try_take()
                    .unwrap_or_else(SystemTime::now)
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .subsec_micros(),
            )
            .extract_from_aligned(address, size),
            _ => unreachable!(),
        }
    }

    fn pre_write(
        _slave: &mut Self::Component,
        _ctx: &mut Context,
        address: Address,
        _size: Size,
    ) -> WaitstatesOrErr {
        if !STDERR_ALIAS_RANGE.contains(&address) && !STDOUT_ALIAS_RANGE.contains(&address) {
            assert!(address.is_aligned_to_4_bytes());
        }
        let reg = SpecialRegisters::try_from(address)?;
        match reg {
            Panic | Exit | Stderr | Stdout => Ok(0),
            _ => Err("Read-only fields"),
        }
    }

    fn write(_slave: &mut Self::Component, _ctx: &mut Context, address: Address, data: DataBus) {
        match SpecialRegisters::try_from(address).expect("already verified?") {
            Panic => panic!("Semi_hosted panic: {data:?}!"),
            Exit => {
                // panic!("Ok {}", data);
                if cfg!(panic = "unwind") {
                    // Caught in main (or simply exiting with an error code (not passed)
                    #[allow(clippy::cast_possible_truncation, reason = "We want to trunc")]
                    std::panic::resume_unwind(Box::new(RequestedExit(u32::from(data) as u8)))
                } else {
                    //  the following doesn't call destructors (cdl won't be printed)
                    std::process::exit(u32::from(data).cast_signed())
                }
            }
            Stdout => data.map_into_slice(|s| stdout().write(s)),
            Stderr => data.map_into_slice(|s| stderr().write(s)),
            _ => unreachable!(),
        }
        .expect("Cannot fail here...");
    }
}
