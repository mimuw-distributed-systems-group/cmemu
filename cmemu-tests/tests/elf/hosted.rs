use predicates::prelude::*;

use crate::{Timeout, cmemu_bin_run};

#[test]
fn minimal() {
    let assert = cmemu_bin_run(test_path!("hosted/minimal.elf"), Timeout::Default, false).assert();
    assert.failure().code(42);
}

#[test]
fn asm_complex_hosting() {
    cmemu_bin_run(
        test_path!("hosted/asm_complex_hosting.elf"),
        Timeout::Default,
        false,
    )
    .assert()
    .failure()
    .code(42)
    .stdout(predicate::str::starts_with(
        "Hello from asm! This is a long message!",
    ));
}

#[test]
fn asm_overriden_entry() {
    let assert = cmemu_bin_run(
        test_path!("hosted/asm_complex_hosting.elf"),
        Timeout::Default,
        false,
    )
    .args(["--entrypoint", "_exit"])
    .assert();
    assert.failure().code(42).stdout(predicate::str::is_empty());
}

#[test]
fn baseline() {
    let assert = cmemu_bin_run(
        test_path!("hosted/test_syscalls_io.elf"),
        Timeout::Cycles(20_000),
        false,
    )
    .write_stdin("1337")
    .assert();
    assert.success().stdout(predicate::str::diff(
        "What is your age?\nYour age is 1337\n",
    ));
}

#[test]
fn syscalls() {
    let assert = cmemu_bin_run(
        test_path!("hosted/test_syscalls.elf"),
        Timeout::Cycles(50_000),
        false,
    )
    .assert();
    assert
        .success()
        .stdout(predicate::str::contains("Current time:"))
        .stderr(predicate::str::contains("Argc: 0"));
}

#[test]
fn arguments_passing() {
    let hosted_arguments = ["arg1", "another argument"];
    let assert = cmemu_bin_run(
        test_path!("hosted/test_syscalls.elf"),
        Timeout::Cycles(100_000),
        false,
    )
    .args(["-E", "LC_ALL=LANG=en_US.UTF-8", "--arg0", "test_prog", "--"])
    .args(hosted_arguments)
    .assert();
    assert
        .success()
        .stderr(predicate::str::contains("Argc: 3"))
        .stderr(predicate::str::contains("Usage: test_prog ["))
        // no splitting
        .stderr(predicate::str::contains(": another argument"))
        .stderr(predicate::str::contains("Locale: LANG=en_US.UTF-8"))
        .stdout(predicate::str::contains("Current time:"));
}

#[test]
fn panic() {
    cmemu_bin_run(
        test_path!("hosted/panic.elf"),
        Timeout::Cycles(10_000),
        false,
    )
    .assert()
    .failure()
    .stderr(predicate::str::contains("panicked at"));
}

#[test]
fn umull_mla_bug() {
    cmemu_bin_run(
        test_path!("hosted/umull_mla_bug.elf"),
        Timeout::Cycles(10_000),
        false,
    )
    .assert()
    .failure()
    .stderr(predicate::str::contains("panicked at"))
    .stderr(predicate::str::contains("xMULL"));
}

#[test]
fn mandelbrot() {
    cmemu_bin_run(
        test_path!("hosted/mandelbrot.elf"),
        Timeout::Cycles(12_345),
        false,
    )
    .assert()
    .failure()
    .stdout(predicate::str::contains(".."))
    .stderr(predicate::str::contains("Timed out after 12345 cycles."));
}

#[test]
fn contiki_aes() {
    // random bigger test executed till the end
    cmemu_bin_run(
        test_path!("hosted/contiki-aes.elf"),
        Timeout::Cycles(100_000),
        false,
    )
    .assert()
    .success()
    .stderr(predicate::str::is_empty())
    .stdout(predicate::str::starts_with("Buf is: 4829ce66"));
}

#[test]
fn min_max_example_from_paper() {
    cmemu_bin_run(
        test_path!("hosted/min_max_example_from_paper.elf"),
        Timeout::Cycles(111),
        false,
    )
    .assert()
    .failure()
    .code(42)
    .stderr(predicate::str::is_empty())
    .stdout(predicate::str::is_empty());
}

// Those should be auto-generated TBH
mod bugs {
    use crate::{Timeout, cmemu_bin_run};
    use rstest::rstest;
    use std::path::PathBuf;

    #[rstest]
    fn all_progs_return_42(
        #[files("*.elf")]
        #[base_dir = "tests/elf/hosted/bugs_mini_tests/"]
        path: PathBuf,
    ) {
        cmemu_bin_run(path, Timeout::Default, false)
            .assert()
            .code(42);
    }
}
