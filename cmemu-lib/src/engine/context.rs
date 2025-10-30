use crate::build_data::EnergyEntity;
use crate::common::utils::FromMarker;
use crate::engine::{EventQueue, PowerMode};
use cmemu_common::Address;
use enum_map::{EnumMap, enum_map};
use std::fmt::Display;
#[cfg(feature = "pretty_log")]
use std::panic::UnwindSafe;

pub trait SymbolsService {
    fn display_named_address(
        &self,
        addr: Address,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result;
}

/// Context of the emulation.
/// Contains events and event queue.
/// Does not contain state of components.
pub(crate) struct Context {
    queue: EventQueue,
    pub(super) node_id: u64,
    cycle_no: u64,
    /// Ground truth state (nodes in transition report TODO state)
    pub(in crate::engine) energy_state: EnumMap<EnergyEntity, PowerMode>,

    #[cfg(feature = "pretty_log")]
    pub(super) symbols_service: Option<Box<dyn SymbolsService + Send + Sync + UnwindSafe>>,
}

// TODO:
//   slab would be better(?) without extra copies:
//   - create without taking new elem
//   - delete without returning old elem
//   most likely we'll fork slab and change/add methods we want

impl Context {
    pub(super) fn new() -> Self {
        Self {
            queue: EventQueue::new(),
            node_id: 0,
            cycle_no: 0,
            // TODO: call generated code with initial state
            energy_state: enum_map! {_ => PowerMode::Active},
            #[cfg(feature = "pretty_log")]
            symbols_service: None,
        }
    }

    #[cfg(test)]
    pub(crate) fn new_for_test() -> Self {
        Self::new()
    }
    #[cfg(test)]
    pub(crate) fn set_node_id_for_test(&mut self, new_id: u64) {
        self.node_id = new_id;
    }

    pub fn node_id(&self) -> u64 {
        self.node_id
    }

    /// Get a number suitable for identifying a cycle by a human
    pub(crate) fn cycle_no(&self) -> u64 {
        self.cycle_no
    }

    // Should not be called by a random code :)
    pub(crate) fn set_cycle_no(&mut self, cycle: u64) {
        self.cycle_no = cycle;
    }

    pub(crate) fn event_queue(&self) -> &EventQueue {
        &self.queue
    }

    pub(crate) fn event_queue_mut(&mut self) -> &mut EventQueue {
        &mut self.queue
    }

    #[cfg_attr(not(debug_assertions), allow(unused))]
    pub(crate) fn get_energy_state<M>(&self) -> PowerMode
    where
        EnergyEntity: FromMarker<M>,
    {
        self.energy_state[FromMarker::<M>::from_marker()]
    }
    pub(crate) fn get_energy_state_of(&self, ent: EnergyEntity) -> PowerMode {
        self.energy_state[ent]
    }
    // FIXME: this should be visible only from the ClockTree
    pub(crate) fn set_energy_state_of(&mut self, ent: EnergyEntity, state: PowerMode) {
        self.energy_state[ent] = state;
    }

    // TODO: decide if we display the raw address or only pretty name (then the name is misleading)
    #[cfg(not(feature = "pretty_log"))]
    #[inline(always)]
    pub(crate) fn display_named_address(&self, _addr: Address) -> impl Display {
        ""
    }

    #[cfg(feature = "pretty_log")]
    pub(crate) fn display_named_address(&self, addr: Address) -> impl Display {
        use crate::utils::{Displayer, VarDisplay};
        // VarDisplay<&'a dyn Display, &'a str>
        if let Some(ref service) = self.symbols_service {
            // TODO: hide the dyn impl behind a feature flag?
            VarDisplay::A(Displayer(move |f| service.display_named_address(addr, f)))
        } else {
            use cc2650_constants::get_register_name;
            // TODO: it should be +2, +4
            VarDisplay::B(
                get_register_name(addr)
                    .or_else(|| get_register_name(addr.aligned_down_to_2_bytes()))
                    .or_else(|| get_register_name(addr.aligned_down_to_4_bytes()))
                    .unwrap_or(""),
            )
        }
    }
}
