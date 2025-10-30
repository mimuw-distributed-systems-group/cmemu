// Exceptions from clippy::pedantic
#![allow(
    clippy::needless_pass_by_value, // By-value indicates semantics, and the lint changes API
    clippy::zero_sized_map_values, // Lints at use sites
)]

//////////////////////////////////////////////
//          Analyze components code
//////////////////////////////////////////////
pub mod components;
//////////////////////////////////////////////
//          Proc-macros code
//////////////////////////////////////////////
mod data_macros;
mod graph;
pub mod proc;
mod utils;
