use log::trace;
use std::mem;

use crate::bridge_ports;
use crate::common::new_ahb::databus::DataBus;
use crate::common::new_ahb::master_driver::MasterDriver;
use crate::common::new_ahb::master_driver::stateless_helpers::SimplerHandler;
use crate::common::new_ahb::ports::AHBPortConfig;
use crate::common::new_ahb::signals::{Protection, Size};
#[cfg(feature = "cycle-debug-logger")]
use crate::common::new_ahb::{
    master_driver::{TransferInfoView, TransferStatus},
    signals::{MasterToSlaveWires, SlaveToMasterWires},
};
use crate::common::{Address, Word};
#[cfg(feature = "cycle-debug-logger")]
use crate::component::core::TransferType;
use crate::component::core::fetch::{DataReadCallback, TransferState};
use crate::component::core::{CoreComponent, IBusM};
use crate::engine::{
    Context, DisableableComponent, Subcomponent, TickComponent, TickComponentExtra,
};
#[cfg(feature = "cycle-debug-logger")]
use crate::proxy::CycleDebugLoggerProxy;
#[cfg(feature = "cycle-debug-logger")]
use crate::utils::IfExpr;
use owo_colors::OwoColorize;
use std::fmt::{Debug, Formatter};

pub(in crate::component::core) type IBusDriver = MasterDriver<IBusDriverSC, Transfers>;

/// Maintains information about each transfer that is requested by the Fetch
/// till the moment when the data arrives.
#[derive(Subcomponent, TickComponent, TickComponentExtra, DisableableComponent)]
#[subcomponent_1to1]
pub(in crate::component::core) struct Transfers {
    /// Transfer that was finished in previous cycle and it was scheduled
    /// to be handled in current cycle.
    ///
    /// The transfer will be handled in [`super::Fetch::handle_requested_data()`].
    // TODO: or where?
    delayed_transfer: Option<(Word, Address, DataReadCallback)>,

    #[subcomponent(pub(in crate::component::core) IBusDriverSC)]
    instruction_bus_driver: IBusDriver,

    /// Type of requested-in-current-cycle transfer. Needed by CDL.
    #[cfg(feature = "cycle-debug-logger")]
    requested_transfer_type: TransferType,
}
bridge_ports!(@auto_configured @master Transfers => @master IBusM);
bridge_ports!(@auto_configured @master IBusDriver => @master Transfers);

// Pub only because interface is parametrized with this type
#[derive(Debug)]
pub(in crate::component::core) struct TransferUserData {
    cb: DataReadCallback,
    #[cfg(feature = "cycle-debug-logger")]
    transfer_type: TransferType,
}

impl Debug for Transfers {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Transfers: {:?} + delayed {:?}",
            self.instruction_bus_driver, self.delayed_transfer
        )
    }
}

impl SimplerHandler for Transfers {
    type UserData = TransferUserData;
    type MasterDriverSC = IBusDriverSC;
    const AHB_LITE_COMPAT: bool = true;
    const DEFAULT_PROT: Protection = Protection::new_instruction();
    const HAS_GRANTING_WIRE: bool = true;

    // TODO: consider DENIES, not only hready
    fn read_will_advance(
        _comp: &mut <Self as AHBPortConfig>::Component,
        _ctx: &mut Context,
        addr: Address,
        _size: Size,
    ) {
        trace!("Fetch moving {:?} (unless denied)", addr);
    }

    fn transfers_will_stall(
        _comp: &mut <Self as AHBPortConfig>::Component,
        _ctx: &mut Context,
        has_addr: bool,
        has_data: bool,
    ) {
        trace!("Fetch stalled (addr: {has_addr}, data: {has_data})");
    }

    fn transfers_aborted(
        _comp: &mut <Self as AHBPortConfig>::Component,
        _ctx: &mut Context,
        _addr_phase: Option<<Self as SimplerHandler>::UserData>,
        _data_phase: Option<<Self as SimplerHandler>::UserData>,
    ) {
        unimplemented!()
    }

    /// Handles data that arrived on the Instruction bus in combinatorial way (same cycle).
    fn read_done(
        comp: &mut <Self as AHBPortConfig>::Component,
        ctx: &mut Context,
        address: Address,
        data: DataBus,
        user: <Self as SimplerHandler>::UserData,
    ) {
        debug_assert!(data.size() == Size::Word);
        trace!(
            "Fetch got response {} *{:?}={:?} (user: {:?})",
            ctx.display_named_address(address),
            address.bright_red(),
            data,
            user
        );
        let data = data.zero_extend_into_word();

        let mut this = Self::get_proxy(comp);
        debug_assert!(this.delayed_transfer.is_none());

        // Delay handling of the data to the next cycle.
        // However it may turn out in the future that some transfers
        // should be handled without delay. If so, do it here.
        let transfer_result = (data, address, user.cb);
        this.delayed_transfer = Some(transfer_result);

        // Transfers::handle_transfer_done(comp, data, address, user.cb);
    }

    #[cfg(feature = "cycle-debug-logger")]
    #[inline(always)]
    fn tap_request(
        ctx: &mut Context,
        req: &mut MasterToSlaveWires<Self::Data>,
        addr_user: Option<TransferInfoView<Self>>,
        data_info: Option<TransferInfoView<Self>>,
    ) {
        // TODO: can we simplify and set tag here?
        if addr_user.is_some() || data_info.is_some() {
            CycleDebugLoggerProxy.on_ibus_request(
                ctx,
                addr_user.map(|info| {
                    (
                        req.addr_phase
                            .meta
                            .meta()
                            .expect("Tap only when active transfer!")
                            .addr,
                        info.status,
                        info.user.transfer_type,
                    )
                }),
                data_info.map(|info| (info.meta.addr, info.status, info.user.transfer_type)),
            );
        }
    }

    #[cfg(feature = "cycle-debug-logger")]
    #[inline(always)]
    fn tap_response(
        ctx: &mut Context,
        data: &SlaveToMasterWires<Self::Data>,
        _has_addr: bool,
        has_data: bool,
    ) {
        // TODO move it to a better place (e.g. provide info here?)
        let status = has_data.then(|| {
            data.meta.is_done().ife(
                TransferStatus::DataPhaseDone,
                TransferStatus::DataPhaseWaiting,
            )
        });
        CycleDebugLoggerProxy.on_ibus_response(ctx, status);
    }
}

impl Transfers {
    pub(super) fn new() -> Self {
        Self {
            delayed_transfer: None,

            instruction_bus_driver: IBusDriver::new(),
            #[cfg(feature = "cycle-debug-logger")]
            requested_transfer_type: TransferType::StackPointer,
        }
    }
}

impl Transfers {
    pub(super) fn take_delayed_transfer(&mut self) -> Option<(Word, Address, DataReadCallback)> {
        self.delayed_transfer.take()
    }

    fn is_ignored(user: &<Self as SimplerHandler>::UserData) -> bool {
        matches!(user.cb, DataReadCallback::Ignore { .. })
    }

    pub(super) fn ignore(core: &mut CoreComponent) {
        let this = Self::component_to_member_mut(core);
        let driver = &mut this.instruction_bus_driver;

        // TODO: after integration rewrite it to match AMBA-incompatibility
        if driver.has_addr_phase() && !Self::is_ignored(driver.view_addr_phase().user) {
            driver.view_addr_phase().user.cb = DataReadCallback::Ignore {
                in_state: TransferState::AddrPhase,
            };
        }
        if driver.has_data_phase() && !Self::is_ignored(driver.view_data_phase().user) {
            driver.view_data_phase().user.cb = DataReadCallback::Ignore {
                in_state: TransferState::DataPhase,
            };
        }

        this.delayed_transfer = None;
    }

    pub(super) fn ignore_data_if_stalled(core: &mut CoreComponent) -> bool {
        let this = Self::component_to_member_mut(core);
        let driver = &mut this.instruction_bus_driver;
        if driver.has_data_phase()
            && !driver.pipeline_advanced()
            && !Self::is_ignored(driver.view_data_phase().user)
        {
            driver.view_data_phase().user.cb = DataReadCallback::Ignore {
                in_state: TransferState::DataPhase,
            };
            true
        } else {
            false
        }
    }

    pub(super) fn ignore_prev_cycle(core: &mut CoreComponent) {
        let this = Self::component_to_member_mut(core);
        let driver = &mut this.instruction_bus_driver;
        let (prev, in_state) = if driver.pipeline_advanced() {
            (driver.view_data_phase(), TransferState::DataPhase)
        } else {
            debug_assert!(driver.has_addr_phase());
            (driver.view_addr_phase(), TransferState::AddrPhase)
        };
        trace!(
            "Cancelling prev fetch transfer to {:?} of {:?}",
            prev.meta, prev.user
        );
        if !Self::is_ignored(prev.user) {
            prev.user.cb = DataReadCallback::Ignore { in_state };
        }
    }

    #[allow(clippy::if_then_some_else_none)]
    pub(super) fn cancel_addr_phase(
        core: &mut CoreComponent,
    ) -> Option<(Address, DataReadCallback)> {
        let this = Self::component_to_member_mut(core);
        let driver = &mut this.instruction_bus_driver;

        if driver.has_addr_phase() && !Self::is_ignored(driver.view_addr_phase().user) {
            let view = driver.view_addr_phase();
            let cb = mem::replace(
                &mut view.user.cb,
                DataReadCallback::Ignore {
                    in_state: TransferState::AddrPhase,
                },
            );
            let addr = view.meta.addr;
            driver.try_force_cancel();
            // debug_assert!(matches!(cb, DataReadCallback::AddToPiq { .. }));
            Some((addr, cb))
        } else {
            None
        }
    }

    pub(super) fn can_request(core: &mut CoreComponent) -> bool {
        Self::component_to_member_mut(core)
            .instruction_bus_driver
            .can_pipeline()
    }

    pub(super) fn request(
        core: &mut CoreComponent,
        _ctx: &mut Context,
        address: Address,
        callback: DataReadCallback,
        #[cfg(feature = "cycle-debug-logger")] transfer_type: TransferType,
    ) {
        debug_assert!(address.is_aligned_to_4_bytes());
        let this = Self::component_to_member_mut(core);
        #[cfg(feature = "cycle-debug-logger")]
        {
            this.requested_transfer_type = transfer_type;
        }

        assert!(this.instruction_bus_driver.try_read_data_maybe_tag(
            address,
            Size::Word,
            TransferUserData {
                cb: callback,
                #[cfg(feature = "cycle-debug-logger")]
                transfer_type,
            },
            #[cfg(feature = "cycle-debug-logger")]
            transfer_type.into(),
        ));
    }

    /// Returns mutable reference to callback of the requested-but-ignored transfer
    /// if the transfer can be reused.
    pub(super) fn try_reuse_ignored_requested(
        core: &mut CoreComponent,
        address: Address,
    ) -> Option<&mut DataReadCallback> {
        let this = Self::component_to_member_mut(core);
        if this.instruction_bus_driver.has_addr_phase() {
            let view = this.instruction_bus_driver.view_addr_phase();

            (view.meta.addr == address
                && matches!(
                    view.user.cb,
                    DataReadCallback::Ignore {
                        in_state: TransferState::AddrPhase
                    }
                ))
            .then_some(&mut view.user.cb)
        } else {
            None
        }
    }
}
