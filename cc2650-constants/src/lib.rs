// No clippy::pedantic, as it chokes on the generated code.
#![allow(clippy::pedantic)]
// Modules here are by definition non-snake-case.
#![allow(non_snake_case)]
// FIXME: this lint is broken on nightly
#![allow(unused_parens)]

mod cortex_m3;
mod generated;
mod manually_copypasted;
#[cfg(feature = "register-names")]
mod revmap;
mod utils;

pub use cortex_m3::*;
pub use generated::*;
pub use manually_copypasted::*;
pub use utils::*;

#[cfg(feature = "register-names")]
pub use revmap::*;
