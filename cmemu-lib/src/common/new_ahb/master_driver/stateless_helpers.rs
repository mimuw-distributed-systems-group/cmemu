use crate::common::Address;
#[cfg(feature = "cycle-debug-logger")]
use crate::common::new_ahb::cdl::CdlTag;
use crate::common::new_ahb::databus::DataBus;
use crate::common::new_ahb::master_driver::{
    Handler, MasterDriver, TransferInfo, TransferInfoView, TransferStatus,
};
use crate::common::new_ahb::ports::{AHBMasterPortOutput, AHBPortConfig};
use crate::common::new_ahb::signals::{Burst, Direction, Protection, Size, TransferMeta};
use crate::common::new_ahb::{MasterToSlaveWires, SlaveToMasterWires};
use crate::engine::{Context, Subcomponent};
use std::fmt::Debug;

pub(crate) trait SimplerHandler: AHBPortConfig + Sized
where
    Self: AHBMasterPortOutput<
            Data = DataBus,
            Component = <<Self as SimplerHandler>::MasterDriverSC as Subcomponent>::Component,
        >,
{
    type UserData: Debug;
    type MasterDriverSC: Subcomponent<Member = MasterDriver<Self::MasterDriverSC, Self>>;

    const AHB_LITE_COMPAT: bool;
    const DEFAULT_PROT: Protection;
    const HAS_GRANTING_WIRE: bool = false;

    #[allow(unused_variables)]
    fn address_presented(
        comp: &mut <Self as AHBPortConfig>::Component,
        ctx: &mut Context,
        cancellable: bool,
        addr: Address,
        size: Size,
    ) {
    }

    #[allow(unused_variables)]
    fn read_will_advance(
        comp: &mut <Self as AHBPortConfig>::Component,
        ctx: &mut Context,
        addr: Address,
        size: Size,
    ) {
    }

    #[allow(unused_variables)]
    fn write_will_advance(
        comp: &mut <Self as AHBPortConfig>::Component,
        ctx: &mut Context,
        addr: Address,
        size: Size,
    ) -> DataBus {
        DataBus::HighZ
    }

    #[allow(unused_variables)]
    fn transfers_will_stall(
        comp: &mut <Self as AHBPortConfig>::Component,
        ctx: &mut Context,
        has_addr: bool,
        has_data: bool,
    ) {
    }

    #[allow(unused_variables)]
    fn write_needs_data_this_cycle(
        comp: &mut <Self as AHBPortConfig>::Component,
        ctx: &mut Context,
        addr: Address,
        size: Size,
    ) -> DataBus {
        panic!("Data was not provided for write transfer to {addr:?}!")
    }

    #[allow(unused_variables)]
    fn transfers_aborted(
        comp: &mut <Self as AHBPortConfig>::Component,
        ctx: &mut Context,
        addr_phase: Option<<Self as SimplerHandler>::UserData>,
        data_phase: Option<<Self as SimplerHandler>::UserData>,
    ) {
        panic!("Transfer errors not handled!")
    }

    // Combinatorial - before end of cycle
    fn read_done(
        comp: &mut <Self as AHBPortConfig>::Component,
        ctx: &mut Context,
        addr: Address,
        data: DataBus,
        user: <Self as SimplerHandler>::UserData,
    );

    #[allow(unused_variables)]
    fn write_done(
        comp: &mut <Self as AHBPortConfig>::Component,
        ctx: &mut Context,
        addr: Address,
        user: <Self as SimplerHandler>::UserData,
    ) {
    }

    #[allow(unused_variables)]
    fn override_prot(prot: &mut Protection, user: &mut Self::UserData) {}
    #[allow(unused_variables)]
    fn tap_request(
        ctx: &mut Context,
        req: &mut MasterToSlaveWires<Self::Data>,
        addr_user: Option<TransferInfoView<Self>>,
        data_info: Option<TransferInfoView<Self>>,
    ) {
    }
    #[allow(unused_variables)]
    fn tap_response(
        ctx: &mut Context,
        data: &SlaveToMasterWires<Self::Data>,
        has_addr: bool,
        has_data: bool,
    ) {
    }
}

impl<T: SimplerHandler> Handler for T
where
    Self: AHBMasterPortOutput<
            Data = DataBus,
            Component = <<Self as SimplerHandler>::MasterDriverSC as Subcomponent>::Component,
        >,
{
    type UserData = <T as SimplerHandler>::UserData;
    const AHB_LITE_COMPAT: bool = <T as SimplerHandler>::AHB_LITE_COMPAT;
    const HAS_GRANTING_WIRE: bool = <T as SimplerHandler>::HAS_GRANTING_WIRE;

    fn address_presented(
        comp: &mut <Self as AHBPortConfig>::Component,
        ctx: &mut Context,
        cancellable: bool,
    ) {
        let view: TransferInfoView<'_, Self> =
            <T as SimplerHandler>::MasterDriverSC::component_to_member_mut(comp).view_addr_phase();
        let addr = view.meta.addr;
        let size = view.meta.size;
        <T as SimplerHandler>::address_presented(comp, ctx, cancellable, addr, size);
    }

    fn transfer_will_advance(
        comp: &mut <Self as AHBPortConfig>::Component,
        ctx: &mut Context,
    ) -> Option<Self::Data> {
        let view: TransferInfoView<'_, Self> =
            <T as SimplerHandler>::MasterDriverSC::component_to_member_mut(comp).view_addr_phase();
        let addr = view.meta.addr;
        let size = view.meta.size;
        if view.meta.is_writing() {
            match <T as SimplerHandler>::write_will_advance(comp, ctx, addr, size) {
                DataBus::HighZ => None,
                x => Some(x),
            }
        } else {
            <T as SimplerHandler>::read_will_advance(comp, ctx, addr, size);
            None
        }
    }

    fn write_needs_data_this_cycle(
        comp: &mut <Self as AHBPortConfig>::Component,
        ctx: &mut Context,
    ) -> DataBus {
        let view: TransferInfoView<'_, Self> =
            <T as SimplerHandler>::MasterDriverSC::component_to_member_mut(comp).view_data_phase();
        let addr = view.meta.addr;
        let size = view.meta.size;
        let data = <T as SimplerHandler>::write_needs_data_this_cycle(comp, ctx, addr, size);
        debug_assert!(!matches!(&data, &DataBus::HighZ), "Data not provided");
        debug_assert!(
            data.size() == size,
            "Mismatched size {data:?} expected {size:?}"
        );
        data
    }
    #[inline(always)]
    fn transfers_will_stall(
        comp: &mut <Self as AHBPortConfig>::Component,
        ctx: &mut Context,
        has_addr: bool,
        has_data: bool,
    ) {
        <T as SimplerHandler>::transfers_will_stall(comp, ctx, has_addr, has_data);
    }

    fn transfer_done(
        comp: &mut <Self as AHBPortConfig>::Component,
        ctx: &mut Context,
        info: TransferInfo<Self>,
    ) {
        if info.meta.is_reading() {
            let data = info.data.expect("No data after read?");
            debug_assert!(
                !matches!(data, DataBus::HighZ),
                "Data was expected on the bus, but slave not provided it for: {:?}",
                info.meta
            );
            <T as SimplerHandler>::read_done(comp, ctx, info.meta.addr, data, info.user);
        } else {
            <T as SimplerHandler>::write_done(comp, ctx, info.meta.addr, info.user);
        }
    }

    #[inline(always)]
    fn transfers_aborted(
        comp: &mut <Self as AHBPortConfig>::Component,
        ctx: &mut Context,
        addr_phase: Option<TransferInfo<Self>>,
        data_phase: Option<TransferInfo<Self>>,
    ) {
        <T as SimplerHandler>::transfers_aborted(
            comp,
            ctx,
            addr_phase.map(|i| i.user),
            data_phase.map(|i| i.user),
        );
    }

    #[inline(always)]
    fn override_prot(prot: &mut Protection, user: &mut Self::UserData) {
        <T as SimplerHandler>::override_prot(prot, user);
    }

    #[inline(always)]
    fn tap_request(
        ctx: &mut Context,
        req: &mut MasterToSlaveWires<Self::Data>,
        addr_info: Option<TransferInfoView<Self>>,
        data_info: Option<TransferInfoView<Self>>,
    ) {
        <T as SimplerHandler>::tap_request(ctx, req, addr_info, data_info);
    }
    #[inline(always)]
    fn tap_response(
        ctx: &mut Context,
        data: &SlaveToMasterWires<Self::Data>,
        has_addr: bool,
        has_data: bool,
    ) {
        <T as SimplerHandler>::tap_response(ctx, data, has_addr, has_data);
    }
}

impl<SC, P> MasterDriver<SC, P>
where
    SC: Subcomponent<Member = Self>,
    P: SimplerHandler<Component = SC::Component, MasterDriverSC = SC> + AHBMasterPortOutput,
{
    pub(crate) fn try_read_data(
        &mut self,
        addr: Address,
        size: Size,
        user: <P as SimplerHandler>::UserData,
    ) -> bool {
        self.try_request(TransferInfo {
            meta: TransferMeta {
                addr,
                size,
                burst: Burst::Single,
                dir: Direction::Read,
                prot: <P as SimplerHandler>::DEFAULT_PROT,
            },
            status: TransferStatus::AddrPhaseNew,
            data: None,
            user,
            #[cfg(feature = "cycle-debug-logger")]
            tag: None,
        })
    }
    // TODO!: simplify it to a single function, HAXX
    pub(crate) fn try_read_data_maybe_tag(
        &mut self,
        addr: Address,
        size: Size,
        user: <P as SimplerHandler>::UserData,
        #[cfg(feature = "cycle-debug-logger")] tag: &'static str,
    ) -> bool {
        self.try_request(TransferInfo {
            meta: TransferMeta {
                addr,
                size,
                burst: Burst::Single,
                dir: Direction::Read,
                prot: <P as SimplerHandler>::DEFAULT_PROT,
            },
            status: TransferStatus::AddrPhaseNew,
            data: None,
            user,
            #[cfg(feature = "cycle-debug-logger")]
            tag: Some(CdlTag::from(tag)),
        })
    }

    #[allow(dead_code)]
    pub(crate) fn try_force_read_data(
        &mut self,
        addr: Address,
        size: Size,
        user: <P as SimplerHandler>::UserData,
    ) -> bool {
        self.try_force_request(TransferInfo {
            meta: TransferMeta {
                addr,
                size,
                burst: Burst::Single,
                dir: Direction::Read,
                prot: <P as SimplerHandler>::DEFAULT_PROT,
            },
            status: TransferStatus::AddrPhaseNew,
            data: None,
            user,
            #[cfg(feature = "cycle-debug-logger")]
            tag: None,
        })
    }

    pub(crate) fn try_write_data(
        &mut self,
        addr: Address,
        size: Size,
        user: <P as SimplerHandler>::UserData,
    ) -> bool {
        self.try_request(TransferInfo {
            meta: TransferMeta {
                addr,
                size,
                burst: Burst::Single,
                dir: Direction::Write,
                prot: <P as SimplerHandler>::DEFAULT_PROT,
            },
            status: TransferStatus::AddrPhaseNew,
            data: None,
            user,
            #[cfg(feature = "cycle-debug-logger")]
            tag: None,
        })
    }

    pub(crate) fn try_write_latched_data(
        &mut self,
        addr: Address,
        size: Size,
        data: DataBus,
        user: <P as SimplerHandler>::UserData,
    ) -> bool {
        self.try_request(TransferInfo {
            meta: TransferMeta {
                addr,
                size,
                burst: Burst::Single,
                dir: Direction::Write,
                prot: <P as SimplerHandler>::DEFAULT_PROT,
            },
            status: TransferStatus::AddrPhaseNew,
            data: Some(data),
            user,
            #[cfg(feature = "cycle-debug-logger")]
            tag: None,
        })
    }
}
