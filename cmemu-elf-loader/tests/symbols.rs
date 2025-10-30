#![cfg(feature = "builder")]

// Test for integration with libbfd and so...

use assert_cmd::Command;
use cmemu_elf_loader::Symbols;
use object::Object;
use predicates::prelude::*;

/// Check if the OS managed to launch the program
fn naive_which(program: &str) -> bool {
    !Command::new(program)
        .ok()
        .is_err_and(|e| e.as_output().is_none())
}

fn find_nm() -> &'static str {
    ["arm-none-eabi-nm", "nm", "llvm-nm", "rust-nm"]
        .into_iter()
        .find(|&p| naive_which(p))
        .unwrap()
}

#[test]
fn nm_present_on_the_os() {
    find_nm();
}

#[test]
fn produce_elf_for_constants() {
    let ss: Symbols = cc2650_constants::iter_known_registers().collect();
    let tmp = tempfile::NamedTempFile::new().unwrap();
    let path = tmp.path();
    println!("Writing symbols to {}", path.display());

    ss.write_stub_to_file(path).unwrap();

    let nm_prog = find_nm();
    let assert = Command::new(nm_prog).arg("-a").arg(path).assert();
    assert
        .success()
        .stdout(predicate::str::contains("EVENT::RFCSEL0"))
        .stdout(predicate::str::contains("40083100"));
}

#[test]
fn read_write_mandelbrot() {
    let f = std::fs::read(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/test.elf")).unwrap();
    let e = object::read::File::parse(&*f).unwrap();
    let ss: Symbols = e.symbols().collect();

    let tmp = tempfile::NamedTempFile::new().unwrap();
    let path = tmp.path();
    ss.write_stub_to_file(path).unwrap();

    let nm_prog = find_nm();
    let assert = Command::new(nm_prog).arg("-a").arg(path).assert();
    assert
        .success()
        .stdout(predicate::str::contains("__fini_array_end")) // local
        .stdout(predicate::str::contains("__aeabi_f2d")) // hidden
        .stdout(predicate::str::contains("BusFaultISR")) // weak
        .stdout(predicate::str::contains(".bss").not()) // section
        .stdout(predicate::str::contains("$t").not()) // debug
        .stdout(predicate::str::contains("puts.o").not()); // file
}
