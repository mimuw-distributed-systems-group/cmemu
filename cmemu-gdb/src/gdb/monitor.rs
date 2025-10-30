use crate::DynResult;
use crate::gdb::DebugMonitor;
use clap::error::ErrorKind;
use clap::{Args, CommandFactory, Parser, Subcommand};
use cmemu_lib::common::{Address, CoreCoupledRegisterId};
use cmemu_lib::engine::Timepoint;
use flexi_logger::{LogSpecBuilder, LogSpecification};
use gdbstub::target::Target;
use gdbstub::target::ext::monitor_cmd::ConsoleOutput;
use gdbstub::target::ext::monitor_cmd::{MonitorCmd, outputln};
use std::borrow::Borrow;
use std::error::Error;
use std::path::PathBuf;
use std::time::Duration;
use std::{fs, io, str};

#[derive(Parser, Debug)]
// No "multicall" as we are always called with "monitor", but it is just missing
#[command(no_binary_name = true, arg_required_else_help = true)]
#[command(name = "monitor", bin_name = "monitor", display_name = "monitor")]
// Don't use our context for extracting this info!
#[command(color = clap::ColorChoice::Always)]
#[command(term_width = 0, max_term_width = 120)]
#[command(verbatim_doc_comment)]
#[command(infer_long_args = true, infer_subcommands = true)]
// #[command(flatten_help = true)]
/// CMEmu debug monitor interface
///
/// Keep in mind we can debug the application in two modes:
///
/// - exact - the use of the debugger doesn't break cycle-exactness, but is limited,
/// - intrusive (TODO) - this acts like the debugging port (DAP) on the real device
struct Monitor {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(subcommand, alias = "sim", visible_alias = "g")]
    Guest(Guest),
    #[command(alias = "c")]
    /// Get current cycle number
    CycleTime,
    #[command(visible_alias = "sc")]
    /// Make step-instruction (si) perform single-cycle steps instead
    StepCycle {
        #[command(subcommand)]
        off: Option<Off>,
    },
    #[command(visible_alias = "bc")]
    /// Add a breakpoint on a cycle number
    BreakCycle { cycle: u64 },

    /// Get AGU value
    Agu,
    #[cfg(feature = "cdl")]
    /// Dump information about LSU request
    LsuRequest,

    #[cfg(feature = "cdl")]
    #[command(visible_alias = "watch-trap", visible_alias = "wwt")]
    /// Make write watchpoints always trigger a break
    ///
    /// This is done by unconditionally replacing them with a "trap" stop reason,
    /// so GDB won't display it as a watchpoint hit.
    WriteWatchpointTrap {
        #[command(subcommand)]
        off: Option<Off>,
    },
    #[cfg(feature = "flash-test-lib")]
    #[command(subcommand)]
    FlashTest(super::flash_test_lib::FlashTestCmd),
    #[cfg(feature = "cdl")]
    #[command(subcommand)]
    Cdl(Cdl),
    #[command(subcommand)]
    Log(Log),
    #[command(subcommand, aliases = ["emu", "host"])]
    Cmemu(Emu),
    #[command(subcommand)]
    Test(Test),
    /// Get information about supported features etc.
    Status,
}

//  Adding Option<Off> makes a generic positional `off` "subcommand"
#[derive(Subcommand, Debug)]
enum Off {
    #[command(short_flag = 'd', long_flag = "disable")]
    /// Turn off
    Off,
}

#[cfg(feature = "cdl")]
/// Manage the Cycle Debug Logger
#[derive(Subcommand, Debug)]
enum Cdl {
    /// Start recording
    Start,
    /// Stop recording
    Stop,
    /// Dump now
    Dump,
    /// Set the file to dump
    SetFile { file: PathBuf },
    /// Unset the file to dump
    UnsetFile,
}

/// Commands related to the simulation (guest)
#[derive(Subcommand, Debug)]
enum Guest {
    /// Print the path to executed flash/elf file
    #[allow(clippy::enum_variant_names)]
    GuestExe,
    /// Print a nice name for the given address
    NameAddress {
        #[arg(value_parser = parse_address)]
        addr: Address,
    },
    #[command(alias = "time")]
    /// Get virtual time of the emulator
    VirtualTime,
    // note: this is copied for consistency
    /// Get current cycle number
    CycleTime,
    /// Modify execution timeout
    ///
    /// Pass 0 to disable execution timeout.
    Timeout { cycle: Option<u64> },
    /// Dump the energy state of components, gates, domains, etc.
    // TODO: add filters
    EnergyState,
}

fn parse_address(s: &str) -> Result<Address, String> {
    let val: u32 = if let Some(stripped) = s.strip_prefix("0x") {
        u32::from_str_radix(stripped, 16)
    } else {
        s.parse::<u32>()
    }
    .map_err(|e| e.to_string())?;
    Ok(Address::from_const(val))
}

/// Commands related to the emulator (host)
#[derive(Subcommand, Debug)]
enum Emu {
    /// Get PID of the cmemu (host) process
    Pid,
    /// Print emulators current working directory (used for relative paths)
    Pwd,
    /// Print the path to cmemu executable
    #[command(visible_alias = "path")]
    EmulatorExe,
    /// If we're in a post-mortem phase, resume unwinding from here
    ResumeUnwind,
    /// Attempt to recreate the emulator. Some initialization may be lost
    ///
    /// Especially if done before giving emulator to cmemu-gdb.
    /// Not suitable for hosted apps (only Flash & ROM are copied).
    /// Clears `panic` state (may cause erroneous behavior if panic was not in the emulator).
    #[command(visible_alias = "reset_halt")]
    AttemptReset,
    #[cfg(target_os = "linux")]
    #[command(verbatim_doc_comment)]
    /// Allow an external debugger to connect to CMEmu by PID.
    ///
    /// On Linux with YAMA `ptrace_scope=1` (default for modern distros),
    /// only an ancestor process can debug another one.
    /// While most docs will tell you to disable the protection globally,
    /// a process may explicitly allow another one to debug it with `PR_SET_PTRACER`.
    ///
    /// If `PID` is not specified, any process of the current user will be allowed to connect.
    ///
    /// See: <https://www.kernel.org/doc/Documentation/security/Yama.txt>
    DebugMe {
        /// Allow debugging only by this PID.
        pid: Option<u32>,
    },
}

/// Various testing functionality
#[derive(Subcommand, Debug)]
enum Test {
    /// Ask for a normal output
    Ping,
    /// Panic on demand
    Panic,
    /// Simulate a monitor fatal error that should terminate the gdb server
    FatalError,
    /// Simulate a custom command usage error
    UsageError,
    /// Simulate a monitor error (non fatal for debugging)
    RuntimeError,
    /// Simulate another monitor error
    RuntimeDynError,
}

#[derive(Args, Debug, Default)]
pub(super) struct MonitorOptions {
    pub step_cycle: bool,
    pub write_watchpoint_is_trap: bool,
}

#[derive(Debug)]
pub(super) enum MonitorError {
    Clap(clap::Error),
    Runtime(String),
    RuntimeDyn(Box<dyn Error>),
    Fatal(<DebugMonitor as Target>::Error),
}

#[cfg(test)]
mod test_clap {
    use super::*;

    #[test]
    fn monitor_clap_asserts() {
        Monitor::command().debug_assert();
    }
}

#[derive(Subcommand, Debug)]
#[command(verbatim_doc_comment)]
/// Manage and dynamically configure the logger
///
/// Push and pop commands operate on `flexi_logger`'s stack of log specification.
/// Pushing new specification also creates a builder, which is the target of the remaining commands.
/// Popping will invalidate the builder, as we don't duplicate the stack.
enum Log {
    /// Push new configuration on the stack
    Push {
        #[command(subcommand)]
        s: LogSpec,
    },
    /// Pop the configuration from the stack
    Pop,
    /// Add rules to the current spec
    Add {
        #[command(subcommand)]
        s: LogSpec,
    },
    /// Push the current builder
    Dup,
    /// Dump the current builder
    Show,
    /// Show the current builder as spec string
    ShowSpec,
    /// Show the current builder as toml
    ShowToml,
}

#[derive(Subcommand, Debug)]
enum LogSpec {
    #[command(verbatim_doc_comment)]
    /// Parse logging setup from a .toml file.
    ///
    /// The file path is relative to `monitor cmemu pwd`.
    /// For example files, see `cmemu-framework/log-presets`.
    File { file: PathBuf },

    #[command(verbatim_doc_comment)]
    /// Parse logging spec like in `RUST_LOG` env.
    ///
    /// The spec is the same as in `env_logger`.
    /// See [`flexi_logger::LogSpecification`] for details.
    Spec { spec: String },

    /// Parse the `RUST_LOG` env var.
    Env,
    /// An empty spec.
    Clear,
}

impl LogSpec {
    fn into_spec(self) -> DynResult<LogSpecification> {
        Ok(match self {
            LogSpec::File { file } => {
                let body = fs::read_to_string(file)?;
                LogSpecification::from_toml(body)?
            }
            LogSpec::Spec { spec } => LogSpecification::parse(spec)?,
            LogSpec::Env => LogSpecification::env()?,
            LogSpec::Clear => LogSpecification::off(),
        })
    }
}

// This should be implemented on the actual stub first!
struct IoConsoleOutput<'a, 'b>(&'a mut ConsoleOutput<'b>);
impl io::Write for IoConsoleOutput<'_, '_> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.write_raw(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.0.flush();
        Ok(())
    }
}

impl DebugMonitor {
    fn display_cmemu_status(&self, out: &mut ConsoleOutput<'_>) {
        let cmemu_lib_cdl = cmemu_lib::BUILT_WITH_CYCLE_DEBUG_LOGGER;
        let our_cdl = cfg!(feature = "cdl");
        outputln!(out, "cmemu-lib built with CDL: {:?}", cmemu_lib_cdl);
        outputln!(out, "cmemu-gdb built with CDL: {:?}", our_cdl);
        outputln!(out, "watchpoints supported: {:?}", cmemu_lib_cdl && our_cdl);
        if self.post_mortem.is_some() {
            outputln!(out, "You are in a post-mortem state with a captured panic!");
        }
        outputln!(out, "exec mode: {:?}", self.exec_mode);
        out.flush();
        #[cfg(feature = "flash-test-lib")]
        if let Some(ref syms) = self.symbols_file {
            outputln!(
                out,
                "Has replacement symbols file: {:}",
                syms.path().display()
            );
            out.flush();
        }
        outputln!(out);

        outputln!(out, "{:#?}", self.options);
        outputln!(out);
        out.flush();

        #[cfg(feature = "flash-test-lib")]
        self.flash_test_status(out);

        // we cannot list watchpoints and breakpoints as GDB keeps removing them...
        outputln!(out, "Note: GDB typically removes all breaks during a halt.");
        outputln!(out, "Breakpoints: {:#?}", self.breakpoints);
        outputln!(out, "Watchpoints: {:#?}", self.watchpoints);
        out.flush();
        outputln!(out, "Cycle breakpoints: {:#?}", self.cycle_breakpoints);
        out.flush();
        outputln!(out, "Extra traps: {:#?}", self.traps);
        outputln!(out);
        out.flush();

        outputln!(out, "Cycle: {:#}", self.cycle_number());
        if let Some(timeout) = self.cycle_timout {
            outputln!(out, "Timeout cycles: {:#}", timeout);
        }
        outputln!(out, "Virtual time: ~{:#?}", self.std_emulation_time());
        out.flush();
        // add more...
    }

    fn add_breakpoint_on_cycle(
        &mut self,
        cycle: u64,
        _out: &mut ConsoleOutput<'_>,
    ) -> Result<(), MonitorError> {
        if self.cycle_number() >= cycle {
            Err(MonitorError::Runtime(format!(
                "Given cycle {cycle} is earlier than current {}",
                self.cycle_number()
            )))
        } else {
            self.cycle_breakpoints.insert(
                self.cycle_breakpoints.partition_point(|&c| c < cycle),
                cycle,
            );
            Ok(())
        }
    }

    fn set_timeout(
        &mut self,
        cycle: Option<u64>,
        out: &mut ConsoleOutput<'_>,
    ) -> Result<(), MonitorError> {
        outputln!(out, "Current timeout: {:?}", self.cycle_timout);
        match cycle {
            None => outputln!(out, "(pass a value to change)"),
            Some(0) => self.cycle_timout = None,
            Some(cycle) if cycle <= self.cycle_number() => Err(MonitorError::Runtime(format!(
                "Given cycle {cycle} is earlier than current {}",
                self.cycle_number()
            )))?,
            Some(_) => self.cycle_timout = cycle,
        }
        Ok(())
    }

    #[cfg(feature = "cdl")]
    fn cdl_command(&mut self, cmd: Cdl, out: &mut ConsoleOutput<'_>) -> Result<(), MonitorError> {
        match cmd {
            Cdl::Start => self.emu.cycle_debug_logger_start_recording(),
            Cdl::Stop => self.emu.cycle_debug_logger_stop_recording(),
            Cdl::Dump => match self.emu.cycle_debug_logger_dump_now() {
                Ok(None) => outputln!(out, "No file set or nothing to dump."),
                Ok(Some(path)) => outputln!(out, "Dumped to {}", path.borrow().display()),
                Err(e) => Err(MonitorError::RuntimeDyn(e))?,
            },
            Cdl::SetFile { file } => self.emu.set_cycle_debug_logger_file(Some(&file)),
            Cdl::UnsetFile => self.emu.set_cycle_debug_logger_file(None::<&PathBuf>),
        }
        Ok(())
    }

    fn log_command(&mut self, cmd: Log, out: &mut ConsoleOutput<'_>) -> Result<(), MonitorError> {
        self.logger.flush();
        // This one trick allows for wrapping the error without the try! blocks
        // The closure is a trick to unify Errors before wrapping them
        (|| {
            match cmd {
                Log::Pop => {
                    self.logger.pop_temp_spec();
                    // Clear the builder as we're not storing the stack copy
                    self.log_spec_builder = None;
                }
                Log::Push { s } => {
                    let spec = s.into_spec()?;
                    let builder = LogSpecBuilder::from_module_filters(spec.module_filters());
                    self.logger.push_temp_spec(builder.build());
                    self.log_spec_builder = Some(builder);
                }
                Log::Dup => self.logger.push_temp_spec(
                    self.log_spec_builder
                        .as_ref()
                        .ok_or("Cannot duplicate the default config.")?
                        .build(),
                ),
                Log::Add { s } => self.logger.set_new_spec(
                    self.log_spec_builder
                        .as_mut()
                        .ok_or("Cannot extend the default config.")?
                        .insert_modules_from(s.into_spec()?)
                        .build(),
                ),
                Log::Show => {
                    outputln!(out, "{:?}", self.log_spec_builder);
                }
                Log::ShowSpec => {
                    if let Some(ref builder) = self.log_spec_builder {
                        outputln!(out, "{}", builder.build());
                    } else {
                        Err("Nothing to show")?;
                    }
                }
                Log::ShowToml => {
                    if let Some(ref builder) = self.log_spec_builder {
                        builder.build().to_toml(&mut IoConsoleOutput(out))?;
                    } else {
                        Err("Nothing to show")?;
                    }
                }
            }
            Ok(())
        })()
        .map_err(|e: Box<dyn Error>| MonitorError::RuntimeDyn(e))
    }

    fn cmemu_command(&mut self, cmd: Emu, out: &mut ConsoleOutput<'_>) -> Result<(), MonitorError> {
        self.logger.flush();
        let styles = self.styles();
        // This one trick allows for wrapping the error without the try! blocks
        // The closure is a trick to unify Errors before wrapping them
        (move || {
            match cmd {
                Emu::Pwd => {
                    outputln!(
                        out,
                        "{}",
                        std::env::current_dir()?.display()
                    );
                }
                Emu::EmulatorExe => {
                    outputln!(
                        out,
                        "{}",
                        std::env::current_exe()?.display()
                    );
                }
                Emu::Pid => {
                    outputln!(out, "{}", std::process::id());
                }
                Emu::ResumeUnwind if self.post_mortem.is_none() => {
                    Err("No panic to unwind")?;
                }
                Emu::ResumeUnwind => {
                    std::panic::resume_unwind(self.post_mortem.take().unwrap());
                }
                Emu::AttemptReset => {
                    if let Err(err) = self.reinit_emulator() {
                        let warn = styles.get_invalid();
                        outputln!(
                            out,
                            "{warn}Attempt canceled - cannot replace the emulator:{warn:#} {err}"
                        );
                    } else {
                        let warn = styles.get_valid();
                        outputln!(
                            out,
                            "Emulator reinitialized!\n\
                            {warn}Be careful{warn:#}, it is not a duplicate and stuff may be not configured or loaded!"
                        );
                    }
                }
                #[cfg(target_os = "linux")]
                Emu::DebugMe { pid } => {
                    allow_debuggers(pid)?;
                    outputln!(out, "Attach to: {}", std::process::id());
                    outputln!(
                        out,
                        "NOTE: don't use the debugger of the guest while halted, as it will time out!"
                    );
                }
            }
            Ok(())
        })()
        .map_err(|e: Box<dyn Error>| MonitorError::RuntimeDyn(e))
    }

    fn monitor_main(
        &mut self,
        cmd: Commands,
        out: &mut ConsoleOutput<'_>,
    ) -> Result<(), MonitorError> {
        // note: there is no auto flushing?
        match cmd {
            Commands::Guest(Guest::GuestExe) => {
                outputln!(out, "{}", self.image_path.display());
            }
            Commands::Guest(Guest::NameAddress { addr }) => {
                outputln!(out, "{}", self.emu.name_an_address(addr));
            }
            Commands::Guest(Guest::VirtualTime) => {
                outputln!(out, "{:?}", self.emu.get_emulation_time());
                outputln!(out, "Roughly {:?}", self.std_emulation_time());
            }
            Commands::CycleTime | Commands::Guest(Guest::CycleTime) => {
                outputln!(out, "{}", self.cycle_number());
            }
            Commands::Guest(Guest::Timeout { cycle }) => {
                self.set_timeout(cycle, out)?;
            }
            Commands::Guest(Guest::EnergyState) => {
                for (node, state) in self.emu.unstable_get_components_energy_state() {
                    outputln!(out, "{}: {:?}", node.borrow(), state.borrow());
                    out.flush();
                }
            }
            Commands::BreakCycle { cycle } => {
                self.add_breakpoint_on_cycle(cycle, out)?;
            }
            Commands::Agu => {
                outputln!(
                    out,
                    "{:#x}",
                    self.emu
                        .get_extended_register(CoreCoupledRegisterId::AGUResult)
                );
            }
            #[cfg(feature = "cdl")]
            Commands::LsuRequest => outputln!(
                out,
                "{:#x?}",
                self.emu
                    .peek_core_lsu_request()
                    .as_ref()
                    .map(Borrow::borrow)
            ),
            Commands::StepCycle { off } => self.options.step_cycle = off.is_none(),
            #[cfg(feature = "cdl")]
            Commands::WriteWatchpointTrap { off } => {
                self.options.write_watchpoint_is_trap = off.is_none();
            }
            Commands::Status => self.display_cmemu_status(out),
            #[cfg(feature = "flash-test-lib")]
            Commands::FlashTest(cmd) => self.flash_test_command(cmd, out)?,
            #[cfg(feature = "cdl")]
            Commands::Cdl(cmd) => self.cdl_command(cmd, out)?,
            Commands::Log(cmd) => self.log_command(cmd, out)?,
            Commands::Cmemu(cmd) => self.cmemu_command(cmd, out)?,
            Commands::Test(Test::Ping) => {
                outputln!(out, "pong");
            }
            Commands::Test(Test::Panic) => {
                panic!("On demand");
            }
            Commands::Test(Test::FatalError) => {
                Err(MonitorError::Fatal("Simulated fatal error".into()))?;
            }
            Commands::Test(Test::UsageError) => Err(MonitorError::Clap(clap::Error::raw(
                ErrorKind::InvalidValue,
                "Some failure case",
            )))?,
            Commands::Test(Test::RuntimeError) => Err(MonitorError::Runtime("Ouch!".into()))?,
            Commands::Test(Test::RuntimeDynError) => {
                Err(MonitorError::RuntimeDyn(io::Error::last_os_error().into()))?;
            }
        }
        Ok(())
    }

    fn std_emulation_time(&self) -> Duration {
        (self.emu.get_emulation_time() - Timepoint::ZERO).into_std()
    }

    #[allow(clippy::unused_self, reason = "API design for future")]
    pub(super) fn styles(&self) -> clap::builder::Styles {
        clap::builder::Styles::styled()
    }
}

/// Allow an external debugger to connect to CMEmu by PID.
///
/// On Linux with YAMA `ptrace_scope=1` (default for modern distros),
/// only an ancestor process can debug another one.
/// While most docs will tell you to disable the protection globally,
/// a process may explicitly allow another one to debug it with `PR_SET_PTRACER`.
///
/// If `dbg_pid` is not specified, any process of the current user will be allowed to connect.
///
/// See: <https://www.kernel.org/doc/Documentation/security/Yama.txt>
#[allow(clippy::redundant_closure_for_method_calls)]
#[allow(unsafe_code, reason = "There is no wrapper, but it is safe.")]
#[cfg(target_os = "linux")]
fn allow_debuggers(dbg_pid: Option<u32>) -> DynResult<()> {
    let ptrace_scope = fs::read_to_string("/proc/sys/kernel/yama/ptrace_scope")?;
    match ptrace_scope.trim().parse() {
        Ok(0) => return Ok(()),
        Ok(1) => (),
        Ok(2) => Err("YAMA ptrace_scope=2 disables designation of a debugger!".to_owned())?,
        Ok(3) => Err("YAMA ptrace_scope=3 disables debuggers altogether!".to_owned())?,
        Ok(code) => Err(format!("Unknown ptrace_scope: {code}"))?,
        Err(err) => Err(format!(
            "Cannot parse ptrace_scope: {err}: '{ptrace_scope}'"
        ))?,
    }

    let pid: libc::c_ulong = dbg_pid.map_or_else(|| libc::PR_SET_PTRACER_ANY, |p| p.into());
    // SAFETY:
    // From rust perspective, this doesn't manipulate memory itself.
    // A debugger could have connected in another way.
    //
    // However, this may expose additional attack vectors if someone
    // thinks giving access to a simulated gdbserver is a security boundary.
    let res = unsafe { libc::prctl(libc::PR_SET_PTRACER, pid) };
    if res != 0 {
        Err(io::Error::last_os_error())?;
    }
    Ok(())
}

impl MonitorCmd for DebugMonitor {
    fn handle_monitor_cmd(
        &mut self,
        cmd: &[u8],
        mut out: ConsoleOutput<'_>,
    ) -> Result<(), Self::Error> {
        let styles = self.styles();
        let red = styles.get_error();
        let Ok(cmd) = str::from_utf8(cmd) else {
            outputln!(out, "{red}The command is not valid UTF-8!{red:#}");
            return Ok(());
        };
        let Some(args) = shlex::split(cmd) else {
            outputln!(out, "{red}Invalid quoting of the command!{red:#}");
            return Ok(());
        };

        let cmd = match Monitor::try_parse_from(args) {
            Ok(cmd) => cmd,
            Err(err) => {
                outputln!(out, "{}", err.render().ansi());
                return Ok(());
            }
        };
        self.monitor_main(cmd.command, &mut out)
            .or_else(|e| match e {
                MonitorError::Clap(ce) => {
                    outputln!(
                        out,
                        "{}",
                        ce.format(&mut Monitor::command()).render().ansi()
                    );
                    Ok(())
                }
                MonitorError::Runtime(d) => {
                    outputln!(out, "{red}Error{red:#}: {d}");
                    Ok(())
                }
                MonitorError::RuntimeDyn(d) => {
                    outputln!(out, "{red}Error{red:#}: {d}");
                    Ok(())
                }
                MonitorError::Fatal(e) => Err(e),
            })
    }
}
