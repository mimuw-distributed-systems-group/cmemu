#![allow(clippy::too_many_arguments)]
use crate::common::Address;
use crate::common::new_ahb::databus::DataBus;
use crate::common::new_ahb::databus::DataBus::*;
use crate::common::new_ahb::ports::{AHBPortConfig, AHBSlavePortOutput};
use crate::common::new_ahb::signals::AhbResponseControl::*;
use crate::common::new_ahb::signals::{AhbResponseControl, MasterToSlaveAddrPhase, Size};
use crate::common::new_ahb::slave_driver::WriteMode;
use crate::common::new_ahb::slave_driver::faking_slave_driver::{
    FakingHandler, FakingSlaveInterface, WaitstatesOrErr,
};
use crate::common::new_ahb::test::logging_ports::SimpleTestMaster;
#[allow(unused_imports)] // It is used, but some tools don't see that.
use crate::common::new_ahb::test::utils::{idle, make_m2s, read, write};
use crate::engine::{
    Context, DisableableComponent, Subcomponent, TickComponent, TickComponentExtra,
};
use crate::test_utils::inc_time;
#[allow(unused_imports)] // It is used, but some tools don't see that.
use crate::{atom_vec, auto_vec};
use crate::{bridge_ports, mix_blocks, zip};
use itertools::Itertools;
use rstest::*;

#[derive(Subcomponent, TickComponent, TickComponentExtra, DisableableComponent, Default)]
struct TestComponent {
    #[subcomponent(SlaveSC)]
    s: Slave,

    #[subcomponent(MasterSC)]
    m: Master,
}

type Slave = FakingTestSlave<SlaveSC>;
type Master = SimpleTestMaster<MasterSC>;

impl TestComponent {
    fn tick(&mut self, ctx: &mut Context) {
        #[cfg(debug_assertions)]
        self.tick_assertions_traverse();
        self.tick_flops_and_extra_traverse();
        Slave::tick(self, ctx);
    }

    fn tock(&mut self, ctx: &mut Context) {
        Slave::tock(self, ctx);
    }
}

impl AHBPortConfig for Master {
    type Data = DataBus;
    type Component = TestComponent;
    const TAG: &'static str = "Master";
}

bridge_ports!(Master => @auto_configured Slave);

bridge_ports!(<SC> @slave FakingTestSlave<SC> => @auto_configured @slave TestIface<SC> where
    SC: Subcomponent<Member = FakingTestSlave<SC>>,
    FakingTestSlave<SC>: AHBSlavePortOutput<Component = SC::Component, Data=DataBus> + FakingHandler,
);

type TestIface<SC> = FakingSlaveInterface<SHandlerSC<SC>, FakingTestSlave<SC>>;
#[derive(Subcomponent, TickComponent, TickComponentExtra, DisableableComponent)]
pub(crate) struct FakingTestSlave<SC>
where
    SC: Subcomponent<Member = FakingTestSlave<SC>>,
    FakingTestSlave<SC>:
        AHBSlavePortOutput<Component = SC::Component, Data = DataBus> + FakingHandler,
{
    #[subcomponent(pub(crate) SHandlerSC)]
    pub iface: TestIface<SC>,
    pub last_delivery_time: Option<u64>,
    pub response_iter: <Vec<WaitstatesOrErr> as IntoIterator>::IntoIter,
}

impl<SC> Default for FakingTestSlave<SC>
where
    SC: Subcomponent<Member = FakingTestSlave<SC>>,
    FakingTestSlave<SC>:
        AHBSlavePortOutput<Component = SC::Component, Data = DataBus> + FakingHandler,
{
    fn default() -> Self {
        Self {
            iface: Default::default(),
            last_delivery_time: None,
            response_iter: vec![].into_iter(),
        }
    }
}
impl<SC> FakingTestSlave<SC>
where
    SC: Subcomponent<Member = FakingTestSlave<SC>>,
    FakingTestSlave<SC>:
        AHBSlavePortOutput<Component = SC::Component, Data = DataBus> + FakingHandler,
{
    pub(crate) fn tick(comp: &mut SC::Component, ctx: &mut Context) {
        TestIface::<SC>::run_driver(comp, ctx);
    }
    pub(crate) fn tock(comp: &mut SC::Component, ctx: &mut Context) {
        TestIface::<SC>::tock(comp, ctx);
    }
}

impl<SC> FakingHandler for FakingTestSlave<SC>
where
    SC: Subcomponent<Member = FakingTestSlave<SC>>,
    FakingTestSlave<SC>: AHBSlavePortOutput<Component = SC::Component, Data = DataBus>,
{
    const WRITE_MODE: WriteMode = WriteMode::Combinatorial;

    fn pre_read(
        slave: &mut Self::Component,
        ctx: &mut Context,
        address: Address,
        size: Size,
    ) -> WaitstatesOrErr {
        SC::component_to_member_mut(slave)
            .response_iter
            .next()
            .expect("Iterator exhausted")
    }

    fn read(
        slave: &mut Self::Component,
        ctx: &mut Context,
        address: Address,
        size: Size,
    ) -> Self::Data {
        SC::component_to_member_mut(slave).last_delivery_time =
            Some(ctx.event_queue().get_current_time().as_picos());
        DataBus::Word(address.to_const())
    }

    fn pre_write(
        slave: &mut Self::Component,
        ctx: &mut Context,
        address: Address,
        size: Size,
    ) -> WaitstatesOrErr {
        SC::component_to_member_mut(slave)
            .response_iter
            .next()
            .expect("Iterator exhausted")
    }

    fn write(slave: &mut Self::Component, ctx: &mut Context, address: Address, data: Self::Data) {
        SC::component_to_member_mut(slave).last_delivery_time =
            Some(ctx.event_queue().get_current_time().as_picos());
    }
}

#[allow(unused_imports)]
use Option::None as na;

#[fixture]
fn component() -> TestComponent {
    Default::default()
}

#[fixture]
fn context() -> Context {
    Context::new_for_test()
}
#[allow(unused_variables)]
#[rstest]
#[should_panic]
fn missed_tock(mut context: Context, mut component: TestComponent) {
    let ctx = &mut context;
    component.tick(ctx);
    component.tick(ctx);
}

#[rstest]
fn noop_works(mut context: Context, mut component: TestComponent) {
    let ctx = &mut context;

    for _ in 0..10 {
        component.tick(ctx);
        component.tock(ctx);
    }
}

#[rstest]
#[test_log::test]
#[case::simple_read(
/* INJECT  */ auto_vec![na,         Ok(0),   na],
/* REQUEST */ auto_vec![read(0x10), idle(),  idle()],
/* HRDATA  */ auto_vec![HighZ,      HighZ,   HighZ],
/* RESPONS */ auto_vec![Success,    Success, Success],
/* DELIVERY*/ auto_vec![na,         15,      na],
)]
#[cfg_attr(
    all(feature = "paranoid", not(feature = "no_paranoid_override")),
    should_panic
)]
#[case::simple_error(
/* INJECT  */ auto_vec![na,         Err("ouch!"), na,     na   ],
/* REQUEST */ auto_vec![read(0x10), read(0x21),   idle(), idle()],
/* HRDATA  */ auto_vec![HighZ,      HighZ,        HighZ,  HighZ],
/* RESPONS */ auto_vec![Success,    Error1,       Error2, Success],
/* DELIVERY*/ auto_vec![na,         na,           na,     na],
)]
#[case::pipeline(
/* INJECT  */ auto_vec![na,          Ok(0),       Ok(0),      Ok(0),       Ok(0),      Ok(5)],
/* REQUEST */ auto_vec![write(0x10), write(0x20), read(0x10), write(0x22), read(0x20), idle()],
/* HRDATA  */ auto_vec![HighZ,       Word(0x1),   Word(0x2),  HighZ,       Word(0x2),  HighZ],
/* RESPONS */ auto_vec![Success,     Success,     Success,    Pending,     Success,    Pending],
/* DELIVERY*/ auto_vec![na,          20,          30,         35,          50,         na],
)]
#[case::read_with_waitstate_to_write(
// Note: master doesn't provide data for write unless it moves to data phase
/* INJECT  */ auto_vec![na,         Ok(1),       na,          Ok(0)],
/* REQUEST */ auto_vec![read(0x10), write(0x10), write(0x10), idle()],
/* HRDATA  */ auto_vec![HighZ,      HighZ,       HighZ,       Word(0x1)],
/* RESPONS */ auto_vec![Success,    Pending,     Success,     Success],
/* DELIVERY*/ auto_vec![na,         na,          25,          40],
)]
#[case::arm_ahb_fig_3_5(
// Note: master doesn't provide data for write unless it moves to data phase
/* INJECT  */ auto_vec![na,          Ok(0),      Ok(1),       na,          Ok(0)],
/* REQUEST */ auto_vec![write(0x10), read(0x20), write(0x20), write(0x20), idle()],
/* HRDATA  */ auto_vec![HighZ,       Word(0x1),  HighZ,       HighZ,       Word(0x2)],
/* RESPONS */ auto_vec![Success,     Success,    Pending,     Success,     Success],
/* DELIVERY*/ auto_vec![na,          20,         na,          35,          50],
)]
#[cfg_attr(
    all(feature = "paranoid", not(feature = "no_paranoid_override")),
    should_panic
)]
#[case::complex_test_with_everything(
// Note: master doesn't provide data for write unless it moves to data phase
/* INJECT  */ auto_vec![na,          Ok(0),      Ok(2),       na,          na,          Err("ouch!"), na,         Err("aaa"), na,         Ok(0),       Ok(0),     ],
/* REQUEST */ auto_vec![write(0x10), read(0x20), write(0x20), write(0x20), write(0x20), write(0x30),  read(0x20), read(0x10), read(0x10), write(0x10), idle(),    ],
/* HWDATA  */ auto_vec![HighZ,       Word(0x1),  HighZ,       HighZ,       HighZ,       Word(0x2),    Word(0x3),  HighZ,      HighZ,      HighZ,       Word(0x1), ],
/* RESPONS */ auto_vec![Success,     Success,    Pending,     Pending,     Success,     Error1,       Error2,     Error1,     Error2,     Success,     Success,   ],
/* DELIVERY*/ auto_vec![na,          20,         na,          na,          45,          na,           na,         na,         na,         95,          110],
)]
fn vec_test(
    #[case] injected_waitstates: Vec<Option<WaitstatesOrErr>>,
    #[case] requests_meta: Vec<MasterToSlaveAddrPhase>,
    #[case] requests_data: Vec<DataBus>,
    #[case] responses_meta: Vec<AhbResponseControl>,
    #[case] delivery_time: Vec<Option<u64>>,
    mut context: Context,
    mut component: TestComponent,
    #[values(true, false)] reorder: bool,
) {
    assert_eq!(
        requests_meta.len(),
        requests_data.len(),
        "Malformed test input"
    );
    assert_eq!(
        requests_meta.len(),
        responses_meta.len(),
        "Malformed test input"
    );
    assert_eq!(
        requests_meta.len(),
        delivery_time.len(),
        "Malformed test input"
    );

    component.s.response_iter = injected_waitstates
        .into_iter()
        .flatten()
        .collect_vec()
        .into_iter();

    let ctx = &mut context;
    let comp = &mut component;

    for (req, (req_data, (resp, delivery))) in
        zip!(requests_meta, requests_data, responses_meta, delivery_time)
    {
        println!("Cycle for {req:?} + {req_data:?} expecting {resp:?}");

        inc_time(ctx, 5);
        comp.tick(ctx);

        inc_time(ctx, 5);
        mix_blocks!(
            reorder,
            Master::send(comp, ctx, make_m2s(req, req_data.clone()), true),
            comp.tock(ctx)
        );

        assert_eq!(comp.s.last_delivery_time.take(), delivery);
        if delivery.is_some() && !req_data.is_present() {
            // This means the READ should be propagated
            assert!(comp.m.last_resp.take().unwrap().data.is_present());
        }
    }
}
