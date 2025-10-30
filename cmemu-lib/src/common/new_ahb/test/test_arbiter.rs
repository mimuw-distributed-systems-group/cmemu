use crate::common::Address;
use crate::common::new_ahb::arbiter::{Arbiter, FixedArbiter, RoundRobinArbiter};
use crate::common::new_ahb::signals::{
    BinaryWire, Burst, Direction, MasterToSlaveAddrPhase, TransferMeta, TransferType,
};
#[allow(unused_imports)] // It is used, but some tools don't see that.
use crate::common::new_ahb::test::utils::{idle, make_m2s, read, write};
use crate::decoder_tags_and_markers;
use crate::test_utils::zip;
#[allow(unused_imports)] // It is used, but some tools don't see that.
use crate::{atom_vec, auto_vec};
use Burst::*;
use enum_map::{EnumMap, enum_map};
use rstest::*;
use std::fmt::{Debug, Formatter};

decoder_tags_and_markers!(@with_markers
   enum Tag {
        A_,
        B_,
        C_,
        D_,
    }
);
#[allow(unused_imports)]
use Tag::{A_ as A, B_ as B, C_ as C, D_ as D};

struct TSet(EnumMap<Tag, bool>);
impl From<Vec<Tag>> for TSet {
    fn from(v: Vec<Tag>) -> Self {
        Self(enum_map! {t => v.contains(&t)})
    }
}
impl From<TSet> for EnumMap<Tag, bool> {
    fn from(t: TSet) -> Self {
        t.0
    }
}
impl Debug for TSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("TSet(")?;
        for (k, v) in self.0 {
            if v {
                f.write_fmt(format_args!("{k:?}, "))?;
            }
        }
        f.write_str(")")
    }
}

#[rstest]
#[case::fixed(fixed_arb())]
#[case::round(rr_arb())]
fn test_no_op(#[case] mut arb: impl Arbiter<Tag>) {
    for _ in 0..5 {
        #[cfg(debug_assertions)]
        arb.tick_assertions_traverse();
        arb.tick_flops_and_extra_traverse();
        arb.arbitrate(enum_map! {_ => false}, MasterToSlaveAddrPhase::default());
    }
}

#[fixture]
fn fixed_arb() -> FixedArbiter<Tag> {
    Default::default()
}

#[fixture]
fn rr_arb() -> RoundRobinArbiter<Tag> {
    Default::default()
}

fn make_a<F>(f: F, burst: Burst, lock: BinaryWire, ready: BinaryWire) -> MasterToSlaveAddrPhase
where
    F: FnOnce(TransferMeta) -> TransferType,
{
    let base = MasterToSlaveAddrPhase::nonseq(Address::from_const(0), Direction::Read);
    MasterToSlaveAddrPhase {
        meta: f(TransferMeta {
            burst,
            ..base.meta.meta().unwrap().clone()
        }),
        lock,
        ready,
        ..base
    }
}

fn nonseq(burst: Burst, lock: BinaryWire, ready: BinaryWire) -> MasterToSlaveAddrPhase {
    make_a(TransferType::NonSeq, burst, lock, ready)
}
fn seq(burst: Burst, lock: BinaryWire, ready: BinaryWire) -> MasterToSlaveAddrPhase {
    make_a(TransferType::Seq, burst, lock, ready)
}

#[rstest]
#[case::simple(fixed_arb(),
auto_vec![vec![], vec![A], vec![B], vec![C, D]],
auto_vec![A, A, B, C], // FIXME: In theory, a startup arbiter shouldn't grant access, but it was needed somewhere.
auto_vec![idle(), idle(), idle(), idle()]
)]
#[case::fixed(fixed_arb(),
auto_vec![vec![A, B], vec![A, B], vec![], vec![]],
auto_vec![A, A, A, A],
auto_vec![idle(), idle(), idle(), idle()]
)]
#[case::fixed_single_master(fixed_arb(),
auto_vec![vec![C], vec![C], vec![C], vec![B, C], vec![]],
auto_vec![C, C, C, B, B],
auto_vec![idle(), read(0), read(0), read(0), read(0)]
)]
#[case::fixed_done(fixed_arb(),
auto_vec![vec![A, B], vec![A, B], vec![B], vec![B], vec![]],
auto_vec![A, A, B, B, B],
auto_vec![idle(), idle(), idle(), idle(), idle()]
)]
#[case::fixed_interrupt(fixed_arb(),
auto_vec![vec![B], vec![A, B], vec![B], vec![B], vec![]],
auto_vec![B, A, B, B, B],
auto_vec![idle(), read(0), idle(), idle(), idle()]
)]
#[case::fixed_no_interrupt_locked(fixed_arb(),
auto_vec![vec![B], vec![A, B], vec![A, B], vec![A, B], vec![]],
auto_vec![B, B, B, A, A],
auto_vec![idle(), nonseq(Single, true, true), nonseq(Single, true, true), idle(), idle()]
)]
#[case::fixed_keep(fixed_arb(),
auto_vec![vec![A, B], vec![A, B], vec![B], vec![B]],
auto_vec![A, A, A, B],
auto_vec![idle(), read(0), read(0), idle()]
)]
#[case::robin(rr_arb(),
auto_vec![vec![A, B], vec![A, B], vec![A, B, D], vec![A, B], vec![]],
auto_vec![A, B, D, A, A],
auto_vec![idle(), idle(), idle(), idle(), idle()]
)]
#[case::robin_single_master(rr_arb(),
auto_vec![vec![C], vec![C], vec![C], vec![B, C], vec![]],
auto_vec![C, C, C, B, None],
auto_vec![idle(), read(0), read(0), read(0), read(0)]
)]
#[case::robin_start_from_last(rr_arb(),
auto_vec![vec![D], vec![D], vec![C], vec![B, C], vec![]],
auto_vec![D, D, C, B, None],
auto_vec![idle(), read(0), read(0), read(0), read(0)]
)]
#[case::robin_no_keep(rr_arb(),
auto_vec![vec![A, B], vec![A, B], vec![A, B, D], vec![A, B], vec![]],
auto_vec![A, B, D, A, None],
auto_vec![idle(), read(0), read(0), read(0), read(0)]
)]
#[case::robin_full_roll(rr_arb(),
auto_vec![vec![A, B, C, D], vec![A, B, C, D], vec![A, B, C, D], vec![A, B, C, D], vec![A, B, C, D], vec![A, B, C, D], ],
auto_vec![A, B, C, D, A, B],
auto_vec![idle(), read(0), read(0), read(0), read(0), read(0)]
)]
#[case::robin_back_to_no_port(rr_arb(),
auto_vec![vec![A, B], vec![], vec![], ],
auto_vec![A, None, None],
auto_vec![idle(), read(0), read(0),]
)]
#[case::robin_no_interrupt_locked(rr_arb(),
auto_vec![vec![B], vec![A, B], vec![A, B], vec![A, B], vec![]],
auto_vec![B, B, B, A, A],
auto_vec![idle(), nonseq(Single, true, true), nonseq(Single, true, true), idle(), idle()]
)]
fn arbitration(
    #[case] mut arb: impl Arbiter<Tag>,
    #[case] reqs_iter: Vec<TSet>,
    #[case] expected_iter: Vec<Option<Tag>>,
    #[case] addr_iter: Vec<MasterToSlaveAddrPhase>,
) {
    assert_eq!(reqs_iter.len(), expected_iter.len(), "Test malformed");
    assert_eq!(reqs_iter.len(), addr_iter.len(), "Test malformed");

    for (reqs, (expected, addr_phase)) in zip!(reqs_iter, expected_iter, addr_iter) {
        println!("Cycle for {reqs:?} expect {expected:?}");
        #[cfg(debug_assertions)]
        arb.tick_assertions_traverse();
        arb.tick_flops_and_extra_traverse();
        let res = arb.arbitrate(reqs.into(), addr_phase);
        assert_eq!(res, expected);
    }
}
