#![allow(clippy::too_many_arguments)]
#![allow(clippy::needless_pub_self)]

use crate::common::Address;
use crate::common::new_ahb::arbiter::{FixedArbiter, RoundRobinArbiter};
use crate::common::new_ahb::decoder::{AhbPort as DPort, Decoder};
use crate::common::new_ahb::input_stage::InputStage;
use crate::common::new_ahb::output_stage::{AhbPort as OPort, OutputStage};
use crate::common::new_ahb::ports::{
    AHBMasterPortInput, AHBMasterPortOutput, AHBPortConfig, AHBSlavePortInput, AHBSlavePortOutput,
};
use crate::common::new_ahb::signals::{
    AhbResponseControl, MasterToSlaveAddrPhase, MasterToSlaveWires, SlaveToMasterWires,
    TransferMeta,
};
use crate::common::new_ahb::test::logging_ports::{Checker, SimpleTestMaster, TestSlave};
use crate::common::new_ahb::vlan::{
    AHBSoftVlanSlavePortInput, AhbDecoderTag, AhbMultiMasterConfig, AhbSlaveOutputDispatcher, Unit,
};
use crate::common::utils::{FromMarker, SubcomponentProxyMut, iter_enum};
use crate::engine::{
    CombFlop, Context, DisableableComponent, Subcomponent, TickComponent, TickComponentExtra,
};
use crate::{
    bridge_ports, build_interconnect, codegen_line_wrapper_for_interconnect,
    decoder_tags_and_markers, make_concrete_dispatcher, make_port_struct, mix_blocks, zip,
};
use enum_map::{EnumMap, enum_map};
use std::fmt::Debug;
use std::ops::Range;

build_interconnect!(
    TestInterconnect
    masters TestMasters => [MasterA, MasterB]
    slaves TestSlaves => [SlaveX, SlaveY]
    using InputStage as input, Decoder=>DPort as decoder, and OPort=>OutputStage as output
);

decoder_tags_and_markers!(@with_markers
    pub(crate) enum TestMasters {
        MasterA,
        MasterB,
    }
);

const X_RANGE: Range<Address> = Address::range_from_len(0x10, 0x10);
const Y_RANGE: Range<Address> = Address::range_from_len(0x20, 0x20);
decoder_tags_and_markers!(@with_dispatcher
    pub(crate) enum TestSlaves {
        SlaveX = X_RANGE,
        SlaveY = Y_RANGE,
    }
);

impl AhbDecoderTag for Decoder<MasterADecoderSC> {
    type Enum = Option<TestSlaves>;
}
impl AhbDecoderTag for Decoder<MasterBDecoderSC> {
    type Enum = Option<TestSlaves>;
}
impl AhbMultiMasterConfig for OutputStage<SlaveXOutputSC> {
    type MastersEnum = TestMasters;
    type Arbiter = RoundRobinArbiter<TestMasters>;
}
impl AhbMultiMasterConfig for OutputStage<SlaveYOutputSC> {
    type MastersEnum = TestMasters;
    type Arbiter = FixedArbiter<TestMasters>;
}
////////////////////////

codegen_line_wrapper_for_interconnect!(TestInterconnect; pub(self));

impl LiteWrapperCfg for LiteWrapper {
    type Data = TestData;
    type InputTag = TestMasters;
    type OutputTag = TestSlaves;
}

// Actual test
macro_rules! impl_ahb_port_config {
    ($(@<$par:ident>)? $id:path) => {
impl$(<$par>)? AHBPortConfig for $id
{
    type Data = TestData;
    type Component = TestComponent;//<Self as Subcomponent>::Component;
    const TAG: &'static str = stringify!($id);
}
    };
    ($($id:path),*) => {
        $(
            impl_ahb_port_config!($id);
        )*
    };
}
impl_ahb_port_config!(LiteOutput<SlaveX>, LiteOutput<SlaveY>);
impl_ahb_port_config!(LiteInput<MasterA>, LiteInput<MasterB>);
impl Checker for XSlaveSC {
    type Data = TestData;

    fn check_and_reply_read(request: &TransferMeta) -> Self::Data {
        Some(TestSlaves::SlaveX)
    }
}

impl Checker for YSlaveSC {
    type Data = TestData;

    fn check_and_reply_read(request: &TransferMeta) -> Self::Data {
        Some(TestSlaves::SlaveY)
    }
}
bridge_ports!(LiteOutput<SlaveX> => @auto_configured TestSlave<XSlaveSC>);
bridge_ports!(LiteOutput<SlaveY> => @auto_configured TestSlave<YSlaveSC>);
type TestData = Option<TestSlaves>;
#[derive(Subcomponent, TickComponent, TickComponentExtra, DisableableComponent, Default)]
struct TestComponent {
    #[subcomponent(LiteWrapper)]
    interconnect: LiteWrapper,

    #[subcomponent(XSlaveSC)]
    x_slave: TestSlave<XSlaveSC>,
    #[subcomponent(YSlaveSC)]
    y_slave: TestSlave<YSlaveSC>,
    #[subcomponent(AMasterSC)]
    am: SimpleTestMaster<AMasterSC>,
    #[subcomponent(BMasterSC)]
    bm: SimpleTestMaster<BMasterSC>,
}

impl TestComponent {
    fn tick(&mut self, ctx: &mut Context) {
        inc_time(ctx, 2);
        #[cfg(debug_assertions)]
        self.tick_assertions_traverse();
        self.tick_flops_and_extra_traverse();

        TestSlave::<YSlaveSC>::tick(self, ctx);
        inc_time(ctx, 1);
        LiteWrapper::tick(self, ctx);
        inc_time(ctx, 1);
        TestSlave::<XSlaveSC>::tick(self, ctx);
        inc_time(ctx, 1);
    }
    fn tock(&mut self, ctx: &mut Context) {
        inc_time(ctx, 2);
        TestSlave::<YSlaveSC>::tock(self, ctx);
        inc_time(ctx, 1);
        LiteWrapper::tock(self, ctx);
        inc_time(ctx, 1);
        TestSlave::<XSlaveSC>::tock(self, ctx);
        inc_time(ctx, 1);
    }
}

bridge_ports!(@auto_configured SimpleTestMaster<AMasterSC> => LiteInput<MasterA>);
bridge_ports!(@auto_configured SimpleTestMaster<BMasterSC> => LiteInput<MasterB>);

use crate::common::new_ahb::test::utils::*;
use rstest::*;

#[fixture]
fn component() -> TestComponent {
    Default::default()
}
#[fixture]
fn context() -> Context {
    Context::new_for_test()
}

#[rstest]
#[test_log::test]
fn idle_loop(mut component: TestComponent, mut context: Context) {
    let ctx = &mut context;
    let comp = &mut component;

    for i in 0..6 {
        println!("--- Tick {i}! ---");
        comp.tick(ctx);
        SimpleTestMaster::<AMasterSC>::send(comp, ctx, make_m2s(idle(), None), false);
        comp.tock(ctx);
        SimpleTestMaster::<BMasterSC>::send(comp, ctx, make_m2s(idle(), None), false);
    }
}

#[rstest]
#[test_log::test]
fn fizzbuzz_noop(mut component: TestComponent, mut context: Context) {
    let ctx = &mut context;
    let comp = &mut component;

    for i in 0..20 {
        println!("--- Tick {i}! ---");
        comp.tick(ctx);
        if i % 3 == 0 {
            SimpleTestMaster::<AMasterSC>::send(comp, ctx, make_m2s(idle(), None), false);
        }
        comp.tock(ctx);
        if i % 5 == 0 {
            SimpleTestMaster::<BMasterSC>::send(comp, ctx, make_m2s(idle(), None), false);
        }
    }
}

#[rstest]
#[test_log::test]
#[case::well_ordered_full(true, false, false)]
#[case::wrapper_on_full(true, true, false)]
#[case::wrapper_on_full_race(true, true, true)]
#[case::wrapper_simple(false, true, false)]
#[case::wrapped_does_actual_job(false, true, true)]
#[should_panic]
#[case::bad_ordered_full(true, false, true)]
#[should_panic]
#[case::dummy(false, false, false)]
fn check_read_loop(
    mut component: TestComponent,
    mut context: Context,
    #[case] master_reflect: bool,
    #[case] wrapper_active: bool,
    #[case] reorder: bool,
    #[values(true, false)] slave_waitstates: bool,
) {
    if slave_waitstates {
        component.x_slave.response_iter = Some(
            [
                SrSuccess, SrPending, SrSuccess, SrSuccess, SrError, SrPending, SrPending,
            ]
            .repeat(5)
            .into_iter(),
        );
        component.y_slave.response_iter = Some(
            [
                SrPending, SrSuccess, SrPending, SrError, SrPending, SrSuccess,
            ]
            .repeat(5)
            .into_iter(),
        );
    }
    let ctx = &mut context;
    let comp = &mut component;
    comp.interconnect.active = wrapper_active;

    for i in 0..20 {
        let a1: u32 = (i * 4 + (i / 7 * i % 7 * 8)) % 0x30;
        let a2 = (i * 4 + 8 + (i / 6 * i % 6 * 4)) % 0x30;
        println!("--- Tick {i}! (A:{a1}, B:{a2})---");
        // This test hit multiple assertions if anything in the code is wrong!
        // prioritize those assert
        #[cfg(debug_assertions)]
        comp.interconnect.tick_assertions();
        comp.tick(ctx);
        // simulate tock reordering
        mix_blocks!(
            reorder,
            {
                comp.tock(ctx);
            },
            {
                SimpleTestMaster::<AMasterSC>::send(
                    comp,
                    ctx,
                    make_m2s(read(a1), None),
                    master_reflect,
                );
            }
        );
        SimpleTestMaster::<BMasterSC>::send(comp, ctx, make_m2s(read(a2), None), master_reflect);
    }

    // Test that we go back to stable state
    for i in 0..20 {
        println!("--- Fizbuzz Tick {i}! ---");
        #[cfg(debug_assertions)]
        comp.interconnect.tick_assertions();
        comp.tick(ctx);
        mix_blocks!(
            reorder,
            comp.tock(ctx),
            if i < 4 || i % 3 == 0 {
                SimpleTestMaster::<AMasterSC>::send(
                    comp,
                    ctx,
                    make_m2s(idle(), None),
                    master_reflect,
                );
            }
        );
        if i < 4 || i % 5 == 0 {
            SimpleTestMaster::<BMasterSC>::send(comp, ctx, make_m2s(idle(), None), master_reflect);
        }
    }
}

// Port tracing tags
const A_TAG: &str = <SimpleTestMaster<AMasterSC> as AHBPortConfig>::TAG;
const B_TAG: &str = <SimpleTestMaster<BMasterSC> as AHBPortConfig>::TAG;
const X_TAG: &str = <TestSlave<XSlaveSC> as AHBPortConfig>::TAG;
const Y_TAG: &str = <TestSlave<YSlaveSC> as AHBPortConfig>::TAG;
/// Any Default Slave
const D_TAG: &str = "*Default*";
/// Any Output Stage
const OS_TAG: &str = "*Output*";
/// Any Input Stage
const IS_TAG: &str = "*Input*";
/// Any Decoder (Stage)
const DS_TAG: &str = "*Decode*";
/// Any output Slave
const S_TAG: &str = "*TestSlave*";
// Wildcard
const W_TAG: &str = "*";

impl MarkerHelpers for TestMasters {
    fn uaddr(self) -> u32 {
        unimplemented!()
    }

    fn tag_str(self) -> &'static str {
        if self == TestMasters::MasterA {
            A_TAG
        } else {
            B_TAG
        }
    }
}

impl MarkerHelpers for Option<TestSlaves> {
    fn uaddr(self) -> u32 {
        match self {
            Some(TestSlaves::SlaveX) => X_RANGE.start.to_const(),
            Some(TestSlaves::SlaveY) => Y_RANGE.start.to_const(),
            None => 0x9990,
        }
    }

    fn tag_str(self) -> &'static str {
        match self {
            Some(TestSlaves::SlaveX) => X_TAG,
            Some(TestSlaves::SlaveY) => Y_TAG,
            None => D_TAG,
        }
    }
}

use crate::common::new_ahb::interconnect::lite_wrapper::LiteWrapperCfg;
use crate::common::new_ahb::signals::AhbResponseControl::*;
use crate::common::new_ahb::slave_driver::SimpleResponse;
use crate::common::new_ahb::test::utils::MarkerHelpers;

#[allow(unused_imports)] // It is used, but some tools don't see that.
use crate::common::new_ahb::test::utils::{idle, make_m2s, read, write};
use crate::test_utils::inc_time;
use crate::utils::Implies;
#[allow(unused_imports)] // It is used, but some tools don't see that.
use crate::{atom_vec, auto_vec};
#[allow(unused_imports)] // It is used, but some tools don't see that.
use Option::None as na;
use TestMasters::{MasterA as A, MasterB as B};
use TestSlaves::{SlaveX as X, SlaveY as Y};

const D: Option<TestSlaves> = None;

fn wr(tag: impl Into<Option<TestSlaves>>) -> MasterToSlaveAddrPhase {
    write(tag.into().uaddr())
}
fn rd(tag: impl Into<Option<TestSlaves>>) -> MasterToSlaveAddrPhase {
    read(tag.into().uaddr())
}

#[rstest]
#[test_log::test]
#[case::easy_with_idles(
/* INJECTX */ auto_vec![na,      na,      SrSuccess, na,      ],
/* ATAG X  */ auto_vec![None,    A_TAG,   A_TAG,     None,    ],
/* INJECTY */ auto_vec![na,      na,      SrSuccess, na,      ],
/* ATAG Y  */ auto_vec![None,    None,    B_TAG,     B_TAG,   ],

/* REQ   A */ auto_vec![wr(X),   idle(),  idle(),    nosel(), ],
/* HWDAT A */ auto_vec![None,    X,       X,         None,    ],
/* RESP A  */ auto_vec![Success, Pending, Success,   Success, ],
/* RTAG A  */ auto_vec![D_TAG,   IS_TAG,  X_TAG,     IS_TAG,  ],

/* REQ   B */ auto_vec![idle(),  rd(Y),   idle(),    idle(),  ],
/* HWDAT B */ auto_vec![None,    None,    None,      None,    ],
/* RESP B  */ auto_vec![Success, Success, Pending,   Success, ],
/* RTAG B  */ auto_vec![D_TAG,   IS_TAG,  IS_TAG,    Y_TAG,   ],
)]
#[case::no_conflict(
/* INJECTX */ auto_vec![na,      na,      SrSuccess, SrPending, SrSuccess, na,        na,      na,     SrSuccess, ],
/* ATAG X  */ auto_vec![None,    A_TAG,   A_TAG,     A_TAG,     A_TAG,     None,      None,    A_TAG,  A_TAG,     ],
/* INJECTY */ auto_vec![na,      na,      na,        na,        SrSuccess, SrPending, SrError, na,     SrSuccess, ],
/* ATAG Y  */ auto_vec![None,    None,    None,      None,      B_TAG,     DS_TAG,    DS_TAG,  B_TAG,  B_TAG,     ],

/* REQ   A */ auto_vec![wr(X),   idle(),  rd(X),     idle(),    nosel(),   nomsg(),   nomsg(), rd(X),  idle(),    ],
/* HWDAT A */ auto_vec![None,    X,       X,         None,      None,      None,      None,    None,   None,      ],
/* RESP A  */ auto_vec![Success, Pending, Success,   Pending,   Success,   None,      None,    None,   Success,   ],
/* RTAG A  */ auto_vec![D_TAG,   IS_TAG,  X_TAG,     X_TAG,     X_TAG,     W_TAG,     IS_TAG,  IS_TAG, X_TAG,     ],

/* REQ   B */ auto_vec![nomsg(), nomsg(), wr(Y),     wr(Y),     wr(Y),     rd(D),     rd(D),   wr(Y),  idle(),    ],
/* HWDAT B */ auto_vec![None,    None,    None,      Y,         Y,         Y,         Y,       None,   Y,         ],
/* RESP B  */ auto_vec![Success, None,    None,      Pending,   Success,   Pending,   Error1,  Error2, Success,   ],
/* RTAG B  */ auto_vec![D_TAG,   IS_TAG,  IS_TAG,    IS_TAG,    Y_TAG,     Y_TAG,     Y_TAG,   Y_TAG,  Y_TAG,     ],
)]
#[case::conflict_on_rr(
/* INJECTX */ auto_vec![na,      na,      SrSuccess, SrSuccess, SrPending, SrSuccess, SrSuccess, SrSuccess, SrSuccess, SrSuccess, na,     ],
/* ATAG X  */ auto_vec![None,    A_TAG,   A_TAG,     B_TAG,     B_TAG,     B_TAG,     A_TAG,     B_TAG,     A_TAG,     DS_TAG,    None,   ],
/* INJECTY */ auto_vec![na,      na,      na,        na,        na,        na,        na,        na,        na,        na,        na,     ],
/* ATAG Y  */ auto_vec![None,    None,    None,      None,      None,      None,      None,      None,      None,      None,      None,   ],

/* REQ   A */ auto_vec![wr(X),   idle(),  rd(X),     idle(),    wr(X),     rd(X),     rd(X),     wr(X),     wr(D),     wr(D),     idle(), ],
/* HWDAT A */ auto_vec![None,    X,       X,         None,      None,      X,         X,         None,      X,         X,         D,      ],
/* RESP A  */ auto_vec![Success, Pending, Success,   Success,   Success,   Pending,   Pending,   Success,   Pending,   Success,   Error1, ],
/* RTAG A  */ auto_vec![D_TAG,   IS_TAG,  X_TAG,     X_TAG,     IS_TAG,    IS_TAG,    IS_TAG,    X_TAG,     IS_TAG,    X_TAG,     D_TAG,  ],

/* REQ   B */ auto_vec![nomsg(), nomsg(), wr(X),     rd(X),     rd(X),     rd(X),     wr(X),     wr(X),     wr(D),     idle(),    idle(), ],
/* HWDAT B */ auto_vec![None,    None,    None,      X,         X,         X,         None,      X,         X,         D,         None,   ],
/* RESP B  */ auto_vec![Success, None,    None,      Pending,   Pending,   Success,   Success,   Pending,   Success,   Error1,    Error2, ],
/* RTAG B  */ auto_vec![D_TAG,   IS_TAG,  IS_TAG,    IS_TAG,    X_TAG,     X_TAG,     X_TAG,     IS_TAG,    X_TAG,     D_TAG,     D_TAG,  ],
)]
#[case::conflict_on_fixed( // currently in reset in the fixed arbiter we allow the first one to get without a waitstate (not a null-default)
/* INJECTX */ auto_vec![na,      /*na,     */ na,        na,        na,      na,        na,        na,        na,      na,        na,        na,     ],
/* ATAG X  */ auto_vec![None,    /*None,   */ None,      None,      None,    None,      None,      None,      None,    None,      None,      None,   ],
/* INJECTY */ auto_vec![na,      /*na,     */ SrSuccess, SrSuccess, na,      SrSuccess, SrSuccess, SrSuccess, na,      SrSuccess, SrSuccess, na,     ],
/* ATAG Y  */ auto_vec![None,    /*A_TAG,  */ A_TAG,     A_TAG,     B_TAG,   A_TAG,     A_TAG,     A_TAG,     B_TAG,   A_TAG,     DS_TAG,    None,   ],

/* REQ   A */ auto_vec![wr(Y),   /*idle(), */ rd(Y),     idle(),    wr(Y),   rd(Y),     rd(Y),     idle(),    wr(Y),   rd(Y),     wr(D),     wr(D),  ],
/* HWDAT A */ auto_vec![None,    /*Y,      */ Y,         None,      None,    Y,         Y,         None,      None,    Y,         Y,         D,      ],
/* RESP A  */ auto_vec![Success, /*Pending,*/ Success,   Success,   Success, Pending,   Success,   Success,   Success, Pending,   Success,   Error1, ],
/* RTAG A  */ auto_vec![D_TAG,   /*IS_TAG, */ Y_TAG,     Y_TAG,     IS_TAG,  IS_TAG,    Y_TAG,     Y_TAG,     IS_TAG,  IS_TAG,    Y_TAG,     D_TAG,  ],

/* REQ   B */ auto_vec![nomsg(), /*nomsg(),*/ wr(Y),     rd(Y),     rd(Y),   rd(Y),     wr(Y),     wr(Y),     wr(Y),   wr(D),     rd(Y),     wr(Y),  ],
/* HWDAT B */ auto_vec![None,    /*None,   */ None,      Y,         Y,       Y,         None,      None,      None,    None,      None,      None,   ],
/* RESP B  */ auto_vec![Success, /*Success,*/ None,      Pending,   Pending, Success,   Pending,   Pending,   Pending, Success,   Error1,    Error2, ],
/* RTAG B  */ auto_vec![D_TAG,   /*IS_TAG, */ IS_TAG,    IS_TAG,    IS_TAG,  Y_TAG,     IS_TAG,    IS_TAG,    IS_TAG,  Y_TAG,     D_TAG,     D_TAG,  ],
)]
#[case::not_ready_propagation_from_changing_decoder( // decoder has low HREADY in data phase, must pass it to addr phase wires
/* INJECTX */ auto_vec![na,      SrSuccess,na,      na,         na,         na,/*nSel*/ SrPending,SrSuccess, na,        ],
/* ATAG X  */ auto_vec![None,    A_TAG,   DS_TAG,   None,       None,       A_TAG,      DS_TAG,   DS_TAG,    None,      ],
/* INJECTY */ auto_vec![na,      na,      na,       SrSuccess,  SrPending,  SrSuccess,  na,       na,        SrSuccess, ],
/* ATAG Y  */ auto_vec![None,    None,    None,     A_TAG,      DS_TAG,     DS_TAG,     None,     A_TAG,     A_TAG,     ],
                        /* RR arbiter needs initialization to grant us wires */
/* REQ   A */ auto_vec![wr(X),   wr(Y),   wr(Y),    rd(Y),      wr(X),      wr(X),      rd(Y),    rd(Y),     idle(),    ],
/* HWDAT A */ auto_vec![None,    X,       None,     Y,          None,       None,       X,        X,         None,      ],
/* RESP A  */ auto_vec![Success, Pending, Success,  Success,    Pending,    Success,    Pending,  Success,   Success,   ],
/* RTAG A  */ auto_vec![D_TAG,   IS_TAG,  X_TAG,    Y_TAG,      Y_TAG,      Y_TAG,      X_TAG,    X_TAG,     Y_TAG,     ],

/* REQ   B */ auto_vec![nomsg(), nomsg(), nomsg(),  nomsg(),    nomsg(),    nomsg(),    nomsg(),  nomsg(),   nomsg(),   ],
/* HWDAT B */ auto_vec![None,    None,    None,     None,       None,       None,       None,     None,      None,      ],
/* RESP B  */ auto_vec![Success, None,    None,     None,       None,       None,       None,     None,      None,      ],
/* RTAG B  */ auto_vec![D_TAG,   IS_TAG,  IS_TAG,   IS_TAG,     IS_TAG,     IS_TAG,     IS_TAG,   IS_TAG,    IS_TAG,    ],
)]
#[case::output_stage_with_waitstate_but_rdy_input( // the output stage has different data phase master than addr phase master
/* INJECTX */ auto_vec![na,      na,      SrPending, SrSuccess, SrPending, SrSuccess, SrSuccess,  SrSuccess, ],
/* ATAG X  */ auto_vec![None,    A_TAG,   B_TAG,     B_TAG,     B_TAG,     B_TAG,     A_TAG,      B_TAG,     ],
/* INJECTY */ auto_vec![na,      na,      na,        na,        na,        na,        na,         na,        ],
/* ATAG Y  */ auto_vec![None,    None,    None,      None,      None,      None,      None,       None,      ],

/* REQ   A */ auto_vec![wr(X),   idle(),  idle(),    idle(),    wr(X),     rd(X),     rd(X),      rd(X),     ],
/* HWDAT A */ auto_vec![None,    X,       X,         X,         None,      X,         X,          None,      ],
/* RESP A  */ auto_vec![Success, Pending, Pending,   Success,   Success,   Pending,   Pending,    Success,   ],
/* RTAG A  */ auto_vec![D_TAG,   IS_TAG,  X_TAG,     X_TAG,     IS_TAG,    IS_TAG,    IS_TAG,     X_TAG,     ],
                                         /* wr is in input stage */
/* REQ   B */ auto_vec![wr(X),   rd(X),   rd(X),     rd(X),     rd(X),     rd(X),     wr(X),      idle(),    ],
/* HWDAT B */ auto_vec![None,    X,       X,         X,         X,         X,         None,       X,         ],
/* RESP B  */ auto_vec![Success, Pending, Pending,   Pending,   Pending,   Success,   Success,    Pending,   ],
/* RTAG B  */ auto_vec![D_TAG,   IS_TAG,  IS_TAG,    IS_TAG,    X_TAG,     X_TAG,     X_TAG,      IS_TAG,    ],
)]
fn test_interleaving(
    #[case] injected_x: Vec<Option<SimpleResponse<()>>>,
    #[case] addr_tag_x: Vec<Option<&'static str>>,
    #[case] injected_y: Vec<Option<SimpleResponse<()>>>,
    #[case] addr_tag_y: Vec<Option<&'static str>>,

    #[case] requests_a: Vec<Option<MasterToSlaveAddrPhase>>,
    #[case] requests_a_data: Vec<TestData>,
    #[case] responses_a_meta: Vec<Option<AhbResponseControl>>,
    #[case] responders_a: Vec<&'static str>,

    #[case] requests_b: Vec<Option<MasterToSlaveAddrPhase>>,
    #[case] requests_b_data: Vec<TestData>,
    #[case] responses_b_meta: Vec<Option<AhbResponseControl>>,
    #[case] responders_b: Vec<&'static str>,
    #[values(true, false)] reorder: bool,
    mut context: Context,
    mut component: TestComponent,
) {
    #[cfg(debug_assertions)]
    {
        assert_eq!(requests_a.len(), injected_x.len(), "Malformed test input");
        assert_eq!(requests_a.len(), injected_y.len(), "Malformed test input");
        assert_eq!(
            requests_a.len(),
            requests_a_data.len(),
            "Malformed test input"
        );
        assert_eq!(
            requests_a.len(),
            responses_a_meta.len(),
            "Malformed test input"
        );
        assert_eq!(requests_a.len(), responders_a.len(), "Malformed test input");
        assert_eq!(requests_a.len(), requests_b.len(), "Malformed test input");
        assert_eq!(
            requests_b.len(),
            requests_b_data.len(),
            "Malformed test input"
        );
        assert_eq!(
            requests_b.len(),
            requests_b_data.len(),
            "Malformed test input"
        );
        assert_eq!(
            requests_b.len(),
            responses_b_meta.len(),
            "Malformed test input"
        );
        assert_eq!(requests_b.len(), responders_b.len(), "Malformed test input");
    }

    component.x_slave.response_iter = Some(
        injected_x
            .into_iter()
            .flatten()
            .collect::<Vec<_>>()
            .into_iter(),
    );
    component.y_slave.response_iter = Some(
        injected_y
            .into_iter()
            .flatten()
            .collect::<Vec<_>>()
            .into_iter(),
    );
    let ctx = &mut context;
    let comp = &mut component;

    let a_zip = zip!(requests_a, requests_a_data, responses_a_meta, responders_a);
    let b_zip = zip!(requests_b, requests_b_data, responses_b_meta, responders_b);
    for (i, (params_a, (params_b, (atag_x, atag_y)))) in
        zip!(a_zip, b_zip, addr_tag_x, addr_tag_y).enumerate()
    {
        println!(
            "\n --> Cycle {} for A:{:?} B:{:?}",
            i + 1,
            params_a,
            params_b
        );
        let (req_a, (req_a_data, (resp_a, from_a))) = params_a;
        let (req_b, (req_b_data, (resp_b, from_b))) = params_b;

        comp.tick(ctx);
        mix_blocks! {reorder, {
        comp.tock(ctx);
        resp_a.implies_then(
            comp.am.last_resp.take(),
            |resp_a, recv_a| {
                #[cfg(feature = "cycle-debug-logger")]
                assert_eq!(recv_a.responder_tag, from_a, "Tag mismatch for {A:?}");
                assert_eq!(recv_a.meta, resp_a, "Resp mismatch for {A:?}");
            },
            |resp_a| panic!("{A:?} Expected reply msg {resp_a:?}"),
        );
        resp_b.implies_then(
            comp.bm.last_resp.take(),
            |resp_b, recv_b| {
                #[cfg(feature = "cycle-debug-logger")]
                assert_eq!(recv_b.responder_tag, from_b, "Tag mismatch for {B:?}");
                assert_eq!(recv_b.meta, resp_b, "Resp mismatch for {B:?}");
            },
            |resp_b| panic!("{B:?} Expected reply msg {resp_b:?}"),
        );
        }, {
        if let Some(req_a) = req_a {
            let m2s_a = make_m2s(req_a, req_a_data).tagged(A.tag_str());
            // XX: note that last_resp is taken when changing master reflection
            SimpleTestMaster::<AMasterSC>::send(comp, ctx, m2s_a, false);
        }
        }
        }

        if let Some(req_b) = req_b {
            let m2s_b = make_m2s(req_b, req_b_data).tagged(B.tag_str());
            SimpleTestMaster::<BMasterSC>::send(comp, ctx, m2s_b, false);
        }

        #[cfg(feature = "cycle-debug-logger")]
        atag_x.implies_then(
            comp.x_slave.last_input.take(),
            |addr_tag, recv| assert_eq!(recv.addr_phase.tag, addr_tag, " for {X:?}"),
            |t| panic!("Expected msg tagged {t:?} on {X:?}"),
        );

        #[cfg(feature = "cycle-debug-logger")]
        atag_y.implies_then(
            comp.y_slave.last_input.take(),
            |addr_tag, recv_y| assert_eq!(recv_y.addr_phase.tag, addr_tag, " for {Y:?}"),
            |t| panic!("Expected msg tagged {t:?} on {Y:?}"),
        );
    }
}
