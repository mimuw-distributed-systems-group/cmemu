//! Module with types abstracting the AHB(-Lite) protocol signal lines.
//!
//! The signals in this module are generic over the data carrier type,
//! as the protocol allows for very wide lines (1024 bits!).
//!
//! Based on [ARM-AHB-Lite] = "AMBA 3 AHB-Lite Protocol" (ARM IHI 0033A) and
//! on [ARM-AMBA5] = "AMBA AHB Protocol Specification" (ARM IHI 0033C (ID090921)),
//! as we can logically use the signals from "full AHB" for both AHB-Lite and APB modeling.
//!
//! Note: AMBA5 changed the language "master" -> "manager", "slave" -> "subordinate".
//! For consistency with more-precise ARM and TI documents, we stick to their terms.
//!
//! ## Types design
//!
//! Types in this module are designed to be used with partial struct expressions:
//!
//! ```ignore
//! MasterToSlaveAddrPhase {
//!     meta,
//!     ..MasterToSlaveAddrPhase::empty::<P>()
//! }
//! ```

#[cfg(feature = "cycle-debug-logger")]
use crate::common::new_ahb::cdl::CdlTag;
#[cfg(feature = "cycle-debug-logger")]
pub(crate) use crate::common::new_ahb::databus::{M2SBus, S2MBus};
use crate::common::new_ahb::ports::AHBPortConfig;
#[cfg(feature = "cdl-ahb-trace")]
use crate::common::new_ahb::ports::ConnectionName;
use crate::engine::Context;
#[cfg(debug_assertions)]
use crate::engine::TransitionValidator;
#[cfg(feature = "cdl-ahb-trace")]
use crate::proxy::CycleDebugLoggerProxy;
use cmemu_common::Address;
use owo_colors::OwoColorize;
#[cfg(feature = "cdl-ahb-trace")]
use std::any::Any;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Deref, Range};
use wire::{HIGH, LOW};

// Basic types and primary helper accessors
pub(crate) type BinaryWire = bool;

pub mod wire {
    pub(crate) const LOW: bool = false;
    pub(crate) const HIGH: bool = true;
}

// Main types of passed data

// Master to Slave (request)

/// A bundle of all wires sent from upstream to downstream.
///
/// We include all the signals marked with "downstream" modules as "Destination",
/// so it also includes signals by the Decoder and Multiplexor.
///
/// The struct groups the signals into parts related to the address and data phases.
/// For consistency and invariants, we require sending these as messages through the emu queue.
/// Actually, only the actual data lines are part of the data phase.
///
/// Use [`Self::empty`] to properly create a base struct and
/// [`Self::stamp_departure`] to properly track components visited by this message.
///
/// See: [ARM-AHB-Lite] 2.2 Master signals, 2.4 Decoder signals, and 2.5 Multiplexor signals
#[cfg_attr(test, derive(PartialEq, Eq))]
#[derive(Default, Clone)]
pub(crate) struct MasterToSlaveWires<D: Default + 'static> {
    pub(crate) addr_phase: MasterToSlaveAddrPhase,
    pub(crate) data_phase: MasterToSlaveDataPhase<D>,
}

/// A bundle of wires for the address phase sent from master (upstream) to a slave (downstream).
///
/// Use [`Self::empty`] or [`Self::not_selected`] to properly create a new base struct.
///
/// See: [ARM-AHB-Lite] 2.2 Master signals, 2.4 Decoder signals, and 2.5 Multiplexor signals
#[cfg_attr(test, derive(PartialEq, Eq))]
#[derive(Clone)]
pub(crate) struct MasterToSlaveAddrPhase {
    /// Combines `HADDR`, `HBURST`, `HPROT`, `HSIZE`, `HTRANS`, `HWRITE`, and `HSELx` semantically.
    pub(crate) meta: TransferType,

    /// [ARM-AHB-Lite] 2.2 Master signals: `HMASTLOCK`
    ///
    /// If high, the transfer is considered part of a sequence of locked transfers.
    pub(crate) lock: BinaryWire, // maybe HIGH on idle?

    /// Formally not a Master signal in lite, but added by the multiplexor.
    /// See also a note to [ARM-AHB-Lite] 2.4 Decoder signals – `HSELx`
    // TODO: add a way to indicate if this wire is present (best at type level)
    pub(crate) ready: BinaryWire,

    /// CMEmu feature for tracing the messages (the actual type is considered opaque here).
    #[cfg(feature = "cycle-debug-logger")]
    pub(crate) tag: CdlTag,
}

// Here for making aliasing visible
impl MasterToSlaveAddrPhase {
    /// Alias for `HMASTLOCK`
    #[allow(non_snake_case, dead_code)]
    pub(crate) fn HMASTLOCK(&self) -> BinaryWire {
        self.lock
    }
    /// Alias for `HREADY` multiplexor -> slave signal as referred to by some docs.
    #[allow(non_snake_case)]
    pub(crate) fn HREADYIN(&self) -> BinaryWire {
        self.ready
    }

    /// Does the address phase wires matter?
    pub fn is_selected(&self) -> bool {
        self.meta.is_selected()
    }

    /// If not for the `SlaveToMasterWires`, would the transfer advance to a data phase
    /// with an address?
    pub fn advances_to_valid(&self) -> bool {
        self.ready && self.meta.is_address_valid()
    }
}

/// Data phase wires from upstream to downstream with tracking information.
///
/// The `D` is generic over the `HWDATA` signal, which can have a configurable width.
/// Use [`Self::continue_read`], [`Self::continue_write`], [`Self::empty`] to properly create a base struct.
#[cfg_attr(test, derive(PartialEq, Eq))]
#[derive(Default, Clone)]
pub(crate) struct MasterToSlaveDataPhase<D: Default + 'static> {
    ///  [ARM-AHB-Lite] 2.2 Master signals: `HWDATA`
    pub(crate) data: D,

    /// CMEmu feature for tracing the messages (the actual type is considered opaque here).
    ///
    /// This tag enables us to match the data phase to the address phase in the logs.
    /// Use [`MasterToSlaveWires::stamp_departure`] to properly track components visited by this message.
    #[cfg(feature = "cycle-debug-logger")]
    pub(crate) tag: CdlTag,
}

// Slave to Master (response)

/// A bundle of all wires sent from downstream to upstream.
///
/// We include all the signals marked with "upstream" modules as "Destination",
/// so it also includes signals by the Decoder and Multiplexor.
/// There are no signals associated with an address phase here, as these would
/// have combinatorial dependence on the request.
///
/// The `D` is generic over the `HRDATA` signal, which can have a configurable width.
///
/// Use [`MasterToSlaveAddrPhase::make_reply`],
/// [`Self::empty`], [`Self::empty_reply`], [`Self::empty_addr_reply`],
/// [`Self::empty_takeover_reply`], or [`Self::takeover_reply`]
/// to properly create a base instance of this struct.
///
/// See: [ARM-AHB-Lite] 2.3 Slave signals, 2.4 Decoder signals, and 2.5 Multiplexor signals
#[cfg_attr(test, derive(PartialEq, Eq))]
#[derive(Clone)]
pub(crate) struct SlaveToMasterWires<D: 'static> {
    ///  Semantic encoding of [ARM-AHB-Lite] 2.3 Slave signals: `HREADYOUT` and `HRESP`
    pub(crate) meta: AhbResponseControl,
    ///  [ARM-AHB-Lite] 2.3 Slave signals: `HRDATA`
    pub(crate) data: D,

    /// CMEmu feature for tracing the messages (the actual type is considered opaque here).
    ///
    /// This field allows us to match the request that led to the generation of this response.
    #[cfg(feature = "cycle-debug-logger")]
    pub(crate) sender_tag: CdlTag,
    /// CMEmu feature for tracing the return messages (the actual type is considered opaque here).
    ///
    /// Use [`Self::stamp_departure`] to properly track components visited by this message.
    #[cfg(feature = "cycle-debug-logger")]
    pub(crate) responder_tag: CdlTag,
}

/// Slave transfer response: `HREADYOUT` x `HRESP`.
///
/// See [ARM-AHB-Lite] Table 5-2 Transfer response
#[derive(Eq, PartialEq, Clone, Copy)]
pub enum AhbResponseControl {
    Success,
    Pending,
    /// First cycle of an ERROR response
    Error1,
    /// Second cycle of an ERROR response
    Error2,
}

/// [ARM-AHB-Lite] 2.3 Slave signals; and Table 5-1 HRESP signal
#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum HRESP {
    OKAY = 0,
    ERROR = 1,
}

impl AhbResponseControl {
    /// [ARM-AHB-Lite] 2.3 Slave signals: `HREADYOUT`
    ///
    /// See [ARM-AHB-Lite] Table 5-2 Transfer response
    #[allow(non_snake_case)]
    pub fn HREADYOUT(self) -> BinaryWire {
        matches!(self, Self::Success | Self::Error2)
    }
    /// Alias to `HREADYOUT` as referred to by some parts of the documentation.
    #[allow(non_snake_case)]
    pub fn HREADY(self) -> BinaryWire {
        self.HREADYOUT()
    }
    /// [ARM-AHB-Lite] 2.3 Slave signals: `HRESP`
    ///
    /// See [ARM-AHB-Lite] Table 5-2 Transfer response
    #[allow(non_snake_case)]
    pub fn HRESP(self) -> HRESP {
        match self {
            Self::Error1 | Self::Error2 => HRESP::ERROR,
            Self::Success | Self::Pending => HRESP::OKAY,
        }
    }

    /// Is the transfer successfully finished?
    pub fn is_done(self) -> bool {
        self == Self::Success
    }
    /// Does this transfer require another cycle?
    pub fn is_waitstate(self) -> bool {
        matches!(self, Self::Pending | Self::Error1)
    }
}

// Master

/// A bundle of wires for the address phase sent from master (upstream) to a slave (downstream),
/// whose meaning depends on the `HTRANS` and `HSELx` wires.
///
/// Most Slave wires are ignored when the `HSELx` is low or `HTRANS` is Idle or Busy.
///
/// See: [ARM-AHB-Lite] 3.2 Transfer types
#[cfg_attr(test, derive(PartialEq, Eq))]
#[derive(Debug, Clone)]
pub enum TransferType {
    /// [ARM-AHB-Lite] 3.2 Transfer types: `b00`
    ///
    /// Note:
    /// > Locked transfers are recommended to end with Idle
    /// > Slaves always respond with Success (OKAY + HREADY)
    ///
    /// In CMEmu, slave components are allowed to not send a response if the current data phase
    /// is not active.
    Idle, // = 0b00
    /// [ARM-AHB-Lite] 3.2 Transfer types: `b01`
    ///
    /// TODO: ref that 3/4 is not issuing Busy transfers
    _Busy, // = 0b01
    /// [ARM-AHB-Lite] 3.2 Transfer types: `b10`
    ///
    /// Unrelated to previous transfer, the first transfer of burst is always nonseq.
    /// Used most of the time.
    NonSeq(TransferMeta), // 0b10
    /// [ARM-AHB-Lite] 3.2 Transfer types: `b11`
    ///
    /// Following transfers in bursts.
    /// The rest of the metadata must match.
    #[allow(dead_code, reason = "Bursts not yet implemented")]
    Seq(TransferMeta), // 0b11

    /// See: [ARM-AHB-Lite] 2.4 Decoder signals: `HSELx`
    ///
    /// We model the special case of low `HSELx`, to indicate that `AddressPhase` wires are invalid.
    /// Therefore, if HSEL is high, we just have the above variants.
    /// Sometimes we need to just send data phase, so this variant is to fill the address wires.
    ///
    /// In CMEmu, a lack of a message is equivalent to `NoSel` or `Idle`.
    /// With `NoSel`, the slave should ignore the address phase of a message.
    NoSel,
}

impl TransferType {
    /// Construct a new "single transfer" based on `HADDR`, `HSIZE`, `HWRITE`, and `HPROT` wires.
    ///
    /// [ARM-AHB-Lite] 3.2 Transfer types: This is a non-sequential transfer with specific `HBURST` value.
    pub fn new_single(addr: Address, size: Size, dir: Direction, protection: Protection) -> Self {
        Self::NonSeq(TransferMeta {
            addr,
            size,
            burst: Burst::Single,
            dir,
            prot: protection,
        })
    }

    /// Addr is not valid for `Idle`, `Busy`, and `NoSel`.
    pub fn is_address_valid(&self) -> bool {
        matches!(self, TransferType::Seq(_) | TransferType::NonSeq(_))
    }
    /// Run a closure if the transfer is valid, otherwise return `None`.
    pub fn lift<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&TransferMeta) -> R,
    {
        use TransferType::*;
        match self {
            Idle | _Busy | NoSel => None,
            NonSeq(m) | Seq(m) => Some(f(m)),
        }
    }
    /// Run a closure if the transfer is valid, otherwise return `false`.
    pub fn is_address_valid_and<F>(&self, f: F) -> bool
    where
        F: FnOnce(&TransferMeta) -> bool,
    {
        use TransferType::*;
        match self {
            Idle | _Busy | NoSel => false,
            NonSeq(m) | Seq(m) => f(m),
        }
    }
    /// Get the transfer address if the wires are valid.
    pub fn address(&self) -> Option<Address> {
        self.lift(|m| m.addr)
    }
    /// Get the transfer size if the wires are valid.
    #[cfg_attr(not(debug_assertions), allow(unused))]
    pub fn size(&self) -> Option<Size> {
        self.lift(|m| m.size)
    }
    /// Get the transfer direction if the wires are valid, otherwise false.
    pub fn is_writing(&self) -> bool {
        self.lift(|m| m.dir == Direction::Write).unwrap_or(false)
    }
    /// Get the transfer protection value for caching if the wires are valid, otherwise false.
    pub fn is_cacheable(&self) -> bool {
        self.lift(|m| m.prot.is_cacheable && m.is_reading())
            .unwrap_or(false)
    }
    /// Get the transfer protection value for buffering if the wires are valid, otherwise false.
    pub fn is_bufferable(&self) -> bool {
        self.lift(|m| m.prot.is_bufferable && m.is_writing())
            .unwrap_or(false)
    }
    /// Get the transfer protection value for privilege if the wires are valid, otherwise false.
    #[allow(dead_code, reason = "To be used in an MPU")]
    pub fn is_unprivileged(&self) -> bool {
        // we do is_unprivileged, since IDLEs ignore HPROT, and it is easier no to trigger warnings,
        // and otherwise do not suggest that the transfer is privileged.
        self.lift(|m| !m.prot.is_privileged).unwrap_or(false)
    }

    /// Get the transfer wires if they are valid.
    pub fn meta(&self) -> Option<&TransferMeta> {
        use TransferType::*;
        match self {
            Idle | _Busy | NoSel => None,
            NonSeq(m) | Seq(m) => Some(m),
        }
    }
    /// Get a mutable reference to the transfer wires if they are valid.
    pub fn meta_mut(&mut self) -> Option<&mut TransferMeta> {
        use TransferType::*;
        match self {
            Idle | _Busy | NoSel => None,
            NonSeq(m) | Seq(m) => Some(m),
        }
    }

    /// Should the slave consider the transfer like idle? `NoSel` is considered like this.
    pub fn is_idle(&self) -> bool {
        // NoSel is implicitly idle
        matches!(self, TransferType::Idle | TransferType::NoSel)
    }
    /// Interpret the enum as a raw `HSELx` wire.
    pub fn is_selected(&self) -> bool {
        !matches!(self, TransferType::NoSel)
    }
}

/// Direction (Read/Write) of the transfer.
///
/// [ARM-AHB-Lite] 2.2 Master signals; 3.1 Basic transfers
#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum Direction {
    Read = 0,
    Write = 1,
}

/// A bundle of wires for the address phase sent from master (upstream) to a slave (downstream),
/// already checked for validity.
///
/// [ARM-AHB-Lite] 2.3 Slave signals
#[cfg_attr(test, derive(PartialEq, Eq))]
#[derive(Clone)]
pub struct TransferMeta {
    /// Address. Corresponds to `HADDR`
    pub addr: Address,
    /// Size of the transfer. Corresponds to `HSIZE`
    pub size: Size,
    /// Multi-transfer mode. Corresponds to `HBURST`
    pub burst: Burst,
    /// Read or write. Corresponds to `HWRITE`
    pub dir: Direction,
    /// Protection wires. Corresponds to `HPROT`
    pub prot: Protection,
}

impl TransferMeta {
    /// Alias to access the wires as used by the docs.
    #[allow(non_snake_case, unused)]
    pub fn HADDR(&self) -> Address {
        self.addr
    }
    /// Alias to access the wires as used by the docs.
    #[allow(non_snake_case, unused)]
    pub fn HSIZE(&self) -> Size {
        self.size
    }
    /// Alias to access the wires as used by the docs.
    #[allow(non_snake_case, unused)]
    pub fn HBURST(&self) -> Burst {
        self.burst
    }
    /// Alias to access the wires as used by the docs.
    #[allow(non_snake_case, unused)]
    pub fn HWRITE(&self) -> BinaryWire {
        self.is_writing()
    }
    /// Alias to access the wires as used by the docs.
    #[allow(non_snake_case, unused)]
    pub fn HPROT(&self) -> Protection {
        self.prot
    }

    /// Is it a Write transfer?
    pub fn is_writing(&self) -> bool {
        self.dir == Direction::Write
    }
    /// Is it a Read transfer?
    pub fn is_reading(&self) -> bool {
        self.dir == Direction::Read
    }
    /// Is the address aligned in respect to the size?
    pub fn is_aligned(&self) -> bool {
        self.size.is_addr_aligned(self.addr)
    }

    /// Get a range (open) of addresses covered by this transfer.
    #[cfg_attr(not(feature = "cycle-debug-logger"), allow(dead_code))]
    pub fn addr_range(&self) -> Range<Address> {
        self.addr..self.size.shift_addr(self.addr)
    }
}

/// Transfer sizes per [ARM-AHB-Lite] 3.5 Transfer size
///
/// The names come from [ARM-AHB-Lite] Table 3-2 Transfer size encoding.
/// Note: the discriminant is size in bytes rather than the table.
#[derive(Eq, PartialEq, Debug, Clone, Copy, Ord, PartialOrd)]
pub enum Size {
    Byte = 1,
    Halfword = 2,
    Word = 4,
    Doubleword = 8,
    FourWord = 16,
    EightWord = 32,
    _512 = 64,
    _1024 = 128,
}

impl Size {
    /// Get the size in bytes (as usize).
    pub fn bytes(self) -> usize {
        self as usize
    }
    /// Get the size in bytes (as u32).
    pub fn bytes32(self) -> u32 {
        self as u32
    }
    /// Get the size in bytes (as u8).
    pub fn bytes8(self) -> u8 {
        self as u8
    }

    /// Get the size in bits (as usize).
    pub fn bits(self) -> usize {
        self.bytes() * 8
    }

    /// Check if the address is aligned with relation to the size.
    pub fn is_addr_aligned(self, addr: Address) -> bool {
        self.offset_from_aligned(addr) == 0
    }

    /// Get the remainder of the address modulo the addressable size.
    #[must_use]
    pub fn offset_from_aligned(self, addr: Address) -> usize {
        addr.masked(self as u32 - 1).to_const() as usize
    }

    /// Align the address (down) to the size
    #[must_use]
    pub fn align_addr(self, addr: Address) -> Address {
        addr.masked(!(self as u32 - 1))
    }

    /// Shift the address by the size number of bytes
    #[must_use]
    pub fn shift_addr(self, addr: Address) -> Address {
        addr.offset(self as u32)
    }

    /// Get the largest size that doesn't exceed `limit` bytes, but which also can be transferred through `DataBus`.
    ///
    /// # Panics
    /// Panics on 0.
    #[must_use]
    pub fn largest_databusable_fit(limit: usize) -> Self {
        match limit {
            0 => panic!("looking for the largest databusable fit for 0 bytes"),
            1 => Self::Byte,
            2..=3 => Self::Halfword,
            4..=7 => Self::Word,
            _ => Self::Doubleword,
        }
    }
}

/// Burst transfer wires as per [ARM-AHB-Lite] 3.5 Burst operation
#[derive(Eq, PartialEq, Debug, Clone, Copy)]
#[non_exhaustive]
pub enum Burst {
    /// Single transfer
    Single = 0b000,
    /// Incrementing address with undefined length
    #[allow(dead_code)] // TODO: implement bursts
    Incr = 0b001,
    // TODO: ref -> CM3/4 sends only single / unbounded
    // Those are left commented to make uncommenting them fail match exhaustiveness checks
    //     _Wrap4 = 0b010,
    //     _Incr4 = 0b011,
    //     _Wrap8 = 0b100,
    //     _Incr8 = 0b101,
    //     _Wrap16 = 0b110,
    //     _Incr16 = 0b111,
}

/// Transfer protection wires as per [ARM-AHB-Lite] 3.7 Protection control
#[derive(Eq, PartialEq, Clone, Copy)]
pub struct Protection {
    /// Corresponds to `HPROT[0]`: Data/Opcode
    pub(crate) is_data: BinaryWire, // opcode otherwise
    /// Corresponds to `HPROT[1]`: Privileged/Opcode
    pub(crate) is_privileged: BinaryWire,
    /// Corresponds to `HPROT[2]`: Bufferable
    pub(crate) is_bufferable: BinaryWire,
    /// Corresponds to `HPROT[3]`: Cacheable
    pub(crate) is_cacheable: BinaryWire,
}

impl Protection {
    /// Create a data transfer, which is privileged, bufferable, and cacheable
    #[must_use]
    pub const fn new_data() -> Self {
        Self {
            is_data: true,
            is_privileged: true,
            is_bufferable: true,
            is_cacheable: true,
        }
    }
    /// Create an instruction transfer, which is privileged, bufferable, and cacheable
    #[must_use]
    pub const fn new_instruction() -> Self {
        Self {
            is_data: false,
            is_privileged: true,
            is_bufferable: true,
            is_cacheable: true,
        }
    }

    /// Is it an opcode transfer? (For instance, to deny non-executable regions)
    #[must_use]
    pub fn is_instruction(self) -> bool {
        !self.is_data
    }

    /// Return new protocol wires with the `bufferable` value overridden.
    #[allow(dead_code, reason = "Completeness of API")]
    #[must_use]
    pub fn with_bufferable(self, new_val: BinaryWire) -> Self {
        Self {
            is_bufferable: new_val,
            ..self
        }
    }

    /// Return new protocol wires with the `cacheable` value overridden.
    #[must_use]
    pub fn with_cacheable(self, new_val: BinaryWire) -> Self {
        Self {
            is_cacheable: new_val,
            ..self
        }
    }
}

/// A generic single signal (lane) with tracking
///
/// Used when sending out-of-bound data (like grant wires).
/// The `D` is just the data sent (usually a bool).
///
/// Use constructors to properly create a base instance of this struct.
#[cfg_attr(test, derive(PartialEq, Eq))]
#[cfg_attr(not(feature = "cycle-debug-logger"), repr(transparent))]
#[derive(Default, Clone)]
pub(crate) struct TrackedWire<D: 'static> {
    pub(crate) data: D,

    /// Use [`Self::stamp_departure`] to properly track components visited by this message.
    #[cfg(feature = "cycle-debug-logger")]
    pub(crate) tag: CdlTag,
}

impl<D: 'static + Copy> Deref for TrackedWire<D> {
    type Target = D;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

//////////////////////////////////////////////////////////
//   IMPLS of non-essential APIs (debug, tracing, tests) /
//////////////////////////////////////////////////////////

impl<D: 'static + Debug> Debug for TrackedWire<D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        #[cfg(feature = "cycle-debug-logger")]
        {
            write!(f, "{}", "[".cyan())?;
            write!(f, "{:?}-> ", self.tag)?;
            write!(f, "{:?}", self.data.bold())?;
            write!(f, "{}", "]".cyan())?;
        };
        #[cfg(not(feature = "cycle-debug-logger"))]
        self.data.fmt(f)?;
        Ok(())
    }
}

impl<D> TrackedWire<D> {
    /// (Possibly) track sending the message from `SRC` to `DST`
    #[allow(unused_mut)]
    #[must_use]
    pub(crate) fn stamp_departure<SRC: AHBPortConfig, DST: AHBPortConfig>(
        mut self,
        #[cfg_attr(not(feature = "cdl-ahb-trace"), allow(unused))] ctx: &mut Context,
    ) -> Self {
        #[cfg(feature = "cdl-ahb-trace")]
        {
            self.tag.stamp::<SRC, DST>(ctx);
            // TODO: implement single wires in CDL
        }
        self
    }
}

pub(crate) type TrackedBool = TrackedWire<bool>;

#[cfg_attr(not(feature = "cycle-debug-logger"), allow(unused, unused_variables))]
impl TrackedBool {
    /// Just `true` with tag based on the `P` port.
    #[must_use]
    pub(crate) fn true_<P: AHBPortConfig + ?Sized>() -> Self {
        // TODO: make const when const traits are stable
        Self {
            data: true,
            #[cfg(feature = "cycle-debug-logger")]
            tag: P::TAG.into(),
        }
    }

    /// Just `true` with tag based on the responder tag of `s2m`.
    #[must_use]
    pub(crate) fn true_from_s2m<P: AHBPortConfig + ?Sized, D: Default + 'static>(
        s2m: &SlaveToMasterWires<D>,
    ) -> Self {
        Self {
            data: true,
            #[cfg(feature = "cycle-debug-logger")]
            tag: s2m.responder_tag.reply_trace(P::TAG),
        }
    }

    /// Just `false` with tag based on the `P` port.
    #[must_use]
    pub(crate) fn false_<P: AHBPortConfig + ?Sized>() -> Self {
        Self {
            data: false,
            #[cfg(feature = "cycle-debug-logger")]
            tag: P::TAG.into(),
        }
    }

    /// Just `false` with tag based on the responder tag of `s2m`.
    #[must_use]
    pub(crate) fn false_from_s2m<P: AHBPortConfig + ?Sized, D: Default + 'static>(
        s2m: &SlaveToMasterWires<D>,
    ) -> Self {
        Self {
            data: false,
            #[cfg(feature = "cycle-debug-logger")]
            tag: s2m.responder_tag.reply_trace(P::TAG),
        }
    }
}

impl<D: Debug> Debug for SlaveToMasterWires<D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", "S2M wires".bright_cyan(), "{".cyan())?;
        write!(f, "{:?}", self.meta)?;
        write!(f, ", {} {:?}", "D:".cyan(), self.data.bright_cyan().bold())?;
        #[cfg(feature = "cycle-debug-logger")]
        {
            write!(f, " from {:?}", self.responder_tag)?;
            write!(f, " {} {:?}", "in reply".bright_magenta(), self.sender_tag)?;
        }
        write!(f, "{}", "}".cyan())?;
        Ok(())
    }
}
impl Display for Size {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u32)
    }
}

impl Debug for AhbResponseControl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success => write!(f, "{}", "Success".green().bold()),
            Self::Pending => write!(f, "{}", "Pending".yellow().bold()),
            Self::Error1 => write!(f, "{}", "Error1".bright_red().bold()),
            Self::Error2 => write!(f, "{}", "Error2".bright_magenta().bold()),
        }
    }
}
impl Display for AhbResponseControl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success => write!(f, "Success"),
            Self::Pending => write!(f, "Pending"),
            Self::Error1 => write!(f, "Error1"),
            Self::Error2 => write!(f, "Error2"),
        }
    }
}

impl<D: Default + Debug> Debug for MasterToSlaveWires<D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", "M2S wires".bright_purple().bold(), "{".cyan())?;
        write!(f, "{}: {:?}", "AddrPh".purple().bold(), self.addr_phase)?;
        write!(f, ", {}: {:?}", "DataPh".blue().bold(), self.data_phase)?;
        write!(f, "{}", "}".cyan())?;
        Ok(())
    }
}

#[cfg(debug_assertions)]
impl TransitionValidator for AhbResponseControl {
    fn assert_is_valid_transition(&self, next: &Self) {
        // [ARM-AHB] 5.1.2, 5.1.3
        match (self, next) {
            (Self::Success | Self::Pending | Self::Error2, Self::Error2) => {
                panic!("Invalid transition to the second cycle of error");
            }
            (Self::Error1, Self::Success | Self::Pending | Self::Error1) => {
                panic!("Invalid transition from a cycle of error");
            }
            _ => (),
        }
    }
}

impl Default for Protection {
    fn default() -> Self {
        Self {
            is_data: true,
            is_privileged: true,
            is_bufferable: true,
            is_cacheable: true,
        }
    }
}

impl Debug for Protection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}",
            if self.is_data { "D" } else { "i" },
            if self.is_privileged { "P" } else { "p" },
            if self.is_bufferable { "B" } else { "b" },
            if self.is_cacheable { "C" } else { "c" }
        )
    }
}

impl Default for MasterToSlaveAddrPhase {
    fn default() -> Self {
        Self {
            meta: TransferType::Idle,
            lock: LOW,
            ready: HIGH,
            #[cfg(feature = "cycle-debug-logger")]
            tag: Default::default(),
        }
    }
}

impl From<TransferType> for MasterToSlaveAddrPhase {
    fn from(meta: TransferType) -> Self {
        Self {
            meta,
            ..Default::default()
        }
    }
}

#[cfg_attr(not(feature = "cycle-debug-logger"), allow(unused, unused_variables))]
impl<D: Default + 'static> SlaveToMasterWires<D> {
    /// Make an empty response (success, default value) with a **missing** `sender_tag`,
    /// and `responder_tag` coming from the `P` port.
    #[must_use]
    pub(crate) fn empty<P: AHBPortConfig>() -> Self {
        Self {
            meta: AhbResponseControl::Success,
            data: Default::default(),
            #[cfg(feature = "cycle-debug-logger")]
            sender_tag: Default::default(),
            #[cfg(feature = "cycle-debug-logger")]
            responder_tag: P::TAG.into(),
        }
    }
    /// Make an empty response (success, default value) with proper `sender_tag`,
    /// and `responder_tag` coming from the `P` port.
    #[allow(dead_code)]
    #[must_use]
    pub(crate) fn empty_reply<P: AHBPortConfig>(addr: &MasterToSlaveDataPhase<D>) -> Self {
        Self {
            #[cfg(feature = "cycle-debug-logger")]
            sender_tag: addr.tag.reply_trace(P::TAG),
            ..Self::empty::<P>()
        }
    }
    /// Make an empty response (success, default value) with proper `sender_tag`,
    /// and `responder_tag` coming from the `P` port.
    #[must_use]
    pub(crate) fn empty_addr_reply<P: AHBPortConfig>(addr: &MasterToSlaveAddrPhase) -> Self {
        Self {
            #[cfg(feature = "cycle-debug-logger")]
            sender_tag: addr.tag.reply_trace(P::TAG),
            ..Self::empty::<P>()
        }
    }
    /// Make an empty response (success, default value) with `sender_tag` taken from `s2m`,
    /// and `responder_tag` coming from the `P` port.
    #[allow(dead_code)]
    #[must_use]
    pub(crate) fn empty_takeover_reply<P: AHBPortConfig, DS: Default + 'static>(
        s2m: &SlaveToMasterWires<DS>,
    ) -> Self {
        Self {
            #[cfg(feature = "cycle-debug-logger")]
            sender_tag: s2m.sender_tag.clone(),
            ..Self::empty::<P>()
        }
    }
    /// Take over the reply by substituting our own `responder_tag` from the `P` port.
    ///
    /// Useful when we completely modify the response, so we need the `sender_tag`,
    /// but the `responder_tag` should not be associated with the original response message.
    #[must_use]
    pub(crate) fn takeover_reply<P: AHBPortConfig>(s2m: Self) -> Self {
        Self {
            #[cfg(feature = "cycle-debug-logger")]
            responder_tag: P::TAG.into(),
            ..s2m
        }
    }

    /// (Possibly) track sending the message from `SRC` to `DST`
    ///
    /// This is done automatically by the `bridge_ports!` macro.
    #[allow(unused_mut)]
    #[must_use]
    pub(crate) fn stamp_departure<SRC: AHBPortConfig, DST: AHBPortConfig>(
        mut self,
        #[cfg_attr(not(feature = "cdl-ahb-trace"), allow(unused))] ctx: &mut Context,
    ) -> Self {
        #[cfg(feature = "cdl-ahb-trace")]
        {
            self.responder_tag.stamp::<SRC, DST>(ctx);
            match (&self as &dyn Any).downcast_ref::<S2MBus>() {
                Some(s2mbus) => CycleDebugLoggerProxy.on_connection_s2m_databus(
                    ctx,
                    ConnectionName::new_from::<DST, SRC>(),
                    s2mbus.clone(),
                ),
                None => None,
            };
        }
        self
    }
}

#[cfg_attr(not(test), allow(dead_code))]
impl MasterToSlaveAddrPhase {
    /// Make an empty transfer (Idle, ready, unclocked) with a proper `tag` based on the `P` port.
    #[must_use]
    pub(crate) fn empty<P: AHBPortConfig>() -> Self {
        Self {
            meta: TransferType::Idle,
            lock: LOW,
            ready: HIGH,
            #[cfg(feature = "cycle-debug-logger")]
            tag: P::TAG.into(),
        }
    }

    /// Make a `NoSel` placeholder message with a proper `tag` based on the `P` port.
    #[must_use]
    pub(crate) fn not_selected<P: AHBPortConfig>() -> Self {
        Self {
            meta: TransferType::NoSel,
            // if someone looks only on HREADY without checking for selection first,
            // HREADY still makes sense when no further transfer is requested,
            // but something needs this signal to drive a state-machine (e.g., output stage)
            ready: HIGH,
            ..Self::empty::<P>()
        }
    }
}

#[cfg_attr(not(test), allow(dead_code))]
impl<D: Default> MasterToSlaveDataPhase<D> {
    /// Make an empty data phase (default data) with a new `tag` based on the `P` port.
    #[must_use]
    pub(crate) fn empty<P: AHBPortConfig>() -> Self {
        Self {
            data: Default::default(),
            #[cfg(feature = "cycle-debug-logger")]
            tag: P::TAG.into(),
        }
    }

    /// Make an empty data phase (default data) with a proper `tag` indicating continuation
    /// of the transfer in `adr`.
    #[must_use]
    pub(crate) fn continue_read(addr: &MasterToSlaveAddrPhase) -> Self {
        debug_assert!(!addr.meta.is_writing());
        Self {
            data: Default::default(),
            #[cfg(feature = "cycle-debug-logger")]
            tag: addr.tag.clone(),
        }
    }

    /// Make a data phase with a proper `tag` indicating continuation
    /// of the transfer in `adr`.
    #[allow(dead_code, reason = "Maybe use or remove if not actually useful.")]
    #[must_use]
    pub(crate) fn continue_write(addr: &MasterToSlaveAddrPhase, data: D) -> Self {
        debug_assert!(!addr.meta.is_writing());
        Self {
            data,
            #[cfg(feature = "cycle-debug-logger")]
            tag: addr.tag.clone(),
        }
    }
}

impl Debug for TransferMeta {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:?}", self.burst)?;
        if self.dir == Direction::Write {
            write!(f, " {}", "Write".yellow().bold())?;
        } else {
            write!(f, " {}", "Read".green().bold())?;
        }
        write!(f, " {}", (self.size as usize).bold().yellow())?;
        write!(f, " @ {:?}", self.addr.bright_red())?;
        write!(f, " prot={:?}", self.prot)?;
        write!(f, "]")
    }
}

impl<D: Default + Debug> Debug for MasterToSlaveDataPhase<D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Data{}", "{".magenta())?;
        #[cfg(feature = "cycle-debug-logger")]
        write!(f, " [{:?}]-> ", self.tag)?;
        write!(f, "{:?}", self.data.bright_cyan().bold())?;
        write!(f, "{}", "}".magenta())?;
        Ok(())
    }
}

impl Debug for MasterToSlaveAddrPhase {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Control{{")?;
        #[cfg(feature = "cycle-debug-logger")]
        write!(f, "[{:?}]-> ", self.tag)?;
        if self.ready {
            write!(f, "{}", "RDY".green())?;
        } else {
            write!(f, "{}", "!NOT rdy!".bright_red())?;
        }
        match &self.meta {
            TransferType::Idle => write!(f, "{}", " IDLE".cyan()),
            TransferType::_Busy => write!(f, "{}", " BUSY".red()),
            TransferType::NonSeq(t) => write!(f, " {}: {:?}", "NonSeq".green(), t),
            TransferType::Seq(t) => write!(f, " {}: {:?}", "Seq".yellow(), t),
            TransferType::NoSel => write!(f, "{}", " NoSel".bright_red()),
        }?;
        if self.lock {
            write!(f, " LOCKed")?;
        }
        write!(f, "}}")
    }
}

#[cfg_attr(not(test), allow(dead_code))]
impl<D: Default + 'static> MasterToSlaveWires<D> {
    /// Make an empty wires bundle (idle, default data) with a proper `tag` based on the `P` port.
    #[allow(dead_code)]
    #[must_use]
    pub(crate) fn empty<P: AHBPortConfig>() -> Self {
        Self {
            addr_phase: MasterToSlaveAddrPhase::empty::<P>(),
            data_phase: MasterToSlaveDataPhase::empty::<P>(),
        }
    }

    /// (Possibly) track sending the message from `SRC` to `DST`
    ///
    /// This is done automatically by the `bridge_ports!` macro.
    #[allow(unused_mut)]
    #[must_use]
    pub(crate) fn stamp_departure<SRC: AHBPortConfig, DST: AHBPortConfig>(
        mut self,
        #[cfg_attr(not(feature = "cdl-ahb-trace"), allow(unused))] ctx: &mut Context,
    ) -> Self {
        #[cfg(feature = "cdl-ahb-trace")]
        {
            self.addr_phase.tag.stamp::<SRC, DST>(ctx);
            self.data_phase.tag.stamp::<SRC, DST>(ctx);
            match (&self as &dyn Any).downcast_ref::<M2SBus>() {
                Some(m2sbus) => CycleDebugLoggerProxy.on_connection_m2s_databus(
                    ctx,
                    ConnectionName::new_from::<SRC, DST>(),
                    m2sbus.clone(),
                ),
                None => None,
            };
        }
        self
    }
}

impl MasterToSlaveAddrPhase {
    /// Make a [`SlaveToMasterWires`] with status and data, properly tracking the origin
    /// and responder tag based on the `P` port.
    #[must_use]
    pub(crate) fn make_reply<P: AHBPortConfig, D: Default>(
        &self,
        status: AhbResponseControl,
        data: D,
    ) -> SlaveToMasterWires<D> {
        SlaveToMasterWires {
            meta: status,
            data,
            ..SlaveToMasterWires::empty_addr_reply::<P>(self)
        }
    }

    /// Shortcut to create nonsequential single word-sized, data transfer in tests.
    #[cfg(test)]
    #[must_use]
    pub fn nonseq(addr: Address, dir: Direction) -> Self {
        Self {
            meta: TransferType::NonSeq(TransferMeta {
                addr,
                size: Size::Word,
                dir,
                burst: Burst::Single,
                prot: Default::default(),
            }),
            lock: LOW,
            ready: HIGH,
            #[cfg(feature = "cycle-debug-logger")]
            tag: CdlTag::from(">Test<"),
        }
    }

    /// Shortcut to create nonsequential single word-sized, idle transfer in tests.
    #[cfg(test)]
    #[must_use]
    pub fn idle() -> Self {
        Self {
            meta: TransferType::Idle,
            lock: LOW,
            ready: HIGH,
            #[cfg(feature = "cycle-debug-logger")]
            tag: CdlTag::from(">Test<"),
        }
    }

    /// Make a copy with an overridden `HREADY` wire.
    #[must_use]
    pub fn with_hready(self, hready: BinaryWire) -> Self {
        Self {
            ready: hready,
            ..self
        }
    }
}

/// Use as a generic argument to `P: AHBPortConfig` when you don't have anything better.
///
/// This is an uninhabited type.
#[cfg_attr(not(test), allow(dead_code))]
#[derive(Clone, Copy, Debug)]
pub(crate) enum UnknownPort {}
impl Default for UnknownPort {
    fn default() -> Self {
        unreachable!("UnknownPort cannot be constructed!")
    }
}
impl AHBPortConfig for UnknownPort {
    type Data = Self;
    type Component = Self;
    #[cfg(feature = "cycle-debug-logger")]
    const TAG: &'static str = CdlTag::DEFAULT_STR;
    #[cfg(not(feature = "cycle-debug-logger"))]
    const TAG: &'static str = "?????";
}
