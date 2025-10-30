// Testing of (hosted) execution of cross-compiled binaries.
//
// For now, there are two main ways for the test to run cmemu:
// - `cmemu_bin_run` will execute a ``cmemu`` binary in a subprocess,
//    enabling us to capture the whole stdout and stderr (including logs, uart lite to file),
// - `run_emulator` uses ``cmemu`` as a lib to execute the emulator in the current thread,
//    allowing us to configure cmemu using component_api (uart lite to vec, radio mock).

use assert_cmd::{Command, cargo_bin};
use clap::Parser;
use cmemu::{App, configure, run_capture_semihosting};
use cmemu_lib::common::{CcaReq, ModemInterface, ModemOp, UARTLiteInterface};
use cmemu_lib::engine::Emulator;
use log::{debug, trace};
use std::error::Error;
use std::ffi::OsString;
use std::mem;
use std::panic::UnwindSafe;
use std::path::Path;
use std::process::ExitCode;
use std::sync::{Arc, Mutex, OnceLock};
use std::time::Duration;

const ROM_PATH: &str = concat!(env!("OUT_DIR"), "/rom.bin");
const DEFAULT_ELF_TEST_CYCLES_TIMEOUT: u64 = 10_000;
const DEFAULT_REALTIME_TIMEOUT: Duration = Duration::from_secs(30);

macro_rules! test_path {
    ($p:expr) => {
        concat!(env!("CARGO_MANIFEST_DIR"), "/tests/elf/", $p)
    };
}

/// Assert a `predicate` evaluates to true.
///
/// Since we're already hooked to the `predicates` crate with `assert_cmd`,
/// this macro simplified employing them for general purpose assertions.
///
#[macro_export]
macro_rules! assert_predicate {
    ($value:expr, $predicate:expr) => {
        $crate::assert_predicate!($value, $predicate, "")
    };
    ($value:expr, $predicate:expr, $($tt:tt)+) => {
        $predicate
            .find_case(false, $value)
            .map(|x| panic!("Assertion failed: {ctx}\n {x:#?}", ctx=format_args!($($tt)+)));
    };
}

// The current file is not actually considered a module "elf.rs", but silently top-level "lib.rs"
mod elf {
    mod contiki;
    mod hosted;
    mod whip6;
}

// Helpers for tests calling cmemu binary

/// Emulation-time timeout to the emulator
#[derive(Debug, Copy, Clone)]
enum Timeout {
    Cycles(u64),
    Seconds(f64),
    Default,
    #[allow(unused)]
    Unlimited,
}

fn cmemu_command() -> Command {
    // Note: Command::cargo_bin() doesn't guarantee the target is up to date, as it has fallbacks!
    Command::new(cargo_bin!("cmemu-bin-dep"))
}

fn cmemu_bin_run(elf_path: impl AsRef<Path>, timeout: Timeout, wants_rom: bool) -> Command {
    // Note! This is not strictly triggering a new build and may use an old binary!
    let mut cmd = cmemu_command();
    cmd.timeout(DEFAULT_REALTIME_TIMEOUT);

    match timeout {
        Timeout::Cycles(n) => cmd.arg("--cycles").arg(n.to_string()),
        Timeout::Seconds(n) => cmd.arg("--seconds").arg(n.to_string()),
        Timeout::Default => cmd
            .arg("--cycles")
            .arg(DEFAULT_ELF_TEST_CYCLES_TIMEOUT.to_string()),
        Timeout::Unlimited => &mut cmd, // wth?
    };

    // This is asserted at the test ignore level
    if wants_rom {
        assert!(cfg!(any(
            cmemu_has_rom = "driverlib",
            cmemu_has_rom = "full"
        )));
        cmd.arg("--rom").arg(ROM_PATH);
    }

    cmd.arg(elf_path.as_ref());
    cmd
}

// Helpers for tests operating on the emulator interface
// TODO: move it into a lib? integrate with cmemu-flash-test-lib? integrate brakpoints?

fn run_emulator(
    elf_path: impl AsRef<Path>,
    timeout: Timeout,
    wants_rom: bool,
    config_cb: impl FnOnce(&mut Emulator),
) -> Result<ExitCode, Box<dyn Error>> {
    let mut args = App::parse_from([
        OsString::from("cmemu").as_os_str(),
        elf_path.as_ref().as_os_str(),
    ]);
    match timeout {
        Timeout::Cycles(n) => args.duration.cycles = Some(n),
        Timeout::Seconds(n) => args.duration.seconds = Some(n),
        Timeout::Default => args.duration.cycles = Some(DEFAULT_ELF_TEST_CYCLES_TIMEOUT),
        Timeout::Unlimited => (),
    }
    if wants_rom {
        args.rom_file = Some(ROM_PATH.into());
    }
    let duration = args.duration.clone();
    let mut emu = configure(args)?;
    config_cb(&mut emu);
    run_capture_semihosting(emu, duration)
}

/// Struct for lock-less/atomic-less collection of the data.
/// The only lock is in Drop, when the data becomes available.
struct CollectUartLiteBackend(Vec<u8>, Arc<OnceLock<Vec<u8>>>);

impl UnwindSafe for CollectUartLiteBackend {}

impl CollectUartLiteBackend {
    fn new() -> Self {
        Self(Vec::new(), Default::default())
    }
    fn get_promise(&self) -> Arc<OnceLock<Vec<u8>>> {
        Arc::clone(&self.1)
    }
}

impl UARTLiteInterface for CollectUartLiteBackend {
    fn send_byte(&mut self, byte: u8) {
        self.0.push(byte);
    }
}

impl Drop for CollectUartLiteBackend {
    fn drop(&mut self) {
        self.1.set(mem::take(&mut self.0)).unwrap();
    }
}

/// uart lite backend that allows shared (interactive) access to the buffer
#[allow(dead_code)]
struct SharedBufUartLiteBackend(Arc<Mutex<Vec<u8>>>);

#[allow(dead_code)]
impl SharedBufUartLiteBackend {
    fn new() -> Self {
        Self(Default::default())
    }

    fn get_arc(&self) -> Arc<Mutex<Vec<u8>>> {
        Arc::clone(&self.0)
    }
}
impl UnwindSafe for SharedBufUartLiteBackend {}

impl UARTLiteInterface for SharedBufUartLiteBackend {
    fn send_byte(&mut self, byte: u8) {
        self.0.lock().unwrap().push(byte);
    }
}

// no lock-less version as the interface gives us only a shared reference anyway!
/// Struct for collection of logs of the radio operations.
/// This mock will always return ok/done/no-input on requests to the modem,
/// what could hang the application.
struct SharedLogModemInterface(Arc<Mutex<MockModem>>);

#[derive(Debug, Clone, Default)]
struct MockModem {
    log: Vec<ModemRequest>,
    wanted_tx: bool,
}
// TODO: implement mock support for replies

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum ModemRequest {
    SendOp(ModemOp),
    TakeRx,
    TakeTxFinished,
    CcaRead,
}

impl UnwindSafe for SharedLogModemInterface {}

impl SharedLogModemInterface {
    fn new() -> Self {
        Self(Default::default())
    }
    #[must_use]
    fn get_arc(&self) -> Arc<Mutex<MockModem>> {
        Arc::clone(&self.0)
    }
}

impl ModemInterface for SharedLogModemInterface {
    fn send_op(&self, op: ModemOp) {
        debug!(target: "cmemu_tests::elf", "MockModem::send_op({op:?})");
        let mut modem = self.0.lock().unwrap();
        if let ModemOp::Strobe(_) = op {
            modem.wanted_tx = true;
        }
        modem.log.push(ModemRequest::SendOp(op));
    }

    fn take_rx(&self) -> Option<Vec<u8>> {
        self.0.lock().unwrap().log.push(ModemRequest::TakeRx);
        trace!(target: "cmemu_tests::elf", "MockModem::take_rx() = None");
        None
    }

    fn take_tx_finished(&self) -> Option<()> {
        let mut modem = self.0.lock().unwrap();
        modem.log.push(ModemRequest::TakeTxFinished);
        let res = modem.wanted_tx.then(|| {
            modem.wanted_tx = false;
        });
        trace!(target: "cmemu_tests::elf", "MockModem::take_tx_finished() = {res:?}");
        res
    }

    fn cca_read(&self) -> Option<CcaReq> {
        debug!(target: "cmemu_tests::elf", "MockModem::cca_read() = clear");
        self.0.lock().unwrap().log.push(ModemRequest::CcaRead);
        Some(CcaReq::new_clear())
    }
}
