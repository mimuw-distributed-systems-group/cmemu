//! Contains shared functionality, configuration / hardware description, etc.
//! defined in [ARM-ARM].
//!
//! Note: if some function is local to given part of core, consider defining it there.

/// [ARM-ARM] D6.7.22
#[allow(dead_code)] // it's not really dead, though
pub(super) const fn have_dsp_ext() -> bool {
    // [TI-TRM] doesn't list `SSAT16` as supported instruction,
    // which depends on this extension
    // Related: `misc/cpuid.asm` test
    false
}

/// [ARM-ARM] D6.7.23
pub(super) const fn have_fp_ext() -> bool {
    // [TI-TRM] 2.5.2.21 reserves bit control[2] as read-only with reset value 0
    // which indicates lack of support for floating point operations.
    // [ARM-TDG] Table 1.1 - floating point isn't listed in other features of Cortex-M3.
    false
}

// Note: this function will probably be a method on some (sub)component, not a
// builtin function.
/// [ARM-ARM] D6.7.30
#[allow(dead_code)] // it's not really dead, though
pub(super) const fn integer_zero_divide_trapping_enabled() -> bool {
    // TODO: Read DIV_0_TRP bit from CCR register. Not supported yet in cmemu.
    false
}
