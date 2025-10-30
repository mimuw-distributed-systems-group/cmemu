//! Components are basic building blocks of the emulator.
//! They are encapsulated and exchange events between themselves.

pub(crate) mod ahb_wiring;
pub(crate) mod aon_bus;
pub(crate) mod aon_event;
pub(crate) mod bitband;
pub(crate) mod bus_matrix;
pub(crate) mod core;
#[cfg(feature = "cycle-debug-logger")]
pub(crate) mod cycle_debug_logger;
pub(crate) mod dwt;
pub(crate) mod event_fabric;
pub(crate) mod flash;
pub(crate) mod gpio;
pub(crate) mod gpram;
pub(crate) mod mem_mock;
pub(crate) mod memory_bypass;
pub(crate) mod nvic;
pub(crate) mod osc;
pub(crate) mod prcm;
pub(crate) mod rfc;
pub(crate) mod rom;
pub(crate) mod rtc;
pub(crate) mod rtc_bypass;
pub(crate) mod semi_hosting;
pub(crate) mod sram;
pub(crate) mod sync_down_bridge;
pub(crate) mod sysbus;
pub(crate) mod uart_lite;
pub(crate) mod vims;
pub(crate) mod wuc;

mod components {
    #![allow(unused_braces, unused_qualifications)]
    #![allow(clippy::absolute_paths)]
    include!(concat!(env!("OUT_DIR"), "/component.rs"));
}
pub(crate) use components::Components;

// Clock tree is a special components, so it is treated separately
pub(crate) mod clock_tree;
pub(crate) use clock_tree::PowerClockManager;

// TODO: move it to some sensible place
#[derive(Debug)]
pub(crate) enum WakeupEvent {
    Radio,
}
