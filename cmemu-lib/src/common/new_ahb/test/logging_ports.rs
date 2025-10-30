#[cfg(feature = "cycle-debug-logger")]
use crate::common::new_ahb::cdl::CdlTag;
use crate::common::new_ahb::ports::*;
use crate::common::new_ahb::ports::{AHBPortConfig, AHBSlavePortInput, AHBSlavePortOutput};
use crate::common::new_ahb::signals::{MasterToSlaveWires, SlaveToMasterWires, TransferMeta};
use crate::common::new_ahb::slave_driver::{
    SimpleHandler, SimpleResponse, SimpleSynchronousSlaveInterface, SimpleWriteResponse, WriteMode,
};
use crate::common::new_ahb::test::utils::*;
use crate::common::utils::SubcomponentProxyMut;
use crate::engine::{
    Context, DisableableComponent, Subcomponent, TickComponent, TickComponentExtra,
};
use std::fmt::Debug;
use std::marker::PhantomData;

pub struct PhonySC;

type TestIface<SC> = SimpleSynchronousSlaveInterface<SHandlerSC<SC>, TestSlave<SC>>;
#[derive(Subcomponent, TickComponent, TickComponentExtra, DisableableComponent, Default)]
pub(crate) struct TestSlave<SC>
where
    SC: Subcomponent<Member = TestSlave<SC>>
        + Checker<Data = <TestSlave<SC> as AHBPortConfig>::Data>,
    TestSlave<SC>: AHBSlavePortOutput<Component = SC::Component> + SimpleHandler,
{
    #[subcomponent(pub(crate) SHandlerSC)]
    pub iface: TestIface<SC>,
    pub delivered_read_req: Option<TransferMeta>,
    pub delivered_pre_write_req: Option<TransferMeta>,
    pub delivered_write_req: Option<TransferMeta>,
    pub delivered_write_data: Option<<Self as AHBPortConfig>::Data>,

    pub last_input: Option<MasterToSlaveWires<<Self as AHBPortConfig>::Data>>,
    pub response_iter: Option<<Vec<SimpleResponse<()>> as IntoIterator>::IntoIter>,
}

impl<SC> TestSlave<SC>
where
    SC: Subcomponent<Member = Self> + Checker<Data = <Self as AHBPortConfig>::Data>,
    Self: AHBSlavePortOutput<Component = SC::Component> + SimpleHandler,
    <Self as AHBPortConfig>::Data: Default,
{
    pub(crate) fn tick(comp: &mut SC::Component, ctx: &mut Context) {
        Self::run_driver(comp, ctx);
    }
    pub(crate) fn run_driver(comp: &mut SC::Component, ctx: &mut Context) {
        TestIface::<SC>::run_driver(comp, ctx);
    }
    pub(crate) fn tock(comp: &mut SC::Component, ctx: &mut Context) {
        TestIface::<SC>::tock(comp, ctx);
    }
}

pub(crate) trait Checker {
    type Data: Default;

    fn check_input(msg: &MasterToSlaveWires<Self::Data>) {}
    fn check_and_reply_read(request: &TransferMeta) -> Self::Data;
    fn check_pre_write(request: &TransferMeta) {}
    fn check_write(request: &TransferMeta, data: &Self::Data, post_success: bool) {}
}
impl<SC> AHBSlavePortInput for TestSlave<SC>
where
    SC: Subcomponent<Member = TestSlave<SC>> + Checker<Data = Self::Data>,
    SHandlerSC<SC>: Subcomponent<Member = TestIface<SC>, Component = SC::Component>,
    Self: AHBSlavePortOutput<Component = SC::Component> + SimpleHandler,
    <Self as AHBPortConfig>::Data: Debug + Clone,
{
    fn on_ahb_input(
        comp: &mut SC::Component,
        ctx: &mut Context,
        msg: MasterToSlaveWires<<Self as AHBPortConfig>::Data>,
    ) {
        println!("Slave {:?} got {:?}", <Self as AHBPortConfig>::TAG, msg);

        let mut this = SubcomponentProxyMut::<SC>::from(comp);
        <SC as Checker>::check_input(&msg);
        this.last_input = Some(msg.clone());

        TestIface::<SC>::on_ahb_input(this.component_mut(), ctx, msg);
    }
}

impl<SC> SimpleHandler for TestSlave<SC>
where
    SC: Subcomponent<Member = TestSlave<SC>> + Checker<Data = Self::Data>,
    Self: AHBSlavePortOutput<Component = SC::Component>,
{
    const WRITE_MODE: WriteMode = WriteMode::Combinatorial;

    fn read_data(
        slave: &mut Self::Component,
        _ctx: &mut Context,
        request: TransferMeta,
    ) -> SimpleResponse<Self::Data> {
        let val = <SC as Checker>::check_and_reply_read(&request);
        let mut this = SubcomponentProxyMut::<SC>::from(slave);
        this.delivered_read_req = Some(request);
        this.response_iter
            .as_mut()
            .map_or(SimpleResponse::SUCCESS, |iter| {
                iter.next().expect("Mock iterator exhausted")
            })
            .with_data(val)
    }

    fn pre_write(
        slave: &mut Self::Component,
        _ctx: &mut Context,
        request: TransferMeta,
    ) -> SimpleWriteResponse {
        <SC as Checker>::check_pre_write(&request);
        let mut this = SubcomponentProxyMut::<SC>::from(slave);
        this.delivered_pre_write_req = Some(request);
        this.response_iter
            .as_mut()
            .map_or(SimpleResponse::SUCCESS, |iter| {
                iter.next().expect("Mock iterator exhausted")
            })
    }

    fn write_data(
        slave: &mut Self::Component,
        _ctx: &mut Context,
        request: TransferMeta,
        data: Self::Data,
        post_success: bool,
    ) -> SimpleWriteResponse {
        <SC as Checker>::check_write(&request, &data, post_success);

        let mut this = SubcomponentProxyMut::<SC>::from(slave);
        this.delivered_write_req = Some(request);
        this.delivered_write_data = Some(data);
        if post_success {
            // this is our post-success response, maybe user should know about that anyway?
            // it is only asserted
            return SimpleResponse::SUCCESS;
        }
        this.response_iter
            .as_mut()
            .map_or(SimpleResponse::SUCCESS, |iter| {
                iter.next().expect("Mock iterator exhausted")
            })
    }
}

#[derive(Subcomponent, TickComponent, DisableableComponent)]
pub(crate) struct SimpleTestMaster<SC>
where
    SC: Subcomponent<Member = SimpleTestMaster<SC>>,
    SimpleTestMaster<SC>: AHBMasterPortOutput<Component = SC::Component>,
{
    pub last_resp: Option<SlaveToMasterWires<<Self as AHBPortConfig>::Data>>,
    sc: PhantomData<SC>,
}

impl<SC> TickComponentExtra for SimpleTestMaster<SC>
where
    SC: Subcomponent<Member = SimpleTestMaster<SC>>,
    SimpleTestMaster<SC>: AHBMasterPortOutput<Component = SC::Component>,
{
    fn tick_extra(&mut self) {
        self.last_resp = None;
    }
}
impl<SC> SimpleTestMaster<SC>
where
    SC: Subcomponent<Member = SimpleTestMaster<SC>>,
    SimpleTestMaster<SC>: AHBMasterPortOutput<Component = SC::Component>,
{
    pub(crate) fn send(
        comp: &mut SC::Component,
        ctx: &mut Context,
        mut msg: MasterToSlaveWires<<Self as AHBPortConfig>::Data>,
        reflect: bool,
    ) {
        #[cfg(feature = "cycle-debug-logger")]
        {
            msg.addr_phase.tag = <Self as AHBPortConfig>::TAG.into();
            msg.data_phase.tag = <Self as AHBPortConfig>::TAG.into();
        }
        if reflect {
            msg = reflect_hready(msg, SC::component_to_member(comp).last_resp.as_ref());
        }
        println!(
            "Master {:?} sending {:?}",
            <Self as AHBPortConfig>::TAG,
            msg
        );
        <Self as AHBMasterPortOutput>::send_ahb_output(comp, ctx, msg);
    }
}
impl<SC> AHBMasterPortInput for SimpleTestMaster<SC>
where
    SC: Subcomponent<Member = SimpleTestMaster<SC>>,
    SimpleTestMaster<SC>: AHBMasterPortOutput<Component = SC::Component>,
{
    fn on_ahb_input(
        comp: &mut Self::Component,
        ctx: &mut Context,
        msg: SlaveToMasterWires<Self::Data>,
    ) {
        let mut this = SubcomponentProxyMut::<SC>::from(comp);
        println!("Master {:?} got {:?}", <Self as AHBPortConfig>::TAG, msg);
        #[cfg(feature = "cycle-debug-logger")]
        assert!(
            msg.sender_tag == <Self as AHBPortConfig>::TAG || msg.sender_tag == CdlTag::default(),
            "Master {} got response not requested by it: {:?}",
            <Self as AHBPortConfig>::TAG,
            msg
        );
        this.last_resp = Some(msg);
    }
}
impl<SC> Default for SimpleTestMaster<SC>
where
    SC: Subcomponent<Member = SimpleTestMaster<SC>>,
    SimpleTestMaster<SC>: AHBMasterPortOutput<Component = SC::Component>,
{
    fn default() -> Self {
        Self {
            last_resp: None,
            sc: PhantomData,
        }
    }
}
