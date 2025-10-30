// Testing simple contiki apps
// Binary names indicates their target in name.
// In particular, `-cmemuhosting` variants are partially aware of cmemu (they exit with cmemu addresses),
// which would generate a MemFault on the hardware.
//
// See `<playground>/bd370768/example-apps/Contiki-NG/README.md` for descriptions of the apps.

use crate::{
    CollectUartLiteBackend, ModemRequest, SharedLogModemInterface, Timeout, cmemu_bin_run,
    run_emulator,
};
use cmemu_lib::common::ModemOp;
use cmemu_lib::common::ModemOp::RequestCca;
use predicates::prelude::*;
use rstest::rstest;
use std::mem;
use std::process::ExitCode;
use std::time::Duration;

macro_rules! contiki_examples_path {
    ($p:expr) => {
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../bd370768/example-apps/Contiki-NG/",
            $p
        )
    };
}

// "Raw" tests with no networking stack

#[test]
#[cfg_attr(not(unix), ignore = "needs /dev/stdout")]
#[cfg_attr(
    not(any(cmemu_has_rom = "driverlib", cmemu_has_rom = "full")),
    ignore = "No ROM"
)]
fn just_printf_and_exit_cmd() {
    // A minimal app that has a process that uses UART-Lite for communication and immediately exists.
    cmemu_bin_run(
        contiki_examples_path!("cc26x0-cc13x0_just-printf-cmemuhosting.elf"),
        Timeout::Cycles(1_000_000), // without nm-unstable it's just slower?
        true,
    )
    .args(["--uart-lite-dump", "/dev/stdout"])
    .assert()
    .code(42)
    .stdout(predicate::str::contains("Starting Contiki-NG-release"))
    .stdout(predicate::str::contains("Hello, world"));
}

#[test]
#[cfg_attr(
    not(any(cmemu_has_rom = "driverlib", cmemu_has_rom = "full")),
    ignore = "No ROM"
)]
fn just_printf_and_exit() {
    let uart_lite = CollectUartLiteBackend::new();
    let buf = uart_lite.get_promise();
    // A minimal app that has a process that uses UART-Lite for communication and immediately exists.
    let code = run_emulator(
        contiki_examples_path!("cc26x0-cc13x0_just-printf-cmemuhosting.elf"),
        Timeout::Cycles(1_000_000), // without nm-unstable it's just slower?
        true,
        |emu| emu.set_uart_lite_interface(Some(Box::new(uart_lite))),
    )
    .unwrap();
    assert_eq!(code, ExitCode::from(42));

    let buf = buf.get().unwrap();
    let stdout = String::from_utf8_lossy(buf);

    assert!(predicate::str::contains("Starting Contiki-NG-release").eval(&stdout));
    assert!(predicate::str::contains("Hello, world").eval(&stdout));
}

#[test]
#[cfg_attr(
    not(any(cmemu_has_rom = "driverlib", cmemu_has_rom = "full")),
    ignore = "No ROM"
)]
fn sleep_idle_and_exit() {
    // TODO: test for "really going to sleep" in logs with cmemu_bin_run?
    let code = run_emulator(
        contiki_examples_path!("cc26x0-cc13x0_sleep-idle-cmemuhosting.elf"),
        Timeout::Cycles(2_000_000), // without nm-unstable it's just slower?
        true,
        |_emu| {},
    )
    .unwrap();
    assert_eq!(code, ExitCode::from(42));
}

#[test]
#[cfg_attr(
    not(any(cmemu_has_rom = "driverlib", cmemu_has_rom = "full")),
    ignore = "No ROM"
)]
fn sleep_standby_and_exit() {
    let code = run_emulator(
        contiki_examples_path!("cc26x0-cc13x0_sleep-standby-cmemuhosting.elf"),
        Timeout::Cycles(1_000_000), // without nm-unstable it's just slower?
        true,
        |_emu| {},
    )
    .unwrap();
    assert_eq!(code, ExitCode::from(42));
}

#[test]
#[ignore = "implement AON_EVENT::AUXWUSEL, CTL0_MASK16B_LOWEST_16_BITS_ADDR"]
#[cfg_attr(
    not(any(cmemu_has_rom = "driverlib", cmemu_has_rom = "full")),
    ignore = "No ROM"
)]
fn shutdown_and_exit() {
    let code = run_emulator(
        contiki_examples_path!("cc26x0-cc13x0_shutdown-cmemuhosting.elf"),
        Timeout::Cycles(1_000_000), // without nm-unstable it's just slower?
        true,
        |_emu| {},
    )
    .unwrap();
    assert_eq!(code, ExitCode::from(42));
}

// Hello world applications with mini network stack

#[rstest]
#[cfg_attr(not(unix), ignore = "needs /dev/stdout")]
#[cfg_attr(
    not(any(cmemu_has_rom = "driverlib", cmemu_has_rom = "full")),
    ignore = "No ROM"
)]
fn hello_world_just_printf_cmd() {
    // Prints "Hello, world" every 10 seconds, enters the Standby sleep.
    cmemu_bin_run(
        contiki_examples_path!("cc26x0-cc13x0_hello-world-just-printf.elf"),
        Timeout::Seconds(2.), // without nm-unstable it's just slower?
        true,
    )
    .args(["--uart-lite-dump", "/dev/stdout"])
    .assert()
    .failure()
    .stderr(predicate::str::contains("Timed out after 2 seconds."))
    .stdout(predicate::str::contains("Starting Contiki-NG-release"))
    .stdout(predicate::str::contains("Hello, world"));
}

#[rstest]
#[timeout(Duration::from_secs(30))]
#[cfg_attr(
    not(any(cmemu_has_rom = "driverlib", cmemu_has_rom = "full")),
    ignore = "No ROM"
)]
fn hello_world_no_radio() {
    // The *hello-world* of Contiki-NG.
    // Prints "Hello, world" every 10 seconds, enters the Standby sleep.
    let uart_lite = CollectUartLiteBackend::new();
    let buf = uart_lite.get_promise();

    let err = run_emulator(
        contiki_examples_path!("cc26x0-cc13x0_hello-world-no-radio.elf"),
        Timeout::Seconds(30.),
        true,
        |emu| {
            emu.set_uart_lite_interface(Some(Box::new(uart_lite)));
        },
    )
    .unwrap_err();
    assert_predicate!(
        &err.to_string(),
        predicate::str::contains("Timed out after 30 seconds."),
        "Timeout should be exactly 30 seconds!"
    );

    let stdout = String::from_utf8_lossy(buf.get().unwrap());

    assert_predicate!(&*stdout, predicate::str::contains("Hello, world").count(3));
    // TODO: make radio interface empty! (this is not the case right now)
}

#[rstest]
// #[test_log::test]
#[timeout(Duration::from_secs(30))]
#[cfg_attr(
    not(any(cmemu_has_rom = "driverlib", cmemu_has_rom = "full")),
    ignore = "No ROM"
)]
fn hello_world() {
    // The *hello-world* of Contiki-NG.
    // Prints "Hello, world" every 10 seconds, enters the Idle sleep.
    // The radio is always on and the default netstack is active.
    let uart_lite = CollectUartLiteBackend::new();
    let buf = uart_lite.get_promise();
    let modem = SharedLogModemInterface::new();
    let modem_log = modem.get_arc();

    let err = run_emulator(
        contiki_examples_path!("cc26x0-cc13x0_hello-world.elf"),
        Timeout::Seconds(30.),
        true,
        |emu| {
            emu.set_uart_lite_interface(Some(Box::new(uart_lite)));
            emu.set_radio_interface(Some(Box::new(modem)));
        },
    )
    .unwrap_err();
    assert_predicate!(
        &err.to_string(),
        predicate::str::contains("Timed out after 30 seconds."),
        "Timeout should be exactly 30 seconds!"
    );

    let stdout = String::from_utf8_lossy(buf.get().unwrap());
    let modem_log = mem::take(&mut *modem_log.lock().unwrap());

    assert_predicate!(&*stdout, predicate::str::contains("Hello, world").count(3));

    // FIXME: modem_log gets pretty large! About an entry per cycle right now, so 10MB+
    // dbg!(modem_log.log.len());
    let in_log_pred = predicate::in_hash(modem_log.log.iter());

    assert_predicate!(
        &ModemRequest::SendOp(ModemOp::SetAutoAck(true)),
        in_log_pred
    );
    assert_predicate!(&ModemRequest::SendOp(RequestCca), in_log_pred);
    assert_predicate!(&ModemRequest::CcaRead, in_log_pred);
    assert_predicate!(&ModemRequest::TakeRx, in_log_pred);
    assert_predicate!(&ModemRequest::TakeTxFinished, in_log_pred);
}
