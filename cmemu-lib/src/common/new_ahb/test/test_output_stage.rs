#![allow(clippy::too_many_arguments)]
#![cfg_attr(not(feature = "cycle-debug-logger"), allow(unused_imports))]

use crate::common::new_ahb::arbiter::RoundRobinArbiter;
#[cfg(feature = "cycle-debug-logger")]
use crate::common::new_ahb::cdl::CdlTag;

use crate::common::new_ahb::output_stage::*;

use crate::common::Address;
use crate::common::new_ahb::ports::AHBPortConfig;
use crate::common::new_ahb::signals::{
    AhbResponseControl, MasterToSlaveDataPhase, MasterToSlaveWires, SlaveToMasterWires,
    TrackedBool, TransferMeta,
};
use crate::common::new_ahb::slave_driver::SimpleResponse;
use crate::common::new_ahb::test::logging_ports::*;
use crate::common::new_ahb::test::utils::*;
use crate::common::new_ahb::vlan::{
    AHBSlavePortTaggedInput, AhbMultiMasterConfig, AhbSlaveOutputDispatcher,
};
use crate::common::utils::iter_enum;
use crate::engine::{
    Context, DisableableComponent, Subcomponent, TickComponent, TickComponentExtra,
};
use crate::utils::ife;
use crate::{bridge_ports, decoder_tags_and_markers, zip};
use enum_map::{Enum, EnumMap};
use rstest::*;
use std::convert::identity;
use std::fmt::Debug;

#[derive(Enum)]
enum TestMasters2 {
    MasterA,
    MasterB,
}
decoder_tags_and_markers!(@with_markers
    enum TestMasters {
        MasterA,
        MasterB,
    }
);
use TestMasters::{MasterA as A, MasterB as B};
const AU: u32 = 0;
const BU: u32 = 1;

impl From<Address> for TestMasters {
    fn from(a: Address) -> Self {
        match a.to_const() {
            self::AU => A,
            self::BU => B,
            _ => unreachable!("Broken tests"),
        }
    }
}

type TestData = Option<TestMasters>;
type Arbiter = RoundRobinArbiter<TestMasters>;
type Output = OutputStage<OutStageSC>;

#[derive(Subcomponent, TickComponent, TickComponentExtra, DisableableComponent)]
struct TestComponent {
    #[subcomponent(OutStageSC)]
    output: Output,

    #[subcomponent(SlaveSC)]
    s: TestSlave<SlaveSC>,

    last_resp: EnumMap<TestMasters, Option<SlaveToMasterWires<TestData>>>,
    last_grant: EnumMap<TestMasters, Option<bool>>,
}

impl TestComponent {
    fn tick(&mut self, ctx: &mut Context) {
        #[cfg(debug_assertions)]
        self.tick_assertions_traverse();
        self.tick_flops_and_extra_traverse();
        self.last_resp.clear();
        TestSlave::<SlaveSC>::tick(self, ctx);
    }

    fn tock(&mut self, ctx: &mut Context) {
        TestSlave::<SlaveSC>::tock(self, ctx);
    }
}

impl Checker for SlaveSC {
    type Data = TestData;

    fn check_and_reply_read(request: &TransferMeta) -> Self::Data {
        Some(request.addr.into())
    }
}

impl AHBPortConfig for OutputStage<OutStageSC> {
    type Data = TestData;
    type Component = TestComponent;
    const TAG: &'static str = "Arbiter";
}

impl AhbMultiMasterConfig for OutputStage<OutStageSC> {
    type MastersEnum = TestMasters;
    type Arbiter = Arbiter;
}

impl AhbSlaveOutputDispatcher<TestMasters> for OutputStage<OutStageSC> {
    fn dispatch_ahb_output(
        comp: &mut Self::Component,
        ctx: &mut Context,
        tag: Self::MastersEnum,
        msg: SlaveToMasterWires<Self::Data>,
    ) {
        println!("Master {tag:?} got resp {msg:?}");
        comp.last_resp[tag] = Some(msg);
    }

    fn on_grant_wire(
        comp: &mut Self::Component,
        ctx: &mut Context,
        tag: Self::MastersEnum,
        granted: TrackedBool,
    ) {
        println!(
            "Master {:?} was {} grant={:?}",
            tag,
            ife(*granted, "GRANTED", "DENIED"),
            granted,
        );
        comp.last_grant[tag] = Some(*granted);
    }
}
bridge_ports!(OutputStage<OutStageSC> => auto_configured TestSlave<SlaveSC>);

// ╔════════════╗
// ║actual tests║
// ╚════════════╝

#[fixture]
fn component() -> TestComponent {
    TestComponent {
        output: OutputStage::default(),
        s: Default::default(),
        last_resp: Default::default(),
        last_grant: Default::default(),
    }
}

#[fixture]
fn context() -> Context {
    Context::new_for_test()
}

#[rstest]
fn noop_works(mut context: Context, mut component: TestComponent) {
    let ctx = &mut context;
    let comp = &mut component;

    for _ in 0..10 {
        println!("-> Tick <-");
        comp.tick(ctx);
        comp.tock(ctx);
        // Output::try_send_output(comp, ctx);
    }
}

#[rstest]
#[test_log::test]
fn one_master_loop(mut context: Context, mut component: TestComponent) {
    let ctx = &mut context;
    let comp = &mut component;

    println!("-> Tick deny <-");
    comp.tick(ctx);
    comp.tock(ctx);
    <Output as AHBSlavePortTaggedInput>::on_ahb_tagged_input(
        comp,
        ctx,
        A,
        make_m2s(write(AU), None),
    );

    assert!(comp.last_resp[A].take().is_none());
    assert!(comp.last_grant[A].take().is_some_and(|g| !g));
    assert!(comp.s.last_input.is_none());

    println!("-> Tick access <-");
    comp.tick(ctx);
    comp.tock(ctx);
    <Output as AHBSlavePortTaggedInput>::on_ahb_tagged_input(
        comp,
        ctx,
        A,
        make_m2s(write(AU), A.into()),
    );

    assert!(comp.last_resp[A].take().is_none_or(|r| r.meta.is_done()));
    assert!(comp.last_grant[A].take().is_some_and(identity));
    let last_in = comp.s.last_input.take().unwrap();
    assert!(last_in.addr_phase.meta.is_writing());
    assert!(last_in.data_phase.data.is_none());
    println!("-> Tick <-");
    comp.tick(ctx);
    assert!(comp.s.delivered_pre_write_req.take().is_some());
    assert!(comp.s.delivered_write_req.take().is_none());

    for _ in 0..10 {
        let msg = make_m2s(write(AU), A.into());
        comp.tock(ctx);
        <Output as AHBSlavePortTaggedInput>::on_ahb_tagged_input(comp, ctx, A, msg.clone());

        assert!(comp.last_resp[A].take().is_some());
        assert!(comp.last_grant[A].take().is_some_and(identity));
        assert_eq!(comp.s.last_input.take().unwrap(), msg);

        println!("-> Tick <-");
        comp.tick(ctx);
        assert!(comp.s.delivered_pre_write_req.take().is_some());
        assert!(comp.s.delivered_write_req.take().is_some());
        assert_eq!(comp.s.delivered_write_data.take().unwrap(), A.into());
    }
}

fn send(
    comp: &mut TestComponent,
    ctx: &mut Context,
    tag: TestMasters,
    msg: &MasterToSlaveWires<TestData>,
) {
    <Output as AHBSlavePortTaggedInput>::on_ahb_tagged_input(comp, ctx, tag, msg.clone());
}

#[rstest]
#[test_log::test]
fn two_masters_loop(mut context: Context, mut component: TestComponent) {
    let ctx = &mut context;
    let comp = &mut component;

    let a_msg = make_m2s(read(AU), None).tagged("A");
    let b_msg = make_m2s(read(BU), None).tagged("B");

    println!("-> Tick deny <-");
    comp.tick(ctx);
    comp.tock(ctx);
    send(comp, ctx, A, &a_msg);
    send(comp, ctx, B, &b_msg);

    assert!(comp.last_resp[A].take().is_none());
    assert!(comp.last_grant[A].take().is_some_and(|g| !g));
    assert!(comp.last_grant[B].take().is_some_and(|g| !g));
    assert!(comp.s.last_input.is_none());

    println!("\n-> Tick access <-");
    comp.tick(ctx);
    comp.tock(ctx);
    send(comp, ctx, A, &a_msg);
    send(comp, ctx, B, &b_msg);

    assert!(comp.last_resp[A].take().is_none_or(|r| r.meta.is_done()));
    assert!(comp.last_grant[A].take().is_some_and(identity));
    assert!(comp.last_grant[B].take().is_some_and(|g| !g));
    let last_in = comp.s.last_input.take().unwrap();
    assert_eq!(last_in.addr_phase, a_msg.addr_phase);
    assert_eq!(
        last_in.data_phase,
        MasterToSlaveDataPhase::empty::<Output>()
    );

    for i in 0..10 {
        println!("\n-> Tick <-");
        comp.tick(ctx);
        assert!(comp.s.delivered_read_req.take().is_some());
        comp.tock(ctx);
        send(comp, ctx, A, &a_msg);
        send(comp, ctx, B, &b_msg);

        println!("Deny: {:?}", comp.last_grant);
        println!("Resp: {:?}", comp.last_resp);
        let input = comp.s.last_input.take().unwrap();
        if i % 2 == 0 {
            assert!(comp.last_resp[A].take().is_some());
            // It was denied last cycle, thus it's addr phase was not propagated, thus
            // it shouldn't expect reply if after the last transfer was finished.
            assert!(comp.last_resp[B].take().is_none());
            assert!(comp.last_grant[A].take().is_some_and(|g| !g));
            assert!(comp.last_grant[B].take().is_some_and(identity));
            assert_eq!(input.addr_phase, b_msg.addr_phase);
            assert_eq!(input.data_phase, a_msg.data_phase);
        } else {
            assert!(comp.last_resp[B].take().is_some());
            assert!(comp.last_resp[A].take().is_none());
            assert!(comp.last_grant[A].take().is_some_and(identity));
            assert!(comp.last_grant[B].take().is_some_and(|g| !g));
            assert_eq!(input.addr_phase, a_msg.addr_phase);
            assert_eq!(input.data_phase, b_msg.data_phase);
        }
    }
}

const A_TAG: &str = "A";
const B_TAG: &str = "B";
const O_TAG: &str = <Output as AHBPortConfig>::TAG;
impl MarkerHelpers for TestMasters {
    fn uaddr(self) -> u32 {
        if self == A { AU } else { BU }
    }

    fn tag_str(self) -> &'static str {
        if self == A { A_TAG } else { B_TAG }
    }
}

use crate::common::new_ahb::signals::AhbResponseControl::*;
use crate::common::new_ahb::test::utils::MarkerHelpers;
use crate::common::new_ahb::test::utils::ReqType::*;
#[allow(unused_imports)] // It is used, but some tools don't see that.
use crate::common::new_ahb::test::utils::{idle, make_m2s, read, write};
#[allow(unused_imports)] // It is used, but some tools don't see that.
use crate::{atom_vec, auto_vec};
#[allow(unused_imports)]
use core::option::Option::None as na;
use enum_map::enum_map;

#[cfg(feature = "cycle-debug-logger")]
#[rstest]
#[test_log::test]
#[case::easy_with_idles(
/* INJECT  */ auto_vec![na,      na,      SrSuccess, SrSuccess, ],
/* REQ   A */ auto_vec![Write,   Write,   Idle,      NoSel,     ],
/* HWDAT A */ auto_vec![None,    A,       A,         None,      ],
/* REQ   B */ auto_vec![Idle,    Read,    Read,      Idle,      ],
/* HWDAT B */ auto_vec![None,    None,    None,      None,      ],
/* RESPONS */ auto_vec![Success, Success, Success,   Success,   ],
/* DROUTE  */ auto_vec![None,    None,    A,         B,         ],
/* ADR TAG */ auto_vec![O_TAG,   A_TAG,   B_TAG,     B_TAG,     ],
/* DAT TAG */ auto_vec![O_TAG,   O_TAG,   A_TAG,     B_TAG,     ],
)]
#[case::one_master_waitstate(
/* INJECT  */ auto_vec![na,      na,      SrPending, SrSuccess, SrSuccess, ],
/* REQ   A */ auto_vec![Write,   Write,   Idle,      Idle,      NoMsg,     ],
/* HWDAT A */ auto_vec![None,    A,       A,         A,         None,      ],
/* REQ   B */ auto_vec![NoMsg,   NoSel,   NoMsg,     NoSel,     NoMsg,     ],
/* HWDAT B */ auto_vec![None,    None,    None,      None,      None,      ],
/* RESPONS */ auto_vec![Success, Success, Pending,   Success,   Success,   ],
/* DROUTE  */ auto_vec![None,    None,    A,         A,         None,      ],
/* ADR TAG */ auto_vec![O_TAG,   A_TAG,   A_TAG,     A_TAG,     O_TAG,     ],
/* DAT TAG */ auto_vec![O_TAG,   O_TAG,   A_TAG,     A_TAG,     O_TAG,     ],
)]
#[case::swap_to_no_op( // Note: in 6th cycle output_stage is allowed to be incompatible with replies
/* INJECT  */ auto_vec![na,      na,      SrPending, SrSuccess, SrSuccess, na,        na,      SrSuccess, SrSuccess],
/* REQ   A */ auto_vec![Write,   Write,   Idle,      Idle,      NoMsg,     NoMsg,     Read,    Read,      Idle],
/* HWDAT A */ auto_vec![None,    A,       A,         A,         None,      None,      None,    None,      None],
/* REQ   B */ auto_vec![Read,    Read,    Read,      Read,      Idle,      NoSel,     Read,    Read,      Idle],
/* HWDAT B */ auto_vec![None,    None,    None,      None,      None,      None,      None,    None,      None],
/* RESPONS */ auto_vec![Success, Success, Pending,   Success,   Success,   Success,   Success, Success,   Success],
/* DROUTE  */ auto_vec![None,    None,    A,         A,         B,         None,      None,    B,         A],
/* ADR TAG */ auto_vec![O_TAG,   A_TAG,   B_TAG,     B_TAG,     B_TAG,     O_TAG,     B_TAG,   A_TAG,     B_TAG],
/* DAT TAG */ auto_vec![O_TAG,   O_TAG,   A_TAG,     A_TAG,     B_TAG,     O_TAG,     O_TAG,   B_TAG,     A_TAG,],
)]
#[case::rapid_decoder_swaps(
/* INJECT  */ auto_vec![na,      na,      SrPending, SrSuccess, SrSuccess, na,      na,     na,      na,      SrSuccess, SrSuccess, SrSuccess],
/* REQ   A */ auto_vec![Write,   Write,   NoSel,     NoSel,     NoMsg,     NoMsg,   NoMsg,  Write,   Write,   NoSel,     Read,      NoSel],
/* HWDAT A */ auto_vec![None,    A,       A,         A,         None,      None,    None,   None,    None,    A,         None,      None],
/* REQ   B */ auto_vec![Write,   Write,   Write,     Write,     NoSel,     NoMsg,   NoMsg,  NoMsg,   Write,   Write,     NoSel,     NoMsg],
/* HWDAT B */ auto_vec![None,    None,    None,      None,      B,         None,    None,   None,    None,    None,      B,         None],
/* RESPONS */ auto_vec![Success, Success, Pending,   Success,   Success,   Success, Success,Success, Success, Success,   Success,   Success],
/* DROUTE  */ auto_vec![None,    None,    A,         A,         B,         None,    None,   None,    None,    A,         B,         None],
/* ADR TAG */ auto_vec![O_TAG,   A_TAG,   B_TAG,     B_TAG,     B_TAG,     O_TAG,   O_TAG,  O_TAG,   A_TAG,   B_TAG,     B_TAG,     O_TAG],
/* DAT TAG */ auto_vec![O_TAG,   O_TAG,   A_TAG,     A_TAG,     B_TAG,     O_TAG,   O_TAG,  O_TAG,   O_TAG,   A_TAG,     B_TAG,     O_TAG,   ],
)]
fn test_interleaving(
    #[case] injected_responses: Vec<Option<SimpleResponse<()>>>,
    #[case] requests_a: Vec<ReqType>,
    #[case] requests_a_data: Vec<TestData>,
    #[case] requests_b: Vec<ReqType>,
    #[case] requests_b_data: Vec<TestData>,
    #[case] responses_meta: Vec<AhbResponseControl>,
    #[case] responses_route: Vec<TestData>,
    #[case] addr_tag: Vec<CdlTag>,
    #[case] data_tag: Vec<CdlTag>,
    mut context: Context,
    mut component: TestComponent,
) {
    assert_eq!(
        requests_a.len(),
        requests_a_data.len(),
        "Malformed test input"
    );
    assert_eq!(requests_a.len(), requests_a.len(), "Malformed test input");
    assert_eq!(
        requests_b.len(),
        requests_b_data.len(),
        "Malformed test input"
    );
    assert_eq!(
        requests_a.len(),
        responses_meta.len(),
        "Malformed test input"
    );
    assert_eq!(
        requests_a.len(),
        responses_route.len(),
        "Malformed test input"
    );
    assert_eq!(requests_a.len(), addr_tag.len(), "Malformed test input");

    component.s.response_iter = Some(
        injected_responses
            .into_iter()
            .flatten()
            .collect::<Vec<_>>()
            .into_iter(),
    );
    let ctx = &mut context;
    let comp = &mut component;

    let mut prev_denied = enum_map!(_ => true);
    let mut prev_req = enum_map!(_ => NoSel);
    for (i, (req_a, (req_a_data, (req_b, (req_b_data, (resp, (resp_route, (atag, dtag)))))))) in
        zip!(
            requests_a,
            requests_a_data,
            requests_b,
            requests_b_data,
            responses_meta,
            responses_route,
            addr_tag,
            data_tag,
        )
        .enumerate()
    {
        println!("\nCycle for A:{req_a:?} B:{req_b:?} resp to: {resp_route:?}");
        comp.tick(ctx);
        comp.tock(ctx);
        println!("Resp: {:?}", comp.last_resp);
        if resp_route.is_some() {
            let s2m = SlaveToMasterWires {
                meta: resp,
                data: resp_route,
                responder_tag: TestSlave::<SlaveSC>::TAG.into(),
                sender_tag: resp_route.unwrap().cdl_tag(),
            };
            let resp = comp.last_resp[resp_route.unwrap()].as_ref().unwrap();
            assert_eq!(resp.meta, s2m.meta);
            assert!(resp.sender_tag == s2m.sender_tag || resp.sender_tag == CdlTag::default());
            assert_eq!(resp.responder_tag, s2m.responder_tag);
        }

        let m2s_a = reflect_hready_granting(
            make_m2s(make_addr_from_req_type(req_a, A), req_a_data),
            comp.last_resp[A].as_ref(),
            comp.last_grant[A].is_some_and(|g| !g),
        );
        let m2s_b = reflect_hready_granting(
            make_m2s(make_addr_from_req_type(req_b, B), req_b_data),
            comp.last_resp[B].as_ref(),
            comp.last_grant[B].is_some_and(|g| !g),
        );
        if req_a != NoMsg {
            <Output as AHBSlavePortTaggedInput>::on_ahb_tagged_input(comp, ctx, A, m2s_a);
        }
        if req_b != NoMsg {
            <Output as AHBSlavePortTaggedInput>::on_ahb_tagged_input(comp, ctx, B, m2s_b);
        }

        println!("Grants: {:?}", comp.last_grant);
        if atag != O_TAG || dtag != O_TAG
        // && (req_a.expects_reply() || req_b.expects_reply() || resp_route.is_some())
        {
            let sent = comp
                .s
                .last_input
                .take()
                .expect("Slave expected input this cycle");
            assert_eq!(sent.addr_phase.tag, atag);
            assert_eq!(sent.data_phase.tag, dtag);
        } else {
            assert!(comp.s.last_input.take().is_none());
        }

        for tag in iter_enum::<TestMasters>() {
            let req = match tag {
                self::A => req_a,
                self::B => req_b,
            };

            assert!(
                comp.last_grant[tag].is_some_and(|g| !g) || !prev_req[tag].expects_reply()
                    || prev_denied[tag]
                    || comp.last_resp[tag].is_some() || (prev_req[tag] == Idle && dtag != tag.tag_str())
                ,
                "When master sends a message, it should get at least grant or a response with HREADY.
                It is not the case for {tag:?}"
            );
            prev_denied[tag] = comp.last_grant[tag].is_some_and(|g| !g);
            prev_req[tag] = req;
        }
        comp.last_resp.clear();
        comp.last_grant.clear();
    }
}
