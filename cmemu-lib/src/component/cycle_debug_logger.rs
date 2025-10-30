//! Component for logging cycle debug information
//
// This component uses some ugly hacks.

#[proxy_use]
use crate::common::new_ahb;
use crate::common::new_ahb::databus::DataBus;
#[proxy_use]
use crate::common::new_ahb::master_driver::TransferStatus;
use crate::common::new_ahb::signals::{AhbResponseControl, MasterToSlaveWires, SlaveToMasterWires};
#[proxy_use]
use crate::common::{Address, Word};
#[proxy_use]
use crate::component::core::Instruction;
#[proxy_use]
use crate::component::core::{ControlRegister, TransferType, XPSR};
#[proxy_use]
use crate::component::dwt::DWTRegisters;
use crate::confeature::cdl as config;
#[proxy_use]
use crate::engine::Context;
use crate::engine::{
    BufferFlop, DisableableComponent, MainComponent, SeqFlop, SkippableClockTreeNode,
    TickComponent, TickComponentExtra,
};
use cc2650_constants as soc;
use cmemu_common::address_match_range;
use cmemu_proc_macros::{component_impl, handler, proxy_use};
use heapless::Deque as ArrayDeque;
use log::{error, info};
#[cfg(feature = "cdl-black-box")]
use std::collections::VecDeque;
use std::collections::{BTreeSet, HashMap};
use std::fmt::{Debug, Display, Formatter};
use std::path::{Path, PathBuf};

mod render_json_log;

#[derive(MainComponent, SkippableClockTreeNode, TickComponent, DisableableComponent)] // TODO: refactor using subcomponents
#[skippable_if_disableable]
pub(crate) struct CycleDebugLoggerComponent {
    // component config
    log_file: Option<PathBuf>,
    is_recording: bool,
    custom_metadata: HashMap<&'static str, String>,

    // data history
    history: Vec<TimeFrame>, // sorted by time
    #[cfg(feature = "cdl-black-box")]
    black_box: VecDeque<TimeFrame>,
    symbols: HashMap<Address, (String, u8)>, // address of instruction
    all_recorded_addresses: BTreeSet<Address>,

    // flops
    // TODO: during refactor, we probably want to group flops in substructures/subcomponents
    #[flop]
    decode: SeqFlop<Address>,
    #[flop]
    folded_decode: SeqFlop<Address>,
    #[flop]
    decode_agu: SeqFlop<Address>,
    #[flop]
    execute: SeqFlop<Address>,
    #[flop]
    execute_conditionally_skipped: SeqFlop<()>,
    #[flop]
    folded_execute: SeqFlop<Address>,
    #[flop]
    pipelined_execute: SeqFlop<Address>,
    #[flop]
    fetch_registration: SeqFlop<Address>,

    #[flop]
    ibus: ConnectionMultiFlopImpl<DataBus>,
    #[flop]
    ibus_addr_phase_addr: SeqFlop<Address>,
    #[flop]
    ibus_data_phase_addr: SeqFlop<Address>,
    #[flop]
    ibus_addr_phase_status: SeqFlop<TransferStatus>,
    #[flop]
    ibus_data_phase_status: SeqFlop<TransferStatus>,
    #[flop]
    ibus_addr_phase_fetch_transfer_type: SeqFlop<TransferType>,
    #[flop]
    ibus_data_phase_fetch_transfer_type: SeqFlop<TransferType>,

    #[flop]
    dbus: ConnectionMultiFlopImpl<DataBus>,
    #[flop]
    dbus_addr_status: SeqFlop<TransferStatus>,
    #[flop]
    dbus_data_status: SeqFlop<TransferStatus>,
    #[flop]
    dbus_data_phase_addr: SeqFlop<Address>,

    #[cfg(feature = "cdl-ahb-trace")]
    #[flop]
    connections: HashMap<new_ahb::ports::ConnectionName, ConnectionMultiFlop>,

    #[disableable_ignore]
    #[flop]
    free_status: HashMap<&'static str, SeqFlop<FreeStatusEnum>>,

    #[flop]
    core_register_bank: SeqFlop<[Word; 16]>,
    #[flop]
    core_xpsr: SeqFlop<XPSR>,
    #[flop]
    core_control: SeqFlop<ControlRegister>,
    #[flop]
    core_stack_pointers: SeqFlop<(Word, Word)>,
    #[flop]
    core_stacking_mode: SeqFlop<StackingMode>,

    #[flop]
    dwt_registers: SeqFlop<DWTRegisters>,
    #[flop]
    dwt_cpicnt_incremented: SeqFlop<()>,
    #[flop]
    dwt_exccnt_incremented: SeqFlop<()>,
    #[flop]
    dwt_sleepcnt_incremented: SeqFlop<()>,
    #[flop]
    dwt_lsucnt_incremented: SeqFlop<()>,
    #[flop]
    dwt_foldcnt_incremented: SeqFlop<()>,
}

#[derive(Clone, Default, Debug)]
struct TimeFrame {
    cycle_number: u64,

    /// Maps address to events at this address at current `TimeFrame`.
    ///
    /// Please add events with `add_event` to run extra assertions.
    /// CDL is allocating lots of memory, so let's try avoiding
    /// extra allocations (see `Self::MAX_EVENTS_AT_SINGLE_ADDRESS`).
    events: HashMap<Address, EventsVec>, // address of instruction

    core: CoreState,
    dbus: DBusState,
    dwt: DWTState,
    #[cfg(feature = "cdl-ahb-trace")]
    connections: HashMap<new_ahb::ports::ConnectionName, ConnectionMultiFlopState>,
    free_status: HashMap<&'static str, FreeStatusEnum>,
}

// Type associated with `TimeFrame`, however it's not supported by the compiler yet.
type EventsVec = ArrayDeque<EventDesc, { TimeFrame::MAX_EVENTS_AT_SINGLE_ADDRESS }>;
type EventDesc = &'static str;

const _EXTRA_EVENT_FOR_LITERALS: usize =
    !matches!(*config::LOG_LITERALS, config::LogLiteralsValues::None) as usize;
impl TimeFrame {
    /// At any given time and ~space~ address, at most this many events
    /// can be present. If it's not enough anymore, adjust this number
    /// to the required minimum (to save memory).
    ///
    /// Justification for current value:
    /// Three events at the same time and address are achievable by,
    /// e.g. the following code:
    /// ```asm
    ///     mov.n r0, 0x80000000
    ///     mov.n r1, 1
    ///     teq.w r1, r1
    ///     udiv.w r2, r0, r1  @ fill the PIQ
    ///     bne.w next
    /// next:
    ///     ldr.w r3, [r4]
    /// ```
    /// Here, address `next` can have corresponding three events:
    /// fetch, decode and AGU running.
    ///
    /// Alternatively,
    /// ```asm
    ///     ldr   r4, =next
    ///     ....
    ///     ldr.w r3, [r4]
    /// next:
    ///     bne.w next
    /// ```
    /// at `next` during the first cycle, it has fetch, decode and literal.
    /// It is possible to have both address and data phase of the literal here.
    const MAX_EVENTS_AT_SINGLE_ADDRESS: usize = 3 + _EXTRA_EVENT_FOR_LITERALS;

    fn new(cycle_number: u64) -> Self {
        Self {
            cycle_number,
            ..Self::default()
        }
    }

    fn add_event(&mut self, address: Address, description: &'static str) {
        debug_assert!(!description.is_empty());
        let svec = self.events.entry(address).or_default();
        svec.push_back(description)
            .expect("Too many events at a single address");
    }
}

#[derive(Clone, Default, Debug)]
struct CoreState {
    register_bank: [Word; 16],
    xpsr: XPSR,
    control: ControlRegister,
    stack_pointers: (Word, Word),
    stacking_mode: Option<StackingMode>,
}

#[derive(Clone, Default, Debug)]
struct DBusState {
    request: Option<new_ahb::signals::TransferType>,
    set_data: Option<DataBus>,
    response: Option<AhbResponseControl>,
    responder: Option<&'static str>,
}

#[derive(Clone, Default, Debug)]
#[allow(clippy::struct_excessive_bools)]
struct DWTState {
    registers: DWTRegisters,
    cpicnt_incremented: bool,
    exccnt_incremented: bool,
    sleepcnt_incremented: bool,
    lsucnt_incremented: bool,
    foldcnt_incremented: bool,
}

#[cfg(feature = "cdl-ahb-trace")]
#[derive(Clone, Default, Debug)]
struct ConnectionMultiFlopState {
    request: Option<MasterToSlaveWires<DataBus>>,
    response: Option<SlaveToMasterWires<DataBus>>,
}

#[derive(Clone, Copy, Debug)]
enum StackingMode {
    Stacking,
    Unstacking,
}

#[cfg(feature = "cdl-black-box")]
#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
const BLACK_BOX_CAPACITY: usize = *config::BLACK_BOX_CAPACITY as usize;

#[component_impl(cycle_debug_logger)]
impl CycleDebugLoggerComponent {
    pub(crate) fn new() -> Self {
        Self {
            log_file: None,
            is_recording: false,
            custom_metadata: HashMap::new(),

            history: Vec::new(),
            #[cfg(feature = "cdl-black-box")]
            black_box: VecDeque::with_capacity(BLACK_BOX_CAPACITY),
            symbols: HashMap::new(),
            all_recorded_addresses: BTreeSet::new(),

            decode: SeqFlop::new(),
            folded_decode: SeqFlop::new(),
            decode_agu: SeqFlop::new(),
            execute: SeqFlop::new(),
            execute_conditionally_skipped: SeqFlop::new(),
            folded_execute: SeqFlop::new(),
            pipelined_execute: SeqFlop::new(),
            fetch_registration: SeqFlop::new(),
            ibus: ConnectionMultiFlopImpl::default(),
            ibus_addr_phase_addr: SeqFlop::new(),
            ibus_data_phase_addr: SeqFlop::new(),
            ibus_addr_phase_status: SeqFlop::new(),
            ibus_data_phase_status: SeqFlop::new(),
            ibus_addr_phase_fetch_transfer_type: SeqFlop::new(),
            ibus_data_phase_fetch_transfer_type: SeqFlop::new(),
            dbus: ConnectionMultiFlopImpl::default(),
            dbus_addr_status: SeqFlop::new(),
            dbus_data_status: SeqFlop::new(),
            dbus_data_phase_addr: SeqFlop::new(),
            #[cfg(feature = "cdl-ahb-trace")]
            connections: HashMap::new(),
            free_status: HashMap::new(),
            core_register_bank: SeqFlop::new(),
            core_xpsr: SeqFlop::new(),
            core_control: SeqFlop::new(),
            core_stack_pointers: SeqFlop::new(),
            core_stacking_mode: SeqFlop::new(),
            dwt_registers: SeqFlop::new(),
            dwt_cpicnt_incremented: SeqFlop::new(),
            dwt_exccnt_incremented: SeqFlop::new(),
            dwt_sleepcnt_incremented: SeqFlop::new(),
            dwt_lsucnt_incremented: SeqFlop::new(),
            dwt_foldcnt_incremented: SeqFlop::new(),
        }
    }
}

#[component_impl(cycle_debug_logger)]
impl TickComponentExtra for CycleDebugLoggerComponent {
    fn tick_extra(&mut self) {
        self.handle_next_flops();
    }
}

#[component_impl(cycle_debug_logger)]
impl CycleDebugLoggerComponent {
    // TODO: refactor using subcomponents
    pub(crate) fn tick(&mut self, ctx: &mut Context) {
        // During the first cycle, there's no captured data (we process it with one cycle delay)
        if !self.core_register_bank.is_set() {
            return;
        }

        if self.is_collecting() {
            let mut tf = TimeFrame::new(ctx.cycle_no());

            self.handle_fetch(&mut tf);
            self.handle_decode(&mut tf);
            self.handle_execute(&mut tf);
            self.handle_core(&mut tf);
            self.handle_dbus(&mut tf);
            self.handle_dwt(&mut tf);
            #[cfg(feature = "cdl-ahb-trace")]
            self.handle_connections(&mut tf);
            self.handle_free_status(&mut tf);

            if self.is_recording {
                #[cfg(feature = "cdl-black-box")]
                let tf = tf.clone();
                self.history.push(tf);
            }
            #[cfg(feature = "cdl-black-box")]
            {
                if self.black_box.len() >= self.black_box.capacity() {
                    drop(self.black_box.pop_front());
                }
                self.black_box.push_back(tf);
            }
        } else {
            self.ignore_flops();
        }
    }

    #[allow(clippy::unused_self)]
    pub(crate) fn tock(&mut self, _ctx: &mut Context) {}

    pub(crate) fn start_recording(&mut self) {
        self.is_recording = true;
    }

    pub(crate) fn stop_recording(&mut self) {
        self.is_recording = false;
    }

    pub(crate) fn set_log_file(&mut self, log_file: Option<impl AsRef<Path>>) {
        self.log_file = log_file.map(|p| p.as_ref().into());
    }

    pub(crate) fn dump_to_log_file(&self) -> Result<Option<&Path>, Box<dyn std::error::Error>> {
        if let Some(path) = &self.log_file {
            self.dump_to_json(path)?;
        }
        Ok(self.log_file.as_deref())
    }

    #[inline]
    fn is_collecting(&self) -> bool {
        cfg!(feature = "cdl-black-box") || self.is_recording
    }

    pub(crate) fn set_custom_metadata(&mut self, key: &'static str, value: String) {
        let old_value = self.custom_metadata.insert(key, value);
        debug_assert_eq!(old_value, None);
    }

    // This is a trick prior to full CDL frames integration
    pub(crate) fn peek_lsu_request(&self) -> Option<&MasterToSlaveWires<DataBus>> {
        self.dbus.m2s.try_this_cycle()
    }

    fn handle_next_flops(&mut self) {}

    fn handle_fetch(&mut self, tf: &mut TimeFrame) {
        if self.fetch_registration.is_set() {
            tf.add_event(*self.fetch_registration, "F:R");
            self.all_recorded_addresses.insert(*self.fetch_registration);
        }

        if self.ibus_addr_phase_addr.is_set() {
            let transfer_status = *self.ibus_addr_phase_status;
            let desc = match *self.ibus_addr_phase_fetch_transfer_type {
                TransferType::Instruction => match transfer_status {
                    TransferStatus::AddrPhaseNew => "F:IA",
                    TransferStatus::AddrPhaseStalled => "F:IA+",
                    TransferStatus::AddrPhaseDenied => "F:IA-",
                    _ => unreachable!(),
                },
                TransferType::VectorTableValue | TransferType::StackPointer => {
                    match transfer_status {
                        TransferStatus::AddrPhaseNew => "V:IA",
                        TransferStatus::AddrPhaseStalled => "V:IA+",
                        TransferStatus::AddrPhaseDenied => "V:IA-",
                        _ => unreachable!(),
                    }
                }
            };
            tf.add_event(*self.ibus_addr_phase_addr, desc);
            self.all_recorded_addresses
                .insert(*self.ibus_addr_phase_addr);
        } else {
            self.ibus_addr_phase_status.ignore();
        }

        if self.ibus_data_phase_addr.is_set() {
            let desc = if *self.ibus_data_phase_status == TransferStatus::DataPhaseWaiting {
                match *self.ibus_data_phase_fetch_transfer_type {
                    TransferType::Instruction => "F:ID+",
                    TransferType::VectorTableValue | TransferType::StackPointer => "V:ID+",
                }
            } else {
                match *self.ibus_data_phase_fetch_transfer_type {
                    TransferType::Instruction => "F:ID",
                    TransferType::VectorTableValue | TransferType::StackPointer => "V:ID",
                }
            };
            tf.add_event(*self.ibus_data_phase_addr, desc);
            self.all_recorded_addresses
                .insert(*self.ibus_data_phase_addr);
        } else {
            self.ibus_data_phase_status.ignore();
        }
    }

    fn handle_decode(&mut self, tf: &mut TimeFrame) {
        if self.decode.is_set() {
            tf.add_event(*self.decode, "D");
            self.all_recorded_addresses.insert(*self.decode);
        }

        if self.folded_decode.is_set() {
            tf.add_event(*self.folded_decode, "D (F)");
            self.all_recorded_addresses.insert(*self.folded_decode);
        }

        if self.decode_agu.is_set() {
            tf.add_event(*self.decode_agu, "D:AGU");
            self.all_recorded_addresses.insert(*self.decode_agu);
        }
    }

    fn handle_execute(&mut self, tf: &mut TimeFrame) {
        let has_pipelined = self.pipelined_execute.is_set();
        let addr_status = self.dbus_addr_status.map_option(|x| *x);
        let data_status = self.dbus_data_status.map_option(|x| *x);
        let skipped = self.execute_conditionally_skipped.is_set_and(|()| true);

        let status_str = |status| match status {
            TransferStatus::AddrPhaseNew => "X:DA",
            TransferStatus::AddrPhaseStalled => "X:DA+",
            TransferStatus::AddrPhaseDenied => "X:DA-",
            TransferStatus::DataPhaseWaiting => "X:DD+",
            TransferStatus::DataPhaseDone => "X:DD",
        };

        // Skipped instruction executes in one cycle.
        //
        // Thus if `Execute` notifies about conditionally skipped instruction
        // and there is a pipelined instruction, then the pipelined instruction is skipped
        // (if there is pipelined instruction, the preceeding one executes in 2 or more cycles).
        // That is, the main instruction is executed normally.
        //
        // If there is only instruction in main slot, then the instruction
        // from main slot is skipped.
        //
        // For more context see: `crate::component::core::execute::Execute`

        if self.execute.is_set() {
            self.all_recorded_addresses.insert(*self.execute);

            #[allow(clippy::match_same_arms)]
            let desc = match (skipped, has_pipelined, addr_status, data_status) {
                (false, _, None, None) => "X",
                (true, true, None, None) => "X",
                (false, false, Some(addr_status), _) => status_str(addr_status),
                (_, _, _, Some(data_status)) => status_str(data_status),
                (true, false, None, None) => "X (S)",
                // TODO: write buffer may hit this
                (false, _, _, _) => {
                    panic!("Invalid pipelining status {addr_status:?} {data_status:?}")
                }
                (true, _, _, _) => panic!("Instruction skipped, but transfer is requested"),
            };
            tf.add_event(*self.execute, desc);

            // This is LDM-kind
            if !has_pipelined
                && addr_status.is_some()
                && let Some(data_status) = data_status
            {
                tf.add_event(*self.execute, status_str(data_status));
            }

            if self.folded_execute.is_set() {
                // Assert explanation:
                // check `Decode` subcomponent where setting folded instruction.
                debug_assert!(
                    !skipped,
                    "assumption: folded instruction is if-then; then: \
                     main instruction is in it block, so other if-then can't be executed so soon"
                );
                tf.add_event(*self.folded_execute, "X (F)");
                self.all_recorded_addresses.insert(*self.folded_execute);
            }
        } else {
            debug_assert!(!self.execute_conditionally_skipped.is_set());
            debug_assert!(!self.folded_execute.is_set());
        }

        if self.pipelined_execute.is_set() {
            self.all_recorded_addresses.insert(*self.pipelined_execute);
            let desc = match (skipped, addr_status) {
                (false, None) => "X (P)",
                (false, Some(addr_status)) => match addr_status {
                    TransferStatus::AddrPhaseNew => "X:DA (P)",
                    TransferStatus::AddrPhaseStalled => "X:DA+ (P)",
                    TransferStatus::AddrPhaseDenied => "X:DA- (P)",
                    p => panic!("Invalid addr phase status of pipelined instr: {p:?}"),
                },
                (true, None) => "X (P+S)",
                (true, _) => panic!("Instruction skipped, but pipelined transfer is requested"),
            };
            tf.add_event(*self.pipelined_execute, desc);
        }
    }

    fn handle_core(&mut self, tf: &mut TimeFrame) {
        // TODO: we rely on the fact if some but not all flops were set, unused flop assertion would warn us about it
        if self.core_register_bank.is_set()
            && self.core_xpsr.is_set()
            && self.core_control.is_set()
            && self.core_stack_pointers.is_set()
        {
            // make sure all fields are written
            tf.core = CoreState {
                register_bank: *self.core_register_bank,
                xpsr: *self.core_xpsr,
                control: *self.core_control,
                stack_pointers: *self.core_stack_pointers,
                stacking_mode: self.core_stacking_mode.map_option(|f| *f),
            };
        }
    }

    fn handle_dbus(&mut self, tf: &mut TimeFrame) {
        // make sure all fields are written

        let (request, set_data) = self.dbus.m2s.try_take().map_or(Default::default(), |m2s| {
            (Some(m2s.addr_phase.meta), m2s.data_phase.data.into_option())
        });
        let (response, set_data2, responder) =
            self.dbus.s2m.try_take().map_or(Default::default(), |s2m| {
                (
                    Some(s2m.meta),
                    s2m.data.into_option(),
                    Some(s2m.responder_tag.as_str()),
                )
            });

        // Support for marking literal loads
        // FIXME: this doesn't handle request width and alignment at all
        'addr_event: {
            let request = request.as_ref();
            if let Some(x) = request.and_then(|t| t.meta()) {
                address_match_range! {x.addr,
                    soc::FLASHMEM::ADDR_SPACE => (),
                    soc::GPRAM::ADDR_SPACE => (),
                    soc::SRAM::ADDR_SPACE => (),
                    _ => if *config::LOG_LITERALS != config::LogLiteralsValues::All  {break 'addr_event},
                };
                let desc = match *self.dbus_addr_status {
                    TransferStatus::AddrPhaseNew => "L:DA",
                    TransferStatus::AddrPhaseStalled => "L:DA+",
                    TransferStatus::AddrPhaseDenied => break 'addr_event,
                    _ => unreachable!(),
                };

                if *config::LOG_LITERALS != config::LogLiteralsValues::None {
                    tf.add_event(x.addr, desc);
                    self.all_recorded_addresses.insert(x.addr);
                }
            }
        }
        'data_event: {
            if let Some(addr) = self.dbus_data_phase_addr.map_option(|&f| f) {
                address_match_range! {addr,
                    soc::FLASHMEM::ADDR_SPACE => (),
                    soc::GPRAM::ADDR_SPACE => (),
                    soc::SRAM::ADDR_SPACE => (),
                    _ => if *config::LOG_LITERALS != config::LogLiteralsValues::All  {break 'data_event},
                };
                let desc = match *self.dbus_data_status {
                    TransferStatus::DataPhaseDone => "L:DD",
                    TransferStatus::DataPhaseWaiting => "L:DD+",
                    _ => unreachable!(),
                };
                if *config::LOG_LITERALS != config::LogLiteralsValues::None {
                    tf.add_event(addr, desc);
                    self.all_recorded_addresses.insert(addr);
                }
            }
        }
        tf.dbus = DBusState {
            request,
            set_data: set_data.or(set_data2),
            response,
            responder,
        };
    }

    #[cfg(feature = "cdl-ahb-trace")]
    fn handle_connections(&mut self, tf: &mut TimeFrame) {
        tf.connections = self
            .connections
            .iter_mut()
            .map(|(name, cmf)| {
                (
                    *name,
                    ConnectionMultiFlopState {
                        request: cmf.unwrap_databus_mut().m2s.try_take(),
                        response: cmf.unwrap_databus_mut().s2m.try_take(),
                    },
                )
            })
            .filter(|(_, cmfs)| cmfs.response.is_some() || cmfs.request.is_some())
            .collect();
    }

    fn handle_free_status(&mut self, tf: &mut TimeFrame) {
        tf.free_status = self
            .free_status
            .iter_mut()
            .filter_map(|(name, fsf)| fsf.try_take().map(|fs| (*name, fs)))
            .collect();
    }

    fn handle_dwt(&mut self, tf: &mut TimeFrame) {
        // TODO: we rely on the fact if some but not all flops were set, unused flop assertion would warn us about it
        if self.dwt_registers.is_set() {
            // make sure all fields are written
            tf.dwt = DWTState {
                registers: self.dwt_registers.clone(),
                cpicnt_incremented: self.dwt_cpicnt_incremented.is_set_and(|()| true),
                exccnt_incremented: self.dwt_exccnt_incremented.is_set_and(|()| true),
                sleepcnt_incremented: self.dwt_sleepcnt_incremented.is_set_and(|()| true),
                lsucnt_incremented: self.dwt_lsucnt_incremented.is_set_and(|()| true),
                foldcnt_incremented: self.dwt_foldcnt_incremented.is_set_and(|()| true),
            };
        }
    }

    fn ignore_flops(&mut self) {
        self.decode.ignore();
        self.folded_decode.ignore();
        self.decode_agu.ignore();
        self.execute.ignore();
        self.execute_conditionally_skipped.ignore();
        self.folded_execute.ignore();
        self.pipelined_execute.ignore();
        self.fetch_registration.ignore();

        self.ibus.ignore();
        self.ibus_addr_phase_addr.ignore();
        self.ibus_addr_phase_status.ignore();
        self.ibus_addr_phase_fetch_transfer_type.ignore();
        self.ibus_data_phase_addr.ignore();
        self.ibus_data_phase_status.ignore();
        self.ibus_data_phase_fetch_transfer_type.ignore();

        self.dbus.ignore();
        self.dbus_addr_status.ignore();
        self.dbus_data_status.ignore();
        self.dbus_data_phase_addr.ignore();

        self.core_register_bank.ignore();
        self.core_xpsr.ignore();
        self.core_control.ignore();
        self.core_stack_pointers.ignore();
        self.core_stacking_mode.ignore();
        #[cfg(feature = "cdl-ahb-trace")]
        for flop in self.connections.values_mut() {
            // XXX: aaa
            let flop = flop.unwrap_databus_mut();
            flop.m2s.ignore();
            flop.s2m.ignore();
        }
        for flop in self.free_status.values_mut() {
            flop.ignore();
        }

        self.dwt_registers.ignore();
        self.dwt_cpicnt_incremented.map_or((), |()| ());
        self.dwt_exccnt_incremented.map_or((), |()| ());
        self.dwt_sleepcnt_incremented.map_or((), |()| ());
        self.dwt_lsucnt_incremented.map_or((), |()| ());
        self.dwt_foldcnt_incremented.map_or((), |()| ());
    }

    #[handler]
    pub(crate) fn on_decode(
        &mut self,
        _ctx: &mut Context,
        instr_addr: Address,
        instr_sym: Instruction,
        instr_len: u8,
    ) {
        self.decode.set_next(instr_addr);

        // avoid formatting and allocation when not recording
        if self.is_collecting() {
            let new_val = (instr_sym.to_string(), instr_len);
            let old_val = self.symbols.insert(instr_addr, new_val.clone());

            debug_assert!(old_val.is_none_or(|old_val| old_val == new_val));
        }
    }

    #[handler]
    pub(crate) fn on_folded_decode(
        &mut self,
        _ctx: &mut Context,
        instr_addr: Address,
        instr_sym: Instruction,
    ) {
        self.folded_decode.set_next(instr_addr);

        // avoid formatting and allocation when not recording
        if self.is_collecting() {
            let new_val = (instr_sym.to_string(), 1);
            let old_val = self.symbols.insert(instr_addr, new_val.clone());

            debug_assert!(old_val.is_none_or(|old_val| old_val == new_val));
        }
    }

    #[handler]
    pub(crate) fn on_decode_agu(&mut self, _ctx: &mut Context, instr_addr: Address) {
        self.decode_agu.set_next(instr_addr);
    }

    #[handler]
    pub(crate) fn on_execute(&mut self, _ctx: &mut Context, instr_addr: Address) {
        self.execute.set_next(instr_addr);
    }

    #[handler]
    pub(crate) fn on_execute_conditionally_skipped(&mut self, _ctx: &mut Context) {
        self.execute_conditionally_skipped.set_next(());
    }

    #[handler]
    pub(crate) fn on_folded_execute(&mut self, _ctx: &mut Context, instr_addr: Address) {
        self.folded_execute.set_next(instr_addr);
    }

    #[handler]
    pub(crate) fn on_pipelined_execute(&mut self, _ctx: &mut Context, instr_addr: Address) {
        self.pipelined_execute.set_next(instr_addr);
    }

    #[handler]
    pub(crate) fn on_fetch_registration(&mut self, _ctx: &mut Context, instr_addr: Address) {
        self.fetch_registration.set_next(instr_addr);
    }

    #[handler]
    pub(crate) fn on_ibus_request(
        &mut self,
        _ctx: &mut Context,
        // req: new_ahb::signals::MasterToSlaveWires<new_ahb::DataBus>,
        addr_phase: Option<(Address, TransferStatus, TransferType)>,
        data_phase: Option<(Address, TransferStatus, TransferType)>,
    ) {
        // self.dbus.m2s.set_next(req.clone());
        if let Some((addr, status, transfer_type)) = addr_phase {
            self.ibus_addr_phase_addr.set_next(addr);
            self.ibus_addr_phase_status.set_next(status);
            self.ibus_addr_phase_fetch_transfer_type
                .set_next(transfer_type);
        }
        if let Some((addr, _status, transfer_type)) = data_phase {
            self.ibus_data_phase_addr.set_next(addr);
            self.ibus_data_phase_fetch_transfer_type
                .set_next(transfer_type);
        }
    }

    #[handler]
    pub(crate) fn on_ibus_response(
        &mut self,
        _ctx: &mut Context,
        // response: new_ahb::signals::SlaveToMasterWires<new_ahb::DataBus>,
        data_status: Option<TransferStatus>,
    ) {
        // We have status here, since we will have better info if finished in tock
        if let Some(status) = data_status {
            self.ibus_data_phase_status.set_next(status);
            // self.ibus.s2m.set_next(response);
        }
    }

    #[handler]
    pub(crate) fn on_dbus_request(
        &mut self,
        _ctx: &mut Context,
        req: new_ahb::signals::MasterToSlaveWires<new_ahb::DataBus>,
        addr_status: Option<TransferStatus>,
        data_addr: Option<Address>,
    ) {
        self.dbus.m2s.set_next(req);
        if let Some(addr_status) = addr_status {
            self.dbus_addr_status.set_next(addr_status);
        }
        if let Some(addr) = data_addr {
            self.dbus_data_phase_addr.set_next(addr);
        }
    }

    #[handler]
    pub(crate) fn on_dbus_response(
        &mut self,
        _ctx: &mut Context,
        response: new_ahb::signals::SlaveToMasterWires<new_ahb::DataBus>,
        data_status: Option<TransferStatus>,
    ) {
        // We have status here, since we will have better info if finished in tock
        // TODO: addr data address?
        if let Some(status) = data_status {
            self.dbus_data_status.set_next(status);
            self.dbus.s2m.set_next(response);
        }
    }

    #[handler]
    pub(crate) fn on_core_register_bank_tick(
        &mut self,
        _ctx: &mut Context,
        register_bank: [Word; 16], // XXX: This is large!
        xpsr: XPSR,
        control: ControlRegister,
        stack_pointers: (Word, Word),
    ) {
        self.core_register_bank.set_next(register_bank);
        self.core_xpsr.set_next(xpsr);
        self.core_control.set_next(control);
        self.core_stack_pointers.set_next(stack_pointers);
    }

    #[handler]
    pub(crate) fn on_core_run_stacking(&mut self, _ctx: &mut Context) {
        self.core_stacking_mode.set_next(StackingMode::Stacking);
    }

    #[handler]
    pub(crate) fn on_core_run_unstacking(&mut self, _ctx: &mut Context) {
        self.core_stacking_mode.set_next(StackingMode::Unstacking);
    }

    #[handler]
    pub(crate) fn on_dwt_tick(&mut self, _ctx: &mut Context, registers: DWTRegisters) {
        self.dwt_registers.set_next(registers);
    }

    #[handler]
    pub(crate) fn on_dwt_increment_cpi_counter(&mut self, _ctx: &mut Context) {
        self.dwt_cpicnt_incremented.set_next(());
    }

    #[handler]
    pub(crate) fn on_dwt_increment_exception_counter(&mut self, _ctx: &mut Context) {
        self.dwt_exccnt_incremented.set_next(());
    }

    #[handler]
    pub(crate) fn on_dwt_increment_sleep_counter(&mut self, _ctx: &mut Context) {
        self.dwt_sleepcnt_incremented.set_next(());
    }

    #[handler]
    pub(crate) fn on_dwt_increment_lsu_counter(&mut self, _ctx: &mut Context) {
        self.dwt_lsucnt_incremented.set_next(());
    }

    #[handler]
    pub(crate) fn on_dwt_increment_fold_counter(&mut self, _ctx: &mut Context) {
        self.dwt_foldcnt_incremented.set_next(());
    }

    #[handler]
    #[allow(dead_code)] // TODO: we need support for gated  handlers
    pub(crate) fn on_connection_m2s_databus(
        &mut self,
        _ctx: &mut Context,
        #[allow(unused)] connection_name: new_ahb::ports::ConnectionName,
        #[allow(unused)] req: new_ahb::signals::M2SBus,
    ) {
        // avoid formatting and allocation when not recording
        #[cfg(feature = "cdl-ahb-trace")]
        if self.is_recording {
            self.connections
                .entry(connection_name)
                .or_insert_with(|| ConnectionMultiFlop::DataBus(Default::default()))
                .unwrap_databus_mut()
                .m2s
                .set_next(req);
        }
    }

    #[handler]
    #[allow(dead_code)]
    pub(crate) fn on_connection_s2m_databus(
        &mut self,
        _ctx: &mut Context,
        #[allow(unused)] connection_name: new_ahb::ports::ConnectionName,
        #[allow(unused)] resp: new_ahb::signals::S2MBus,
    ) {
        // avoid formatting and allocation when not recording
        #[cfg(feature = "cdl-ahb-trace")]
        if self.is_recording {
            self.connections
                .entry(connection_name)
                .or_insert_with(|| ConnectionMultiFlop::DataBus(Default::default()))
                .unwrap_databus_mut()
                .s2m
                .set_next(resp);
        }
    }

    #[handler]
    pub(crate) fn on_free_static_str(
        &mut self,
        _ctx: &mut Context,
        name: &'static str,
        value: &'static str,
    ) {
        // avoid formatting and allocation when not recording
        if self.is_recording {
            self.free_status
                .entry(name)
                .or_default()
                .set_next(FreeStatusEnum::StaticStr(value));
        }
    }
    #[handler]
    pub(crate) fn on_free_formatted_u64(
        &mut self,
        _ctx: &mut Context,
        name: &'static str,
        value: u64,
        fmter: fn(u64) -> String,
    ) {
        // avoid formatting and allocation when not recording
        if self.is_recording {
            self.free_status
                .entry(name)
                .or_default()
                .set_next(FreeStatusEnum::U64Fmter(value, fmter));
        }
    }
}

#[component_impl(cycle_debug_logger)]
impl Drop for CycleDebugLoggerComponent {
    fn drop(&mut self) {
        if let Some(path) = &self.log_file {
            match self.dump_to_json(path) {
                Ok(()) => info!("Written cycle debug log to {}.", path.display()),
                Err(err) => error!(
                    "Failed to write cycle debug log to {}. Error: {}",
                    path.display(),
                    err
                ),
            }
        }
        #[cfg(feature = "cdl-black-box")]
        if *config::BLACK_BOX_AUTO && std::thread::panicking() {
            self.launch_black_box();
        }
    }
}

trait FlopTickTrickery {
    fn tick(&mut self, #[cfg(debug_assertions)] my_name: &str);
    fn ignore(&mut self);
    fn is_empty(&self) -> bool;
}

#[derive(Debug, Default)]
struct ConnectionMultiFlopImpl<D: Debug + Default + Clone + 'static> {
    m2s: BufferFlop<MasterToSlaveWires<D>>,
    s2m: BufferFlop<SlaveToMasterWires<D>>,
}

impl<D: Debug + Default + Clone> FlopTickTrickery for ConnectionMultiFlopImpl<D> {
    fn tick(&mut self, #[cfg(debug_assertions)] my_name: &str) {
        self.m2s.tick(
            #[cfg(debug_assertions)]
            my_name,
        );
        self.s2m.tick(
            #[cfg(debug_assertions)]
            my_name,
        );
    }

    fn ignore(&mut self) {
        self.m2s.ignore();
        self.s2m.ignore();
    }

    fn is_empty(&self) -> bool {
        self.m2s.is_empty() && self.s2m.is_empty()
    }
}

#[derive(Clone, Debug)]
enum FreeStatusEnum {
    StaticStr(&'static str),
    U64Fmter(u64, fn(u64) -> String),
}

impl FlopTickTrickery for HashMap<&'static str, SeqFlop<FreeStatusEnum>> {
    #[cfg_attr(not(debug_assertions), allow(unused_variables))]
    fn tick(&mut self, #[cfg(debug_assertions)] _my_name: &str) {
        for (name, flop) in self.iter_mut() {
            flop.tick(
                #[cfg(debug_assertions)]
                name,
            );
        }
    }
    fn ignore(&mut self) {
        for flop in self.values_mut() {
            flop.ignore();
        }
    }
    fn is_empty(&self) -> bool {
        for flop in self.values() {
            if !flop.is_empty() {
                return false;
            }
        }
        true
    }
}

impl Display for FreeStatusEnum {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::StaticStr(s) => f.write_str(s),
            Self::U64Fmter(u, fmter) => f.write_str(fmter(u).as_str()),
        }
    }
}

#[cfg(feature = "cdl-ahb-trace")]
#[derive(Debug)]
enum ConnectionMultiFlop {
    DataBus(ConnectionMultiFlopImpl<DataBus>),
    // Leave those for a while in case we use AHB with this as data somewhere.
    #[allow(dead_code)]
    B4(ConnectionMultiFlopImpl<[u8; 4]>),
    #[allow(dead_code)]
    B8(ConnectionMultiFlopImpl<[u8; 8]>),
}

#[cfg(feature = "cdl-ahb-trace")]
impl ConnectionMultiFlop {
    fn unwrap_databus_mut(&mut self) -> &mut ConnectionMultiFlopImpl<DataBus> {
        if let Self::DataBus(inner) = self {
            return inner;
        }
        panic!("ConnectionMultiFlop unwrapped on wrong type!");
    }

    fn as_flop(&mut self) -> &mut dyn FlopTickTrickery {
        match self {
            Self::DataBus(flop) => flop,
            Self::B4(flop) => flop,
            Self::B8(flop) => flop,
        }
    }
}

#[cfg(feature = "cdl-ahb-trace")]
impl FlopTickTrickery for HashMap<new_ahb::ports::ConnectionName, ConnectionMultiFlop> {
    fn tick(&mut self, #[cfg(debug_assertions)] my_name: &str) {
        for (_name, flop) in self.iter_mut() {
            flop.tick(
                #[cfg(debug_assertions)]
                my_name, //name,
            );
        }
    }
    fn ignore(&mut self) {
        for flop in self.values_mut() {
            flop.ignore();
        }
    }
    fn is_empty(&self) -> bool {
        for flop in self.values() {
            if !flop.is_empty() {
                return false;
            }
        }
        true
    }
}
#[cfg(feature = "cdl-ahb-trace")]
impl FlopTickTrickery for ConnectionMultiFlop {
    fn tick(&mut self, #[cfg(debug_assertions)] my_name: &str) {
        match self {
            Self::DataBus(flop) => flop.tick(
                #[cfg(debug_assertions)]
                my_name,
            ),
            Self::B4(flop) => flop.tick(
                #[cfg(debug_assertions)]
                my_name,
            ),
            Self::B8(flop) => flop.tick(
                #[cfg(debug_assertions)]
                my_name,
            ),
        }
    }
    fn ignore(&mut self) {
        self.as_flop().ignore();
    }
    fn is_empty(&self) -> bool {
        match self {
            Self::DataBus(flop) => flop.is_empty(),
            Self::B4(flop) => flop.is_empty(),
            Self::B8(flop) => flop.is_empty(),
        }
    }
}
