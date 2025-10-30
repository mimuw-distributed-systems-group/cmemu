use anyhow::{Context, bail};
use clap::Parser;
use cmemu_flash_test_lib::{FlashTestCase, MemoryFormat, TestRunner};
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

#[derive(Parser)]
#[command(
    version,
    about = "Cherry Mote Emulator flash test executor",
    arg_required_else_help = true
)]
struct App {
    /// cmemu-tests/tests/path/to/archive.tzst.{test_case} (recommended)
    ///
    /// or `cmemu-tests/tests/path/to/archive.zip.{test_case}` (good for large archives)
    /// or `cmemu-tests/tests/path/to/unpacked_archive/{test_case}` (fastest)
    /// or `path::to::archive_{test_case}` (might infer wrong file name; tests location was evaluated in compile time)
    #[arg(verbatim_doc_comment)]
    test_path: String,

    #[arg(long, alias("rom"), env = "CMEMU_TEST_ROM_FILE")]
    /// ROM memory contents, zeroed if unspecified
    pub rom_file: Option<PathBuf>,

    #[arg(long, alias("cdl-file"))]
    #[cfg_attr(not(feature = "cycle-debug-logger"), arg(value_parser = reject_missing_cdl_support))]
    /// Output json file with cycle debug information; to be loaded with cdl-viewer
    ///
    /// Supports compression (use `.gz` extension) and outputting multiple parts into a directory
    /// (use `.d` extension).
    cycle_debug_log_file: Option<PathBuf>,

    #[arg(long)]
    /// don't skip startup sequence in CDL log file
    cycle_debug_full_log: bool,

    #[arg(long)]
    /// don't validate test integrity
    skip_integrity_check: bool,

    #[arg(long)]
    /// return parsable YAML on stdout
    machine_readable: bool,

    #[arg(long)]
    /// show full dump information
    show_dump: bool,

    #[arg(long, env = "CMEMU_TEST_MEMORY_FORMAT")]
    /// specifies shown memory format
    memory_format: Option<MemoryFormat>,

    #[arg(long, env = "CMEMU_TEST_CYCLES_TIMEOUT")]
    /// overwrites cycles timeout (protection against looping tests)
    cycles_timeout: Option<u64>,

    #[cfg(feature = "gdb")]
    #[command(flatten, next_help_heading = "Gdb options")]
    pub gdb_params: cmemu_gdb::GdbArgs,
}

fn main() -> anyhow::Result<ExitCode> {
    #[allow(unused)]
    let logger = flexi_logger::Logger::try_with_env()?
        .adaptive_format_for_stderr(pretty_flexi_logger::ADAPTIVE_PRETTY_FORMAT)
        .start()?;

    let args = App::parse();

    let mut test = if args.skip_integrity_check {
        // Note: The closure is needed to work around monomorphisation limitations
        load_test_case(&args.test_path, |p, n| {
            FlashTestCase::load_from_test_file_and_case_without_integrity_check(p, n)
        })?
    } else {
        load_test_case(&args.test_path, |p, n| {
            FlashTestCase::load_from_test_file_and_case(p, n)
        })?
    };

    if let Some(f) = args.rom_file {
        match fs::read(f) {
            Ok(mem) => test.set_rom(Some(mem)),
            Err(err) => {
                bail!("Failed to load rom memory file: {}", err)
            }
        }
    }

    #[cfg(feature = "gdb")]
    if args.gdb_params.wants_to_manage_loop() {
        println!("Running under GDB:\n{}", test.display_metadata());

        let mut gdb_params = args.gdb_params;
        gdb_params.flash_file = Some(test.display_test_path().to_string().into());
        let (mut emulator, dump) = test.deconstruct_configured()?;
        #[cfg(feature = "cycle-debug-logger")]
        {
            emulator.set_cycle_debug_logger_file(args.cycle_debug_log_file);
            if args.cycle_debug_full_log {
                emulator.cycle_debug_logger_start_recording();
            }
        }
        return gdb_params
            .run_emulator_for_flash_test(emulator, args.cycles_timeout, dump, Some(logger))
            .map_err(|e| anyhow::anyhow!(e.to_string()));
        // we need to lose all info as the error is not Sync & Send
    }

    // configure the test runner
    if let Some(mem_fmt) = args.memory_format {
        test.set_memory_format(mem_fmt);
    }
    let use_yaml = args.machine_readable;
    test.set_machine_readable(use_yaml);
    let mut test_runner = test.get_new_test_runner()?;
    if let Some(cycles_timeout) = args.cycles_timeout {
        test_runner.set_cycles_timeout(cycles_timeout);
    }
    #[cfg(feature = "cycle-debug-logger")]
    test_runner.configure_cycle_debug_logger(args.cycle_debug_log_file, !args.cycle_debug_full_log);

    if use_yaml {
        println!("---");
    }
    // run the test
    println!("{}", test.display_metadata());
    if args.show_dump {
        println!("{}", test.display_dump()?);
    }
    test_runner.run().map_or_else(
        |r| {
            if use_yaml {
                println!("status:   error");
                println!("reason: |\n {}", r.to_string().replace('\n', "\n  "));
            }
            Err(r)
        },
        |()| {
            if use_yaml {
                println!("status:   ok");
            } else {
                println!("ok");
            }
            Ok(())
        },
    )?;

    if use_yaml {
        println!("...");
    }
    Ok(ExitCode::SUCCESS)
}

fn load_test_case(
    test_path: &str,
    test_loader: fn(&Path, u32) -> anyhow::Result<FlashTestCase>,
) -> anyhow::Result<FlashTestCase> {
    let test = if test_path.contains('.') {
        // parse known formats:
        //   path/to/archive.tzst.{test_case}
        //   path/to/archive.zip.{test_case}
        //   path/to/unpacked_archive/{test_case}(.flash)?
        let test_file_path = Path::new(test_path);
        if let Some(test_case) = test_file_path
            .extension()
            .and_then(OsStr::to_str)
            .and_then(|e| e.parse().ok())
        {
            test_loader(test_file_path.with_extension("").as_ref(), test_case)?
        } else if let Some(test_case) = test_file_path
            .file_stem()
            .and_then(OsStr::to_str)
            .and_then(|e| e.parse().ok())
        {
            test_loader(test_file_path.parent().unwrap(), test_case)?
        } else {
            bail!("test case must be a number (u32), got: {test_path:?}",)
        }
    } else {
        // parse: path::to::archive_{test_case}
        // note: doesn't work for names altered by build script, i.e. paths containing "-";
        //       we can ignore this detail, forbid such names or build an index that pairs file name with test name
        let test_file_path = test_path.replace("::", "/");
        let split_point = test_file_path
            .rfind('_')
            .context("missing `_{test_case}' in the path")?;
        let test_case = test_file_path[split_point + 1..].parse().with_context(|| {
            format!(
                "test case must be a number (u32), got: \"{}\"",
                &test_file_path[split_point + 1..]
            )
        })?;
        let test_file_path = format!(
            "{}/../cmemu-tests/tests/{}.tzst",
            env!("CARGO_MANIFEST_DIR"),
            &test_file_path[..split_point]
        );
        test_loader(test_file_path.as_ref(), test_case)?
    };
    Ok(test)
}

#[cfg(not(feature = "cycle-debug-logger"))]
fn reject_missing_cdl_support(_: &str) -> Result<PathBuf, String> {
    Err("emulator was built without Cycle Debug Logger support".to_string())
}

#[cfg(test)]
mod test_clap {
    use crate::App;

    #[test]
    fn app_clap_assert() {
        <App as clap::CommandFactory>::command().debug_assert();
    }
}
