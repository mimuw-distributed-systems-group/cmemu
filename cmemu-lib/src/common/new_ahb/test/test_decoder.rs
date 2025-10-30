#![allow(non_upper_case_globals)]
#![allow(clippy::too_many_arguments)]
#![cfg_attr(not(feature = "cycle-debug-logger"), allow(unused_imports))]

use crate::auto_vec;
use crate::common::Address;
#[cfg(feature = "cycle-debug-logger")]
use crate::common::new_ahb::cdl::CdlTag;
use crate::common::new_ahb::decoder::*;
use crate::common::new_ahb::ports::*;
use crate::common::new_ahb::ports::{AHBPortConfig, AHBSlavePortInput, AHBSlavePortOutput};
use crate::common::new_ahb::signals::AhbResponseControl::{Error1, Error2, Success};
use crate::common::new_ahb::signals::{
    AhbResponseControl, MasterToSlaveAddrPhase, MasterToSlaveWires, SlaveToMasterWires,
    TransferMeta, TransferType,
};
use crate::common::new_ahb::slave_driver::SimpleResponse;
use crate::common::new_ahb::test::logging_ports::*;
use crate::common::new_ahb::test::utils::*;
use crate::common::new_ahb::vlan::AhbDecoderTag;
use crate::common::utils::FromMarker;
use crate::engine::{
    Context, DisableableComponent, Subcomponent, TickComponent, TickComponentExtra,
};
use crate::{bridge_ports, decoder_tags_and_markers, zip};
use cc2650_constants::{CPU_DWT, CPU_SCS};
use std::fmt::Debug;
use std::ops::Range;

pub struct PhonySC;

decoder_tags_and_markers!(@with_dispatcher
    pub enum ExSlaves {
        NVIC = CPU_SCS::ADDR_SPACE,
        DWT = CPU_DWT::ADDR_SPACE,
    }
);

decoder_tags_and_markers!(@with_dispatcher
    pub enum NoSlaves {}
);

impl Subcomponent for PhonySC {
    type Component = Decoder<PhonySC>;
    type Member = Decoder<PhonySC>;

    fn component_to_member(component: &Self::Component) -> &Self::Member {
        component
    }

    fn component_to_member_mut(component: &mut Self::Component) -> &mut Self::Member {
        component
    }
}

impl AHBPortConfig for Decoder<PhonySC> {
    type Data = Option<ExSlaves>;
    type Component = Decoder<PhonySC>;
    const TAG: &'static str = "input of decoder";
}

impl AhbDecoderTag for Decoder<PhonySC> {
    type Enum = Option<ExSlaves>;
}

impl AHBSlavePortOutput for Decoder<PhonySC>
where
    Self::Data: Debug,
{
    fn send_ahb_output(
        comp: &mut Self::Component,
        ctx: &mut Context,
        msg: SlaveToMasterWires<Self::Data>,
    ) {
        println!("Decoder got response{msg:?}");
    }
}

macro_rules! dummy_output {
    ($mark:ident) => {
        impl AHBMasterPortOutput for AhbPort<PhonySC, $mark>
        where
            PhonySC: Subcomponent<Member = Decoder<PhonySC>, Component = Decoder<PhonySC>>,
            Decoder<PhonySC>: AhbDecoderTag,
            Decoder<PhonySC>: AHBSlavePortOutput<Component = Decoder<PhonySC>>,
            <Decoder<PhonySC> as AhbDecoderTag>::Enum: FromMarker<$mark>,
            Self::Data: Debug,
        {
            fn send_ahb_output(
                comp: &mut Self::Component,
                ctx: &mut Context,
                msg: MasterToSlaveWires<Self::Data>,
            ) {
                println!("Slave {} got {:?}", stringify!($mark), msg);
                // if let Some(addr) = msg.addr_phase.meta.address() {
                //     assert!(addr.is_in_range(&$mark::ROUTING_RANGE))
                // }
                if let Some(data) = msg.data_phase.data {
                    assert_eq!(data, $mark.into());
                }
            }
        }
    };
}
dummy_output!(NVIC);
dummy_output!(DWT);

use AhbResponseControl::*;
use TestSlaves::{SlaveA as A, SlaveB as B};
use rstest::*;

type DecoderComponent = Decoder<PhonySC>;
impl DecoderComponent {
    fn tick(&mut self, ctx: &mut Context) {
        #[cfg(debug_assertions)]
        self.tick_assertions_traverse();
        self.tick_flops_and_extra_traverse();
        Self::sub_tick(self, ctx);
    }

    fn tock_(&mut self, ctx: &mut Context) {
        Self::tock(self, ctx);
    }
}
#[fixture]
fn component() -> DecoderComponent {
    DecoderComponent::default()
}

#[fixture]
fn context() -> Context {
    // not actually used anywhere.
    Context::new_for_test()
}

#[allow(unused_variables)]
#[rstest]
#[should_panic]
fn missed_tock(mut context: Context, mut component: DecoderComponent) {
    let ctx = &mut context;
    component.tick(ctx);
    component.tick(ctx);
    component.tick(ctx);
}

#[rstest]
#[case::valid_data_tag(None)]
// #[should_panic]
// #[case::invalid_data_tag(Some(ExSlaves::NVIC))]
fn test_basic_correctness(
    #[case] data: Option<ExSlaves>,
    mut context: Context,
    mut component: DecoderComponent,
) {
    let comp = &mut component;
    let ctx = &mut context;

    comp.tick(ctx);
    comp.tock_(ctx);
    <Decoder<PhonySC> as AHBSlavePortInput>::on_ahb_input(comp, ctx, make_m2s(write(0x0), None));
    comp.tick(ctx);
    comp.tock_(ctx);
    <Decoder<PhonySC> as AHBSlavePortInput>::on_ahb_input(
        comp,
        ctx,
        make_m2s(
            MasterToSlaveAddrPhase {
                ready: false,
                ..TransferType::Idle.into()
            },
            data,
        ),
    );
    comp.tick(ctx);
    comp.tock_(ctx);
    <Decoder<PhonySC> as AHBSlavePortInput>::on_ahb_input(
        comp,
        ctx,
        make_m2s(write(CPU_DWT::COMP0::ADDR.to_const()), None),
    );
}

// // Extended test
const A_RANGE: Range<Address> = Address::range_from_len(0x10, 0x10);
const B_RANGE: Range<Address> = Address::range_from_len(0x20, 0x20);
decoder_tags_and_markers!(@with_dispatcher
    pub(crate) enum TestSlaves {
        SlaveA = A_RANGE,
        SlaveB = B_RANGE,
    }
);
// We pass slave tag as data to easily verify that destination is valid
type TestData = Option<TestSlaves>;

#[derive(Subcomponent, TickComponent, TickComponentExtra, DisableableComponent)]
struct TestComponent {
    #[subcomponent(DecoderSC)]
    decoder: Decoder<DecoderSC>,
    last_response: Option<SlaveToMasterWires<TestData>>,
    #[subcomponent(SlaveASC)]
    slave_a: TestSlave<SlaveASC>,
    #[subcomponent(SlaveBSC)]
    slave_b: TestSlave<SlaveBSC>,
}

impl AHBPortConfig for Decoder<DecoderSC>
where
    Self: AhbDecoderTag,
{
    type Data = TestData;
    type Component = TestComponent;
    const TAG: &'static str = "Decoder";
}

impl AhbDecoderTag for Decoder<DecoderSC>
where
    DecoderSC: Subcomponent<Member = Self>,
{
    type Enum = Option<TestSlaves>;
}

impl AHBSlavePortOutput for Decoder<DecoderSC>
where
    Self::Data: Debug,
{
    fn send_ahb_output(
        comp: &mut Self::Component,
        ctx: &mut Context,
        msg: SlaveToMasterWires<Self::Data>,
    ) {
        println!("Decoder got response{msg:?}");
        comp.last_response = Some(msg);
    }
}

#[fixture]
fn test_component() -> TestComponent {
    TestComponent {
        decoder: Default::default(),
        last_response: None,
        slave_a: TestSlave::<SlaveASC>::default(),
        slave_b: TestSlave::<SlaveBSC>::default(),
    }
}

impl TestComponent {
    fn tick(&mut self, ctx: &mut Context) {
        #[cfg(debug_assertions)]
        self.tick_assertions_traverse();
        self.tick_flops_and_extra_traverse();
        Decoder::<DecoderSC>::sub_tick(self, ctx);
        TestSlave::<SlaveASC>::tick(self, ctx);
        TestSlave::<SlaveBSC>::tick(self, ctx);
    }

    fn tock(&mut self, ctx: &mut Context) {
        Decoder::<DecoderSC>::tock(self, ctx);
        TestSlave::<SlaveASC>::tock(self, ctx);
        TestSlave::<SlaveBSC>::tock(self, ctx);
    }
}

bridge_ports!(AhbPort<DecoderSC, SlaveA> => auto_configured TestSlave<SlaveASC>);
bridge_ports!(AhbPort<DecoderSC, SlaveB> => auto_configured TestSlave<SlaveBSC>);

trait TMarker {
    type M;
    const TAG: TestSlaves;
}

impl<T> Checker for T
where
    T: TMarker,
{
    type Data = TestData;

    fn check_input(msg: &MasterToSlaveWires<Self::Data>) {
        #[cfg(not(feature = "cycle-debug-logger"))]
        println!("Slave with tag {:?} got {:?}", Self::TAG, msg);
        if let Some(addr) = msg.addr_phase.meta.address() {
            assert_eq!(Option::<TestSlaves>::decode(addr), Some(Self::TAG));
        }
        if let Some(data) = msg.data_phase.data {
            assert_eq!(data, Self::TAG);
        }
    }
    fn check_and_reply_read(request: &TransferMeta) -> Self::Data {
        Some(Self::TAG)
    }
    fn check_pre_write(request: &TransferMeta) {}
    fn check_write(request: &TransferMeta, data: &Self::Data, post_success: bool) {
        assert_eq!(data, &Some(Self::TAG));
    }
}

impl TMarker for SlaveASC {
    type M = SlaveA;
    const TAG: TestSlaves = TestSlaves::SlaveA;
}

impl TMarker for SlaveBSC {
    type M = SlaveB;
    const TAG: TestSlaves = TestSlaves::SlaveB;
}

#[allow(unused_variables)]
#[rstest]
fn no_op(mut context: Context, mut test_component: TestComponent) {
    let ctx = &mut context;
    let comp = &mut test_component;
    for _ in 0..10 {
        comp.tick(ctx);
        comp.tock(ctx);
        <Decoder<DecoderSC> as AHBSlavePortInput>::on_ahb_input(
            comp,
            ctx,
            make_m2s(TransferType::Idle.into(), None),
        );
        let resp = comp.last_response.take().unwrap();
        assert_eq!(resp.meta, Success);
        #[cfg(feature = "cycle-debug-logger")]
        assert_eq!(resp.responder_tag, AhbPort::<DecoderSC, DefaultSlave>::TAG);
    }
}

#[rstest]
#[test_log::test]
fn round_robin_read(mut context: Context, mut test_component: TestComponent) {
    let ctx = &mut context;
    let comp = &mut test_component;
    for i in 0..10 {
        println!("Cycle {i}!");
        comp.tick(ctx);
        comp.tock(ctx);
        // reflect
        let msg = reflect_hready(
            make_m2s(read(i * 8), None),
            comp.last_response.take().as_ref(),
        );
        <Decoder<DecoderSC> as AHBSlavePortInput>::on_ahb_input(comp, ctx, msg);
    }
}

#[rstest]
fn round_robin_write(mut context: Context, mut test_component: TestComponent) {
    let ctx = &mut context;
    let comp = &mut test_component;
    for i in 0..15 {
        let i = i % 10 + 1;
        let msg = make_m2s(
            write(i * 8),
            Option::<TestSlaves>::decode(Address::from_const((i - 1) * 8)),
        );
        println!("Cycle {i}!");
        comp.tick(ctx);
        comp.tock(ctx);

        // reflect
        let resp = comp.last_response.take();
        <Decoder<DecoderSC> as AHBSlavePortInput>::on_ahb_input(
            comp,
            ctx,
            reflect_hready(msg, resp.as_ref()),
        );
    }
}

const A_TAG: &str = TestSlave::<SlaveASC>::TAG;
const B_TAG: &str = TestSlave::<SlaveBSC>::TAG;
const D_TAG: &str = AhbPort::<DecoderSC, DefaultSlave>::TAG;

const Au32: u32 = A_RANGE.start.to_const();
const Bu32: u32 = B_RANGE.start.to_const();
const Du32: u32 = 0;
const D: Option<TestSlaves> = None;

#[cfg(feature = "cycle-debug-logger")]
#[rstest]
fn switching_test(mut context: Context, mut test_component: TestComponent) {
    let ctx = &mut context;
    let comp = &mut test_component;

    comp.tick(ctx);
    comp.tock(ctx);
    <Decoder<DecoderSC> as AHBSlavePortInput>::on_ahb_input(
        comp,
        ctx,
        reflect_hready(make_m2s(read(Au32), None), comp.last_response.as_ref()),
    );
    let resp = comp.last_response.take().unwrap();
    assert_eq!(resp.meta, Success);
    assert_eq!(resp.responder_tag, D_TAG);

    println!("Tick 2");
    comp.tick(ctx);
    assert!(comp.slave_a.delivered_read_req.take().is_some());

    comp.tock(ctx);
    <Decoder<DecoderSC> as AHBSlavePortInput>::on_ahb_input(
        comp,
        ctx,
        reflect_hready(make_m2s(write(Bu32), None), comp.last_response.as_ref()),
    );
    let resp = comp.last_response.take().unwrap();
    assert_eq!(resp.meta, Success);
    assert_eq!(resp.responder_tag, A_TAG);
    assert_eq!(resp.data, A.into());

    println!("Tick 3");
    comp.tick(ctx);
    assert!(comp.slave_a.delivered_read_req.take().is_none());
    assert!(comp.slave_b.delivered_read_req.take().is_none());
    assert!(comp.slave_b.delivered_pre_write_req.take().is_some());

    comp.tock(ctx);
    <Decoder<DecoderSC> as AHBSlavePortInput>::on_ahb_input(
        comp,
        ctx,
        reflect_hready(make_m2s(write(Du32), B.into()), comp.last_response.as_ref()),
    );
    let resp = comp.last_response.take().unwrap();
    assert_eq!(resp.meta, Success);
    assert_eq!(resp.responder_tag, B_TAG);
    assert_eq!(resp.data, None);

    println!("Tick 4");
    comp.tick(ctx);
    assert!(comp.slave_a.delivered_read_req.take().is_none());
    assert!(comp.slave_b.delivered_write_req.take().is_some());
    assert_eq!(comp.slave_b.delivered_write_data.take().unwrap(), Some(B));

    comp.tock(ctx);
    <Decoder<DecoderSC> as AHBSlavePortInput>::on_ahb_input(
        comp,
        ctx,
        reflect_hready(make_m2s(write(Bu32), D), comp.last_response.as_ref()),
    );
    let resp = comp.last_response.take().unwrap();
    assert_eq!(resp.meta, Error1);
    assert_eq!(resp.responder_tag, D_TAG);

    println!("Tick 5");
    comp.tick(ctx);
    assert!(comp.slave_a.delivered_read_req.take().is_none());
    assert!(comp.slave_b.delivered_write_req.take().is_none());
    // HREADY should be false, because waitstate on default
    assert!(comp.slave_b.delivered_pre_write_req.take().is_none());

    comp.tock(ctx);
    <Decoder<DecoderSC> as AHBSlavePortInput>::on_ahb_input(
        comp,
        ctx,
        reflect_hready(make_m2s(write(Bu32), None), comp.last_response.as_ref()),
    );
    let resp = comp.last_response.take().unwrap();
    assert_eq!(resp.meta, Error2);
    assert_eq!(resp.responder_tag, D_TAG);

    println!("Tick 6");
    comp.tick(ctx);
    assert!(comp.slave_a.delivered_read_req.take().is_none());
    assert!(comp.slave_b.delivered_write_req.take().is_none());
    assert!(comp.slave_b.delivered_pre_write_req.take().is_some());

    comp.tock(ctx);
    <Decoder<DecoderSC> as AHBSlavePortInput>::on_ahb_input(
        comp,
        ctx,
        reflect_hready(make_m2s(idle(), B.into()), comp.last_response.as_ref()),
    );
    let resp = comp.last_response.take().unwrap();
    assert_eq!(resp.meta, Success);
    assert_eq!(resp.responder_tag, B_TAG);
}

#[cfg(feature = "cycle-debug-logger")]
#[rstest]
#[case::easy_with_idles(
    /* INJECTA */ auto_vec![             SrSuccess,  SrSuccess,],
    /* INJECTB */ auto_vec![],
    /* REQUEST */ auto_vec![write(Au32), read(Au32), idle(),  idle(),  ],
    /* HWDATA  */ auto_vec![None,        A,          None,    None,    ],
    /* RESPONS */ auto_vec![Success,     Success,    Success, Success, ],
    /* HRDATA  */ auto_vec![None,        None,       A,       None,    ],
    /* DST TAG */ auto_vec![D_TAG,       A_TAG,      A_TAG,   A_TAG,   ],
    )]
#[case::change_slave_on_error(
    /* INJECTA */ auto_vec![],
    /* INJECTB */ auto_vec![                                       SrSuccess],
    /* REQUEST */ auto_vec![write(Du32), write(Au32), write(Bu32), idle(),  ],
    /* HWDATA  */ auto_vec![None,        D,           None,        B,       ],
    /* RESPONS */ auto_vec![Success,     Error1,      Error2,      Success, ],
    /* HRDATA  */ auto_vec![None,        None,        None,        None,    ],
    /* DST TAG */ auto_vec![D_TAG,       D_TAG,       D_TAG,       B_TAG,   ],
    )]
#[case::change_idle_on_waitstate(
    /* INJECTA */ auto_vec![             SrPending, SrSuccess],
    /* INJECTB */ auto_vec![                                   SrSuccess],
    /* REQUEST */ auto_vec![write(Au32), idle(),  write(Bu32), idle(),  ],
    /* HWDATA  */ auto_vec![None,        A,       A,           B,       ],
    /* RESPONS */ auto_vec![Success,     Pending, Success,     Success, ],
    /* HRDATA  */ auto_vec![None,        None,    None,        None,    ],
    /* DST TAG */ auto_vec![D_TAG,       A_TAG,   A_TAG,       B_TAG,   ],
    )]
#[case::optimal_pipeline(
    /* INJECTA */ auto_vec![             SrSuccess,                SrSuccess,],
    /* INJECTB */ auto_vec![                          SrSuccess,               SrSuccess],
    /* REQUEST */ auto_vec![write(Au32), write(Bu32), write(Au32), read(Bu32), idle(),  ],
    /* HWDATA  */ auto_vec![None,        A,           B,           A,          None],
    /* RESPONS */ auto_vec![Success,     Success,     Success,     Success,    Success],
    /* HRDATA  */ auto_vec![None,        None,        None,        None,       B],
    /* DST TAG */ auto_vec![D_TAG,       A_TAG,       B_TAG,       A_TAG,      B_TAG],
    )]
#[case::keep_route(
    /* INJECTA */ auto_vec![             SrSuccess,],
    /* INJECTB */ auto_vec![],
    /* REQUEST */ auto_vec![write(Au32), idle(),  read(Du32), idle(), idle(), idle() ],
    /* HWDATA  */ auto_vec![None,        A,       None,       None,   None,   None,     ],
    /* RESPONS */ auto_vec![Success,     Success, Success,    Error1, Error2, Success],
    /* HRDATA  */ auto_vec![None,        None,    None,       None,   None,   None],
    /* DST TAG */ auto_vec![D_TAG,       A_TAG,   A_TAG,      D_TAG,  D_TAG,  D_TAG,    ],
    )]
#[case::waitstate(
    /* INJECTA */ auto_vec![             SrPending,   SrSuccess,                           SrSuccess],
    /* INJECTB */ auto_vec![                                       SrPending,  SrSuccess],
    /* REQUEST */ auto_vec![write(Au32), write(Bu32), write(Bu32), read(Au32), read(Au32), idle() ],
    /* HWDATA  */ auto_vec![None,        A,           A,           B,          B,          None   ],
    /* RESPONS */ auto_vec![Success,     Pending,     Success,     Pending,    Success,    Success],
    /* HRDATA  */ auto_vec![None,        None,        None,        None,       None,       A],
    /* DST TAG */ auto_vec![D_TAG,       A_TAG,       A_TAG,       B_TAG,      B_TAG,      A_TAG],
    )]
#[case::cortexm3_uncompat_change_on_waitstate(
    /* INJECTA */ auto_vec![            SrPending,  SrSuccess,  SrPending,  SrSuccess],
    /* INJECTB */ auto_vec![                                                            SrSuccess],
    /* REQUEST */ auto_vec![read(Au32), read(Bu32), read(Au32), read(Au32), read(Bu32), idle() ],
    /* HWDATA  */ auto_vec![None,       None,       None,       None,       None,       None   ],
    /* RESPONS */ auto_vec![Success,    Pending,    Success,    Pending,    Success,    Success],
    /* HRDATA  */ auto_vec![None,       None,       A,          None,       A,          B],
    /* DST TAG */ auto_vec![D_TAG,      A_TAG,      A_TAG,      A_TAG,      A_TAG,      B_TAG],
    )]
#[case::complex_test_with_everything(
    /* INJECTA */ auto_vec![             SrSuccess,                                                      SrPending, SrError,                      SrSuccess],
    /* INJECTB */ auto_vec![                         SrPending,   SrPending,   SrError,],
    /* REQUEST */ auto_vec![write(Au32), read(Bu32), write(Bu32), write(Du32), write(Au32), write(Au32), idle(),  read(Du32), idle(), read(Au32), idle()],
    /* HWDATA  */ auto_vec![None,        A,          None,        None,        None,        None,        A,       A,          None,   None,       None],
    /* RESPONS */ auto_vec![Success,     Success,    Pending,     Pending,     Error1,      Error2,      Pending, Error1,     Error2, Success,    Success],
    /* HRDATA  */ auto_vec![None,        None,       None,        None,        None,        None,        None,    None,       None,   None,       A],
    /* SRC TAG */ auto_vec![D_TAG,       A_TAG,      B_TAG,       B_TAG,       B_TAG,       B_TAG,       A_TAG,   A_TAG,      A_TAG,  A_TAG,      A_TAG],
    )]
fn test_interleaving(
    #[case] injected_responses_a: Vec<SimpleResponse<()>>,
    #[case] injected_responses_b: Vec<SimpleResponse<()>>,
    #[case] requests_meta: Vec<MasterToSlaveAddrPhase>,
    #[case] requests_data: Vec<TestData>,
    #[case] responses_meta: Vec<AhbResponseControl>,
    #[case] responses_data: Vec<TestData>,
    #[case] responses_tag: Vec<CdlTag>,
    mut context: Context,
    mut test_component: TestComponent,
    #[values(true, false)] reorder: bool,
) {
    assert_eq!(
        requests_meta.len(),
        requests_data.len(),
        "Malformed test input"
    );
    assert_eq!(
        requests_meta.len(),
        responses_data.len(),
        "Malformed test input"
    );
    assert_eq!(
        responses_meta.len(),
        responses_data.len(),
        "Malformed test input"
    );
    assert_eq!(
        requests_meta.len(),
        responses_tag.len(),
        "Malformed test input"
    );

    test_component.slave_a.response_iter = Some(injected_responses_a.into_iter());
    test_component.slave_b.response_iter = Some(injected_responses_b.into_iter());
    let ctx = &mut context;
    let comp = &mut test_component;

    comp.tick(ctx);
    for (req, (req_data, (resp, (resp_data, rtag)))) in zip!(
        requests_meta,
        requests_data,
        responses_meta,
        responses_data,
        responses_tag
    ) {
        let m2s = make_m2s(req.clone(), req_data);
        let s2m = SlaveToMasterWires {
            meta: resp,
            data: resp_data,
            responder_tag: rtag,
            // first sender tag is from unknown
            sender_tag: "*".into(),
        };

        println!("\nCycle for {m2s:?} resp {s2m:?}");

        comp.tock(ctx);
        let m2s = reflect_hready(m2s, comp.last_response.as_ref());
        <Decoder<DecoderSC> as AHBSlavePortInput>::on_ahb_input(comp, ctx, m2s);

        assert_eq!(comp.last_response.take().unwrap(), s2m);

        comp.tick(ctx);
    }
}
