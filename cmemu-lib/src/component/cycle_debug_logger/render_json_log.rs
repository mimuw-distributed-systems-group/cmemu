// Note: serialize numbers exceeding 52-bit range as strings
// to avoid precision loss due to JavaScript.

use flate2::Compression;
use flate2::write::GzEncoder;
use std::collections::{BTreeSet, HashMap};
use std::ffi::{OsStr, OsString};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::{env, fs};

use serde::Serialize;

use crate::common::Address;
use crate::common::new_ahb::signals::{AhbResponseControl, Burst, TransferType};
#[cfg(feature = "cdl-ahb-trace")]
use crate::utils::IfExpr;
use crate::utils::dife_lazy;

use super::{CycleDebugLoggerComponent, StackingMode, TimeFrame};

/// Returns a path with a new dotted extension component appended to the end.
/// Note: does not check if the path is a file or directory; you should do that.
/// # Example
/// ```ignore
/// use std::path::PathBuf;
/// let path = PathBuf::from("foo/bar/baz.txt");
/// if !path.is_dir() {
///    assert_eq!(append_ext("app", path), PathBuf::from("foo/bar/baz.txt.app"));
/// }
/// ```
/// Source: <https://internals.rust-lang.org/t/pathbuf-has-set-extension-but-no-add-extension-cannot-cleanly-turn-tar-to-tar-gz/14187/11>
pub fn append_ext(ext: impl AsRef<OsStr>, path: PathBuf) -> PathBuf {
    let mut os_string: OsString = path.into();
    os_string.push(".");
    os_string.push(ext.as_ref());
    os_string.into()
}

impl CycleDebugLoggerComponent {
    /// Dump to a JSON file
    ///
    /// Supports compressed files (ending with .gz) or splitting into parts in a directory (.d suffix)
    pub(super) fn dump_to_json(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: expose these options as clap flags?
        const DEFAULT_CDL_PARTS_SIZE: usize = 50_000;
        const DEFAULT_CDL_PARTS_OVERLAP: usize = 250;
        let size = env::var("CDL_PARTS_SIZE")
            .map(|s| s.parse().expect("Cannot parse 'CDL_PARTS_SIZE'"))
            .unwrap_or(DEFAULT_CDL_PARTS_SIZE);
        let overlap = env::var("CDL_PARTS_OVERLAP")
            .map(|s| s.parse().expect("Cannot parse 'CDL_PARTS_OVERLAP'"))
            .unwrap_or(DEFAULT_CDL_PARTS_OVERLAP);
        // Pass 0 to auto detect
        let auto_split: Option<usize> = env::var("CDL_PARTS_AUTOSPLIT")
            .map(|s| s.parse().expect("Cannot parse 'CDL_PARTS_AUTOSPLIT'"))
            .ok();

        if path.extension().is_some_and(|e| e == "d") {
            if path.is_dir() {
                fs::remove_dir_all(path)?;
            } else if path.exists() {
                panic!(
                    "The specified path {} has a .d extension -- not removing a file",
                    path.display()
                );
            }
            fs::create_dir(path)?;

            for (cycle_idx, log) in self.render_windowed_json_log(size, overlap) {
                let sub_path = path.join(format!("part-{cycle_idx:09}.gz"));
                Self::write_json(log, sub_path.as_path())?;
            }
        } else {
            let log = self.render_json_log();
            Self::write_json(log, path)?;

            // Write also partial files if requested
            if let Some(mut auto_split) = auto_split {
                let dir_path = append_ext("d", path.to_path_buf());
                let dir_path = dir_path.as_path();
                if auto_split == 0 {
                    auto_split = size * 3;
                }
                if self.history.len() > auto_split {
                    Self::dump_to_json(self, dir_path)?;
                } else if dir_path.exists() {
                    // Make sure that by checking for dir_path, one can know if we generated partial files.
                    fs::remove_dir_all(dir_path)?;
                }
            }
        }
        Ok(())
    }

    fn write_json<'a>(
        log: impl Serialize + 'a,
        path: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let file = fs::File::create(path)?;
        let mut writer = BufWriter::with_capacity(4 * 1024 * 1024, file);
        let mut encoder;
        let writer: &mut dyn Write = if path.extension().is_some_and(|e| e == "gz") {
            encoder = GzEncoder::new(writer, Compression::default());
            &mut encoder
        } else {
            &mut writer
        };
        serde_json::to_writer(writer, &log)?;
        Ok(())
    }

    fn render_windowed_json_log(
        &self,
        size: usize,
        overlap: usize,
    ) -> impl Iterator<Item = (u64, impl Serialize + '_)> {
        let len = self.history.len();
        (0..len)
            .step_by(size)
            .map(move |start| {
                (
                    start,
                    &self.history[start.saturating_sub(overlap)..(start + size).min(len)],
                )
            })
            .map(move |(offset, history)| {
                (
                    self.history[offset].cycle_number,
                    JsonLog {
                        cycles_desc: history,
                        mem_instr_desc: (&self.all_recorded_addresses, &self.symbols),
                        events: history,
                        metadata: &self.custom_metadata,
                    },
                )
            })
    }

    fn render_json_log(&self) -> impl Serialize + '_ {
        JsonLog {
            cycles_desc: self.history.as_slice(),
            mem_instr_desc: (&self.all_recorded_addresses, &self.symbols),
            events: self.history.as_slice(),
            metadata: &self.custom_metadata,
        }
    }

    #[cfg(feature = "cdl-black-box")]
    fn render_json_black_box(&self) -> JsonLog<'_> {
        JsonLog {
            cycles_desc: self.black_box.as_slices().0,
            mem_instr_desc: (&self.all_recorded_addresses, &self.symbols),
            events: self.black_box.as_slices().0,
            metadata: &self.custom_metadata,
        }
    }

    #[cfg(feature = "cdl-black-box")]
    #[allow(clippy::print_stderr)]
    pub(crate) fn launch_black_box(&mut self) {
        let mut file = match tempfile::Builder::new()
            .prefix("cdl-black-box-")
            .suffix(".json")
            .tempfile()
        {
            Err(e) => return eprintln!("failed to create temporary file for CDL black box: {e:?}"),
            Ok(file) => file,
        };
        self.black_box.make_contiguous();
        let log = self.render_json_black_box();
        if let Err(e) = serde_json::to_writer(file.as_file_mut(), &log) {
            return eprintln!("failed to write CDL black box: {e:?}");
        }
        match file.keep() {
            Err(e) => eprintln!("failed to retain CDL black box: {e:?}"),
            Ok((_, path)) => eprintln!("CDL black box saved to {}", path.display()),
        }
    }
}

#[derive(Serialize)]
struct JsonLog<'a> {
    #[serde(serialize_with = "serialize_cycles_desc")]
    cycles_desc: &'a [TimeFrame],
    #[serde(serialize_with = "serialize_mem_instr_desc")]
    mem_instr_desc: (&'a BTreeSet<Address>, &'a HashMap<Address, (String, u8)>),
    // see: `addr_cycle_to_key`
    #[serde(serialize_with = "serialize_events")]
    events: &'a [TimeFrame],
    metadata: &'a HashMap<&'static str, String>,
}

#[derive(Serialize)]
struct CycleDescription {
    cycle_no: String,
    core: CoreStateDescription,
    dbus: DBusStateDescription,
    dwt: DWTStateDescription,
    #[cfg(feature = "cdl-ahb-trace")]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    connections: HashMap<String, ConnectionMultiFlopStateDescription>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    free_status: HashMap<&'static str, String>,
}

#[derive(Serialize)]
struct CoreStateDescription {
    register_bank: Vec<String>,
    xpsr: String,
    control: String,
    stack_pointers: String,
    stacking_mode: Option<&'static str>,
}

#[derive(Serialize)]
struct DBusStateDescription {
    request: String,
    response: &'static str,
    data: String,
    responder: &'static str,
}

#[cfg(feature = "cdl-ahb-trace")]
#[derive(Serialize)]
struct ConnectionMultiFlopStateDescription {
    #[serde(skip_serializing_if = "Option::is_none")]
    request: Option<AhbRequestDescription>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response: Option<AhbResponseDescription>,
}

#[derive(Serialize)]
#[allow(clippy::struct_excessive_bools)]
struct DWTStateDescription {
    cyccnt: u32,
    cpicnt: u8,
    cpicnt_incremented: bool,
    exccnt: u8,
    exccnt_incremented: bool,
    sleepcnt: u8,
    sleepcnt_incremented: bool,
    lsucnt: u8,
    lsucnt_incremented: bool,
    foldcnt: u8,
    foldcnt_incremented: bool,
}

#[cfg(feature = "cdl-ahb-trace")]
#[derive(Serialize)]
struct AhbRequestDescription {
    state: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    direction: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protection: Option<String>,
    #[cfg(feature = "cycle-debug-logger")]
    addr_tag: &'static str,
    #[cfg(feature = "cdl-ahb-trace")]
    addr_trace: AhbTraceDescription,

    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<String>,
    #[cfg(feature = "cycle-debug-logger")]
    data_tag: &'static str,
    #[cfg(feature = "cdl-ahb-trace")]
    data_trace: AhbTraceDescription,
}

#[cfg(feature = "cdl-ahb-trace")]
#[derive(Serialize)]
struct AhbResponseDescription {
    status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<String>,
    #[cfg(feature = "cycle-debug-logger")]
    src_tag: &'static str,
    #[cfg(feature = "cdl-ahb-trace")]
    src_trace: AhbTraceDescription,
    #[cfg(feature = "cycle-debug-logger")]
    reply_tag: &'static str,
    #[cfg(feature = "cdl-ahb-trace")]
    reply_trace: AhbTraceDescription,
}

#[cfg(feature = "cdl-ahb-trace")]
#[derive(Serialize)]
struct AhbTraceDescription {
    id: u32,
    depth: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    story: Option<String>,
}

fn addr_cycle_to_key(addr: Address, cycle_no: u64) -> String {
    format!("{addr:?},{cycle_no}")
}

fn serialize_mem_instr_desc<S: serde::Serializer>(
    (all_recorded_addresses, symbols): &(&BTreeSet<Address>, &HashMap<Address, (String, u8)>),
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.collect_seq(all_recorded_addresses.iter().map(|&addr| {
        let (instr, instr_len) = match symbols.get(&addr) {
            None => (None, None),
            Some((instr, len)) => (Some(instr), Some(*len)),
        };
        AddressWithInstruction {
            addr,
            instr,
            instr_len,
        }
    }))
}

#[derive(Serialize)]
struct AddressWithInstruction<'a> {
    #[serde(serialize_with = "serialize_via_debug")]
    addr: Address,
    #[serde(skip_serializing_if = "Option::is_none")]
    instr: Option<&'a String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instr_len: Option<u8>,
}

fn serialize_cycles_desc<S: serde::Serializer>(
    history: &[TimeFrame],
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.collect_seq(history.iter().map(|tf| {
        CycleDescription {
            cycle_no: tf.cycle_number.to_string(),
            core: CoreStateDescription {
                register_bank: tf
                    .core
                    .register_bank
                    .iter()
                    .map(|v| format!("{v:02x}"))
                    .collect(),
                xpsr: tf.core.xpsr.to_string(),
                control: tf.core.control.to_string(),
                stack_pointers: format!(
                    "[ MSP: {:02x} | PSP: {:02x} ]",
                    tf.core.stack_pointers.0, tf.core.stack_pointers.1
                ),
                stacking_mode: match tf.core.stacking_mode {
                    None => None,
                    Some(StackingMode::Stacking) => Some("STK"),
                    Some(StackingMode::Unstacking) => Some("UNSTK"),
                },
            },
            dbus: DBusStateDescription {
                request: tf.dbus.request.as_ref().map_or_else(
                    || "n/a".to_owned(),
                    |req| {
                        format!(
                            "{}{}",
                            match &req {
                                TransferType::Idle => "Idle",
                                TransferType::NonSeq(..) => "NonSeq",
                                TransferType::Seq(..) => "Seq",
                                TransferType::NoSel => "",
                                TransferType::_Busy => "Busy",
                            },
                            dife_lazy(
                                req.is_address_valid(),
                                || {
                                    let meta = req.meta().unwrap();
                                    format!(
                                        "{:?}{} {} @ {:?}",
                                        meta.dir,
                                        meta.size,
                                        match meta.burst {
                                            Burst::Single => "",
                                            Burst::Incr => "Incr",
                                        },
                                        meta.addr,
                                    )
                                },
                                || ""
                            ),
                        )
                    },
                ),
                response: match tf.dbus.response {
                    Some(AhbResponseControl::Success) => "Success",
                    Some(AhbResponseControl::Pending) => "Pending",
                    Some(AhbResponseControl::Error1) => "Error1",
                    Some(AhbResponseControl::Error2) => "Error2",
                    None => "n/a",
                },
                data: tf
                    .dbus
                    .set_data
                    .as_ref()
                    .map_or_else(|| "n/a".to_owned(), |data| format!("0x{data}")),
                responder: tf.dbus.responder.unwrap_or("n/a"),
            },
            dwt: DWTStateDescription {
                cyccnt: tf.dwt.registers.cycle_counter,
                cpicnt: tf.dwt.registers.cpi_counter,
                cpicnt_incremented: tf.dwt.cpicnt_incremented,
                exccnt: tf.dwt.registers.exception_counter,
                exccnt_incremented: tf.dwt.exccnt_incremented,
                sleepcnt: tf.dwt.registers.sleep_counter,
                sleepcnt_incremented: tf.dwt.sleepcnt_incremented,
                lsucnt: tf.dwt.registers.lsu_counter,
                lsucnt_incremented: tf.dwt.lsucnt_incremented,
                foldcnt: tf.dwt.registers.fold_counter,
                foldcnt_incremented: tf.dwt.foldcnt_incremented,
            },
            #[cfg(feature = "cdl-ahb-trace")]
            connections: tf
                .connections
                .iter()
                .map(|(name, cmfs)| {
                    (
                        name.to_string(),
                        ConnectionMultiFlopStateDescription {
                            request: cmfs.request.as_ref().map(|req| AhbRequestDescription {
                                state: format!(
                                    "{}{}{}{}",
                                    req.addr_phase.ready.ife("", "Nrdy "),
                                    req.addr_phase.lock.ife("L ", ""),
                                    match &req.addr_phase.meta {
                                        TransferType::Idle => "Idle",
                                        TransferType::NonSeq(..) => "NonSeq",
                                        TransferType::Seq(..) => "Seq",
                                        TransferType::NoSel => "",
                                        TransferType::_Busy => "Busy",
                                    },
                                    dife_lazy(
                                        req.addr_phase.meta.is_address_valid(),
                                        || {
                                            let meta = req.addr_phase.meta.meta().unwrap();
                                            format!(
                                                "{} {}",
                                                meta.size,
                                                match meta.burst {
                                                    Burst::Single => "",
                                                    Burst::Incr => "Incr",
                                                }
                                            )
                                        },
                                        || ""
                                    ),
                                ),
                                address: req.addr_phase.meta.address().map(|a| format!("{a:?}")),
                                direction: req
                                    .addr_phase
                                    .meta
                                    .lift(|m| m.is_writing().ife("Write", "Read")),
                                protection: req.addr_phase.meta.lift(|m| format!("{:?}", m.prot)),
                                addr_tag: req.addr_phase.tag.as_str(),
                                #[cfg(feature = "cdl-ahb-trace")]
                                addr_trace: {
                                    let cdl_tag = &req.addr_phase.tag;
                                    AhbTraceDescription {
                                        id: cdl_tag.get_id(),
                                        depth: cdl_tag.count_hops(),
                                        story: None, // TODO!
                                    }
                                },
                                data: req
                                    .data_phase
                                    .data
                                    .as_option()
                                    .map(|data| format!("{data}")),
                                data_tag: req.data_phase.tag.as_str(),
                                #[cfg(feature = "cdl-ahb-trace")]
                                data_trace: {
                                    let cdl_tag = &req.data_phase.tag;
                                    AhbTraceDescription {
                                        id: cdl_tag.get_id(),
                                        depth: cdl_tag.count_hops(),
                                        story: None, // TODO!
                                    }
                                },
                            }),

                            response: cmfs.response.as_ref().map(|resp| {
                                AhbResponseDescription {
                                    status: format!("{}", resp.meta),
                                    data: resp.data.as_option().map(|data| format!("{data}")),
                                    src_tag: resp.sender_tag.as_str(),
                                    #[cfg(feature = "cdl-ahb-trace")]
                                    src_trace: {
                                        let cdl_tag = &resp.sender_tag;
                                        AhbTraceDescription {
                                            id: cdl_tag.get_id(),
                                            depth: cdl_tag.count_hops(),
                                            story: None, // TODO!
                                        }
                                    },
                                    reply_tag: resp.responder_tag.as_str(),
                                    #[cfg(feature = "cdl-ahb-trace")]
                                    reply_trace: {
                                        let cdl_tag = &resp.responder_tag;
                                        AhbTraceDescription {
                                            id: cdl_tag.get_id(),
                                            depth: cdl_tag.count_hops(),
                                            story: None, // TODO!
                                        }
                                    },
                                }
                            }),
                        },
                    )
                })
                .collect(),
            free_status: tf
                .free_status
                .iter()
                .map(|(name, fs)| (*name, fs.to_string()))
                .collect(),
        }
    }))
}

fn serialize_events<S: serde::Serializer>(
    history: &[TimeFrame],
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.collect_map(history.iter().flat_map(|tf| {
        tf.events
            .iter()
            .map(|(addr, evs)| (addr_cycle_to_key(*addr, tf.cycle_number), evs))
    }))
}

fn serialize_via_debug<T: std::fmt::Debug, S: serde::Serializer>(
    value: &T,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.collect_str(&format_args!("{value:?}"))
}
