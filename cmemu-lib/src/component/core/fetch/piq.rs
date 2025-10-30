use crate::common::Word;
#[cfg(feature = "cycle-debug-logger")]
use crate::confeature::cdl as cdl_conf;
#[cfg(feature = "cycle-debug-logger")]
use crate::engine::Context;
#[cfg(feature = "cycle-debug-logger")]
use crate::proxy::CycleDebugLoggerProxy;
use crate::utils::IfExpr;
#[cfg(feature = "cycle-debug-logger")]
use crate::utils::dife;
use cmemu_common::Address;
use heapless::Deque as ArrayDeque;
use log::trace;
use std::fmt;
use strum::IntoStaticStr;

/// [ARM-TRM-G] 1.2.1
/// Prefetch cache size is 3 words. Each entry is halfword-size.
const PREFETCH_CACHE_ENTRIES_COUNT: usize = 6;
const PREFETCH_HEAD_ENTRIES_COUNT: usize = 2;
const PREFETCH_QUEUE_ENTRIES_COUNT: usize =
    PREFETCH_CACHE_ENTRIES_COUNT - PREFETCH_HEAD_ENTRIES_COUNT;

#[derive(Copy, Clone, Eq, PartialEq, Debug, IntoStaticStr)]
pub enum PIQShiftMode {
    /// Fill the F/D reg if it is not fully occupied.
    Populate,
    /// A decode-time branch is under a multi-cycle instruction.
    HoldBranch(/* not last? */ bool), // see point E. of ShadowBuffer
    /// A narrow instruction was consumed
    ShiftHalf,
    /// A wide (or narrow and folded) instruction was consumed
    ShiftFull,
    /// There was a branch going on, we should put the target into F/D register now
    CompleteBranch,
}

pub(crate) type FDReg = ArrayDeque<u16, PREFETCH_HEAD_ENTRIES_COUNT>;
type ShadowQ = ShadowBuffer<u16, PREFETCH_CACHE_ENTRIES_COUNT>;

/// Stores subsequent, prefetched words, which are consumed by Decode when needed.
/// Since each instruction is 16 or 32 bits, Fetch operates on halfwords (16 bits).
///
/// The current model has two dual views of the prefetch buffer: a Train view and a Shadow Ring view.
/// The Train operates on the following vision of the prefetch cache: (\[X\] represents a 16b latch)
/// ```text
///      Instruction bus "incoming"
///      ║
///      v "queue"  "ShiftFull"
///     [X|X] ═> [X|X] ═> │M╲
///                 ╚>┌┐  │U ╲  "head reg"
///     "ShiftHalf" ╔>┝┥═>│X  │═>[X|X] -> directly to Decode PLA
///                 ║ └┘  │E ╱   (+ phase 2 latch) ══> as instruction
///     "Hold"      ╠═══> │R╱    ║                     data bus
///                 ╚════════════╝
///  ```
/// Note that the **queue** may contain data of addresses not subsequent to **head** (in case of jumps).
/// This view explains all decisions of when to prefetch next instructions.
///
/// The Shadow Ring view is needed to explain the (phantom) IT curse. It has a fixed "start":
/// ```text
///     ╔> [ X | X | X | X | X | X ] ═╗
///     ╚═════════════════════════════╝
/// ```
/// These two views are consolidated when we consider the "16b latches" from the Train view to be
/// implemented as offsets in the Shadow Ring view.
///
/// This module operates in 3 logical stages:
/// 1. "`@pos_edge`" `tick_piq` do the requested shift + pull incoming data into the structure.
/// 2. Provide data (immutable) and record reservations.
/// 3. @late tock: remember incoming data to use on the edge `push_back_bytes`
#[derive(Clone, Debug)]
pub struct PrefetchInputQueue {
    /// Contains data-phase of the fetch transfers to be used on the phi1 edge.
    ///
    /// TODO: interoperate this with the registers engine
    incoming: Option<(Address, [u16; 2], bool)>,

    /// Stores prefetched data.
    queue: ArrayDeque<u16, PREFETCH_QUEUE_ENTRIES_COUNT>,
    // After jump, queue may have different addr than reg
    queue_head_addr: Address,

    // It seems that decode-time branch always occupy a full wide slot?
    // See: narrow decode time branch to unaligned
    // Test: .align 3; [1 or 3] * add.n; ldr cyc; x_cyc ?; b.n unaligned; ... .align 2; add.n; unaligned: ldr cyc
    // See cbz.tzst, piq_narrow_b_to_unaligned.tzst
    /// Stores address of first word in cache.
    head_addr: Address,
    head_reg: FDReg,

    /// Shadow queue stores all the original data, if the PIQ would be implemented with a ring buff,
    /// which keeps stale values present and makes them available.
    ///
    /// See comments to `Decode::is_instr_tainted` for more context,
    /// and comments to `ShadowBuffer` for the rules specific to the Shadow ring view.
    /// `peek_shadow_head` must be consistent with old `peek head`.
    shadow: ShadowQ,
    /// In the same cycle of a jumping "D" of a branch, we should allow existing "F:D" in the buffer
    allow_stale_data_this_cycle: bool,
    /// Data view of the buffer is 32-bit, but we need to remember if the jump target was unaligned.
    shadow_unaligned_head: bool,

    prepare_for_impact: bool,

    /// Number of reserved entries in the cache.
    /// Entries are reserved when transfer is requested on `IBus`
    /// so that when the data arrives, there will be place for it in the cache.
    in_flight: u8,
}

use crate::confeature::cm_hyp::{ablation, shadow_piq};

impl PrefetchInputQueue {
    pub(super) fn new() -> Self {
        Self::new_at(Address::from_const(0))
    }

    // This is basically #[cfg(test)], but is not due to DRY
    #[inline(always)]
    fn new_at(address: Address) -> Self {
        Self {
            incoming: None,
            queue: ArrayDeque::new(),
            head_addr: address,
            queue_head_addr: address,
            head_reg: ArrayDeque::new(),
            shadow: ShadowBuffer::new(),
            allow_stale_data_this_cycle: false,
            shadow_unaligned_head: false,
            prepare_for_impact: false,
            in_flight: 0,
        }
    }

    pub(super) fn speculated_would_fit(&self, speculated_shift: PIQShiftMode) -> bool {
        debug_assert!(self.incoming.is_none(), "wrong phase (TODO: add stm)");
        if *ablation::NO_PIQ_OVERCOMMIT {
            return false;
        }
        let mut q_slots: usize = self.total_slots_allocated();
        q_slots = match speculated_shift {
            PIQShiftMode::ShiftHalf => q_slots.saturating_sub((self.head_reg.len() == 1).ife(2, 1)),
            PIQShiftMode::ShiftFull | PIQShiftMode::CompleteBranch => q_slots.saturating_sub(2),
            PIQShiftMode::Populate => q_slots.saturating_sub(2 - self.head_reg.len()),
            PIQShiftMode::HoldBranch(..) => q_slots,
        };
        q_slots <= PREFETCH_QUEUE_ENTRIES_COUNT
    }

    #[inline(always)]
    pub(super) fn is_overcommit(&self) -> bool {
        self.total_slots_allocated()
            + self
                .incoming
                .as_ref()
                .map_or(0, |(_, _, s)| 2 - usize::from(*s))
            > PREFETCH_CACHE_ENTRIES_COUNT
    }

    pub(super) fn tick_piq(&mut self, shift: PIQShiftMode) {
        // Process combinatorial data shift managed by PIQ.
        // That is: move data from the "queue" to the "head", according to decided shift op.
        // [queue; incoming] is considered to be a single "queue", here we
        trace!("PIQ Shift {:?}", shift);
        if self.prepare_for_impact {
            debug_assert!(shift == PIQShiftMode::Populate);
            return;
        }
        if self.shadow.trick {
            match shift {
                PIQShiftMode::HoldBranch(true) => {
                    self.shadow.hold_branch();
                }
                _ => {
                    self.shadow.reset();
                }
            }
        }
        // 1. Empty "head".
        match shift {
            PIQShiftMode::ShiftHalf => {
                debug_assert!(!self.head_reg.is_empty());
                self.shadow.pop();
                self.head_reg.pop_front();
                self.head_addr = self.head_addr.offset(2);
            }
            PIQShiftMode::ShiftFull => {
                debug_assert!(self.head_reg.len() == 2);
                self.shadow.pop();
                self.shadow.pop();
                self.head_reg.clear();
                self.head_addr = self.queue_head_addr;
            }
            PIQShiftMode::CompleteBranch => {
                self.shadow.complete_branch();
                self.head_reg.clear();
                self.head_addr = self.queue_head_addr;
            }
            PIQShiftMode::Populate if self.head_reg.is_empty() => {
                self.head_addr = self.queue_head_addr;
            }
            PIQShiftMode::HoldBranch(..) | PIQShiftMode::Populate => (),
        }
        // 2. Move data from "queue" to "head" to make space for incoming data.
        if !matches!(shift, PIQShiftMode::HoldBranch(..)) {
            self.fill_head_from_queue();
        }
        // 3.
        self.pull_incoming_into_queue();
        // 4. Pull more from the queue if we get new stuff.
        if !matches!(shift, PIQShiftMode::HoldBranch(..)) {
            self.fill_head_from_queue();
        }
        self.incoming = None;
        self.allow_stale_data_this_cycle = false;
        if !matches!(shift, PIQShiftMode::HoldBranch(..)) && self.shadow_unaligned_head {
            // We may need to virtually jump 3 slots
            self.shadow_unaligned_head = false;
            self.shadow.pop();
        }
    }

    #[inline(always)]
    fn fill_head_from_queue(&mut self) {
        debug_assert_eq!(
            self.head_addr
                .offset(u32::try_from(self.head_reg.len()).unwrap() * 2),
            self.queue_head_addr,
            "Invalid PIQ shifting with incompatible addresses."
        );
        while self.head_reg.len() < self.head_reg.capacity() && !self.queue.is_empty() {
            self.head_reg
                .push_back(self.queue.pop_front().unwrap())
                .unwrap();
            self.queue_head_addr = self.queue_head_addr.offset(2);
        }
    }

    #[inline(always)]
    fn pull_incoming_into_queue(&mut self) {
        debug_assert!(
            self.incoming.is_none() || self.queue.capacity() - self.queue.len() >= 2,
            "No place in PIQ for received data"
        );
        #[cfg_attr(not(debug_assertions), allow(unused_variables))]
        if let Some((address, data, skip_half)) = self.incoming.take() {
            #[cfg(debug_assertions)]
            {
                #[allow(clippy::cast_possible_truncation)]
                let expected_address = self.queue_head_addr.offset(self.queue.len() as u32 * 2);
                assert_eq!(
                    address, expected_address,
                    "Wrong instruction address in the input buffer"
                );
            }

            // Note: handling of branches and unaligned destination results (*phantom_it*.asm tests)
            //  - if the "skipped half" is in the buffer, it can trigger curse
            //  - trash after "b.n; trash" can trigger the curse
            //  - if already prefetched -> yes (maybe speculative -> no) data is present
            // See comments to ShadowBuffer for details.
            self.shadow.push(data[0]);
            self.shadow.push(data[1]);

            if skip_half {
                self.shadow_unaligned_head = true;
            }
            self.queue.extend(&data[skip_half.ife(1, 0)..]);
            debug_assert!(self.queue.len() <= PREFETCH_QUEUE_ENTRIES_COUNT);
        }
    }

    #[cfg(feature = "cycle-debug-logger")]
    #[allow(clippy::cast_possible_truncation, clippy::cast_lossless)]
    pub(super) fn log_in_cdl(&self, ctx: &mut Context, name: &'static str) {
        let data: u64 = u64::from_le_bytes([
            self.head_reg.len() as u8,
            self.queue.len() as u8,
            self.incoming.is_some() as u8,
            self.in_flight,
            self.total_slots_allocated() as u8,
            self.is_overcommit() as u8,
            0,
            0,
        ]);
        CycleDebugLoggerProxy.on_free_formatted_u64(ctx, name, data, |data: u64| {
            let [reg_len, q_len, incoming, in_flight, total, ov, _, _] = data.to_le_bytes();
            let in_flight = in_flight * 2;
            format!(
                "{reg_len}R + {q_len}Q {} + {in_flight}F = {total}{}",
                dife(incoming > 0, "+ <2-", ""),
                dife(ov > 0, "O", ""),
            )
        });

        if *cdl_conf::LOG_SHADOW_PIQ {
            // This is the dumbest way how to conceive a &'static str related to a &'static str
            self.shadow.log_in_cdl(ctx, &name[1..]);
        }
    }

    #[inline(always)]
    pub(super) fn total_slots_allocated(&self) -> usize {
        2 * (self.head_reg.is_empty().ife(0, 1) + self.in_flight as usize) + self.queue.len()
    }

    #[inline(always)]
    pub(super) fn peek_head(&self) -> FDReg {
        #[cfg(debug_assertions)]
        self.assert_shadow_head_is_consistent();
        self.head_reg.clone()
    }

    pub(super) fn peek_shadow_head(&self) -> FDReg {
        if !*shadow_piq::ENABLED {
            let mut x = self.head_reg.clone();
            let _ = x.push_back(0);
            x
        } else {
            let (a, b) = self.shadow.peek2();
            let mut buf = FDReg::new();
            buf.extend([a, b]);
            buf
        }
    }

    #[cfg(debug_assertions)]
    fn assert_shadow_head_is_consistent(&self) {
        if !*shadow_piq::ENABLED {
            return;
        }
        let (a, b) = self.shadow.peek2();

        assert!(
            self.head_reg.front().is_none_or(|&f| f == a)
                && self.head_reg.get(1).is_none_or(|&s| s == b),
            "Shadow PIQ ring is inconsistent with the Train PIQ view"
        );
    }

    #[inline(always)]
    pub(super) fn reserve(&mut self) {
        self.in_flight += 1;
        // account for overflow
        // can break when both spec fetch + spec branch
        // debug_assert!(self.total_slots_allocated() <= PREFETCH_CACHE_ENTRIES_COUNT + 2);
        debug_assert!(self.total_slots_allocated() <= PREFETCH_CACHE_ENTRIES_COUNT + 4);
    }

    #[inline(always)]
    pub(super) fn abort(&mut self) {
        // TODO: check if we need to allow late here
        debug_assert!(self.in_flight > 0);
        self.in_flight -= 1;
    }

    #[inline]
    pub(super) fn push_back_bytes(&mut self, data: Word, skip_half: bool, address: Address) {
        debug_assert!(!self.prepare_for_impact);
        debug_assert!(self.in_flight > 0);
        debug_assert!(self.incoming.is_none());

        self.in_flight -= 1;

        let address = if skip_half {
            address.offset(2)
        } else {
            address
        };
        let data = data.to_le_bytes();
        let first = u16::from_le_bytes([data[0], data[1]]);
        let second = u16::from_le_bytes([data[2], data[3]]);

        self.incoming = Some((address, [first, second], skip_half));

        // NOTE: this was over-conservative (we check it correctly in Tick anyway)
        // debug_assert!(self.total_slots_allocated() <= PREFETCH_CACHE_ENTRIES_COUNT);
    }

    // See point D. of `ShadowBuffer`
    pub(super) fn ignored_data(&mut self, data: Word, _address: Address) {
        if self.allow_stale_data_this_cycle {
            let data = data.to_le_bytes();
            let first = u16::from_le_bytes([data[0], data[1]]);
            let second = u16::from_le_bytes([data[2], data[3]]);
            self.shadow.push(first);
            self.shadow.push(second);
        }
    }

    #[inline(always)]
    pub(super) fn get_head_address(&self) -> Address {
        self.head_addr
    }

    /// Flush the "queue" part, while retaining the Fetch/Decode register.
    #[inline(always)]
    pub(super) fn branch(&mut self, new_head_addr: Address) {
        if *shadow_piq::ENABLED {
            // We need to allow FD from this cycle (it's prior to the branch),
            // but anything next may be either from before the branch (ignored)
            // or after the branch (to the new tail)
            if !self.prepare_for_impact {
                self.shadow.reset_trick(); // This is only a part...
                self.allow_stale_data_this_cycle = true;
            } else {
                self.allow_stale_data_this_cycle = false;
            }
        }
        self.queue.clear();
        self.queue_head_addr = new_head_addr;
        self.in_flight = 0;
        self.prepare_for_impact = false;
    }

    #[inline(always)]
    pub(super) fn confirm_speculated_branch(&mut self, new_head_addr: Address, held: bool) {
        // We may receive the correct data this cycle.
        self.branch(new_head_addr);
        if held {
            self.shadow.hold_branch();
        }
        self.allow_stale_data_this_cycle = false;
        // self.reserve(); // reserve back in flight
    }

    /// Clear all managed data, including the F/D register.
    #[inline(always)]
    pub(super) fn prepare_for_impact(&mut self, new_head_addr: Address) {
        self.branch(new_head_addr);
        self.prepare_for_impact = true;
        self.head_reg.clear();
        self.head_addr = new_head_addr;
    }

    #[allow(dead_code)]
    pub(super) fn hard_reset(&mut self, new_head_addr: Address) {
        self.queue.clear();
        self.queue_head_addr = new_head_addr;
        self.head_reg.clear();
        self.head_addr = new_head_addr;
        self.in_flight = 0;
        self.shadow.reset();
        self.shadow_unaligned_head = false;
        self.allow_stale_data_this_cycle = false;
        self.prepare_for_impact = false;
    }

    #[inline(always)]
    pub(super) fn available_slots_for_new_entries(&self) -> u8 {
        PREFETCH_CACHE_ENTRIES_COUNT
            .saturating_sub(self.total_slots_allocated())
            .try_into()
            .expect("available slots should fit into u8, see: `PREFETCH_CACHE_ENTRIES_COUNT`")
    }
}

impl fmt::Display for PrefetchInputQueue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use itertools::Itertools;

        write!(f, "{:?} [", self.head_addr)?;
        let mut comma = false;
        for e in &self.head_reg {
            if comma {
                write!(f, ", ")?;
            }
            comma = true;
            write!(f, "{e:x?}")?;
        }
        #[allow(clippy::cast_possible_truncation)]
        if self.head_addr.offset(2 * self.head_reg.len() as u32) == self.queue_head_addr {
            write!(f, "]::[")?;
        } else {
            write!(f, "] / {:?} [", self.queue_head_addr)?;
        }
        let mut comma = false;
        for e in &self.queue {
            if comma {
                write!(f, ", ")?;
            }
            comma = true;
            write!(f, "{e:x?}")?;
        }
        write!(f, "] <- [")?;
        let mut comma = false;
        if let Some((_addr, data, s)) = &self.incoming {
            for e in data.iter().skip(usize::from(*s)) {
                if comma {
                    write!(f, ", ")?;
                }
                comma = true;
                write!(f, "{e:x?}")?;
            }
        }
        for _ in 0..2 * self.in_flight {
            if comma {
                write!(f, ", ")?;
            }
            comma = true;
            write!(f, "_")?;
        }
        write!(
            f,
            "] : {}/{}",
            self.total_slots_allocated(),
            self.available_slots_for_new_entries()
        )?;

        write!(
            f,
            " S[{:x}]@h{}t{}",
            self.shadow.iter().format(", "),
            self.shadow.head,
            self.shadow.tail,
        )
    }
}

/// Shadow PIQ ring buffer implementation (with logically `MaybeUninit` parts)
///
/// During normal use, this buffer behaves like a ring buffer with N (=6) slots.
/// Empty slots are logically `MaybeUninit` from the CPU perspective, but not the emulator.
/// An "IT curse" is when the buffer looks like:
/// ```text
///          0   1   2 H 3 T  4   5        H - head
///     ╔> [ X | X | X | N | it | ? ] ═╗   T - tail
///     ╚══════════════════════════════╝
/// ```
///
/// Branches require extra logic: after a branch, the buffer "rewinds" to the 0-th position.
/// But, after a branch, an existing prefetch in data-phase may still write to the buffer.
/// ```text
///     for: B.w 1f; C.w; D.w; -- 1: X.w
///        HT  T' F:D  H'                 H', T' - transitional values
///     ╔> [ C | D | D | B | B | C ] ═╗   C is prefetched
///     ╚═════════════════════════════╝   F:D - cancelled transfer
///     will, at some point transition to:
///        H       T
///     ╔> [ X | X | D | B | B | C ] ═╗
///     ╚═════════════════════════════╝
/// ```
///
/// Moreover, a decode-time branch under a multicycle instruction should allow fetching 2 words.
/// These would override the branch in the buffer, but there is an observable effect
/// of preventing IT curse at 4th position (only in such a case!)
/// Therefore, we model it as moving the branch bytes there.
///
/// ```text
///     for: It; Div; B.w 1f; -- 1: X.w; Y.w
///        HT  H'      T'
///     ╔> [ D | B | B | ? | I | D ] ═╗
///     ╚═════════════════════════════╝
///     while stalled, at some point transition to:
///        H       T       H'
///     ╔> [ X | X | B | ? | B | B ] ═╗
///     ╚═════════════════════════════╝
///     then:
///        H              TH'
///     ╔> [ X | X | Y | Y | B | B ] ═╗
///     ╚═════════════════════════════╝
/// ```
///
/// Overall, only aligned halfwords of this buffer is observable (by phantom IT curse).
///
/// Note: this structure doesn't check buffer capacity invariants (so no issues at H=T).
// Rationale: (based on *phantom_it*.asm and others)
// 1. In normal case, we needed a "last-in-buffer" unaligned narrow-skipped instruction,
//    and IT was exactly 5 halfwords (not instructions!) earlier in the instruction stream.
//    We called it "IT curse".
//   a) There could've been multiple such pairs at the same time, e.g.,
//          .align 2; it.n; a.n; it.n; c.n; str.n; nop.n; str.n nop.n;
//   b) Later, we've noticed (by studying branch offsets) that bytes -- not instruction -- matters.
//      This was called "phantom IT".
//      There are not that many valid instructions with hidden IT in the second halfword:
//      - B.W with a large positive offset, but also a small negative. "5 halfwords" doesn't work.
//      - STR/LDR r11 with specific offsets (not detected by Fred, because it reserved r11...)
//      - many unpredictable wide instructions, where 15 encodes a destination (so pc, or cp15)
//      Three curses at a time become possible:
//          .align 2; it.n; a.n; it.n; str.w; nop.n; str.n nop.n; str.n; nop.n
//          in final buffer model: [ I | a | I | S |phI| N ]
// 2. The foldable IT, IT curse and phantom IT affected pipelining of nop and skipped instructions.
//    While the original rules were convoluted, register-offset str + nop became a clear detector,
//    as the nop always pipelined except for any of the IT cases. (Folding was even too strong.)
// 3. Thus, by placing "str.n/w ra, [rb, rc]; nop; and manipulating/modeling fetches we may detect
//    the curse caused by decoding an IT. It requires the word after NOP to be not fetched yet.
//    This is the modus operandi of `definitive_phantom_it_curse.asm`.
// 4. The three cases: normal IT, IT curse, and phantom IT could be explained by always doing
//    a part of the "normal folding logic", while disregarding the validity of F/D reg.
//    This is where the use-after-free ring buffer idea comes from.
//    Further tests were consistent with this idea, albeit needing some adjustments.
//
// The rules of the buffer come from `definitive_phantom_it_curse.asm`.
// You can disable the shadow buffer here, run the test, and then use `explain.py`/dtale
// to see which configurations result in atypical counter values.
// A. When jumping to unaligned addresses, the aligned part is in the buffer too.
//    Proof: IT prior to the label will trigger point 1. scenario.
// B. After a branch, the buffer is filled from the "0th" position.
//    Part 1 of the argument: for a code like `pad1; isb.w; pad2; b; ...; label: pad3; nop;`
//    the position of IT which triggers the curse doesn't depend on `pad1` (apart from alignment),
//    but only on the *difference* in size between `pad2` and `pad3` (with tweaks for alignment).
//    This comes from feature engineering done in mm319369/test_debug/expl_feat.py:cursed_it
//      which is enough to cover and replace all other placement rules.
//    In particular, this rule implies that some values of pad2 cannot inflict the curse
//    with branches (namely IT at 0th position).
// C. Mispredicted branches (and double fetches) have no effect on the buffer.
//    (See the "mispredicted" scenario, some offsets even trigger double-fetching,
//     like the one with stall_pos=0; stall_n=6; no pad1/pad2; it_pos=0, paddington=0).
// D. Prefetched data beyond a branch. First of all, all finished transfers are in the buffer.
//    Looking at CDLs, we see that transfers not-in-data-phase at the point of the branch
//    are ignored (what is consistent with point C.) For other cases, we may observe a clear
//    cut-point of stall_n which allows an update to the buffer (that is, the position of curse changes)
//
//    The CDL may look like this:
//    it?   D  X   <- position of the curse if stall_n is less than below
//    #N             D  X  X  X  X
//    ^^    FD FD FD ^  ^  ^  ^  ^
//    mov   FD ^^ ^^    D           X
//    b.n   fa fa fa FD FD FD       D  X  <- the branch position depends on stall_n
//    n/a   fa ^^ ^^ FD ^^ ^^       |--- this is the important moment
//    it?            fa fa fa FD FD FD  <- alternative position of the curse if stall_n >= 4
//    n/a            fa ^^ ^^ FD ^^ ^^.
//
//    That is, a transfer may be placed as a "finished prefetch" even if it finishes in the same
//    cycle as D. If there were more data-phase cycles after D, the data would be ignored.
//    Moreover, it's the D cycle (not X), that is important as seen with stalled branches.
// E. Stalled branches (under N-cycles gadget) pose a threat of validity to point B., as the Train
//    view dictates that 2 extra words may be loaded while the branch is stalled (indicating that
//    the branch occupies the third word). Combining that with point B. means slots 0-3 would be
//    overridden, so the branch must be held in slots 4-5 (or somewhere else), but according to
//    other rules, it could be in any other slot.
//
//    The CDL below has a curious effect if the IT curse is at the 4th slot,
//    but no effect at 2nd (and 0th is impossible from B.).
//
//    it    D  X   <- trigger curse only if stall_n is less than below
//    mov      D  X           /- D is not stalled yet if this is the last cycle
//    #N    FD FD FD D  X  X  X  X
//    ^^    FD ^^ ^^ ^  ^  ^  ^  ^
//    b.n   fa fa fa FD FD FD D     X  <- only the X position depends on stall_n
//    n/a   ^^ ^^ ^^ ^^ ^^ ^^             the branch is considered stalled if X is not next to D
//
//    With such a layout, the curse at 4th position will occur only when the branch is not stalled.
//    It is also possible to generate a case (combining point D.) where the IT curse is after
//    the branch: again, the behavior changing depending on if it uses the 4th or 2nd slot.
//
//    If we consider that the branch "escapes" from being overridden by conditionally "moving"
//    to slots 4-5, this would explain the aforementioned 4-vs-2 difference and resolve the issue
//    to point B.
//
//    TODO: the order of computation in CMemu makes some logic more complex than it needs to be.
//          Consider using flops for PIQ.
// TODO: note about execute-time branches

#[derive(Clone, Debug)]
struct ShadowBuffer<T, const N: usize>
where
    T: Default + Copy,
{
    arr: [T; N],
    tail: usize,
    head: usize,
    // a potentially-stalled decode-time branch occurs
    trick: bool,
}

impl<T, const N: usize> ShadowBuffer<T, N>
where
    T: Default + Copy,
{
    fn new() -> Self {
        Self {
            arr: [T::default(); N],
            tail: 0,
            head: 0,
            trick: false,
        }
    }

    fn reset(&mut self) {
        // See point B. above
        self.tail = 0;
        self.head = 0;
        self.trick = false;
    }

    fn reset_head(&mut self) {
        self.head = 0;
    }

    // See point E. above. Also, we don't reset the tail right away as we need data from point D.
    fn reset_trick(&mut self) {
        self.trick = true;
    }

    fn complete_branch(&mut self) {
        self.reset_head();
        // TODO: we handle self.trick in PIQ::shift_piq right now: move it here.
        // self.trick = false;
    }

    // Second part of point E.
    fn hold_branch(&mut self) {
        if self.trick {
            // Point E.
            // Note: the second part may be some trash overridden by a new fetch,
            // but we cannot determine what's in the unaligned part of the buffer.
            let (a, b) = self.peek2();
            // TODO: verify what happens when B is stalled under unaligned STR.w r11
            //       (that is, does the aligned phantom IT or the branch is copied to N-2)
            self.arr[N - 2] = a;
            self.arr[N - 1] = b;
            self.head = N - 2;
            // Point B.
            self.tail = 0;
            self.trick = false;
        }
    }

    fn inc_tail(&mut self) {
        self.tail = Self::next_mod(self.tail);
    }
    fn inc_head(&mut self) {
        self.head = Self::next_mod(self.head);
    }
    fn next_mod(mut a: usize) -> usize {
        a += 1;
        if a >= N {
            a = 0;
        }
        a
    }
    fn push(&mut self, val: T) {
        self.arr[self.tail] = val;
        self.inc_tail();
    }
    fn pop(&mut self) {
        self.inc_head();
    }
    fn peek2(&self) -> (T, T) {
        (self.arr[self.head], self.arr[Self::next_mod(self.head)])
    }
    fn iter(&self) -> impl Iterator<Item = T> + '_ {
        self.arr.iter().copied()
    }
}

impl ShadowBuffer<u16, 6> {
    #[cfg(feature = "cycle-debug-logger")]
    #[allow(clippy::pedantic)]
    fn log_in_cdl(&self, ctx: &mut Context, name: &'static str) {
        // NOP is like IT but with 0 at the last byte
        let mut iter = self.iter().map(|h| if h >> 8 == 0b1011_1111 { h & 0xf | 0x10 } else { 0 } as u8).chain(
            [self.head as u8, self.tail as u8]
        );
        let data: u64 = u64::from_le_bytes(std::array::from_fn(|_| iter.next().unwrap()));
        CycleDebugLoggerProxy.on_free_formatted_u64(ctx, name, data, |buf| {
            let [a, b, c, d, e, f, head, tail] = buf.to_le_bytes();
            format!("S[{a:x},{b:x},{c:x},{d:x},{e:x},{f:x}] H: {head}, T: {tail}")
        });
    }
}

#[cfg(test)]
mod test {
    use crate::component::core::fetch::piq::{PIQShiftMode, PrefetchInputQueue};
    use crate::confeature::cm_hyp::shadow_piq;
    use crate::utils::IfExpr;
    use heapless::Deque;
    use rstest::*;

    #[rstest]
    #[test_log::test]
    fn dummy() {
        let piq = PrefetchInputQueue::new();
        println!("Empty: {piq}");
    }

    // missing impls there
    fn into_deque<const N: usize>(i: impl IntoIterator<Item = u16>) -> Deque<u16, N> {
        let mut deque = Deque::<u16, N>::new();
        deque.extend(i);
        deque
    }

    #[rstest]
    #[test_log::test]
    fn normal() {
        let mut piq = PrefetchInputQueue::new_at(0x888.into());

        println!("Empty: {piq}");

        assert!(piq.peek_head().is_empty());

        // Branch to
        piq.branch(0x42.into());
        assert!(piq.peek_head().is_empty());

        // it is still no here
        assert_eq!(piq.get_head_address(), 0x888.into());
        piq.reserve();
        piq.push_back_bytes(0x1337_0000.into(), true, 0x40.into());

        println!("Pre-edge: {piq}",);

        // it is still no here
        assert_eq!(piq.get_head_address(), 0x888.into());

        // not it is!
        piq.tick_piq(PIQShiftMode::CompleteBranch);
        assert_eq!(piq.get_head_address(), 0x42.into());
        assert_eq!(piq.total_slots_allocated(), 2);

        println!("Post-edge: {piq}",);

        piq.reserve();
        piq.reserve();
        piq.push_back_bytes(0x1111_0000.into(), false, 0x44.into());

        println!("new data {piq}");
        piq.tick_piq(PIQShiftMode::Populate);
        println!("populated {piq}");
        assert_eq!(piq.get_head_address(), 0x42.into());
        assert_eq!(piq.total_slots_allocated(), 5);
        assert_eq!(piq.peek_head(), into_deque([0x1337, 0x0000]));

        piq.push_back_bytes(0x3333_2222.into(), false, 0x48.into());
        println!("new data {piq}");
        piq.tick_piq(PIQShiftMode::ShiftFull);
        println!("fully shifted {piq}");

        assert_eq!(piq.get_head_address(), 0x46.into());
        assert_eq!(piq.peek_head(), into_deque([0x1111, 0x2222]));

        piq.tick_piq(PIQShiftMode::ShiftFull);
        println!("fully shifted {piq}");

        // Branching, half in buffer
        piq.reserve();
        piq.tick_piq(PIQShiftMode::HoldBranch(true));
        assert_eq!(piq.total_slots_allocated(), 4);

        piq.branch(0x10.into());
        println!("branched {piq}");
        assert_eq!(piq.get_head_address(), 0x4a.into());
        assert_eq!(piq.peek_head(), into_deque([0x3333]));

        piq.reserve();
        piq.tick_piq(PIQShiftMode::HoldBranch(true));
        piq.push_back_bytes(0x5555_4444.into(), false, 0x10.into());
        piq.tick_piq(PIQShiftMode::HoldBranch(false));
        println!("received {piq}");
        assert_eq!(piq.total_slots_allocated(), 4);

        // execute the branch
        piq.tick_piq(PIQShiftMode::CompleteBranch);
        println!("branch performed {piq}");
        assert_eq!(piq.get_head_address(), 0x10.into());
        assert_eq!(piq.peek_head(), into_deque([0x4444, 0x5555]));
        assert_eq!(piq.total_slots_allocated(), 2);
    }

    #[rstest]
    #[test_log::test]
    #[should_panic(expected = "in_flight")]
    #[case::no_reservation(false, 0x0, true, 0x0)]
    #[should_panic(expected = "address")]
    #[case::invalid_addr(true, 0x0, false, 0x0)]
    #[should_panic(expected = "address")]
    #[case::invalid_addr_unaligned(true, 0x0, true, 0xa0)]
    #[case::ok(true, 0x42, false, 0xa0)]
    fn invalid_push(
        #[case] reserves: bool,
        #[case] data: u32,
        #[case] skip_half: bool,
        #[case] address: u32,
    ) {
        let mut piq = PrefetchInputQueue::new_at(0xa0.into());

        if reserves {
            piq.reserve();
        }
        piq.push_back_bytes(data.into(), skip_half, address.into());
        piq.tick_piq(PIQShiftMode::Populate);
    }

    #[rstest]
    #[test_log::test]
    fn overcommit_buffer() {
        let mut piq = PrefetchInputQueue::new_at(0xa0.into());

        piq.reserve();
        piq.reserve();
        piq.push_back_bytes(0.into(), false, 0xa0.into());
        piq.tick_piq(PIQShiftMode::Populate);
        piq.push_back_bytes(0.into(), false, 0xa4.into());
        piq.tick_piq(PIQShiftMode::Populate);
        assert!(piq.speculated_would_fit(PIQShiftMode::ShiftFull));
        assert!(piq.speculated_would_fit(PIQShiftMode::ShiftHalf));
        assert!(piq.speculated_would_fit(PIQShiftMode::Populate));
        assert!(piq.speculated_would_fit(PIQShiftMode::HoldBranch(false)));
        piq.reserve();

        assert!(piq.speculated_would_fit(PIQShiftMode::ShiftFull));
        assert!(!piq.speculated_would_fit(PIQShiftMode::ShiftHalf));
        assert!(!piq.speculated_would_fit(PIQShiftMode::HoldBranch(false)));

        piq.push_back_bytes(0.into(), false, 0xa8.into());
        piq.tick_piq(PIQShiftMode::Populate);
        println!("full: {piq}");
        assert_eq!(piq.total_slots_allocated(), 6);
        assert_eq!(piq.head_addr, 0xa0.into());
        assert_eq!(piq.queue_head_addr, 0xa4.into());

        assert!(piq.speculated_would_fit(PIQShiftMode::ShiftFull));
        assert!(!piq.speculated_would_fit(PIQShiftMode::ShiftHalf));
        assert!(!piq.speculated_would_fit(PIQShiftMode::Populate));
        assert!(!piq.speculated_would_fit(PIQShiftMode::HoldBranch(false)));
        piq.reserve();
        println!("overflow: {piq}");

        // Test hypothetical late overflow -> the incoming data fit "just in time"
        // piq.tick_piq(PIQShiftMode::ShiftFull);
        piq.tick_piq(PIQShiftMode::Populate);
        // TODO: do we really need to support this? Or maybe it won't ever be like that,
        //       and on the edge when speculative data is sampled, the queue already has free space?
        assert!(piq.is_overcommit());
        piq.push_back_bytes(1.into(), false, 0xac.into());
        println!("late overflow: {piq}");
        assert!(piq.is_overcommit());
        piq.tick_piq(PIQShiftMode::ShiftFull);
        println!("final: {piq}");
        assert!(!piq.is_overcommit());
    }

    fn fill_piq(
        piq: &mut PrefetchInputQueue,
        vals: impl IntoIterator<Item = u32>,
        mut skip_half: bool,
        mut address: u32,
    ) {
        for val in vals {
            piq.reserve();
            piq.push_back_bytes(val.into(), skip_half, address.into());
            piq.tick_piq(
                piq.is_overcommit()
                    .ife(PIQShiftMode::ShiftFull, PIQShiftMode::Populate),
            );
            skip_half = false;
            address += 4;
        }
    }

    #[rstest]
    #[test_log::test]
    fn shadow_simple() {
        if !*shadow_piq::ENABLED {
            return;
        }
        let mut piq = PrefetchInputQueue::new_at(0xa0.into());

        fill_piq(
            &mut piq,
            [0x2222_1111, 0x4444_3333, 0x6666_5555],
            false,
            0xa0,
        );

        assert_eq!(piq.peek_head(), into_deque([0x1111, 0x2222]));
        assert_eq!(piq.peek_shadow_head(), into_deque([0x1111, 0x2222]));
        piq.tick_piq(PIQShiftMode::ShiftHalf);
        assert_eq!(piq.peek_head(), into_deque([0x2222, 0x3333]));
        assert_eq!(piq.peek_shadow_head(), into_deque([0x2222, 0x3333]));
        piq.tick_piq(PIQShiftMode::ShiftFull);
        piq.tick_piq(PIQShiftMode::ShiftFull);
        println!("half left: {piq}");

        assert_eq!(piq.peek_head(), into_deque([0x6666,]));
        assert_eq!(piq.peek_shadow_head(), into_deque([0x6666, 0x1111]));
        piq.tick_piq(PIQShiftMode::ShiftHalf);
        assert_eq!(piq.peek_head(), into_deque([]));
        assert_eq!(piq.peek_shadow_head(), into_deque([0x1111, 0x2222]));

        piq.reserve();
        piq.push_back_bytes(0x8888_7777u32.into(), false, 0xac.into());
        // Not yet -> or does it really matter?
        assert_eq!(piq.peek_shadow_head(), into_deque([0x1111, 0x2222]));
        piq.tick_piq(PIQShiftMode::Populate);
        assert_eq!(piq.peek_head(), into_deque([0x7777, 0x8888]));
        assert_eq!(piq.peek_shadow_head(), into_deque([0x7777, 0x8888]));

        piq.tick_piq(PIQShiftMode::ShiftHalf);
        assert_eq!(piq.peek_head(), into_deque([0x8888]));
        assert_eq!(piq.peek_shadow_head(), into_deque([0x8888, 0x3333]));
        println!("final: {piq}");
    }

    #[rstest]
    #[test_log::test]
    fn shadow_simple_branch() {
        // Simple aligned destination, no outstanding transfers
        // Unaligned branch instruction
        if !*shadow_piq::ENABLED {
            return;
        }
        let mut piq = PrefetchInputQueue::new_at(0xa0.into());

        fill_piq(&mut piq, [0x2222_1111, 0x4444_3333], false, 0xa0);
        piq.tick_piq(PIQShiftMode::ShiftFull);
        piq.tick_piq(PIQShiftMode::ShiftHalf);

        piq.branch(0x40.into());
        assert_eq!(piq.peek_head(), into_deque([0x4444,]));
        assert_eq!(piq.peek_shadow_head(), into_deque([0x4444, 0 /* uninit */]));

        piq.reserve();
        piq.tick_piq(PIQShiftMode::CompleteBranch);
        assert_eq!(piq.peek_head(), into_deque([]));
        assert_eq!(piq.peek_shadow_head(), into_deque([0x1111, 0x2222]));

        piq.push_back_bytes(0x8888_7777u32.into(), false, 0x40.into());
        piq.tick_piq(PIQShiftMode::Populate);
        assert_eq!(piq.peek_head(), into_deque([0x7777, 0x8888]));
        assert_eq!(piq.peek_shadow_head(), into_deque([0x7777, 0x8888]));
        println!("half left: {piq}");

        piq.tick_piq(PIQShiftMode::ShiftHalf);
        assert_eq!(piq.peek_head(), into_deque([0x8888,]));
        assert_eq!(piq.peek_shadow_head(), into_deque([0x8888, 0x3333]));
        println!("final: {piq}");

        // Branch to unaligned this time
        piq.branch(0x2.into());
        piq.reserve();
        piq.tick_piq(PIQShiftMode::CompleteBranch);
        piq.push_back_bytes(0x1234_5678u32.into(), true, 0x0.into());
        piq.tick_piq(PIQShiftMode::Populate);
        assert_eq!(piq.peek_head(), into_deque([0x1234,]));
        assert_eq!(piq.peek_shadow_head(), into_deque([0x1234, 0x3333]));
    }

    #[rstest]
    #[test_log::test]
    fn shadow_outstanding_branch() {
        // Outstanding transfers, but branch is not held under a multicycle.
        // Unaligned branch instruction
        if !*shadow_piq::ENABLED {
            return;
        }
        let mut piq = PrefetchInputQueue::new_at(0xa2.into());

        fill_piq(
            &mut piq,
            [0x0002_0001, 0x0004_0003, 0x0006_0005, 0x0008_0007],
            true,
            0xa0,
        );
        println!("start: {piq}");
        piq.reserve(); // T1:
        piq.tick_piq(PIQShiftMode::ShiftFull);

        assert!(piq.speculated_would_fit(PIQShiftMode::ShiftHalf));
        piq.reserve(); // T2: speculated (single cycle)

        piq.branch(0x42.into()); // decode time: positioned at 7

        assert_eq!(piq.peek_head(), into_deque([6, 7]));
        assert_eq!(piq.peek_shadow_head(), into_deque([6, 7]));
        piq.reserve(); // new fetch

        // T1 Finishes in the same cycle as the branch decodes (under single-cycle instr)
        piq.ignored_data(0x1234_5678.into(), 0xac.into());
        piq.tick_piq(PIQShiftMode::CompleteBranch);

        // T2 Finishes - ignored
        piq.ignored_data(0xaaaa_bbbbu32.into(), 0xb0.into());
        // Nothing to decode
        piq.tick_piq(PIQShiftMode::Populate);

        println!("after branch: {piq}");
        assert_eq!(piq.peek_head(), into_deque([]));
        assert_eq!(piq.peek_shadow_head(), into_deque([7, 8]));

        // New data
        piq.push_back_bytes(0x000b_000au32.into(), true, 0x40.into());
        piq.tick_piq(PIQShiftMode::Populate);
        piq.reserve();

        assert_eq!(piq.peek_head(), into_deque([0xb,]));
        assert_eq!(piq.peek_shadow_head(), into_deque([0xb, 0x5678]));
        println!("final: {piq}");

        piq.push_back_bytes(0x000d_000cu32.into(), false, 0x44.into());
        piq.tick_piq(PIQShiftMode::Populate);
        piq.tick_piq(PIQShiftMode::ShiftFull);

        assert_eq!(piq.peek_head(), into_deque([0x000d,]));
        assert_eq!(piq.peek_shadow_head(), into_deque([0x000d, 0x0005]));
    }

    #[rstest]
    #[test_log::test]
    #[case::branch_stalled(3)]
    #[case::branch_at_last(2)]
    #[case::branch_after_normal(1)]
    fn shadow_outstanding_multicycle_branch(#[case] steps_to_branch: u32) {
        // Outstanding transfers,  branch is held under a multicycle.
        if !*shadow_piq::ENABLED {
            return;
        }
        let mut piq = PrefetchInputQueue::new_at(0xa2.into());

        fill_piq(
            &mut piq,
            [0x0002_0001, 0x0004_0003, 0x0006_0005, 0x0008_0007],
            true,
            0xa0,
        );
        println!("start: {piq}");
        piq.reserve(); // T1:
        piq.tick_piq(PIQShiftMode::ShiftFull);

        // No place for speculated transfer
        piq.branch(0x42.into()); // decode time: positioned at 6

        assert_eq!(piq.peek_head(), into_deque([6, 7]));
        assert_eq!(piq.peek_shadow_head(), into_deque([6, 7]));
        piq.reserve(); // new fetch

        // T1 Finishes in the same cycle as the branch decodes
        piq.ignored_data(0x1234_5678.into(), 0xac.into());

        let modes_to_branch = [
            PIQShiftMode::CompleteBranch,
            PIQShiftMode::HoldBranch(false),
            PIQShiftMode::HoldBranch(true),
        ];
        let mut was_stalled = false;

        for step in modes_to_branch
            .into_iter()
            .take(steps_to_branch as usize)
            .rev()
        {
            was_stalled |= matches!(step, PIQShiftMode::HoldBranch(true));
            piq.tick_piq(step);
        }

        println!("after branch: {piq}");
        assert_eq!(piq.peek_head(), into_deque([]));
        assert_eq!(piq.peek_shadow_head(), into_deque([7, 8]));

        // New data
        piq.push_back_bytes(0x000b_000au32.into(), true, 0x40.into());
        piq.tick_piq(PIQShiftMode::Populate);
        piq.reserve();

        assert_eq!(piq.peek_head(), into_deque([0xb,]));
        assert_eq!(piq.peek_shadow_head(), into_deque([0xb, 0x5678]));
        println!("final: {piq}");

        piq.push_back_bytes(0x000d_000cu32.into(), false, 0x44.into());
        piq.tick_piq(PIQShiftMode::Populate);
        piq.tick_piq(PIQShiftMode::ShiftFull);

        assert_eq!(piq.peek_head(), into_deque([0x000d,]));
        if was_stalled {
            // Here we see the branch moved to the last slot!
            assert_eq!(piq.peek_shadow_head(), into_deque([0x000d, 0x0006]));
        } else {
            assert_eq!(piq.peek_shadow_head(), into_deque([0x000d, 0x0005]));
        }
    }
}
