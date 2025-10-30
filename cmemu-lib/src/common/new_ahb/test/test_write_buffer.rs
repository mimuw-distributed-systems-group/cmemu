#![allow(clippy::too_many_arguments)]
use crate::common::new_ahb::ports::AHBPortConfig;
use crate::common::new_ahb::signals::AhbResponseControl::Success;
use crate::common::new_ahb::signals::AhbResponseControl::*;
use crate::common::new_ahb::signals::{AhbResponseControl, MasterToSlaveWires, TransferMeta};
use crate::common::new_ahb::slave_driver::SimpleResponse;
use crate::common::new_ahb::test::logging_ports::*;
use crate::common::new_ahb::test::utils::ReqType::*;
use crate::common::new_ahb::test::utils::make_m2s;
use crate::common::new_ahb::test::utils::*;
use crate::common::new_ahb::write_buffer::{WriteBuffer, WriteBufferCfg};
use crate::engine::{
    Context, DisableableComponent, Subcomponent, TickComponent, TickComponentExtra,
};
use crate::test_utils::inc_time;
#[allow(unused_imports)]
use crate::{atom_vec, auto_vec};
use crate::{bridge_ports, mix_blocks, zip};

use rstest::{fixture, rstest};
use std::fmt::Debug;

type Master = SimpleTestMaster<MasterSC>;
type Slave = TestSlave<SlaveSC>;
type WB = WriteBuffer<WriteBufferSC>;

// TODO: test stacked writebuffers
#[derive(Subcomponent, TickComponent, TickComponentExtra, DisableableComponent)]
struct TestComponent {
    #[subcomponent(MasterSC)]
    m: Master,

    #[subcomponent(SlaveSC)]
    s: Slave,

    #[subcomponent(WriteBufferSC)]
    wb: WB,
}

impl TestComponent {
    fn tick(&mut self, ctx: &mut Context) {
        #[cfg(debug_assertions)]
        self.tick_assertions_traverse();
        self.tick_flops_and_extra_traverse();
        Slave::tick(self, ctx);
        WB::tick(self, ctx);
    }

    fn tock(&mut self, ctx: &mut Context) {
        Slave::tock(self, ctx);
        WB::tock(self, ctx);
    }
}

impl AHBPortConfig for Master {
    type Data = i32;
    type Component = TestComponent;
    const TAG: &'static str = "Master";
}

bridge_ports!(Master => @auto_configured WB);
bridge_ports!(WB => @auto_configured Slave);

impl WriteBufferCfg for WB {
    const IS_BUF_TO_LOAD_FAST: bool = true;
}

impl Checker for SlaveSC {
    type Data = i32;

    fn check_and_reply_read(request: &TransferMeta) -> Self::Data {
        15
    }
}

#[derive(Copy, Clone, Debug)]
struct MasterMarker;

impl MarkerHelpers for MasterMarker {
    fn uaddr(self) -> u32 {
        12
    }

    fn tag_str(self) -> &'static str {
        "Master"
    }
}

#[fixture]
fn component() -> TestComponent {
    TestComponent {
        m: Default::default(),
        s: Default::default(),
        wb: WB::new(),
    }
}

// Pycharm doesn't ganerate "run all tests" without that
#[test]
fn dummy() {}

#[fixture]
fn context() -> Context {
    Context::new_for_test()
}

#[rstest]
fn noop_works(mut context: Context, mut component: TestComponent) {
    let ctx = &mut context;

    for _ in 0..10 {
        component.tick(ctx);
        component.tock(ctx);
    }
}

#[allow(unused_imports)]
use std::option::Option::None as na;
#[allow(non_upper_case_globals)]
const null: i32 = 0;

#[rstest]
#[test_log::test]
#[case::easy_with_idles(
/* INJECT  */ auto_vec![na,      SrSuccess, SrSuccess, ],
/* REQ   M */ auto_vec![Write,   Idle,      NoSel,     ],
/* HWDAT M */ auto_vec![null,    1,         null,      ],
/* REQ @ S */ auto_vec![Write,   Idle,      NoSel,     ],
/* HWDAT@S */ auto_vec![null,    1,         null,      ],
/* RESPONS */ auto_vec![Success, Success,   Success,   ],
/* RESP DT */ auto_vec![null,    null,      null,   ],
)]
#[case::easy_just_reads(
/* INJECT  */ auto_vec![na,      SrPending, SrSuccess, SrSuccess, na,      SrSuccess],
/* REQ   M */ auto_vec![Read,    Read,      Read,      Idle,      Read,    NoSel],
/* HWDAT M */ auto_vec![null,    null,      null,      null,      null,    null],
/* REQ @ S */ auto_vec![Read,    Read,      Read,      Idle,      Read,    NoSel],
/* HWDAT@S */ auto_vec![null,    null,      null,      null,      null,    null],
/* RESPONS */ auto_vec![Success, Pending,   Success,   Success,   Success, Success],
/* RESP DT */ auto_vec![null,    null,      15,        15,        null,    15],
)]
#[case::no_ws_pipelining(
/* INJECT  */ auto_vec![na,      SrSuccess, SrSuccess, SrSuccess, na,      SrSuccess],
/* REQ   M */ auto_vec![Write,   Read,      Write,     Idle,      Write,   NoSel],
/* HWDAT M */ auto_vec![null,    1,         null,      2,         null,    3],
/* REQ   S */ auto_vec![Write,   Read,      Write,     Idle,      Write,   NoSel],
/* HWDAT@S */ auto_vec![null,    1,         null,      2,         null,    3],
/* RESPONS */ auto_vec![Success, Success,   Success,   Success,   Success, Success],
/* RESP DT */ auto_vec![null,    null,      15,        null,      null,    null],
)]
#[case::write_train_ws(
/* INJECT  */ auto_vec![na,      SrPending, SrPending, SrSuccess, SrPending, SrSuccess, ],
/* REQ   M */ auto_vec![Write,   Write,     Write,     Write,     Write,     NoSel,     ],
/* HWDAT M */ auto_vec![null,    1,         2,         2,         2,         3,         ],
/* REQ   S */ auto_vec![Write,   Write,     Write,     Write,     Write,     Write,     ],
/* HWDAT@S */ auto_vec![null,    1,         1,         1,         2,         2,         ],
/* RESPONS */ auto_vec![Success, Success,   Pending,   Pending,   Success,   Pending,   ],
/* RESP DT */ auto_vec![null,    null,      null,      null,      null,      null,      ],
)]
#[ignore] // reorder is broken
#[case::fast_switch_after_idle_wait_after_read(
/* INJECT  */ auto_vec![na,      SrPending, SrSuccess, SrPending, SrSuccess, SrPending, SrSuccess],
/* REQ   M */ auto_vec![Write,   Idle,      Read,      Write,     Write,     NoSel,     NoMsg],
/* HWDAT M */ auto_vec![null,    1,         null,      null,      null,      3,         null],
/* REQ   S */ auto_vec![Write,   Idle,      Read,      Write,     Write,     NoSel,     Idle],
/* HWDAT@S */ auto_vec![null,    1,         1,         null,      null,      3,         3],
/* RESPONS */ auto_vec![Success, Success,   Success,   Pending,   Success,   Success,   Success],
/* RESP DT */ auto_vec![null,    null,      null,      null,      15,        null,      null],
)]
fn vec_test(
    #[case] injected_responses: Vec<Option<SimpleResponse<()>>>,
    #[case] requests_meta: Vec<ReqType>,
    #[case] requests_data: Vec<i32>,
    #[case] slave_meta: Vec<ReqType>,
    #[case] slave_data: Vec<i32>,
    #[case] responses_meta: Vec<AhbResponseControl>,
    #[case] responses_data: Vec<Option<i32>>,
    mut context: Context,
    mut component: TestComponent,
    #[values(false, true)] reorder: bool,
) {
    assert_eq!(
        requests_meta.len(),
        requests_data.len(),
        "Malformed test input"
    );
    assert_eq!(
        requests_meta.len(),
        slave_meta.len(),
        "Malformed test input"
    );
    assert_eq!(slave_meta.len(), slave_data.len(), "Malformed test input");
    assert_eq!(
        requests_meta.len(),
        responses_meta.len(),
        "Malformed test input"
    );
    assert_eq!(
        requests_meta.len(),
        responses_data.len(),
        "Malformed test input"
    );
    assert_eq!(
        requests_meta.len(),
        injected_responses.len(),
        "Malformed test input"
    );

    component.s.response_iter = Some(
        injected_responses
            .into_iter()
            .flatten()
            .collect::<Vec<_>>()
            .into_iter(),
    );
    let ctx = &mut context;
    let comp = &mut component;

    for (req, (req_data, (s_req, (s_data, (resp, resp_data))))) in zip!(
        requests_meta,
        requests_data,
        slave_meta,
        slave_data,
        responses_meta,
        responses_data
    ) {
        println!("\nCycle for {req:?} + {req_data:?} expecting {resp:?}");

        inc_time(ctx, 5);
        comp.tick(ctx);

        inc_time(ctx, 5);
        mix_blocks!(
            reorder,
            if req != NoMsg {
                Master::send(
                    comp,
                    ctx,
                    make_m2s(make_addr_from_req_type(req, MasterMarker), req_data),
                    true,
                );
            },
            comp.tock(ctx)
        );

        let s_got = comp.s.last_input.take();
        assert_eq!(s_got.is_none(), s_req == NoMsg);
        if let Some(MasterToSlaveWires {
            addr_phase,
            data_phase,
        }) = s_got
        {
            assert!(
                s_req.matches_kind(&addr_phase.meta),
                "Mismatched slave input, expected: {s_req:?}, got {addr_phase:?}"
            );
            assert_eq!(data_phase.data, s_data);
        }

        let m_resp = comp.m.last_resp.take();
        assert_eq!(m_resp.as_ref().map_or(Success, |r| r.meta), resp);
        if let Some(rdata) = resp_data {
            assert_eq!(m_resp.unwrap().data, rdata);
        }
    }
}
