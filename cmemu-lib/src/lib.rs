// As of 1.89, we cannot override workspace lints in crate's Cargo.toml, so we do it here
// Local exemptions from clippy::pedantic
#![allow(
    clippy::upper_case_acronyms, // temporarily ignored pending naming decisions
    clippy::must_use_candidate, // would be useful, but not working meaningfully here
    clippy::needless_pass_by_value, // this is a semantic statement
    clippy::redundant_closure_for_method_calls, // Check for it sometimes, but mostly less readable
    clippy::inline_always,
    clippy::if_not_else, // This is too pedantic over negatives that readability if provides
    clippy::redundant_else, // better readability
    clippy::struct_field_names,
    clippy::enum_glob_use,

    clippy::should_panic_without_expect, // TODO
    clippy::ignore_without_reason, // TODO
    clippy::manual_assert, // TODO: replace with meaningful names (whose error is this)

    // FIXME: enable these
    clippy::unused_self,
    clippy::too_many_lines,
)]
// Extra clippy::restrictions lints for lib-creates
#![warn(
    clippy::print_stdout, // Don't leave debug code! Or explicitly allow this lint if needed.
    clippy::print_stderr,
)]

#[macro_use]
pub mod common;
mod component;
pub mod engine;
mod proxy;

#[cfg(test)]
#[macro_use]
pub(crate) mod test_utils;
#[macro_use]
pub(crate) mod utils;

pub(crate) mod build_data {
    #![allow(dead_code, unused_braces, unused_qualifications)]
    #![allow(clippy::absolute_paths)]
    include!(concat!(env!("OUT_DIR"), "/build_data.rs"));
}

pub const BUILT_WITH_CYCLE_DEBUG_LOGGER: bool = cfg!(feature = "cycle-debug-logger");

::confeature::include_out_dir! {pub(crate) mod confeature;}
