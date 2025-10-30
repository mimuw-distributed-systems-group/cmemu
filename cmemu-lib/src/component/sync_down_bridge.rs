use cmemu_proc_macros::proxy_use;

use crate::bridge_ports;
use crate::common::Address;
#[proxy_use]
use crate::common::new_ahb::databus::DataBus;
use crate::common::new_ahb::master_driver::MasterDriver;
use crate::common::new_ahb::master_driver::stateless_helpers::SimplerHandler as MasterDriverSimplerHandler;
use crate::common::new_ahb::ports::AhbMasterPortInputWithGranting;
#[proxy_use]
use crate::common::new_ahb::ports::{AHBMasterPortOutput, AHBPortConfig};
#[proxy_use]
use crate::common::new_ahb::signals::{Protection, Size};
use crate::common::new_ahb::slave_driver::stateless_simplifiers::SimplerHandler as SlaveDriverSimplerHandler;
use crate::common::new_ahb::slave_driver::{
    SimpleResponse, SimpleSynchronousSlaveInterface, SimpleWriteResponse, WriteMode,
};
use crate::component::aon_bus::SlowDecoder;
use crate::component::clock_tree::nodes::Divider;
#[proxy_use]
use crate::engine::Context;
use crate::engine::{DisableableComponent, Subcomponent, TickComponent, TickComponentExtra};

use crate::common::new_ahb::signals::TrackedBool;
use log::debug;

// TODO: is it right?
/// Subcomponent that allows HF-clocked masters to access LF-clocked slaves.

#[derive(Subcomponent, TickComponent, DisableableComponent)]
#[subcomponent_1to1]
pub(crate) struct SyncDownBridge {
    #[subcomponent(pub(crate) DriverSC)]
    driver: BusDriver,

    // Not a subcomponent as we need to tick it only in cycles where slow clock ticks.
    data_bus_driver: DBusDriver,

    // Bus handlers are executed in tock(), but accesses to other kinds of memory are only possible in tick().
    request_buffer: Option<DataRequest>,

    // TODO: make it a flop to show off that we do consume it in the next cycle.
    response_buffer: Option<DataResponse>,

    divider: Divider,
    is_free: bool,

    // Visibility for SlowDecoderSC's Subcomponent implementation
    pub(crate) decoder: SlowDecoder,
}
pub(crate) type BusDriver = SimpleSynchronousSlaveInterface<DriverSC, SyncDownBridge>;
pub(crate) type DBusDriver = MasterDriver<DBusDriverSC, SyncDownBridge>;

enum DataRequest {
    Read(Address, Size),
    Write(Address, DataBus),
}

enum DataResponse {
    Read(DataBus),
    Write,
}

impl SyncDownBridge {
    pub fn new() -> Self {
        Self {
            driver: Default::default(),
            data_bus_driver: Default::default(),
            request_buffer: None,
            response_buffer: None,
            divider: Divider::new(1536), // TODO: explain this constant (and change to it 1536 IIRC)
            is_free: true,
            decoder: SlowDecoder::new(),
        }
    }

    // pub fn divider(&self) -> &Divider {
    //     &self.divider
    // }

    pub fn divider_mut(&mut self) -> &mut Divider {
        &mut self.divider
    }

    pub fn sub_tick(comp: &mut <Self as Subcomponent>::Component, ctx: &mut Context) {
        BusDriver::run_driver(comp, ctx);

        // debug!("SyncDownBridge sub_tick");

        // The divider is ticked in tick_extra().
        if Self::component_to_member_mut(comp).divider.should_tick() {
            // debug!("SyncDownBridge sub_tick slow");
            DBusDriver::run_driver(comp, ctx);
            SlowDecoder::sub_tick(comp, ctx);
        }

        let this = Self::component_to_member_mut(comp);
        // debug!(
        //     "buffers: {:?}, {:?}",
        //     this.request_buffer.is_some(),
        //     this.response_buffer.is_some()
        // );

        // debug!(
        //     "is_free parts: {:?}, {:?}",
        //     this.divider.should_tick(),
        //     this.data_bus_driver.is_free()
        // );

        // TODO: is the request correctly buffered (and not received again until the response is sent)?

        // TODO: next cycle or prev cycle? Make sure the debug print is correct.

        if this.request_buffer.is_some()
            && this.divider.should_tick()
            && this.data_bus_driver.is_free()
        {
            match this.request_buffer.take().unwrap() {
                DataRequest::Read(addr, size) => {
                    assert!(this.data_bus_driver.try_read_data(addr, size, ()));
                    // debug!("Read requested");
                }
                DataRequest::Write(addr, data) => {
                    assert!(this.data_bus_driver.try_write_latched_data(
                        addr,
                        data.size(),
                        data,
                        ()
                    ));
                    // debug!("Write requested");
                }
            }
        }
    }

    pub fn tock(comp: &mut <Self as Subcomponent>::Component, ctx: &mut Context) {
        // debug!("SyncDownBridge tock");

        BusDriver::tock(comp, ctx);
        // debug!("divider tock");
        Self::component_to_member_mut(comp).divider.tock();

        if Self::component_to_member_mut(comp).divider.should_tock() {
            // debug!("SyncDownBridge tock slow");
            DBusDriver::tock(comp, ctx);
            SlowDecoder::tock(comp, ctx);
        }
    }
}

bridge_ports!(@slave SyncDownBridge => @slave BusDriver);
bridge_ports!(@auto_configured @master DBusDriver => @master SyncDownBridge);

impl SlaveDriverSimplerHandler for SyncDownBridge {
    const WRITE_MODE: WriteMode = WriteMode::Combinatorial;

    fn read_data(
        slave: &mut Self::Component,
        _ctx: &mut Context,
        address: Address,
        size: Size,
    ) -> SimpleResponse<DataBus> {
        debug!("SDB read: {:?} {:?}", address, size);

        let this = Self::component_to_member_mut(slave);

        if this.response_buffer.is_some() {
            // debug!("SDB read #1");

            match this.response_buffer.take().unwrap() {
                DataResponse::Read(data) => {
                    this.is_free = true;
                    SimpleResponse::Success(data)
                }
                DataResponse::Write => unreachable!(),
            }
        } else if this.is_free {
            // debug!("SDB read #2");
            this.request_buffer = Some(DataRequest::Read(address, size));
            this.is_free = false;
            SimpleResponse::Pending
        } else {
            // debug!("SDB read #3");
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
        debug!("SDB write: {:?} {:?}", address, data);

        let this = Self::component_to_member_mut(slave);

        if post_success {
            // debug!("SDB write #0");

            assert!(this.is_free && this.response_buffer.is_none());
            return SimpleWriteResponse::Success(());
        }

        if this.response_buffer.is_some() {
            // debug!("SDB write #1");
            match this.response_buffer.take().unwrap() {
                DataResponse::Write => {
                    this.is_free = true;
                    SimpleWriteResponse::SUCCESS
                }
                DataResponse::Read(_) => unreachable!(),
            }
        } else if this.is_free {
            // debug!("SDB write #2");
            this.request_buffer = Some(DataRequest::Write(address, data));
            this.is_free = false;
            SimpleResponse::Pending
        } else {
            // debug!("SDB write #3");
            SimpleResponse::Pending
        }
    }
}

impl MasterDriverSimplerHandler for SyncDownBridge {
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
        debug!("SDB read done: {:?} {:?}", addr, data);

        let this = Self::component_to_member_mut(comp);
        this.response_buffer = Some(DataResponse::Read(data));
    }

    fn write_done(
        comp: &mut <Self as AHBPortConfig>::Component,
        _ctx: &mut Context,
        addr: Address,
        _user: <Self as MasterDriverSimplerHandler>::UserData,
    ) {
        debug!("SDB write done: {:?}", addr);

        let this = Self::component_to_member_mut(comp);
        this.response_buffer = Some(DataResponse::Write);
    }
}

// impl TickComponent for SyncDownBridge {
//     #[cfg(debug_assertions)]
//     fn tick_assertions_traverse(&self) {
//         TickComponent::tick_assertions_traverse(&self.driver);
//         TickComponent::tick_assertions_traverse(&self.data_bus_driver);
//         TickComponentExtra::tick_assertions(self);
//     }
//     fn tick_flops_and_extra_traverse(&mut self) {
//         TickComponent::tick_flops_and_extra_traverse(&mut self.driver);
//         TickComponent::tick_flops_and_extra_traverse(&mut self.data_bus_driver);
//         TickComponent::tick_flops(self);
//         TickComponentExtra::tick_extra(self);
//     }
//     fn tick_flops(&mut self) {}
// }

// use std::backtrace::Backtrace; // TODO: remove
impl TickComponentExtra for SyncDownBridge {
    #[cfg(debug_assertions)]
    fn tick_assertions(&self) {
        // We do the assertion tick in tick_extra to avoid problems with mutating divider here (immutable `self`).
    }

    fn tick_extra(&mut self) {
        // debug!("tick_extra backtrace:\n{}", Backtrace::force_capture()); // TODO: remove

        // debug!("divider tick");
        self.divider.tick();

        #[cfg(debug_assertions)]
        if self.divider.should_tick() {
            TickComponent::tick_assertions_traverse(&self.data_bus_driver);
            TickComponent::tick_assertions_traverse(&self.decoder);
        }

        if self.divider.should_tick() {
            TickComponent::tick_flops_and_extra_traverse(&mut self.data_bus_driver);
            TickComponent::tick_flops_and_extra_traverse(&mut self.decoder);
        }
    }
}

impl AhbMasterPortInputWithGranting for SyncDownBridge
where
    Self: AHBMasterPortOutput,
{
    fn on_grant_wire(_comp: &mut Self::Component, _ctx: &mut Context, granted: TrackedBool) {
        if !*granted {
            unimplemented!("SyncDownBridge's deny_access() called!");
        }
    }
}

// Autoderived subcomponent copied from `cargo expand` output.
pub(crate) struct DBusDriverSC;
#[automatically_derived]
impl Default for DBusDriverSC {
    #[inline]
    fn default() -> DBusDriverSC {
        DBusDriverSC {}
    }
}
impl crate::engine::Subcomponent for DBusDriverSC {
    type Component = <SyncDownBridge as crate::engine::Subcomponent>::Component;
    type Member = DBusDriver;
    fn component_to_member(component: &Self::Component) -> &Self::Member {
        &<SyncDownBridge as crate::engine::Subcomponent>::component_to_member(component)
            .data_bus_driver
    }
    fn component_to_member_mut(component: &mut Self::Component) -> &mut Self::Member {
        &mut <SyncDownBridge as crate::engine::Subcomponent>::component_to_member_mut(component)
            .data_bus_driver
    }
}
