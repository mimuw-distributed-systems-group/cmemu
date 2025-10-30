use core::ops::Range;

use cmemu_common::address_match_range;
use cmemu_proc_macros::{component_impl, handler, proxy_use};
use log::{trace, warn};
use scopeguard::guard;

use crate::bridge_ports;
use crate::common::new_ahb::databus::DataBus;
#[proxy_use]
use crate::common::new_ahb::ports::{AHBPortConfig, AHBSlavePortInput, AHBSlavePortProxiedInput};
#[proxy_use]
use crate::common::new_ahb::signals::{MasterToSlaveWires, TransferMeta};
use crate::common::new_ahb::slave_driver::{
    SimpleHandler, SimpleResponse, SimpleSynchronousSlaveInterface, SimpleWriteResponse, WriteMode,
};
use crate::common::{Address, BitstringUtils, Word};
#[proxy_use]
use crate::component::core::{BasePriorityMaskRegister, FaultMaskRegister, PriorityMaskRegister};
#[proxy_use(proxy_only)]
use crate::component::nvic::{CoreStateChange, InterruptId};
use crate::engine::{
    CpuMode, DisableableComponent, MainComponent, SeqFlop, SeqFlopMemoryBank,
    SeqFlopMemoryBankSimple, SkippableClockTreeNode, TickComponent, TickComponentExtra,
};
// export for core
pub(in crate::component) use self::system_control_block::{SCBRegister, VTOR};
#[proxy_use]
use crate::engine::Context;
use crate::proxy::{CoreProxy, NVICProxy, PRCMProxy};
use crate::utils::IfExpr;

mod register_bank;
mod stir;
mod system_control_block;
mod systick;

const REGISTER_BYTES: usize = 4;
const REGISTER_BITS: usize = 32;

// NVIC registers and interrupts counts.

/// [TI-TRM] Table 4-1.
const EXCEPTIONS_COUNT: usize = 16;
/// [TI-TRM] Table 4-2.
const INTERRUPTS_COUNT: usize = 34;
const EXCEPTIONS_AND_INTERRUPTS_COUNT: usize = EXCEPTIONS_COUNT + INTERRUPTS_COUNT;

// NVIC memory length of registers.
#[allow(clippy::cast_possible_truncation)]
const BIT_REGISTERS_ADDRESS_SPACE_LENGTH: u32 =
    (register_bank::REGISTERS_COUNT_PER_BIT_REGISTER_SET * REGISTER_BYTES) as u32;
#[allow(clippy::cast_possible_truncation)]
const PRIORITY_REGISTERS_ADDRESS_SPACE_LENGTH: u32 =
    (register_bank::PRIORITY_REGISTERS_COUNT * REGISTER_BYTES) as u32;

// NVIC valid memory sections

/// Set enable bits address space.
/// [ARM-TRM-G] Table 8-1.
const SET_ENABLE_ADDR_SPACE: Range<Address> =
    Address::range_from_len(0xE000_E100, BIT_REGISTERS_ADDRESS_SPACE_LENGTH);
/// Clear enable bits address space.
/// [ARM-TRM-G] Table 8-1.
const CLEAR_ENABLE_ADDR_SPACE: Range<Address> =
    Address::range_from_len(0xE000_E180, BIT_REGISTERS_ADDRESS_SPACE_LENGTH);
/// Set pending bits address space.
/// [ARM-TRM-G] Table 8-1.
const SET_PENDING_ADDR_SPACE: Range<Address> =
    Address::range_from_len(0xE000_E200, BIT_REGISTERS_ADDRESS_SPACE_LENGTH);
/// Clear pending bits address space.
/// [ARM-TRM-G] Table 8-1.
const CLEAR_PENDING_ADDR_SPACE: Range<Address> =
    Address::range_from_len(0xE000_E280, BIT_REGISTERS_ADDRESS_SPACE_LENGTH);
/// Active bits address space.
/// [ARM-TRM-G] Table 8-1.
const ACTIVE_ADDR_SPACE: Range<Address> =
    Address::range_from_len(0xE000_E300, BIT_REGISTERS_ADDRESS_SPACE_LENGTH);
/// Priority registers address space.
/// [ARM-TRM-G] Table 8-1.
const PRIORITY_ADDR_SPACE: Range<Address> =
    Address::range_from_len(0xE000_E400, PRIORITY_REGISTERS_ADDRESS_SPACE_LENGTH);
/// System Control Block address space.
/// [ARM-ARM] Table B3-3 SCS address space regions.
const SCB_ADDR_SPACE: Range<Address> =
    Address::from_const(0xE000_ED00)..Address::from_const(0xE000_ED90);
/// `SysTick` address space.
/// [ARM-ARM] Table B3-3 SCS address space regions.
const SYSTICK_ADDR_SPACE: Range<Address> =
    Address::from_const(0xE000_E010)..Address::from_const(0xE000_E100);

pub(crate) type InterruptId = interrupt::Id;
type BusDriver = SimpleSynchronousSlaveInterface<BusDriverSubcomponent, NVICComponent>;
type NVICRegisterBank = register_bank::NVICRegisterBank<NVICRegisterBankSubcomponent>;
type SystemControlBlock = system_control_block::SystemControlBlock<SystemControlBlockSubcomponent>;
type SysTick = systick::SysTick<SysTickSubcomponent>;

#[derive(MainComponent, TickComponent, TickComponentExtra, DisableableComponent)]
pub(crate) struct NVICComponent {
    core: CoreProxy,

    #[subcomponent(BusDriverSubcomponent)]
    bus_driver: BusDriver,
    #[subcomponent(NVICRegisterBankSubcomponent)]
    register_bank: NVICRegisterBank,
    #[subcomponent(SystemControlBlockSubcomponent)]
    system_control_block: SystemControlBlock,
    #[subcomponent(SysTickSubcomponent)]
    systick: SysTick,

    #[flop]
    core_state_change: SeqFlop<CoreStateChange>,
    /// Stores read-only copy of [`PriorityMaskRegister`]. Can be updated only
    /// by handler [`Self::update_primask()`], that is used to synchronize value.
    #[flop]
    primask_copy: SeqFlopMemoryBankSimple<PriorityMaskRegister>,
    /// Stores read-only copy of [`FaultMaskRegister`]. Can be updated only
    /// by handler [`Self::update_faultmask()`], that is used to synchronize value.
    #[flop]
    faultmask_copy: SeqFlopMemoryBankSimple<FaultMaskRegister>,
    /// Stores read-only copy of [`BasePriorityMaskRegister`]. Can be updated only
    /// by handler [`Self::update_basepri()`], that is used to synchronize value.
    #[flop]
    basepri_copy: SeqFlopMemoryBankSimple<BasePriorityMaskRegister>,
    #[flop]
    cpu_mode: SeqFlopMemoryBankSimple<CpuMode>,
    #[flop]
    interrupt_state: SeqFlopMemoryBankSimple<InterruptState>,
    #[flop]
    tail_chained_interrupt: SeqFlopMemoryBankSimple<Option<InterruptData>>,

    /// [ARM-ARM] B1.5.5 Reset behavior - "it's a conceptual array of active flag
    /// bits for all exceptions". It has been decided to keep active exceptions
    /// in that way to make implementation easier and more similar to docs.
    #[flop]
    exception_active: SeqFlopMemoryBank<[bool; EXCEPTIONS_AND_INTERRUPTS_COUNT], (usize, bool)>,

    /// Counts how many exceptions have been started, but not yet finished.
    exception_nesting_level: u8,

    /// This is used to create a scope, in which nested calls to `ExecutionPriority()` will
    /// handle the special consideration of PRIMASK during WPI.
    ///
    /// [ARM-ARM] B1.5.19 Wait For Interrupt – "The processor ignores the value of PRIMASK
    /// in determining whether an asynchronous exception is a WFI wakeup event."
    /// However, this is used normally to indicate if the new exception is taken,
    /// therefore the CPU may continue execution after the WFI instruction (as a spurious wakeup).
    wfi_primask_handling_scope: bool,
}

#[derive(Clone, Copy, Debug)]
enum InterruptState {
    None,
    /// [ARM-TRM-G] Figure 5-2 Exception entry timing.
    /// Based on this figure entry to an interrupt is started after 2 cycles
    /// after being raised. Because of that, this state is added to delay
    /// handling interrupt by one cycle.
    InterruptFoundToHandle {
        data: InterruptData,
    },
    Entry {
        id: InterruptId,
    },
    Handling {
        id: InterruptId,
    },
    TailChained {
        id: InterruptId,
    },
    Exit,
}

// pub(crate) because used by proxy (shared with core)
#[derive(Debug, Copy, Clone)]
pub(crate) enum CoreStateChange {
    FinishedEntry,
    StartedTailChain,
    FinishedTailChain,
    StartedExit,
    /// Stores preempted interrupt data (in case of nested interrupts).
    /// At the exit stage, core knows if return from handler should happen to
    /// some other interrupt, so it can send this interrupt data. Because of
    /// that [`NVICComponent`] doesn't have to keep stack of nested interrupts.
    FinishedExit(Option<InterruptData>),
}

// pub(crate) because used by proxy (shared with core)
#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct InterruptData {
    pub(in crate::component) interrupt_id: InterruptId,
}

#[component_impl(nvic)]
impl NVICComponent {
    pub(crate) fn new() -> Self {
        Self {
            core: CoreProxy::new(),

            bus_driver: BusDriver::new(),
            register_bank: NVICRegisterBank::new(),
            system_control_block: SystemControlBlock::new(),
            systick: SysTick::new(),

            core_state_change: SeqFlop::new(),
            interrupt_state: SeqFlopMemoryBankSimple::new(InterruptState::None),
            primask_copy: SeqFlopMemoryBankSimple::new(PriorityMaskRegister::new()),
            faultmask_copy: SeqFlopMemoryBankSimple::new(FaultMaskRegister::new()),
            basepri_copy: SeqFlopMemoryBankSimple::new(BasePriorityMaskRegister::new()),
            cpu_mode: SeqFlopMemoryBankSimple::new(CpuMode::Run),

            tail_chained_interrupt: SeqFlopMemoryBankSimple::new(None),

            exception_active: SeqFlopMemoryBank::new([false; EXCEPTIONS_AND_INTERRUPTS_COUNT]),

            exception_nesting_level: 0,
            wfi_primask_handling_scope: false,
        }
    }

    pub(crate) fn tick(&mut self, ctx: &mut Context) {
        BusDriver::run_driver(self, ctx);

        self.check_core_state_change();

        // [ARM-ARM] B3.3.1 - SysTick can use the processor clock or an external
        // clock, but [TI-TRM] 2.7.4.3 `CLKSOURCE` field description says that
        // an external clock is not available and writes to this field are ignored.
        // It's been verified by experiments.
        // SysTick logic has to be run after checking change of the core state,
        // because 2 things can happen at the same type:
        // - changing state of SysTick exception from pending to active,
        // - pending once again.
        // It seems that the latter has the higher priority or is in fact
        // executed after clearing pending state.
        SysTick::run_tick(self);

        // TODO: can be done better, e.g. only when state changes in a specific way.
        self.check_interrupts(ctx);
    }

    pub(crate) fn tock(&mut self, ctx: &mut Context) {
        BusDriver::tock(self, ctx);
    }

    fn inner_raise_exception(&mut self, _ctx: &mut Context, exc: InterruptId) {
        trace!("Raised exception: {:?}", exc);
        self.register_bank.set_interrupt_pending(exc);
    }

    #[handler]
    pub(crate) fn change_core_state(&mut self, _ctx: &mut Context, state_change: CoreStateChange) {
        self.core_state_change.set_next(state_change);
        trace!("Received signal from core: {:?}", state_change);
    }

    #[handler]
    pub fn raise_interrupt(&mut self, ctx: &mut Context, interrupt_id: u8) {
        debug_assert!((interrupt_id as usize) < INTERRUPTS_COUNT);
        self.inner_raise_exception(ctx, InterruptId::Interrupt(interrupt_id));
    }

    #[handler]
    #[allow(dead_code)]
    pub(crate) fn raise_exception(&mut self, ctx: &mut Context, interrupt: InterruptId) {
        self.inner_raise_exception(ctx, interrupt);
    }

    #[handler]
    pub fn on_new_ahb_slave_input(
        &mut self,
        ctx: &mut Context,
        msg: MasterToSlaveWires<<NVICComponent as AHBPortConfig>::Data>,
    ) {
        <Self as AHBSlavePortInput>::on_ahb_input(self, ctx, msg);
    }

    #[handler]
    pub(crate) fn update_primask(&mut self, _ctx: &mut Context, primask: PriorityMaskRegister) {
        self.primask_copy.set_next(primask);
    }

    #[handler]
    pub(crate) fn update_faultmask(&mut self, _ctx: &mut Context, faultmask: FaultMaskRegister) {
        self.faultmask_copy.set_next(faultmask);
    }

    #[handler]
    pub(crate) fn update_basepri(&mut self, _ctx: &mut Context, basepri: BasePriorityMaskRegister) {
        self.basepri_copy.set_next(basepri);
    }

    #[handler]
    pub(crate) fn start_sleep(&mut self, ctx: &mut Context) {
        let deep_sleep = self
            .system_control_block
            .scr()
            .read(Word::from(!0))
            .get_bit(2);

        let cpu_mode = deep_sleep.ife(CpuMode::DeepSleep, CpuMode::Sleep);
        self.cpu_mode.set_next(cpu_mode);

        // Drive SLEEPING to low (aka send a msg to PRCM)
        PRCMProxy.on_cpu_mode(ctx, cpu_mode);
    }
}

// TODO: try reworking SlaveDriver to impl the slave interface for the assigned port
bridge_ports!(@slave NVICComponent =>  @slave BusDriver);

#[component_impl(nvic)]
impl AHBPortConfig for NVICComponent {
    type Data = DataBus;
    type Component = Self;
    const TAG: &'static str = "Nvic";
}

#[component_impl(nvic)]
impl AHBSlavePortProxiedInput for NVICComponent {
    fn proxy_ahb_input(ctx: &mut Context, msg: MasterToSlaveWires<Self::Data>) {
        NVICProxy.on_new_ahb_slave_input(ctx, msg);
    }
}

#[component_impl(nvic)]
impl NVICComponent {
    fn check_core_state_change(&mut self) {
        if self.core_state_change.is_set() {
            let next_state = match *self.core_state_change {
                CoreStateChange::FinishedEntry => {
                    if let InterruptState::Entry { id } = *self.interrupt_state {
                        self.activate_interrupt(id);
                        InterruptState::Handling { id }
                    } else {
                        panic!(
                            "Tried to finish entry in state {:?}.",
                            *self.interrupt_state
                        )
                    }
                }
                CoreStateChange::StartedTailChain => {
                    if let InterruptState::Handling { id } = *self.interrupt_state {
                        debug_assert!((*self.tail_chained_interrupt).is_some());

                        self.deactivate_interrupt(id);
                        InterruptState::TailChained {
                            id: self.tail_chained_interrupt.unwrap().interrupt_id,
                        }
                    } else {
                        panic!("Tried to tail chain in state {:?}.", *self.interrupt_state)
                    }
                }
                CoreStateChange::FinishedTailChain => {
                    if let InterruptState::TailChained { id } = *self.interrupt_state {
                        self.activate_interrupt(id);
                        InterruptState::Handling { id }
                    } else {
                        panic!(
                            "Tried to finish tail chain in state {:?}.",
                            *self.interrupt_state
                        )
                    }
                }
                CoreStateChange::StartedExit => {
                    if let InterruptState::Handling { id } = *self.interrupt_state {
                        debug_assert!((*self.tail_chained_interrupt).is_none());

                        self.deactivate_interrupt(id);
                        InterruptState::Exit
                    } else {
                        panic!("Tried to start exit in state {:?}.", *self.interrupt_state)
                    }
                }
                CoreStateChange::FinishedExit(preempted_interrupt) => {
                    if let InterruptState::Exit = *self.interrupt_state {
                        preempted_interrupt.map_or(InterruptState::None, |interrupt_data| {
                            InterruptState::Handling {
                                id: interrupt_data.interrupt_id,
                            }
                        })
                    } else {
                        panic!("Tried to finish exit in state {:?}.", *self.interrupt_state)
                    }
                }
            };
            self.interrupt_state.set_next(next_state);
        }
    }
}

macro_rules! manual_unroll{
    (for $i:ident in [$($xs:expr),* $(,)?] $(.chain($e:expr))? $b:block) => {
        $(
        {
            let $i = $xs;
            $b
        }
        )*
        $(
        {
            for $i in $e {
                $b
            }
        }
        )?
    }
}
// ---------------------------------------------------------------------------
// interrupt checking logic
// ---------------------------------------------------------------------------
#[component_impl(nvic)]
impl NVICComponent {
    fn check_interrupts(&mut self, ctx: &mut Context) {
        #[allow(clippy::single_match)]
        match *self.interrupt_state {
            InterruptState::None => {
                if let Some(interrupt_data) = self.find_interrupt_to_handle() {
                    self.interrupt_state
                        .set_next(InterruptState::InterruptFoundToHandle {
                            data: interrupt_data,
                        });
                } else if *self.cpu_mode != CpuMode::Run {
                    self.check_non_exc_wakeup_event(ctx);
                }
            }
            InterruptState::InterruptFoundToHandle { data } => {
                if let InterruptId::Interrupt(_) = data.interrupt_id {
                    self.set_icsr_interrupt_pending(
                        data.interrupt_id.as_exception_number().try_into().unwrap(),
                    );
                }
                self.interrupt_state.set_next(InterruptState::Entry {
                    id: data.interrupt_id,
                });
                self.core.start_interrupt_entry(ctx, data);
                if *self.cpu_mode != CpuMode::Run {
                    self.wakeup_cpu(ctx);
                }
            }
            InterruptState::Handling { id } => {
                // [ARM-ARM] B1.5.12 Tail chain happens when there is pending
                // interrupt with priority higher than the target priority (explained
                // in `target_priority`).
                let found_pending_interrupt =
                    self.find_pending_interrupt_with_higher_priority(self.target_priority(id));

                let interrupt_for_tail_chaining =
                    found_pending_interrupt.and_then(|found_interrupt| {
                        let found_interrupt_priority =
                            self.get_interrupt_priority(found_interrupt.interrupt_id);
                        let execution_priority = self.execution_priority();

                        if found_interrupt_priority < execution_priority {
                            self.interrupt_state
                                .set_next(InterruptState::InterruptFoundToHandle {
                                    data: found_interrupt,
                                });
                            None
                        } else {
                            found_pending_interrupt
                        }
                    });

                if interrupt_for_tail_chaining != *self.tail_chained_interrupt {
                    if let Some(InterruptData {
                        interrupt_id: interrupt_id @ InterruptId::Interrupt(_),
                    }) = interrupt_for_tail_chaining
                    {
                        self.set_icsr_interrupt_pending(
                            interrupt_id.as_exception_number().try_into().unwrap(),
                        );
                    }

                    self.core.tail_chain_interrupt(ctx, found_pending_interrupt);
                    self.tail_chained_interrupt
                        .set_next(found_pending_interrupt);
                }
            }
            // TODO add support for late arrivals
            _ => {}
        }
    }

    fn set_icsr_interrupt_pending(&mut self, exception_number: u32) {
        self.system_control_block.icsr_mut().set_isrpending();
        self.system_control_block
            .icsr_mut()
            .set_vectpending(exception_number);
    }

    fn find_interrupt_to_handle(&self) -> Option<InterruptData> {
        let max_priority = self.execution_priority();
        self.find_pending_interrupt_with_higher_priority(max_priority)
    }

    fn find_pending_interrupt_with_higher_priority(
        &self,
        mut max_priority: i32,
    ) -> Option<InterruptData> {
        let mut interrupt_id_to_handle = None;

        // Checking interrupts starts from the one with the lowest id - NMI.
        // XXX: DUMB RUST unrolls the loop first and then fails to propagate constants therefore bloating this func
        //      Like bloating from 20 instructions to a 1000 then taking 10% of emulation time
        manual_unroll!(for interrupt_id in [
            InterruptId::NMI,
            InterruptId::HardFault,
            InterruptId::MemManage,
            InterruptId::BusFault,
            InterruptId::UsageFault,
            InterruptId::SVCall,
            InterruptId::DebugMonitor,
            InterruptId::PendSV,
            InterruptId::SysTick,
        ] {
            if self.exception_is_enabled_and_pending(interrupt_id) {
                let priority = self.get_interrupt_priority(interrupt_id);
                // Lower priority value means higher priority level.
                if priority < max_priority {
                    interrupt_id_to_handle = Some(interrupt_id);
                    max_priority = priority;
                }
            }
        });

        if self.register_bank.is_any_interrupt_pending()
            && self.register_bank.is_any_interrupt_enabled()
        {
            interrupt_id_to_handle = InterruptId::iterate_interrupts()
                .filter(|&id| self.exception_is_enabled_and_pending(id))
                .min_by_key(|&id| self.get_interrupt_priority(id))
                .and_then(|id| (self.get_interrupt_priority(id) < max_priority).then_some(id))
                .or(interrupt_id_to_handle);
        }

        interrupt_id_to_handle.map(|interrupt_id| InterruptData { interrupt_id })
    }

    fn exception_is_enabled_and_pending(&self, id: InterruptId) -> bool {
        match id {
            InterruptId::NMI => {
                self.system_control_block.icsr().get_nmipendset()
                    && !self.exception_active[InterruptId::NMI.as_exception_number()]
            }
            InterruptId::SysTick => self.system_control_block.icsr().get_pendstset(),
            InterruptId::Interrupt(_) => {
                self.register_bank.get_interrupt_enabled(id)
                    && self.register_bank.get_interrupt_pending(id)
            }
            InterruptId::Reset
            | InterruptId::HardFault
            | InterruptId::MemManage
            | InterruptId::BusFault
            | InterruptId::UsageFault
            | InterruptId::SVCall
            | InterruptId::DebugMonitor
            | InterruptId::PendSV => {
                // TODO: Change to checking corresponding registers for each
                // interrupt (ARCR, ICSR and SHCSR not implemented yet).
                false
            }
        }
    }

    /// Check for wakeup events according to [ARM-ARM] B1.5.9 Wait For Interrupt
    ///
    /// NVIC manages the wakeup of the processor, and in the `DeepSleep` state,
    /// this may be partly delegated to a Wakeup Controller (WUC).
    /// The processor will wake up from WFI on:
    /// - Reset - handled externally
    /// - Debug Event
    /// - Implementation defined event (are there any?)
    /// - Async exception preempting the current priority (handled in main exc logic)
    /// - Async exception that would preempt if PRIMASK were zero – just causes a wake-up
    ///   (handled here)
    fn check_non_exc_wakeup_event(&mut self, ctx: &mut Context) {
        if self.primask_copy.primask() {
            let would_preempt_if_not_primask = {
                self.wfi_primask_handling_scope = true;
                // Fixup on possible unwinding
                let this = guard(&mut *self, |this| this.wfi_primask_handling_scope = false);
                this.find_interrupt_to_handle()
            };
            trace!("NVIC check for spurious wakeup. PRIMASK=0 => {would_preempt_if_not_primask:?}");
            if would_preempt_if_not_primask.is_some() {
                self.core.spurious_wakeup(ctx);
                self.wakeup_cpu(ctx);
            }
        }
    }

    fn wakeup_cpu(&mut self, ctx: &mut Context) {
        self.cpu_mode.set_next(CpuMode::Run);
        // Set SLEEPING wire to low
        PRCMProxy.on_cpu_mode(ctx, CpuMode::Run);
    }
}

// ---------------------------------------------------------------------------
// priority logic
// ---------------------------------------------------------------------------
#[component_impl(nvic)]
impl NVICComponent {
    /// [ARM-ARM] B1.5.4 Exception priorities and preemption
    /// - `ExecutionPriority()` pseudocode.
    /// This code and pseudocode differ on including PRIGROUP effect.
    /// According to the [ARM-ARM] Priority grouping: "The group priorities of
    /// Reset, NMI and `HardFault` are -3, -2, and -1 respectively, regardless of
    /// the value of PRIGROUP.". The reason for this change is that pseudocode
    /// doesn't produce the same results.
    fn execution_priority(&self) -> i32 {
        let mut highest_priority = interrupt::THREAD_MODE_PRIORITY;

        // TODO: use array bit-set and this would be just two loads and or
        if self.exception_active.iter().any(|&x| x) {
            // XXX: DUMB RUST unrolls the loop first and then fails to propagate constants therefore bloating this func
            //      Like bloating from 20 instructions to a 1000
            manual_unroll!(for interrupt_id in [
                InterruptId::NMI,
                InterruptId::HardFault,
                InterruptId::MemManage,
                InterruptId::BusFault,
                InterruptId::UsageFault,
                InterruptId::SVCall,
                InterruptId::DebugMonitor,
                InterruptId::PendSV,
                InterruptId::SysTick,
            ]
            .chain(InterruptId::iterate_interrupts())
            {
                if self.exception_active[interrupt_id.as_exception_number()] {
                    let priority = self.get_interrupt_priority(interrupt_id);
                    // Lower priority value means higher priority level.
                    if priority < highest_priority {
                        highest_priority = self.compute_group_priority(priority);
                    }
                }
            });
        }

        let boosted_priority = self.get_boosted_priority();

        if boosted_priority < highest_priority {
            boosted_priority
        } else {
            highest_priority
        }
    }

    /// Implements [ARM-ARM] B1.5.4 - Priority grouping and is part of
    /// [`NVICComponent::execution_priority`].
    /// Includes the PRIGROUP effect as shown in [ARM-ARM] B1.5.4 -
    /// `ExecutionPriority()` pseudocode. Priorities < 0 have the group priority
    /// equal to their own priority.
    fn compute_group_priority(&self, priority: i32) -> i32 {
        // [ARM-ARM] B1.5.4 - Priority grouping.
        // Priorities < 0 have the group priority equal to their own priority.
        if priority < 0 {
            return priority;
        }

        let subgroup_shift = self.system_control_block.aircr().get_prigroup();
        let group_value: i32 = Word::from_const(0b0000_0010).lsl(subgroup_shift).into();

        debug_assert!(0 <= priority && 0 <= group_value);

        // For non-negative values % operator has the same effect as
        // MOD described in [ARM-ARM] D6.5.4 - Division and modulo.
        // https://doc.rust-lang.org/std/primitive.i32.html#impl-Rem%3Ci32%3E
        let subgroup_value = priority % group_value;
        priority - subgroup_value
    }

    fn get_interrupt_priority(&self, id: InterruptId) -> i32 {
        match id {
            InterruptId::Reset => interrupt::RESET_PRIORITY,
            InterruptId::NMI => interrupt::NMI_PRIORITY,
            InterruptId::HardFault => interrupt::HARD_FAULT_PRIORITY,
            InterruptId::SysTick => {
                i32::from(self.system_control_block.shpr3().get_systick_priority())
            }
            InterruptId::Interrupt(_) => i32::from(self.register_bank.get_interrupt_priority(id)),
            InterruptId::MemManage
            | InterruptId::BusFault
            | InterruptId::UsageFault
            | InterruptId::SVCall
            | InterruptId::DebugMonitor
            | InterruptId::PendSV => {
                // TODO: Change to reading value from SHPR - not implemented yet.
                unimplemented!()
            }
        }
    }

    /// [ARM-ARM] B1.5.4 - Priority boosting.
    /// This function corresponds to the code that computes boosted priority in
    /// `ExecutionPriority()`.
    fn get_boosted_priority(&self) -> i32 {
        // Priority influence of BASEPRI, PRIMASK and FAULTMASK.
        let mut boosted_priority = interrupt::THREAD_MODE_PRIORITY;

        let basepri: i32 = Word::from(*self.basepri_copy).into();
        if basepri != 0 {
            boosted_priority = self.compute_group_priority(basepri);
        }

        // [ARM-ARM] B1.5.19 Wait For Interrupt
        // Value of PRIMASK is ignored in determining whether an exception is a wakeup event.
        if self.primask_copy.primask() && !self.wfi_primask_handling_scope {
            boosted_priority = 0;
        }

        if self.faultmask_copy.faultmask() {
            boosted_priority = -1;
        }

        boosted_priority
    }

    /// [ARM-ARM] B1.5.12.
    /// Target priority is explained as "the higher of:
    /// - The priority of the highest priority active exception,
    ///   excluding the exception being returned from.
    /// - The boosted priority set by the special-purpose mask registers."
    /// Conducted tests have shown that target priority could be also priority
    /// of the thread mode.
    fn target_priority(&self, interrupt_return_from: InterruptId) -> i32 {
        let mut highest_priority = interrupt::THREAD_MODE_PRIORITY;

        for id in InterruptId::NMI.as_exception_number()..EXCEPTIONS_AND_INTERRUPTS_COUNT {
            if InterruptId::is_reserved(id) {
                continue;
            }

            let interrupt_id = InterruptId::try_from_exception_number(id).unwrap();
            if interrupt_id == interrupt_return_from {
                continue;
            }

            if self.exception_active[id] {
                let priority = self.get_interrupt_priority(interrupt_id);
                // Lower priority value means higher priority level.
                if priority < highest_priority {
                    highest_priority = priority;
                }
            }
        }

        let boosted_priority = self.get_boosted_priority();

        if boosted_priority < highest_priority {
            boosted_priority
        } else {
            highest_priority
        }
    }
}

// ---------------------------------------------------------------------------
// memory writing helpers
// ---------------------------------------------------------------------------
#[component_impl(nvic)]
impl NVICComponent {
    fn set_data_for_address(&mut self, ctx: &mut Context, req: WriteRequest) {
        let WriteRequest { addr, data, mask } = req;

        trace!(
            "Writing to address: {:?}, value {:x}, mask {:x}",
            addr, data, mask
        );

        address_match_range! {addr,
            SET_ENABLE_ADDR_SPACE => {
                let offset = addr.offset_from(SET_ENABLE_ADDR_SPACE.start);
                self.register_bank
                    .write_to_interrupt_set_enable(offset, data, mask);
            },
            CLEAR_ENABLE_ADDR_SPACE => {
                let offset = addr.offset_from(CLEAR_ENABLE_ADDR_SPACE.start);
                self.register_bank
                    .write_to_interrupt_clear_enable(offset, data, mask);
            },
            SET_PENDING_ADDR_SPACE => {
                let offset = addr.offset_from(SET_PENDING_ADDR_SPACE.start);
                self.register_bank
                    .write_to_interrupt_set_pending(offset, data, mask);
            },
            CLEAR_PENDING_ADDR_SPACE => {
                let offset = addr.offset_from(CLEAR_PENDING_ADDR_SPACE.start);
                self.register_bank
                    .write_to_interrupt_clear_pending(offset, data, mask);
            },
            PRIORITY_ADDR_SPACE => {
                let offset = addr.offset_from(PRIORITY_ADDR_SPACE.start);
                self.register_bank
                    .write_to_interrupt_priority(offset, data, mask);
            },
            SCB_ADDR_SPACE => {
                SystemControlBlock::write_register(self, ctx, req);
            },
            SYSTICK_ADDR_SPACE => {
                SysTick::write_register(self, ctx, req);
            },
            stir::STIR_ADDR => stir::SoftwareTriggerInterruptRegister::write(self, data),
            _ => panic!("Cannot write to memory adress: {addr:?}"),
        }
    }
}

// ---------------------------------------------------------------------------
// memory reading helpers
// ---------------------------------------------------------------------------
#[component_impl(nvic)]
impl NVICComponent {
    fn get_data_for_address(&mut self, req: ReadRequest) -> Word {
        let ReadRequest { addr, mask } = req;

        address_match_range! {addr,
            SET_ENABLE_ADDR_SPACE => {
                let offset = addr.offset_from(SET_ENABLE_ADDR_SPACE.start);
                self.register_bank.read_interrupt_enabled(offset, mask)
            },
            CLEAR_ENABLE_ADDR_SPACE => {
                let offset = addr.offset_from(CLEAR_ENABLE_ADDR_SPACE.start);
                self.register_bank.read_interrupt_enabled(offset, mask)
            },
            SET_PENDING_ADDR_SPACE => {
                let offset = addr.offset_from(SET_PENDING_ADDR_SPACE.start);
                self.register_bank.read_interrupt_pending(offset, mask)
            },
            CLEAR_PENDING_ADDR_SPACE => {
                let offset = addr.offset_from(CLEAR_PENDING_ADDR_SPACE.start);
                self.register_bank.read_interrupt_pending(offset, mask)
            },
            ACTIVE_ADDR_SPACE => {
                let offset = addr.offset_from(ACTIVE_ADDR_SPACE.start);
                self.register_bank.read_interrupt_active(offset, mask)
            },
            PRIORITY_ADDR_SPACE => {
                let offset = addr.offset_from(PRIORITY_ADDR_SPACE.start);
                self.register_bank.read_interrupt_priority(offset, mask)
            },
            SCB_ADDR_SPACE => SystemControlBlock::read_register(self, req),
            SYSTICK_ADDR_SPACE => SysTick::read_register(self, req),
            // FIXME: STIR can only be written by unprivileged when a special bit in CSR is set
            stir::STIR_ADDR => stir::SoftwareTriggerInterruptRegister::read(),
            _ => panic!("We cannot read from memory adresses: {addr:?}"),
        }
    }
}

#[component_impl(nvic)]
impl NVICComponent {
    fn activate_interrupt(&mut self, id: InterruptId) {
        // In case of entry to interrupt, its registers have to be updated.
        #[allow(clippy::cast_possible_truncation)]
        self.system_control_block
            .icsr_mut()
            .set_vectactive(id.as_exception_number() as u32);

        self.increase_exception_nesting_level();

        match id {
            InterruptId::NMI => {
                self.system_control_block.icsr_mut().set_nmipendset();
            }
            InterruptId::MemManage => {
                self.system_control_block.shcsr_mut().clear_memfaultpended();
                self.system_control_block.shcsr_mut().set_memfaultact();
            }
            InterruptId::BusFault => {
                self.system_control_block.shcsr_mut().clear_busfaultpended();
                self.system_control_block.shcsr_mut().set_busfaultact();
            }
            InterruptId::UsageFault => {
                self.system_control_block.shcsr_mut().clear_usgfaultpended();
                self.system_control_block.shcsr_mut().set_usgfaultact();
            }
            InterruptId::SVCall => {
                self.system_control_block.shcsr_mut().clear_svcallpended();
                self.system_control_block.shcsr_mut().set_svcallact();
            }
            InterruptId::PendSV => {
                self.system_control_block.icsr_mut().clear_pendsvset();
                self.system_control_block.shcsr_mut().set_pendsvact();
            }
            InterruptId::SysTick => {
                self.system_control_block.icsr_mut().clear_pendstset();
                self.system_control_block.shcsr_mut().set_systickact();
            }
            InterruptId::Interrupt(_) => {
                self.register_bank.clear_interrupt_pending(id);
                self.register_bank.set_interrupt_active(id);

                // It is safe to clear vectpending and isrpending even if there is some exception
                // pending because call to `check_interrupts` in the next cycle is going to find
                // any pending exceptions and set vectpending accordingly.
                self.system_control_block.icsr_mut().clear_isrpending();
                self.system_control_block.icsr_mut().clear_vectpending();
            }
            _ => (),
        }

        self.exception_active.mutate_next(
            (id.as_exception_number(), true),
            |exception_active, (idx, value)| exception_active[idx] = value,
        );
    }

    fn deactivate_interrupt(&mut self, id: InterruptId) {
        self.exception_active.mutate_next(
            (id.as_exception_number(), false),
            |exception_active, (idx, value)| exception_active[idx] = value,
        );

        self.system_control_block.icsr_mut().clear_vectactive();

        self.decrease_exception_nesting_level();

        match id {
            InterruptId::NMI => self.system_control_block.icsr_mut().clear_nmipendset(),
            InterruptId::MemManage => self.system_control_block.shcsr_mut().clear_memfaultact(),
            InterruptId::BusFault => self.system_control_block.shcsr_mut().clear_busfaultact(),
            InterruptId::UsageFault => self.system_control_block.shcsr_mut().clear_usgfaultact(),
            InterruptId::SVCall => self.system_control_block.shcsr_mut().clear_svcallact(),
            InterruptId::PendSV => self.system_control_block.shcsr_mut().clear_pendsvact(),
            InterruptId::SysTick => self.system_control_block.shcsr_mut().clear_systickact(),
            InterruptId::Interrupt(_) => self.register_bank.clear_interrupt_active(id),
            _ => unimplemented!("No more exceptions are supported."),
        }
    }

    fn increase_exception_nesting_level(&mut self) {
        self.exception_nesting_level += 1;

        if self.exception_nesting_level > 1 {
            self.system_control_block.icsr_mut().clear_rettobase();
        } else {
            self.system_control_block.icsr_mut().set_rettobase();
        }
    }

    fn decrease_exception_nesting_level(&mut self) {
        debug_assert!(self.exception_nesting_level > 0);
        self.exception_nesting_level -= 1;

        if self.exception_nesting_level == 0 {
            self.system_control_block.icsr_mut().clear_rettobase();
        }
    }
}

// Almost same as "skippable_if_disableable", but we have SysTick to handle!
#[component_impl(nvic)]
impl SkippableClockTreeNode for NVICComponent {
    fn max_cycles_to_skip(
        comp: &mut Self::Component,
        _ctx: &mut Context,
        _parent: Self::IdSpace,
        _extra: &mut Self::Extra,
    ) -> u64 {
        let this = comp;
        if this.can_be_disabled_now()
        // && todo!("systick")
        {
            u64::MAX
        } else {
            0
        }
    }

    fn emulate_skipped_cycles(
        _comp: &mut Self::Component,
        _ctx: &mut Context,
        _parent: Self::IdSpace,
        _extra: &mut Self::Extra,
        skipped_cycles: u64,
    ) {
        trace!("NVIC::emulate_skipped_cycles({skipped_cycles:?})");
        warn!("TODO: SysTick cycles count was not updated on skipping cycles!");
    }
}

// ---------------------------------------------------------------------------
// Bus Driver configuration
// ---------------------------------------------------------------------------

/// A request to write a word, halfword or byte to NVIC address space.
#[derive(Clone, Copy)]
struct WriteRequest {
    /// The address of the write. (Not necessarily word-aligned.)
    addr: Address,
    /// The data to be written, padded to a full aligned word.
    /// For a write of 0x42 to bits 8-15 of a word it will be 0x00004200.
    data: Word,
    /// Selects the bytes of data to be written to the memory word containing this write.
    /// One of 0x000000FF, 0x0000FF00, 0x00FF0000, 0xFF000000, 0x0000FFFF, 0xFFFF0000, 0xFFFFFFFF.
    /// For a write of 0x42 to bits 8-15 of a word it will be 0x0000FF00.
    mask: Word,
}

/// A request to read a word, halfword or byte from NVIC address space.
#[derive(Clone, Copy)]
struct ReadRequest {
    /// The address of the read. (Not necessarily word-aligned.)
    addr: Address,
    /// Selects the bytes of data to be read from the memory word containing this read.
    /// One of 0x000000FF, 0x0000FF00, 0x00FF0000, 0xFF000000, 0x0000FFFF, 0xFFFF0000, 0xFFFFFFFF.
    /// For a read of 0x42 from bits 8-15 of a word it will be 0x0000FF00.
    mask: Word,
}

#[component_impl(nvic)]
impl SimpleHandler for NVICComponent {
    const WRITE_MODE: WriteMode = WriteMode::Combinatorial;

    fn read_data(
        slave: &mut Self::Component,
        _ctx: &mut Context,
        request: TransferMeta,
    ) -> SimpleResponse<Self::Data> {
        let addr = request.addr;
        let mask = DataBus::build_word_mask(request.addr, request.size);
        let value = slave.get_data_for_address(ReadRequest { addr, mask });
        SimpleResponse::Success(DataBus::extract_from_word(
            value,
            request.addr,
            request.size,
        ))
    }

    fn pre_write(
        _slave: &mut Self::Component,
        _ctx: &mut Context,
        _request: TransferMeta,
    ) -> SimpleWriteResponse {
        SimpleResponse::Success(())
    }

    fn write_data(
        slave: &mut Self::Component,
        ctx: &mut Context,
        request: TransferMeta,
        data: Self::Data,
        post_success: bool,
    ) -> SimpleWriteResponse {
        debug_assert!(post_success, "Not waitstates in NVIC");
        let addr = request.addr;
        let mask = DataBus::build_word_mask(request.addr, request.size);
        let data = DataBus::emplace_in_word(Word::from_const(0), addr, data);
        slave.set_data_for_address(ctx, WriteRequest { addr, data, mask });
        SimpleWriteResponse::SUCCESS
    }
}

mod interrupt {
    use super::{EXCEPTIONS_AND_INTERRUPTS_COUNT, EXCEPTIONS_COUNT, INTERRUPTS_COUNT};

    /// [ARM-TDG] Table 7.1 - List of System Exceptions.
    pub(super) const RESET_PRIORITY: i32 = -3;
    /// [ARM-TDG] Table 7.1 - List of System Exceptions.
    pub(super) const NMI_PRIORITY: i32 = -2;
    /// [ARM-TDG] Table 7.1 - List of System Exceptions.
    pub(super) const HARD_FAULT_PRIORITY: i32 = -1;
    /// [ARM-ARM] B1.5.4 Exception priorities and preemption -
    /// `ExecutionPriority()` pseudocode.
    /// Priority of Thread mode with no active exceptions. The value is
    /// `PriorityMax` + 1 = 256 (configurable priority maximum bit field is 8 bits).
    pub(super) const THREAD_MODE_PRIORITY: i32 = 256;

    /// [ARM-ARM] B1.5.2 Exception number definition - Table B1-4.
    ///
    /// Represents id of exceptions and interrupts.
    /// According to [ARM-TDG] 7.1, exceptions are numbered from 1 to `EXCEPTIONS_COUNT`-1
    /// for system exceptions and `EXCEPTIONS_COUNT` and above for external interrupt inputs.
    /// This number is called `exception_number` ([ARM-ARM] B1.5.2).
    /// For an external interrupt, there is also `interrupt number`. It is equal
    /// to (exception number - `EXCEPTIONS_COUNT`) and it corresponds to bit in
    /// interrupt registers ([TI-TRM] Table 4-2).
    /// `interrupt_number` is defined in [ARM-ARM] B3.4.1 NVIC operation - the last note.
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub enum Id {
        Reset,
        NMI,
        HardFault,
        MemManage,
        BusFault,
        UsageFault,
        SVCall,
        DebugMonitor,
        PendSV,
        SysTick,
        Interrupt(u8), // contains `interrupt_number`
    }

    impl Id {
        pub(in crate::component) fn iterate_interrupts() -> impl Iterator<Item = Self> {
            #[allow(clippy::cast_possible_truncation)]
            (0u8..(INTERRUPTS_COUNT as u8)).map(Self::Interrupt)
        }

        /// [ARM-ARM] B1.5.2 Exception number definition - Table B1-4.
        pub(in crate::component) const fn try_from_exception_number(
            val: usize,
        ) -> Result<Self, &'static str> {
            const LAST_INTERRUPT_NO: usize = EXCEPTIONS_AND_INTERRUPTS_COUNT - 1;
            match val {
                1 => Ok(Id::Reset),
                2 => Ok(Id::NMI),
                3 => Ok(Id::HardFault),
                4 => Ok(Id::MemManage),
                5 => Ok(Id::BusFault),
                6 => Ok(Id::UsageFault),
                11 => Ok(Id::SVCall),
                12 => Ok(Id::DebugMonitor),
                14 => Ok(Id::PendSV),
                15 => Ok(Id::SysTick),
                EXCEPTIONS_COUNT..=LAST_INTERRUPT_NO => {
                    #[allow(clippy::cast_possible_truncation)]
                    let val = (val - EXCEPTIONS_COUNT) as u8;
                    Ok(Id::Interrupt(val))
                }
                _ => Err("Given id value is invalid."),
            }
        }

        /// [ARM-ARM] B1.5.2 Exception number definition - Table B1-4.
        pub(in crate::component) const fn as_exception_number(self) -> usize {
            match self {
                Id::Reset => 1,
                Id::NMI => 2,
                Id::HardFault => 3,
                Id::MemManage => 4,
                Id::BusFault => 5,
                Id::UsageFault => 6,
                Id::SVCall => 11,
                Id::DebugMonitor => 12,
                Id::PendSV => 14,
                Id::SysTick => 15,
                #[allow(clippy::cast_lossless)]
                #[allow(clippy::cast_possible_truncation)]
                Id::Interrupt(number) => (number as usize) + EXCEPTIONS_COUNT,
            }
        }

        pub(super) const fn as_interrupt_number(self) -> usize {
            match self {
                Self::Interrupt(number) => {
                    let number = number as usize;
                    debug_assert!(number < INTERRUPTS_COUNT);
                    number
                }
                _ => panic!("Function used on non-interrupt id"),
            }
        }

        /// [ARM-ARM] B1.5.2 Exception number definition - Table B1-4.
        pub(super) const fn is_reserved(val: usize) -> bool {
            matches!(val, 7..=10 | 13)
        }
    }
}
