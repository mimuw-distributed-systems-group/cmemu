use crate::bridge_ports;
#[proxy_use]
use crate::common::Address;
#[proxy_use]
use crate::common::new_ahb::databus::DataBus;
use crate::common::new_ahb::ports::AHBSlavePortProxiedInput;
#[proxy_use]
use crate::common::new_ahb::ports::{AHBPortConfig, AHBSlavePortInput};
#[proxy_use]
use crate::common::new_ahb::signals::{MasterToSlaveWires, Size};
use crate::common::new_ahb::slave_driver::WriteMode;
use crate::common::new_ahb::slave_driver::faking_slave_driver::{
    FakingHandler, FakingIface, WaitstatesOrErr,
};
#[proxy_use]
use crate::engine::{
    Context, DisableableComponent, MainComponent, SkippableClockTreeNode, Subcomponent,
    TickComponent, TickComponentExtra,
};
use crate::proxy::{RTCBypassProxy, RTCProxy};
use cc2650_constants::AON_RTC as RTC;
use cmemu_proc_macros::{component_impl, handler, proxy_use};
use core::ops::Range;

#[derive(
    MainComponent, SkippableClockTreeNode, TickComponent, TickComponentExtra, DisableableComponent,
)]
#[skippable_if_disableable]
pub(crate) struct RTCBypass {
    #[subcomponent(SlaveDriverSubcomponent)]
    driver: BusDriver,

    bypassed_value: Option<(Address, DataBus)>,
}
type BusDriver = FakingIface<SlaveDriverSubcomponent, RTCBypass>;

#[component_impl(rtc_bypass)]
impl RTCBypass {
    pub fn new() -> Self {
        Self {
            driver: Default::default(),
            bypassed_value: None,
        }
    }
    pub fn tick(&mut self, ctx: &mut Context) {
        BusDriver::run_driver(self, ctx);
    }
    pub fn tock(&mut self, ctx: &mut Context) {
        BusDriver::tock(self, ctx);
    }

    #[handler]
    pub(crate) fn on_new_ahb_slave_input(
        &mut self,
        ctx: &mut Context,
        msg: MasterToSlaveWires<<RTCBypass as AHBPortConfig>::Data>,
    ) {
        <Self as AHBSlavePortInput>::on_ahb_input(self, ctx, msg);
    }

    #[handler]
    pub fn bypass_read(&mut self, _ctx: &mut Context, address: Address, data: DataBus) {
        self.bypassed_value = Some((address, data));
    }
}

bridge_ports!(@slave RTCBypass => @auto_configured @slave BusDriver);

#[component_impl(rtc_bypass)]
impl AHBPortConfig for RTCBypass {
    type Data = DataBus;
    type Component = Self;
    const TAG: &'static str = "RTCBypass";
}
#[component_impl(rtc_bypass)]
impl AHBSlavePortProxiedInput for RTCBypass {
    fn proxy_ahb_input(ctx: &mut Context, msg: MasterToSlaveWires<Self::Data>) {
        RTCBypassProxy.on_new_ahb_slave_input(ctx, msg);
    }
}

#[component_impl(rtc_bypass)]
impl FakingHandler for RTCBypass {
    const WRITE_MODE: WriteMode = WriteMode::Combinatorial;

    fn pre_read(
        _slave: &mut Self::Component,
        ctx: &mut Context,
        address: Address,
        size: Size,
    ) -> WaitstatesOrErr {
        RTCProxy.request_bypass_read(ctx, address, size);
        Ok(1) // just to make sure the bypass goes well
    }

    fn read(
        slave: &mut Self::Component,
        _ctx: &mut Context,
        address: Address,
        size: Size,
    ) -> Self::Data {
        let this = Self::component_to_member_mut(slave);
        let (bypassed_addr, data) = this.bypassed_value.take().unwrap();
        assert_eq!(bypassed_addr, address);
        assert_eq!(size, data.size());
        // eprintln!("n={:?} addr={address:?} size={size:?} read {data:?}", ctx.node_id);
        data
    }

    fn pre_write(
        _slave: &mut Self::Component,
        _ctx: &mut Context,
        _address: Address,
        _size: Size,
    ) -> WaitstatesOrErr {
        Ok(0)
    }

    fn write(_slave: &mut Self::Component, ctx: &mut Context, address: Address, data: Self::Data) {
        // eprintln!("n={:?} addr={address:?} write {data:?}", ctx.node_id);
        RTCProxy.bypass_write(ctx, address, data);
    }
}

pub const ROUTE_INJECTION: Range<Address> = RTC::SEC::ADDR..RTC::CH2CMP::ADDR;
