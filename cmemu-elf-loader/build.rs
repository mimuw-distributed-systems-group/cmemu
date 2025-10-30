// Runs make in live_playground/ if it is out of date.
// This probably should live somewhere else, but I don't know how we could
// automatically trigger the rebuild on `cargo run` otherwise.
// FIXME: this triggers cmemu binary rebuilding even if only playground files changed!

use std::cmp::max;
use std::env;
use std::ffi::OsStr;
use std::fs::DirEntry;
use std::path::{Path, absolute};
use std::process::Command;
use std::time::SystemTime;

fn main() {
    // For tests
    emit_drivelib_rom();
    rebuild_live_playground();
}

fn emit_drivelib_rom() {
    let rom_dir = absolute(env::var("CARGO_MANIFEST_DIR").unwrap())
        .unwrap()
        .parent()
        .unwrap()
        .join("rom");
    println!("cargo:rerun-if-changed={}", rom_dir.display());

    println!("cargo:rustc-check-cfg=cfg(cmemu_has_rom, values(\"full\", \"driverlib\"))");
    let driverlib_rom = rom_dir.join("driverlib.elf");

    if driverlib_rom.exists() {
        println!("cargo:rustc-cfg=cmemu_has_rom=\"driverlib\"");
        println!("cargo:rerun-if-changed={}", driverlib_rom.display());
        println!("cargo:rustc-env=DRIVERLIB_PATH={}", driverlib_rom.display());
    }
}

fn rebuild_live_playground() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let this_dir = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let glue_dir = Path::new(&this_dir).join("cmemu-target");
    let live_dir = Path::new(&this_dir).join("live_playground");
    let mut newest_mtime = SystemTime::UNIX_EPOCH;
    let mut need_to_run_make = false;

    println!("cargo:rerun-if-changed=build.rs");
    eprintln!("Last generated {:?}", SystemTime::now());

    // cmemu-target
    for entry in walk_with_extensions(&glue_dir, &["c", "s", "S", "lds", "base"].map(OsStr::new)) {
        println!("cargo:rerun-if-changed={}", entry.path().display());
        newest_mtime = max(newest_mtime, entry.metadata().unwrap().modified().unwrap());
    }

    // live playground - check if any elf is out of date
    let live_makefile = live_dir.join("Makefile");
    newest_mtime = max(
        newest_mtime,
        live_makefile.metadata().unwrap().modified().unwrap(),
    );
    println!("cargo:rerun-if-changed={}", live_makefile.display());

    for entry in walk_with_extensions(&live_dir, &["c", "s", "S"].map(OsStr::new)) {
        println!("cargo:rerun-if-changed={}", entry.path().display());
        let elf_path = entry.path().with_extension("elf");
        need_to_run_make |= elf_path.exists()
            && elf_path.metadata().unwrap().modified().unwrap()
                < max(newest_mtime, entry.metadata().unwrap().modified().unwrap());
    }

    if need_to_run_make {
        // Naive check for the compiler
        if let Err(e) = Command::new("arm-none-eabi-gcc")
            .arg("-print-sysroot")
            .status()
        {
            println!(
                "cargo:warning=Missing 'arm-none-eabi-gcc' compiler. Make will not run. Code: {e:?}"
            );
        } else if let Ok(status) = Command::new("make")
            .current_dir(live_dir)
            .status()
            .map_err(|e| println!("cargo:warning=Failed to start make {e:?}"))
        {
            eprintln!("Make status {status:?}");
            assert!(
                status.success(),
                "Make failed {status:?}. Look next to {}",
                out_dir.display()
            );
        }
    }
}

fn walk_with_extensions<'a>(
    path: &'a Path,
    extensions: &'a [&OsStr],
) -> impl Iterator<Item = DirEntry> {
    path.read_dir()
        .expect("wrong dir?")
        .flatten()
        .filter(|entry| {
            entry
                .path()
                .extension()
                .is_some_and(|x| extensions.contains(&x))
                && entry.file_type().unwrap().is_file()
        })
}
