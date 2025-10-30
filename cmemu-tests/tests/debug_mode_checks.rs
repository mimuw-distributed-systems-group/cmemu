// The following tests are meant as a way to verify at test runtime that
// our mechanism for running tests faster (with opt-level 3 and LTO) is
// not making the test suite less useful.

#[cfg(feature = "test-debug-mode-checks")]
#[test]
#[should_panic(expected = "test debug assertions")]
fn debug_assertions_work() {
    debug_assert!(false, "fake failed assertion to test debug assertions");
}

#[cfg(feature = "test-debug-mode-checks")]
#[test]
#[should_panic(expected = "overflow")]
#[allow(arithmetic_overflow)]
fn overflow_checks_work() {
    dbg!(0u8 - 1u8);
}
