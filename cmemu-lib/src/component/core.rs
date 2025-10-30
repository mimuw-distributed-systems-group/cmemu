#[proxy_use]
use crate::common::Word;
use crate::common::new_ahb::databus::DataBus;
use crate::common::new_ahb::ports::AhbMasterPortInputWithGranting;
#[proxy_use]
use crate::common::new_ahb::ports::{AHBMasterPortInput, AHBMasterPortProxiedInput, AHBPortConfig};
#[proxy_use]
use crate::common::new_ahb::signals::SlaveToMasterWires;
#[proxy_use]
use crate::component::nvic::InterruptData;
#[proxy_use]
use crate::engine::{Context, PowerNode};
use crate::engine::{
    DisableableComponent, MainComponent, SkippableClockTreeNode, TickComponent, TickComponentExtra,
};
use crate::make_port_struct;
use crate::proxy::{BusMatrixProxy, CoreProxy, DWTProxy, NVICProxy};
use cc2650_constants::operation::StackPointer;
use cmemu_common::Address;
use cmemu_proc_macros::{component_impl, handler, proxy_use};
use fetch::Fetch;
use log::info;
use lsu::LSU;

mod builtins;
mod decode;
mod execute;
mod fetch;
mod instruction;
mod interrupt;
mod lsu;
mod memory;
mod register_bank;

pub(crate) use self::register_bank::{
    BasePriorityMaskRegister, FaultMaskRegister, PriorityMaskRegister,
}; // NVIC export
#[cfg(feature = "cycle-debug-logger")] // CDL export
pub(crate) use self::{
    fetch::TransferType, instruction::Instruction, register_bank::ControlRegister,
    register_bank::XPSR,
};

#[proxy_use]
use crate::common::new_ahb::signals::TrackedBool;

pub use register_bank::RegisterID; // crate::common export

// todo: consider the following regarding ticking the flops
// a) build script check: component must derive TickComponent (programmer could implement it by hand improperly...)
// b) proc macro: warning if #[flop] not on Flop<_> or if Flop<_> is missing #[flop]
// c) developer can unintentionally call .tick() on flops or .tick_flops() on the component - it would be a bug hard to debug!
//    solutions:
//    1) compile/build time -- probably too complex
//    2) runtime -- on debug mode we have global state with all flops registered
//                  and in clock_tree.tick() we check if all flops ticked exactly once

/// Component representing ARM Cortex-M3 Core, see [TRM, section 2.1]
#[derive(MainComponent, SkippableClockTreeNode, TickComponent, TickComponentExtra)]
pub(crate) struct CoreComponent {
    dwt: DWTProxy,
    nvic: NVICProxy,

    #[subcomponent(Fetch)]
    fetch: Fetch,
    #[subcomponent(Decode)]
    decode: Decode,
    #[subcomponent(Execute)]
    execute: Execute,

    #[subcomponent(LSU)]
    lsu: LSU,
    #[subcomponent(RegisterBankSubcomponent)]
    registers: RegisterBank,

    #[subcomponent(InterruptEntryAndExitHandler)]
    interrupt_handler: InterruptEntryAndExitHandler,

    /// Ability to start from another address than `ResetISR`
    nonstandard_entrypoint: Option<Address>,
    /// Did the pipeline advance this cycle? Used for the component API.
    pipeline_advanced: bool,
}

#[component_impl(core)]
impl CoreComponent {
    pub(crate) fn new() -> Self {
        Self {
            dwt: DWTProxy::new(),
            nvic: NVICProxy::new(),

            fetch: Fetch::new(),
            decode: Decode::new(),
            execute: Execute::new(),
            lsu: LSU::new(),
            registers: RegisterBank::new(),

            interrupt_handler: InterruptEntryAndExitHandler::new(),
            nonstandard_entrypoint: None,
            pipeline_advanced: false,
        }
    }

    pub(crate) fn tick(&mut self, ctx: &mut Context) {
        /* Stuff that is probably writen to registers: tick_extra() { */
        self.pipeline_advanced = false;
        // In the current impl nothing really happens, apart from chainging intra-cycle STM
        Fetch::run_driver(self, ctx);
        LSU::run_driver(self, ctx);

        // Processes data that came in the previous cycle – those may either have combinatorial
        // logic before or after registers.
        Fetch::handle_requested_data(self);
        Fetch::log_cache_status(self, ctx, "fetch_1_early");

        Execute::run_update_interruption_state(self);
        let pipeline_action = if Execute::is_stacking_or_unstacking_running(self) {
            InterruptEntryAndExitHandler::run_interrupt_entry_and_exit(self, ctx)
        } else {
            InterruptEntryAndExitHandler::delay_interrupt_entry_if_scheduled(self);
            PipelineAction::RunFullPipeline
        };

        if let PipelineAction::RunFullPipeline = &pipeline_action {
            if Decode::is_ready(self) {
                let (instr, it_skipped, has_folded_instr) = Decode::peek_instruction(self, ctx);
                // I.e. "was last cycle or pipelineable"
                if Execute::is_ready(self, ctx, instr, it_skipped, has_folded_instr) {
                    // That is: load into registers on edge (both F-D and D-E interfaces)
                    // F-D should shift  piq, but that may be prevented by D.
                    let pipeline_step_pack = Decode::move_pipeline(self, ctx);
                    Execute::move_pipeline(self, pipeline_step_pack);
                    self.pipeline_advanced = true;
                }
            }
            Execute::handle_dwt_counters(self, ctx);
        }
        // TODO: manage PIQ shift - more of tick_extra

        // Idea on Fetch:
        // (When shifting reg: consider previously locked.)
        // Data may be shifted out of F/D reg, when it is stored in D/E reg.
        // F does a speculative data request, when executing single cycle instruction.
        // Branches are special: they are kept in PIQ until after their X
        //      (is it true only for postponed or all branches?)
        // Question: does it work like thi only for d-time branches or also for x-time? what abt' skipped?
        // 2: does skipped branches (cbz) halt decode?
        // Idea: PIQ is 2 words + register
        Fetch::tick_piq(
            self,
            ctx,
            Execute::was_last_cycle(self),
            Execute::postponing_in_exec(self),
        );

        Fetch::log_cache_status(self, ctx, "fetch_5_edge");

        /* } // tick_extra */
        /* actual early-to-late cycle tick() { */

        let trigger_data = match pipeline_action {
            PipelineAction::RunFullPipeline => {
                // XXX: here we may leak late-cycle information to decode?
                //      Most notably we purposefully leak next_xpsr. The main question is whether
                //      LSU instructions (finished in tock phase) may affect XPSR the same as
                //      last-cycle information of multi-cycle ALU operations.
                // Jumps are early-cycle stuff and affect both Fetch and Decode.
                // Same for speculative branches status.
                Execute::run_execute(self, ctx)
            }
            PipelineAction::RunDecode(trigger_data) => Some(trigger_data),
            PipelineAction::None => None,
        };

        // Execute knows correct PC value.
        let pc = Execute::get_pc(self);
        RegisterBank::log_registers(
            self,
            #[cfg(feature = "cycle-debug-logger")]
            ctx,
            pc,
        );

        if let Some(trigger_data) = trigger_data {
            Decode::run_decode(
                self,
                #[cfg(feature = "cycle-debug-logger")]
                ctx,
                trigger_data,
            );
        }

        Fetch::run_fetch(self, ctx);

        Fetch::log_cache_status(self, ctx, "fetch_8_end_of_tick");
        /* } tick */
    }

    pub(crate) fn tock(&mut self, ctx: &mut Context) {
        // Late cycle and asynchronous data processing: driving outputs & reacting to AHB messages.
        Fetch::tock(self, ctx);
        LSU::tock(self, ctx);
        BusMatrixProxy.on_core_tock_done(ctx);
    }

    #[handler]
    pub(crate) fn start_interrupt_entry(&mut self, _ctx: &mut Context, data: InterruptData) {
        let this_instr_addr = Execute::this_instr_addr(self);
        let next_instr_addr = Execute::next_instr_addr(self);
        info!("Core starts interrupt: {data:?} at PC: {this_instr_addr:x}");
        InterruptEntryAndExitHandler::init_interrupt_entry(
            self,
            interrupt::InterruptEntryData {
                interrupt_id: data.interrupt_id,
                this_instr_addr,
                next_instr_addr,
            },
        );
        Execute::request_interruption(self);
    }

    #[handler]
    pub(crate) fn tail_chain_interrupt(&mut self, _ctx: &mut Context, data: Option<InterruptData>) {
        InterruptEntryAndExitHandler::tail_chain_interrupt(self, data);
    }

    #[handler]
    pub(crate) fn update_vector_table_offset_register(&mut self, _ctx: &mut Context, vtor: Word) {
        InterruptEntryAndExitHandler::update_vector_table_offset_register(self, vtor);
    }

    /// Wake up the processor if it is sleeping and continue execution after the WFI instruction.
    #[handler]
    pub(crate) fn spurious_wakeup(&mut self, _ctx: &mut Context) {
        info!("CPU spurious wakeup");
        Execute::restore_execution(self);
        // This requires we are still frozen without a tick, and may panic otherwise.
        // TODO: check the timing - here we cleared the Fetch on entering sleep
        Fetch::make_delayed_branch(self, Execute::next_instr_addr(self));
    }

    #[handler]
    pub fn on_ahb_ibus_input(
        &mut self,
        ctx: &mut Context,
        msg: SlaveToMasterWires<<IBusM as AHBPortConfig>::Data>,
    ) {
        if !<Self as PowerNode>::is_active(self, ctx) {
            debug_assert!(msg.is_inert());
            return;
        }
        <IBusM as AHBMasterPortInput>::on_ahb_input(self, ctx, msg);
    }

    #[handler]
    pub fn on_ahb_dbus_input(
        &mut self,
        ctx: &mut Context,
        msg: SlaveToMasterWires<<DBusM as AHBPortConfig>::Data>,
    ) {
        if !<Self as PowerNode>::is_active(self, ctx) {
            debug_assert!(msg.is_inert());
            return;
        }

        <DBusM as AHBMasterPortInput>::on_ahb_input(self, ctx, msg);
    }

    #[handler]
    pub(crate) fn on_grant_data(&mut self, ctx: &mut Context, granted: TrackedBool) {
        <lsu::DBusDriver as AhbMasterPortInputWithGranting>::on_grant_wire(self, ctx, granted);
    }

    #[handler]
    pub(crate) fn on_grant_instruction(&mut self, ctx: &mut Context, granted: TrackedBool) {
        <fetch::IBusDriver as AhbMasterPortInputWithGranting>::on_grant_wire(self, ctx, granted);
    }

    pub(crate) fn get_register(&self, reg: CoreCoupledRegisterId) -> Word {
        use CoreCoupledRegisterId::*;
        use SpecialPurposeRegisterId::*;
        match reg {
            Core(RegisterID::PC) => {
                // PC value is context dependent
                // Here we want to show it as it is seen in execute phase and by debugger
                Execute::get_pc(self)
            }
            Core(rid) => RegisterBank::get_register(self, rid),
            StackPointer(sp) => RegisterBank::get_stack_pointer(self, sp),
            SpecialPurpose(XPSR) => RegisterBank::get_xpsr(self).into(),
            SpecialPurpose(Itstate) => RegisterBank::get_xpsr(self).get_itstate().into(),
            SpecialPurpose(Primask) => RegisterBank::get_primask(self).into(),
            SpecialPurpose(Faultmask) => RegisterBank::get_faultmask(self).into(),
            SpecialPurpose(Basepri) => RegisterBank::get_basepri(self).into(),
            SpecialPurpose(Control) => RegisterBank::get_control(self).into(),
            CoreCoupledRegisterId::AGUResult => RegisterBank::get_agu_result(self),
        }
    }

    pub(crate) fn get_this_instr_addr(&self) -> Address {
        Execute::this_instr_addr(self).into()
    }

    pub(crate) fn get_next_instr_addr(&self) -> Address {
        // TODO: we should assert these are called in the right place, but still allow post-mortem
        // gdb analysis.
        // This value is only valid just before the next .tick()
        Execute::next_instr_addr(self).into()
    }

    pub(crate) fn get_pipeline_advanced(&self) -> bool {
        self.pipeline_advanced
    }

    pub(crate) fn set_nonstandard_entrypoint(&mut self, entrypoint: Option<Address>) {
        // assert!(!entrypoint.is_some_and(|e| Word::from(e).get_bit(0)));
        self.nonstandard_entrypoint = entrypoint;
    }
}

#[component_impl(core)]
impl DisableableComponent for CoreComponent {
    fn can_be_disabled_now(&self) -> bool {
        let fetch = self.fetch.can_be_disabled_now();
        let lsu = self.lsu.can_be_disabled_now();
        info!("can_be_disabled_now: fetch: {} lsu: {}", fetch, lsu);
        fetch && lsu
    }
}

/// `PipelineAction` informs whether the full pipeline or only decoding phase
/// should be executed during the core tick.
enum PipelineAction {
    RunFullPipeline,
    RunDecode(decode::TriggerData),
    None,
}

// ----------------------------------------------------------------------------
// Component API definitions
// ----------------------------------------------------------------------------

/// [ARM-ARM] B1.4 Registers
/// "The ARMv7-M profile has the following registers closely coupled to the processor:"
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CoreCoupledRegisterId {
    Core(RegisterID),
    StackPointer(StackPointer),
    SpecialPurpose(SpecialPurposeRegisterId),
    /// This is our special register
    AGUResult,
}

/// [ARM-ARM] B1.4.2-4
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum SpecialPurposeRegisterId {
    XPSR,
    Itstate,
    Primask,
    Faultmask,
    Basepri,
    Control,
}

impl From<RegisterID> for CoreCoupledRegisterId {
    fn from(id: RegisterID) -> Self {
        Self::Core(id)
    }
}
impl From<StackPointer> for CoreCoupledRegisterId {
    fn from(id: StackPointer) -> Self {
        Self::StackPointer(id)
    }
}
impl From<SpecialPurposeRegisterId> for CoreCoupledRegisterId {
    fn from(id: SpecialPurposeRegisterId) -> Self {
        Self::SpecialPurpose(id)
    }
}

// ----------------------------------------------------------------------------
// Subcomponents definition
// ----------------------------------------------------------------------------

make_port_struct!(pub(crate) IBusM);
#[proxy_use(proxy_only)] // export for codegen
use crate::component::core::IBusM;
impl AHBPortConfig for IBusM {
    type Data = DataBus;
    type Component = CoreComponent;
    const TAG: &'static str = "IBus";
}
impl AHBMasterPortProxiedInput for IBusM {
    fn proxy_ahb_input(ctx: &mut Context, msg: SlaveToMasterWires<Self::Data>) {
        CoreProxy.on_ahb_ibus_input(ctx, msg);
    }
}

make_port_struct!(pub(crate) DBusM);
#[proxy_use(proxy_only)] // export for codegen
use crate::component::core::DBusM;
use crate::component::core::decode::Decode;
use crate::component::core::execute::Execute;
use crate::component::core::interrupt::InterruptEntryAndExitHandler;

impl AHBPortConfig for DBusM {
    type Data = DataBus;
    type Component = CoreComponent;
    const TAG: &'static str = "DBus";
}
impl AHBMasterPortProxiedInput for DBusM {
    fn proxy_ahb_input(ctx: &mut Context, msg: SlaveToMasterWires<Self::Data>) {
        CoreProxy.on_ahb_dbus_input(ctx, msg);
    }
}

type RegisterBank = register_bank::RegisterBank<RegisterBankSubcomponent>;
