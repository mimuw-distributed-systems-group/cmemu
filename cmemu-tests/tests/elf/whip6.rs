// Testing simple whip6 apps, located in <playground>/mm319369/whip6_tests
// In particular, `-cmemuexit` variants are partially aware of cmemu (they exit with cmemu addresses),
// which would generate a MemFault on the hardware.

use crate::{CollectUartLiteBackend, Timeout, cmemu_bin_run, run_emulator};
use predicates::prelude::*;
use rstest::rstest;
use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Duration;

macro_rules! whip6_tests_path {
    ($p:expr) => {
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../mm319369/whip6_tests/",
            $p
        )
    };
}

#[test]
#[cfg_attr(
    not(any(cmemu_has_rom = "driverlib", cmemu_has_rom = "full")),
    ignore = "No ROM"
)]
fn minimal() {
    let uart_lite = CollectUartLiteBackend::new();
    let buf = uart_lite.get_promise();
    // A minimal app that has a process that uses UART-Lite for communication and immediately exists.
    let code = run_emulator(
        whip6_tests_path!("MinimalForCmemu/artifacts/minimal-default-cmemuexit.elf"),
        Timeout::Cycles(500_000),
        true,
        |emu| emu.set_uart_lite_interface(Some(Box::new(uart_lite))),
    )
    .unwrap();
    assert_eq!(code, ExitCode::from(42));

    let stdout = String::from_utf8_lossy(buf.get().unwrap());
    assert_predicate!(
        &*stdout,
        predicate::str::contains("This is all I can emulate.")
    );
}

#[rstest]
#[cfg_attr(
    not(any(cmemu_has_rom = "driverlib", cmemu_has_rom = "full")),
    ignore = "No ROM"
)]
#[timeout(Duration::from_secs(30))]
#[should_panic(expected = "not yet implemented: CommandId::StartRat")] // boots and fails radio setup
fn blink_if_see_each_other_prefix() {
    let uart_lite = CollectUartLiteBackend::new();
    // Ping-pong app
    run_emulator(
        whip6_tests_path!("MinimalForCmemu/artifacts/blinkifseeeachother-default.elf"),
        Timeout::Cycles(500_000),
        true,
        |emu| emu.set_uart_lite_interface(Some(Box::new(uart_lite))),
    )
    .unwrap();
}

macro_rules! whip6_examples_path {
    ($p:expr) => {
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../bd370768/example-apps/Whip6/",
            $p
        )
    };
}

#[rstest]
#[case::whip6_startup("whip6_startup")]
#[case::xosc("xosc")]
#[case::led("led")]
#[case::cache("cache")]
#[case::rtc_interrupt("rtc_interrupt")]
#[should_panic(expected = "no wakeup event")] // explicit check right now!
#[case::endless_idle("endless_idle")]
#[case::idle("idle")]
#[should_panic(expected = "no wakeup event")] // explicit check right now!
#[case::endless_standby("endless_standby")]
#[case::standby("standby")]
#[case::standby_xosc("standby_xosc")]
#[case::standby_no_cache_retain("standby_no_cache_retain")]
#[ignore = "implement AON_EVENT::AUXWUSEL, WUC data write 1 for address 0x40091018"]
#[should_panic(expected = "no wakeup event")] // explicit check right now!
#[case::endless_shutdown("endless_shutdown")]
#[timeout(Duration::from_secs(30))] // We need realtime limit for tests which may skip al ot of cycs
#[cfg_attr(
    not(any(cmemu_has_rom = "driverlib", cmemu_has_rom = "full")),
    ignore = "No ROM"
)]
fn all_progs_return_42(#[case] test: &str) {
    let path = PathBuf::from(whip6_examples_path!("x"))
        .with_file_name(format!("cmemu_raw_c-{test}.debug.elf"));
    cmemu_bin_run(path, Timeout::Seconds(0.3), true)
        .assert()
        .code(42);
}
