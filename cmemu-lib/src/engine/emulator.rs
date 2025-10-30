// Allow `EmulatorError` that starts with the same name as the module.
#![allow(clippy::module_name_repetitions)]

use super::{Context, Duration, Timepoint};
use crate::component::{Components, PowerClockManager, WakeupEvent};
use crate::proxy::{ClockTreeProxy, event_data::EventData};
pub use component_api::EmulatorError;
use log::{debug, info, log_enabled, trace};

mod component_api;

#[allow(missing_debug_implementations)]
pub struct Emulator {
    components: Components,
    clock_tree: PowerClockManager,
    context: Context,
}

impl Emulator {
    pub fn new(flash_mem: &[u8], rom_mem: Option<&[u8]>) -> Self {
        let mut context = Context::new();
        ClockTreeProxy.power_on_reset(&mut context);

        Self {
            components: Components::new(flash_mem, rom_mem),
            clock_tree: PowerClockManager::new(),
            context,
        }
    }

    pub fn get_emulation_time(&self) -> Timepoint {
        self.context.event_queue().get_current_time()
    }

    pub fn get_next_event_time(&self) -> Timepoint {
        self.context
            .event_queue()
            .peek_timepoint()
            .unwrap_or(Timepoint::from_picos(u64::MAX))
    }

    pub fn step_until(&mut self, timepoint: Timepoint) {
        if log_enabled!(log::Level::Debug) {
            debug!(
                "Emulating until: {timepoint:?}, next: {:?}: {:?}",
                self.get_next_event_time(),
                self.context.event_queue().peek_payload(),
            );
        }
        while self.get_next_event_time() < timepoint {
            self.step_phase();
        }
    }

    // This makes sense only when a single OSC is running
    pub fn step_cycle(&mut self) {
        self.step_phase();
        self.step_phase();
    }

    /// Steps until all events for the current phase of the HF clock are handled.
    fn step_phase(&mut self) {
        let now = self.get_emulation_time();
        let start_timepoint = self.get_next_event_time();
        let diff = start_timepoint.wrapping_sub_timepoint(now);
        trace!("Emulation step phase at: {start_timepoint:?}, after: {diff:#?} from {now:?}");
        if diff > Duration::LOOKS_SATURATED {
            // Is it the right place?
            panic!("Skipped an ethernity, because no wakeup event was present!");
        }

        // This is either a Tick/Tock, external wakeup, or other event from the clocks
        let first_payload = self.context.event_queue_mut().pop().unwrap();
        match first_payload {
            EventData::Wakeup(event) => {
                // External wakeup essentially makes all clocks reevaluate their skip condition
                info!(
                    "External wakeup event {event:?}: t={:#?} n={}",
                    self.context.event_queue().get_current_time(),
                    self.context.node_id()
                );
                self.clock_tree
                    .external_wake_up(&mut self.context, &mut self.components);
            }
            payload => self.dispatch_event(payload),
        }

        // Just iterating over ArrayDeque -- heavily optimized
        while let Some(payload) = self.context.event_queue_mut().pop_now() {
            #[cfg(debug_assertions)]
            if let EventData::Wakeup(wakeup) = &payload {
                panic!("received wakeup {wakeup:?} in the middle of a phase")
            }
            self.dispatch_event(payload);
        }
    }

    pub fn trigger_radio_wakeup(&mut self, timepoint: Timepoint) {
        // TODO: panic on wrap?
        let now = self.get_emulation_time();
        let desired_delay = timepoint.wrapping_sub_timepoint(now);
        self.context
            .event_queue_mut()
            .add(desired_delay, EventData::Wakeup(WakeupEvent::Radio));
        self.components.rfc.notify_radio_wakeup();
    }

    #[cfg(feature = "cdl-black-box")]
    pub fn launch_black_box(&mut self) {
        self.components.cycle_debug_logger.launch_black_box();
    }
}

include!(concat!(env!("OUT_DIR"), "/emulator_handler_dispatch.rs"));

#[cfg(feature = "flash-test-lib")]
impl Emulator {
    /// Setup internal emulator state for flash tests.
    // Note: function for temporary initializations (for outdated tests) which should be removed
    // when all tests are preparing given component by themselves in their prologue.
    pub fn prepare_for_flash_test(&mut self) {
        use super::PowerMode;
        use crate::build_data::ClockTreeNodes;

        self.components.dwt.prepare_for_test();
        // Multiple (mostly large-) tests depend on GPIO or Radio being turned on by the startup code.
        self.clock_tree.want_node_state(
            &mut self.context,
            ClockTreeNodes::PeriphPowerDomain,
            PowerMode::Active,
        );
        self.clock_tree.want_node_state(
            &mut self.context,
            ClockTreeNodes::GpioGate,
            PowerMode::Active,
        );
        // TODO: Radio PD: should be requested explicitly by the test... (it's in the config)
        self.clock_tree.want_node_state(
            &mut self.context,
            ClockTreeNodes::RfcorePowerDomain,
            PowerMode::Active,
        );
        self.clock_tree.want_node_state(
            &mut self.context,
            ClockTreeNodes::RfcGate,
            PowerMode::Active,
        );
    }
}
