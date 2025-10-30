use clap::Args;
use cmemu_lib::engine::Emulator;
use flexi_logger::LoggerHandle;
use gdbstub::common::Signal;
use gdbstub::conn::{Connection, ConnectionExt};
use gdbstub::stub::run_blocking::{BlockingEventLoop, Event, WaitForStopReasonError};
use gdbstub::stub::{DisconnectReason, GdbStub, SingleThreadStopReason};
use gdbstub::target::Target;
use log::{debug, trace};
use std::error::Error;
use std::ffi::OsString;
use std::fmt::Display;
use std::net;
#[cfg(unix)]
use std::os::unix::net as unix_net;
use std::path::PathBuf;
use std::process::{Child, Command, ExitCode};
use std::str::FromStr;
use std::time::{Duration, Instant};
use std::{fs, io};

pub mod arch;
mod gdb;

use gdb::DebugMonitor;

type DynResult<T> = Result<T, Box<dyn Error>>;
type ConnectionType = Box<dyn ConnectionExt<Error = io::Error>>;
type StopReason = SingleThreadStopReason<u32>;

#[derive(Args, Debug, Default, Clone)]
#[non_exhaustive]
pub struct GdbArgs {
    #[arg(long, group = "address")]
    /// Make the emulator act as a gdb server available at the given location.
    ///
    /// Use `target remote <location>` in gdb to connect to cmemu.
    pub gdb: Option<String>,

    /// Like --gdb, but attempt to automatically spawn arm-none-eabi-gdb here
    // Option<Option<T>> is for clap to understand optional arg with optional val
    #[arg(long, group = "address")]
    #[cfg_attr(not(unix), arg(hide = true))]
    pub gdb_here: Option<Option<String>>,

    /// This should be used to pass `flash_file` from the main App parameters.
    #[arg(skip)]
    pub flash_file: Option<PathBuf>,
}

// Is there really no default for that?
#[derive(Debug)]
enum BindPath {
    TcpPort(net::SocketAddr),
    #[cfg(unix)]
    Unix(unix_net::SocketAddr),
}

impl Display for BindPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BindPath::TcpPort(addr) => write!(f, "tcp:{addr}"),
            BindPath::Unix(addr) => {
                if let Some(path) = addr.as_pathname() {
                    write!(f, "{}", path.display())
                } else {
                    write!(f, "unix:{addr:?}")
                }
            }
        }
    }
}

impl BindPath {
    fn bind(self) -> DynResult<(ConnectionType, BindPath)> {
        match self {
            BindPath::TcpPort(addr) => {
                let sock = net::TcpListener::bind(addr)?;
                let (stream, addr) = sock.accept()?;
                Ok((Box::new(stream), BindPath::TcpPort(addr)))
            }
            #[cfg(unix)]
            BindPath::Unix(addr) => {
                if let Some(path) = addr.as_pathname()
                    && path.exists()
                {
                    fs::remove_file(path)?;
                }
                let sock = unix_net::UnixListener::bind_addr(&addr)?;
                let (stream, addr) = sock.accept()?;
                Ok((Box::new(stream), BindPath::Unix(addr)))
            }
        }
    }
}

impl FromStr for BindPath {
    type Err = Box<dyn Error>;

    fn from_str(bind_to: &str) -> Result<Self, Self::Err> {
        Ok(if let Ok(port) = bind_to.parse::<u64>() {
            // Parse larger numbers just to yell at the user
            let port: u16 = port.try_into().map_err(|_| "TCP port is 16 bit!")?;
            BindPath::TcpPort(net::SocketAddr::from(([127, 0, 0, 1], port)))
        } else if let Ok(addr) = bind_to.parse::<net::SocketAddr>() {
            BindPath::TcpPort(addr)
        } else {
            #[cfg(unix)]
            {
                BindPath::Unix(unix_net::SocketAddr::from_pathname(bind_to)?)
            }
            #[cfg(not(unix))]
            {
                Err("Binding to a path (unix socket) is not supported on this platform".into())?
            }
        })
    }
}

impl GdbArgs {
    #[must_use]
    pub fn wants_to_manage_loop(&self) -> bool {
        self.gdb.is_some() || self.gdb_here.is_some()
    }

    #[cfg(feature = "flash-test-lib")]
    pub fn run_emulator_for_flash_test(
        mut self,
        emulator: Emulator,
        cycles_timout: Option<u64>,
        dump: cmemu_flash_test_lib::TestDump,
        logger: Option<LoggerHandle>,
    ) -> DynResult<ExitCode> {
        let image_path = self.flash_file.take().unwrap();
        let mut emu = DebugMonitor::new(
            emulator,
            cycles_timout,
            image_path,
            logger.expect("cmemu-gdb ran without logger: create a default one?"),
        );
        emu.configure_for_test(dump)?;
        self.run_loop(&mut emu)
    }

    pub fn run_emulator(
        mut self,
        emulator: Emulator,
        cycles_timout: Option<u64>,
        logger: Option<LoggerHandle>,
    ) -> DynResult<ExitCode> {
        let image_path = self.flash_file.take().unwrap();
        let mut emu = DebugMonitor::new(
            emulator,
            cycles_timout,
            image_path,
            logger.expect("cmemu-gdb ran without logger: create a default one?"),
        );

        self.run_loop(&mut emu)
    }

    fn run_loop(self, dm: &mut DebugMonitor) -> DynResult<ExitCode> {
        let (conn, child) = self.wait_for_gdb()?;
        let gdb = GdbStub::new(conn);
        if self.gdb_here.is_some() {
            // NOTE: hitting Ctrl-C should reliably produce only a single "stopped by sigint".
            // Report an error if it does not on your platform.
            dm.install_ctrl_c_ignoring()?;
        } else {
            dm.install_ctrl_c_handler()?;
        }

        let exit_result = gdb.run_blocking::<GdbEventLoop>(dm);

        let res = match exit_result {
            Ok(DisconnectReason::TargetExited(code)) => Ok(ExitCode::from(code)),
            Ok(DisconnectReason::Disconnect) => Ok(ExitCode::SUCCESS),
            Ok(DisconnectReason::Kill) => {
                eprintln!("Kill command received");
                Ok(ExitCode::FAILURE)
            }
            Ok(DisconnectReason::TargetTerminated(sig)) => {
                eprintln!("Target terminated with signal {sig} (?!)");
                Ok(ExitCode::FAILURE)
            }
            Err(e) => Err(format!("Error: {e}").into()),
        };
        if let Some(mut child) = child {
            child.wait()?;
        }
        res
    }

    fn wait_for_gdb(&self) -> Result<(ConnectionType, Option<Child>), Box<dyn Error>> {
        let res = if self.gdb_here.is_some() {
            self.spawn_gdb_client()?
        } else {
            (self.wait_for_connection()?, None)
        };
        eprintln!();
        eprintln!("Hint: ROM symbols are not automatically loaded. Try:");
        eprintln!("add-symbol-file rom/driverlib.elf");
        Ok(res)
    }

    fn wait_for_connection(&self) -> DynResult<ConnectionType> {
        let bind_path: BindPath = self.gdb.as_ref().unwrap().parse()?;

        eprintln!("Waiting for a GDB connection on {bind_path}...");
        eprintln!("You can connect using:");
        eprintln!("target remote {bind_path}");
        eprintln!();
        eprintln!("For a quickstart, just run on this machine:");
        eprintln!("arm-none-eabi-gdb -iex \"target remote {bind_path}\"");
        let (stream, source) = bind_path.bind()?;
        eprintln!("Debugger connected from {source:?}");
        Ok(stream)
    }

    #[cfg(not(unix))]
    fn spawn_gdb_client(&self) -> DynResult<(ConnectionType, Option<Child>)> {
        Err("How did you get that? --gdb-here is not implemented for this platform yet".into())
    }

    #[cfg(unix)]
    fn spawn_gdb_client(&self) -> DynResult<(ConnectionType, Option<Child>)> {
        let (listener, tmp_path) = tempfile::Builder::new()
            .prefix("cmemu-gdb-")
            .suffix(".sock")
            .make(|path| unix_net::UnixListener::bind(path))?
            .into_parts();
        debug!("Listening on {listener:?} for immediate connection.");

        // Is it the correct way to concat OsStrings?
        let mut target_arg = OsString::from("target remote ");
        target_arg.push(&tmp_path); // NOTE: we would drop the file!

        let extra_args = self
            .gdb_here
            .clone() // really? I just want .flatten_as_ref!
            .flatten()
            .and_then(|s| shlex::split(&s))
            .unwrap_or_default();

        // Note: we need to handle Ctrl-C interrupts ourselves.
        let child = find_gdb_command()?
            // Interesting: we can either pass the binary here and use "ex",
            // or just use "iex" and implement exec-file extension.
            // .arg(self.flash_file.as_ref().unwrap())
            .arg("-iex")
            .arg(target_arg)
            .args(extra_args.iter())
            .spawn()?;
        let (conn, _) = listener.accept()?;

        trace!("Connected to {conn:?}. Dropping tmp now.");
        Ok((Box::new(conn), Some(child)))
    }
}

fn find_gdb_command() -> DynResult<Command> {
    let gdb_commands = [
        ("arm-none-eabi-gdb", vec![]),
        ("gdb-multiarch", vec!["-iex", "set architecture armv7"]),
    ];
    for (program, initial_args) in &gdb_commands {
        if Command::new(program)
            .arg("--version")
            .status()
            .is_ok_and(|status| status.success())
        {
            let mut command = Command::new(program);
            command.args(initial_args);
            return Ok(command);
        }
    }
    return Err(format!(
        "GDB not found (need one of {})",
        gdb_commands.map(|(program, _)| program).join(", ")
    )
    .into());
}

struct GdbEventLoop;

impl BlockingEventLoop for GdbEventLoop {
    type Target = DebugMonitor;
    type Connection = ConnectionType;
    type StopReason = SingleThreadStopReason<u32>; // Target::Arch::Usize

    fn wait_for_stop_reason(
        target: &mut Self::Target,
        conn: &mut Self::Connection,
    ) -> Result<
        Event<Self::StopReason>,
        WaitForStopReasonError<
            <Self::Target as Target>::Error,
            <Self::Connection as Connection>::Error,
        >,
    > {
        trace!("GdbStub wants us to wait for a stop reason");
        // TODO: this is naive polling (each is a syscall!)
        // TODO: checking time uses VDSO probably, maybe we should limit this too
        let check_every = Duration::from_millis(10);
        'main: loop {
            let epoch_start = Instant::now();

            while epoch_start.elapsed() < check_every {
                let step_result = target
                    .step_check_event()
                    .map_err(WaitForStopReasonError::Target)?;
                if let Some(event) = step_result {
                    break 'main Ok(Event::TargetStopped(event));
                }
            }

            if conn
                .peek()
                .map_err(WaitForStopReasonError::Connection)?
                .is_some()
            {
                break 'main Ok(Event::IncomingData(conn.read().unwrap()));
            }
        }
    }

    fn on_interrupt(
        _target: &mut Self::Target,
    ) -> Result<Option<Self::StopReason>, <Self::Target as Target>::Error> {
        trace!("GdbStub wants us to handle Ctrl-C");
        // The emulator runs only within the `wait_for_stop_reason` method,
        // so if we receive this action, that method cannot be running.
        // Hence, nothing needs to be done here.
        Ok(Some(SingleThreadStopReason::Signal(Signal::SIGINT)))
    }
}
