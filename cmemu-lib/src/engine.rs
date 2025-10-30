//! Definition of the engine - "the components runtime".
//!
//! Contains everything required to run the components and provides some
//! building blocks for them - flops, subcomponents, auto ticks - which
//! emulator is aware of in opposite to the contents of the `crate::common`.
//! Highly connected with `build.rs` and `cmemu-proc-macros/component.rs`.

mod component_traits;
mod context;
mod emulator;
mod event_queue;
mod flop;
mod stm;
mod time;

pub(crate) use component_traits::{
    ClockTreeNode, CpuMode, DisableableComponent, EnergyNode, MainComponent, PowerMode, PowerNode,
    PureSubcomponentMarker, SkippableClockTreeNode, Subcomponent, TickComponent,
    TickComponentExtra,
};
pub(crate) use context::Context;
pub use context::SymbolsService;
pub use emulator::{Emulator, EmulatorError};
use event_queue::EventQueue;
pub(crate) use event_queue::EventRevokeToken;
pub(crate) use flop::{
    BufferFlop, CombFlop, CombFlopMemoryBank, CombFlopMemoryBankSimple, CombRegister, LatchFlop,
    SeqFlop, SeqFlopMemoryBank, SeqFlopMemoryBankSimple, SeqFlopMemoryBankSimpleLarge, SeqRegister,
};
#[cfg(debug_assertions)]
pub(crate) use stm::TransitionValidator;
pub(crate) use stm::{StateMachine, debug_move_state_machine, move_state_machine};
pub use time::{Duration, PICOS_IN_SECOND, Timepoint};
