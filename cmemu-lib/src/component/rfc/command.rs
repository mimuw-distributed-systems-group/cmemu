use crate::component::rfc::{CommandStatusHandle, ResultByte};
use cmemu_common::Address;
use modular_bitfield::prelude::*;
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Clone, Copy, Debug)]
pub(super) struct Command {
    pub(super) id: CommandId,
    pub(super) data: CommandData,
}

/// [TI-TRM-I] Table 23-2. Common Radio Operation Status Codes
#[derive(Debug, PartialEq, Clone, Copy, IntoPrimitive, TryFromPrimitive)]
#[repr(u16)]
pub(super) enum CommandStatus {
    // Operation not finished
    /// Operation has not started.
    Idle = 0x0000,
    /// Waiting for a start trigger.
    Pending = 0x0001,
    /// Running an operation.
    Active = 0x0002,
    /// Operation skipped due to condition in another command.
    Skipped = 0x0003,

    // Operation Finished Normally
    /// Operation ended normally.
    DoneOk = 0x0400,
    /// Counter reached zero.
    DoneCountdown = 0x0401,
    /// Operation ended with CRC error.
    DoneRxErr = 0x0402,
    /// Operation ended with time-out.
    DoneTimeout = 0x0403,
    /// Operation stopped after CMD_STOP command.
    DoneStopped = 0x0404,
    /// Operation aborted by CMD_ABORT command.
    DoneAbort = 0x0405,

    // Operation Finished With Error
    /// The start trigger occurred in the past.
    ErrorPastStart = 0x0800,
    /// Illegal start trigger parameter
    ErrorStartTrig = 0x0801,
    /// Illegal condition for next operation
    ErrorCondition = 0x0802,
    /// Error in a command specific parameter
    ErrorPar = 0x0803,
    /// Invalid pointer to next operation
    ErrorPointer = 0x0804,
    /// The next operation has a command ID that is undefined or not a radio operation command.
    ErrorCmdId = 0x0805,
    /// Operation using RX, TX, or synthesizer attempted without
    ErrorNoSetup = 0x0807,
    /// Operation using RX or TX attempted without the synthesizer being programmed or powered on.
    ErrorNoFs = 0x0808,
    /// Synthesizer programming failed.
    ErrorSynthProg = 0x0809,
    /// Modem TX underflow observed.
    ErrorTxUnf = 0x080A,
    /// Modem RX overflow observed.
    ErrorRxOvf = 0x080B,
    /// Data requested from last RX when no such data exists.
    ErrorNoRx = 0x080C,

    IeeeSuspended = 0x2001,
    // Operation ended normally
    IeeeDoneOk = 0x2400,
    // CSMA-CA operation ended with failure
    IeeeDoneBusy = 0x2401,
    // Operation stopped after stop command
    IeeeDoneStopped = 0x2402,
    // ACK packet received with pending data bit cleared
    IeeeDoneAck = 0x2403,
    // ACK packet received with pending data bit set
    IeeeDoneAckpend = 0x2404,
    // Operation ended due to time-out
    IeeeDoneTimeout = 0x2405,
    // FG operation ended because necessary background level operation ended
    IeeeDoneBgend = 0x2406,
    // Operation aborted by command
    IeeeDoneAbort = 0x2407,
    // Foreground level operation is not compatible with running
    ErrorWrongBg = 0x0806,
    // Illegal parameter
    IeeeErrorPar = 0x2800,
    // Radio was not set up in IEEE 802.15.4 mode
    IeeeErrorNoSetup = 0x2801,
    // Synthesizer was not programmed when running RX or TX
    IeeeErrorNoFs = 0x2802,
    // Synthesizer programming failed
    IeeeErrorSynthProg = 0x2803,
    // RX overflow observed during operation
    IeeeErrorRxovf = 0x2804,
    // TX underflow observed during operation
    IeeeErrorTxunf = 0x2805,
}
/// [TI-TRM-I] 23.3.3 Command Definitions
#[derive(Debug, PartialEq, Clone, Copy, IntoPrimitive, TryFromPrimitive)]
#[repr(u16)]
pub(super) enum CommandId {
    Nop = 0x0801,
    RadioSetup = 0x0802,
    FsPowerup = 0x080C,
    FsPowerdown = 0x080D,
    Fs = 0x0803,
    FsOff = 0x0804,
    RxTest = 0x0807,
    TxTest = 0x0808,
    SyncStopRat = 0x0809,
    SyncStartRat = 0x080A,
    Count = 0x080B,
    SchImm = 0x0810,
    CountBranch = 0x0812,
    PatternCheck = 0x0813,

    Abort = 0x0401,
    Stop = 0x0402,
    GetRssi = 0x0403,
    UpdateRadioSetup = 0x0001,
    Trigger = 0x0404,
    GetFwInfo = 0x0002,
    StartRat = 0x0405,
    Ping = 0x0406,
    ReadRfreg = 0x0601,
    SetRatCmp = 0x000A,
    SetRatCpt = 0x0603,
    DisableRatCh = 0x0408,
    SetRatOutput = 0x0604,
    ArmRatCh = 0x0409,
    DisarmRatCh = 0x040A,
    SetTxPower = 0x0010,
    UpdateFs = 0x0011,
    ModifyFs = 0x0013,
    BusRequest = 0x040E,

    AddDataEntry = 0x0005,
    RemoveDataEntry = 0x0006,
    FlushQueue = 0x0007,
    ClearRx = 0x0008,
    RemovePendingEntries = 0x0009,

    /// [TI-TRM] 23.5 IEEE 802.15.4
    IeeeRx = 0x2801,
    IeeeEdScan = 0x2802,
    IeeeTx = 0x2C01,
    IeeeCsma = 0x2C02,
    IeeeRxAck = 0x2C03,
    IeeeAbortBg = 0x2C04,
    IeeeModCca = 0x2001,
    IeeeModFilt = 0x2002,
    IeeeModSrcMatch = 0x2003,
    IeeeAbortFg = 0x2401,
    IeeeStopFg = 0x2402,
    IeeeCcaReq = 0x2403,

    /// <https://github.com/contiki-ng/contiki-ng/blob/482e65555a600df848a120ce3addeb4e8a7db126/arch/cpu/cc26x0-cc13x0/rf-core/rf-core.c#L142>
    MagicalSecretSauce = 0x0607,
}
impl CommandId {
    pub(super) fn is_immediate(self) -> bool {
        matches!(
            self,
            CommandId::Abort
                | CommandId::Stop
                | CommandId::GetRssi
                | CommandId::UpdateRadioSetup
                | CommandId::Trigger
                | CommandId::GetFwInfo
                | CommandId::StartRat
                | CommandId::Ping
                | CommandId::ReadRfreg
                | CommandId::SetRatCmp
                | CommandId::SetRatCpt
                | CommandId::DisableRatCh
                | CommandId::SetRatOutput
                | CommandId::ArmRatCh
                | CommandId::DisarmRatCh
                | CommandId::SetTxPower
                | CommandId::UpdateFs
                | CommandId::ModifyFs
                | CommandId::BusRequest
                | CommandId::AddDataEntry
                | CommandId::RemoveDataEntry
                | CommandId::FlushQueue
                | CommandId::ClearRx
                | CommandId::RemovePendingEntries
                | CommandId::IeeeModCca
                | CommandId::IeeeModFilt
                | CommandId::IeeeModSrcMatch
                | CommandId::IeeeAbortFg
                | CommandId::IeeeStopFg
                | CommandId::IeeeCcaReq
                | CommandId::MagicalSecretSauce
        )
    }

    pub(super) fn is_direct(&self) -> bool {
        // TODO(pk): Add those:
        // - CMD_PROP_SET_LEN
        // - CMD_PROP_RESTART_RX

        matches!(
            self,
            CommandId::Abort
                | CommandId::Stop
                | CommandId::GetRssi
                | CommandId::Trigger
                | CommandId::StartRat
                | CommandId::Ping
                | CommandId::ReadRfreg
                | CommandId::SetRatCpt
                | CommandId::DisableRatCh
                | CommandId::SetRatOutput
                | CommandId::ArmRatCh
                | CommandId::DisarmRatCh
                | CommandId::BusRequest
                | CommandId::IeeeAbortFg
                | CommandId::IeeeStopFg
        )
    }

    pub(super) fn is_radio(&self) -> bool {
        !self.is_direct() && !self.is_immediate()
    }

    /// Returns number of bytes occupied command struct.
    fn size(self) -> usize {
        match self {
            CommandId::Nop => 14,
            CommandId::RadioSetup => 24,
            CommandId::FsPowerup => 20,
            CommandId::FsPowerdown => 14,
            CommandId::Fs => 24,
            CommandId::FsOff => 14,
            CommandId::RxTest => 24,
            CommandId::TxTest => 28,
            CommandId::SyncStopRat => 20,
            CommandId::SyncStartRat => 20,
            CommandId::Count => 16,
            CommandId::SchImm => 24,
            CommandId::CountBranch => 20,
            CommandId::PatternCheck => 32,

            CommandId::Abort => 2,
            CommandId::Stop => 2,
            CommandId::GetRssi => 2,
            CommandId::UpdateRadioSetup => 8,
            CommandId::Trigger => 3,
            CommandId::GetFwInfo => 10,
            CommandId::StartRat => 2,
            CommandId::Ping => 2,
            CommandId::ReadRfreg => 8,
            CommandId::SetRatCmp => 8,
            CommandId::SetRatCpt => 4,
            CommandId::DisableRatCh => 3,
            CommandId::SetRatOutput => 4,
            CommandId::ArmRatCh => 3,
            CommandId::DisarmRatCh => 3,
            CommandId::SetTxPower => 4,
            CommandId::UpdateFs => 18,
            CommandId::ModifyFs => 6,
            CommandId::BusRequest => 3,

            CommandId::AddDataEntry => 12,
            CommandId::RemoveDataEntry => 12,
            CommandId::FlushQueue => 12,
            CommandId::ClearRx => 8,
            CommandId::RemovePendingEntries => 12,

            CommandId::IeeeRx => 60,
            CommandId::IeeeEdScan => 24,
            CommandId::IeeeTx => 24,
            CommandId::IeeeCsma => 32,
            CommandId::IeeeRxAck => 20,
            CommandId::IeeeAbortBg => 14,
            CommandId::IeeeModCca => 4,
            CommandId::IeeeModFilt => 4,
            CommandId::IeeeModSrcMatch => 4,
            CommandId::IeeeAbortFg => 2,
            CommandId::IeeeStopFg => 2,
            CommandId::IeeeCcaReq => 5,

            CommandId::MagicalSecretSauce => 2, // TODO
        }
    }
}

impl TryFrom<&[u8]> for Command {
    type Error = (CommandStatusHandle, CommandStatus);

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let id_bytes: [u8; 2] = value[0..=1].try_into().unwrap();
        let id: CommandId = u16::from_le_bytes(id_bytes).try_into().map_err(|_| {
            (
                CommandStatusHandle::from(ResultByte::UnknownCommand),
                CommandStatus::ErrorCmdId,
            )
        })?;
        let data = if id.is_immediate() {
            let parameters = match id {
                CommandId::Abort => ImmediateCommandParameters::Abort {},
                CommandId::Stop => ImmediateCommandParameters::Stop {},
                CommandId::UpdateRadioSetup => ImmediateCommandParameters::UpdateRadioSetup {
                    p_reg_override: parse_u32(&value[4..=7]),
                },
                CommandId::Trigger => ImmediateCommandParameters::Trigger {
                    trigger_no: value[2],
                },
                CommandId::GetFwInfo => ImmediateCommandParameters::GetFwInfo {
                    version_no: parse_u16(&value[2..=3]),
                    start_offset: parse_u16(&value[4..=5]),
                    free_ram_sz: parse_u16(&value[6..=7]),
                    avail_rat_ch: parse_u16(&value[8..=9]),
                },
                CommandId::StartRat => ImmediateCommandParameters::StartRat {},
                CommandId::Ping => ImmediateCommandParameters::Ping {},
                CommandId::ReadRfreg => ImmediateCommandParameters::ReadRfreg {
                    address: parse_u16(&value[2..=3]),
                    value: parse_u32(&value[4..=7]),
                },
                CommandId::SetRatCmp => ImmediateCommandParameters::SetRatCmp {
                    rat_ch: value[2],
                    compare_time: parse_u32(&value[4..=7]),
                },
                CommandId::SetRatCpt => ImmediateCommandParameters::SetRatCpt {
                    config: parse_u16(&value[2..=3]),
                },
                CommandId::DisableRatCh => {
                    ImmediateCommandParameters::DisableRatCh { rat_ch: value[2] }
                }
                CommandId::SetRatOutput => ImmediateCommandParameters::SetRatOutput {
                    config: parse_u16(&value[2..=3]),
                },
                CommandId::ArmRatCh => ImmediateCommandParameters::ArmRatCh { rat_ch: value[2] },
                CommandId::SetTxPower => ImmediateCommandParameters::SetTxPower {
                    tx_power: parse_u16(&value[2..=3]),
                },
                CommandId::UpdateFs => ImmediateCommandParameters::UpdateFs {
                    frequency: parse_u16(&value[14..=15]),
                    fract_freq: parse_u16(&value[16..=17]),
                },
                CommandId::ModifyFs => ImmediateCommandParameters::ModifyFs {
                    frequency: parse_u16(&value[2..=3]),
                    fract_freq: parse_u16(&value[4..=5]),
                },
                CommandId::BusRequest => ImmediateCommandParameters::BusRequest {
                    b_sys_bus_needed: value[2],
                },
                CommandId::AddDataEntry => ImmediateCommandParameters::AddDataEntry {
                    p_queue: parse_u32(&value[4..=7]),
                    p_entry: parse_u32(&value[8..=11]),
                },
                CommandId::RemoveDataEntry => ImmediateCommandParameters::RemoveDataEntry {
                    p_queue: parse_u32(&value[4..=7]),
                    p_entry: parse_u32(&value[8..=11]),
                },
                CommandId::FlushQueue => ImmediateCommandParameters::FlushQueue {
                    p_queue: parse_u32(&value[4..=7]),
                    p_first_entry: parse_u32(&value[8..=11]),
                },
                CommandId::ClearRx => ImmediateCommandParameters::ClearRx {
                    p_queue: parse_u32(&value[4..=7]),
                },
                CommandId::RemovePendingEntries => {
                    ImmediateCommandParameters::RemovePendingEntries {
                        p_queue: parse_u32(&value[4..=7]),
                        p_first_entry: parse_u32(&value[8..=11]),
                    }
                }
                CommandId::IeeeModCca => ImmediateCommandParameters::IeeeModCca {
                    new_cca_opt: value[2],
                    new_cca_rssi_thr: value[3],
                },
                CommandId::IeeeModFilt => ImmediateCommandParameters::IeeeModFilt {
                    new_frame_filt_opt: parse_u16(&value[2..=3]),
                    new_frame_types: value[4],
                },
                CommandId::IeeeModSrcMatch => ImmediateCommandParameters::IeeeModSrcMatch {
                    options: value[2],
                    entry_no: value[3],
                },
                CommandId::IeeeAbortFg => ImmediateCommandParameters::IeeeAbortFg {},
                CommandId::IeeeStopFg => ImmediateCommandParameters::IeeeStopFg {},
                CommandId::IeeeCcaReq => ImmediateCommandParameters::IeeeCcaReq(
                    CcaReq::from_bytes(value[2..][..3].try_into().unwrap()),
                ),
                _ => unreachable!("Unknown immediate command: {:?}", id),
            };
            CommandData::Immediate { parameters }
        } else {
            let status_bytes: [u8; 2] = value[2..=3].try_into().unwrap();
            let status: CommandStatus = u16::from_le_bytes(status_bytes)
                .try_into()
                .map_err(|_| todo!())?;
            let p_next_op_bytes: [u8; 4] = value[4..=7].try_into().unwrap();
            let p_next_op = u32::from_le_bytes(p_next_op_bytes)
                .try_into()
                .map_err(|_| todo!())
                .map(|addr| Address::from_const(addr))?;
            let start_time_bytes: [u8; 4] = value[8..=11].try_into().unwrap();
            let start_time: u32 = u32::from_le_bytes(start_time_bytes)
                .try_into()
                .map_err(|_| todo!())?;
            let start_trigger = StartTrigger::from_bytes(value[12..13].try_into().unwrap());
            let condition = Condition::from_bytes(value[13..14].try_into().unwrap());
            let preamble = RadioOperationCommandPreamble {
                status,
                p_next_op,
                start_time,
                start_trigger,
                condition,
            };
            let parameters = match id {
                CommandId::Nop => RadioOperationCommandParameters::Nop {},
                CommandId::RadioSetup => RadioOperationCommandParameters::RadioSetup {
                    mode: value[14],
                    io_divider: value[15],
                    config: parse_u16(&value[16..=17]),
                    tx_power: parse_u16(&value[18..=19]),
                    p_reg_override: parse_u32(&value[20..=23]),
                },
                CommandId::FsPowerup => RadioOperationCommandParameters::FsPowerup {
                    p_reg_override: parse_u32(&value[16..=19]),
                },
                CommandId::FsPowerdown => RadioOperationCommandParameters::FsPowerdown {},
                CommandId::Fs => RadioOperationCommandParameters::Fs {
                    frequency: parse_u16(&value[14..=15]),
                    fract_freq: parse_u16(&value[16..=17]),
                    synth_conf: value[18],
                },
                CommandId::FsOff => RadioOperationCommandParameters::FsOff {},
                CommandId::RxTest => RadioOperationCommandParameters::RxTest {
                    config: value[14],
                    end_trigger: value[15],
                    sync_word: parse_u32(&value[16..=19]),
                    end_time: parse_u32(&value[20..=23]),
                },
                CommandId::TxTest => RadioOperationCommandParameters::TxTest {
                    config: value[14],
                    tx_word: parse_u16(&value[16..=17]),
                    end_trigger: value[19],
                    sync_word: parse_u32(&value[20..=23]),
                    end_time: parse_u32(&value[24..=27]),
                },
                CommandId::SyncStopRat => RadioOperationCommandParameters::SyncStopRat {
                    rat0: parse_u32(&value[16..=19]),
                },
                CommandId::SyncStartRat => RadioOperationCommandParameters::SyncStartRat {
                    rat0: parse_u32(&value[16..=19]),
                },
                CommandId::Count => RadioOperationCommandParameters::Count {
                    counter: parse_u16(&value[14..=15]),
                },
                CommandId::SchImm => RadioOperationCommandParameters::SchImm {
                    cmdr_val: parse_u32(&value[16..=19]),
                    cmdsta_val: parse_u32(&value[20..=23]),
                },
                CommandId::CountBranch => RadioOperationCommandParameters::CountBranch {
                    counter: parse_u16(&value[14..=15]),
                    p_next_op_if_ok: parse_u32(&value[16..=19]),
                },
                CommandId::PatternCheck => RadioOperationCommandParameters::PatternCheck {
                    pattern_opt: parse_u16(&value[14..=15]),
                    p_next_op_if_ok: parse_u32(&value[16..=19]),
                    p_value: parse_u32(&value[20..=23]),
                    mask: parse_u32(&value[24..=27]),
                    compare_val: parse_u32(&value[28..=31]),
                },
                CommandId::IeeeRx => RadioOperationCommandParameters::IeeeRx {
                    channel: value[14],
                    rx_config: IeeeRxConfig::from_bytes([value[15]]),
                    p_rx_q: Address::from(parse_u32(&value[16..=19])),
                    p_output: Address::from(parse_u32(&value[20..=23])),
                    frame_filt_opt: FrameFilteringConfiguration::from_bytes(
                        value[24..=25].try_into().unwrap(),
                    ),
                    frame_types: value[26],
                    cca_opt: value[27],
                    cca_rssi_thr: value[28],
                    num_ext_entries: value[30],
                    num_short_entries: value[31],
                    p_ext_entry_list: parse_u32(&value[32..=35]),
                    p_short_entry_list: parse_u32(&value[36..=39]),
                    local_ext_address: parse_u64(&value[40..=47]),
                    local_short_address: parse_u16(&value[48..=49]),
                    local_pan_id: parse_u16(&value[50..=51]),
                    end_trigger: value[55],
                    end_time: parse_u32(&value[56..=59]),
                },
                CommandId::IeeeEdScan => RadioOperationCommandParameters::IeeeEdScan {
                    channel: value[14],
                    cca_opt: value[15],
                    cca_rssi_thr: value[16],
                    max_rssi: value[18],
                    end_trigger: value[19],
                    end_time: parse_u32(&value[20..=23]),
                },
                CommandId::IeeeTx => RadioOperationCommandParameters::IeeeTx {
                    tx_opt: IeeeTxConfig::from_bytes(value[14..15].try_into().unwrap()),
                    payload_len: value[15],
                    p_payload: parse_u32(&value[16..=19]),
                    time_stamp: parse_u32(&value[20..=23]),
                },
                CommandId::IeeeCsma => RadioOperationCommandParameters::IeeeCsma {
                    random_state: parse_u16(&value[14..=15]),
                    mac_max_be: value[16],
                    mac_max_cdma_backoffs: value[17],
                    csma_config: value[18],
                    nb: value[19],
                    be: value[20],
                    remaining_periods: value[21],
                    last_rssi: value[22],
                    end_trigger: value[23],
                    last_time_stamp: parse_u32(&value[24..=27]),
                    end_time: parse_u32(&value[20..=23]),
                },
                CommandId::IeeeRxAck => RadioOperationCommandParameters::IeeeRxAck {
                    seq_no: value[14],
                    end_trigger: value[15],
                    end_time: parse_u32(&value[16..=19]),
                },
                CommandId::IeeeAbortBg => RadioOperationCommandParameters::IeeeAbortBg {},
                _ => unreachable!("Unknown radio operation command: {:?}", id),
            };
            CommandData::RadioOperation {
                preamble,
                parameters,
            }
        };
        Ok(Command { id, data })
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum CommandData {
    RadioOperation {
        preamble: RadioOperationCommandPreamble,
        parameters: RadioOperationCommandParameters,
    },
    Immediate {
        parameters: ImmediateCommandParameters,
    },
    Direct {
        param: u8,
        extension: u8,
    },
}

#[derive(Clone, Copy, Debug)]
pub(super) enum RadioOperationCommandParameters {
    Nop {},
    RadioSetup {
        mode: u8,
        io_divider: u8,
        config: u16,
        tx_power: u16,
        p_reg_override: u32,
    },
    FsPowerup {
        p_reg_override: u32,
    },
    FsPowerdown {},
    Fs {
        frequency: u16,
        fract_freq: u16,
        synth_conf: u8,
    },
    FsOff {},
    RxTest {
        config: u8,
        end_trigger: u8,
        sync_word: u32,
        end_time: u32,
    },
    TxTest {
        config: u8,
        tx_word: u16,
        end_trigger: u8,
        sync_word: u32,
        end_time: u32,
    },
    SyncStopRat {
        rat0: u32,
    },
    SyncStartRat {
        rat0: u32,
    },
    Count {
        counter: u16,
    },
    SchImm {
        cmdr_val: u32,
        cmdsta_val: u32,
    },
    CountBranch {
        counter: u16,
        p_next_op_if_ok: u32,
    },
    PatternCheck {
        pattern_opt: u16,
        p_next_op_if_ok: u32,
        p_value: u32,
        mask: u32,
        compare_val: u32,
    },

    IeeeRx {
        channel: u8,
        rx_config: IeeeRxConfig,
        p_rx_q: Address,
        p_output: Address,
        frame_filt_opt: FrameFilteringConfiguration,
        frame_types: u8,
        cca_opt: u8,
        cca_rssi_thr: u8,
        num_ext_entries: u8,
        num_short_entries: u8,
        p_ext_entry_list: u32,
        p_short_entry_list: u32,
        local_ext_address: u64,
        local_short_address: u16,
        local_pan_id: u16,
        end_trigger: u8,
        end_time: u32,
    },
    IeeeEdScan {
        channel: u8,
        cca_opt: u8,
        cca_rssi_thr: u8,
        max_rssi: u8,
        end_trigger: u8,
        end_time: u32,
    },
    IeeeTx {
        tx_opt: IeeeTxConfig,
        payload_len: u8,
        p_payload: u32,
        time_stamp: u32,
    },
    IeeeCsma {
        random_state: u16,
        mac_max_be: u8,
        mac_max_cdma_backoffs: u8,
        csma_config: u8,
        nb: u8,
        be: u8,
        remaining_periods: u8,
        last_rssi: u8,
        end_trigger: u8,
        last_time_stamp: u32,
        end_time: u32,
    },
    IeeeRxAck {
        seq_no: u8,
        end_trigger: u8,
        end_time: u32,
    },
    IeeeAbortBg {},
}

impl RadioOperationCommandParameters {
    #[must_use]
    pub(super) const fn is_ieee(&self) -> bool {
        use RadioOperationCommandParameters::*;
        matches!(
            self,
            IeeeRx { .. } | IeeeEdScan { .. } | IeeeTx { .. } | IeeeCsma { .. } | IeeeRxAck { .. }
        )
    }
}

#[bitfield]
#[derive(Clone, Copy, Debug)]
pub(super) struct StartTrigger {
    trigger_type: B4,
    b_ena_cmd: bool,
    trigger_no: B2,
    past_trig: bool,
}

#[bitfield]
#[derive(Clone, Copy, Debug)]
pub(super) struct Condition {
    rule: B4,
    n_skip: B4,
}

/// [TI-TRM] 23.3.2.6.1 Table 23-8
#[derive(Clone, Copy, Debug)]
pub(super) struct RadioOperationCommandPreamble {
    /// Byte index: 2-3
    pub(super) status: CommandStatus,
    /// Byte index: 4-7
    pub(super) p_next_op: Address,
    /// Byte index: 8-11
    pub(super) start_time: u32,
    /// Byte index: 12
    pub(super) start_trigger: StartTrigger,
    /// Byte index: 13
    pub(super) condition: Condition,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum ImmediateCommandParameters {
    Abort {},
    Stop {},
    GetRssi {},
    UpdateRadioSetup {
        p_reg_override: u32,
    },
    Trigger {
        trigger_no: u8,
    },
    GetFwInfo {
        version_no: u16,
        start_offset: u16,
        free_ram_sz: u16,
        avail_rat_ch: u16,
    },
    StartRat {},
    Ping {},
    ReadRfreg {
        address: u16,
        value: u32,
    },
    SetRatCmp {
        rat_ch: u8,
        compare_time: u32,
    },
    SetRatCpt {
        config: u16,
    },
    DisableRatCh {
        rat_ch: u8,
    },
    SetRatOutput {
        config: u16,
    },
    ArmRatCh {
        rat_ch: u8,
    },
    DisarmRatCh {
        rat_ch: u8,
    },
    SetTxPower {
        tx_power: u16,
    },
    UpdateFs {
        frequency: u16,
        fract_freq: u16,
    },
    ModifyFs {
        frequency: u16,
        fract_freq: u16,
    },
    BusRequest {
        b_sys_bus_needed: u8,
    },

    AddDataEntry {
        p_queue: u32,
        p_entry: u32,
    },
    RemoveDataEntry {
        p_queue: u32,
        p_entry: u32,
    },
    FlushQueue {
        p_queue: u32,
        p_first_entry: u32,
    },
    ClearRx {
        p_queue: u32,
    },
    RemovePendingEntries {
        p_queue: u32,
        p_first_entry: u32,
    },

    IeeeModCca {
        new_cca_opt: u8,
        new_cca_rssi_thr: u8,
    },
    IeeeModFilt {
        new_frame_filt_opt: u16,
        new_frame_types: u8,
    },
    IeeeModSrcMatch {
        options: u8,
        entry_no: u8,
    },
    IeeeAbortFg {},
    IeeeStopFg {},
    IeeeCcaReq(CcaReq),
}

fn parse_u64(bytes: &[u8]) -> u64 {
    return u64::from_le_bytes(bytes.try_into().unwrap()).into();
}

fn parse_u32(bytes: &[u8]) -> u32 {
    return u32::from_le_bytes(bytes.try_into().unwrap()).into();
}

fn parse_u16(bytes: &[u8]) -> u16 {
    return u16::from_le_bytes(bytes.try_into().unwrap()).into();
}

#[bitfield]
#[derive(Clone, Copy, Debug)]
pub struct CcaReq {
    pub current_rssi: B8,
    pub max_rssi: B8,
    pub cca_state: B2,
    pub cca_energy: B2,
    pub cca_corr: B2,
    pub cca_sync: bool,
    #[skip]
    __: B1,
}

impl CcaReq {
    pub(super) fn new_invalid() -> Self {
        let invalid_rssi = -128i8;
        let rssi_stored = invalid_rssi.to_le_bytes()[0];
        CcaReq::new()
            .with_current_rssi(rssi_stored)
            .with_max_rssi(rssi_stored)
            .with_cca_state(0b10)
            .with_cca_energy(0b10)
            .with_cca_corr(0b10)
            .with_cca_sync(false)
    }
    pub fn new_clear() -> Self {
        let clear_rssi = -100i8;
        let rssi_stored = clear_rssi.to_le_bytes()[0];
        CcaReq::new()
            .with_current_rssi(rssi_stored)
            .with_max_rssi(rssi_stored)
            .with_cca_state(0b00)
            .with_cca_energy(0b00)
            .with_cca_corr(0b00)
            .with_cca_sync(false)
    }
}
const CC26XX_DEFAULT_CHANNEL: u8 = 11;

#[bitfield]
#[derive(Clone, Copy, Debug)]
pub(super) struct IeeeRxConfig {
    pub(super) b_auto_flush_crc: bool,
    pub(super) b_auto_flush_ign: bool,
    pub(super) b_include_phy_hdr: bool,
    pub(super) b_include_crc: bool,
    pub(super) b_append_rssi: bool,
    pub(super) b_append_corr_crc: bool,
    pub(super) b_append_src_ind: bool,
    pub(super) b_append_timestamp: bool,
}

#[bitfield]
#[derive(Clone, Copy, Debug)]
pub(super) struct IeeeTxConfig {
    pub(super) b_include_phy_hdr: bool,
    pub(super) b_include_crc: bool,
    #[skip]
    __: bool,
    pub(super) payload_len_msb: B5,
}

/// [TI-TRM-I] Table 23-71. Frame Filtering Configuration Bit Field
#[bitfield]
#[derive(Clone, Copy, Debug)]
pub(super) struct FrameFilteringConfiguration {
    pub(super) frame_filt_en: bool,
    pub(super) frame_filt_stop: bool,
    pub(super) auto_ack_en: bool,
    pub(super) slotted_ack_en: bool,
    pub(super) auto_pend_en: bool,
    pub(super) default_pend: bool,
    pub(super) b_pend_data_req_only: bool,
    pub(super) b_pan_coord: bool,
    pub(super) max_frame_version: B2,
    pub(super) fcf_reserved_mask: B3,
    pub(super) modify_ft_filter: B2,
    pub(super) b_strict_len_filter: bool,
}
