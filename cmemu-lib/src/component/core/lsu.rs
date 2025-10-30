use std::fmt::{Debug, Formatter};

use log::trace;

use crate::bridge_ports;
use crate::common::Word;
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
use crate::engine::{
    BufferFlop, Context, DisableableComponent, Subcomponent, TickComponent, TickComponentExtra,
};
#[cfg(feature = "cycle-debug-logger")]
use crate::proxy::CycleDebugLoggerProxy;
#[cfg(feature = "cycle-debug-logger")]
use crate::utils::IfExpr;
use cmemu_common::Address;

use super::register_bank::RegisterID;
use super::{CoreComponent, DBusM};

/// Load-Store Unit
/// Handles data bus transfers for execute and (in the future) interrupts.
///
/// This component is poorly represented in documentation.
/// Its main witnesses are:
/// * DWT LSUCNT register that count cycles in which load or store instruction is executed
/// * [ARM-TDG]
///
/// Nevertheless it is convenient to have such a component.
///
/// Note: we have function that sets write data for currently-in-data-phase
///   and for currently-in-address-phase transfers.
///
///   This is because sometimes we get data to write at the end of address phase
///   of the writing transfer.
///   For example: `ldr r0, [r1]; str r0, [r2]`.
///   * In the first cycle we request read transfer.
///   * In the second cycle we request write transfer.
///   * At the end of second cycle we get data to write.
///   * But register bank will have correct value of `r0` from the beginning of the third cycle.
///
///   On the other hand sometimes we want to finish instruction when its related transfer
///   is in address phase.
///   For example: `str r0, [r1]; mov r2, 42`.
///   * In the first cycle we request write transfer and read contents of `r0` register.
///   * In the first cycle we also finish the `str` instruction execution.
///   * In the second cycle we execute `mov` instruction and in the background
///     set write data for the requested in the previous cycle transfer.
#[derive(Subcomponent, TickComponent, DisableableComponent)]
#[subcomponent_1to1]
pub(super) struct LSU {
    // Writeback happens when the transfer goes to data phase (i.e., it won't be repeated)
    // TODO: refactor this into transfer's UserData
    pub(super) addr_advanced_callback: Option<AddrAdvancedCallback>,

    #[flop]
    unaligned_special_case: BufferFlop<()>,

    #[subcomponent(pub(super) DBusDriverSC)]
    data_bus_driver: DBusDriver,

    /// Checks if we do not execute `on_read_data` multiple times in a single cycle.
    #[cfg(debug_assertions)]
    on_read_data_executed: bool,
}

pub(super) type DBusDriver = MasterDriver<DBusDriverSC, LSU>;
bridge_ports!(@auto_configured @master LSU => @master DBusM);
bridge_ports!(@auto_configured @master DBusDriver => @master LSU);

/// Family of functions that translates data received on Data bus to value
/// that can be e.g. stored into a register
pub(super) type DecodeFn = fn(DataBus) -> Word;

/// Represents callback that can be executed when data on Data bus arrives
pub(super) enum ReadDataCallback {
    WithDecodeFn(fn(&mut CoreComponent, DecodeFn, DataBus), DecodeFn),
    WithRegisterAndDecodeFn(
        // bug in rustfmt removes `#[cfg(...)]` from inside the type
        #[cfg(feature = "cycle-debug-logger")]
        fn(&mut CoreComponent, &mut Context, RegisterID, DecodeFn, DataBus),
        #[cfg(not(feature = "cycle-debug-logger"))]
        fn(&mut CoreComponent, RegisterID, DecodeFn, DataBus),
        RegisterID,
        DecodeFn,
    ),
    WriteCallbacks {
        get_data: fn(&mut CoreComponent, RegisterID, Size) -> DataBus,
        write_done: fn(&mut CoreComponent, RegisterID),
        reg: RegisterID,
    },
    #[allow(dead_code)]
    NoCallback,
}
pub(super) enum AddrAdvancedCallback {
    WritebackCallback {
        cb: fn(&mut CoreComponent, &mut Context, RegisterID, Word),
        reg: RegisterID,
        data: Word,
    },
    #[allow(dead_code)]
    NoCallback,
}

impl TickComponentExtra for LSU {
    fn tick_extra(&mut self) {
        #[cfg(debug_assertions)]
        {
            self.on_read_data_executed = false;
        }
        self.unaligned_special_case.allow_skip();
    }
}

impl SimplerHandler for LSU {
    type UserData = ReadDataCallback;
    type MasterDriverSC = DBusDriverSC;
    const AHB_LITE_COMPAT: bool = false;
    const DEFAULT_PROT: Protection = Protection::new_data();
    const HAS_GRANTING_WIRE: bool = true;

    fn address_presented(
        comp: &mut <Self as AHBPortConfig>::Component,
        ctx: &mut Context,
        _cancellable: bool,
        addr: Address,
        _size: Size,
    ) {
        trace!("Transfer meta locked for {:?}", addr);
        let mut this = Self::get_proxy(comp);
        if let Some(cb) = this.addr_advanced_callback.take() {
            cb.call(this.component_mut(), ctx);
        }
    }

    fn read_will_advance(
        _comp: &mut <Self as AHBPortConfig>::Component,
        _ctx: &mut Context,
        addr: Address,
        _size: Size,
    ) {
        trace!("Data read advancing for {:?}", addr);
    }

    fn write_will_advance(
        _comp: &mut <Self as AHBPortConfig>::Component,
        _ctx: &mut Context,
        addr: Address,
        _size: Size,
    ) -> DataBus {
        trace!("Data write advancing for {:?}", addr);
        DataBus::HighZ
    }

    fn transfers_will_stall(
        _comp: &mut <Self as AHBPortConfig>::Component,
        _ctx: &mut Context,
        has_addr: bool,
        has_data: bool,
    ) {
        trace!("LSU stall (addr: {has_addr}, data: {has_data})");
    }

    fn write_needs_data_this_cycle(
        comp: &mut <Self as AHBPortConfig>::Component,
        _ctx: &mut Context,
        _addr: Address,
        size: Size,
    ) -> DataBus {
        let view = Self::MasterDriverSC::component_to_member_mut(comp).view_data_phase();
        if let ReadDataCallback::WriteCallbacks { get_data, reg, .. } = view.user {
            let reg = *reg;
            get_data(comp, reg, size)
        } else {
            unreachable!("Wrong callback for write!")
        }
    }

    fn transfers_aborted(
        comp: &mut <Self as AHBPortConfig>::Component,
        _ctx: &mut Context,
        _addr_phase: Option<<Self as SimplerHandler>::UserData>,
        _data_phase: Option<<Self as SimplerHandler>::UserData>,
    ) {
        let this = Self::component_to_member_mut(comp);
        this.addr_advanced_callback = None;
    }

    fn read_done(
        comp: &mut <Self as AHBPortConfig>::Component,
        #[allow(unused)] ctx: &mut Context,
        addr: Address,
        data: DataBus,
        cb: <Self as SimplerHandler>::UserData,
    ) {
        trace!(
            "Read finished from {} *{:?}={:?}",
            ctx.display_named_address(addr),
            addr,
            data,
        );

        #[cfg_attr(not(debug_assertions), allow(unused))]
        let this = Self::component_to_member_mut(comp);

        #[cfg(debug_assertions)]
        {
            assert!(
                !this.on_read_data_executed,
                "Cannot call on_read_data multiple times in a cycle"
            );
            this.on_read_data_executed = true;
        }

        cb.call(
            comp,
            #[cfg(feature = "cycle-debug-logger")]
            ctx,
            data,
        );
    }

    fn write_done(
        comp: &mut <Self as AHBPortConfig>::Component,
        ctx: &mut Context,
        addr: Address,
        user: <Self as SimplerHandler>::UserData,
    ) {
        trace!(
            "Write finished to {:?} {}",
            addr,
            ctx.display_named_address(addr)
        );

        if let ReadDataCallback::WriteCallbacks {
            write_done, reg, ..
        } = user
        {
            write_done(comp, reg);
        } else {
            // TODO: probably remove this comment, but it was commented out in ab352a63c9e2d5cb200fcba66072815acbd6c3f8 ("michcioperz's fix to interrupts")
            unreachable!("Wrong callback for write!")
        }
    }

    #[cfg(feature = "cycle-debug-logger")]
    #[inline(always)]
    fn tap_request(
        ctx: &mut Context,
        req: &mut MasterToSlaveWires<Self::Data>,
        addr_user: Option<TransferInfoView<Self>>,
        data_info: Option<TransferInfoView<Self>>,
    ) {
        if addr_user.is_some() || data_info.is_some() {
            CycleDebugLoggerProxy.on_dbus_request(
                ctx,
                req.clone(),
                addr_user.map(|i| i.status),
                data_info.map(|i| i.meta.addr),
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
        CycleDebugLoggerProxy.on_dbus_response(ctx, data.clone(), status);
    }
}

// ============================================================================
// API
// ============================================================================

impl LSU {
    // XXX: LSU uses always INCR bursts
    // https://developer.arm.com/documentation/ka001196/1-0/?lang=en
    // https://developer.arm.com/documentation/ka001187/1-0/?lang=en
    // D-Code bus interface produces D-side and DAP transfers. (SINGLE/NONSEQ, INCR/NONSEQ and INCR/SEQ/32-bit)
    // System bus interface produces I-side, D-side and DAP transfers. (SINGLE/NONSEQ, INCR/NONSEQ and INCR/SEQ/32-bit)
    pub(super) fn new() -> Self {
        Self {
            addr_advanced_callback: None,
            data_bus_driver: DBusDriver::new(),
            unaligned_special_case: BufferFlop::new(),

            #[cfg(debug_assertions)]
            on_read_data_executed: false,
        }
    }
    pub(super) fn run_driver(core: &mut CoreComponent, ctx: &mut Context) {
        DBusDriver::run_driver(core, ctx);
        let mut this = Self::get_proxy(core);

        // Proof: see memory_tests/pipelining_unaligned_lsu.asm
        // TODO: extract the proof somewhere nicer
        // and analysis in https://docs.google.com/spreadsheets/d/1g7yJolJABXHm8pCJIN51AlQhXfmAAHpDA3RMnLxGljQ/edit#gid=1621501606&range=BB1528
        // Complete hypothesis:
        // "- Unaligned accesses have the same pipelining behavior as aligned,
        // but it is shifted by a single cycle"
        //
        // This is proven by (references to the spreadsheet):
        // *2 -- not pipelining in the first data phase cycle
        // *3 - not pipelining with the third data phase part
        // *1 - pipelineing with second cycle/part of data event after decoded perfectly
        // *4 - pipelining is with second cycle of first data phase, not second advancement to data phase
        // *5 - in the case as above, but aligned -- it is not pipelined
        if this.data_bus_driver.pipeline_advanced() {
            let view = this.data_bus_driver.view_data_phase();
            if !view.meta.is_aligned() {
                this.unaligned_special_case.set_this_cycle(());
            }
        }
    }
    pub(super) fn tock(core: &mut CoreComponent, ctx: &mut Context) {
        DBusDriver::tock(core, ctx);
    }

    pub(super) fn can_pipeline_new_request(core: &CoreComponent) -> bool {
        let this = Self::component_to_member(core);
        // TODO: maybe this should be a separate component that also does the wire remapping
        // of AHB (unaligned would need a barrel shift)
        if this.unaligned_special_case.has_this_cycle() {
            false
        } else if this.unaligned_special_case.has_prev_cycle() {
            true
        } else {
            this.data_bus_driver.pipeline_advanced()
                || (this.data_bus_driver.is_free() && this.data_bus_driver.can_pipeline())
        }
    }
    pub(super) fn in_unaligned_special(core: &CoreComponent) -> bool {
        let this = Self::component_to_member(core);
        this.unaligned_special_case.has_prev_cycle()
    }

    pub(super) fn can_request(core: &CoreComponent) -> bool {
        let this = Self::component_to_member(core);
        this.data_bus_driver.can_pipeline()
    }

    pub(super) fn request_read(
        core: &mut CoreComponent,
        addr: Word,
        size: Size,
        cb: ReadDataCallback,
    ) {
        let this = Self::component_to_member_mut(core);
        let posted = this.data_bus_driver.try_read_data(addr.into(), size, cb);
        // TODO: use RegisterBank::current_mode_is_privileged -> we already implement this!
        debug_assert!(
            posted,
            "Core attempted to schedule AHB transfer to {addr} when not available"
        );
    }

    pub(super) fn request_write(
        core: &mut CoreComponent,
        addr: Word,
        size: Size,
        cb: ReadDataCallback,
    ) {
        let this = Self::component_to_member_mut(core);
        let posted = this.data_bus_driver.try_write_data(addr.into(), size, cb);
        debug_assert!(
            posted,
            "Core attempted to schedule AHB transfer to {addr} when not available"
        );
    }

    pub(super) fn request_write_multiple(
        core: &mut CoreComponent,
        addr: Word,
        size: Size,
        cb: ReadDataCallback,
        data: DataBus,
    ) {
        let this = Self::component_to_member_mut(core);
        let posted = this
            .data_bus_driver
            .try_write_latched_data(addr.into(), size, data, cb);
        debug_assert!(
            posted,
            "Core attempted to schedule AHB transfer to {addr} when not available"
        );
    }

    /// Sets callback to run when read data arrives on data bus in current cycle.
    /// XXX: why would we knew that that data will arrive?
    #[allow(dead_code)] // Leave the impl for now.
    pub(super) fn set_read_data_callback(
        core: &mut <Self as Subcomponent>::Component,
        cb: ReadDataCallback,
    ) {
        let this = Self::component_to_member_mut(core);
        debug_assert!(this.data_bus_driver.has_data_phase());
        let cur_cb = this.data_bus_driver.view_data_phase().user;
        debug_assert!(matches!(cur_cb, ReadDataCallback::NoCallback));
        *cur_cb = cb;
    }

    /// Sets write data for currently-in-data-phase write transfer.
    #[allow(dead_code)]
    pub(super) fn set_write_data(core: &mut CoreComponent, _ctx: &mut Context, data: [u8; 4]) {
        let this = Self::component_to_member_mut(core);
        // TODO: investigate what kind of data is that
        debug_assert!(this.data_bus_driver.has_data_phase());
        let meta = this.data_bus_driver.view_data_phase().meta;
        let data = DataBus::from(data).extract_from_aligned(meta.addr, meta.size);
        this.data_bus_driver.provide_data(data);
    }
}

// ============================================================================
// Helper functions
// ============================================================================

impl ReadDataCallback {
    pub(super) fn call(
        &self,
        core: &mut CoreComponent,
        #[cfg(feature = "cycle-debug-logger")] ctx: &mut Context,
        data: DataBus,
    ) {
        match self {
            ReadDataCallback::WithDecodeFn(f, decode) => f(core, *decode, data),
            ReadDataCallback::WithRegisterAndDecodeFn(f, reg, decode) => f(
                core,
                #[cfg(feature = "cycle-debug-logger")]
                ctx,
                *reg,
                *decode,
                data,
            ),
            _ => unimplemented!(),
        }
    }
}

impl AddrAdvancedCallback {
    pub(super) fn call(&self, core: &mut CoreComponent, ctx: &mut Context) {
        match self {
            Self::WritebackCallback { cb, reg, data } => cb(core, ctx, *reg, *data),
            Self::NoCallback => (),
        }
    }
}

impl Debug for ReadDataCallback {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ReadCallback...")
    }
}
impl Debug for AddrAdvancedCallback {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "AddrAdvancedCallback...")
    }
}
