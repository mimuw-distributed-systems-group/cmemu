use anyhow::{Context, bail, ensure};
use clap::ValueEnum;
use cmemu_lib::common::Address;
use cmemu_lib::engine::Emulator;
use rmp_serde as rmps;
use serde::{Deserialize, Deserializer};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::default::Default;
use std::error::Error;
use std::ffi::OsStr;
use std::fmt::{self, Display, Write};
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

/// Determines whether cmemu-lib was built with Cycle Debug Logger.
/// Cargo might build dependency with a feature even though it is not requested.
pub use cmemu_lib::BUILT_WITH_CYCLE_DEBUG_LOGGER;

pub trait TestRunner<'a> {
    #[cfg(feature = "cycle-debug-logger")]
    fn configure_cycle_debug_logger(
        &mut self,
        log_file: Option<impl AsRef<Path>>,
        skip_startup: bool,
    );
    /// Configures which symbols are checked. Set to `None` (remove the mask), to check all symbols.
    fn configure_checked_symbols_mask(&mut self, new_checked_symbols_mask: Option<&'a [&'a str]>);
    fn set_cycles_timeout(&mut self, cycles_timeout: u64);
    /// Sets a callback to be run at certain moments defined by [`CallbackType`].
    fn set_callback(&mut self, callback: Option<CallbackPtr<'a>>);
    // Consume the runner: allow for just a single run.
    /// # Errors
    /// When cycles time out (too many cycles), or internal error (unexpected end of emulation).
    fn run(self) -> anyhow::Result<()>;
}

pub trait BenchmarkRunner {
    fn set_cycles_timeout(&mut self, cycles_timeout: u64);

    #[cfg(feature = "cycle-debug-logger")]
    fn start_cdl_recording(&mut self);

    // Take self by ref, not to measure emulator drop (destructor).
    /// Can be called before `execute_test`, but must *NOT* be called after it.
    /// Executes `emulator_main` prolog: initialization of memory.
    ///
    /// # Errors
    /// When cycles time out (too many cycles), or internal error (unexpected end of emulation).
    fn execute_init(&mut self) -> anyhow::Result<u64>;

    // Take self by ref, not to measure emulator drop (destructor).
    /// Executes test until its end including initialization, unless
    /// the initialization was run separately with `execute_init`.
    ///
    /// # Errors
    /// When cycles time out (too many cycles), or internal error (unexpected end of emulation).
    fn execute_test(&mut self) -> anyhow::Result<u64>;
}

/// Instantiated `FlashTestCase`.
/// Can be treated either as `TestRunner` XOR `BenchmarkRunner`
/// (that is what public API is enforcing).
struct Runner<'a> {
    emulator: Emulator,
    flash_test_case: &'a FlashTestCase,

    // configuration
    #[cfg(feature = "cycle-debug-logger")]
    cdl_file: Option<PathBuf>,
    #[cfg(feature = "cycle-debug-logger")]
    cdl_skip_startup: bool,
    checked_symbols_mask: Option<&'a [&'a str]>,
    cycles_timeout: u64,
    callback: Option<CallbackPtr<'a>>,
}

// Note: possible ideas for the callback API (if needed):
// - `CallbackType::OnStepCycle` comes with `cycle_number`.
// - Callback returns `CallbackAction { Continue, Stop(Err) }`.

pub type CallbackPtr<'a> = &'a mut dyn FnMut(&mut Emulator, CallbackType);

/// When callback is called.
#[derive(Debug, Copy, Clone)]
#[allow(clippy::enum_variant_names)] // Prefix `On` is intended and desired.
#[non_exhaustive]
pub enum CallbackType {
    /// Run on initialization.
    OnInit,
    /// Run before each call to [`Emulator::step_cycle`].
    OnStepCycle,
    /// Run right after the test is done, before checking memory represented by symbols.
    OnFinish,
}

/// Represents a single "configuration"/test case from a `.tzst` archive.
#[derive(Debug)]
pub struct FlashTestCase {
    flash: Vec<u8>,
    dump: TestDump,

    rom: Option<Vec<u8>>,
    // configuration
    mem_fmt: MemoryFormat,
    output_yaml: bool,

    // metadata
    test_path: PathBuf,
    test_case: u32,
}

// Assumption: emulator_cdl_start_addr is visited before emulator_exit_addr.
/// Parsed dump of a single test case.
///
/// Note: the format might change.
#[derive(Deserialize, Debug)]
// #[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct TestDump {
    #[serde(deserialize_with = "deserialize_address")]
    pub emulator_main_addr: Address,
    #[serde(deserialize_with = "deserialize_address")]
    pub emulator_cdl_start_addr: Address,
    #[serde(deserialize_with = "deserialize_address")]
    pub emulator_exit_addr: Address,
    #[serde(with = "serde_bytes")]
    pub flash_sha256: Vec<u8>,
    #[serde(with = "serde_bytes")]
    pub asm_sha256: Vec<u8>,
    pub configuration_name: String,
    #[serde(deserialize_with = "deserialize_timestamp")]
    pub generation_time: SystemTime,
    pub mem_dump: Vec<TestDumpMemoryChunk>,
    #[serde(default, deserialize_with = "deserialize_symbols")]
    pub symbols: Option<HashMap<String, Address>>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct TestDumpMemoryChunk {
    pub symbol_name: String,
    #[serde(deserialize_with = "deserialize_address")]
    pub addr: Address,
    #[serde(with = "serde_bytes")]
    pub content: Vec<u8>,
}

fn deserialize_address<'de, D>(deserializer: D) -> Result<Address, D::Error>
where
    D: Deserializer<'de>,
{
    let addr = u32::deserialize(deserializer)?;
    Ok(Address::from_const(addr))
}

fn deserialize_symbols<'de, D>(
    deserializer: D,
) -> Result<Option<HashMap<String, Address>>, D::Error>
where
    D: Deserializer<'de>,
{
    let native = Option::<HashMap<String, u32>>::deserialize(deserializer)?;
    Ok(native.map(|n| {
        n.into_iter()
            .map(|(k, v)| (k, Address::from_const(v)))
            .collect()
    }))
}

fn deserialize_timestamp<'de, D>(deserializer: D) -> Result<SystemTime, D::Error>
where
    D: Deserializer<'de>,
{
    let ms_since_epoch = u64::deserialize(deserializer)?;
    Ok(SystemTime::UNIX_EPOCH + Duration::from_millis(ms_since_epoch))
}

#[derive(Debug)]
#[non_exhaustive]
pub enum TestError {
    TimedOut(u64),
    InvalidInput,
    MismatchedOutput,
}
impl Error for TestError {}
impl Display for TestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

//---------------------------------------------------
// Runner
//---------------------------------------------------

// Constructor
impl<'a> TryFrom<&'a FlashTestCase> for Runner<'a> {
    type Error = anyhow::Error;

    fn try_from(flash_test_case: &'a FlashTestCase) -> anyhow::Result<Self> {
        // construct emulator
        let mut emulator = Emulator::new(&flash_test_case.flash, flash_test_case.rom.as_deref());
        emulator.set_nonstandard_entrypoint(Some(flash_test_case.dump.emulator_main_addr));

        emulator.prepare_for_flash_test();

        Ok(Runner {
            emulator,
            flash_test_case,

            #[cfg(feature = "cycle-debug-logger")]
            cdl_file: None,
            #[cfg(feature = "cycle-debug-logger")]
            cdl_skip_startup: true,
            checked_symbols_mask: None,
            cycles_timeout: Self::DEFAULT_TEST_CYCLES_TIMEOUT,
            callback: None,
        })
    }
}

// Test runner
impl<'a> TestRunner<'a> for Runner<'a> {
    #[cfg(feature = "cycle-debug-logger")]
    fn configure_cycle_debug_logger(
        &mut self,
        log_file: Option<impl AsRef<Path>>,
        skip_startup: bool,
    ) {
        self.cdl_file = log_file.map(|p| p.as_ref().to_path_buf());
        self.cdl_skip_startup = skip_startup;
    }

    // (see comments & docs on the trait definition)
    fn configure_checked_symbols_mask(&mut self, new_checked_symbols_mask: Option<&'a [&'a str]>) {
        self.checked_symbols_mask = new_checked_symbols_mask;
    }

    fn set_cycles_timeout(&mut self, cycles_timeout: u64) {
        self.cycles_timeout = cycles_timeout;
    }

    fn set_callback(&mut self, callback: Option<CallbackPtr<'a>>) {
        self.callback = callback;
    }

    // (see comments & docs on the trait definition)
    fn run(mut self) -> anyhow::Result<()> {
        if let Some(cb) = &mut self.callback {
            cb(&mut self.emulator, CallbackType::OnInit);
        }

        #[cfg(feature = "cycle-debug-logger")]
        if self.cdl_file.is_some() {
            self.emulator
                .set_cycle_debug_logger_file(self.cdl_file.as_ref());
            self.set_cdl_custom_metadata();

            if self.cdl_skip_startup {
                let cycles_executed = self.step_until_execution_at_address_with_timeout(
                    self.flash_test_case.dump.emulator_cdl_start_addr,
                    self.cycles_timeout,
                )?;
                self.cycles_timeout -= cycles_executed;
            }
            self.emulator.cycle_debug_logger_start_recording();
        }

        let _ = self.step_until_execution_at_address_with_timeout(
            self.flash_test_case.dump.emulator_exit_addr,
            self.cycles_timeout,
        )?;

        if let Some(cb) = &mut self.callback {
            cb(&mut self.emulator, CallbackType::OnFinish);
        }

        self.compare_memory()
    }
}

// Test runner helper
impl Runner<'_> {
    #[cfg(feature = "cycle-debug-logger")]
    /// Sends test case metadata to CDL for inclusion in the debug log.
    /// The metadata used is the same as in [`FlashTestCase::display_metadata`].
    fn set_cdl_custom_metadata(&mut self) {
        self.emulator.set_cycle_debug_logger_metadata(
            "Test file",
            self.flash_test_case.test_path.display().to_string(),
        );
        self.emulator.set_cycle_debug_logger_metadata(
            "Test case",
            self.flash_test_case.test_case.to_string(),
        );
        self.emulator.set_cycle_debug_logger_metadata(
            "Configuration",
            self.flash_test_case.dump.configuration_name.to_string(),
        );
        self.emulator.set_cycle_debug_logger_metadata(
            "Generation time",
            self.flash_test_case.display_generation_time().to_string(),
        );
    }

    fn compare_memory(&mut self) -> anyhow::Result<()> {
        let mut buf = vec![];
        let mut err_msg = "Mismatched memory:\n".to_owned();
        let mut mismatch = false;

        if self.flash_test_case.output_yaml {
            println!("results:");
        }

        for mem_chunk in &self.flash_test_case.dump.mem_dump {
            if self
                .checked_symbols_mask
                .is_some_and(|m| !m.contains(&mem_chunk.symbol_name.as_str()))
            {
                continue;
            }

            buf.resize(mem_chunk.content.len(), 0);
            self.emulator
                .read_memory(mem_chunk.addr, &mut buf[..])
                .with_context(|| {
                    format!("Failed to read memory for symbol {}", mem_chunk.symbol_name)
                })?;

            if self.flash_test_case.output_yaml {
                println!(
                    "-\n  symbol: {}\n  matches: {}\n  expected: {}\n  got: {}",
                    mem_chunk.symbol_name,
                    buf == mem_chunk.content,
                    self.flash_test_case
                        .mem_fmt
                        .display(&mem_chunk.content, true),
                    self.flash_test_case.mem_fmt.display(&buf, true)
                );
            }
            if buf != mem_chunk.content {
                mismatch = true;
                write!(
                    err_msg,
                    "---\n\
                     symbol:   {}\n\
                     expected: {}\n\
                     got:      {}\n",
                    mem_chunk.symbol_name,
                    self.flash_test_case
                        .mem_fmt
                        .display(&mem_chunk.content, true),
                    self.flash_test_case.mem_fmt.display(&buf, true)
                )?;
            }
        }

        if mismatch {
            Err(TestError::MismatchedOutput).context(err_msg)
        } else {
            Ok(())
        }
    }
}

// Benchmark runner
impl BenchmarkRunner for Runner<'_> {
    fn set_cycles_timeout(&mut self, cycles_timeout: u64) {
        self.cycles_timeout = cycles_timeout;
    }

    #[cfg(feature = "cycle-debug-logger")]
    fn start_cdl_recording(&mut self) {
        self.emulator.cycle_debug_logger_start_recording();
    }

    // (see comments & docs on the trait definition)
    fn execute_init(&mut self) -> anyhow::Result<u64> {
        let cycles_executed = self.step_until_execution_at_address_with_timeout(
            self.flash_test_case.dump.emulator_cdl_start_addr,
            self.cycles_timeout,
        )?;
        self.cycles_timeout -= cycles_executed;
        Ok(cycles_executed)
    }

    // (see comments & docs on the trait definition)
    fn execute_test(&mut self) -> anyhow::Result<u64> {
        let cycles_executed = self.step_until_execution_at_address_with_timeout(
            self.flash_test_case.dump.emulator_exit_addr,
            self.cycles_timeout,
        )?;
        self.cycles_timeout -= cycles_executed;
        Ok(cycles_executed)
    }
}

// Auxiliary impl
impl Runner<'_> {
    const DEFAULT_TEST_CYCLES_TIMEOUT: u64 = 10_000_000;

    #[inline]
    fn is_currently_executing_at(&self, addr: Address) -> bool {
        self.emulator.get_current_instruction_address() == addr
    }

    fn step_until_execution_at_address_with_timeout(
        &mut self,
        stop_point: Address,
        cycles_timeout: u64,
    ) -> anyhow::Result<u64> {
        for i in 0..cycles_timeout {
            if self.is_currently_executing_at(stop_point) {
                return Ok(i);
            }

            if let Some(cb) = &mut self.callback {
                cb(&mut self.emulator, CallbackType::OnStepCycle);
            }

            self.emulator.step_cycle();
        }

        if self.is_currently_executing_at(stop_point) {
            Ok(cycles_timeout)
        } else {
            Err(TestError::TimedOut(cycles_timeout)).context("cycles timed out")
        }
    }
}

//---------------------------------------------------
// Flash test case
//---------------------------------------------------

impl FlashTestCase {
    /// Loads specific flash test case.
    ///
    /// # Errors
    /// Archive or specific test within it cannot be found, or integrity check fails.
    pub fn load_from_test_file_and_case(
        test_archive_path: impl AsRef<Path>,
        test_case: u32,
    ) -> anyhow::Result<Self> {
        let test = Self::load_from_test_file_and_case_without_integrity_check(
            test_archive_path,
            test_case,
        )?;
        test.integrity_check()?;
        Ok(test)
    }

    /// **Use only when you know what you're doing.***
    /// `load_from_test_file_and_case` is preferred and should be enough
    /// most of the time.
    ///
    /// This function doesn't perform integrity check, so potentially you
    /// could be running outdated, mismatched or malformed test.
    ///
    /// # Errors
    /// Archive or specific test within it cannot be found.
    pub fn load_from_test_file_and_case_without_integrity_check(
        test_archive_path: impl AsRef<Path>,
        test_case: u32,
    ) -> anyhow::Result<Self> {
        let test_archive_path = test_archive_path.as_ref();
        ensure!(
            test_archive_path.try_exists()?,
            "File does not exist: {}",
            test_archive_path.display()
        );

        let (flash, dump) = if test_archive_path.is_dir() {
            Self::find_files_in_directory(test_archive_path, test_case)?
        } else {
            Self::find_files_in_archive(test_archive_path, test_case)?
        };

        Ok(FlashTestCase {
            flash,
            dump,
            rom: None,
            mem_fmt: MemoryFormat::default(),
            output_yaml: false,
            test_path: test_archive_path.into(),
            test_case,
        })
    }

    fn integrity_check(&self) -> anyhow::Result<()> {
        let asm_path = self.test_path.with_extension("asm");
        let asm_content = fs::read_to_string(&asm_path)
            .with_context(|| format!("{}: failed to load asm file", asm_path.display()))?
            .replace("\r\n", "\n")
            .replace('\r', "\n"); // normalize line endings

        let mut hasher = Sha256::new();
        hasher.update(&self.flash);
        if hasher.finalize_reset().as_slice() != &self.dump.flash_sha256[..] {
            return Err(TestError::InvalidInput).context(format!(
                "{}: sha256 check failed for {}.flash (from archive)",
                self.test_path.display(),
                self.test_case,
            ));
        }

        hasher.update(asm_content);
        if hasher.finalize_reset().as_slice() != &self.dump.asm_sha256[..] {
            return Err(TestError::InvalidInput).context(format!(
                "{}: sha256 check failed for \"{}\"",
                self.test_path.display(),
                asm_path.display(),
            ));
        }

        Ok(())
    }

    /// # Errors
    /// Internal error occurred. Should never happen.
    pub fn get_new_test_runner(&self) -> anyhow::Result<impl TestRunner<'_> + '_> {
        let runner: Result<Runner, _> = self.try_into();
        runner
    }

    /// # Errors
    /// Internal error occurred. Should never happen.
    pub fn get_new_benchmark_runner(&self) -> anyhow::Result<impl BenchmarkRunner + '_> {
        let runner: Result<Runner, _> = self.try_into();
        runner
    }

    /// Consume the test case and return an emulator instance
    ///
    /// # Errors
    /// Internal error occurred. Should never happen.
    pub fn deconstruct_configured(self) -> anyhow::Result<(Emulator, TestDump)> {
        #[cfg_attr(not(feature = "cycle-debug-logger"), allow(unused_mut))]
        let mut runner: Runner = Runner::try_from(&self)?;

        #[cfg(feature = "cycle-debug-logger")]
        runner.set_cdl_custom_metadata();
        Ok((runner.emulator, self.dump))
    }

    pub fn set_rom(&mut self, rom: Option<Vec<u8>>) {
        self.rom = rom;
    }

    pub fn set_memory_format(&mut self, mem_fmt: MemoryFormat) {
        self.mem_fmt = mem_fmt;
    }

    pub fn set_machine_readable(&mut self, flag: bool) {
        self.output_yaml = flag;
    }

    #[must_use]
    pub fn get_dump(&self) -> &TestDump {
        &self.dump
    }

    #[must_use]
    pub fn display_metadata(&self) -> impl Display {
        format!(
            "Test: {}\n\
             Case: {}\n\
             Configuration: {}\n\
             Generation time: {}\n",
            self.test_path.display(),
            self.test_case,
            self.dump.configuration_name,
            self.display_generation_time(),
        )
    }

    #[must_use]
    pub fn display_test_path(&self) -> impl Display {
        format!("{}.{}", self.test_path.display(), self.test_case)
    }

    fn display_generation_time(&self) -> impl Display {
        humantime::format_rfc3339_millis(self.dump.generation_time)
    }

    /// # Errors
    /// Writing to `String` failed.
    pub fn display_dump(&self) -> anyhow::Result<impl Display> {
        fn display_sha256(sha256: &[u8]) -> anyhow::Result<impl Display> {
            debug_assert_eq!(sha256.len(), 32);
            let mut res = String::new();
            for byte in sha256 {
                write!(res, "{byte:02x}")?;
            }
            Ok(res)
        }

        let mut displayed = String::new();
        // rest of the fields is returned by `display_metadata`
        write!(
            displayed,
            "emulator main:       {:?}\n\
             emulator cdl start:  {:?}\n\
             emulator exit:       {:?}\n\
             flash sha256:        {}\n\
             asm sha256:          {}\n",
            self.dump.emulator_main_addr,
            self.dump.emulator_cdl_start_addr,
            self.dump.emulator_exit_addr,
            display_sha256(&self.dump.flash_sha256[..])?,
            display_sha256(&self.dump.asm_sha256[..])?,
        )?;
        writeln!(displayed, "dump:")?;
        // keep this a valid yaml array
        for mem_chunk in &self.dump.mem_dump {
            write!(
                displayed,
                "-
  symbol:   {}
  address:  {:?}
  expected: {}\n",
                mem_chunk.symbol_name,
                mem_chunk.addr,
                self.mem_fmt.display(&mem_chunk.content, true),
            )?;
        }
        Ok(displayed)
    }
}

// TODO: move it to somewhere shared
trait AndAnyhow {
    type Source;
    /// A helper method for `Result` to simplify chaining `.and_then`.
    ///
    /// Without this function, code like: `Err(12).and_then(u32::try_from)` would have mismatched
    /// `Error` types.
    fn and_then_anyhow<R, E>(
        self,
        f: impl FnOnce(Self::Source) -> Result<R, E>,
    ) -> anyhow::Result<R>
    where
        E: Into<anyhow::Error>;
}
impl<R1, E1> AndAnyhow for Result<R1, E1>
where
    E1: Into<anyhow::Error>,
{
    type Source = R1;
    fn and_then_anyhow<R, E>(self, f: impl FnOnce(R1) -> Result<R, E>) -> anyhow::Result<R>
    where
        E: Into<anyhow::Error>,
    {
        self.map_err(<E1 as Into<anyhow::Error>>::into)
            .and_then(|ok| f(ok).map_err(<E as Into<anyhow::Error>>::into))
    }
}
// Details of unpacking
impl FlashTestCase {
    fn find_files_in_directory(
        test_archive_path: &Path,
        test_case: u32,
    ) -> anyhow::Result<(Vec<u8>, TestDump)> {
        let flash_path = &test_archive_path.join(format!("{test_case}.flash"));
        let dump_path = &flash_path.with_extension("dump");
        let flash = fs::read(flash_path)
            .with_context(|| format!("Error while reading flash file: {}", flash_path.display()))?;
        let dump = File::open(dump_path)
            .and_then_anyhow(rmps::from_read)
            .with_context(|| {
                format!("Cannot open associated dump file: {}", dump_path.display())
            })?;
        Ok((flash, dump))
    }
    fn find_files_in_archive(
        test_archive_path: &Path,
        test_case: u32,
    ) -> anyhow::Result<(Vec<u8>, TestDump)> {
        let test_dir_in_archive = Path::new(test_archive_path.file_stem().unwrap());

        // expected files
        let flash_path = test_dir_in_archive.join(format!("{test_case}.flash"));
        let dump_path = flash_path.with_extension("dump");

        match test_archive_path.extension().and_then(OsStr::to_str) {
            Some("tzst") => Self::find_files_in_tzst(test_archive_path, &flash_path, &dump_path),
            Some("zip") => Self::find_files_in_zip(
                test_archive_path,
                flash_path.to_str().unwrap(),
                dump_path.to_str().unwrap(),
            ),
            None | Some(_) => bail!(
                "{}: test must be named <name>.tzst or <name>.zip",
                test_archive_path.display()
            ),
        }
    }

    fn find_files_in_zip(
        test_archive_path: &Path,
        flash_path: &str,
        dump_path: &str,
    ) -> anyhow::Result<(Vec<u8>, TestDump)> {
        let raw_zip = File::open(test_archive_path)
            .with_context(|| format!("{}: failed to open file", test_archive_path.display()))?;
        let mut zip = zip::ZipArchive::new(raw_zip).with_context(|| {
            format!(
                "{}: failed to decompress with zip",
                test_archive_path.display()
            )
        })?;
        let mut flash = vec![];
        zip.by_name(flash_path)
            .and_then_anyhow(|mut f| f.read_to_end(&mut flash))
            .with_context(|| {
                format!(
                    "Error while reading flash file: {} from archive {}",
                    flash_path,
                    test_archive_path.display()
                )
            })?;

        let dump = zip
            .by_name(dump_path)
            .and_then_anyhow(rmps::from_read)
            .with_context(|| {
                format!(
                    "{}: failed to load and parse dump file \"{}\" from the archive",
                    test_archive_path.display(),
                    dump_path,
                )
            })?;
        Ok((flash, dump))
    }

    fn find_files_in_tzst(
        test_archive_path: &Path,
        flash_path: &Path,
        dump_path: &Path,
    ) -> anyhow::Result<(Vec<u8>, TestDump)> {
        // read archive
        // TODO: if decompressed once,
        //       raw_tarball could be shared (cache) between test threads for optimization
        // For now, we employ stream decompression to save memory, since each test is loaded
        // independently anyway.
        let mut archive = File::open(test_archive_path)
            .and_then_anyhow(zstd::Decoder::new)
            .map(tar::Archive::new)
            .with_context(|| {
                format!(
                    "{}: failed to open and decompress with zstd",
                    test_archive_path.display()
                )
            })?;
        let entries = archive.entries().with_context(|| {
            format!(
                "{}: failed to iterate over compressed archive",
                test_archive_path.display()
            )
        })?;

        let mut flash = None;
        let mut dump = None;

        // find expected files
        for file in entries {
            let err_msg = || {
                format!(
                    "{}: invalid file entry in compressed tarball",
                    test_archive_path.display()
                )
            };
            let mut file = file.with_context(err_msg)?;
            let file_path = file.path().with_context(err_msg)?;

            if file_path == flash_path {
                if flash.is_some() {
                    return Err(TestError::InvalidInput).context(format!(
                        "{}: found second flash with same path \"{}\"",
                        test_archive_path.display(),
                        flash_path.display()
                    ));
                }
                let mut buf = vec![];
                file.read_to_end(&mut buf).with_context(err_msg)?;
                flash = Some(buf);
            } else if file_path == dump_path {
                if dump.is_some() {
                    return Err(TestError::InvalidInput).context(format!(
                        "{}: found second dump with same path \"{}\"",
                        test_archive_path.display(),
                        dump_path.display()
                    ));
                }
                let d: TestDump = rmps::from_read(file).with_context(|| {
                    format!(
                        "{}: failed to load and parse dump file \"{}\" from the archive",
                        test_archive_path.display(),
                        dump_path.display(),
                    )
                })?;
                dump = Some(d);
            }
        }
        match (flash, dump) {
            (Some(flash), Some(dump)) => Ok((flash, dump)),
            _ => Err(TestError::InvalidInput).context(format!(
                "{}: {} and/or {} not found in archive",
                test_archive_path.display(),
                flash_path.display(),
                dump_path.display(),
            )),
        }
    }
}

//---------------------------------------------------
// Helper type
//---------------------------------------------------

/// Specifies how memory dumps are displayed by `FlashTestCase`
#[derive(Copy, Clone, Debug, ValueEnum)]
#[non_exhaustive]
pub enum MemoryFormat {
    ByteHex,
    ByteAscii,
    ByteDec,
    WordHex,
    WordDec,
}

#[allow(clippy::derivable_impls)]
impl Default for MemoryFormat {
    fn default() -> Self {
        // Note: / TODO:
        // Temporarily set to WordDec, because it's currently most convenient for development purpose.
        // In general ByteHex is the best option:
        // doesn't require alignment to 4 bytes and is well suited for binary data.
        MemoryFormat::WordDec
    }
}

impl TryFrom<&str> for MemoryFormat {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::MEMORY_FORMATS
            .iter()
            .find_map(|(name, variant)| (value == *name).then_some(*variant))
            .ok_or(TestError::InvalidInput)
            .context("invalid memory format variant")
    }
}

impl MemoryFormat {
    pub const MEMORY_FORMATS: &'static [(&'static str, MemoryFormat)] = &[
        ("byte-hex", MemoryFormat::ByteHex),
        ("byte-ascii", MemoryFormat::ByteAscii),
        ("byte-dec", MemoryFormat::ByteDec),
        ("word-hex", MemoryFormat::WordHex),
        ("word-dec", MemoryFormat::WordDec),
    ];

    #[must_use]
    pub fn elem_len(self) -> usize {
        match self {
            MemoryFormat::ByteHex | MemoryFormat::ByteAscii | MemoryFormat::ByteDec => 1,
            MemoryFormat::WordHex | MemoryFormat::WordDec => 4,
        }
    }

    #[must_use]
    pub fn display(self, mem: &[u8], separators: bool) -> impl Display + '_ {
        // helper type
        struct DisplayMemory<'a>(&'a [u8], MemoryFormat, bool);
        impl Display for DisplayMemory<'_> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let (start_char, separator, end_char) = match self.1 {
                    MemoryFormat::ByteHex
                    | MemoryFormat::ByteDec
                    | MemoryFormat::WordHex
                    | MemoryFormat::WordDec => ("[", ", ", "]"),
                    MemoryFormat::ByteAscii => ("\"", "", "\""),
                };

                if self.2 {
                    write!(f, "{start_char}")?;
                }
                let mut print_separator = false;
                for chunk in self.0.chunks(self.1.elem_len()) {
                    // note: last chunk might be smaller
                    write!(f, "{}", if print_separator { separator } else { "" })?;
                    print_separator = self.2;

                    match self.1 {
                        MemoryFormat::ByteHex => {
                            write!(f, "{:02x}", chunk[0])?;
                        }
                        MemoryFormat::ByteAscii => {
                            let dont_escape = chunk[0] as char == '\n';
                            if dont_escape {
                                write!(f, "{}", chunk[0] as char)?;
                            } else {
                                write!(f, "{}", std::ascii::escape_default(chunk[0]))?;
                            }
                        }
                        MemoryFormat::ByteDec => {
                            write!(f, "{}", { chunk[0] })?;
                        }
                        MemoryFormat::WordHex => {
                            let val = u32::from_le_bytes([
                                chunk[0],
                                *chunk.get(1).unwrap_or(&0),
                                *chunk.get(2).unwrap_or(&0),
                                *chunk.get(3).unwrap_or(&0),
                            ]);
                            write!(f, "{val:08x}")?;
                        }
                        MemoryFormat::WordDec => {
                            let val = u32::from_le_bytes([
                                chunk[0],
                                *chunk.get(1).unwrap_or(&0),
                                *chunk.get(2).unwrap_or(&0),
                                *chunk.get(3).unwrap_or(&0),
                            ]);
                            write!(f, "{val}")?;
                        }
                    }
                }
                if self.2 {
                    write!(f, "{end_char}")?;
                }

                let last_elem_has_less_bytes = !self.0.len().is_multiple_of(self.1.elem_len());
                if last_elem_has_less_bytes {
                    write!(f, " # (trailing element expanded with zero bytes)")?;
                }

                Ok(())
            }
        }

        DisplayMemory(mem, self, separators)
    }
}
