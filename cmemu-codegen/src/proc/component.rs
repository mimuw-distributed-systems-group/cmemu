//! Module containing implementation of component-related macros.
//! Part of cmemu engine.

mod component_impl;
mod derive_disableable_component;
mod derive_main_component;
mod derive_tick_component;
mod derive_tick_component_extra;
mod handler;
mod proxy_use;

pub use component_impl::component_impl;
pub use derive_disableable_component::derive_disableable_component;
pub use derive_main_component::{derive_component, derive_skippable_clock_tree_node};
pub use derive_tick_component::{derive_subcomponent, derive_tick_component};
pub use derive_tick_component_extra::derive_tick_component_extra;
pub use handler::handler;
pub use proxy_use::proxy_use;

// helper method
// "field_name" refers to "cmemu_build_conf.yaml"
fn validate_field_name(field_name: &str) -> bool {
    !field_name.is_empty()
        && field_name
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
        && !field_name.chars().next().unwrap().is_ascii_digit()
}
