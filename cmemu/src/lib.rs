use clap::{Args, Parser};
use cmemu_lib::common::{RequestedExit, UARTLiteInterface};
use cmemu_lib::engine::{Emulator, Timepoint};
use flexi_logger::LoggerHandle;
use log::{error, info, warn};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io::Write;
use std::panic::UnwindSafe;
use std::path::PathBuf;
use std::process::{ExitCode, Termination};
use std::{fs, io};

#[derive(Debug)]
pub struct TimeoutError(Duration);
impl Display for TimeoutError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Timed out after {}.", self.0)
    }
}
impl Error for TimeoutError {}

#[derive(Debug)]
pub struct ConfigError(&'static str, Option<io::Error>);
impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}

impl Error for ConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.1.as_ref().map(|e| e as &(dyn Error + 'static))
    }
}

#[derive(Parser, Debug)]
#[command(version, about = "Cherry Mote Emulator", arg_required_else_help = true)]
#[non_exhaustive]
pub struct App {
    /// flash memory contents
    pub flash_file: PathBuf,

    #[arg(long, alias("rom"))]
    /// rom memory contents, zeroed if unspecified
    pub rom_file: Option<PathBuf>,

    #[arg(long, alias("uart"))]
    /// path to dump data sent to UART Lite (scif) to, hint: try `/dev/stdout`
    pub uart_lite_dump: Option<PathBuf>,

    #[arg(long, alias("mocked-mem-os"))]
    /// name of OS for which memory accesses should be mocked (DEPRECATED)
    pub mocked_memory_os: Option<String>,

    #[command(flatten, next_help_heading = "Duration")]
    pub duration: Duration,

    #[arg(long, alias("cdl-file"))]
    #[cfg_attr(not(feature = "cycle-debug-logger"), arg(value_parser = reject_missing_cdl_support))]
    /// output json file with cycle debug information; to be loaded with cdl-viewer
    ///
    /// Supports compression (use `.gz` extension) and outputting multiple parts into a directory
    /// (use `.d` extension).
    pub cycle_debug_log_file: Option<PathBuf>,

    #[cfg(feature = "elf")]
    #[command(flatten, next_help_heading = "Elf options")]
    pub elf_params: cmemu_elf_loader::ElfArgs,

    #[cfg(feature = "gdb")]
    #[command(flatten, next_help_heading = "Gdb options")]
    pub gdb_params: cmemu_gdb::GdbArgs,
}

#[derive(Args, Debug, Clone)]
#[group(multiple = false, required = false)]
#[non_exhaustive]
pub struct Duration {
    #[arg(long, group = "duration")]
    /// number of emulator cycles to be performed
    pub cycles: Option<u64>,

    #[arg(long, group = "duration", allow_negative_numbers = false)]
    /// number of seconds of emulation cycles to be performed
    pub seconds: Option<f64>,
}

impl Display for Duration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match (self.cycles, self.seconds) {
            (Some(cycles), None) => write!(f, "{cycles} cycles"),
            (None, Some(seconds)) => write!(f, "{seconds} seconds"),
            (None, None) => write!(f, "forever"),
            (Some(_), Some(_)) => unreachable!(),
        }
    }
}

#[allow(clippy::needless_pass_by_value, reason = "false positive")]
pub fn run(
    args: App,
    #[allow(unused)] logger: Option<LoggerHandle>,
) -> Result<ExitCode, Box<dyn Error>> {
    let duration = args.duration.clone();
    #[cfg(feature = "gdb")]
    if args.gdb_params.wants_to_manage_loop() {
        let mut gdb_params = args.gdb_params.clone();
        // FIXME: this interface of passing some params down is not very scalable,
        //        but we cannot let cmemu_gdb know the App type as that introduces circ deps.
        gdb_params.flash_file = Some(args.flash_file.clone());
        let emulator = configure(args)?;
        return gdb_params.run_emulator(emulator, duration.cycles, logger);
    }
    let emulator = configure(args)?;
    run_capture_semihosting(emulator, duration)
}

#[allow(clippy::needless_pass_by_value, reason = "false positive")]
pub fn configure(args: App) -> Result<Emulator, Box<dyn Error>> {
    if args.mocked_memory_os.is_some() {
        warn!(
            "Attempted to set mocked memory OS: option is deprecated following memory mock unification"
        );
    }

    let flash_mem = fs::read(&args.flash_file)
        .map_err(|err| ConfigError("Failed to load Flash memory file", Some(err)))?;

    let rom_mem = if let Some(ref f) = args.rom_file {
        Some(fs::read(f).map_err(|err| ConfigError("Failed to load ROM memory file", Some(err)))?)
    } else {
        None
    };

    let uart_lite_dump = if let Some(ref f) = args.uart_lite_dump {
        Some(FileBasedUartLiteBackend(fs::File::create(f).map_err(
            |err| ConfigError("Failed to open UART Lite dump file", Some(err)),
        )?))
    } else {
        None
    };

    // construct emulator, configure it & run it
    let mut emulator = {
        #[cfg(feature = "elf")]
        {
            let elf =
                cmemu_elf_loader::ElfLoader::new(&flash_mem, rom_mem.as_deref(), &args.elf_params);
            let mut emulator = Emulator::new(elf.flash_base(), elf.rom_base());
            elf.load(&mut emulator);
            emulator
        }
        #[cfg(not(feature = "elf"))]
        {
            assert!(
                !flash_mem.starts_with("\x7fELF".as_ref()),
                "ELF support is not compiled. Build this binary with the `elf` feature."
            );
            Emulator::new(&flash_mem, rom_mem.as_deref())
        }
    };

    if let Some(scif_dumper) = uart_lite_dump {
        emulator.set_uart_lite_interface(Some(Box::new(scif_dumper)));
    }

    #[cfg(feature = "cycle-debug-logger")]
    if let log_file @ Some(_) = args.cycle_debug_log_file {
        emulator.set_cycle_debug_logger_file(log_file);
        emulator.set_cycle_debug_logger_metadata(
            "Flash file",
            args.flash_file.to_string_lossy().to_string(),
        );
        emulator.set_cycle_debug_logger_metadata(
            "ROM file",
            args.rom_file.map_or_else(
                || "<none>".to_owned(),
                |cow| cow.to_string_lossy().to_string(),
            ),
        );
        emulator.cycle_debug_logger_start_recording();
    }

    Ok(emulator)
}

pub fn run_capture_semihosting(
    emulator: Emulator,
    duration: Duration,
) -> Result<ExitCode, Box<dyn Error>> {
    info!("Executing {}.", duration);
    let runner = || run_emulator(emulator, duration);
    std::panic::catch_unwind(runner).unwrap_or_else(|p| match p.downcast::<RequestedExit>() {
        Ok(code) => Ok(ExitCode::from(code.code())),
        Err(p) => std::panic::resume_unwind(p),
    })
}

fn run_emulator(mut emulator: Emulator, duration: Duration) -> Result<ExitCode, Box<dyn Error>> {
    // TODO: reintroduce siginfo while accounting for step_until?
    match (duration.cycles, duration.seconds) {
        (Some(cycles), None) => {
            for _ in 0..cycles {
                emulator.step_cycle();
            }
        }
        (None, Some(seconds)) => {
            #[allow(
                clippy::cast_possible_truncation,
                clippy::cast_sign_loss,
                reason = "We're okay as this is CLI."
            )]
            emulator.step_until(Timepoint::from_picos(
                (seconds * 1_000_000_000_000f64) as u64,
            ));
        }
        (None, None) => loop {
            emulator.step_cycle();
        },
        (Some(_), Some(_)) => unreachable!(),
    }
    error!("Timed out after {}.", duration);
    Err(TimeoutError(duration).into())
}

#[cfg(not(feature = "cycle-debug-logger"))]
fn reject_missing_cdl_support(_: &str) -> Result<PathBuf, String> {
    Err("emulator was built without Cycle Debug Logger support".to_owned())
}

struct FileBasedUartLiteBackend(fs::File);

impl UnwindSafe for FileBasedUartLiteBackend {}

impl UARTLiteInterface for FileBasedUartLiteBackend {
    fn send_byte(&mut self, byte: u8) {
        self.0
            .write_all(&[byte])
            .expect("Error while dumping UARTLite.");
    }
}

// For whatever reason, default <Result as Termination> uses debug instead of Display + source.
#[derive(Debug)]
pub struct TerminationDisplay<T: Termination, E: AsRef<dyn Error>>(Result<T, E>);

pub type PrettyTermination = TerminationDisplay<ExitCode, Box<dyn Error>>;
impl<T: Termination, E: AsRef<dyn Error>> Termination for TerminationDisplay<T, E> {
    fn report(self) -> ExitCode {
        match self.0 {
            Ok(val) => val.report(),
            Err(err) => {
                let mut err: &dyn Error = err.as_ref();
                eprintln!("Error: {err}");
                while let Some(next) = err.source() {
                    eprintln!("Caused by: {next}");
                    err = next;
                }
                ExitCode::FAILURE
            }
        }
    }
}

impl<T: Termination, E: AsRef<dyn Error>> From<Result<T, E>> for TerminationDisplay<T, E> {
    fn from(value: Result<T, E>) -> Self {
        Self(value)
    }
}

#[cfg(test)]
mod test_clap {
    use super::*;

    #[test]
    fn app_clap_assert() {
        <App as clap::CommandFactory>::command().debug_assert();
    }
}
