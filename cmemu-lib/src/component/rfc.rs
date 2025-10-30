#![cfg_attr(
    i_dont_care_about_warnings_in = "rfc",
    allow(warnings, clippy::all, clippy::pedantic)
)]
// NOTE: This is a highly experimental module in a snapshot state!

mod register_bank;
mod timing;

use crate::component::rfc::command::{
    Command, CommandData, CommandId, CommandStatus, ImmediateCommandParameters,
    RadioOperationCommandParameters,
};
pub mod command;
use crate::bridge_ports;
#[proxy_use]
use crate::common::Address;
use crate::common::CcaReq;
use crate::common::new_ahb::Size;
use crate::common::new_ahb::ports::{AHBSlavePortInput, AHBSlavePortProxiedInput};
use crate::common::new_ahb::slave_driver::WriteMode;
use crate::common::new_ahb::slave_driver::faking_slave_driver::{FakingHandler, WaitstatesOrErr};
#[proxy_use]
use crate::common::new_ahb::{
    AHBPortConfig, DataBus, MasterToSlaveWires, slave_driver::faking_slave_driver::FakingIface,
};
#[proxy_use]
use crate::component::memory_bypass::MemoryBypassReceiver;
#[proxy_use(proxy_only)]
use crate::component::rfc::RfcMemoryBypassReceiver;
use crate::component::rfc::timing::{cycles_to_ack, cycles_to_done};
#[proxy_use]
use crate::engine::Context;
use crate::engine::{
    DisableableComponent, MainComponent, SeqFlopMemoryBankSimple, SkippableClockTreeNode,
    TickComponent, TickComponentExtra,
};
use crate::proxy::{EventFabricProxy, FlashProxy, GPRAMProxy, ROMProxy, RfcProxy, SRAMProxy};
use cc2650_constants::{self as soc};
use cmemu_common::address_match_range;
use cmemu_proc_macros::{component_impl, handler, proxy_use};
use log::{debug, info, trace, warn};
use modular_bitfield::specifiers::B4;
use modular_bitfield::{BitfieldSpecifier, bitfield};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::panic::UnwindSafe;

type RfcRegisterBank = register_bank::RfcRegisterBank<RfcRegisterBankSubcomponent>;

pub trait ModemInterface {
    // TODO: This interface shouldn't limit the user to a shared reference!
    fn send_op(&self, op: ModemOp);
    fn take_rx(&self) -> Option<Vec<u8>>;
    fn take_tx_finished(&self) -> Option<()>;
    fn cca_read(&self) -> Option<CcaReq>;
    #[deprecated]
    fn strobe(&self, strobe: u8) {
        self.send_op(ModemOp::Strobe(strobe))
    }
    #[deprecated]
    fn push_to_tx_fifo(&self, byte: u8) {
        self.send_op(ModemOp::PushTx(byte))
    }
    #[deprecated]
    fn cca_prereq(&self) {
        self.send_op(ModemOp::RequestCca)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ModemOp {
    PushTx(u8),
    Strobe(u8),
    RequestCca,
    SetAddr { panid: u16, short: u16, ext: u64 },
    SetAutoAck(bool),
}

type BusDriver = FakingIface<SlaveDriverSubcomponent, RFCComponent>;

pub type ModemImpl = Box<dyn ModemInterface + Send + Sync + UnwindSafe>;

const CC26XX_DEFAULT_CHANNEL: u8 = 11;
const MAXIMUM_IEEE_TX_PAYLOAD_SIZE: usize = 256;
/// The biggest radio command is IEEE_RX
const BIGGEST_RADIO_COMMAND_SIZE: usize = 60;

#[derive(Clone, Copy, Debug)]
enum TaskHandle {
    /// CMDR is zero
    Nothing,
    /// Radio or Immediate command
    Pointer(Address),
    /// Direct command
    Direct {
        id: CommandId,
        param: u8,
        extension: u8,
    },
}

#[derive(Clone, Copy, Debug)]
struct PendingCommand {
    command: Command,
    /// Address of the Radio or Immediate command. It's None for Direct commands.
    pointer: Option<Address>,
    received_at_cycle: usize,
}

#[derive(Debug)]
struct MemoryBypassStorage {
    data: Vec<u8>,
    address: Address,
    consumed: bool,
}

impl MemoryBypassStorage {
    fn new(size: usize) -> Self {
        Self {
            data: vec![0u8; size],
            address: Address::from(0),
            consumed: true,
        }
    }
}

#[derive(MainComponent, TickComponent, TickComponentExtra)]
pub(crate) struct RFCComponent {
    #[subcomponent(SlaveDriverSubcomponent)]
    driver: BusDriver,

    #[flop]
    fake_clock: SeqFlopMemoryBankSimple<usize>,
    #[flop]
    cycles_clock: SeqFlopMemoryBankSimple<usize>,

    #[flop]
    pending_command: SeqFlopMemoryBankSimple<Option<PendingCommand>>,
    #[flop]
    current_task: SeqFlopMemoryBankSimple<TaskHandle>,

    #[subcomponent(RfcRegisterBankSubcomponent)]
    register_bank: RfcRegisterBank,

    #[flop]
    address_for_pending_cca_req: SeqFlopMemoryBankSimple<Option<Address>>,

    modem_impl: Option<ModemImpl>,

    command_storage: Option<MemoryBypassStorage>,
    payload_storage: Option<MemoryBypassStorage>,
    rx_queue_storage: Option<MemoryBypassStorage>,
    rx_entry_header_storage: Option<MemoryBypassStorage>,
    rx_buffer_to_write: Option<Vec<u8>>,

    #[flop]
    background_command: SeqFlopMemoryBankSimple<Option<(RadioOperationCommandParameters, Address)>>,
    tx_awaiting_done: Option<Address>,

    // Used externally to inform radio about events on the modem
    need_wakeup: bool,
}

#[component_impl(rfc)]
impl RFCComponent {
    pub(crate) fn new() -> Self {
        Self {
            driver: BusDriver::new(),
            fake_clock: SeqFlopMemoryBankSimple::new(0usize),
            cycles_clock: SeqFlopMemoryBankSimple::new(0usize),
            address_for_pending_cca_req: SeqFlopMemoryBankSimple::new(None),
            pending_command: SeqFlopMemoryBankSimple::new(None),
            current_task: SeqFlopMemoryBankSimple::new(TaskHandle::Nothing),
            command_storage: Some(MemoryBypassStorage::new(BIGGEST_RADIO_COMMAND_SIZE)),
            payload_storage: Some(MemoryBypassStorage::new(MAXIMUM_IEEE_TX_PAYLOAD_SIZE)),
            rx_queue_storage: Some(MemoryBypassStorage::new(8)),
            rx_entry_header_storage: Some(MemoryBypassStorage::new(8)),
            rx_buffer_to_write: None,
            register_bank: RfcRegisterBank::new(),
            background_command: SeqFlopMemoryBankSimple::new(None),
            tx_awaiting_done: None,
            modem_impl: None,
            need_wakeup: false,
        }
    }

    pub(crate) fn tick(&mut self, ctx: &mut Context) {
        self.need_wakeup = false;
        BusDriver::run_driver(self, ctx);
        self.cycles_clock
            .set_next(self.cycles_clock.overflowing_add(1).0);
        self.fake_clock_tick(ctx);
        self.try_acknowledge_task(ctx);
        self.try_run_task(ctx);
    }

    fn fake_clock_tick(&mut self, ctx: &mut Context) {
        if let Some(()) = self
            .modem_impl
            .as_ref()
            .and_then(|modem| modem.take_tx_finished())
        {
            // TODO: czy moze się nie powieść?
            let pointer = self.tx_awaiting_done.take();
            self.write_back_status_field(ctx, pointer, CommandStatus::IeeeDoneOk);
        }

        if let Some(cmd_addr) = *self.address_for_pending_cca_req {
            let maybe_cca_resp = if let Some(modem) = self.modem_impl.as_ref() {
                modem.cca_read()
            } else {
                Some(CcaReq::new_clear())
            };
            if let Some(cca_resp) = maybe_cca_resp {
                trace!("read {cca_resp:?} from modem, finishing the command");
                self.ack_command_with_status(ctx, ResultByte::Done);

                self.write_back_status_field(ctx, Some(cmd_addr), CommandStatus::DoneOk);
                let cca_resp_bytes = cca_resp.into_bytes();
                self.request_memory_write(
                    ctx,
                    cmd_addr.offset(2),
                    DataBus::from_slice(&cca_resp_bytes[0..2]),
                );
                self.request_memory_write(
                    ctx,
                    cmd_addr.offset(2 + 2),
                    DataBus::from_slice(&cca_resp_bytes[2..3]),
                );

                self.address_for_pending_cca_req.set_next(None);
            }
        }

        if let (
            Some((
                RadioOperationCommandParameters::IeeeRx {
                    p_output,
                    rx_config,
                    p_rx_q,
                    ..
                },
                addr,
            )),
            Some(modem),
        ) = (*self.background_command, self.modem_impl.as_ref())
        {
            if let Some(rx_buf) = modem.take_rx() {
                assert_eq!(usize::from(rx_buf[0]) + 1, rx_buf.len());
                assert!(
                    self.rx_buffer_to_write.is_none(),
                    "received another rx buffer before handling the previous one"
                );
                // dbg!(rx_config);
                self.rx_buffer_to_write = Some(rx_buf);
                let storage = self.rx_queue_storage.take();
                self.request_memory_read(ctx, p_rx_q, storage, 8, RfcMemoryBypassReceiver::RxQueue);
            }
        }

        // TODO(pk): [TI-TRM 23.2.1] Implement behaviour described there.
        // For immediate commands, CMDSTA becomes non 0 only after command is processed
        match *self.current_task {
            TaskHandle::Pointer(ptr) => {
                if self.pending_command.is_some() {
                    self.ack_command_with_status(ctx, ResultByte::SchedulingError);
                    return;
                }
                match &mut self.command_storage {
                    Some(MemoryBypassStorage {
                        data,
                        address,
                        consumed,
                    }) if *address == ptr && !*consumed => {
                        *consumed = true;
                        self.current_task.set_next(TaskHandle::Nothing);
                        match Command::try_from(data.as_slice()) {
                            Ok(command) => {
                                trace!("parsed command as {:?}", command);
                                if let Command {
                                    id: CommandId::IeeeTx,
                                    data:
                                        CommandData::RadioOperation {
                                            parameters:
                                                RadioOperationCommandParameters::IeeeTx {
                                                    payload_len,
                                                    p_payload,
                                                    ..
                                                },
                                            ..
                                        },
                                } = command
                                {
                                    self.request_radio_message_payload_read(
                                        ctx,
                                        Address::from(p_payload),
                                        payload_len as usize,
                                    );
                                }

                                // Command has to be persisted here, as we may need more that single cycle to process.
                                // It is the case when handling IeeeTx command as we need to fetch additional data.
                                self.pending_command.set_next(Some(PendingCommand {
                                    command,
                                    pointer: Some(ptr),
                                    received_at_cycle: *self.cycles_clock,
                                }));
                            }
                            Err(status @ (register_status, _memory_status)) => {
                                trace!("command parsing resulted in {:?}", status);
                                self.ack_command_with_status(ctx, register_status.result);
                            }
                        }
                    }
                    None => {
                        // We are waiting for data to be ready.
                    }
                    _ => {
                        // We have buffer in hand, and the data in the buffer is not for the address we are interested in,
                        // or it has been already processed meaning - it may be older than the actual data under this address.
                        self.request_radio_command_read(ctx, ptr);
                    }
                }
            }
            TaskHandle::Direct {
                id,
                param,
                extension,
            } => {
                debug_assert!(
                    self.pending_command.is_none(),
                    "overwriting pending {:?} with {:?}",
                    *self.pending_command,
                    *self.current_task
                );
                self.current_task.set_next(TaskHandle::Nothing);
                let pending_command = PendingCommand {
                    command: Command {
                        id,
                        data: CommandData::Direct { param, extension },
                    },
                    pointer: None,
                    received_at_cycle: *self.cycles_clock,
                };
                self.pending_command.set_next(Some(pending_command));
            }
            TaskHandle::Nothing => {}
        }
    }

    fn write_back_status_field(
        &self,
        ctx: &mut Context,
        command_pointer: Option<Address>,
        status: CommandStatus,
    ) {
        if let Some(pointer) = command_pointer {
            trace!(
                "writing back status field (CMDSTA) with {:?} under {:?}",
                status, pointer
            );
            let status_field = pointer.offset(2);
            let status_value: u16 = status.into();
            self.request_memory_write(ctx, status_field, status_value.into())
        } else {
            warn!(
                "Tried to write status {:?} field on empty command pointer.",
                command_pointer
            );
        }
    }

    fn run_command(
        &mut self,
        ctx: &mut Context,
        command: Command,
        pointer: Option<Address>,
    ) -> CommandStatus {
        info!("Running command: {command:?}");
        match command {
            Command {
                id: CommandId::MagicalSecretSauce | CommandId::Ping,
                data: CommandData::Immediate { .. } | CommandData::Direct { .. },
            } => {
                self.ack_command_with_status(ctx, ResultByte::Done);
                self.write_back_status_field(ctx, pointer, CommandStatus::DoneOk);
            }
            Command {
                id: CommandId::Abort,
                data: CommandData::Immediate { .. } | CommandData::Direct { .. },
            } => {
                // TODO: handle those commands somewhere up the stack - or introduce something like tiers of command handling
                // New - not processed command
                // Acked - sent ack for command
                // Done? - finished

                // I think in practice it's only used to stop IeeeRx and Cca, which we keep in totally separate place that pending_command.
                // We don't need to introduce something like queue of commands.
                self.ack_command_with_status(ctx, ResultByte::Done);
                self.write_back_status_field(ctx, pointer, CommandStatus::DoneOk);
                if let Some((cmd, cmd_addr)) = *self.background_command {
                    self.write_back_status_field(
                        ctx,
                        Some(cmd_addr),
                        if cmd.is_ieee() {
                            CommandStatus::IeeeDoneAbort
                        } else {
                            CommandStatus::DoneAbort
                        },
                    );
                    self.background_command.set_next(None);
                }
            }
            Command {
                id: CommandId::SyncStartRat,
                data:
                    CommandData::RadioOperation {
                        parameters: params @ RadioOperationCommandParameters::SyncStartRat { .. },
                        ..
                    },
            } => {
                trace!("running {:?}", &params);
                // TODO: move to actual ack
                self.ack_command_with_status(ctx, ResultByte::Done);

                self.write_back_status_field(ctx, pointer, CommandStatus::DoneOk);

                self.raise_cpe_interrupt_if_enabled(ctx, Interrupt::CommandDone);
                // TODO: Chained commands are ignore, so pretend current command is the last one.
                self.raise_cpe_interrupt_if_enabled(ctx, Interrupt::LastCommandDone);
            }
            Command {
                id: CommandId::RadioSetup,
                data:
                    CommandData::RadioOperation {
                        parameters: params @ RadioOperationCommandParameters::RadioSetup { .. },
                        ..
                    },
            } => {
                trace!("running {:?}", &params);
                trace!("scheduling XOSCON strobe");
                if let Some(modem) = self.modem_impl.as_ref() {
                    modem.strobe(Strobe::RegSxoscon.into());
                }
                self.ack_command_with_status(ctx, ResultByte::Done);
                self.write_back_status_field(ctx, pointer, CommandStatus::DoneOk);

                self.raise_cpe_interrupt_if_enabled(ctx, Interrupt::CommandDone);
                // TODO: Chained commands are ignore, so pretend current command is the last one.
                self.raise_cpe_interrupt_if_enabled(ctx, Interrupt::LastCommandDone);
            }
            Command {
                id: CommandId::SyncStopRat,
                data:
                    CommandData::RadioOperation {
                        parameters: params @ RadioOperationCommandParameters::SyncStopRat { .. },
                        ..
                    },
            } => {
                trace!("running {:?}", &params);
                self.ack_command_with_status(ctx, ResultByte::Done);
                self.write_back_status_field(ctx, pointer, CommandStatus::DoneOk);
            }
            Command {
                id: CommandId::FsPowerdown,
                data:
                    CommandData::RadioOperation {
                        parameters: params @ RadioOperationCommandParameters::FsPowerdown {},
                        ..
                    },
            } => {
                trace!("running {:?}", &params);
                self.ack_command_with_status(ctx, ResultByte::Done);
                self.write_back_status_field(ctx, pointer, CommandStatus::DoneOk);
            }
            Command {
                id: CommandId::IeeeRx,
                data:
                    CommandData::RadioOperation {
                        parameters:
                            params @ RadioOperationCommandParameters::IeeeRx {
                                channel,
                                rx_config,
                                p_rx_q,
                                p_output,
                                frame_filt_opt,
                                frame_types,
                                cca_opt,
                                cca_rssi_thr,
                                num_ext_entries,
                                num_short_entries,
                                p_ext_entry_list,
                                p_short_entry_list,
                                local_ext_address,
                                local_short_address,
                                local_pan_id,
                                end_trigger,
                                end_time,
                            },
                        ..
                    },
            } => {
                assert_eq!(rx_config.b_auto_flush_crc(), true);
                assert_eq!(rx_config.b_include_phy_hdr(), false);
                assert_eq!(rx_config.b_append_src_ind(), false);
                assert_eq!(p_ext_entry_list, 0);
                assert_eq!(p_short_entry_list, 0);
                // dbg!(
                //     frame_filt_opt,
                //     frame_types,
                //     local_ext_address,
                //     local_short_address,
                //     local_pan_id
                // );

                trace!("running {:?}", &params);
                trace!("scheduling RXON strobe");
                if let Some(modem) = self.modem_impl.as_ref() {
                    modem.send_op(ModemOp::SetAddr {
                        panid: local_pan_id,
                        short: local_short_address,
                        ext: local_ext_address,
                    });
                    modem.send_op(ModemOp::SetAutoAck(frame_filt_opt.auto_ack_en()));
                    modem.strobe(Strobe::RegSrxon.into());
                }
                self.write_back_status_field(ctx, pointer, CommandStatus::Active);
                self.ack_command_with_status(ctx, ResultByte::Done);
                self.background_command
                    .set_next(pointer.map(|addr| (params, addr)));
                // contiki will spin until it gets it or 0.001 (?) RTC seconds pass
            }
            Command {
                id: CommandId::IeeeTx,
                data:
                    CommandData::RadioOperation {
                        preamble,
                        parameters:
                            RadioOperationCommandParameters::IeeeTx {
                                tx_opt,
                                payload_len,
                                p_payload,
                                time_stamp,
                            },
                    },
            } => {
                // Acording to 23.5.3.2
                // We optionally need to append PHY header and CRC
                match &self.payload_storage {
                    Some(MemoryBypassStorage {
                        data,
                        address,
                        consumed,
                    }) if *address == Address::from(p_payload) => {
                        if let Some(modem) = self.modem_impl.as_ref() {
                            modem.push_to_tx_fifo(payload_len + 2);

                            // Write PHY header.
                            if tx_opt.b_include_phy_hdr() {
                                // 23.5.3.1
                                // As Contiki does not include PHY header in RxQueue entries, we can send anything as header byte.
                                todo!();
                                modem.push_to_tx_fifo(0u8);
                            }

                            // Write payload.
                            for byte in &data[..payload_len as usize] {
                                modem.push_to_tx_fifo(*byte);
                            }

                            // Write crc.
                            if tx_opt.b_include_crc() {
                                // TODO add actual crc
                                // I think this is the one: https://opencores.org/projects/crc802154 this is CRC16
                                // put 0 for now a hope it works
                                todo!();
                                modem.push_to_tx_fifo(0u8);
                            }

                            // Notify modem.
                            modem.strobe(Strobe::RegStxon.into());
                        } else {
                            warn!(
                                "Running IeeeTx command, but there's no modem implementation set. No data will be sent."
                            );
                        }

                        self.register_bank.set_command_status(ResultByte::Done);
                        self.write_back_status_field(ctx, pointer, CommandStatus::Active);
                        assert!(self.tx_awaiting_done.is_none());
                        self.tx_awaiting_done = pointer;

                        // When transmission of the packet starts, the trigger RAT time used for starting the modem is written to the
                        // timeStamp field by the radio CPU. This timestamp is delayed by the firmware-defined parameter
                        // startToTXRatOffset.

                        // I couldn't find any usages of timestamp in Contiki code.
                        // Timestamp is used in case of received frames.

                        match pointer {
                            Some(pointer) => {
                                let status_field = pointer.offset(20);
                                let timestamp = 0u32;
                                self.request_memory_write(ctx, status_field, timestamp.into());
                            }
                            None => {
                                panic!(
                                    "Pointer for IeeeTx should be set, as it can't be send as direct command."
                                );
                            }
                        }

                        // TODO(pk): When do ieee commands raise ack interrupt?
                        self.raise_cpe_interrupt_if_enabled(ctx, Interrupt::FgCommandDone);
                    }
                    _ => unreachable!(
                        "IeeeTx should have been prealoaded but was: {:?}",
                        self.payload_storage
                    ),
                }
                match &mut self.payload_storage {
                    Some(MemoryBypassStorage { consumed, .. }) => {
                        *consumed = true;
                    }
                    _ => unreachable!(),
                }
            }
            Command {
                id: CommandId::IeeeCcaReq,
                data:
                    CommandData::Immediate {
                        parameters: ImmediateCommandParameters::IeeeCcaReq(params),
                    },
            } => {
                trace!("running {:?}", &params);
                // TODO: should return ContextError if no ongoing RX or energy-detect scan
                trace!("requesting cca info");
                if let Some(modem) = self.modem_impl.as_ref() {
                    modem.cca_prereq();
                }
                self.address_for_pending_cca_req.set_next(pointer);
            }
            Command {
                id: CommandId::StartRat,
                data: CommandData::Direct { param, extension },
            } => {
                todo!("CommandId::StartRat");
            }

            Command {
                id: CommandId::BusRequest,
                data: CommandData::Direct { param, extension },
            } => {
                todo!("CommandId::BusRequest");
            }
            Command {
                id: CommandId::RadioSetup,
                data:
                    CommandData::RadioOperation {
                        preamble,
                        parameters,
                    },
            } => {
                todo!("CommandId::RadioSetup");
            }
            _ => {
                todo!("{command:?}")
            }
        }
        CommandStatus::DoneOk
    }

    fn get_modem(&self) -> &ModemImpl {
        self.modem_impl
            .as_ref()
            .expect("Modem impl is not set, but radio received some command.")
    }

    fn try_acknowledge_task(&mut self, ctx: &mut Context) -> bool {
        // For immediate command there is no distinction between acknowledging command and executing it -
        // command is acknowledged once its executed.
        match *self.pending_command {
            Some(PendingCommand {
                command:
                    command @ Command {
                        data: CommandData::RadioOperation { .. },
                        ..
                    },
                received_at_cycle,
                pointer: Some(cmd_addr),
            }) => {
                let (cycles_since_received, _) =
                    self.cycles_clock.overflowing_sub(received_at_cycle);
                if cycles_to_ack(&command) == cycles_since_received {
                    self.write_back_status_field(ctx, Some(cmd_addr), CommandStatus::Active);
                    self.ack_command_with_status(ctx, ResultByte::Done);
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    fn try_run_task(&mut self, ctx: &mut Context) -> bool {
        let Some(PendingCommand {
            command,
            pointer,
            received_at_cycle,
        }) = *self.pending_command
        else {
            return false;
        };

        let cycles_to_done_for_command = cycles_to_done(&command);
        let (cycles_since_received, _) = self.cycles_clock.overflowing_sub(received_at_cycle);
        if cycles_since_received < cycles_to_done_for_command {
            if cycles_since_received % 100 == 0 {
                // trace!("Command {command:?} not ready to ba handled now. Elapsed {cycles_since_received} since radio received command, but it takes {cycles_to_done_for_command} to handle this command.");
            }
            return false;
        } else if cycles_since_received > cycles_to_done_for_command {
            panic!(
                "Should run command {command:?} after {cycles_to_done_for_command} cycles, but {cycles_since_received} cycles passed since received."
            );
        }

        let command_status = self.run_command(ctx, command, pointer);
        match command_status {
            CommandStatus::DoneOk | CommandStatus::IeeeDoneOk => {
                self.pending_command.set_next(None);
            }
            _ => {}
        }
        true
    }

    pub(crate) fn tock(&mut self, ctx: &mut Context) {
        BusDriver::tock(self, ctx);
    }

    fn write_rx_contents(
        &mut self,
        ctx: &mut Context,
        entry_address: Address,
        entry_header: RxEntryHeader,
    ) {
        let Some(mut buffer) = self.rx_buffer_to_write.take() else {
            todo!("requested to write rx contents when there isn't any pending rx buffer")
        };
        let Some((
            RadioOperationCommandParameters::IeeeRx {
                p_output,
                rx_config,
                p_rx_q,
                ..
            },
            cmd_addr,
        )) = *self.background_command
        else {
            todo!("requested to write rx contents when background command isn't rx")
        };
        assert!(matches!(entry_header.status, RxEntryStatus::Pending));
        self.request_memory_write(
            ctx,
            p_rx_q.offset(0),
            entry_header.p_next_entry.to_const().into(),
        );
        // self.request_memory_write(ctx, entry_address.offset(2), DataBus::Byte(RxEntryStatus::Busy.into())); // TODO: to w tym miejscu by było trochę na prozno
        let mut offset = entry_address.offset(8);

        // Modify the message to match rx_config
        if rx_config.b_include_phy_hdr() {
            unimplemented!("requested phy hdr in rx config")
        }
        if !rx_config.b_include_crc() {
            // CRC is included in passed messages, so it must be removed
            _ = buffer.pop();
            _ = buffer.pop();
        }
        if rx_config.b_append_rssi() {
            buffer.push(0); // TODO
        }
        if rx_config.b_append_corr_crc() {
            buffer.push(0); // TODO
        }
        if rx_config.b_append_src_ind() {
            unimplemented!("requested source index in rx config")
        }
        if rx_config.b_append_timestamp() {
            buffer.push(0);
            buffer.push(0);
            buffer.push(0);
            buffer.push(0);
        }

        let mut data = &buffer[1..];
        let length = data.len();
        match entry_header.config.len_sz() {
            RxEntryLengthSize::NoIndicator => {}
            RxEntryLengthSize::OneByte => {
                self.request_memory_write(ctx, offset, DataBus::Byte(length.try_into().unwrap()));
                offset = offset.offset(1);
            }
            RxEntryLengthSize::TwoByte => {
                self.request_memory_write(ctx, offset, DataBus::Short(length.try_into().unwrap()));
                offset = offset.offset(2);
            }
            RxEntryLengthSize::Reserved => todo!("{entry_header:?}"),
        }
        while data.len() > 0 {
            let chunk_size = data.len().min(4);
            let chunk_size = if chunk_size.is_power_of_two() {
                chunk_size
            } else {
                chunk_size.next_power_of_two() / 2
            };
            self.request_memory_write(ctx, offset, DataBus::from_slice(&data[0..chunk_size]));
            offset = offset.offset(chunk_size.try_into().unwrap());
            data = &data[chunk_size..];
        }
        self.request_memory_write(
            ctx,
            entry_address.offset(4),
            DataBus::Byte(RxEntryStatus::Finished.into()),
        );
        // self.write_back_status_field(ctx, Some(cmd_addr), CommandStatus::DoneOk);
        // self.raise_cpe_interrupt_if_enabled(ctx, Interrupt::CommandDone);
        // self.raise_cpe_interrupt_if_enabled(ctx, Interrupt::RxOk);
        self.raise_cpe_interrupt_if_enabled(ctx, Interrupt::RxEntryDone);
    }

    #[handler]
    pub(crate) fn on_new_ahb_slave_input(
        &mut self,
        ctx: &mut Context,
        msg: MasterToSlaveWires<<RFCComponent as AHBPortConfig>::Data>,
    ) {
        <Self as AHBSlavePortInput>::on_ahb_input(self, ctx, msg);
    }

    pub(crate) fn set_interface(&mut self, interface: Option<ModemImpl>) {
        self.modem_impl = interface;
    }

    pub(crate) fn notify_radio_wakeup(&mut self) {
        self.need_wakeup = true;
    }
}

// Reading and writing memory.
#[component_impl(rfc)]
impl RFCComponent {
    fn request_memory_read(
        &self,
        ctx: &mut Context,
        start_address: Address,
        storage: Option<MemoryBypassStorage>,
        size: usize,
        subreceiver: RfcMemoryBypassReceiver,
    ) {
        trace!("requesting memory bypass {:?}", start_address);

        match storage {
            Some(MemoryBypassStorage { data, consumed, .. }) => {
                if !consumed {
                    warn!(
                        "Request read on non consumed storage for receiver: {:?}.",
                        subreceiver
                    );
                }

                let receiver = MemoryBypassReceiver::Rfc(subreceiver);
                address_match_range!(start_address,
                    soc::FLASH::ADDR_SPACE => FlashProxy.request_memory_read_bypass(ctx, start_address, data, size, receiver),
                    soc::GPRAM::ADDR_SPACE => GPRAMProxy.request_memory_read_bypass(ctx, start_address, data, size, receiver),
                    soc::SRAM::ADDR_SPACE => SRAMProxy.request_memory_read_bypass(ctx, start_address, data, size, receiver),
                    soc::BROM::ADDR_SPACE => ROMProxy.request_memory_read_bypass(ctx, start_address, data, size, receiver),
                    _ => todo!("attempting to read memory from uhhh not memory? {start_address:?}"),
                );
            }
            None => {
                panic!(
                    "There is read requested already for this receiver: {:?}.",
                    subreceiver
                )
            }
        }
    }

    fn request_radio_command_read(&mut self, ctx: &mut Context, start_address: Address) {
        let storage = self.command_storage.take();
        self.request_memory_read(
            ctx,
            start_address,
            storage,
            BIGGEST_RADIO_COMMAND_SIZE,
            RfcMemoryBypassReceiver::Command,
        );
    }

    fn request_radio_message_payload_read(
        &mut self,
        ctx: &mut Context,
        start_address: Address,
        size: usize,
    ) {
        let storage = self.payload_storage.take();
        self.request_memory_read(
            ctx,
            start_address,
            storage,
            size,
            RfcMemoryBypassReceiver::Payload,
        );
    }

    fn request_memory_write(&self, ctx: &mut Context, start_address: Address, data: DataBus) {
        trace!(
            "requesting memory bypass write 0x{:?}<-{:?}",
            start_address, &data,
        );

        address_match_range!(start_address,
            soc::FLASH::ADDR_SPACE => FlashProxy.request_memory_write_bypass(ctx, start_address, data),
            soc::GPRAM::ADDR_SPACE => GPRAMProxy.request_memory_write_bypass(ctx, start_address, data),
            soc::SRAM::ADDR_SPACE => SRAMProxy.request_memory_write_bypass(ctx, start_address, data),
            soc::BROM::ADDR_SPACE => ROMProxy.request_memory_write_bypass(ctx, start_address, data),
            _ => todo!("attempting to write memory to uhhh not memory? {start_address:?}"),
        );
    }

    #[handler]
    pub(crate) fn receive_memory_bypass(
        &mut self,
        ctx: &mut Context,
        data: Vec<u8>,
        address: Address,
        receiver: RfcMemoryBypassReceiver,
    ) {
        trace!("Received memory bypass for address: 0x{address:?} and receiver: {receiver:?}");
        let mut storage = MemoryBypassStorage {
            data,
            address,
            consumed: false,
        };
        match receiver {
            RfcMemoryBypassReceiver::Command => {
                self.command_storage = Some(storage);
            }
            RfcMemoryBypassReceiver::Payload => {
                self.payload_storage = Some(storage);
            }
            RfcMemoryBypassReceiver::RxQueue => {
                // quick validation
                assert_eq!(storage.data.len(), 8);
                assert_eq!(
                    &storage.data[4..8],
                    &[0u8; 4],
                    "we only support circular data entry queues for now"
                );
                assert_ne!(
                    &storage.data[0..4],
                    &[0u8; 4],
                    "the data entry queue must not be empty"
                );
                let entry_address =
                    Address::from(u32::from_le_bytes(storage.data[0..4].try_into().unwrap()));
                storage.consumed = true;
                self.rx_queue_storage = Some(storage);
                let storage = self.rx_entry_header_storage.take();
                self.request_memory_read(
                    ctx,
                    entry_address,
                    storage,
                    8,
                    RfcMemoryBypassReceiver::RxEntryHeader,
                );
            }
            RfcMemoryBypassReceiver::RxEntryHeader => {
                let data: [u8; 8] = storage.data[0..8].try_into().unwrap();
                let entry_header = RxEntryHeader::from(data);
                let entry_address = storage.address;
                storage.consumed = true;
                // dbg!(entry_address, entry_header);
                self.rx_entry_header_storage = Some(storage);
                self.write_rx_contents(ctx, entry_address, entry_header);
            }
        }
    }
}

/// [TI-TRM-I] 23.3.2.7.2 Data Entry
/// [TI-TRM-I] Table 23-10. General Data Entry Structure
#[derive(Clone, Copy, Debug)]
struct RxEntryHeader {
    p_next_entry: Address,
    status: RxEntryStatus,
    config: RxEntryConfig,
    length: u16,
}

impl From<[u8; 8]> for RxEntryHeader {
    fn from(value: [u8; 8]) -> Self {
        let p_next_entry = Address::from(u32::from_le_bytes(value[0..4].try_into().unwrap()));
        let status = RxEntryStatus::try_from_primitive(value[4]).unwrap();
        let config = RxEntryConfig::from_bytes(value[5..6].try_into().unwrap());
        let length = u16::from_le_bytes(value[6..8].try_into().unwrap());
        Self {
            p_next_entry,
            status,
            config,
            length,
        }
    }
}

/// [TI-TRM-I] 23.3.2.7.2 Data Entry
#[derive(Clone, Copy, Debug, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
enum RxEntryStatus {
    Pending = 0,
    Active = 1,
    Busy = 2,
    Finished = 3,
}

#[bitfield]
#[derive(Clone, Copy, Debug)]
struct RxEntryConfig {
    r#type: RxEntryType,
    len_sz: RxEntryLengthSize,
    irq_intv: B4,
}

#[derive(BitfieldSpecifier, Clone, Copy, Debug)]
#[bits = 2]
enum RxEntryType {
    General = 0,
    Multielement = 1,
    Pointer = 2,
}

#[derive(BitfieldSpecifier, Clone, Copy, Debug)]
#[bits = 2]
enum RxEntryLengthSize {
    NoIndicator = 0,
    OneByte = 1,
    TwoByte = 2,
    Reserved = 3,
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum RfcMemoryBypassReceiver {
    Command,
    Payload,
    RxQueue,
    RxEntryHeader,
}

bridge_ports!(@slave RFCComponent => @auto_configured @slave BusDriver);

#[component_impl(rfc)]
impl AHBPortConfig for RFCComponent {
    type Data = DataBus;
    type Component = Self;
    const TAG: &'static str = "RFC";
}

#[component_impl(rfc)]
impl AHBSlavePortProxiedInput for RFCComponent {
    fn proxy_ahb_input(ctx: &mut Context, msg: MasterToSlaveWires<Self::Data>) {
        RfcProxy.on_new_ahb_slave_input(ctx, msg);
    }
}

#[component_impl(rfc)]
impl FakingHandler for RFCComponent {
    const WRITE_MODE: WriteMode = WriteMode::Combinatorial;

    fn pre_read(
        comp: &mut Self::Component,
        _ctx: &mut Context,
        address: Address,
        _size: Size,
    ) -> WaitstatesOrErr {
        Ok(comp.get_waitstates_for_address(address, false) as u8)
    }

    fn read(
        comp: &mut Self::Component,
        _ctx: &mut Context,
        address: Address,
        _size: Size,
    ) -> Self::Data {
        DataBus::from(comp.get_data_for_address(address))
    }

    fn pre_write(
        comp: &mut Self::Component,
        _ctx: &mut Context,
        address: Address,
        _size: Size,
    ) -> WaitstatesOrErr {
        Ok(comp.get_waitstates_for_address(address, true) as u8)
    }

    fn write(comp: &mut Self::Component, ctx: &mut Context, address: Address, data: Self::Data) {
        // TODO: use aligner (??)
        comp.set_data_for_address(ctx, address, data.into());
    }
}

// Talking with registers
#[component_impl(rfc)]
impl RFCComponent {
    fn get_data_for_address(&mut self, addr: Address) -> u32 {
        self.register_bank.get_data_for_address(addr)
    }

    fn set_data_for_address(&mut self, ctx: &mut Context, addr: Address, data: u32) {
        self.register_bank.set_data_for_address(ctx, addr, data);
    }

    fn get_waitstates_for_address(&self, _addr: Address, _writing: bool) -> u32 {
        (10 - 1 - *self.fake_clock).try_into().unwrap_or(0u32)
    }

    #[handler]
    pub fn on_non_zero_cmdr_write(&mut self, ctx: &mut Context, cmdr: u32) {
        let task = match cmdr & 0b11 {
            0b01 => {
                let numeric_id = u16::try_from(cmdr >> 16).unwrap();
                let Ok(id) = CommandId::try_from(numeric_id) else {
                    todo!(
                        "received invalid CMDR value {:08x} (unknown command id {:04x})",
                        cmdr,
                        numeric_id
                    );
                };
                let param = u8::try_from((cmdr & 0xf0) >> 8).unwrap();
                let extension = u8::try_from((cmdr & 0b11111100) >> 2).unwrap();
                TaskHandle::Direct {
                    id,
                    param,
                    extension,
                }
            }
            0b00 => TaskHandle::Pointer(Address::from_const(cmdr)),
            0b10 | 0b11 => todo!("received invalid CMDR value: {:08x}", cmdr),
            _ => unreachable!(),
        };
        self.current_task.set_next(task);
        self.register_bank.set_command_status(ResultByte::Pending);
    }
}

#[component_impl(rfc)]
impl RFCComponent {
    fn raise_cpe_interrupt_if_enabled(&mut self, ctx: &mut Context, interrupt: Interrupt) {
        if self.register_bank.is_cpe_interrupt_enabled(interrupt) {
            self.register_bank.set_cpe_interrupt_pending(interrupt);
            EventFabricProxy.notify(
                ctx,
                self.register_bank.get_event_for_cpe_interrupt(interrupt),
            );
        }
    }

    /// Writes to CMDSTA register, zeros CMDR register sets interrupt flag and send interrupt event to EventFabric.
    fn ack_command_with_status(&mut self, ctx: &mut Context, status: ResultByte) {
        self.register_bank.clean_cmdr();
        self.register_bank.set_command_status(status);
        self.register_bank.set_doorbell_interrupt_pending();
        EventFabricProxy.notify(ctx, self.register_bank.get_event_for_doorbell_interrupt());
    }

    // fn raise_interrupt_if_enabled(&mut self, ) {
    //
    // }
}

#[component_impl(rfc)]
impl DisableableComponent for RFCComponent {
    fn can_be_disabled_now(&self) -> bool {
        // TODO: this is implementation of "can skip cycles", not whether can be disabled completely!
        let subs = self.driver.can_be_disabled_now() && self.register_bank.can_be_disabled_now();
        subs && self.pending_command.is_empty()
            && self.pending_command.is_none()
            && self.current_task.is_empty()
            && matches!(*self.current_task, TaskHandle::Nothing)
            && self.address_for_pending_cca_req.is_empty()
            && self.address_for_pending_cca_req.is_none()
            && self.background_command.is_empty()
            && self.tx_awaiting_done.is_none()
            // storages are not pending
            && self.command_storage.is_some()
            && self.payload_storage.is_some()
            && self.rx_queue_storage.is_some()
            && self.rx_entry_header_storage.is_some()
            // writing not pending
            && self.rx_buffer_to_write.is_none()
    }
}

#[component_impl(rfc)]
impl SkippableClockTreeNode for RFCComponent {
    fn max_cycles_to_skip(
        comp: &mut Self::Component,
        _ctx: &mut Context,
        _parent: Self::IdSpace,
        _extra: &mut Self::Extra,
    ) -> u64 {
        // TODO: triggers and timing
        if comp.need_wakeup || !comp.can_be_disabled_now() {
            debug!(
                "RFC cannot be disabled now: bg: {:?}, pend: {:?}",
                comp.background_command, comp.pending_command
            );
            0
        } else {
            u64::MAX
        }
    }

    fn emulate_skipped_cycles(
        comp: &mut Self::Component,
        ctx: &mut Context,
        _parent: Self::IdSpace,
        _extra: &mut Self::Extra,
        skipped_cycles: u64,
    ) {
        let this = comp;
        // todo: fast forward internal tickers
        // this.cycles_clock
        //     .set_next(this.cycles_clock.wrapping_add(skipped_cycles as usize));
    }
}

/// [TI-TRM-I] Table 23-1. Values of the Result Byte in the CMDSTA Register
#[derive(Debug, PartialEq, Clone, Copy, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum ResultByte {
    Pending = 0x00,
    Done = 0x01,
    IllegalPointer = 0x81,
    UnknownCommand = 0x82,
    UnknownDirCommand = 0x83,
    ContextError = 0x85,
    SchedulingError = 0x86,
    ParError = 0x87,
    QueueError = 0x88,
    QueueBusy = 0x89,
}

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

impl TryFrom<TaskHandle> for Command {
    type Error = (CommandStatusHandle, CommandStatus);

    fn try_from(value: TaskHandle) -> Result<Self, Self::Error> {
        let TaskHandle::Direct {
            id,
            param,
            extension,
        } = value
        else {
            unreachable!("Can't parse {:?} into Command.", value)
        };
        if !id.is_direct() {
            return Result::Err((
                CommandStatusHandle::from(ResultByte::UnknownCommand),
                CommandStatus::ErrorCmdId,
            ));
        }
        Ok(Command {
            id,
            data: CommandData::Direct { param, extension },
        })
    }
}

#[derive(Debug, PartialEq, Clone, Copy, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
enum Strobe {
    RegSxoscon = 0x01,
    RegSrxon = 0x03,
    RegStxon = 0x04,
}

/// [TI-TRM-I] 23.8.2.5 Command and Packet Engine Generated Interrupts
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
enum Interrupt {
    InternalError = 31,
    BootDone = 30,
    ModulesUnlocked = 29,
    SynthNoLock = 28,
    IRQ27 = 27,
    RxAborted = 26,
    RxNDataWritten = 25,
    RxDataWritten = 24,
    RxEntryDone = 23,
    RxBufFull = 22,
    RxCtrlAck = 21,
    RxCtrl = 20,
    RxEmpty = 19,
    RxIgnored = 18,
    RxNok = 17,
    RxOk = 16,
    IRQ15 = 15,
    IRQ14 = 14,
    IRQ13 = 13,
    IRQ12 = 12,
    TxBufferChanged = 11,
    TxEntryDone = 10,
    TxRetrans = 9,
    TxCtrlAckAck = 8,
    TxCtrlAck = 7,
    TxCtrl = 6,
    TxAck = 5,
    TxDone = 4,
    LastFgCommandDone = 3,
    FgCommandDone = 2,
    LastCommandDone = 1,
    CommandDone = 0,
}
