#![allow(clippy::too_many_arguments)]
use crate::common::new_ahb::line_buffer::{LineBuffer, LineBufferCfg};
use crate::common::new_ahb::ports::AHBPortConfig;
use crate::common::new_ahb::signals::AhbResponseControl::Success;
use crate::common::new_ahb::signals::AhbResponseControl::*;
use crate::common::new_ahb::signals::Size::*;
use crate::common::new_ahb::signals::{
    AhbResponseControl, MasterToSlaveAddrPhase, MasterToSlaveWires, TransferMeta,
};
use crate::common::new_ahb::slave_driver::SimpleResponse;
use crate::common::new_ahb::test::logging_ports::*;
use crate::common::new_ahb::test::utils::ReqType::*;
use crate::common::new_ahb::test::utils::make_m2s;
use crate::common::new_ahb::test::utils::*;
use crate::engine::{
    Context, DisableableComponent, Subcomponent, TickComponent, TickComponentExtra,
};
use crate::test_utils::inc_time;
#[allow(unused_imports)]
use crate::{atom_vec, auto_vec};
use crate::{bridge_ports, mix_blocks, zip};

use rstest::{fixture, rstest};

type Master = SimpleTestMaster<MasterSC>;
type Slave = TestSlave<SlaveSC>;
type LB = LineBuffer<WriteBufferSC>;

#[derive(Subcomponent, TickComponent, TickComponentExtra, DisableableComponent)]
struct TestComponent {
    #[subcomponent(MasterSC)]
    m: Master,

    #[subcomponent(SlaveSC)]
    s: Slave,

    #[subcomponent(WriteBufferSC)]
    lb: LB,
}

impl TestComponent {
    fn tick(&mut self, ctx: &mut Context) {
        #[cfg(debug_assertions)]
        self.tick_assertions_traverse();
        self.tick_flops_and_extra_traverse();
        Slave::tick(self, ctx);
        LB::tick(self, ctx);
    }

    fn tock(&mut self, ctx: &mut Context) {
        Slave::tock(self, ctx);
        LB::tock(self, ctx);
    }
}

impl AHBPortConfig for Master {
    type Data = DataBus;
    type Component = TestComponent;
    const TAG: &'static str = "Master";
}

bridge_ports!(Master => @auto_configured LB);
bridge_ports!(LB => @auto_configured Slave);

impl LineBufferCfg for LB {
    const UPSIZED: Size = Word;
    const ENABLED_BY_DEFAULT: bool = true;
    const MASKS_CACHEABLE: bool = true;

    fn extract_upstream_from_upsized(
        addr: &TransferMeta,
        data: &<Self as AHBPortConfig>::Data,
    ) -> <Self as AHBPortConfig>::Data {
        data.clone().extract_from_aligned(addr.addr, addr.size)
    }
}

impl Checker for SlaveSC {
    type Data = DataBus;

    fn check_and_reply_read(request: &TransferMeta) -> Self::Data {
        match request {
            TransferMeta {
                size: Word, addr, ..
            } if *addr == Address::from_const(12) => DataBus::Word(0x1234_5678),
            TransferMeta {
                size: Word, addr, ..
            } if *addr == Address::from_const(16) => DataBus::Word(0xaabb_ccdd),
            TransferMeta {
                size: Byte,
                addr,
                prot,
                ..
            } if !prot.is_cacheable => DataBus::Byte(
                0x1234_5678u32.to_le_bytes()
                    [addr.offset_from(addr.aligned_down_to_4_bytes()) as usize],
            ),
            _ => panic!("Unexpected transfer meta: {request:#?}"),
        }
    }
}

#[fixture]
fn component() -> TestComponent {
    TestComponent {
        m: Default::default(),
        s: Default::default(),
        lb: LB::new(),
    }
}

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
/// Read non-cacheable
fn read_nc(addr: u32) -> MasterToSlaveAddrPhase {
    let mut addr = read(addr);
    let Some(meta) = addr.meta.meta_mut() else {
        unreachable!()
    };
    meta.prot = meta.prot.with_cacheable(false);
    addr
}

use crate::common::new_ahb::{DataBus, Size};
use cmemu_common::Address;
#[allow(unused_imports)]
use std::option::Option::None as na;

#[allow(non_upper_case_globals)]
const null: DataBus = DataBus::HighZ;

#[rstest]
#[test_log::test]
#[case::easy_no_change(
/* INJECT  */ auto_vec![na,         SrSuccess,     na,        ],
/* REQ   M */ auto_vec![read(12),   idle(),        nosel(),   ],
/* SIZE  M */ auto_vec![Word,       na,            na,        ],
/* REQ @ S */ auto_vec![Read,       Idle,          NoSel,     ],
/* ADDR @S */ auto_vec![12,         na,            na,        ],
/* RESPONS */ auto_vec![Success,    Success,       Success,   ],
/* RESP DT */ auto_vec![null,       0x1234_5678u32,null,      ],
)]
#[case::easy_just_extract(
/* INJECT  */ auto_vec![na,         SrSuccess,     na,        na,        ],
/* REQ   M */ auto_vec![read(13),   idle(),        read(14),  idle(),    ],
/* SIZE  M */ auto_vec![Byte,       na,            Halfword,  na,        ],
/* REQ @ S */ auto_vec![Read,       Idle,          NoMsg,     Idle,      ],
/* ADDR @S */ auto_vec![12,         na,            na,        na,        ],
/* RESPONS */ auto_vec![Success,    Success,       Success,   Success,   ],
/* RESP DT */ auto_vec![null,       0x56u8,        null,      0x1234u16, ],
)]
#[case::just_extract_pipelined(
/* INJECT  */ auto_vec![na,         SrSuccess,     na,        na,        ],
/* REQ   M */ auto_vec![read(13),   read(14),      read(12),  idle(),    ],
/* SIZE  M */ auto_vec![Byte,       Halfword,      Word,      na,        ],
/* REQ @ S */ auto_vec![Read,       Idle,          NoMsg,     Idle,      ],
/* ADDR @S */ auto_vec![12,         na,            na,        na,        ],
/* RESPONS */ auto_vec![Success,    Success,       Success,   Success,   ],
/* RESP DT */ auto_vec![null,       0x56u8,        0x1234u16, 0x1234_5678u32, ],
)]
#[case::just_extract_waitstates(
/* INJECT  */ auto_vec![na,         SrPending,     SrSuccess, na,        ],
/* REQ   M */ auto_vec![read(13),   read(14),      read(14),  idle(),    ],
/* SIZE  M */ auto_vec![Byte,       Halfword,      Halfword,  na,        ],
/* REQ @ S */ auto_vec![Read,       Idle,          Idle,      Idle,      ],
/* ADDR @S */ auto_vec![12,         na,            na,        na,        ],
/* RESPONS */ auto_vec![Success,    Pending,       Success,   Success,   ],
/* RESP DT */ auto_vec![null,       null,          0x56u8,    0x1234u16, ],
)]
#[case::thrashing_nows(
/* INJECT  */ auto_vec![na,         SrSuccess,     SrSuccess, SrSuccess, SrSuccess, ],
/* REQ   M */ auto_vec![read(13),   read(17),      read(15),  read(16),  read(17),  ],
/* SIZE  M */ auto_vec![Byte,       Byte,          Byte,      Byte,      Byte,      ],
/* REQ @ S */ auto_vec![Read,       Read,          Read,      Read,      Idle,      ],
/* ADDR @S */ auto_vec![12,         16,            12,        16,        na,        ],
/* RESPONS */ auto_vec![Success,    Success,       Success,   Success,   Success,   ],
/* RESP DT */ auto_vec![null,       0x56u8,        0xccu8,    0x12u8,    0xddu8,    ],
)]
#[case::thrashing_ws(  // Some REQ@S are marked as Any as they depend on order (reflecting HREADY)
/* INJECT  */ auto_vec![na,         SrPending,     SrSuccess,   SrPending,   SrSuccess, SrSuccess, SrSuccess, ],
/* REQ   M */ auto_vec![read(13),   read(17),      read(17),    read(15),    read(15),  read(16),  read(17),  ],
/* SIZE  M */ auto_vec![Byte,       Byte,          Byte,        Byte,        Byte,      Byte,      Byte,      ],
/* REQ @ S */ auto_vec![Read,       Any,           Read,        Any,         Read,      Read,      Idle,      ],
/* ADDR @S */ auto_vec![12,         16,            16,          12,          12,        16,        na,        ],
/* RESPONS */ auto_vec![Success,    Pending,       Success,     Pending,     Success,   Success,   Success,   ],
/* RESP DT */ auto_vec![null,       null,          0x56u8,      null,        0xccu8,    0x12u8,    0xddu8,    ],
)]
#[case::noncacheable_passthough(
/* INJECT  */ auto_vec![na,       SrSuccess,   SrSuccess, na,          SrPending, SrSuccess, na,       ],
/* REQ   M */ auto_vec![read(13), read_nc(30), read(15),  read_nc(15), read(15),  read(15),  idle(),  ],
/* SIZE  M */ auto_vec![Byte,     Byte,        Byte,      Byte,        Byte,      Byte,      na,      ],
/* REQ @ S */ auto_vec![Read,     Read,        Idle,      Read,        Idle,      Idle,      Idle,    ],
/* ADDR @S */ auto_vec![12,       30,          na,        15,          na,        na,        na,      ],
/* RESPONS */ auto_vec![Success,  Success,     Success,   Success,     Pending,   Success,   Success, ],
/* RESP DT */ auto_vec![null,     0x56u8,      0x34u8,    0x12u8,      null,      0x12u8,    0x12u8,  ],
)]
#[should_panic = "not implemented"]
#[case::write_though(
/* INJECT  */ auto_vec![na,         SrSuccess,     ],
/* REQ   M */ auto_vec![read(12),   write(14),     ],
/* SIZE  M */ auto_vec![Byte,       Byte,          ],
/* REQ @ S */ auto_vec![Read,       Write,         ],
/* ADDR @S */ auto_vec![12,         14,            ],
/* RESPONS */ auto_vec![Success,    Success,       ],
/* RESP DT */ auto_vec![null,       0x78u8,        ],
)]

fn vec_test(
    #[case] injected_responses: Vec<Option<SimpleResponse<()>>>,
    #[case] requests_meta: Vec<Option<MasterToSlaveAddrPhase>>,
    #[case] requests_size: Vec<Option<Size>>,
    #[case] slave_meta: Vec<ReqType>,
    #[case] slave_addr: Vec<Option<u32>>,
    #[case] responses_meta: Vec<AhbResponseControl>,
    #[case] responses_data: Vec<DataBus>,
    mut context: Context,
    mut component: TestComponent,
    #[values(false, true)] reorder: bool,
) {
    assert_eq!(
        requests_meta.len(),
        slave_meta.len(),
        "Malformed test input"
    );
    assert_eq!(
        requests_meta.len(),
        requests_size.len(),
        "Malformed test input"
    );
    assert_eq!(slave_meta.len(), slave_addr.len(), "Malformed test input");
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

    for (req, (r_size, (s_req, (s_addr, (resp, resp_data))))) in zip!(
        requests_meta,
        requests_size,
        slave_meta,
        slave_addr,
        responses_meta,
        responses_data
    ) {
        let req = req.map(|r| {
            let mut r = r.tagged("Master");
            if let Some(m) = r.meta.meta_mut() {
                m.size = r_size.unwrap();
            }
            r
        });
        println!("\nCycle for {req:?} + expecting {resp:?} w/ {resp_data:?}");

        inc_time(ctx, 5);
        comp.tick(ctx);

        inc_time(ctx, 5);
        mix_blocks!(
            reorder,
            if let Some(req) = req {
                Master::send(comp, ctx, make_m2s(req, DataBus::HighZ), true);
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
            if s_req != Any || addr_phase.meta.is_address_valid() {
                assert_eq!(addr_phase.meta.address(), s_addr.map(|a| a.into()));
            }
        }

        let m_resp = comp.m.last_resp.take();
        assert_eq!(m_resp.as_ref().map_or(Success, |r| r.meta), resp);
        if let Some(m_resp) = m_resp {
            assert_eq!(m_resp.data, resp_data);
        }
    }
}
