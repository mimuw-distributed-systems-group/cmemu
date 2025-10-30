#![allow(unsafe_code, reason = "This is a test. Needed for an allocator.")]
// Note: in theory, it should be enough to run these tests in release mode,
//       but running them in debug mode should not hurt (except execution time).

use anyhow::Context;
use cmemu_flash_test_lib::{CallbackType, FlashTestCase, TestRunner};
use cmemu_lib::engine::Emulator;
use std::fs;
use std::path::{Path, PathBuf};

const CYCLES_TIMEOUT: u64 = 1_000_000_000; // required by coremark

#[test]
fn dijkstra() -> anyhow::Result<()> {
    const TEST_PATH: &str = "tests/flash/misc/dijkstra.tzst";
    const TEST_CASE: u32 = 4;
    run_single_test(TEST_PATH, TEST_CASE)
}

#[test]
#[cfg_attr(not(feature = "include-large-tests"), ignore)]
fn coremark() -> anyhow::Result<()> {
    const TEST_PATH: &str = "../cmemu-benchmark/coremark/coremark.tzst";
    const TEST_CASE: u32 = 2;
    run_single_test(TEST_PATH, TEST_CASE)
}

#[test]
fn random_tests() -> anyhow::Result<()> {
    fn search_for_tests(
        path: &dyn AsRef<Path>,
        tests_accumulator: &mut Vec<PathBuf>,
    ) -> anyhow::Result<()> {
        let path = path.as_ref();
        for dir_entry in fs::read_dir(path)
            .with_context(|| format!("failed to read {}", path.display()))?
            .map(|r| r.expect("reading testsuite directory entry"))
        {
            let p = dir_entry.path();

            // Ignore files & dirs starting with `.`, which could be editor temporary files & dirs
            let skip = p
                .file_stem()
                .is_none_or(|stem| stem.to_str().is_none_or(|stem| stem.starts_with('.')));
            if skip {
                continue;
            }

            // Check if dir
            match dir_entry.metadata() {
                Ok(metadata) => {
                    if metadata.is_dir() {
                        search_for_tests(&p, tests_accumulator)?;
                    } else if let Some(ext) = p.extension()
                        && ext == "tzst"
                        && !p.with_extension("ignore").exists()
                    {
                        tests_accumulator.push(p.clone());
                    }
                }
                Err(err) => {
                    return Err(err)
                        .context(format!("failed to get metadata of {}", path.display()));
                }
            }
        }

        Ok(())
    }

    use rand::seq::IteratorRandom;

    // White-listing directories to skip new large-tests files
    let mut tests = vec![];
    search_for_tests(&"tests/flash/cache", &mut tests)?;
    search_for_tests(&"tests/flash/instructions", &mut tests)?;
    search_for_tests(&"tests/flash/interrupts", &mut tests)?;
    search_for_tests(&"tests/flash/misc", &mut tests)?;

    let tests_cnt = 5;
    let rng = &mut rand::rng();
    for file in tests.iter().choose_multiple(rng, tests_cnt) {
        run_single_test(file, 0).with_context(|| format!("Test {}", file.display()))?;
    }

    Ok(())
}

fn run_single_test(test_path: impl AsRef<Path>, test_case: u32) -> anyhow::Result<()> {
    use counter_allocator::COUNTER_ALLOC;

    let (mut counters_start, mut counters_end) = (None, None);
    let mut callback = |_: &mut Emulator, event: CallbackType| match event {
        CallbackType::OnInit => {
            #[cfg(feature = "allocations-panic")]
            COUNTER_ALLOC.lock();
            counters_start = Some((
                COUNTER_ALLOC.get_alloc_cnt(),
                COUNTER_ALLOC.get_dealloc_cnt(),
            ));
        }
        CallbackType::OnFinish => {
            #[cfg(feature = "allocations-panic")]
            COUNTER_ALLOC.unlock();
            counters_end = Some((
                COUNTER_ALLOC.get_alloc_cnt(),
                COUNTER_ALLOC.get_dealloc_cnt(),
            ));
        }
        _ => (),
    };

    let test = FlashTestCase::load_from_test_file_and_case(test_path, test_case)?;
    let mut test_runner = test.get_new_test_runner()?;
    test_runner.configure_checked_symbols_mask(Some(&[])); // check no symbols
    test_runner.set_cycles_timeout(CYCLES_TIMEOUT);
    test_runner.set_callback(Some(&mut callback));

    test_runner.run()?;

    assert!(
        counters_start.is_some() && counters_end.is_some(),
        "Error occurred and test did not finish."
    );
    assert_eq!(
        counters_start, counters_end,
        "Some allocations or deallocations occurred."
    );

    Ok(())
}

/// Global allocator wrapper that includes thread local counters.
/// Note: The emulator is single-threaded.
mod counter_allocator {
    use std::alloc::{GlobalAlloc, Layout, System};
    use std::cell::RefCell;
    use std::thread_local;

    pub(crate) struct CounterAllocator;

    struct CounterAllocatorState {
        alloc_cnt: u32,
        dealloc_cnt: u32,
        #[cfg(feature = "allocations-panic")]
        lockdown: bool,
    }

    thread_local! {
        static COUNTER_ALLOC_STATE: RefCell<CounterAllocatorState> =
            const { RefCell::new(CounterAllocatorState::new()) };
    }

    unsafe impl GlobalAlloc for CounterAllocator {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            self.increment_alloc_cnt();
            unsafe { SYSTEM_ALLOC.alloc(layout) }
        }

        unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
            self.increment_dealloc_cnt();
            unsafe {
                SYSTEM_ALLOC.dealloc(ptr, layout);
            }
        }
    }

    impl CounterAllocator {
        #[cfg(feature = "allocations-panic")]
        pub(crate) fn lock(&self) {
            self.with_state(|s| {
                s.lockdown = true;
            });
        }

        #[cfg(feature = "allocations-panic")]
        pub(crate) fn unlock(&self) {
            self.with_state(|s| {
                s.lockdown = false;
            });
        }

        pub(crate) fn get_alloc_cnt(&self) -> u32 {
            self.with_state(|s| s.alloc_cnt)
        }

        pub(crate) fn get_dealloc_cnt(&self) -> u32 {
            self.with_state(|s| s.dealloc_cnt)
        }

        fn increment_alloc_cnt(&self) {
            // We need to free the RefCell before panicking (as it allocates)
            #[cfg(feature = "allocations-panic")]
            let mut should_panic = false;
            self.with_state(|s| {
                #[cfg(feature = "allocations-panic")]
                if s.lockdown {
                    // Unlock for recursive call in panic
                    s.lockdown = false;
                    should_panic = true;
                }
                s.alloc_cnt += 1;
            });
            #[cfg(feature = "allocations-panic")]
            #[allow(clippy::manual_assert, reason = "This is not an assertion")]
            if should_panic {
                panic!("Allocation performed during lockdown!");
            }
        }

        fn increment_dealloc_cnt(&self) {
            #[cfg(feature = "allocations-panic")]
            let mut should_panic = false;
            self.with_state(|s| {
                #[cfg(feature = "allocations-panic")]
                if s.lockdown {
                    // Unlock for recursive call in panic
                    s.lockdown = false;
                    should_panic = true;
                }
                s.dealloc_cnt += 1;
            });
            #[cfg(feature = "allocations-panic")]
            #[allow(clippy::manual_assert, reason = "This is not an assertion")]
            if should_panic {
                panic!("Deallocation performed during lockdown!");
            }
        }

        #[allow(clippy::unused_self)]
        fn with_state<T>(&self, f: impl FnOnce(&mut CounterAllocatorState) -> T) -> T {
            COUNTER_ALLOC_STATE.with(|state_ref_cell| {
                let mut state = state_ref_cell.borrow_mut();
                f(&mut state)
            })
        }
    }

    impl CounterAllocatorState {
        const fn new() -> Self {
            Self {
                alloc_cnt: 0,
                dealloc_cnt: 0,
                #[cfg(feature = "allocations-panic")]
                lockdown: false,
            }
        }
    }

    #[global_allocator]
    pub(crate) static COUNTER_ALLOC: CounterAllocator = CounterAllocator;
    static SYSTEM_ALLOC: System = System;
}
