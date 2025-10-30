use cmemu_proc_macros::proxy_use;

use crate::bridge_ports;
use crate::common::Address;
#[proxy_use]
use crate::common::new_ahb::databus::DataBus;
use crate::common::new_ahb::master_driver::MasterDriver;
use crate::common::new_ahb::master_driver::stateless_helpers::SimplerHandler as MasterDriverSimplerHandler;
#[proxy_use]
use crate::common::new_ahb::ports::AHBPortConfig;
#[proxy_use]
use crate::common::new_ahb::signals::{Protection, Size};
use crate::common::new_ahb::slave_driver::stateless_simplifiers::SimplerHandler as SlaveDriverSimplerHandler;
use crate::common::new_ahb::slave_driver::{
    SimpleResponse, SimpleSynchronousSlaveInterface, SimpleWriteResponse, WriteMode,
};
#[proxy_use]
use crate::engine::Context;
use crate::engine::{
    BufferFlop, DisableableComponent, Subcomponent, TickComponent, TickComponentExtra,
};
use cc2650_constants::CoreMap;
use cmemu_common::address::offset_range;
use std::ops::Range;

/// [ARM-TRM] 3.4 - Figure 3-1 - SRAM
const SRAM_BITBAND_REGION: Range<Address> = offset_range(
    CoreMap::BIT_BAND_REGION_RANGE,
    CoreMap::SRAM_RANGE.start.to_const(),
);
/// [ARM-TRM] 3.4 - Figure 3-1 - Peripheral
const PERIPH_BITBAND_REGION: Range<Address> = offset_range(
    CoreMap::BIT_BAND_REGION_RANGE,
    CoreMap::PERIPH_RANGE.start.to_const(),
);
/// [ARM-TRM] 3.4 - Figure 3-1
const BITBAND_ALIAS_TO_BITBAND_REGION_BASE_OFFSET: u32 =
    CoreMap::BIT_BAND_ALIAS_RANGE.start.to_const();

pub(crate) const SRAM_BITBAND_ALIAS_REGION: Range<Address> =
    to_bitband_alias_range(SRAM_BITBAND_REGION).unwrap();
pub(crate) const PERIPH_BITBAND_ALIAS_REGION: Range<Address> =
    to_bitband_alias_range(PERIPH_BITBAND_REGION).unwrap();

/// Subcomponent that handles requests translation from Bus Matrix to System Bus.
///
/// The high level idea is that we schedule accesses to SRAM and Periph whenever bus drivers are free.
/// We have to do the actual memory accesses and checking `is_free()` in `tick()` because these operations
/// are forbidden from happening in `tock()`.
///
/// Remarks about implementation:
/// It doesn't care about cycle-accuracy.
/// It wasn't written with interrupts and other complicated aspects of emulation in mind.
/// It disallows unaligned accesses.
/// It disallows accesses outside of bit-band alias region.
///
/// Contains quite a lot of code copied from previous implementation
/// which could be found at cmemu-framework/cmemu-lib/src/component/bus_matrix/bitband.rs.

#[derive(Subcomponent, TickComponent, TickComponentExtra, DisableableComponent)]
#[subcomponent_1to1]
pub(crate) struct Bitband {
    #[subcomponent(pub(crate) DriverSC)]
    driver: BusDriver,

    #[subcomponent(pub(crate) DBusDriverSC)]
    data_bus_driver: DBusDriver,

    #[flop]
    read_reg: BufferFlop<(Address, DataBus)>,

    // Bus handlers are executed in tock(), but accesses to other kinds of memory are only possible in tick().
    request_buffer: Option<DataRequest>,

    is_free: bool,
}
pub(crate) type BusDriver = SimpleSynchronousSlaveInterface<DriverSC, Bitband>;
pub(crate) type DBusDriver = MasterDriver<DBusDriverSC, Bitband>;

enum DataRequest {
    Read(Address),
    Write(Address, u8),
}

impl Bitband {
    pub fn new() -> Self {
        Self {
            driver: Default::default(),
            data_bus_driver: DBusDriver::new(),
            read_reg: BufferFlop::new(),
            request_buffer: None,
            is_free: true,
        }
    }

    pub fn tick(comp: &mut <Self as Subcomponent>::Component, ctx: &mut Context) {
        BusDriver::run_driver(comp, ctx);
        DBusDriver::run_driver(comp, ctx);

        let this = Self::component_to_member_mut(comp);
        this.is_free = this.request_buffer.is_none() && this.data_bus_driver.is_free();

        if let Some(request) = this.request_buffer.take() {
            match request {
                DataRequest::Read(normal_address) => {
                    assert!(
                        this.data_bus_driver
                            .try_read_data(normal_address, Size::Byte, ())
                    );
                }
                DataRequest::Write(normal_address, data) => {
                    assert!(this.data_bus_driver.try_write_latched_data(
                        normal_address,
                        Size::Byte,
                        DataBus::Byte(data),
                        ()
                    ));
                }
            }
        }
    }

    pub fn tock(comp: &mut <Self as Subcomponent>::Component, ctx: &mut Context) {
        BusDriver::tock(comp, ctx);
        DBusDriver::tock(comp, ctx);
    }
}

bridge_ports!(@slave Bitband => @slave BusDriver);
bridge_ports!(@auto_configured @master DBusDriver => @master Bitband);

impl AHBPortConfig for Bitband {
    type Data = DataBus;
    type Component = <Bitband as Subcomponent>::Component;
    const TAG: &'static str = "Bitband";
}

impl SlaveDriverSimplerHandler for Bitband {
    const WRITE_MODE: WriteMode = WriteMode::Combinatorial;

    fn read_data(
        slave: &mut Self::Component,
        _ctx: &mut Context,
        address: Address,
        size: Size,
    ) -> SimpleResponse<DataBus> {
        // The implementation is brutish:
        // 1. Wait until the bus driver is free, in the meantime return pending.
        // 2. Once bus driver is free send a request to the appropriate subcomponent.
        //    Await response, in the meantime return pending.
        // 3. When the response arrives use the read data to compute the value to return and return it.
        let this = Self::component_to_member_mut(slave);

        let (normal_address, bit, offset) =
            destruct_bitband_address(address).expect("Bitband address expected");

        assert!(offset == 0 && (0..=7).contains(&bit));
        if this.read_reg.is_set() {
            let (read_reg_addr, read_reg_data) = this.read_reg.take();
            assert!(read_reg_addr == normal_address);

            let DataBus::Byte(read_reg_byte) = read_reg_data else {
                panic!("We requested a byte read!")
            };
            let bit_set = 1 & (read_reg_byte >> bit);
            let response_data = match size {
                Size::Word => DataBus::Word(u32::from(bit_set)),
                Size::Halfword => DataBus::Short(u16::from(bit_set)),
                Size::Byte => DataBus::Byte(bit_set),
                _ => unreachable!(),
            };

            SimpleResponse::Success(response_data)
        } else if this.is_free {
            this.request_buffer = Some(DataRequest::Read(normal_address));
            this.is_free = false;
            SimpleResponse::Pending
        } else {
            SimpleResponse::Pending
        }
    }

    fn pre_write(
        _slave: &mut Self::Component,
        _ctx: &mut Context,
        _address: Address,
        _size: Size,
    ) -> SimpleWriteResponse {
        SimpleWriteResponse::Pending
    }

    fn write_data(
        slave: &mut Self::Component,
        _ctx: &mut Context,
        address: Address,
        data: DataBus,
        post_success: bool,
    ) -> SimpleWriteResponse {
        // The implementation is brutish:
        // 1. Wait until the bus driver is free, in the meantime return pending.
        // 2. Once bus driver is free send a byte read request to the appropriate subcomponent.
        //    Await response, in the meantime return pending.
        // 3. When the response arrives, use it to calculate a new value of the byte and send
        //    a byte write request to the appropriate subcomponent. Return success.
        // 4. Any other transfer would have to wait for the driver to finish.
        let this = Self::component_to_member_mut(slave);

        if post_success {
            assert!(!this.read_reg.is_set());
            return SimpleWriteResponse::SUCCESS;
        }

        let (normal_address, bit, offset) =
            destruct_bitband_address(address).expect("Bitband address expected");
        assert!(offset == 0 && (0..=7).contains(&bit));

        if this.read_reg.is_set() {
            let (read_reg_addr, read_reg_data) = this.read_reg.take();
            assert_eq!(normal_address, read_reg_addr);

            let DataBus::Byte(read_byte_data) = read_reg_data else {
                panic!("We requested a byte read!")
            };
            let bit_set = (data.raw() & 1) as u8;
            let mask: u8 = !(1 << bit);
            let out_data: u8 = (read_byte_data & mask) | (bit_set << bit);

            this.request_buffer = Some(DataRequest::Write(normal_address, out_data));
            this.is_free = false;

            SimpleWriteResponse::SUCCESS
        } else if this.is_free {
            this.request_buffer = Some(DataRequest::Read(normal_address));
            this.is_free = false;
            SimpleResponse::Pending
        } else {
            SimpleResponse::Pending
        }
    }
}

impl MasterDriverSimplerHandler for Bitband {
    type UserData = ();
    type MasterDriverSC = DBusDriverSC;
    const AHB_LITE_COMPAT: bool = false;
    const DEFAULT_PROT: Protection = Protection::new_data();

    fn read_done(
        comp: &mut <Self as AHBPortConfig>::Component,
        _ctx: &mut Context,
        addr: Address,
        data: DataBus,
        _user: <Self as MasterDriverSimplerHandler>::UserData,
    ) {
        let this = Self::component_to_member_mut(comp);
        this.read_reg.set_next((addr, data));
    }
}

// ================================ BITBAND ADDRESS-RELATED LOGIC ================================

const fn to_bitband_alias_range(range: Range<Address>) -> Option<Range<Address>> {
    // `?` is not supported in const context yet
    let Some(alias_start) = make_bitband_address(range.start, 0) else {
        return None;
    };

    #[allow(clippy::cast_sign_loss)]
    let range_end_incl = range.end.offset(-4_i32 as u32);

    let alias_end = match make_bitband_address(range_end_incl, 31) {
        Some(addr) => addr.offset(4),
        None => return None,
    };

    Some(alias_start..alias_end)
}

/// Makes bitband address for given register `reg_addr` and given bit `bit_number` in it
/// [ARM-TRM] 3.7.1 (About bit-banding)
const fn make_bitband_address(reg_addr: Address, bit_number: u32) -> Option<Address> {
    let base_addr = if reg_addr.is_in_range(&SRAM_BITBAND_REGION) {
        SRAM_BITBAND_REGION.start
    } else if reg_addr.is_in_range(&PERIPH_BITBAND_REGION) {
        PERIPH_BITBAND_REGION.start
    } else {
        return None;
    };

    let bitband_alias_region_base = base_addr.offset(BITBAND_ALIAS_TO_BITBAND_REGION_BASE_OFFSET);
    let byte_offset = reg_addr.offset_from(base_addr);

    let bit_word_offset = byte_offset * 32 + bit_number * 4;
    let bit_word_addr = bitband_alias_region_base.offset(bit_word_offset);

    Some(bit_word_addr)
}

/// Computes register address + bit + offset (aka remainder, < 4) out of bitband address `bitband_addr`.
/// Effectively, a reverse function to `make_bitband_address`.
/// [ARM-TRM] 3.7.1 (About bit-banding)
const fn destruct_bitband_address(bitband_addr: Address) -> Option<(Address, u8, u8)> {
    let base_addr = if bitband_addr.is_in_range(&SRAM_BITBAND_ALIAS_REGION) {
        SRAM_BITBAND_REGION.start
    } else if bitband_addr.is_in_range(&PERIPH_BITBAND_ALIAS_REGION) {
        PERIPH_BITBAND_REGION.start
    } else {
        return None;
    };

    let base_alias_addr = base_addr.offset(BITBAND_ALIAS_TO_BITBAND_REGION_BASE_OFFSET);

    let bitband_addr_offset = bitband_addr.offset_from(base_alias_addr);

    #[allow(clippy::cast_possible_truncation)]
    let remainder = (bitband_addr_offset & 0x3) as u8;
    #[allow(clippy::cast_possible_truncation)]
    let bit_number = ((bitband_addr_offset >> 2) & 0x7) as u8;
    let reg_addr = base_addr.offset(bitband_addr_offset >> 5);

    Some((reg_addr, bit_number, remainder))
}

// Const-time bitband edge-case correctness check. It's unused, just needs to compile.
const _SRAM_ALIAS_REGION_EDGE_CASE_CHECK: () = {
    // [ARM-TDG] Table 5.2 last row - last address inside sram alias region
    const SRAM_ALIAS_REGION_END_INCL_ADDRESS: Address = Address::from_const(0x23FF_FFFC);
    const SRAM_ALIAS_REGION_END_ADDRESS: Address = SRAM_ALIAS_REGION_END_INCL_ADDRESS.offset(4);

    const _: () =
        assert!(SRAM_ALIAS_REGION_END_INCL_ADDRESS.is_in_range(&SRAM_BITBAND_ALIAS_REGION));
    const _: () = assert!(!SRAM_ALIAS_REGION_END_ADDRESS.is_in_range(&SRAM_BITBAND_ALIAS_REGION));
};
