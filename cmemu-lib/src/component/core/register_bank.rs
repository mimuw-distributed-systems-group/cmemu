#[cfg(debug_assertions)]
use std::cell::Cell;
use std::fmt;
use std::fmt::{Display, Formatter};

use cc2650_constants::operation::{ExecutionMode, InstructionSetState, StackPointer};
use itertools::Itertools;
#[cfg_attr(not(debug_assertions), allow(unused_imports))]
use log::{debug, trace, warn};
use owo_colors::OwoColorize;

use super::instruction::Condition;
use crate::common::bitstring::bitfield::ExpandedBitfield;
use crate::common::{BitstringUtils, Word, bitstring::constants as bsc};
use crate::component::core::{CoreComponent, instruction::Instruction};
use crate::engine::Context;
use crate::engine::{
    DisableableComponent, SeqFlopMemoryBank, SeqFlopMemoryBankSimple, Subcomponent, TickComponent,
    TickComponentExtra,
};
#[cfg(feature = "cycle-debug-logger")]
use crate::proxy::CycleDebugLoggerProxy;
use crate::utils::IfExpr;
use crate::{Bitstring, bitfield, bitstring_concat, bitstring_extract, bitstring_substitute};

#[derive(Subcomponent, TickComponent, DisableableComponent)]
pub(super) struct RegisterBank<SC>
where
    SC: Subcomponent<Member = Self>,
{
    #[flop]
    core_registers: SeqFlopMemoryBank<CoreRegisters, (RegisterName, Word)>,

    /// PC is only valid when `PCContext` is alive
    /// See: `RegisterBank::with_pc(...)` and `PCContext`
    pc: Option<Word>,

    #[flop]
    xpsr: SeqFlopMemoryBankSimple<XPSR>,

    #[flop]
    agu_result: SeqFlopMemoryBankSimple<Word>, // address calculated by AGU

    #[flop]
    primask: SeqFlopMemoryBankSimple<PriorityMaskRegister>,

    #[flop]
    faultmask: SeqFlopMemoryBankSimple<FaultMaskRegister>,

    #[flop]
    basepri: SeqFlopMemoryBankSimple<BasePriorityMaskRegister>,

    #[flop]
    control: SeqFlopMemoryBankSimple<ControlRegister>,

    // XXX(cm4): This is temporary hack to make the RegisterBank have two write ports,
    //      a more long-term solution is required.
    #[cfg(feature = "soc-cc2652")]
    second_write_port_hack: Option<(RegisterName, Word)>,
    phantom_subcomponent: std::marker::PhantomData<SC>,
}

/// Defines scope where `RegisterBank` contains valid program counter (R15).
/// `PCContext` must be manually released before getting out of its scope.
/// Make sure to use same core instance. Not automated RAII, because of borrow checker.
pub(super) struct PCContext<SC>
where
    SC: Subcomponent<Member = RegisterBank<SC>>,
{
    #[cfg(debug_assertions)]
    released: Cell<bool>,

    phantom_sc: std::marker::PhantomData<SC>,
}
impl<SC> TickComponentExtra for RegisterBank<SC>
where
    SC: Subcomponent<Member = Self>,
{
    fn tick_extra(&mut self) {
        // TODO(cm4): temporary hack, see a comment in the struct definition
        #[cfg(feature = "soc-cc2652")]
        if let Some((name, v)) = self.second_write_port_hack.take() {
            let core_registers = self.core_registers.unsafe_as_mut();
            Self::_do_register_writing(core_registers, (name, v));
        }
    }
}

impl<SC> RegisterBank<SC>
where
    SC: Subcomponent<Member = Self>,
{
    pub(super) fn new() -> Self {
        Self {
            core_registers: SeqFlopMemoryBank::new(CoreRegisters::new()),
            pc: None,
            xpsr: SeqFlopMemoryBankSimple::new(XPSR::new()),

            agu_result: SeqFlopMemoryBankSimple::new(Word::from(0)),

            primask: SeqFlopMemoryBankSimple::new(PriorityMaskRegister::new()),

            faultmask: SeqFlopMemoryBankSimple::new(FaultMaskRegister::new()),

            basepri: SeqFlopMemoryBankSimple::new(BasePriorityMaskRegister::new()),

            control: SeqFlopMemoryBankSimple::new(ControlRegister::new()),

            #[cfg(feature = "soc-cc2652")]
            second_write_port_hack: None,
            phantom_subcomponent: std::marker::PhantomData,
        }
    }
}

impl<SC> RegisterBank<SC>
where
    SC: Subcomponent<Member = Self>,
{
    pub(super) fn get_register(component: &SC::Component, register: RegisterID) -> Word {
        let this = SC::component_to_member(component);
        match register {
            RegisterID::SP => match this.look_up_sp() {
                StackPointer::Main => this.core_registers.sp_main,
                StackPointer::Process => this.core_registers.sp_process,
            },
            RegisterID::LR => this.core_registers.lr,
            RegisterID::PC => this.pc.expect("PC should be read in PCContext scope only"),
            _ => this.core_registers.regs[register.as_array_index()],
        }
    }

    pub(super) fn set_register(
        component: &mut SC::Component,
        register: RegisterID,
        mut value: Word,
    ) {
        let this = SC::component_to_member(component);
        // [ARM-ARM] B1.4.7 Pseudocode details of ARM core register accesses
        // The ARMv7-M architecture guarantees that stack pointer values are at least 4-byte aligned:
        // when 13 _R[LookUpSP()] = value<31:2>:'00';
        // TODO: On the other side, [ARM-ARM] A5.1.3 Use of 0b1101 as register specifier states that
        //   Bits[1:0] of SP must be treated as SBZP (Should Be Zero or Preserved).
        //   Writing a non-zero value to bits[1:0] results in UNPREDICTABLE behavior.
        if register == RegisterID::SP {
            #[cfg(debug_assertions)]
            if value.uint() & 3 != 0 {
                warn!(
                    "Writing non-zero bits to SP[1:0]. The bits are SBZP and this may be unpredictable."
                );
            }
            value = value.align(4);
        }
        let register_name = match register.0 {
            0..=12 => RegisterName::R(register.0),
            13 => RegisterName::SP(this.look_up_sp()),
            14 => RegisterName::LR,
            15 => RegisterName::PC,
            _ => unreachable!("RegisterID has range 0..=15"),
        };
        Self::set_register_based_on_name(component, register_name, value);
    }

    /// [ARM-ARM] B1.4.7 Pseudocode details of ARM core register accesses
    fn look_up_sp(&self) -> StackPointer {
        if self.control.stack_pointer_selector() == StackPointer::Process
            && self.xpsr.current_mode() == ExecutionMode::Handler
        {
            panic!(
                "Unpredictable behaviour: Cannot use process stack when in exception handler mode."
            )
        }
        self.control.stack_pointer_selector()
    }

    pub(super) fn with_pc(component: &mut SC::Component, value: Word) -> PCContext<SC> {
        let this = SC::component_to_member_mut(component);
        debug_assert!(this.pc.is_none());

        this.pc = Some(value);
        PCContext {
            #[cfg(debug_assertions)]
            released: Cell::new(false),

            phantom_sc: std::marker::PhantomData,
        }
    }

    pub(super) fn log_registers(
        component: &SC::Component,
        #[cfg(feature = "cycle-debug-logger")] ctx: &mut Context,
        pc: Word,
    ) {
        let this = SC::component_to_member(component);
        let sp = match this.look_up_sp() {
            StackPointer::Main => this.core_registers.sp_main,
            StackPointer::Process => this.core_registers.sp_process,
        };
        debug!("Registers: {}", this.registers_repr(sp, pc));

        #[cfg(feature = "cycle-debug-logger")]
        {
            let mut regs: [Word; 16] = [Word::from(0); 16];
            regs[0..=12].copy_from_slice(&this.core_registers.regs[..]);
            regs[13] = sp;
            regs[14] = this.core_registers.lr;
            regs[15] = pc;
            CycleDebugLoggerProxy::new().on_core_register_bank_tick(
                ctx,
                regs,
                *this.xpsr,
                *this.control,
                (this.core_registers.sp_main, this.core_registers.sp_process),
            );
        }
    }

    // `impl Display`, so the printing is done lazily only when needed.
    #[inline]
    fn registers_repr(&self, sp: Word, pc: Word) -> impl Display + '_ {
        struct RegsDisplay<'a>(&'a [Word; 13], Word, Word, Word);
        impl Display for RegsDisplay<'_> {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                write!(f, "[")?;
                for r in self.0 {
                    write!(f, "{r:x}, ")?;
                }
                write!(f, "{:x}, {:x}, {:x}]", self.1, self.2, self.3)
            }
        }
        RegsDisplay(&self.core_registers.regs, sp, self.core_registers.lr, pc)
    }

    fn set_register_based_on_name(
        component: &mut SC::Component,
        register_name: RegisterName,
        value: Word,
    ) {
        let this = SC::component_to_member_mut(component);
        // TODO(cm4): temporary hack, see a comment in the struct definition
        #[cfg(feature = "soc-cc2652")]
        if this.core_registers.is_mutator_set() {
            this.second_write_port_hack = Some((register_name, value));
            return;
        }
        this.core_registers
            .mutate_next((register_name, value), Self::_do_register_writing);
    }

    fn _do_register_writing(core_registers: &mut CoreRegisters, (name, v): (RegisterName, Word)) {
        match name {
            RegisterName::R(i) => core_registers.regs[i as usize] = v,
            RegisterName::SP(StackPointer::Main) => core_registers.sp_main = v,
            RegisterName::SP(StackPointer::Process) => core_registers.sp_process = v,
            RegisterName::LR => core_registers.lr = v,
            RegisterName::PC => unreachable!("PC cannot be changed by register assignment"),
        }
    }
}

impl<SC> PCContext<SC>
where
    SC: Subcomponent<Member = RegisterBank<SC>>,
{
    #[cfg_attr(not(debug_assertions), allow(clippy::unused_self))]
    pub(super) fn release(&self, component: &mut SC::Component) {
        let this = SC::component_to_member_mut(component);
        debug_assert!(this.pc.is_some());

        #[cfg(debug_assertions)]
        {
            assert!(!self.released.get());
            self.released.set(true);
        }

        this.pc = None;
    }
}

impl<SC> Drop for PCContext<SC>
where
    SC: Subcomponent<Member = RegisterBank<SC>>,
{
    fn drop(&mut self) {
        #[cfg(debug_assertions)]
        {
            assert!(self.released.get());
        }
    }
}

impl<SC> RegisterBank<SC>
where
    SC: Subcomponent<Member = Self>,
{
    pub(super) fn get_xpsr(component: &SC::Component) -> XPSR {
        let this = SC::component_to_member(component);
        *this.xpsr
    }

    pub(super) fn set_xpsr(component: &mut SC::Component, value: XPSR) {
        let this = SC::component_to_member_mut(component);
        this.xpsr.set_next(value);
    }
}

impl<SC> RegisterBank<SC>
where
    SC: Subcomponent<Member = Self>,
{
    pub(super) fn get_agu_result(component: &SC::Component) -> Word {
        let this = SC::component_to_member(component);
        *this.agu_result
    }

    pub(super) fn set_agu_result(component: &mut SC::Component, value: Word) {
        let this = SC::component_to_member_mut(component);
        this.agu_result.set_next(value);
    }
}

impl<SC> RegisterBank<SC>
where
    SC: Subcomponent<Component = CoreComponent, Member = Self>,
{
    pub(super) fn get_primask(core: &SC::Component) -> PriorityMaskRegister {
        let this = SC::component_to_member(core);
        *this.primask
    }

    /// Sets [`PriorityMaskRegister`] and sends new value to the [`NVICComponent`]
    /// which stores its read-only copy.
    ///
    /// [`NVICComponent`]: crate::component::nvic::NVICComponent
    pub(super) fn set_primask(
        core: &mut SC::Component,
        ctx: &mut Context,
        value: PriorityMaskRegister,
    ) {
        let this = SC::component_to_member_mut(core);
        this.primask.set_next(value);
        core.nvic.update_primask(ctx, value);
    }
}

impl<SC> RegisterBank<SC>
where
    SC: Subcomponent<Component = CoreComponent, Member = Self>,
{
    pub(super) fn get_faultmask(core: &SC::Component) -> FaultMaskRegister {
        let this = SC::component_to_member(core);
        *this.faultmask
    }

    /// Sets [`FaultMaskRegister`] and sends new value to the [`NVICComponent`]
    /// which stores its read-only copy.
    ///
    /// [`NVICComponent`]: crate::component::nvic::NVICComponent
    pub(super) fn set_faultmask(
        core: &mut SC::Component,
        ctx: &mut Context,
        value: FaultMaskRegister,
    ) {
        let this = SC::component_to_member_mut(core);
        this.faultmask.set_next(value);
        core.nvic.update_faultmask(ctx, value);
    }
}

impl<SC> RegisterBank<SC>
where
    SC: Subcomponent<Component = CoreComponent, Member = Self>,
{
    pub(super) fn get_basepri(core: &SC::Component) -> BasePriorityMaskRegister {
        let this = SC::component_to_member(core);
        *this.basepri
    }

    /// Sets [`BasePriorityMaskRegister`] and sends new value to the [`NVICComponent`]
    /// which stores its read-only copy.
    ///
    /// [`NVICComponent`]: crate::component::nvic::NVICComponent
    pub(super) fn set_basepri(
        core: &mut SC::Component,
        ctx: &mut Context,
        value: BasePriorityMaskRegister,
    ) {
        let this = SC::component_to_member_mut(core);
        this.basepri.set_next(value);
        core.nvic.update_basepri(ctx, value);
    }
}

impl<SC> RegisterBank<SC>
where
    SC: Subcomponent<Member = Self>,
{
    /// [ARM-ARM] B1.3.1
    pub(super) fn current_mode_is_privileged(component: &SC::Component) -> bool {
        let this = SC::component_to_member(component);
        this.xpsr.current_mode() == ExecutionMode::Handler || !this.control.unprivileged()
    }

    pub(super) fn get_control(component: &SC::Component) -> ControlRegister {
        let this = SC::component_to_member(component);
        *this.control
    }

    pub(super) fn set_control(component: &mut SC::Component, value: ControlRegister) {
        let this = SC::component_to_member_mut(component);
        this.control.set_next(value);
    }
}

impl<SC> RegisterBank<SC>
where
    SC: Subcomponent<Member = Self>,
{
    pub(super) fn get_stack_pointer(component: &SC::Component, sp: StackPointer) -> Word {
        let this = SC::component_to_member(component);
        match sp {
            StackPointer::Main => this.core_registers.sp_main,
            StackPointer::Process => this.core_registers.sp_process,
        }
    }

    pub(super) fn set_stack_pointer(component: &mut SC::Component, sp: StackPointer, value: Word) {
        Self::set_register_based_on_name(component, RegisterName::SP(sp), value);
    }
}

// ============================================================================
// [ARM-ARM] B1.4.7 Data types
// ============================================================================

/// B1.4.7 Register-related definitions for pseudocode/ `RName`
#[derive(Clone, Copy, Eq, PartialEq)]
enum RegisterName {
    R(u8),
    SP(StackPointer),
    LR,
    PC,
}

// ============================================================================
// Core CoreRegisters
// ============================================================================

/// [ARM-ARM] A2.3.1 ARM Core registers
///
/// PC register is stored in `RegisterBank::pc`
pub struct CoreRegisters {
    regs: [Word; 13],
    sp_main: Word,
    sp_process: Word,
    lr: Word,
}

/// B1.4.1 The ARM core registers
impl CoreRegisters {
    fn new() -> Self {
        Self {
            regs: [Word::from(0); 13],
            sp_main: Word::from(0),
            sp_process: Word::from(0),
            lr: Word::from(0),
        }
    }
}

// ============================================================================
// Core RegisterID
// ============================================================================

/// Core general purpose register.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RegisterID(u8);

impl RegisterID {
    pub const R0: Self = Self(0);
    pub const R1: Self = Self(1);
    pub const R2: Self = Self(2);
    pub const R3: Self = Self(3);
    pub const R4: Self = Self(4);
    pub const R5: Self = Self(5);
    pub const R6: Self = Self(6);
    pub const R7: Self = Self(7);
    pub const R8: Self = Self(8);
    pub const R9: Self = Self(9);
    pub const R10: Self = Self(10);
    pub const R11: Self = Self(11);
    pub const R12: Self = Self(12);
    pub const SP: Self = Self(13);
    pub const LR: Self = Self(14);
    pub const PC: Self = Self(15);

    /// Creates identifier representing general purpose register
    /// `r<index>`, e.g., `r0`.
    ///
    /// # Panics
    /// When `idx` is invalid.
    pub fn from_index<T>(idx: T) -> Self
    where
        T: TryInto<u8>,
        <T as TryInto<u8>>::Error: fmt::Debug,
    {
        let idx: u8 = idx
            .try_into()
            .expect("Register index should be in range 0..=15");
        assert!(idx < 16);
        Self(idx)
    }

    const fn as_array_index(self) -> usize {
        debug_assert!(self.0 <= 14);
        self.0 as usize
    }
}

impl Display for RegisterID {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.0 {
            0..=12 => write!(f, "r{}", self.0),
            13 => write!(f, "sp"),
            14 => write!(f, "lr"),
            15 => write!(f, "pc"),
            _ => unreachable!(),
        }
    }
}

// ============================================================================
// [ARM-ARM] B1.4.2  The special-purpose Program Status Registers, xPSR
// ============================================================================

// Used inside core (methods are pub(super)),
// but passed to CDL (so the type is pub(crate)).
bitfield! {
/// [ARM-ARM] B1.4.2
#[derive(Clone, Copy, PartialEq)]
pub(crate) struct XPSR[32]{
    N[31:31]: 1 bits,
    Z[30:30]: 1 bits,
    C[29:29]: 1 bits,
    V[28:28]: 1 bits,
    Q[27:27]: 1 bits,
    /// With DSP extension
    GE[19:16]: 4 bits,

    /// Thumb mode bit
    T[24:24]: 1 bits,

    ICI_IT_26[26:25]: 2 bits,
    ICI_IT_15[15:10]: 6 bits,

    /// [ARM-ARM] B1.5.7 Operation of 8-byte stack alignment.
    /// This bit is used to remember if stack align adjustment was required.
    /// In normal operation this bit is *reserved*.
    stack_realigned[9:9]: 1 bits,
    /// Valid in Handler mode, reads 0 in Thread
    exception_no[8:0]: 9 bits,
}}

impl XPSR {
    const APSR_MASK: Word = Word::from_const(0xF80F_0000);
    const IPSR_MASK: Word = Word::from_const(0x0000_01FF);
    const EPSR_MASK: Word = Word::from_const(0x0700_FC00);

    pub(super) fn new() -> Self {
        // TODO: the THUMB bit should be set during reset
        Self(0.into()).with_T_bit(true)
    }

    pub(super) fn negative_flag(self) -> bool {
        self.get_N_bit()
    }

    pub(super) fn with_negative(self, flag: bool) -> Self {
        self.with_N_bit(flag)
    }

    pub(super) fn zero_flag(self) -> bool {
        self.get_Z_bit()
    }

    pub(super) fn with_zero(self, flag: bool) -> Self {
        self.with_Z_bit(flag)
    }

    pub(super) fn carry_flag(self) -> bool {
        self.get_C_bit()
    }

    pub(super) fn with_carry(self, flag: bool) -> Self {
        self.with_C_bit(flag)
    }

    pub(super) fn overflow_flag(self) -> bool {
        self.get_V_bit()
    }

    pub(super) fn with_overflow(self, flag: bool) -> Self {
        self.with_V_bit(flag)
    }

    pub(super) fn saturation_flag(self) -> bool {
        self.get_Q_bit()
    }

    pub(super) fn with_saturation(self, flag: bool) -> Self {
        self.with_Q_bit(flag)
    }

    pub(super) fn get_exception_number(self) -> u32 {
        self.exception_no().into()
    }

    pub(super) fn with_exception_number(self, interrupt_id: u32) -> Self {
        let interrupt_id = Word::from(interrupt_id);
        self.with_exception_no(bitstring_extract!(interrupt_id<8:0> | 9 bits))
    }

    /// [ARM-ARM] B1.4.2 / The EPSR
    pub(super) fn instruction_set_state(self) -> InstructionSetState {
        if self.get_T_bit() {
            InstructionSetState::Thumb
        } else {
            InstructionSetState::ARM
        }
    }

    /// [ARM-ARM] B1.4.2 / The IPSR
    /// Note: the Thread mode is simply when no exception is executing (`exception_no=0`)
    pub(super) fn current_mode(self) -> ExecutionMode {
        if self.ipsr_as_word() == Word::from_const(0) {
            ExecutionMode::Thread
        } else {
            ExecutionMode::Handler
        }
    }

    pub(super) fn sp_align_was_required(self) -> bool {
        self.get_stack_realigned_bit()
    }

    pub(super) fn with_sp_align_was_required_bit(self, sp_align_was_required: bool) -> Self {
        self.with_stack_realigned_bit(sp_align_was_required)
    }

    pub(super) fn with_modified_apsr<F>(self, f: F) -> Self
    where
        F: FnOnce(XPSR) -> XPSR,
    {
        let new_xpsr = f(self);
        debug_assert!(
            Word::from(self) & !XPSR::APSR_MASK == Word::from(new_xpsr) & !XPSR::APSR_MASK
        );
        new_xpsr
    }

    #[allow(unused)]
    pub(super) fn with_modified_ipsr<F>(self, f: F) -> Self
    where
        F: FnOnce(XPSR) -> XPSR,
    {
        let new_xpsr = f(self);
        debug_assert!(
            Word::from(self) & !XPSR::IPSR_MASK == Word::from(new_xpsr) & !XPSR::IPSR_MASK
        );
        new_xpsr
    }

    pub(super) fn with_modified_epsr<F>(self, f: F) -> Self
    where
        F: FnOnce(XPSR) -> XPSR,
    {
        let new_xpsr = f(self);
        debug_assert!(
            Word::from(self) & !XPSR::EPSR_MASK == Word::from(new_xpsr) & !XPSR::EPSR_MASK
        );
        new_xpsr
    }

    pub(super) fn apsr_as_word(self) -> Word {
        Word::from(self.0) & Self::APSR_MASK
    }

    pub(super) fn ipsr_as_word(self) -> Word {
        Word::from(self.0) & Self::IPSR_MASK
    }

    pub(super) fn epsr_as_word(self) -> Word {
        Word::from(self.0) & Self::EPSR_MASK
    }

    // ------------------------------------------------------------------------
    // [ARM-ARM] B1.4.2, table B1-2 ICI/IT bit allocation in the EPSR
    // ------------------------------------------------------------------------

    /// [ARM-ARM] B1.4.2 -- itstate bits allocation
    /// [ARM-ARM] A7.3.3 -- actual check
    pub(super) fn in_it_block(self) -> bool {
        self.get_itstate().in_it_block()
    }

    /// [ARM-ARM] B1.4.2 -- itstate bits allocation
    /// [ARM-ARM] A7.3.3 -- actual check
    pub(super) fn last_in_it_block(self) -> bool {
        self.get_itstate().last_in_it_block()
    }

    /// [ARM-ARM] B1.4.2, table B1-2 ICI/IT bit allocation in the EPSR
    pub(super) fn get_itstate(self) -> ItState {
        let it_low = self.ICI_IT_26();
        let it_high = self.ICI_IT_15();
        // Note: These bits are overlaid with ICI state
        ItState(bitstring_concat!(it_high : it_low | 8 bits))
    }

    /// [ARM-ARM] B1.4.2, table B1-2 ICI/IT bit allocation in the EPSR
    pub(super) fn with_itstate(self, itstate: ItState) -> Self {
        let itstate: Bitstring![8] = itstate.0;
        debug_assert!(
            bitstring_extract!(itstate<3:0> | 4 bits) != bsc::C_0000 || itstate == bsc::C_0000_0000,
            "If setting inactive itstate, higher bits should be zeroed, too."
        );
        let it_low = bitstring_extract!(itstate<1:0> | 2 bits);
        let it_high = bitstring_extract!(itstate<7:2> | 6 bits);
        self.with_ICI_IT_26(it_low).with_ICI_IT_15(it_high)
    }

    /// [ARM-ARM] A7.3.3 `ITAdvance()`
    pub(super) fn with_it_advanced(self) -> Self {
        trace!("{}", "IT ADVANCING".bright_red());
        self.with_itstate(self.get_itstate().with_advance())
    }

    // [ARM-ARM] A7.3.1 `ConditionPassed()` (only for if-then instruction; extracts the condition)
    pub(super) fn it_condition_passed(self) -> bool {
        debug_assert!(self.in_it_block());
        self.get_itstate().get_current_condition().passed(self)
    }

    /// [ARM-ARM] B1.4.2, table B1-2 ICI/IT bit allocation in the EPSR
    pub(super) fn get_ici_reg_num(self) -> Option<Bitstring![4]> {
        // TODO: should always return some value?
        if self.in_it_block() {
            None
        } else {
            Some(bitstring_extract!((self.ICI_IT_15())<5:2> | 4 bits))
        }
    }
}

impl From<Word> for XPSR {
    fn from(val: Word) -> Self {
        // TODO: do we need some checks here?
        Self(val)
    }
}

impl From<XPSR> for Word {
    fn from(val: XPSR) -> Self {
        val.0
    }
}

impl Display for XPSR {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let _ici_reg = self.get_ici_reg_num();

        write!(
            f,
            "[ {n}{z}{c}{v}{q} | exc: {excno:0>3x} | it: {it_val} | ici: todo | {is_mode:^5} ]",
            n = if self.negative_flag() { "N" } else { "n" },
            z = if self.zero_flag() { "Z" } else { "z" },
            c = if self.carry_flag() { "C" } else { "c" },
            v = if self.overflow_flag() { "V" } else { "v" },
            q = if self.saturation_flag() { "Q" } else { "q" },
            excno = self.get_exception_number(),
            it_val = self.get_itstate(),
            is_mode = self.instruction_set_state(),
        )
    }
}

impl Default for XPSR {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// [ARM-ARM] B1.4.3  The special-purpose mask registers
// ============================================================================

// Used inside core and nvic, so that's the reason for pub(crate).
/// [ARM-ARM] B1.4.3
/// [TI-TRM] 2.5.2.18
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct PriorityMaskRegister(Bitstring![1]);

impl PriorityMaskRegister {
    // pub(crate) is required, because NVIC has to create PriorityMaskRegister.
    pub(crate) fn new() -> Self {
        Self(bsc::C_0)
    }

    // pub(crate) is required, because NVIC has to read value of `primask`.
    pub(crate) fn primask(self) -> bool {
        self.0.into()
    }

    pub(super) fn with_primask(self, v: bool) -> Self {
        Self(v.into())
    }

    pub(super) fn value_increases_priority(self, v: bool) -> bool {
        !self.primask() && v
    }
}

impl From<PriorityMaskRegister> for Word {
    fn from(reg: PriorityMaskRegister) -> Self {
        // [ARM-ARM] B1.4.3 defines it as a 1-bit register
        Word::from(reg.0)
    }
}

// Used inside core and nvic, so that's the reason for pub(crate).
/// [ARM-ARM] B1.4.3
/// [TI-TRM] 2.5.2.19
#[derive(Clone, Copy, Debug)]
pub(crate) struct FaultMaskRegister(Bitstring![1]);

impl FaultMaskRegister {
    // pub(crate) is required, because NVIC has to create FaultMaskRegister.
    pub(crate) fn new() -> Self {
        Self(bsc::C_0)
    }

    // pub(crate) is required, because NVIC has to read value of `faultmask`.
    pub(crate) fn faultmask(self) -> bool {
        self.0.into()
    }

    pub(super) fn with_faultmask(self, v: bool) -> Self {
        Self(v.into())
    }

    pub(super) fn value_increases_priority(self, v: bool) -> bool {
        !self.faultmask() && v
    }
}

impl From<FaultMaskRegister> for Word {
    fn from(reg: FaultMaskRegister) -> Self {
        // [ARM-ARM] B1.4.3 defines it as a 1-bit register
        Word::from(reg.0)
    }
}

// TODO: make this configurable as number of priority bits is IMPLEMENTATION DEFINED (3 here)
// Used inside core and nvic, so that's the reason for pub(crate).
// According to [ARM-ARM] `BasePriorityMaskRegister` is 8-bit register, but
// [TI-TRM] shows that only 3 of them are used to store value. That's the reason
// for the wrapped type.
/// [ARM-ARM] B1.4.3
/// [TI-TRM] 2.5.2.20
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct BasePriorityMaskRegister(Bitstring![3]);

impl BasePriorityMaskRegister {
    // pub(crate) is required, because NVIC has to create BasePriorityMaskRegister.
    pub(crate) fn new() -> Self {
        Self(bsc::C_000)
    }

    pub(super) fn basepri(self) -> Bitstring![8] {
        let val = self.0;
        // [TI-TRM] 2.5.2.20 - bits 7-5 are used to store value, but
        // [ARM-ARM] B1.4.3 defines this register as 8-bit.
        bitstring_concat!(val : bsc::C_0_0000 | 8 bits)
    }

    pub(super) fn with_basepri(val: Bitstring![8]) -> Self {
        // [TI-TRM] 2.5.2.20 - bits 7-5 are used to store value.
        let val = bitstring_extract!(val<7:5> | 3 bits);
        Self(val)
    }

    pub(super) fn value_increases_priority(self, v: Bitstring![8]) -> bool {
        (v != bsc::C_0000_0000) && (v < self.basepri() || self.basepri() == bsc::C_0000_0000)
    }
}

impl From<BasePriorityMaskRegister> for Word {
    fn from(reg: BasePriorityMaskRegister) -> Self {
        Word::from(reg.basepri())
    }
}

// ============================================================================
// [ARM-ARM] B1.4.4  The special-purpose CONTROL register
// ============================================================================

// Used inside core (methods are pub(super)),
// but passed to CDL (so the type is pub(crate)).
bitfield! {
/// [ARM-ARM] B1.4.4
#[derive(Clone, Copy, PartialEq)]
pub(crate) struct ControlRegister[3] {
        /// Execution privilege in Thread mode: HIGH=unpriv, Handler mode is always priv.
        nPRIV[0:0]: 1 bits,
        /// Stack to be used:
        /// - 0: Main stack pointer
        /// - 1: Process stack pointer (only in Thread mode)
        SPSEL[1:1]: 1 bits,
        /// Is FP extension active? (high=yes)
        FPCA[2:2]: 1 bits,
    }
}

impl ControlRegister {
    pub(super) fn new() -> Self {
        Self(bsc::C_000)
    }

    /// Gets `nPRIV` bit.
    pub(super) fn unprivileged(self) -> bool {
        self.get_nPRIV_bit()
    }

    /// Gets `SPSEL` bit.
    pub(super) fn stack_pointer_selector(self) -> StackPointer {
        self.get_SPSEL_bit()
            .ife(StackPointer::Process, StackPointer::Main)
    }

    /// Creates a new instance of `ControlRegister` with `nPRIV` bit changed.
    pub(super) fn with_unprivileged(self, v: bool) -> Self {
        self.with_nPRIV_bit(v)
    }

    /// Creates a new instance of `ControlRegister` with `SPSEL` bit changed.
    pub(super) fn with_stack_pointer_selector(self, v: StackPointer) -> Self {
        self.with_SPSEL_bit(v == StackPointer::Process)
    }
}

impl From<ControlRegister> for Word {
    fn from(reg: ControlRegister) -> Self {
        Word::from(reg.0)
    }
}

impl Display for ControlRegister {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "[ nPriv: {nPriv} | SPSEL: {SPSEL} ]",
            nPriv = self.unprivileged(),
            SPSEL = self.stack_pointer_selector(),
        )
    }
}

impl Default for ControlRegister {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// [ARM-ARM] A7.3.3 ITSTATE (in Conditional Execution)
// ============================================================================

// Used inside core (methods are pub(super)),
// but passed to CDL (so the type is pub(crate)).
bitfield! {
/// [ARM-ARM] A7.3.*
#[derive(Clone, Copy, PartialEq)]
pub(crate) struct ItState[8]{
        condition[7:4]: 4 bits,
        size_bits[3:0]: 4 bits,
    }
}

impl ItState {
    /// Pseudocode from [ARM-ARM] A7.3.3
    pub(super) fn with_advance(self) -> Self {
        let mut itstate = self.0;
        if bitstring_extract!(itstate<2:0> | 3 bits) == bsc::C_000 {
            itstate = bsc::C_0000_0000;
        } else {
            let new_itstate_4_0 = bitstring_extract!(itstate<4:0> | 5 bits).lsl(1);
            bitstring_substitute!(itstate<4:0> = new_itstate_4_0);
        }
        Self(itstate)
    }

    /// [ARM-ARM] A7.3.1
    fn get_current_condition(self) -> Condition {
        let itstate = self.expanded();
        let cond = if !itstate.size_bits.is_zero() {
            itstate.condition
        } else if self.0 == bsc::C_0000_0000 {
            bsc::C_1110
        } else {
            unreachable!("UNPREDICTABLE")
        };
        Condition(cond)
    }

    fn get_base_condition(self) -> Condition {
        // bits 4:0 actually indicate reflection of the condition
        Condition(self.get_current_condition().0.with_bit_set(0, false))
    }

    /// Pseudocode from [ARM-ARM] A7.3.3
    fn in_it_block(self) -> bool {
        self.size_bits() != bsc::C_0000
    }

    /// Pseudocode from [ARM-ARM] A7.3.3
    fn last_in_it_block(self) -> bool {
        self.size_bits() == bsc::C_1000
    }

    /// From table [ARM-ARM] Table A7-2
    pub(crate) fn get_remaining_instructions(self) -> u32 {
        let enc = u8::from(self.size_bits());
        4u32.saturating_sub(enc.trailing_zeros())
    }

    // Helper functions
    pub(crate) fn new_outside_it_block() -> Self {
        Self(bsc::C_0000_0000)
    }

    pub(crate) fn from_instruction(instr: &Instruction) -> Self {
        // [ARM-ARM] A7.7.38
        let Instruction::IfThen { firstcond, mask } = instr else {
            panic!("Expected IfThen instruction")
        };
        let mask = *mask;
        let firstcond = firstcond.0;
        let itstate = bitstring_concat!(firstcond : mask | 8 bits);
        Self(itstate)
    }
}

impl Display for ItState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if !self.in_it_block() {
            write!(f, "inactive")
        } else {
            write!(f, "{} i", self.get_base_condition())?;
            // From table [ARM-ARM] Table A7-2
            for i in 0..self.get_remaining_instructions() {
                write!(f, "{}", self.0.get_bit(4 - i).ife("e", "t"))?;
            }
            Ok(())
        }
    }
}

impl From<ItState> for Word {
    fn from(reg: ItState) -> Self {
        // [ARM-ARM] A7.3.3 continuous view of ITSTATE
        Word::from(reg.0)
    }
}

// ============================================================================
// RegisterBitmap
// ============================================================================

// Used inside core (methods are pub(super)),
// but passed to CDL as part of Instruction (so the type is pub(crate)).
/// Stores one bit per each general-purpose register.
/// Bit for register rN is stored as N-th bit of underlying integer.
///
/// The `RegisterBitmap` is compatible with `register_list`
/// used in `push` and `pop` (and some other) encodings in [ARM-ARM] A7.7.x.
#[derive(Clone, Copy, Debug)]
pub(crate) struct RegisterBitmap(u16);

impl RegisterBitmap {
    pub(super) const fn new() -> Self {
        Self(0)
    }

    pub(super) const fn singleton(reg: RegisterID) -> Self {
        Self::new().with(reg, true)
    }

    pub(super) const fn count(self) -> u32 {
        self.0.count_ones()
    }

    pub(super) const fn get(self, reg: RegisterID) -> bool {
        let aidx = if matches!(reg, RegisterID::PC) {
            15
        } else {
            reg.as_array_index()
        };
        self.0 & (1 << aidx) != 0
    }

    pub(super) const fn with(self, reg: RegisterID, val: bool) -> Self {
        let aidx = if matches!(reg, RegisterID::PC) {
            15
        } else {
            reg.as_array_index()
        };
        let mask = 1 << aidx;
        let v = if val { mask } else { 0 };
        Self((self.0 & !mask) | v)
    }

    pub(super) const fn are_all_cleared(self) -> bool {
        self.0 == 0
    }
}

impl std::ops::BitOr for RegisterBitmap {
    type Output = RegisterBitmap;

    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitAnd for RegisterBitmap {
    type Output = RegisterBitmap;

    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

impl std::ops::BitOr<RegisterID> for RegisterBitmap {
    type Output = RegisterBitmap;

    fn bitor(self, rhs: RegisterID) -> Self {
        self | Self::singleton(rhs)
    }
}

impl std::ops::Not for RegisterBitmap {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl From<Bitstring![16]> for RegisterBitmap {
    fn from(bs: Bitstring![16]) -> Self {
        Self(bs.into())
    }
}

impl From<RegisterID> for RegisterBitmap {
    fn from(value: RegisterID) -> Self {
        Self::singleton(value)
    }
}

impl Display for RegisterBitmap {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{{{}}}", self.into_iter().format(", "))
    }
}

#[derive(Clone, Debug)]
pub(crate) struct RegisterListIterator(Bitstring![16], u8);

impl IntoIterator for RegisterBitmap {
    type Item = RegisterID;
    type IntoIter = RegisterListIterator;

    fn into_iter(self) -> Self::IntoIter {
        RegisterListIterator(self.0.into(), 0)
    }
}

impl Iterator for RegisterListIterator {
    type Item = RegisterID;

    fn next(&mut self) -> Option<Self::Item> {
        for i in self.1..=15 {
            if self.0.get_bit(i.into()) {
                self.1 = i + 1;
                return Some(RegisterID::from_index(i));
            }
        }
        self.1 = 16;
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.0.bit_count() as usize;
        (0, Some(remaining))
    }
}
