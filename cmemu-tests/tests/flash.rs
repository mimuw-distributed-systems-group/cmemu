use anyhow::Context;
use cmemu_flash_test_lib::{FlashTestCase, MemoryFormat, TestRunner};
use std::env::{self, VarError};
use std::fs;
use std::path::Path;

include!(concat!(env!("OUT_DIR"), "/flash_tests.rs"));
const ROM_PATH: &str = concat!(env!("OUT_DIR"), "/rom.bin");

fn run_flash_test(
    test_archive_path: impl AsRef<Path>,
    test_case: u32,
    checked_symbols: &[&str],
    wants_rom: bool,
) -> anyhow::Result<()> {
    let mut test = FlashTestCase::load_from_test_file_and_case(test_archive_path, test_case)?;
    if let Some(mf) = get_memory_format()? {
        test.set_memory_format(mf);
    }
    // This is asserted at the test ignore level
    if wants_rom {
        test.set_rom(Some(fs::read(ROM_PATH)?));
    }
    let mut test_runner = test.get_new_test_runner()?;
    #[cfg(feature = "test-cdl-generates-correctly")]
    let cdl_path = generate_temporary_cdl_path(&test);
    #[cfg(feature = "test-cdl-generates-correctly")]
    test_runner.configure_cycle_debug_logger(Some(cdl_path.clone()), true);
    test_runner.configure_checked_symbols_mask(Some(checked_symbols));
    if let Some(ct) = get_cycles_timeout()? {
        test_runner.set_cycles_timeout(ct);
    }
    println!("{}", test.display_metadata());
    println!("checked_symbols_mask: {checked_symbols:?}");
    test_runner.run()?;
    #[cfg(feature = "test-cdl-generates-correctly")]
    std::fs::remove_file(cdl_path)?;
    Ok(())
}

/// The tests should pass with default settings.
/// This one ensures the settings are not modified accidentally.
#[test]
fn ensure_not_reconfigured() {
    assert!(
        matches!(get_memory_format(), Ok(None)),
        "setting meant for developer use only!"
    );
    assert!(
        matches!(get_cycles_timeout(), Ok(None)),
        "setting meant for developer use only!"
    );
}

// ----------------------------------------------------------------------------
// helper methods
// ----------------------------------------------------------------------------

fn get_memory_format() -> anyhow::Result<Option<MemoryFormat>> {
    parse_env_var("CMEMU_TEST_MEMORY_FORMAT", |v| v.as_str().try_into())
}

fn get_cycles_timeout() -> anyhow::Result<Option<u64>> {
    parse_env_var("CMEMU_TEST_CYCLES_TIMEOUT", |v| {
        v.parse::<u64>().context("invalid cycles timeout value")
    })
}

#[cfg(feature = "test-cdl-generates-correctly")]
fn generate_temporary_cdl_path(test: &FlashTestCase) -> std::path::PathBuf {
    use std::collections::hash_map;
    use std::hash::{Hash, Hasher};
    let mut hasher = hash_map::DefaultHasher::new();
    test.display_metadata().to_string().hash(&mut hasher);
    let hash = format!("test-cdl-{}.json", hasher.finish());
    std::env::temp_dir().join(hash)
}

fn parse_env_var<F, T>(var: &str, f: F) -> anyhow::Result<Option<T>>
where
    F: FnOnce(String) -> anyhow::Result<T>,
{
    match env::var(var) {
        Ok(v) => Ok(Some(f(v)?)),
        Err(VarError::NotPresent) => Ok(None),
        Err(err) => Err(err).with_context(|| format!("invalid {var} env var value")),
    }
}
