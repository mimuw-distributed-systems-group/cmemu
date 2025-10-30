#![allow(non_upper_case_globals)]

use crate::common::Address;
#[cfg(feature = "cycle-debug-logger")]
use crate::common::new_ahb::cdl::CdlTag;
use crate::common::new_ahb::ports::AHBPortConfig;
use crate::common::new_ahb::signals::wire::LOW;
use crate::common::new_ahb::signals::*;
use crate::common::new_ahb::slave_driver::SimpleResponse;

#[must_use]
pub(crate) fn read(addr: u32) -> MasterToSlaveAddrPhase {
    MasterToSlaveAddrPhase::nonseq(Address::from(addr), Direction::Read)
}

#[must_use]
pub(crate) fn write(addr: u32) -> MasterToSlaveAddrPhase {
    MasterToSlaveAddrPhase::nonseq(Address::from(addr), Direction::Write)
}

#[must_use]
pub(crate) fn idle() -> MasterToSlaveAddrPhase {
    MasterToSlaveAddrPhase::idle()
}

#[must_use]
pub(crate) fn nosel() -> MasterToSlaveAddrPhase {
    MasterToSlaveAddrPhase::not_selected::<UnknownPort>()
}

#[must_use]
pub(crate) fn nomsg() -> Option<MasterToSlaveAddrPhase> {
    None
}

#[must_use]
pub(crate) fn make_m2s<D: Default>(ap: MasterToSlaveAddrPhase, dp: D) -> MasterToSlaveWires<D> {
    MasterToSlaveWires {
        data_phase: MasterToSlaveDataPhase {
            data: dp,
            #[cfg(feature = "cycle-debug-logger")]
            tag: ap.tag.clone(),
        },
        addr_phase: ap,
    }
}

#[must_use]
pub(crate) fn make_s2m<D: Default>() -> SlaveToMasterWires<D> {
    SlaveToMasterWires::empty::<UnknownPort>()
}

#[must_use]
pub(crate) fn make_s2m_from<D: Default>(
    d: D,
    req: &MasterToSlaveWires<D>,
) -> SlaveToMasterWires<D> {
    SlaveToMasterWires {
        meta: AhbResponseControl::Success,
        data: d,
        ..SlaveToMasterWires::empty_reply::<UnknownPort>(&req.data_phase)
    }
}

#[must_use]
pub(crate) fn make_s2m_from_resp<P: AHBPortConfig, D: Default>(
    d: D,
    req: &MasterToSlaveWires<D>,
) -> SlaveToMasterWires<D> {
    SlaveToMasterWires {
        meta: AhbResponseControl::Success,
        data: d,
        ..SlaveToMasterWires::empty_reply::<P>(&req.data_phase)
    }
}

// Useful alias names
pub(crate) const SrSuccess: SimpleResponse<()> = SimpleResponse::SUCCESS;
pub(crate) const SrPending: SimpleResponse<()> = SimpleResponse::Pending;
pub(crate) const SrError: SimpleResponse<()> = SimpleResponse::Error;

#[must_use]
pub(crate) fn reflect_hready<D: Default>(
    mut msg: MasterToSlaveWires<D>,
    resp: Option<&SlaveToMasterWires<D>>,
) -> MasterToSlaveWires<D> {
    if let Some(resp) = resp.as_ref() {
        msg.addr_phase.ready = resp.meta.HREADYOUT();
    }
    msg
}
#[must_use]
pub(crate) fn reflect_hready_granting<D: Default>(
    mut msg: MasterToSlaveWires<D>,
    resp: Option<&SlaveToMasterWires<D>>,
    deny: bool,
) -> MasterToSlaveWires<D> {
    if deny {
        msg.addr_phase.ready = LOW;
    } else if let Some(resp) = resp.as_ref() {
        msg.addr_phase.ready = resp.meta.HREADYOUT();
    }
    msg
}

impl<D: Default> MasterToSlaveWires<D> {
    pub(in crate::common::new_ahb) fn tagged(
        #[allow(unused_mut)] mut self,
        tag: &'static str,
    ) -> Self {
        #[cfg(feature = "cycle-debug-logger")]
        {
            self.addr_phase.tag = tag.into();
            self.data_phase.tag = tag.into();
        }
        self
    }
}

impl MasterToSlaveAddrPhase {
    pub(in crate::common::new_ahb) fn set_tag(&mut self, tag: &'static str) {
        #[cfg(feature = "cycle-debug-logger")]
        {
            self.tag = tag.into();
        }
    }

    #[must_use]
    pub(in crate::common::new_ahb) fn tagged(mut self, tag: &'static str) -> Self {
        self.set_tag(tag);
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(in crate::common::new_ahb) enum ReqType {
    NoSel, // nosel msg
    Idle,
    Read,
    Write,
    NoMsg, // not sending anything
    Any,   // only for matching!
}

impl ReqType {
    pub fn expects_reply(self) -> bool {
        self != Self::NoMsg && self != Self::NoSel
    }
    #[must_use]
    pub fn matches_kind(self, tt: &TransferType) -> bool {
        use ReqType::*;
        match (self, tt) {
            (Any, _) | (NoSel, TransferType::NoSel) | (Idle, TransferType::Idle) => true,
            (Write, TransferType::Seq(m) | TransferType::NonSeq(m)) => m.is_writing(),
            (Read, TransferType::Seq(m) | TransferType::NonSeq(m)) => m.is_reading(),
            _ => false,
        }
    }
}

pub(in crate::common::new_ahb) trait MarkerHelpers: Copy {
    fn addr(self) -> Address {
        Address::from_const(self.uaddr())
    }
    fn uaddr(self) -> u32;
    fn tag_str(self) -> &'static str;
    #[cfg(feature = "cycle-debug-logger")]
    fn cdl_tag(self) -> CdlTag {
        self.tag_str().into()
    }
}

#[must_use]
pub(in crate::common::new_ahb) fn make_addr_from_req_type<T: MarkerHelpers>(
    req_type: ReqType,
    tag: T,
) -> MasterToSlaveAddrPhase {
    use ReqType::*;
    match req_type {
        NoSel | NoMsg => {
            MasterToSlaveAddrPhase::not_selected::<UnknownPort>().tagged(tag.tag_str())
        }
        Idle => idle(),
        Read => read(tag.uaddr()),
        Write => write(tag.uaddr()),
        Any => unimplemented!(),
    }
    .tagged(tag.tag_str())
}

#[must_use]
pub(in crate::common::new_ahb) fn wildcard_eq(a: &str, b: &str) -> bool {
    // partial wildcards
    fn one_sided(a: &str, b_trim: &str) -> bool {
        let a_trim = a.trim_matches('*');
        match (a.starts_with('*'), a.ends_with('*')) {
            (true, true) => b_trim.contains(a_trim),
            (true, false) => b_trim.ends_with(a_trim),
            (false, true) => b_trim.starts_with(a_trim),
            _ => false,
        }
    }

    if a == b || a == "*" || b == "*" {
        return true;
    }

    let a_trim = a.trim_matches('*');
    let b_trim = b.trim_matches('*');
    one_sided(a, b_trim) || one_sided(b, a_trim)
}

#[cfg(test)]
mod test {
    use super::wildcard_eq;
    use rstest::rstest;

    #[rstest]
    #[case("ala", "ala")]
    #[case("ala", "*")]
    #[case("*", "ala")]
    #[case("*", "*")]
    #[case("*la", "ala")]
    #[case("ala", "a*")]
    #[case("*a", "a*")]
    #[case("*l*", "ala")]
    #[case("ala", "*a*")]
    #[case("ala", "*ala*")]
    #[case("*l*", "*ala*")]
    fn test_wildcard_eq(#[case] a: &'static str, #[case] b: &'static str) {
        println!("{a} == {b}");
        assert!(wildcard_eq(a, b));
    }

    #[rstest]
    #[case("ala", "bla")]
    #[case("a*", "bla")]
    #[case("ala", "bl*")]
    #[case("*la", "al*")]
    #[case("ala", "*b*")]
    #[case("*la", "*al*")]
    fn test_wildcard_neq(#[case] a: &'static str, #[case] b: &'static str) {
        println!("{a} != {b}");
        assert!(!wildcard_eq(a, b));
    }
}
