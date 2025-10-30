#![allow(unused)] // Dead code for a while
use crate::component::rfc::command::{
    Command, CommandData, CommandId, CommandStatus, RadioOperationCommandParameters,
    RadioOperationCommandPreamble,
};
use log::warn;
pub(super) fn cycles_to_ack(command: &Command) -> usize {
    1
    // (match command {
    //     Command {
    //         id: CommandId::Ping,
    //         data: CommandData::Direct { .. },
    //     } => 770,
    //     Command {
    //         id: CommandId::StartRat,
    //         data: CommandData::Direct { .. },
    //     } => 902,
    //     Command {
    //         id: CommandId::BusRequest,
    //         data: CommandData::Direct { param, extension },
    //     } => 758,
    //     Command {
    //         id: CommandId::RadioSetup,
    //         data:
    //             CommandData::RadioOperation {
    //                 preamble:
    //                     RadioOperationCommandPreamble {
    //                         status: CommandStatus::Idle,
    //                         p_next_op,
    //                         start_time: 0,
    //                         start_trigger,
    //                         condition,
    //                     },
    //                 parameters:
    //                     RadioOperationCommandParameters::RadioSetup {
    //                         mode: 1,
    //                         io_divider,
    //                         config,
    //                         tx_power,
    //                         p_reg_override,
    //                     },
    //             },
    //     } => 758,
    //     Command {
    //         id: CommandId::MagicalSecretSauce,
    //         ..
    //     } => {
    //         // https://github.com/contiki-ng/contiki-ng/blob/482e65555a600df848a120ce3addeb4e8a7db126/arch/cpu/cc26x0-cc13x0/rf-core/rf-core.c#L312
    //         // Contiki doesn't give a fuck about waiting for this command, it just sends next one :)
    //         1
    //     }
    //     Command {
    //         id: CommandId::FsPowerdown,
    //         ..
    //     } => {
    //         // TODO(pk): actually measure it
    //         700
    //     }
    //     Command {
    //         id: CommandId::SyncStopRat,
    //         ..
    //     } => {
    //         // TODO(pk): actually measure it
    //         700
    //     }

    //     _ => {
    //         // TODO(pk): uncomment once done with testing
    //         // todo!("Command {command:?} cycles to ack not yet measured."),
    //         50
    //     }
    // }).max(1500) / 1500
}

pub(super) fn cycles_to_done(command: &Command) -> usize {
    // I think Ieee commands don't rise ack interrupt
    // (match command {
    //     Command {
    //         id: CommandId::IeeeTx,
    //         data:
    //             CommandData::RadioOperation {
    //                 preamble,
    //                 parameters:
    //                     RadioOperationCommandParameters::IeeeTx {
    //                         tx_opt,
    //                         payload_len: 120,
    //                         p_payload,
    //                         time_stamp: 0,
    //                     },
    //             },
    //     } => 10101,
    //     Command {
    //         id: CommandId::IeeeRx,
    //         data:
    //             CommandData::RadioOperation {
    //                 preamble:
    //                     RadioOperationCommandPreamble {
    //                         status: CommandStatus::Idle,
    //                         p_next_op, // Address(0)
    //                         start_time: 0,
    //                         start_trigger,
    //                         // StartTrigger {
    //                         //     trigger_type: 0,
    //                         //     b_ena_cmd: false,
    //                         //     trigger_no: 0,
    //                         //     past_trig: false,
    //                         // },
    //                         condition,
    //                         // Condition { rule: 1, n_skip: 0 },
    //                     },
    //                 parameters:
    //                     RadioOperationCommandParameters::IeeeRx {
    //                         channel: CC26XX_DEFAULT_CHANNEL,
    //                         ..
    //                     },
    //             },
    //     } => 100,
    //     Command {
    //         id: CommandId::MagicalSecretSauce,
    //         ..
    //     } => cycles_to_ack(command) * 1500 + 1,

    //     command => {
    //         // warn!("Command timing is not yet measured {command:?}");
    //         cycles_to_ack(command) * 1500 + 1
    //     }
    //     _ => todo!("Command timing is not yet measured {command:?}"),
    // }).max(1500) / 1500
    2
}
