fn main() -> anyhow::Result<()> {
    tests::find_rom()?;
    tests::scan_generate_tests()?;
    elf_tests::rebuild_live_tests()
}

//////////////////////////////////////////////
//          Scan & generate tests
//////////////////////////////////////////////
mod tests {
    use anyhow::Context;
    use serde::Deserialize;
    use serde_yaml::Value;
    use std::collections::{HashMap, HashSet};
    use std::env;
    use std::error::Error;
    use std::fmt::{self, Write};
    use std::fs;
    use std::path::{Path, PathBuf, absolute};
    use std::process::Command;

    #[cfg(feature = "soc-cc2652")]
    macro_rules! tests_top_level {
        () => {
            "cc2652"
        };
    }
    #[cfg(all(feature = "soc-stm32f100rbt6", not(feature = "soc-cc2652")))]
    macro_rules! tests_top_level {
        () => {
            "stm32f100rbt6"
        };
    }
    #[cfg(not(any(feature = "soc-cc2652", feature = "soc-stm32f100rbt6")))]
    macro_rules! tests_top_level {
        () => {
            "flash"
        };
    }
    const FLASH_TESTS_DIRECTORY: &str = concat!("tests/", tests_top_level!());
    const FLASH_EXTERNAL_BENCHMARKS_DIRECTORY: &str =
        concat!("tests/", tests_top_level!(), "/benchmarks");
    const FLASH_LARGE_TESTS_DIRECTORY: &str = concat!("tests/", tests_top_level!(), "/large-tests");
    const FLASH_MEMORY_TESTS_DIRECTORY: &str =
        concat!("tests/", tests_top_level!(), "/memory_tests");

    #[derive(Debug)]
    struct BuildError;
    impl Error for BuildError {}
    impl fmt::Display for BuildError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{self:?}")
        }
    }

    #[derive(Deserialize, Debug)]
    struct TestAsm {
        configurations: Vec<Value>,
        // We only care about symbol names [keys],
        // so using Value as a value of HashMap is more general.
        dumped_symbols: HashMap<String, Value>,
    }

    #[derive(Deserialize, Debug)]
    #[serde(deny_unknown_fields)]
    struct TestIgnore {
        reason: String,
        // Maybe this should be in the ASM?
        #[serde(default)]
        need_rom: Option<RomType>,
        ignored: Vec<TestIgnoreEntry>,
    }

    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "lowercase")]
    enum RomType {
        Driverlib,
        Full,
    }

    #[derive(Deserialize, Debug)]
    #[serde(deny_unknown_fields)]
    struct TestIgnoreEntry {
        symbols: Vec<String>,
        configurations: Vec<Value>,
    }

    pub(crate) fn find_rom() -> anyhow::Result<()> {
        // TODO: we have this decoupled from the main logic by a cfg, but maybe we should integrate it clearly
        let out_path = PathBuf::from(env::var("OUT_DIR")?).join("rom.bin");
        let rom_dir = absolute(env::var("CARGO_MANIFEST_DIR")?)?
            .parent()
            .with_context(|| "cannot resolve cargo manifest")?
            .join("rom");
        println!("cargo:rerun-if-changed={}", rom_dir.display());

        println!("cargo:rustc-check-cfg=cfg(cmemu_has_rom, values(\"full\", \"driverlib\"))");
        let full_rom = rom_dir.join("brom.bin");
        let driverlib_rom = rom_dir.join("driverlib.bin");

        if full_rom.exists() {
            println!("cargo:rustc-cfg=cmemu_has_rom=\"full\"");
            println!("cargo:rerun-if-changed={}", full_rom.display());
            fs::copy(full_rom, out_path)?;
        } else if driverlib_rom.exists() {
            println!("cargo:rustc-cfg=cmemu_has_rom=\"driverlib\"");
            println!("cargo:rerun-if-changed={}", driverlib_rom.display());
            fs::copy(driverlib_rom, out_path)?;
        }

        Ok(())
    }

    #[allow(clippy::module_name_repetitions)] // we want to keep current name
    pub(crate) fn scan_generate_tests() -> anyhow::Result<()> {
        let out_dir = env::var("OUT_DIR")?;
        let out_dir = Path::new(&out_dir);
        let mut out = String::new();

        writeln!(out, "#[allow(non_snake_case)]")?;
        writeln!(
            out,
            "#[allow(clippy::pedantic, clippy::redundant_test_prefix)]"
        )?;
        tests_tree_directory(&mut out, FLASH_TESTS_DIRECTORY)?;

        // Write out our auto-generated tests and opportunistically format them with
        // `rustfmt` if it's installed.
        let output = out_dir.join("flash_tests.rs");
        fs::write(&output, out)?;
        drop(Command::new("rustfmt").arg(&output).status());
        Ok(())
    }

    fn tests_tree_directory(out: &mut String, path: impl AsRef<Path>) -> anyhow::Result<()> {
        let path = path.as_ref();
        println!("cargo:rerun-if-changed={}", path.display());
        let mut dir_entries = path
            .read_dir()
            .with_context(|| format!("failed to read {}", path.display()))?
            .map(|r| r.expect("reading testsuite directory entry"))
            .filter_map(|dir_entry| {
                let p = dir_entry.path();

                // Ignore files & dirs starting with `.`, which could be editor temporary files & dirs
                if p.file_stem()?.to_str()?.starts_with('.') {
                    return None;
                }

                // Check if dir
                match dir_entry.metadata() {
                    Ok(metadata) => {
                        if metadata.is_dir() {
                            return Some(Ok((true, p)));
                        }
                    }
                    Err(err) => {
                        return Some(
                            Err(err)
                                .context(format!("failed to get metadata of {}", path.display())),
                        );
                    }
                }

                // Only look at asm files.
                let ext = p.extension()?;
                if ext != "asm" {
                    if ext == "tzst" && !p.with_extension("asm").is_file() {
                        println!(
                            "cargo:warning=Test {} is missing .asm source. Skipping.",
                            p.display()
                        );
                    }
                    return None;
                }

                Some(Ok((false, p)))
            })
            .collect::<Result<Vec<_>, _>>()?;

        if dir_entries.is_empty() {
            println!(
                "cargo:warning=No tests or subdirectories found in {}!",
                path.display()
            );
        }

        // order (lexicographically tests in current dir, then lexicographically subdirectories)
        dir_entries.sort();

        let testsuite = &extract_name(path);
        start_tests_submodule(out, testsuite)?;
        for (is_dir, path) in &dir_entries {
            if *is_dir {
                tests_tree_directory(out, path)?;
            } else {
                write_test_cases(out, path)?;
            }
        }
        finish_tests_submodule(out);
        Ok(())
    }

    fn start_tests_submodule(out: &mut String, testsuite: &str) -> anyhow::Result<()> {
        writeln!(out, "mod {testsuite} {{")?;
        Ok(())
    }

    fn finish_tests_submodule(out: &mut String) {
        out.push_str("}\n\n");
    }

    fn write_test_cases(out: &mut String, path: impl AsRef<Path>) -> anyhow::Result<()> {
        let path = path.as_ref();
        let testname = extract_name(path);
        let archive_path = path.with_extension("tzst");
        let is_external_benchmark = path.starts_with(FLASH_EXTERNAL_BENCHMARKS_DIRECTORY);
        let is_large_test = path.starts_with(FLASH_LARGE_TESTS_DIRECTORY);
        let is_interrupt_trace_test = path.starts_with(FLASH_MEMORY_TESTS_DIRECTORY);

        if is_external_benchmark || is_large_test || is_interrupt_trace_test {
            // XXX: skip large tests for now, since they use product confugurations
            return Ok(());
        } else if !archive_path.is_file() {
            println!(
                "cargo:warning=Test {} is missing .tzst results! Skipping.",
                path.display()
            );
            // Cargo doesn't support appearing files
            // println!("cargo:rerun-if-changed={}", archive_path.display());
            return Ok(());
        }

        let (test_asm, test_ignore) = load_test_description(path)?;

        // helper lambda
        let mut write_single_test_case =
            |i: usize, ignored: bool, checked_symbols: &Vec<String>| -> anyhow::Result<()> {
                assert!(!checked_symbols.is_empty());
                writeln!(out, "#[test]")?;
                if ignored {
                    writeln!(out, "#[ignore]")?;
                } else if is_large_test {
                    writeln!(
                        out,
                        "#[cfg_attr(not(feature = \"include-large-tests\"), ignore)]"
                    )?;
                } else if let Some(TestIgnore {
                    need_rom: Some(rom_type),
                    ..
                }) = &test_ignore
                {
                    writeln!(
                        out,
                        "#[cfg_attr(not({cond}), ignore)]",
                        cond = match rom_type {
                            RomType::Driverlib =>
                                "any(cmemu_has_rom = \"driverlib\", cmemu_has_rom = \"full\")",
                            RomType::Full => "cmemu_has_rom = \"full\"",
                        }
                    )?;
                }
                writeln!(
                    out,
                    "fn r#{}_{}{}() -> anyhow::Result<()> {{",
                    &testname,
                    i,
                    if ignored { "_ignored" } else { "" }
                )?;
                writeln!(
                    out,
                    "let checked_symbols = &[\"{}\"];",
                    checked_symbols.join("\", \""),
                )?;
                writeln!(
                    out,
                    "crate::run_flash_test(r#\"{}\"#, {}, checked_symbols, {:?})",
                    archive_path.display(),
                    i,
                    test_ignore.as_ref().is_some_and(|ti| ti.need_rom.is_some())
                )?;
                writeln!(out, "}}")?;
                writeln!(out)?;
                Ok(())
            };

        for (i, val) in test_asm.configurations.iter().enumerate() {
            let empty_vec = vec![];
            let ignored_symbols =
                get_ignored_symbols(val, test_ignore.as_ref()).unwrap_or(&empty_vec);

            if !ignored_symbols.is_empty() {
                write_single_test_case(i, true, ignored_symbols)?;
            }
            if ignored_symbols.len() < test_asm.dumped_symbols.len() {
                let not_ignored_symbols = test_asm
                    .dumped_symbols
                    .keys()
                    .filter(|v| !ignored_symbols.contains(v))
                    .cloned()
                    .collect();
                write_single_test_case(i, false, &not_ignored_symbols)?;
            }
        }
        Ok(())
    }

    /// Extract a valid Rust identifier from the stem of a path.
    fn extract_name(path: impl AsRef<Path>) -> String {
        path.as_ref()
            .file_stem()
            .expect("filename should have a stem")
            .to_str()
            .expect("filename should be representable as a string")
            .replace(['-', '/'], "_")
    }

    #[allow(clippy::if_then_some_else_none, reason = "false positive")]
    fn load_test_description(
        path: impl AsRef<Path>,
    ) -> anyhow::Result<(TestAsm, Option<TestIgnore>)> {
        let path = path.as_ref();
        let test_asm = load_asm_file(path)?;

        let ignore_path = path.with_extension("ignore");
        let test_ignore = if ignore_path.is_file() {
            Some(load_ignore_file(ignore_path, &test_asm)?)
        } else {
            None
        };

        Ok((test_asm, test_ignore))
    }

    fn load_asm_file(path: impl AsRef<Path>) -> anyhow::Result<TestAsm> {
        let path = path.as_ref();

        println!("cargo:rerun-if-changed={}", path.display()); // revalidate content if changed

        // cut content in two parts, parse the first one
        let test_asm_str = fs::read_to_string(path)?
            .replace("\r\n", "\n")
            .replace('\r', "\n"); // normalize line endings
        let test_asm_str = if let Some(idx) = test_asm_str.find("\n...\n") {
            test_asm_str.split_at(idx).0
        } else {
            return Err(BuildError).context(format!(
                "{}: file doesn't contain \"\\n...\\n\" separator",
                path.display()
            ));
        };
        let test_asm: TestAsm = serde_yaml::from_str(test_asm_str)
            .with_context(|| format!("{}: failed to parse test .asm (as yaml)", path.display()))?;

        // validate test asm
        // XXX: disabled because we use product ???
        // if test_asm.configurations.is_empty() {
        //     return Err(BuildError).context(format!(
        //         "{}: there should be at least one configuration",
        //         path.display()
        //     ));
        // }

        if test_asm.dumped_symbols.is_empty() {
            return Err(BuildError).context(format!(
                "{}: there should be at least one dumped symbol",
                path.display()
            ));
        }

        Ok(test_asm)
    }

    fn load_ignore_file(
        ignore_path: impl AsRef<Path>,
        test_asm: &TestAsm,
    ) -> anyhow::Result<TestIgnore> {
        // assumption: `ignore_path` points to an actual file
        let ignore_path = ignore_path.as_ref();

        println!("cargo:rerun-if-changed={}", ignore_path.display()); // revalidate content if changed
        let test_ignore: TestIgnore = serde_yaml::from_reader(&fs::File::open(ignore_path)?)
            .with_context(|| {
                format!(
                    "{}: failed to parse test .ignore (as yaml)",
                    ignore_path.display()
                )
            })?;

        // validate the ignore file
        if test_ignore.reason.trim() == "" {
            return Err(BuildError).context(format!(
                "{}: ignore file content should contain some reason",
                ignore_path.display()
            ));
        }

        let mut already_ignored_configurations = HashSet::new();

        for entry in &test_ignore.ignored {
            // symbols
            if entry.symbols.is_empty() {
                return Err(BuildError).context(format!(
                    "{}: ignored entry with no symbols",
                    ignore_path.display(),
                ));
            }
            for (i, sym) in entry.symbols.iter().enumerate() {
                if !test_asm.dumped_symbols.contains_key(sym) {
                    return Err(BuildError).context(format!(
                        "{}: unknown symbol {}",
                        ignore_path.display(),
                        sym,
                    ));
                }

                if i > 0 && entry.symbols[..i - 1].contains(sym) {
                    return Err(BuildError).context(format!(
                        "{}: duplicate symbol {}",
                        ignore_path.display(),
                        sym,
                    ));
                }
            }

            // configurations
            if entry.configurations.is_empty() {
                return Err(BuildError).context(format!(
                    "{}: ignored entry with no configurations",
                    ignore_path.display(),
                ));
            }
            for val in &entry.configurations {
                if !test_asm.configurations.contains(val) {
                    return Err(BuildError).context(format!(
                        "{}: unknown configuration {}",
                        ignore_path.display(),
                        serde_yaml::to_string(val)
                            .expect("failed to serialize deserialized data (it should be correct)"),
                    ));
                }

                if !already_ignored_configurations.insert(val) {
                    return Err(BuildError).context(format!(
                        "{}: duplicate configuration {}",
                        ignore_path.display(),
                        serde_yaml::to_string(val)
                            .expect("failed to serialize deserialized data (it should be correct)"),
                    ));
                }
            }
        }

        Ok(test_ignore)
    }

    fn get_ignored_symbols<'a>(
        value: &Value,
        test_ignore: Option<&'a TestIgnore>,
    ) -> Option<&'a Vec<String>> {
        if let Some(test_ignore) = &test_ignore {
            for entry in &test_ignore.ignored {
                if entry.configurations.contains(value) {
                    return Some(&entry.symbols);
                }
            }
            None
        } else {
            None
        }
    }
}

mod elf_tests {
    use anyhow::Context;
    use std::cmp::max;
    use std::env;
    use std::ffi::OsStr;
    use std::fs::DirEntry;
    use std::path::{Path, PathBuf};
    use std::process::Command;
    use std::time::SystemTime;

    const ELF_TESTS_DIRECTORY: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/elf/");
    const LIVE_TESTS_ENV_NAME: &str = "CMEMU__LIVE_TESTS";
    const LIVE_TESTS_MINI_DIR: &str = "bugs_mini_tests";

    // NOTE: This is alsmost a copy-paste from cmemu-elf-loader build.rs!
    pub(crate) fn rebuild_live_tests() -> anyhow::Result<()> {
        let this_dir = env::var_os("CARGO_MANIFEST_DIR").unwrap();
        let glue_dir = Path::new(&this_dir).join("../cmemu-elf-loader/cmemu-target");
        let live_dir = PathBuf::from(ELF_TESTS_DIRECTORY).join("hosted");
        let mut newest_mtime = SystemTime::UNIX_EPOCH;
        let mut need_to_run_make = false;

        println!("cargo:rerun-if-env-changed={LIVE_TESTS_ENV_NAME}");
        if !env::var(LIVE_TESTS_ENV_NAME).is_ok_and(|s| s == "true" || s == "1") {
            return Ok(());
        }

        eprintln!("Last generated {:?}", SystemTime::now());

        // cmemu-target
        for entry in
            walk_with_extensions(&glue_dir, &["c", "s", "S", "lds", "base"].map(OsStr::new))?
        {
            println!("cargo:rerun-if-changed={}", entry.path().display());
            newest_mtime = max(newest_mtime, mtime_de(&entry)?);
        }

        // live tests - check if any elf is out of date
        let live_makefile = live_dir.join("Makefile");
        newest_mtime = max(newest_mtime, mtime(&live_makefile)?);
        println!("cargo:rerun-if-changed={}", live_makefile.display());

        // TODO: we may miss a change to a dependency / header, but over-rebuilding will rebuild the whole crate!
        // TODO: we need to walk recursively?
        let src_extensions = &["c", "s", "S"].map(OsStr::new);
        let mini_tests = live_dir.join(LIVE_TESTS_MINI_DIR);
        let source_files = walk_with_extensions(&live_dir, src_extensions)?
            .chain(walk_with_extensions(&mini_tests, src_extensions)?);
        for entry in source_files {
            println!("cargo:rerun-if-changed={}", entry.path().display());
            let elf_path = entry.path().with_extension("elf");
            need_to_run_make |=
                elf_path.exists() && mtime(&elf_path)? < max(newest_mtime, mtime_de(&entry)?);
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
                assert!(status.success(), "Make failed {status:?}.");
            }
        }
        Ok(())
    }

    fn mtime_de(entry: &DirEntry) -> anyhow::Result<SystemTime> {
        Ok(entry
            .metadata()
            .with_context(|| format!("cannot get metadata for {entry:?}"))?
            .modified()?)
    }

    fn mtime(path: &Path) -> anyhow::Result<SystemTime> {
        Ok(path
            .metadata()
            .with_context(|| format!("cannot get metadata for {}", path.display()))?
            .modified()?)
    }

    fn walk_with_extensions<'a>(
        path: &'a Path,
        extensions: &'a [&OsStr],
    ) -> anyhow::Result<impl Iterator<Item = DirEntry>> {
        Ok(path
            .read_dir()
            .with_context(|| format!("failed to walk {}", path.display()))?
            .flatten()
            .filter(|entry| {
                entry
                    .path()
                    .extension()
                    .is_some_and(|x| extensions.contains(&x))
                    && entry.file_type().unwrap().is_file()
            }))
    }
}
