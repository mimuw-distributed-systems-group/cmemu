#![allow(clippy::inline_always)]
use crate::arch::{ArmMProfileRegId, ArmMProfileRegs, Armv7m};
use crate::{DynResult, StopReason};
use cc2650_constants::operation::StackPointer;
use cc2650_constants::{BROM, FLASHMEM};
use cmemu_lib::common::{
    Address, CoreCoupledRegisterId, RegisterID, RequestedExit, SpecialPurposeRegisterId, Word,
};
use cmemu_lib::engine::{Emulator, EmulatorError};
use flexi_logger::{LogSpecBuilder, LoggerHandle};
use gdbstub::common::{Pid, Signal};
use gdbstub::target::ext::base::BaseOps;
use gdbstub::target::ext::base::reverse_exec::{ReverseContOps, ReverseStepOps};
use gdbstub::target::ext::base::single_register_access::{
    SingleRegisterAccess, SingleRegisterAccessOps,
};
use gdbstub::target::ext::base::singlethread::{
    SingleThreadBase, SingleThreadRangeStepping, SingleThreadRangeSteppingOps, SingleThreadResume,
    SingleThreadResumeOps, SingleThreadSingleStep, SingleThreadSingleStepOps,
};
use gdbstub::target::ext::breakpoints::{
    Breakpoints, BreakpointsOps, HwBreakpointOps, HwWatchpoint, HwWatchpointOps, SwBreakpoint,
    SwBreakpointOps, WatchKind,
};
use gdbstub::target::ext::exec_file::{ExecFile, ExecFileOps};
use gdbstub::target::ext::host_io::HostIoErrno::{EFAULT, ENOENT};
use gdbstub::target::ext::monitor_cmd::MonitorCmdOps;
use gdbstub::target::{Target, TargetError, TargetResult};
use gdbstub_arch::arm::ArmBreakpointKind;
use log::{trace, warn};
use std::any::Any;
use std::cmp::min;
use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::ops::Range;
use std::panic::AssertUnwindSafe;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::{array, mem};

mod monitor;
use monitor::MonitorOptions;
#[cfg(feature = "flash-test-lib")]
mod flash_test_lib;

// TODO: rethink whether we should take mut ref to the old emulator
pub(super) type EmulatorResetHook = dyn Fn(&DebugMonitor, &mut Emulator) -> DynResult<()>;

pub(crate) struct DebugMonitor {
    emu: Emulator,
    image_path: PathBuf,
    logger: LoggerHandle,
    log_spec_builder: Option<LogSpecBuilder>,
    // This is here as it is useful in general, but gated for now
    #[cfg(feature = "flash-test-lib")]
    symbols_file: Option<tempfile::NamedTempFile>,
    #[cfg(feature = "flash-test-lib")]
    flash_test: Option<flash_test_lib::FlashTestState>,

    // TODO: use this from CDL frame
    our_cycle_number: u64,
    options: MonitorOptions,
    exec_mode: ExecMode,

    // breaks
    cycle_timout: Option<u64>,
    // sorted
    cycle_breakpoints: VecDeque<u64>,
    breakpoints: HashSet<Address>,
    // breakpoints, but done by us
    traps: HashSet<Address>,
    watchpoints: Vec<(Range<Address>, WatchKind)>,
    // our management of Ctrl+C
    ctrl_c_flag: Arc<AtomicBool>,
    /// CMEmu panicked with the panic
    post_mortem: Option<Box<dyn Any + Send + 'static>>,
    /// Rerun (new emulator) hooks for monitor reset command.
    ///
    /// Returning `Result::Err` will cancel the process.
    rerun_hooks: Vec<Box<EmulatorResetHook>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum ExecMode {
    Halted,
    Continue,
    StepInstruction,
    StepCycle,
    RangeStep(Range<Address>),
}

// helper functions
impl DebugMonitor {
    fn cycle_number(&self) -> u64 {
        self.our_cycle_number
    }
}

impl DebugMonitor {
    pub(crate) fn new(
        emu: Emulator,
        cycle_timout: Option<u64>,
        image_path: PathBuf,
        logger: LoggerHandle,
    ) -> DebugMonitor {
        DebugMonitor {
            emu,
            image_path,
            logger,
            #[cfg(feature = "flash-test-lib")]
            symbols_file: None,
            #[cfg(feature = "flash-test-lib")]
            flash_test: None,
            log_spec_builder: None,
            our_cycle_number: 0,
            cycle_timout,
            options: MonitorOptions::default(),
            exec_mode: ExecMode::Halted,
            cycle_breakpoints: VecDeque::new(),
            breakpoints: HashSet::new(),
            traps: HashSet::new(),
            watchpoints: Vec::new(),
            ctrl_c_flag: Arc::new(AtomicBool::new(false)),
            post_mortem: None,
            rerun_hooks: Vec::new(),
        }
    }

    // Note: this method is here and not in monitor command handler as it is too tightly integrated.
    pub(crate) fn reinit_emulator(&mut self) -> DynResult<()> {
        warn!("Reinitializing emulator is very experimental");
        let mut flash_buf = vec![0u8; FLASHMEM::SIZE as usize];
        self.emu.read_memory(FLASHMEM::ADDR, &mut flash_buf)?;
        let mut rom_buf = vec![0u8; BROM::SIZE as usize];
        self.emu.read_memory(BROM::ADDR, &mut rom_buf)?;
        let rom_present = rom_buf.iter().any(|b| *b != 0);

        let mut new_emu = Emulator::new(&flash_buf[..], rom_present.then(|| &rom_buf[..]));
        for hook in &self.rerun_hooks {
            hook(self, &mut new_emu)?;
        }
        // Deconstruct old emulator;
        let mut old_emu = mem::replace(&mut self.emu, new_emu);
        self.emu
            .set_symbols_service(old_emu.set_symbols_service(None));
        // FIXME: uart lite / modem services
        // drop the stored panic if possible
        self.our_cycle_number = 0;
        self.post_mortem = None;
        Ok(())
    }

    pub(crate) fn install_ctrl_c_handler(&mut self) -> DynResult<()> {
        use signal_hook::consts::signal::SIGINT;
        use signal_hook::flag;

        // Kill if we didn't handle the signal
        // Hitting it while we're not running will kill us... so maybe mask the signal?
        // It also breaks the terminal from gdb mode.
        flag::register_conditional_shutdown(SIGINT, 2, self.ctrl_c_flag.clone())?;
        flag::register(SIGINT, self.ctrl_c_flag.clone())?;
        Ok(())
    }

    #[allow(clippy::unused_self)]
    pub(crate) fn install_ctrl_c_ignoring(&mut self) -> DynResult<()> {
        use signal_hook::consts::signal::SIGINT;
        use signal_hook::flag;

        flag::register(SIGINT, Arc::new(AtomicBool::new(false)))?;
        Ok(())
    }

    #[allow(clippy::too_many_lines, clippy::unnecessary_wraps)]
    pub(crate) fn step_check_event(&mut self) -> DynResult<Option<StopReason>> {
        // Handle ctrl-c event first to clear the flag
        if self.ctrl_c_flag.load(Ordering::Relaxed) {
            self.ctrl_c_flag.store(false, Ordering::Relaxed);
            return Ok(Some(StopReason::Signal(Signal::SIGINT)));
        } else if self.post_mortem.is_some() {
            return Ok(Some(StopReason::Terminated(Signal::SIGKILL)));
        }
        // return event only if we hit anything (including step or range step)
        if self.exec_mode == ExecMode::Halted {
            return Ok(Some(StopReason::DoneStep));
        }

        self.our_cycle_number += 1; // overflow in ages
        let step_error = std::panic::catch_unwind(AssertUnwindSafe(|| self.emu.step_cycle()));
        if let Err(panic) = step_error {
            match panic.downcast::<RequestedExit>() {
                // TODO: add a flag to catch exit for post-mortem
                Ok(code) => return Ok(Some(StopReason::Exited(code.code()))),
                Err(p) => {
                    // NOTE: we eat up the panic and allow investigation of the emulator.
                    // In a non-invasive mode, it should not result in more panics,
                    // but we cannot allow continuing the execution.
                    // (Unless someone implements a "zombie" mode....)
                    let styles = self.styles();
                    let bold = styles.get_header();
                    let red = styles.get_error();
                    let hint = styles.get_valid();
                    eprintln!();
                    eprintln!("{bold}Captured the panic for postmortem analysis{bold:#}",);
                    eprintln!("    with typeid: {:?}", (*p).type_id());
                    eprintln!("This is your {red}LAST CHANCE{red:#} to investigate.");
                    eprintln!("May I suggest connecting with a debugger to cmemu itself?");
                    eprintln!("Hint: get the PID with `{hint}monitor emu pid{hint:#}`.");
                    #[cfg(target_os = "linux")]
                    eprintln!("On Linux, you may need `{hint}monitor emu debug-me{hint:#}`.");
                    eprintln!(
                        "You can resume the unwind with `{hint}monitor emu resume-unwind{hint:#}`."
                    );
                    eprintln!();

                    self.exec_mode = ExecMode::Halted;
                    self.post_mortem = Some(p);
                    return Ok(Some(StopReason::Signal(Signal::SIGKILL)));
                }
            }
        }
        if self
            .cycle_timout
            .is_some_and(|timeout| timeout <= self.cycle_number())
        {
            self.cycle_timout = None;
            return Ok(Some(StopReason::Signal(Signal::SIGXCPU)));
        }

        // NOTE: the step/breakpoint logic is mixed, as we're inspecting the emulator
        // AFTER the first cycle was processed, but
        // BEFORE the flops were ticked to update their state, yet
        // some values (like curr_instr_addr) are not flopped!
        let instr_addr = self.emu.get_current_instruction_address();

        if self.exec_mode == ExecMode::StepCycle {
            return Ok(Some(StopReason::DoneStep));
        } else if self.emu.current_instruction_changed() {
            if self.exec_mode == ExecMode::StepInstruction {
                return Ok(Some(StopReason::DoneStep));
            } else if let ExecMode::RangeStep(ref addr_range) = self.exec_mode
                && !instr_addr.is_in_range(addr_range)
            {
                trace!("Stepped out of range {addr_range:#x?}");
                return Ok(Some(StopReason::DoneStep));
            }

            if self.breakpoints.contains(&instr_addr) {
                trace!("Breakpoint hit for {instr_addr:#x?}");
                return Ok(Some(StopReason::SwBreak(())));
            }
            if self.traps.contains(&instr_addr) {
                trace!("Trap hit for {instr_addr:#x?}");
                return Ok(Some(StopReason::Signal(Signal::SIGEMT)));
            }
        }

        // Make sure we didn't skip over a cycle
        while self
            .cycle_breakpoints
            .front()
            .is_some_and(|cyc| *cyc < self.cycle_number())
        {
            // VecDeque.pop_front_if is unstable
            self.cycle_breakpoints.pop_front();
        }
        if self
            .cycle_breakpoints
            .front()
            .is_some_and(|cyc| *cyc == self.cycle_number())
        {
            self.cycle_breakpoints.pop_front();
            return Ok(Some(StopReason::Signal(Signal::SIGTRAP)));
        }

        // Note: watchpoints should probably be under "current_instruction_changed",
        // but that would miss accesses by LDM etc.
        // However, GDB does a single step after a watchpoint - is it our fault?
        #[cfg(feature = "cdl")]
        if let Some(lsu_transfer) = self.emu.peek_core_lsu_request().as_ref() {
            use log::info;
            use std::borrow::Borrow;
            use std::cmp::max;

            let lsu_transfer = lsu_transfer.borrow();
            for (watched_range, kind) in &self.watchpoints {
                let addr_intersect = {
                    let touched_range = lsu_transfer.addr_range();
                    max(touched_range.start, watched_range.start)
                        ..min(touched_range.end, watched_range.end)
                };
                let kind_ok = match kind {
                    WatchKind::Write => lsu_transfer.is_writing(),
                    WatchKind::Read => lsu_transfer.is_reading(),
                    WatchKind::ReadWrite => true,
                };
                if !addr_intersect.is_empty() && kind_ok {
                    trace!("Watchpoint {watched_range:?} kind {kind:?} hit by {lsu_transfer:?}");
                    // Note GDB will ignore *write* watchpoint breaks if the value didn't change,
                    // OR it cannot read the memory!
                    if *kind == WatchKind::Write && self.options.write_watchpoint_is_trap {
                        info!("Write Watchpoint upgraded to a trap!");
                        return Ok(Some(StopReason::Signal(Signal::SIGTRAP)));
                    }
                    return Ok(Some(StopReason::Watch {
                        tid: (),
                        kind: *kind,
                        addr: addr_intersect.start.into(),
                    }));
                }
            }
        }

        // No event, call again
        Ok(None)
    }

    fn request_mode_change(
        &mut self,
        mut exec_mode: ExecMode,
        signal: Option<Signal>,
    ) -> DynResult<()> {
        if exec_mode == ExecMode::StepInstruction && self.options.step_cycle {
            exec_mode = ExecMode::StepCycle;
        }
        // This is weird this API is not allowed to return errors
        match signal {
            Some(Signal::SIGKILL) => {
                eprintln!("Cannot change step mode");
            }
            Some(Signal::SIGTRAP | Signal::SIGXCPU | Signal::SIGINT | Signal::SIGEMT) | None => {
                self.exec_mode = exec_mode;
            }
            Some(s) => Err(format!("Unexpected signal received {s}"))?,
        }
        Ok(())
    }
}

type EmuUsize = u32;

impl Target for DebugMonitor {
    type Arch = Armv7m;
    type Error = Box<dyn Error>;

    #[inline(always)]
    fn base_ops(&mut self) -> BaseOps<'_, Self::Arch, Self::Error> {
        BaseOps::SingleThread(self)
    }

    #[inline(always)]
    fn support_breakpoints(&mut self) -> Option<BreakpointsOps<'_, Self>> {
        Some(self)
    }

    #[inline(always)]
    fn support_monitor_cmd(&mut self) -> Option<MonitorCmdOps<'_, Self>> {
        Some(self)
    }

    #[inline(always)]
    fn support_exec_file(&mut self) -> Option<ExecFileOps<'_, Self>> {
        Some(self)
    }
}

impl SingleThreadBase for DebugMonitor {
    fn read_registers(&mut self, regs: &mut ArmMProfileRegs) -> TargetResult<(), Self> {
        trace!("Dumping all registers");
        regs.regs = array::from_fn(|i| self.emu.get_register(RegisterID::from_index(i)));
        regs.pc = self.emu.get_current_instruction_address().into();
        regs.xpsr = self
            .emu
            .get_extended_register(SpecialPurposeRegisterId::XPSR.into());
        Ok(())
    }

    fn write_registers(&mut self, regs: &ArmMProfileRegs) -> TargetResult<(), Self> {
        trace!("Writing registers: {regs:#x?}");
        eprintln!("Writing registers to a pipelined processor is not a great idea...");
        Err(TargetError::NonFatal)
    }

    #[inline(always)]
    fn support_single_register_access(&mut self) -> Option<SingleRegisterAccessOps<'_, (), Self>> {
        Some(self)
    }

    fn read_addrs(&mut self, start_addr: EmuUsize, data: &mut [u8]) -> TargetResult<usize, Self> {
        trace!("Reading memory at {start_addr:#x}/{}b", data.len());
        // FIXME this is a workaround, because we start with pc = 0, and upon connecting gdb tries to read from
        // pc - 4 and pc - 2
        if start_addr >= 0xfffffffc {
            return Err(TargetError::Errno(EFAULT as u8));
        }
        match self.emu.read_memory(start_addr.into(), data) {
            Ok(()) => Ok(data.len()),
            Err(EmulatorError::InvalidAddress) => Err(TargetError::Errno(EFAULT as u8)),
            Err(e) => Err(TargetError::Fatal(e.into())),
        }
    }

    fn write_addrs(&mut self, start_addr: EmuUsize, data: &[u8]) -> TargetResult<(), Self> {
        trace!("Writing memory at {start_addr:#x}/{}b", data.len());
        match self.emu.write_memory(start_addr.into(), data) {
            Ok(()) => Ok(()),
            Err(EmulatorError::InvalidAddress) => Err(TargetError::Errno(EFAULT as u8)),
            Err(e) => Err(TargetError::Fatal(e.into())),
        }
    }

    #[inline(always)]
    fn support_resume(&mut self) -> Option<SingleThreadResumeOps<'_, Self>> {
        Some(self)
    }
}

fn gdb_reg_to_cmemu_reg(reg: ArmMProfileRegId) -> CoreCoupledRegisterId {
    match reg {
        ArmMProfileRegId::Gpr(i) => CoreCoupledRegisterId::Core(RegisterID::from_index(i)),
        ArmMProfileRegId::Sp => CoreCoupledRegisterId::Core(RegisterID::SP),
        ArmMProfileRegId::Lr => CoreCoupledRegisterId::Core(RegisterID::LR),
        ArmMProfileRegId::Pc => CoreCoupledRegisterId::Core(RegisterID::PC),
        ArmMProfileRegId::Xpsr => SpecialPurposeRegisterId::XPSR.into(),
        ArmMProfileRegId::Msp => StackPointer::Main.into(),
        ArmMProfileRegId::Psp => StackPointer::Process.into(),
        ArmMProfileRegId::Itstate => SpecialPurposeRegisterId::Itstate.into(),
        ArmMProfileRegId::Primask => SpecialPurposeRegisterId::Primask.into(),
        ArmMProfileRegId::Basepri => SpecialPurposeRegisterId::Basepri.into(),
        ArmMProfileRegId::Faultmask => SpecialPurposeRegisterId::Faultmask.into(),
        ArmMProfileRegId::Control => SpecialPurposeRegisterId::Control.into(),
    }
}

impl SingleRegisterAccess<()> for DebugMonitor {
    fn read_register(
        &mut self,
        _tid: (),
        reg_id: ArmMProfileRegId,
        buf: &mut [u8],
    ) -> TargetResult<usize, Self> {
        trace!("Reading register {reg_id:?}");
        let word: Word = match reg_id {
            ArmMProfileRegId::Pc => self.emu.get_current_instruction_address().into(),
            _ => self.emu.get_extended_register(gdb_reg_to_cmemu_reg(reg_id)),
        };
        buf.copy_from_slice(word.to_le_bytes()[..buf.len()].as_ref());
        Ok(buf.len())
    }

    fn write_register(
        &mut self,
        _tid: (),
        reg_id: ArmMProfileRegId,
        val: &[u8],
    ) -> TargetResult<(), Self> {
        trace!("Writing register: {reg_id:?} {val:#x?}");
        eprintln!("Writing registers to a pipelined processor is not a great idea...");
        Err(TargetError::NonFatal)
    }
}

impl SingleThreadResume for DebugMonitor {
    fn resume(&mut self, signal: Option<Signal>) -> Result<(), Self::Error> {
        trace!("Resuming {signal:?}");
        self.request_mode_change(ExecMode::Continue, signal)
    }

    #[inline(always)]
    fn support_single_step(&mut self) -> Option<SingleThreadSingleStepOps<'_, Self>> {
        Some(self)
    }

    #[inline(always)]
    fn support_range_step(&mut self) -> Option<SingleThreadRangeSteppingOps<'_, Self>> {
        Some(self)
    }

    #[inline(always)]
    fn support_reverse_step(&mut self) -> Option<ReverseStepOps<'_, (), Self>> {
        // TODO: walk cdl frames
        None
    }

    #[inline(always)]
    fn support_reverse_cont(&mut self) -> Option<ReverseContOps<'_, (), Self>> {
        None
    }
}

impl SingleThreadSingleStep for DebugMonitor {
    fn step(&mut self, signal: Option<Signal>) -> Result<(), Self::Error> {
        trace!("Step {signal:#x?}");
        self.request_mode_change(ExecMode::StepInstruction, signal)
    }
}

impl SingleThreadRangeStepping for DebugMonitor {
    fn resume_range_step(&mut self, start: EmuUsize, end: EmuUsize) -> Result<(), Self::Error> {
        trace!("Step range {start:#x?}..{end:#x?}");
        self.request_mode_change(ExecMode::RangeStep(start.into()..end.into()), None)
    }
}

impl Breakpoints for DebugMonitor {
    #[inline(always)]
    fn support_sw_breakpoint(&mut self) -> Option<SwBreakpointOps<'_, Self>> {
        Some(self)
    }

    #[inline(always)]
    fn support_hw_breakpoint(&mut self) -> Option<HwBreakpointOps<'_, Self>> {
        // There is no need for this with emulators
        None
    }

    #[inline(always)]
    fn support_hw_watchpoint(&mut self) -> Option<HwWatchpointOps<'_, Self>> {
        cfg!(feature = "cdl").then_some(self)
    }
}

impl SwBreakpoint for DebugMonitor {
    fn add_sw_breakpoint(
        &mut self,
        addr: EmuUsize,
        kind: ArmBreakpointKind,
    ) -> TargetResult<bool, Self> {
        trace!("Breakpoint on {addr:#x?} {kind:?}");
        Ok(self.breakpoints.insert(addr.into()))
    }

    fn remove_sw_breakpoint(
        &mut self,
        addr: EmuUsize,
        kind: ArmBreakpointKind,
    ) -> TargetResult<bool, Self> {
        trace!("Removing a breakpoint on {addr:#x?} {kind:?}");
        Ok(self.breakpoints.remove(&Address::from(addr)))
    }
}

impl HwWatchpoint for DebugMonitor {
    fn add_hw_watchpoint(
        &mut self,
        addr: EmuUsize,
        len: EmuUsize,
        kind: WatchKind,
    ) -> TargetResult<bool, Self> {
        trace!("Watching memory at {addr:#x}/{len}b for {kind:?}");
        self.watchpoints
            .push((Address::range_from_len(addr, len), kind));
        Ok(true)
    }

    fn remove_hw_watchpoint(
        &mut self,
        addr: EmuUsize,
        len: EmuUsize,
        kind: WatchKind,
    ) -> TargetResult<bool, Self> {
        trace!("Removing watchpoint at {addr:#x}/{len}b for {kind:?}");
        match self
            .watchpoints
            .iter()
            .position(|v| *v == (Address::range_from_len(addr, len), kind))
        {
            None => Ok(false),
            Some(p) => {
                self.watchpoints.swap_remove(p);
                Ok(true)
            }
        }
    }
}

impl ExecFile for DebugMonitor {
    #[allow(clippy::cast_possible_truncation)]
    fn get_exec_file(
        &self,
        pid: Option<Pid>,
        offset: u64,
        length: usize,
        buf: &mut [u8],
    ) -> TargetResult<usize, Self> {
        // TODO: we should be asked for no PID, but GDB asks for PID 1, cause we returned something.
        trace!("Returning exec filename for {pid:?}  at {offset:#x}");
        if pid.is_some_and(|pid| pid.get() != 1) {
            Err(TargetError::Errno(ENOENT as u8))
        } else {
            #[cfg(feature = "flash-test-lib")]
            if let Some(ref name) = self.symbols_file {
                return Ok(copy_to_buffer(
                    name.path().as_os_str().as_encoded_bytes(),
                    offset as usize,
                    length,
                    buf,
                ));
            }

            Ok(copy_to_buffer(
                self.image_path.as_os_str().as_encoded_bytes(),
                offset as usize,
                length,
                buf,
            ))
        }
    }
}

pub(crate) fn copy_to_buffer(
    src: impl AsRef<[u8]>,
    off: usize,
    len: usize,
    out: &mut [u8],
) -> usize {
    let src = src.as_ref();
    let src_len = src.len();
    if src_len <= off {
        return 0;
    }
    let to_copy = min(min(len, out.len()), src_len - off);
    out[..to_copy].copy_from_slice(&src[off..off + to_copy]);
    to_copy
}
