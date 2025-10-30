#![allow(clippy::too_many_arguments)]
#![allow(non_upper_case_globals)]

#[cfg(feature = "cycle-debug-logger")]
use crate::common::new_ahb::cdl::CdlTag;

use crate::common::new_ahb::ports::*;
use crate::common::new_ahb::ports::{AHBPortConfig, AHBSlavePortInput, AHBSlavePortOutput};
use crate::common::new_ahb::signals::AhbResponseControl::{Pending, Success};
#[cfg_attr(not(feature = "cycle-debug-logger"), allow(unused_imports))]
use crate::common::new_ahb::signals::{
    AhbResponseControl, MasterToSlaveAddrPhase, MasterToSlaveDataPhase, MasterToSlaveWires,
    SlaveToMasterWires, TransferMeta,
};
use crate::common::new_ahb::slave_driver::SimpleResponse;
use crate::common::new_ahb::test::logging_ports::*;
use crate::common::new_ahb::test::utils::*;
use crate::engine::{
    Context, DisableableComponent, Subcomponent, TickComponent, TickComponentExtra,
};
use crate::{mix_blocks, zip};
use rstest::*;

pub struct PhonySC;
make_port_struct!(AhbPort<SC, PM>);
type Port = TestSlave<SlaveSC>;

#[derive(Subcomponent, TickComponent, TickComponentExtra, DisableableComponent, Default)]
struct TestComponent {
    #[subcomponent(SlaveSC)]
    s: TestSlave<SlaveSC>,
    last_response: Option<SlaveToMasterWires<DataBus>>,
}

impl Checker for SlaveSC {
    type Data = DataBus;

    fn check_and_reply_read(request: &TransferMeta) -> Self::Data {
        DataBus::Word(request.addr.to_const())
    }
}

impl AHBPortConfig for Port {
    type Data = DataBus;
    type Component = TestComponent;
    const TAG: &'static str = "TestPort";
}

impl AHBSlavePortOutput for Port {
    fn send_ahb_output(
        comp: &mut Self::Component,
        _ctx: &mut Context,
        msg: SlaveToMasterWires<Self::Data>,
    ) {
        comp.last_response = Some(msg);
    }
}

impl TestComponent {
    fn tick(&mut self, ctx: &mut Context) {
        #[cfg(debug_assertions)]
        self.tick_assertions_traverse();
        self.tick_flops_and_extra_traverse();
        Port::tick(self, ctx);
    }

    fn tock(&mut self, ctx: &mut Context) {
        Port::tock(self, ctx);
    }
}

#[fixture]
fn component() -> TestComponent {
    Default::default()
}

#[fixture]
fn context() -> Context {
    // not actually used anywhere.
    Context::new_for_test()
}

#[allow(unused_variables)]
#[rstest]
fn do_nothing(context: Context, component: TestComponent) {}

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

fn address_eq(r: TransferMeta, addr: u32) -> bool {
    r.addr.to_const() == addr
}
#[rstest]
fn simple_pipelined_reads(
    mut context: Context,
    mut component: TestComponent,
    #[values(true, false)] reorder: bool,
) {
    let ctx = &mut context;
    let comp = &mut component;

    for it in 0..10 {
        comp.tick(ctx);
        assert!(it == 0 || address_eq(comp.s.delivered_read_req.take().unwrap(), 0x20 + it - 1));

        // order of tock is not determined
        mix_blocks! {reorder, {
            Port::on_ahb_input(comp, ctx, make_m2s(read(0x20 + it), DataBus::HighZ));
            assert!(comp.s.delivered_read_req.take().is_none());
            }, {
            comp.tock(ctx);}
        }
        if it > 0 {
            let resp = comp.last_response.take().unwrap();
            assert!(resp.meta.is_done());
            assert!(resp.data.raw() == 0x20 + it - 1);
        }
    }
}

#[rstest]
fn simple_pipelined_comb_writes(
    mut context: Context,
    mut component: TestComponent,
    #[values(true, false)] reorder: bool,
) {
    let ctx = &mut context;
    let comp = &mut component;

    comp.tick(ctx);
    Port::on_ahb_input(comp, ctx, make_m2s(write(0x20 - 1), DataBus::HighZ));
    comp.tock(ctx);
    assert!(comp.s.delivered_pre_write_req.is_none());
    assert!(comp.s.delivered_write_req.take().is_none());
    assert!(comp.last_response.take().unwrap().meta.is_done());

    for it in 0..10 {
        println!("Tick {it}");
        comp.tick(ctx);
        assert!(address_eq(
            comp.s.delivered_pre_write_req.take().unwrap(),
            0x20 + it - 1
        ));
        // Response not yet sent, data not delivered in this phase
        assert!(comp.last_response.take().is_none());
        assert!(comp.s.delivered_write_req.take().is_none());

        println!("Tock {it}");
        // order of tock is not determined
        mix_blocks! {reorder, {
            comp.tock(ctx);
        }, {
            Port::on_ahb_input(comp, ctx, make_m2s(write(0x20 + it), DataBus::Word(it)));
        }}

        // Address if from previous cycle, data from current.
        assert!(address_eq(
            comp.s.delivered_write_req.take().unwrap(),
            0x20 + it - 1
        ));
        assert!(comp.s.delivered_write_data.take().unwrap().raw() == it);
        assert!(comp.last_response.take().unwrap().meta.is_done());
    }
}

use crate::common::new_ahb::databus::DataBus;
use crate::common::new_ahb::databus::DataBus::*;
#[allow(unused_imports)] // It is used, but some tools don't see that.
use crate::{atom_vec, auto_vec};
use AhbResponseControl::*;

#[rstest]
#[case::simple_read(
/* REQUEST */ auto_vec![read(0x10), idle(),     idle()],
/* HWDATA  */ auto_vec![HighZ,      HighZ,      HighZ],
/* RESPONS */ auto_vec![Success,    Success,    Success],
/* HRDATA  */ auto_vec![HighZ,      Word(0x10), HighZ],
)]
#[case::simple_write(
/* REQUEST */ auto_vec![write(0x10), idle(),    idle()],
/* HWDATA  */ auto_vec![HighZ,       Word(0x1), HighZ],
/* RESPONS */ auto_vec![Success,     Success,   Success],
/* HRDATA  */ auto_vec![HighZ,       HighZ,     HighZ],
)]
#[case::write_after_write_same_addr(
/* REQUEST */ auto_vec![write(0x10), write(0x10), idle(),    idle()],
/* HWDATA  */ auto_vec![HighZ,       Word(0x1),   Word(0x2), HighZ],
/* RESPONS */ auto_vec![Success,     Success,     Success,   Success],
/* HRDATA  */ auto_vec![HighZ,       HighZ,       HighZ,     HighZ],
)]
#[case::write_after_write_different_addr(
/* REQUEST */ auto_vec![write(0x10), write(0x20), idle(),   idle()],
/* HWDATA  */ auto_vec![HighZ,       Word(0x1),   Word(0x2), HighZ],
/* RESPONS */ auto_vec![Success,     Success,     Success,   Success],
/* HRDATA  */ auto_vec![HighZ,       HighZ,       HighZ,     HighZ],
)]
#[case::write_after_read_different_addr(
/* REQUEST */ auto_vec![read(0x10), write(0x02), idle(),    idle()],
/* HWDATA  */ auto_vec![HighZ,      HighZ,       Word(0x2), HighZ],
/* RESPONS */ auto_vec![Success,    Success,     Success,   Success],
/* HRDATA  */ auto_vec![HighZ,      Word(0x10),  HighZ,     HighZ],
)]
#[case::read_after_write_different_addr(
/* REQUEST */ auto_vec![write(0x10), read(0x20), idle(),     idle()],
/* HWDATA  */ auto_vec![HighZ,       Word(0x2),  HighZ,      HighZ],
/* RESPONS */ auto_vec![Success,     Success,    Success,    Success],
/* HRDATA  */ auto_vec![HighZ,       HighZ,      Word(0x20), HighZ],
)]
#[case::read_after_write_same_addr(
/* REQUEST */ auto_vec![write(0x10), read(0x10), idle(),     idle()],
/* HWDATA  */ auto_vec![HighZ,       Word(0x2),  HighZ,      HighZ],
/* RESPONS */ auto_vec![Success,     Success,    Success,    Success],
/* HRDATA  */ auto_vec![HighZ,       HighZ,      Word(0x10), HighZ],
)]
#[case::complex_with_idles(
/* REQUEST */ auto_vec![write(0x10), idle(),    read(0x10), write(0x20), idle()],
/* HWDATA  */ auto_vec![HighZ,       Word(0x2), HighZ,      HighZ,       Word(0x3)],
/* RESPONS */ auto_vec![Success,     Success,   Success,    Success,     Success],
/* HRDATA  */ auto_vec![HighZ,       HighZ,     HighZ,      Word(0x10),  HighZ],
)]
#[case::complex_with_idles2(
/* REQUEST */ auto_vec![read(0x10), idle(),     write(0x10), read(0x20), write(0x20), idle()],
/* HWDATA  */ auto_vec![HighZ,      HighZ,      HighZ,       Word(0x2),  HighZ,       Word(0x2)],
/* RESPONS */ auto_vec![Success,    Success,    Success,     Success,    Success,     Success],
/* HRDATA  */ auto_vec![HighZ,      Word(0x10), HighZ,       HighZ,      Word(0x20),  HighZ],
)]
#[should_panic]
#[case::no_data_for_write(
/* REQUEST */ auto_vec![write(0x10), read(0x10), idle()],
/* HWDATA  */ auto_vec![HighZ,       HighZ,      Word(0x2)],
/* RESPONS */ auto_vec![Success,     Success,    Success],
/* HRDATA  */ auto_vec![HighZ,       HighZ,      Word(0x10)],
)]
#[should_panic]
#[case::unexpected_data_for_read(
/* REQUEST */ auto_vec![write(0x10), read(0x10), idle()],
/* HWDATA  */ auto_vec![HighZ,       Word(0x2),  Word(0x2)],
/* RESPONS */ auto_vec![Success,     Success,    Success],
/* HRDATA  */ auto_vec![HighZ,       HighZ,      Word(0x10)],
)]
fn vec_test(
    #[case] requests_meta: Vec<MasterToSlaveAddrPhase>,
    #[case] requests_data: Vec<DataBus>,
    #[case] responses_meta: Vec<AhbResponseControl>,
    #[case] responses_data: Vec<DataBus>,
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
        responses_data.len(),
        "Malformed test input"
    );
    assert_eq!(
        responses_meta.len(),
        responses_data.len(),
        "Malformed test input"
    );

    let ctx = &mut context;
    let comp = &mut component;

    comp.tick(ctx);
    let mut prev_req = MasterToSlaveAddrPhase::default();
    for (req, (req_data, (resp, resp_data))) in
        zip!(requests_meta, requests_data, responses_meta, responses_data)
    {
        println!("Cycle for {req:?} resp {resp:?}");

        mix_blocks! {reorder, {
            comp.tock(ctx);
        }, {
            Port::on_ahb_input(comp, ctx, make_m2s(req.clone(), req_data.clone()));
        }}
        let their_resp = comp.last_response.take().unwrap();
        assert_eq!(their_resp.meta, resp);
        assert_eq!(their_resp.data, resp_data);

        if req_data.is_present() {
            assert_eq!(
                comp.s.delivered_write_req.take().as_ref(),
                prev_req.meta.meta()
            );
            assert_eq!(comp.s.delivered_write_data.take().unwrap(), req_data);
        }

        comp.tick(ctx);
        // Only makes sense with no waitstates
        if req.meta.is_idle() {
            assert!(comp.s.delivered_pre_write_req.take().is_none());
            assert!(comp.s.delivered_read_req.take().is_none());
        } else if req.meta.is_writing() {
            assert_eq!(
                comp.s.delivered_pre_write_req.take().unwrap(),
                *req.meta.meta().unwrap()
            );
        } else {
            assert_eq!(
                comp.s.delivered_read_req.take().unwrap(),
                *req.meta.meta().unwrap()
            );
        }

        // No double delivery (comb write delivers in tock)
        assert!(comp.s.delivered_write_req.is_none());

        assert!(comp.s.delivered_write_data.is_none());

        prev_req = req;
    }
}

#[rstest]
#[case::simple_read(
/* INJECT  */ auto_vec![            SrSuccess,  SrSuccess],
/* REQUEST */ auto_vec![read(0x10), idle(),     idle()],
/* HRDATA  */ auto_vec![HighZ,      HighZ,      HighZ],
/* RESPONS */ auto_vec![Success,    Success,    Success],
/* HRDATA  */ auto_vec![HighZ,      Word(0x10), HighZ],
)]
#[case::simple_error(
/* INJECT  */ auto_vec![            SrError,    ],
/* REQUEST */ auto_vec![read(0x10), read(0x21), idle(), idle()],
/* HRDATA  */ auto_vec![HighZ,      HighZ,      HighZ,  HighZ],
/* RESPONS */ auto_vec![Success,    Error1,     Error2, Success],
/* HRDATA  */ auto_vec![HighZ,      HighZ,      HighZ,  HighZ],
)]
#[case::read_with_recovery(
// we did not deliver any event, as the iterator ( \/ here ) is noy exhausted
/* INJECT  */ auto_vec![            SrError,                SrSuccess],
/* REQUEST */ auto_vec![read(0x10), read(0x21), read(0x30), idle()],
/* HRDATA  */ auto_vec![HighZ,      HighZ,      HighZ,      HighZ],
/* RESPONS */ auto_vec![Success,    Error1,     Error2,     Success],
/* HRDATA  */ auto_vec![HighZ,      HighZ,      HighZ,      Word(0x30)],
)]
#[case::write_change_to_read(
// we did not deliver any event, as the iterator ( \/ here ) is noy exhausted
/* INJECT  */ auto_vec![             SrError,                 SrSuccess],
/* REQUEST */ auto_vec![write(0x10), write(0x11), read(0x30), idle()],
/* HRDATA  */ auto_vec![HighZ,       HighZ,       HighZ,      HighZ],
/* RESPONS */ auto_vec![Success,     Error1,      Error2,     Success],
/* HRDATA  */ auto_vec![HighZ,       HighZ,       HighZ,      Word(0x30)],
)]
#[case::read_with_waitstates_incomaptibile(
// here we change address during waitstate -> this is Cortex M3/4 AMBA incompatibility
/* INJECT  */ auto_vec![            SrPending,  SrSuccess,  SrSuccess],
/* REQUEST */ auto_vec![read(0x10), read(0x11), read(0x30), idle()],
/* HRDATA  */ auto_vec![HighZ,      HighZ,      HighZ,      HighZ],
/* RESPONS */ auto_vec![Success,    Pending,    Success,    Success],
/* HRDATA  */ auto_vec![HighZ,      HighZ,      Word(0x10), Word(0x30)],
)]
#[case::write_with_waitstate(
/* INJECT  */ auto_vec![             SrPending,   SrSuccess,   SrPending,            SrSuccess, ],
/* REQUEST */ auto_vec![write(0x10), write(0x11), write(0x11), idle(),    idle(),    idle()],
/* HRDATA  */ auto_vec![HighZ,       Word(0x1),   Word(0x1),   Word(0x2), Word(0x2), HighZ],
/* RESPONS */ auto_vec![Success,     Pending,     Success,     Pending,   Success,   Success],
/* HRDATA  */ auto_vec![HighZ,       HighZ,       HighZ,       HighZ,     HighZ,     HighZ],
)]
#[case::write_with_waitstate_to_read(
/* INJECT  */ auto_vec![            SrPending,  SrSuccess,  SrSuccess],
/* REQUEST */ auto_vec![write(0x10), read(0x10), read(0x10), idle()],
/* HRDATA  */ auto_vec![HighZ,       Word(0x1),  Word(0x1),  HighZ],
/* RESPONS */ auto_vec![Success,     Pending,    Success,    Success],
/* HRDATA  */ auto_vec![HighZ,       HighZ,      HighZ,      Word(0x10)],
)]
#[case::read_with_waitstate_to_write(
// Note: master doesn't provide data for write unless it moves to data phase
/* INJECT  */ auto_vec![            SrPending,   SrSuccess,   SrSuccess],
/* REQUEST */ auto_vec![read(0x10), write(0x10), write(0x10), idle()],
/* HRDATA  */ auto_vec![HighZ,      HighZ,       HighZ,       Word(0x1)],
/* RESPONS */ auto_vec![Success,    Pending,     Success,     Success],
/* HRDATA  */ auto_vec![HighZ,      HighZ,       Word(0x10),  HighZ],
)]
#[case::arm_ahb_fig_3_5(
// Note: master doesn't provide data for write unless it moves to data phase
/* INJECT  */ auto_vec![             SrSuccess,  SrPending,   SrSuccess,   SrSuccess],
/* REQUEST */ auto_vec![write(0x10), read(0x20), write(0x20), write(0x20), idle()],
/* HRDATA  */ auto_vec![HighZ,       Word(0x1),  HighZ,       HighZ,       Word(0x2)],
/* RESPONS */ auto_vec![Success,     Success,    Pending,     Success,     Success],
/* HRDATA  */ auto_vec![HighZ,       HighZ,      HighZ,       Word(0x20),  HighZ],
)]
#[case::complex_test_with_everything(
// Note: master doesn't provide data for write unless it moves to data phase
/* INJECT  */ auto_vec![             SrSuccess,  SrPending,   SrPending,   SrError,                  SrPending, SrError,                        SrSuccess],
/* REQUEST */ auto_vec![write(0x10), read(0x20), write(0x20), write(0x20), write(0x20), write(0x30), idle(),    read(0x10), idle(), read(0x10), idle()],
/* HRDATA  */ auto_vec![HighZ,       Word(0x1),  HighZ,       HighZ,       HighZ,       HighZ,       Word(0x3), Word(0x3),  HighZ,  HighZ,      HighZ],
/* RESPONS */ auto_vec![Success,     Success,    Pending,     Pending,     Error1,      Error2,      Pending,   Error1,     Error2, Success,    Success],
/* HRDATA  */ auto_vec![HighZ,       HighZ,      HighZ,       HighZ,       HighZ,       HighZ,       HighZ,     HighZ,      HighZ,  HighZ,      Word(0x10)],
)]
#[should_panic]
#[case::malformed_test_too_short_inject(
/* INJECT  */ auto_vec![            SrError,               /* consumes for read() here */],
/* REQUEST */ auto_vec![read(0x10), read(0x21), read(0x30), idle()],
/* HRDATA  */ auto_vec![HighZ,      HighZ,      HighZ,      HighZ],
/* RESPONS */ auto_vec![Success,    Error1,     Error2,     Success],
/* HRDATA  */ auto_vec![HighZ,      HighZ,      HighZ,      HighZ],
)]
fn injected_vec_test(
    #[case] injected_responses: Vec<SimpleResponse<()>>,
    #[case] requests_meta: Vec<MasterToSlaveAddrPhase>,
    #[case] requests_data: Vec<DataBus>,
    #[case] responses_meta: Vec<AhbResponseControl>,
    #[case] responses_data: Vec<DataBus>,
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
        responses_data.len(),
        "Malformed test input"
    );
    assert_eq!(
        responses_meta.len(),
        responses_data.len(),
        "Malformed test input"
    );

    component.s.response_iter = Some(injected_responses.into_iter());
    let ctx = &mut context;
    let comp = &mut component;

    comp.tick(ctx);
    for (req, (req_data, (resp, resp_data))) in
        zip!(requests_meta, requests_data, responses_meta, responses_data)
    {
        println!("Cycle for {req:?} resp {resp:?}");

        mix_blocks! {reorder, {
            comp.tock(ctx);
        }, {
            Port::on_ahb_input(comp, ctx, make_m2s(req.clone(), req_data.clone()));
        }}

        let their_resp = comp.last_response.take().unwrap();
        assert_eq!(their_resp.meta, resp);
        assert_eq!(their_resp.data, resp_data);

        // Don't deliver on error
        if req_data.is_present() && matches!(resp, Success | Pending) {
            assert!(comp.s.delivered_write_req.take().is_some());
            assert_eq!(comp.s.delivered_write_data.take().unwrap(), req_data);
        }

        comp.tick(ctx);
        // Only makes sense with no waitstates
        if req.meta.is_idle() {
            assert!(comp.s.delivered_pre_write_req.take().is_none());
            assert!(comp.s.delivered_write_req.take().is_none());
            assert!(comp.s.delivered_read_req.take().is_none());
        }
        // dunno how to check them
        comp.s.delivered_read_req.take();
        comp.s.delivered_pre_write_req.take();

        // No double delivery (comb write delivers in tock)
        assert!(comp.s.delivered_write_req.is_none());
        assert!(comp.s.delivered_write_data.is_none());
    }
}

#[rstest]
#[case::complex_test_with_everything(
// Note: master doesn't provide data for write unless it moves to data phase
/* INJECT  */ auto_vec![             SrSuccess,  SrPending,   SrPending,   SrError,                  SrPending, SrError,                        SrSuccess],
/* REQUEST */ auto_vec![write(0x10), read(0x20), write(0x20), write(0x20), write(0x20), write(0x30), idle(),    read(0x10), idle(), read(0x10), idle()],
/* HRDATA  */ auto_vec![HighZ,       Word(0x1),  HighZ,       HighZ,       HighZ,       HighZ,       Word(0x3), Word(0x3),  HighZ,  HighZ,      HighZ],
/* RESPONS */ auto_vec![Success,     Success,    Pending,     Pending,     Error1,      Error2,      Pending,   Error1,     Error2, Success,    Success],
/* HRDATA  */ auto_vec![HighZ,       HighZ,      HighZ,       HighZ,       HighZ,       HighZ,       HighZ,     HighZ,      HighZ,  HighZ,      Word(0x10)],
)]
fn injected_integrative_test(
    #[case] injected_responses: Vec<SimpleResponse<()>>,
    #[case] requests_meta: Vec<MasterToSlaveAddrPhase>,
    #[case] requests_data: Vec<DataBus>,
    #[case] responses_meta: Vec<AhbResponseControl>,
    #[case] responses_data: Vec<DataBus>,
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
        responses_data.len(),
        "Malformed test input"
    );
    assert_eq!(
        responses_meta.len(),
        responses_data.len(),
        "Malformed test input"
    );

    component.s.response_iter = Some(injected_responses.into_iter());
    let ctx = &mut context;
    let comp = &mut component;

    comp.tick(ctx);
    for (req, (req_data, (resp, resp_data))) in
        zip!(requests_meta, requests_data, responses_meta, responses_data)
    {
        println!("Cycle for {req:?} resp {resp:?}");

        mix_blocks! {reorder, {
            comp.tock(ctx);
        }, {
            Port::on_ahb_input(comp, ctx, make_m2s(req.clone(), req_data.clone()));
        }}

        let their_resp = comp.last_response.take().unwrap();
        assert_eq!(their_resp.meta, resp);
        assert_eq!(their_resp.data, resp_data);

        comp.tick(ctx);
    }
}

/// Manual verification of tricky test
#[rstest]
fn hard_write_with_ws_to_read(
    mut context: Context,
    mut component: TestComponent,
    #[values(true, false)] reorder: bool,
) {
    let injected_responses = /* INJECT  */ vec![SrPending, SrSuccess, SrSuccess];
    component.s.response_iter = Some(injected_responses.into_iter());
    /*
    /* INJECT  */ vec![             SrPending,  SrSuccess,  SrSuccess],
    /* REQUEST */ vec![write(0x10), read(0x10), read(0x10), idle()],
    /* HRDATA  */ vec![HighZ,       Word(0x1),  Word(0x1),  HighZ],
    /* RESPONS */ vec![Success,     Pending,    Success,    Success],
    /* HRDATA  */ vec![HighZ,       HighZ,      HighZ,      Word(0x10)],
     */

    let ctx = &mut context;
    let comp = &mut component;

    comp.tick(ctx);
    {
        println!("Cycle 1: write");
        assert!(comp.s.delivered_read_req.take().is_none());
        assert!(comp.s.delivered_write_req.take().is_none());
        assert!(comp.s.delivered_write_data.take().is_none());
        assert!(comp.s.delivered_pre_write_req.take().is_none());

        println!("Cycle 1: tock");
        mix_blocks! {reorder, {
            comp.tock(ctx);
        }, {
            Port::on_ahb_input(comp, ctx, make_m2s(write(0x10), HighZ));
        }}

        let resp = comp.last_response.take().unwrap();
        assert_eq!(resp.meta, Success);
        assert!(!resp.data.is_present());

        assert!(comp.s.delivered_read_req.take().is_none());
        assert!(comp.s.delivered_write_req.take().is_none());
        assert!(comp.s.delivered_write_data.take().is_none());
        assert!(comp.s.delivered_pre_write_req.take().is_none());
    }

    comp.tick(ctx);
    {
        println!("Cycle 2: write data (waitstate) read on address");
        assert!(comp.s.delivered_read_req.take().is_none());
        assert!(comp.s.delivered_write_req.take().is_none());
        assert!(comp.s.delivered_write_data.take().is_none());

        assert!(comp.s.delivered_pre_write_req.take().is_some());

        println!("Cycle 2: tock");
        mix_blocks! {reorder, {
            comp.tock(ctx);
        }, {
            Port::on_ahb_input(comp, ctx, make_m2s(read(0x10), DataBus::Word(0x1)));
        }}
        let resp = comp.last_response.take().unwrap();
        assert_eq!(resp.meta, Pending);
        assert!(!resp.data.is_present());

        assert!(comp.s.delivered_read_req.is_none());
        assert!(comp.s.delivered_pre_write_req.take().is_none());
        // Late combinatorial arrival
        assert!(comp.s.delivered_write_req.take().is_some());
        assert!(comp.s.delivered_write_data.take().unwrap().raw() == 0x1);
    }

    comp.tick(ctx);
    {
        println!("Cycle 3: write data (ok) read on to be latched");
        // no delivery during waitstate
        assert!(comp.s.delivered_read_req.take().is_none());
        // no double-delivery
        assert!(comp.s.delivered_write_req.take().is_none());
        assert!(comp.s.delivered_write_data.take().is_none());

        assert!(comp.s.delivered_pre_write_req.take().is_none());

        println!("Cycle 3: tock");
        mix_blocks! {reorder, {
            comp.tock(ctx);
        }, {
            Port::on_ahb_input(comp, ctx, make_m2s(read(0x10), DataBus::Word(0x1)));
        }}
        let resp = comp.last_response.take().unwrap();
        assert_eq!(resp.meta, Success);
        assert!(!resp.data.is_present());

        assert!(comp.s.delivered_read_req.take().is_none());
        assert!(comp.s.delivered_pre_write_req.take().is_none());
        // Late combinatorial arrival
        assert!(comp.s.delivered_write_req.take().is_some());
        assert!(comp.s.delivered_write_data.take().unwrap().raw() == 0x1);
    }

    comp.tick(ctx);
    {
        println!("Cycle 4: idle, responding to read");
        assert!(comp.s.delivered_read_req.take().is_some());
        // TODO: don't check it here, have a simple check in slave_driver and make this field private
        assert!(comp.s.iface.delayed_reply.is_some());

        // no double-delivery
        assert!(comp.s.delivered_write_req.take().is_none());
        assert!(comp.s.delivered_write_data.take().is_none());

        assert!(comp.s.delivered_pre_write_req.take().is_none());

        println!("Cycle 4: tock");
        mix_blocks! {reorder, {
            comp.tock(ctx);
        }, {
            Port::on_ahb_input(comp, ctx,  make_m2s(idle(), DataBus::HighZ));
        }}
        let resp = comp.last_response.take().unwrap();
        assert_eq!(resp.meta, Success);
        assert!(resp.data.raw() == 0x10);

        assert!(comp.s.delivered_read_req.take().is_none());
        assert!(comp.s.delivered_pre_write_req.take().is_none());
        assert!(comp.s.delivered_write_req.take().is_none());
        assert!(comp.s.delivered_write_data.take().is_none());
    }
}

#[cfg(feature = "cycle-debug-logger")]
#[rstest]
#[case::easy_with_idles(
// Note: master doesn't provide data for write unless it moves to data phase
/* INJECT  */ auto_vec![             SrSuccess,  SrSuccess,   ],
/* REQUEST */ auto_vec![write(0x10), read(0x20), idle(),     idle(),    ],
/* ADR TAG */ atom_vec![Writer,      Reader,     Idler,      ?,       ],
/* HWDATA  */ auto_vec![HighZ,       Word(0x1),  HighZ,      HighZ,   ],
/* DAT TAG */ atom_vec![?,           Writer,     Reader,     ?,       ],
/* RESPONS */ auto_vec![Success,     Success,    Success,    Success, ],
/* HRDATA  */ auto_vec![HighZ,       HighZ,      Word(0x20), HighZ,   ],
/* SRC TAG */ atom_vec![?,           Writer,     Reader,     Idler,   ],
)]
#[case::complex_test_with_everything(
// Note: master doesn't provide data for write unless it moves to data phase
/* INJECT  */ auto_vec![             SrSuccess,  SrPending,   SrPending,   SrError,                  SrPending, SrError,                        SrSuccess],
/* REQUEST */ auto_vec![write(0x10), read(0x20), write(0x20), write(0x20), write(0x20), write(0x30), idle(),    read(0x10), idle(), read(0x10), idle()],
/* ADR TAG */ atom_vec![A,           B,          A,           A,           A,           Z,           ?,         D,          ?,      D,          ?],
/* HWDATA  */ auto_vec![HighZ,       Word(0x1),  HighZ,       HighZ,       HighZ,       HighZ,       Word(0x3), Word(0x3),  HighZ,  HighZ,      HighZ],
/* DAT TAG */ atom_vec![?,           A,          B,           B,           B,           ?,           Z,         Z,          ignore, ?,          D],
/* RESPONS */ auto_vec![Success,     Success,    Pending,     Pending,     Error1,      Error2,      Pending,   Error1,     Error2, Success,    Success],
/* HRDATA  */ auto_vec![HighZ,       HighZ,      HighZ,       HighZ,       HighZ,       HighZ,       HighZ,     HighZ,      HighZ,  HighZ,      Word(0x10)],
/* SRC TAG */ atom_vec![?,           A,          B,           B,           B,           B,           Z,         Z,          Z,      ?,          D],
)]
fn cdl_tag_propagation(
    #[case] injected_responses: Vec<SimpleResponse<()>>,
    #[case] requests_meta: Vec<MasterToSlaveAddrPhase>,
    #[case] requests_atag: Vec<CdlTag>,
    #[case] requests_data: Vec<DataBus>,
    #[case] requests_dtag: Vec<CdlTag>,
    #[case] responses_meta: Vec<AhbResponseControl>,
    #[case] responses_data: Vec<DataBus>,
    #[case] responses_tag: Vec<CdlTag>,
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
        requests_atag.len(),
        "Malformed test input"
    );
    assert_eq!(
        requests_meta.len(),
        requests_dtag.len(),
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

    component.s.response_iter = Some(injected_responses.into_iter());
    let ctx = &mut context;
    let comp = &mut component;

    comp.tick(ctx);
    let default_fixer = |t| if t == "?" { CdlTag::default() } else { t };
    let tags_zipper = zip!(requests_atag, requests_dtag, responses_tag);
    for (req, (req_data, (resp, (resp_data, (atag, (dtag, rtag)))))) in zip!(
        requests_meta,
        requests_data,
        responses_meta,
        responses_data,
        tags_zipper
    ) {
        let m2s = MasterToSlaveWires {
            addr_phase: MasterToSlaveAddrPhase {
                tag: default_fixer(atag),
                ..req.clone()
            },
            data_phase: MasterToSlaveDataPhase {
                tag: default_fixer(dtag),
                data: req_data.clone(),
            },
        };

        let s2m = SlaveToMasterWires {
            meta: resp,
            data: resp_data,
            responder_tag: Port::TAG.into(),
            sender_tag: default_fixer(rtag),
        };

        println!("Cycle for {m2s:?} resp {s2m:?}");

        mix_blocks! {reorder, {
            comp.tock(ctx);
        }, {
            Port::on_ahb_input(comp, ctx, m2s);
        }}

        assert_eq!(comp.last_response.take().unwrap(), s2m);

        comp.tick(ctx);
    }
}

// TODO: Registered write delivery tests.
