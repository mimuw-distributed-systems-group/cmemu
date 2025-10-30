//! Emulator public API (that is aware of the internal components).

use super::Emulator;
use crate::build_data::EnergyEntity;
#[cfg(feature = "cycle-debug-logger")]
use crate::common::new_ahb::signals::{TransferMeta, TransferType};
use crate::component::core::CoreCoupledRegisterId;
use crate::engine::PowerMode;
#[cfg(feature = "pretty_log")]
use crate::engine::context::SymbolsService;
use crate::{
    common::{Address, RegisterID, UARTLiteInterface, Word},
    component::rfc::ModemImpl,
};
use std::borrow::Borrow;
use std::fmt::Display;
use std::panic::UnwindSafe;
#[cfg(feature = "cycle-debug-logger")]
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum EmulatorError {
    #[error("requested invalid address range")]
    InvalidAddress,
}

#[cfg(feature = "pretty_log")]
pub type SymbolsServiceImpl = Box<dyn SymbolsService + Send + Sync + UnwindSafe>;

impl Emulator {
    pub fn get_register(&self, register: RegisterID) -> Word {
        self.components
            .core
            .get_register(CoreCoupledRegisterId::Core(register))
    }

    /// Access any [ARM-ARM] B1.4 defined register that is not memory-mapped.
    pub fn get_extended_register(&self, register: CoreCoupledRegisterId) -> Word {
        self.components.core.get_register(register)
    }

    /// The architecture defines the PC to be the instruction address + 4,
    /// and that's what returned from ``get_register``.
    /// This method returns the actual address as required for tooling.
    ///
    /// NOTE: folded and skipped instructions may be not present in the sequence of
    ///       consecutively returned addresses.
    pub fn get_current_instruction_address(&self) -> Address {
        // FIXME: this is not flopped and thus a different semantics
        self.components.core.get_this_instr_addr()
    }

    /// The next instruction address should be valid in most cases,
    /// as long as this method is not called in the middle of a cycle.
    ///
    /// NOTE: folded and skipped instructions may be not present in the sequence of
    ///       consecutively returned addresses.
    pub fn get_next_instruction_address(&self) -> Address {
        self.components.core.get_next_instr_addr()
    }

    /// Check if the core is executing logicaly different instruction than in the prior cycle.
    ///
    /// In the case of an in-place loop, this won't be detectable from the address.
    /// It is not specified what state the Core is in, just that this method will return
    /// ``true`` exactly once per advancing the Core's pipeline.
    pub fn current_instruction_changed(&self) -> bool {
        self.components.core.get_pipeline_advanced()
    }

    /// Note: integers are stored using little endian
    /// # Errors
    /// `EmulatorError::InvalidAddress` if address range is not fully
    /// covered by address space of single memory component.
    ///
    /// # Panics
    /// Internal error occurred. Should never happen.
    pub fn write_memory(
        &mut self,
        start_address: Address,
        memory: &[u8],
    ) -> Result<(), EmulatorError> {
        let mut successes = 0;

        if self
            .components
            .flash
            .write_memory(start_address, memory)
            .is_ok()
        {
            successes += 1;
        }
        if self
            .components
            .rom
            .write_memory(start_address, memory)
            .is_ok()
        {
            successes += 1;
        }
        if self
            .components
            .gpram
            .write_memory(start_address, memory)
            .is_ok()
        {
            successes += 1;
        }
        if self
            .components
            .sram
            .write_memory(start_address, memory)
            .is_ok()
        {
            successes += 1;
        }
        if self
            .components
            .sysbus
            .semi_hosting
            .write_memory(start_address, memory)
            .is_ok()
        {
            successes += 1;
        }

        match successes {
            0 => Err(EmulatorError::InvalidAddress),
            1 => Ok(()),
            _ => {
                unreachable!("Given memory address should be handled by only one memory component")
            }
        }
    }

    /// Note: integers are stored using little endian
    /// # Errors
    /// `EmulatorError::InvalidAddress` if address range is not fully
    /// covered by address space of single memory component.
    ///
    /// # Panics
    /// Internal error occurred. Should never happen.
    pub fn read_memory(
        &self,
        start_address: Address,
        memory: &mut [u8],
    ) -> Result<(), EmulatorError> {
        let mut successes = 0;

        if self
            .components
            .flash
            .read_memory(start_address, memory)
            .is_ok()
        {
            successes += 1;
        }
        if self
            .components
            .rom
            .read_memory(start_address, memory)
            .is_ok()
        {
            successes += 1;
        }
        if self
            .components
            .gpram
            .read_memory(start_address, memory)
            .is_ok()
        {
            successes += 1;
        }
        if self
            .components
            .sram
            .read_memory(start_address, memory)
            .is_ok()
        {
            successes += 1;
        }
        if self
            .components
            .sysbus
            .semi_hosting
            .read_memory(start_address, memory)
            .is_ok()
        {
            successes += 1;
        }

        match successes {
            0 => Err(EmulatorError::InvalidAddress),
            1 => Ok(()),
            _ => {
                unreachable!("Given memory address should be handled by only one memory component")
            }
        }
    }

    // Send + Sync enforcement makes Emulator shared between threads safely.
    // return old value if possible, safe to drop
    #[cfg(feature = "pretty_log")]
    pub fn set_symbols_service(
        &mut self,
        service: Option<SymbolsServiceImpl>,
    ) -> Option<SymbolsServiceImpl> {
        std::mem::replace(&mut self.context.symbols_service, service)
    }

    pub fn name_an_address(&self, addr: Address) -> impl Display + '_ {
        self.context.display_named_address(addr)
    }

    pub fn set_uart_lite_interface(
        &mut self,
        interface: Option<Box<dyn UARTLiteInterface + Send + Sync + UnwindSafe>>,
    ) {
        self.components.uart_lite.set_interface(interface);
    }

    pub fn set_radio_interface(&mut self, interface: Option<ModemImpl>) {
        self.components.rfc.set_interface(interface);
    }

    /// Start the emulation process from another address than `ResetISR` (located at 0x4).
    ///
    /// It doesn't change the behavior of reset (interrupt), e.g., after shutting down.
    /// Useful for instance to skip loader to run a dual-compiled binary -- i.e., that works
    /// both as userspace application on hosted arm systems and a barebone Cortex-M.
    pub fn set_nonstandard_entrypoint(&mut self, entrypoint: Option<Address>) {
        self.components.core.set_nonstandard_entrypoint(entrypoint);
    }

    // The public API is purposely limited.
    #[cfg(feature = "cycle-debug-logger")]
    pub fn peek_core_lsu_request(&self) -> Option<impl Borrow<TransferMeta>> {
        match &self
            .components
            .cycle_debug_logger
            .peek_lsu_request()?
            .addr_phase
            .meta
        {
            TransferType::Idle | TransferType::_Busy | TransferType::NoSel => None,
            TransferType::NonSeq(meta) | TransferType::Seq(meta) => Some(meta),
        }
    }

    #[cfg(feature = "cycle-debug-logger")]
    pub fn cycle_debug_logger_start_recording(&mut self) {
        self.components.cycle_debug_logger.start_recording();
    }

    #[cfg(feature = "cycle-debug-logger")]
    pub fn cycle_debug_logger_stop_recording(&mut self) {
        self.components.cycle_debug_logger.stop_recording();
    }

    #[cfg(feature = "cycle-debug-logger")]
    pub fn cycle_debug_logger_dump_now(
        &mut self,
    ) -> Result<Option<impl Borrow<Path>>, Box<dyn std::error::Error>> {
        self.components.cycle_debug_logger.dump_to_log_file()
    }

    #[cfg(feature = "cycle-debug-logger")]
    pub fn set_cycle_debug_logger_file(&mut self, log_file: Option<impl AsRef<Path>>) {
        self.components.cycle_debug_logger.set_log_file(log_file);
    }

    #[cfg(feature = "cycle-debug-logger")]
    pub fn set_cycle_debug_logger_metadata(&mut self, key: &'static str, value: String) {
        self.components
            .cycle_debug_logger
            .set_custom_metadata(key, value);
    }

    pub fn get_leds_state(&self) -> LedsState {
        LedsState {
            yellow: *self.components.gpio.yellow_led_state,
            vled1: *self.components.gpio.v1_led_state,
            vled2: *self.components.gpio.v2_led_state,
        }
    }

    pub fn set_node_id(&mut self, id: u64) {
        self.context.node_id = id;
        let [byte_1st, byte_2nd, ..] = id.to_le_bytes();
        let mac_address = [
            byte_1st, byte_1st, byte_1st, byte_2nd, byte_1st, 0x74, 0x12, 0x00,
        ];
        *self
            .components
            .mem_mock
            .automock
            .fcfg1_mac_15_4_1
            .unsafe_as_mut() = u32::from_le_bytes(mac_address[4..8].try_into().unwrap()).into();
        *self
            .components
            .mem_mock
            .automock
            .fcfg1_mac_15_4_0
            .unsafe_as_mut() = u32::from_le_bytes(mac_address[0..4].try_into().unwrap()).into();
        self.components.mem_mock.set_rng_seed(id);
        #[cfg(feature = "cycle-debug-logger")]
        self.components
            .cycle_debug_logger
            .set_custom_metadata("MAC address", format!("{id:016x}"));
    }

    // unstable interface
    pub fn unstable_get_components_energy_state(
        &self,
    ) -> impl Iterator<Item = (impl Borrow<str>, impl Borrow<PowerMode>)> {
        fn render_ent(ent: EnergyEntity) -> &'static str {
            match ent {
                EnergyEntity::Component(c) => <&'static str as From<_>>::from(c),
                EnergyEntity::ClockTree(c) => <&'static str as From<_>>::from(c),
                EnergyEntity::Oscillator(o) => <&'static str as From<_>>::from(o),
            }
        }
        self.context
            .energy_state
            .iter()
            .map(|(entity, mode)| (render_ent(entity), *mode))
    }
}

#[derive(Copy, Clone, Debug)]
pub struct LedsState {
    pub yellow: bool,
    pub vled1: bool,
    pub vled2: bool,
}
