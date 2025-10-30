use crate::component::event_fabric::EventFabricEvent;
use crate::component::rfc::{Interrupt, ResultByte};
use crate::engine::{Context, SeqFlopMemoryBankSimple};
use crate::engine::{
    DisableableComponent, SeqFlopMemoryBank, Subcomponent, TickComponent, TickComponentExtra,
};
use crate::proxy::RfcProxy;
use cc2650_constants::RFC;
use cmemu_common::Address;
use cmemu_common::HwRegister;
use log::{trace, warn};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct CommandStatusHandle {
    result: ResultByte,
    return_byte_1: Option<u8>,
    return_byte_2: Option<u8>,
    return_byte_3: Option<u8>,
}
impl From<ResultByte> for CommandStatusHandle {
    fn from(result: ResultByte) -> Self {
        CommandStatusHandle {
            result,
            return_byte_1: None,
            return_byte_2: None,
            return_byte_3: None,
        }
    }
}

impl From<CommandStatusHandle> for u32 {
    fn from(status: CommandStatusHandle) -> Self {
        u32::from(u8::from(status.result))
            | (u32::from(u8::from(status.return_byte_1.unwrap_or(0))) << 8)
            | (u32::from(u8::from(status.return_byte_2.unwrap_or(0))) << 16)
            | (u32::from(u8::from(status.return_byte_3.unwrap_or(0))) << 24)
    }
}

#[derive(Subcomponent, TickComponent, TickComponentExtra, DisableableComponent)]
pub(super) struct RfcRegisterBank<SC>
where
    SC: Subcomponent<Member = Self>,
{
    #[flop]
    cmdr: SeqFlopMemoryBank<RFC::DBELL::CMDR::Register, u32>,
    #[flop]
    // This can't be RFC::DBELL::CMDR::Register, as this register is RO
    cmdsta: SeqFlopMemoryBankSimple<CommandStatusHandle>,
    #[flop]
    rfackifg: SeqFlopMemoryBank<RFC::DBELL::RFACKIFG::Register, u32>,
    #[flop]
    rfcpeifg: SeqFlopMemoryBank<RFC::DBELL::RFCPEIFG::Register, u32>,
    #[flop]
    rfcpeien: SeqFlopMemoryBank<RFC::DBELL::RFCPEIEN::Register, u32>,
    #[flop]
    rfcpeisl: SeqFlopMemoryBank<RFC::DBELL::RFCPEISL::Register, u32>,
    #[flop]
    ratcnt: SeqFlopMemoryBank<RFC::RAT::RATCNT::Register, u32>,
    #[flop]
    pwmclken: SeqFlopMemoryBank<RFC::PWR::PWMCLKEN::Register, u32>,
    phantom_subcomponent: std::marker::PhantomData<SC>,
}

impl<SC> RfcRegisterBank<SC>
where
    SC: Subcomponent<Member = Self>,
{
    pub(super) fn new() -> Self {
        Self {
            cmdr: SeqFlopMemoryBank::new(RFC::DBELL::CMDR::Register::new()),
            cmdsta: SeqFlopMemoryBankSimple::new(CommandStatusHandle::from(ResultByte::Pending)),
            rfackifg: SeqFlopMemoryBank::new(RFC::DBELL::RFACKIFG::Register::new()),
            rfcpeifg: SeqFlopMemoryBank::new(RFC::DBELL::RFCPEIFG::Register::new()),
            rfcpeien: SeqFlopMemoryBank::new(RFC::DBELL::RFCPEIEN::Register::new()),
            rfcpeisl: SeqFlopMemoryBank::new(RFC::DBELL::RFCPEISL::Register::new()),
            ratcnt: SeqFlopMemoryBank::new(RFC::RAT::RATCNT::Register::new()),
            pwmclken: SeqFlopMemoryBank::new(RFC::PWR::PWMCLKEN::Register::new()),
            phantom_subcomponent: std::marker::PhantomData,
        }
    }
}

impl<SC> RfcRegisterBank<SC>
where
    SC: Subcomponent<Member = Self>,
{
    pub(super) fn get_data_for_address(&mut self, addr: Address) -> u32 {
        match addr {
            RFC::DBELL::CMDR::ADDR => {
                let value = self.cmdr.read();
                trace!("reading CMDR: {:#x}", value);
                value
            }
            RFC::DBELL::RFACKIFG::ADDR => {
                let value = self.rfackifg.read();
                trace!("reading RFACKIFG: {:#x}", value);
                value
            }
            RFC::DBELL::RFCPEIFG::ADDR => {
                let value = self.rfcpeifg.read();
                trace!("reading RFCPEIFG: {:#x}", value);
                value
            }
            RFC::DBELL::RFCPEIEN::ADDR => {
                let value = self.rfcpeien.read();
                trace!("reading RFCPEIEN: {:#x}", value);
                value
            }
            RFC::DBELL::RFCPEISL::ADDR => {
                let value = self.rfcpeisl.read();
                trace!("reading RFCPEISL: {:#x}", value);
                value
            }
            RFC::PWR::PWMCLKEN::ADDR => {
                let value = self.pwmclken.read();
                trace!("reading PWMCLKEN: {:#x}", value);
                value
            }
            RFC::RAT::RATCNT::ADDR => {
                let value = self.ratcnt.read();
                trace!("reading RATCNT: {:#x}", value);
                value
            }
            RFC::DBELL::CMDSTA::ADDR => {
                let value = *self.cmdsta;
                trace!("reading CMDSTA: {:#x} ({:?})", u32::from(value), value);
                u32::from(value)
            }
            _ => todo!("reading unknown register {addr:?}"),
        }
    }

    pub(super) fn set_data_for_address(&mut self, ctx: &mut Context, addr: Address, data: u32) {
        match addr {
            RFC::DBELL::CMDR::ADDR => {
                if self.cmdr.read() != 0 {
                    warn!(
                        "Requested CMDR write, when it wasn't 0. CMDR value: {:08x}",
                        self.cmdr.read()
                    );
                }
                trace!("writing CMDR => {:#x}", data);
                self.cmdr.mutate_next(data, |reg, val| reg.mutate(val));
                if data != 0 {
                    RfcProxy.on_non_zero_cmdr_write(ctx, data);
                }
            }
            RFC::DBELL::CMDSTA::ADDR => {
                panic!(
                    "Tried to write {:#x} to CMDSTA, but CMDSTA register is readonly.",
                    data
                );
            }
            RFC::DBELL::RFACKIFG::ADDR => {
                trace!("writing RFACKIFG => {:#x}", data);
                self.rfackifg.mutate_next(data, |reg, val| reg.mutate(val));
            }
            RFC::DBELL::RFCPEIFG::ADDR => {
                trace!("writing RFCPEIFG => {:#x}", data);
                self.rfcpeifg.mutate_next(data, |reg, val| reg.mutate(val));
            }
            RFC::DBELL::RFCPEIEN::ADDR => {
                trace!("writing RFCPEIEN => {:#x}", data);
                self.rfcpeien.mutate_next(data, |reg, val| reg.mutate(val));
            }
            RFC::DBELL::RFCPEISL::ADDR => {
                trace!("writing RFCPEISL => {:#x}", data);
                self.rfcpeisl.mutate_next(data, |reg, val| reg.mutate(val));
            }
            RFC::PWR::PWMCLKEN::ADDR => {
                trace!("writing PWMCLKEN => {:#x}", data);
                self.pwmclken.mutate_next(data, |reg, val| reg.mutate(val));
            }
            RFC::RAT::RATCNT::ADDR => {
                trace!("writing RATCNT => {:#x}", data);
                self.ratcnt.mutate_next(data, |reg, val| reg.mutate(val));
            }
            _ => todo!("writing {data:08x} to unknown register {addr:?}"),
        }
    }
}

impl<SC> RfcRegisterBank<SC>
where
    SC: Subcomponent<Member = Self>,
{
    pub(super) fn is_cpe_interrupt_enabled(&self, interrupt: Interrupt) -> bool {
        let bitmask: u32 = 1u32 << interrupt as u8;
        self.rfcpeien.read() & bitmask == bitmask
    }

    pub(super) fn get_event_for_cpe_interrupt(&self, interrupt: Interrupt) -> EventFabricEvent {
        let bitmask: u32 = 1u32 << interrupt as u8;
        match self.rfcpeisl.read() & bitmask {
            0 => EventFabricEvent::RFC_CPE_0,
            _nonzero => EventFabricEvent::RFC_CPE_1,
        }
    }

    pub(super) fn set_cpe_interrupt_pending(&mut self, interrupt: Interrupt) {
        debug_assert!(self.is_cpe_interrupt_enabled(interrupt));
        let bitmask: u32 = 1u32 << interrupt as u8;
        self.rfcpeifg
            .mutate_next(bitmask, |reg, bitmask| reg.mutate(reg.read() | bitmask));
    }

    pub(super) fn set_command_status(&mut self, value: ResultByte) {
        self.cmdsta.set_next(CommandStatusHandle::from(value));
    }

    pub(super) fn set_doorbell_interrupt_pending(&mut self) {
        self.rfackifg
            .mutate_next(1, |reg, val| reg.mut_bitfields().set_ACKFLAG(val as u8));
    }

    pub(super) const fn get_event_for_doorbell_interrupt(&self) -> EventFabricEvent {
        EventFabricEvent::RFC_CMD_ACK
    }

    pub(super) fn clean_cmdr(&mut self) {
        self.cmdr.mutate_next(0, |reg, data| *reg = data.into());
    }
}
