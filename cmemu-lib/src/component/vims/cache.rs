use crate::common::Address;
use crate::common::new_ahb::databus::DataBus;
use crate::common::new_ahb::ports::AHBPortConfig;
use crate::common::new_ahb::ports::{
    AHBMasterPortInput, AHBMasterPortOutput, AHBSlavePortInput, AHBSlavePortOutput,
};
use crate::common::new_ahb::signals::HRESP::ERROR;
use crate::common::new_ahb::signals::{
    AhbResponseControl, MasterToSlaveAddrPhase, MasterToSlaveDataPhase, MasterToSlaveWires,
    SlaveToMasterWires,
};
use crate::common::new_ahb::state_track::AHBStateTrack;
use crate::component::vims::VIMSComponent;
use crate::component::vims::cache_ram::CacheRAMComponent;
use crate::component::vims::internal_routing::CodeCacheLineBuffer;
use crate::engine::Context;
use crate::engine::{
    CombFlop, DisableableComponent, SeqFlopMemoryBank, Subcomponent, TickComponent,
    TickComponentExtra,
};
use crate::make_port_struct;
#[cfg(debug_assertions)]
use crate::proxy::FlashProxy;
use cc2650_constants as soc;
use fixedbitset::FixedBitSet;
#[cfg_attr(not(debug_assertions), allow(unused_imports))]
use log::{debug, info, trace};
use owo_colors::OwoColorize;

use crate::confeature;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub(crate) enum Mode {
    GPRAM = 0,
    Cache = 1,
    Off = 3,
}

/// The sequence used for choosing which way to evict. Be advised that not all cache misses choose their way
/// using this sequence, and the rules according to which the position in the sequence is shifted are non-trivial.
/// Additionally, in some cases the value obtained from `POLY` is altered before it is used as a way number.
///
/// See cache thesis, Chapter 4
const POLY: [u8; 255] = [
    0, 0, 0, 0, 0, 0, 1, 2, 0, 0, 1, 3, 3, 2, 0, 0, 1, 2, 0, 1, 2, 1, 3, 3, 2, 0, 0, 0, 0, 0, 1, 3,
    2, 0, 1, 2, 0, 1, 2, 0, 1, 3, 2, 1, 3, 3, 2, 0, 1, 2, 0, 0, 0, 0, 1, 2, 1, 2, 1, 3, 2, 1, 3, 2,
    1, 2, 1, 3, 2, 0, 1, 2, 1, 3, 2, 0, 0, 0, 1, 3, 3, 3, 3, 2, 1, 3, 2, 1, 3, 3, 3, 2, 1, 2, 1, 3,
    3, 2, 1, 2, 0, 0, 1, 2, 0, 0, 0, 1, 3, 2, 1, 3, 2, 0, 0, 1, 3, 3, 3, 2, 0, 1, 3, 3, 2, 0, 1, 3,
    2, 0, 0, 1, 2, 1, 3, 2, 1, 2, 0, 1, 2, 0, 0, 1, 2, 1, 2, 0, 1, 2, 1, 2, 1, 2, 0, 1, 3, 3, 2, 1,
    3, 3, 2, 1, 3, 2, 0, 1, 3, 3, 3, 2, 1, 3, 3, 3, 3, 3, 2, 1, 2, 0, 1, 3, 2, 0, 1, 3, 2, 1, 2, 1,
    2, 0, 0, 1, 3, 2, 0, 0, 0, 0, 1, 3, 3, 2, 1, 2, 1, 2, 1, 2, 1, 3, 3, 3, 3, 2, 0, 1, 2, 1, 2, 0,
    0, 0, 1, 2, 0, 1, 3, 3, 3, 3, 3, 3, 3, 2, 0, 0, 0, 1, 2, 1, 3, 3, 3, 2, 0, 0, 1, 3, 2, 1, 2,
];

#[derive(Eq, PartialEq, Debug)]
enum PendingRequestType {
    None,
    Delayed,
    Flash,
    Cache,
}

#[derive(Eq, PartialEq, Debug)]
enum RequestThisTick {
    None,
    Delayed,
    Flash,
    Cache,
}

#[derive(Debug)]
struct PendingTagUpdate {
    addr: Address,
    data: [u8; 8],
    is_prefetched: bool,
    set_was_full_according_to_tag_ram_when_tag_ram_was_read: bool,
}

#[derive(Debug)]
struct PendingFlashRead {
    is_prefetched: bool,
    set_was_full_according_to_tag_ram_when_tag_ram_was_read: bool,
}

/// The form of TAG RAM entries reflects the description from a cache user's guide from TI:
/// <https://www.ti.com/lit/ug/spru656a/spru656a.pdf>.
#[derive(Default, Copy, Clone)]
struct TagRamEntry {
    valid: bool,
    tag: u32,
}

#[derive(Subcomponent, TickComponent, DisableableComponent)]
#[subcomponent_1to1]
pub(crate) struct CacheComponent {
    // Partly-manual flop that has a transition period
    mode: Mode,
    /// Next mode is Some only when an actual change is in progress!
    next_mode: Option<Mode>,

    /// The current position of cache in POLY.
    #[flop]
    poly_pos: CombFlop<u8>,

    /// Generally speaking, `next_set_pos` is used to keep track of the next way to fill if a set is not yet full.
    /// However, because of pipelining it is not trivial how many loads consider the set to not be full. Additionally,
    /// due to various edge cases these loads may use and modify `next_set_pos` in a non-trivial manner.
    #[flop]
    next_set_pos: SeqFlopMemoryBank<[u8; 256], u8>,

    /// A TAG RAM update is considered to treat its set as full if the following two conditions hold:
    /// - the set is already full according to tag ram during the update (way 3 in this set is used)
    /// - the update does not reuse the previous value of `next_set_pos[set]`
    #[flop]
    tag_update_treated_set_as_full: SeqFlopMemoryBank<FixedBitSet, u8>,

    /// The memory that contains metadata about which addresses are cached in which ways.
    #[flop]
    tag_ram: SeqFlopMemoryBank<[TagRamEntry; 1024], (u32, u32)>,

    /// Used for storing the request when a simulated wait state for TAG lookup is added.
    #[flop]
    delayed_msg: CombFlop<MasterToSlaveWires<<Self as AHBPortConfig>::Data>>,

    /// Used for remembering the address of currently handled request so that on reply it can be included in
    /// the metadata required for the TAG update. It is separate from `pending_flash_read` so that it can also be used
    /// for a debug assertion that checks if data read from cache match the data in flash.
    #[flop]
    pending_addr: CombFlop<Address>,

    /// Used for remembering the metadata required for the TAG update, since the update happens a few cycles after memory
    /// response.
    #[flop]
    pending_tag_update: CombFlop<PendingTagUpdate>,

    /// Used for remembering some metadata about the currently handled cache miss so that on a reply it can be included
    /// in the metadata required for the TAG update.
    #[flop]
    pending_flash_read: CombFlop<PendingFlashRead>,

    #[flop]
    most_recent_tag_update_reused_previous_next_set_pos: CombFlop<bool>,

    #[flop]
    pending_request_type: CombFlop<PendingRequestType>,

    /// [TI-TRM] 7.2.4
    ///
    /// Be advised that there are special cases which cause a load to be unprefetched regardless
    /// of the value of `tag_prefetch`
    #[flop]
    tag_prefetch: CombFlop<u32>,

    /// A TAG update series is a series of TAG updates whose corresponding loads happened in quick succession.
    #[flop]
    most_recent_tag_update_series_had_prefetch: CombFlop<bool>,

    #[flop]
    set_of_most_recent_tag_update: CombFlop<u32>,

    #[flop]
    cycles_since_last_flash_request: CombFlop<u64>,

    #[flop]
    cycles_since_last_flash_read: CombFlop<u64>,

    #[flop]
    cycles_since_last_gpram_read: CombFlop<u64>,

    #[flop]
    cycles_since_most_recent_tag_update: CombFlop<u64>,

    request_this_tick: RequestThisTick,

    // For inversing tock
    tocked_this_cycle: bool,
    flash_track: AHBStateTrack,
    ram_track: AHBStateTrack,
    upstream_track: AHBStateTrack,
}

impl TickComponentExtra for CacheComponent {
    #[cfg(debug_assertions)]
    fn tick_assertions(&self) {
        debug!(
            "{} mode={:?}->{:?} pos={:#?} prev_rtt={:#?} pref={:#?};",
            "CACHE SUMMARY".bright_cyan(),
            self.mode,
            self.next_mode,
            self.poly_pos,
            self.request_this_tick,
            self.tag_prefetch,
        );
        trace!(
            "{} p_type={:#?}, p_flash={:#?}, p_tag={:#?}, p_addr={:#?}, d_msg={:#?}",
            "CACHE PEND".bright_cyan(),
            self.pending_request_type,
            self.pending_flash_read,
            self.pending_tag_update,
            self.pending_addr,
            self.delayed_msg,
        );
        trace!(
            "{} tutsaf={:#?}, mrturpnsp={:#?} mrtushp={:#?} somrtu={:#?} cslfrq={:#?} cslfrd={:#?} cslgrd={:#?} csmrtu={:#?}",
            "CACHE COUNTS".bright_cyan(),
            self.tag_update_treated_set_as_full,
            self.most_recent_tag_update_reused_previous_next_set_pos,
            self.most_recent_tag_update_series_had_prefetch,
            self.set_of_most_recent_tag_update,
            self.cycles_since_last_flash_request,
            self.cycles_since_last_flash_read,
            self.cycles_since_last_gpram_read,
            self.cycles_since_most_recent_tag_update,
        );
        trace!(
            "{} up: {:?}, flash: {:?}, ram: {:?};",
            "CACHE TRACK".bright_cyan(),
            self.upstream_track,
            self.flash_track,
            self.ram_track,
        );
        self.upstream_track.assert_hready_is_reflected();
    }

    fn tick_extra(&mut self) {
        self.tocked_this_cycle = false;
        self.flash_track.update();
        self.ram_track.update();
        self.upstream_track.update();

        self.poly_pos.keep_current_as_next();
        self.most_recent_tag_update_series_had_prefetch
            .keep_current_as_next();
        self.most_recent_tag_update_reused_previous_next_set_pos
            .keep_current_as_next();
        self.pending_request_type.keep_current_as_next();
        if self.next_mode.is_some()
            && !self.pending_addr.is_set()
            && *self.pending_request_type == PendingRequestType::None
            && !self.pending_tag_update.is_set()
            && !self.delayed_msg.is_set()
        {
            // Do the actual mode changing here
            if self.mode == Mode::Cache {
                self.reset_cache_metadata();
            }
            self.mode = self.next_mode.take().unwrap();
        } else if self.mode == Mode::Cache {
            keep_flop_if_set(&mut self.tag_prefetch);
            keep_flop_if_set(&mut self.pending_addr);
            keep_flop_if_set(&mut self.pending_tag_update);
            keep_flop_if_set(&mut self.pending_flash_read);
            keep_flop_if_set(&mut self.set_of_most_recent_tag_update);
            increment_counter_flop_if_set(&mut self.cycles_since_last_flash_request);
            increment_counter_flop_if_set(&mut self.cycles_since_last_flash_read);
            increment_counter_flop_if_set(&mut self.cycles_since_last_gpram_read);
            increment_counter_flop_if_set(&mut self.cycles_since_most_recent_tag_update);
        }
        self.request_this_tick = RequestThisTick::None;
    }
}

fn keep_flop_if_set<T>(flop: &mut CombFlop<T>) {
    if flop.is_set() {
        flop.keep_current_as_next();
    }
}

fn increment_counter_flop_if_set(flop: &mut CombFlop<u64>) {
    if flop.is_set() {
        increment_flop(flop);
    }
}

fn increment_flop(flop: &mut CombFlop<u64>) {
    flop.set_next(**flop + 1);
}

#[derive(Copy, Clone, Debug)]
struct AddrPart {
    _offset: u32,
    set: u32,
    tag: u32,
}
impl From<Address> for AddrPart {
    fn from(a: Address) -> Self {
        let addr_val = a.to_const();
        Self {
            _offset: addr_val & 0x7,
            set: (addr_val >> 3) & 0xFF,
            tag: (addr_val >> 11) & 0x1F_FFFF,
        }
    }
}

type MainComponent = <CacheComponent as Subcomponent>::Component;
impl CacheComponent {
    pub(crate) fn new() -> Self {
        Self {
            mode: Mode::GPRAM,
            next_mode: None,
            poly_pos: CombFlop::new_from(0),
            next_set_pos: SeqFlopMemoryBank::new([0; 256]),
            tag_update_treated_set_as_full: SeqFlopMemoryBank::new(FixedBitSet::with_capacity(256)),
            tag_ram: SeqFlopMemoryBank::new([TagRamEntry::default(); 1024]),
            delayed_msg: CombFlop::new(),
            pending_addr: CombFlop::new(),
            pending_tag_update: CombFlop::new(),
            pending_flash_read: CombFlop::new(),
            most_recent_tag_update_reused_previous_next_set_pos: CombFlop::new_from(false),
            pending_request_type: CombFlop::new_from(PendingRequestType::None),
            tag_prefetch: CombFlop::new(),
            most_recent_tag_update_series_had_prefetch: CombFlop::new_from(false),
            set_of_most_recent_tag_update: CombFlop::new(),
            cycles_since_last_flash_request: CombFlop::new(),
            cycles_since_last_flash_read: CombFlop::new(),
            cycles_since_last_gpram_read: CombFlop::new(),
            cycles_since_most_recent_tag_update: CombFlop::new(),

            request_this_tick: RequestThisTick::None,

            tocked_this_cycle: false,
            flash_track: Default::default(),
            ram_track: Default::default(),
            upstream_track: Default::default(),
        }
    }

    pub(crate) fn tick(comp: &mut MainComponent, _ctx: &mut Context) {
        let this = Self::component_to_member_mut(comp);
        if this.should_update_tag_ram() {
            let way = this.update_tag_ram();

            let set = this.pending_tag_update_set();
            let data = this.pending_tag_update.data;
            CacheRAMComponent::pend_memory_update(comp, gpram_addr(set, way), data);
        }
    }

    /// This delay can be observed in cases from Figures 6.2 and 6.3 from the cache thesis.
    fn should_update_tag_ram(&self) -> bool {
        self.pending_tag_update.is_set()
            && self
                .cycles_since_last_flash_read
                .is_set_and(|cycles| *cycles == 2)
    }

    fn update_tag_ram(&mut self) -> u8 {
        let PendingTagUpdate {
            addr,
            is_prefetched,
            set_was_full_according_to_tag_ram_when_tag_ram_was_read,
            ..
        } = *self.pending_tag_update;
        let AddrPart { set, tag, .. } = addr.into();

        // See cache thesis, Chapter 4 and Figure 6.7.
        let way = if set_was_full_according_to_tag_ram_when_tag_ram_was_read {
            self.choose_way_according_to_poly()
        } else {
            self.choose_way_according_to_next_set_pos()
        };

        // See cache thesis, Section 5.3.
        if !is_prefetched {
            self.flop_advance_poly_pos();
        }

        if !self.tag_update_treated_set_as_full[set as usize]
            && self.pending_tag_update_is_treating_set_as_full()
        {
            self.flop_mark_that_tag_update_treated_set_as_full(set);
        }

        self.flop_update_most_recent_tag_update_metadata();

        self.flop_mark_line_in_tag_ram_as_cached(set, way, tag);

        self.pending_tag_update.unset_next();

        trace!(
            "{} addr={:#?} set={:#?} way={:#?} pref={:#?} full_on_read={:?};",
            "CACHE TAG UPDATE".bright_cyan(),
            addr,
            set,
            way,
            is_prefetched,
            set_was_full_according_to_tag_ram_when_tag_ram_was_read,
        );

        way
    }

    fn choose_way_according_to_poly(&self) -> u8 {
        let poly_val = POLY[*self.poly_pos as usize];
        if self.poly_plus_one_special_case_is_active() {
            (poly_val + 1) % 4
        } else {
            poly_val
        }
    }

    /// For an example of this edge case, see cache thesis, Figure 6.8.
    fn poly_plus_one_special_case_is_active(&self) -> bool {
        if *confeature::cache::NAIVE_MODE {
            return false;
        }

        self.tag_update_series_is_in_progress()
            && *self.set_of_most_recent_tag_update == self.pending_tag_update_set()
            && self.tag_update_treated_set_as_full[*self.set_of_most_recent_tag_update as usize]
            && !(*self.most_recent_tag_update_series_had_prefetch
                || self.pending_tag_update.is_prefetched)
    }

    /// A TAG update series corresponds to the miss series described in Section 6.6 in the cache thesis.
    /// The condition in this function is equivalent to the formulation from the thesis - the number of cycles
    /// between tag updates is the same as between ends of data phases, and a data phase of a cache miss lasts
    /// 4 cycles if it is unprefetched and 3 if it is prefetched, but due to a special case prefetch cannot happen
    /// 2 cycles after the end of a cache miss's data phase.
    fn tag_update_series_is_in_progress(&self) -> bool {
        self.cycles_since_most_recent_tag_update
            .is_set_and(|cycles| *cycles <= 5)
    }

    fn pending_tag_update_set(&self) -> u32 {
        AddrPart::from(self.pending_tag_update.addr).set
    }

    fn choose_way_according_to_next_set_pos(&mut self) -> u8 {
        let set = self.pending_tag_update_set();

        if self.reuse_previous_next_set_pos_special_case_is_active() {
            (self.next_set_pos[set as usize] + 4 - 1) % 4
        } else {
            // It is deliberately possible for next_set_pos[set] to be used after it has wrapped around and is equal to 0.
            // See Figure 6.7 in the cache thesis.
            self.flop_advance_next_set_pos(set);
            self.next_set_pos[set as usize]
        }
    }

    // For examples of this behavior, see Figures 6.5 and 6.6 in the cache thesis.
    fn reuse_previous_next_set_pos_special_case_is_active(&self) -> bool {
        if *confeature::cache::NAIVE_MODE {
            return false;
        }

        !self
            .pending_tag_update
            .set_was_full_according_to_tag_ram_when_tag_ram_was_read
            && self.current_tag_update_series_had_prefetch()
            && *self.set_of_most_recent_tag_update == self.pending_tag_update_set()
            && !*self.most_recent_tag_update_reused_previous_next_set_pos
    }

    fn current_tag_update_series_had_prefetch(&self) -> bool {
        self.tag_update_series_is_in_progress() && *self.most_recent_tag_update_series_had_prefetch
    }

    fn flop_advance_next_set_pos(&mut self, set: u32) {
        self.next_set_pos
            .mutate_next(u8::try_from(set).unwrap(), |next_set_pos, set| {
                next_set_pos[set as usize] = (next_set_pos[set as usize] + 1) % 4;
            });
    }

    fn flop_advance_poly_pos(&mut self) {
        self.poly_pos
            .set_next((*self.poly_pos + 1) % u8::try_from(POLY.len()).unwrap());
    }

    fn pending_tag_update_is_treating_set_as_full(&self) -> bool {
        self.set_is_full_according_to_tag_ram(self.pending_tag_update_set())
            && !self.reuse_previous_next_set_pos_special_case_is_active()
    }

    fn set_is_full_according_to_tag_ram(&self, set: u32) -> bool {
        self.tag_ram[tag_ram_index(set, 3) as usize].valid
    }

    fn flop_mark_that_tag_update_treated_set_as_full(&mut self, set: u32) {
        self.tag_update_treated_set_as_full.mutate_next(
            u8::try_from(set).unwrap(),
            |tag_update_treated_set_as_full, set| {
                tag_update_treated_set_as_full.insert(set as usize);
            },
        );
    }

    fn flop_update_most_recent_tag_update_metadata(&mut self) {
        self.cycles_since_most_recent_tag_update.set_next(1);
        self.most_recent_tag_update_series_had_prefetch.set_next(
            self.pending_tag_update.is_prefetched || self.current_tag_update_series_had_prefetch(),
        );
        self.set_of_most_recent_tag_update
            .set_next(self.pending_tag_update_set());
        self.most_recent_tag_update_reused_previous_next_set_pos
            .set_next(self.reuse_previous_next_set_pos_special_case_is_active());
    }

    fn flop_mark_line_in_tag_ram_as_cached(&mut self, set: u32, way: u8, tag: u32) {
        self.tag_ram
            .mutate_next((tag_ram_index(set, way), tag), |tag_ram, (index, tag)| {
                tag_ram[index as usize] = TagRamEntry { valid: true, tag };
            });
    }

    pub(crate) fn tock(comp: &mut MainComponent, ctx: &mut Context) {
        let mut this = Self::get_proxy(comp);
        // Tock only once (TODO: use statemachine)
        if this.tocked_this_cycle {
            return;
        }
        this.tocked_this_cycle = true;
        if this.delayed_msg.is_set() {
            let reply = this
                .delayed_msg
                .addr_phase
                .make_reply::<Self, _>(AhbResponseControl::Pending, Default::default());
            this.upstream_track.set_last_reply(reply.meta);
            <Self as AHBSlavePortOutput>::send_ahb_output(this.component_mut(), ctx, reply);

            let delayed_msg = this.delayed_msg.take();
            Self::read_tag_ram_and_request_data(this.component_mut(), ctx, delayed_msg, false);
        }
    }

    pub(crate) fn get_mode(comp: &MainComponent) -> (Mode, Option<Mode>) {
        let this = Self::component_to_member(comp);
        (this.mode, this.next_mode)
    }

    pub(crate) fn is_mode_changing(comp: &MainComponent) -> bool {
        let this = Self::component_to_member(comp);
        this.next_mode.is_some()
    }

    pub(crate) fn get_target_mode_for_addr_routing(&self) -> Mode {
        // TODO: we should prefer off during transition!
        self.next_mode.unwrap_or(self.mode)
    }

    pub(crate) fn set_mode(comp: &mut MainComponent, _ctx: &mut Context, mode: Mode) {
        let mut this = Self::get_proxy(comp);
        info!(
            "{} requested mode CHANGE {:?} -> {:?}",
            "Cache".bright_cyan(),
            this.mode,
            mode
        );
        debug_assert!(
            this.next_mode.is_none(),
            "VIMS::CTL should ignore writes during transition."
        );
        if mode != this.mode {
            this.next_mode = Some(mode);
        }
        CodeCacheLineBuffer::set_cache_enabled(this.component_mut(), mode == Mode::Cache);
    }

    fn reset_cache_metadata(&mut self) {
        self.next_set_pos
            .mutate_next(Default::default(), |next_set_pos, _| next_set_pos.fill(0));
        self.tag_update_treated_set_as_full.mutate_next(
            Default::default(),
            |set_had_load_while_full, _| {
                set_had_load_while_full.clear();
            },
        );
        self.tag_ram.mutate_next(Default::default(), |tag_ram, _| {
            tag_ram.fill(TagRamEntry::default());
        });

        // updated prior to this call
        // self.delayed_msg.unset_next();
        debug_assert!(!self.delayed_msg.is_next_set());
        self.most_recent_tag_update_series_had_prefetch
            .set_next(false);
        self.most_recent_tag_update_reused_previous_next_set_pos
            .set_next(false);
        self.pending_request_type.set_next(PendingRequestType::None);

        // Asserted previously
        debug_assert!(!self.pending_addr.is_set());
        self.pending_addr.unset_next();
        self.pending_tag_update.unset_next();
        self.pending_flash_read.unset_next();
        self.tag_prefetch.ignore();
        self.tag_prefetch.unset_next();
        self.set_of_most_recent_tag_update.ignore();
        self.set_of_most_recent_tag_update.unset_next();

        self.cycles_since_last_flash_request.ignore();
        self.cycles_since_last_flash_request.unset_next();
        self.cycles_since_last_flash_read.ignore();
        self.cycles_since_last_flash_read.unset_next();
        self.cycles_since_last_gpram_read.ignore();
        self.cycles_since_last_gpram_read.unset_next();
        self.cycles_since_most_recent_tag_update.ignore();
        self.cycles_since_most_recent_tag_update.unset_next();
    }

    fn read_tag_ram_and_request_data(
        comp: &mut MainComponent,
        ctx: &mut Context,
        mut msg: MasterToSlaveWires<<CacheComponent as AHBPortConfig>::Data>,
        is_prefetched: bool,
    ) {
        let this = Self::component_to_member_mut(comp);

        let addr = &mut msg.addr_phase.meta.meta_mut().unwrap().addr;
        let AddrPart { set, tag, .. } = (*addr).into();

        if let Some(way) = this.get_way_in_which_line_is_cached_according_to_tag_ram(set, tag) {
            this.request_this_tick = RequestThisTick::Cache;
            this.pending_request_type
                .set_next(PendingRequestType::Cache);
            this.cycles_since_last_gpram_read.set_next(0);

            *addr = gpram_addr(set, way);
            Self::send_to_ram(comp, ctx, msg);
        } else {
            this.request_this_tick = RequestThisTick::Flash;
            this.pending_flash_read.set_next(PendingFlashRead {
                is_prefetched,
                set_was_full_according_to_tag_ram_when_tag_ram_was_read: this
                    .set_is_full_according_to_tag_ram(set),
            });
            this.pending_request_type
                .set_next(PendingRequestType::Flash);
            this.cycles_since_last_flash_request.set_next(1);

            Self::send_to_flash(comp, ctx, msg);
        }
    }

    fn get_way_in_which_line_is_cached_according_to_tag_ram(
        &self,
        set: u32,
        tag: u32,
    ) -> Option<u8> {
        for way in 0..4 {
            let entry = self.tag_ram[tag_ram_index(set, way) as usize];
            if entry.valid && entry.tag == tag {
                return Some(way);
            }
        }
        None
    }
}

fn tag_ram_index(set: u32, way: u8) -> u32 {
    set + 256 * u32::from(way)
}

fn gpram_addr(set: u32, way: u8) -> Address {
    soc::GPRAM::ADDR.offset(tag_ram_index(set, way) * 8)
}

impl AHBPortConfig for CacheComponent {
    type Data = DataBus;
    type Component = MainComponent;
    const TAG: &'static str = "Cache";
}

impl AHBMasterPortInput for CacheComponent {
    fn on_ahb_input(
        comp: &mut MainComponent,
        ctx: &mut Context,
        msg: SlaveToMasterWires<Self::Data>,
    ) {
        let mut this = Self::get_proxy(comp);
        if this.mode == Mode::Cache
            && msg.meta == AhbResponseControl::Success
            && this.request_this_tick == RequestThisTick::None
        {
            this.pending_addr.unset_next();
            this.pending_request_type.set_next(PendingRequestType::None);
        }
        this.upstream_track.set_last_reply(msg.meta);
        <Self as AHBSlavePortOutput>::send_ahb_output(this.component_mut(), ctx, msg);
    }
}

impl AHBSlavePortInput for CacheComponent {
    fn on_ahb_input(
        comp: &mut MainComponent,
        ctx: &mut Context,
        msg: MasterToSlaveWires<Self::Data>,
    ) {
        // make sure we tocked already
        Self::tock(comp, ctx);
        let mut this = Self::get_proxy(comp);
        this.upstream_track.set_last_addr(msg.addr_phase.clone());
        // We need to use the new mode in case someone transitions GPRAM <-> Cache directly
        match this.get_target_mode_for_addr_routing() {
            Mode::Cache => {
                // Any decoders must reflect this wire
                if msg.addr_phase.HREADYIN() {
                    if this.request_this_tick != RequestThisTick::None {
                        this.cancel_previous_request_from_this_tick();
                    }
                    if msg.addr_phase.meta.address().is_some() {
                        Self::handle_new_request(comp, ctx, msg);
                    }
                }
            }
            _ => <Self as AHBMasterPortOutput>::send_ahb_output(this.component_mut(), ctx, msg),
        }

        Self::keep_ahb_invariants(comp, ctx);
    }
}

impl CacheComponent {
    fn cancel_previous_request_from_this_tick(&mut self) {
        self.pending_addr.unset_next();
        cancel_update_of_persistent_flop(&mut self.tag_prefetch);
        self.pending_request_type.set_next(PendingRequestType::None);
        match self.request_this_tick {
            RequestThisTick::Delayed => {
                self.delayed_msg.unset_next();
            }
            RequestThisTick::Flash => {
                self.pending_flash_read.unset_next();
                cancel_reset_of_counter_flop(&mut self.cycles_since_last_flash_read);
            }
            RequestThisTick::Cache => {
                cancel_reset_of_counter_flop(&mut self.cycles_since_last_gpram_read);
            }
            RequestThisTick::None => unreachable!(),
        }
        self.request_this_tick = RequestThisTick::None;
    }

    fn handle_new_request(
        comp: &mut MainComponent,
        ctx: &mut Context,
        msg: MasterToSlaveWires<<Self as AHBPortConfig>::Data>,
    ) {
        let mut this = Self::get_proxy(comp);

        let addr = msg.addr_phase.meta.address().unwrap();

        this.pending_addr.set_next(addr);
        this.tag_prefetch.set_next(u32::from(addr) + 8);

        if this.addr_is_effectively_prefetched(addr) {
            Self::read_tag_ram_and_request_data(comp, ctx, msg, true);
        } else {
            Self::delay_request(comp, ctx, msg);
        }
    }

    fn addr_is_effectively_prefetched(&self, addr: Address) -> bool {
        !self.do_not_use_tag_prefetch_special_case_is_active()
            && self
                .tag_prefetch
                .is_set_and(|tag_prefetch| *tag_prefetch == u32::from(addr))
    }

    // For examples, see end of Section 5.2 and Figure 6.4 in the cache thesis.
    fn do_not_use_tag_prefetch_special_case_is_active(&self) -> bool {
        if *confeature::cache::NAIVE_MODE {
            return false;
        }

        let special_case_1_is_active = || {
            self.cycles_since_last_flash_read
                .is_set_and(|cycles_since_flash| *cycles_since_flash == 2)
        };
        let special_case_2_is_active = || {
            self.cycles_since_last_flash_read
                .is_set_and(|cycles_since_flash| *cycles_since_flash == 3)
                && self
                    .cycles_since_last_gpram_read
                    .is_set_and(|cycles_since_gpram| *cycles_since_gpram == 0)
        };
        special_case_1_is_active() || special_case_2_is_active()
    }

    fn delay_request(
        comp: &mut MainComponent,
        ctx: &mut Context,
        msg: MasterToSlaveWires<<Self as AHBPortConfig>::Data>,
    ) {
        let mut this = Self::get_proxy(comp);

        this.request_this_tick = RequestThisTick::Delayed;
        this.pending_request_type
            .set_next(PendingRequestType::Delayed);
        this.delayed_msg.set_next(msg.clone());

        let no_sel_msg = MasterToSlaveWires {
            addr_phase: MasterToSlaveAddrPhase::not_selected::<Self>(),
            ..msg
        };
        Self::send_to_flash(this.component_mut(), ctx, no_sel_msg);
        VIMSComponent::cache_arbitration_hax(comp, true);
    }

    fn send_to_flash(
        comp: &mut MainComponent,
        ctx: &mut Context,
        msg: MasterToSlaveWires<<Self as AHBPortConfig>::Data>,
    ) {
        let mut this = Self::get_proxy(comp);
        this.flash_track.set_last_addr(msg.addr_phase.clone());
        <FlashPort as AHBMasterPortOutput>::send_ahb_output(this.component_mut(), ctx, msg);
    }

    fn send_to_ram(
        comp: &mut MainComponent,
        ctx: &mut Context,
        msg: MasterToSlaveWires<<Self as AHBPortConfig>::Data>,
    ) {
        let mut this = Self::get_proxy(comp);
        this.ram_track.set_last_addr(msg.addr_phase.clone());
        <CacheRAMPort as AHBMasterPortOutput>::send_ahb_output(this.component_mut(), ctx, msg);
    }
    // Call this after no more sending wil be performed
    fn keep_ahb_invariants(comp: &mut MainComponent, ctx: &mut Context) {
        let mut this = Self::get_proxy(comp);
        if !this.flash_track.is_last_addr_set()
            && let Some(addr) = this.flash_track.data_address()
        {
            let msg = MasterToSlaveWires {
                addr_phase: MasterToSlaveAddrPhase::not_selected::<Self>(),
                data_phase: MasterToSlaveDataPhase::continue_read(addr),
            };
            Self::send_to_flash(this.component_mut(), ctx, msg);
        }
        if !this.ram_track.is_last_addr_set()
            && let Some(addr) = this.ram_track.data_address()
        {
            let msg = MasterToSlaveWires {
                addr_phase: MasterToSlaveAddrPhase::not_selected::<Self>(),
                data_phase: MasterToSlaveDataPhase::continue_read(addr),
            };
            Self::send_to_ram(this.component_mut(), ctx, msg);
        }
    }
}

fn cancel_update_of_persistent_flop<T>(flop: &mut CombFlop<T>) {
    if flop.is_set() {
        flop.keep_current_as_next();
    } else {
        flop.unset_next();
    }
}

fn cancel_reset_of_counter_flop(flop: &mut CombFlop<u64>) {
    if flop.is_set() {
        increment_flop(flop);
    } else {
        flop.unset_next();
    }
}

impl AHBMasterPortOutput for CacheComponent {
    fn send_ahb_output(
        comp: &mut MainComponent,
        ctx: &mut Context,
        msg: MasterToSlaveWires<Self::Data>,
    ) {
        let mut this = Self::get_proxy(comp);
        if msg_addr_is_in_flash(&msg) {
            Self::send_to_flash(this.component_mut(), ctx, msg);
        } else {
            Self::send_to_ram(this.component_mut(), ctx, msg);
        }
    }
}

fn msg_addr_is_in_flash(msg: &MasterToSlaveWires<<CacheComponent as AHBPortConfig>::Data>) -> bool {
    msg.addr_phase
        .meta
        .address()
        .is_some_and(|addr| soc::FLASHMEM::ADDR_SPACE.contains(&addr))
}

make_port_struct!(pub(crate) FlashPort);

impl AHBPortConfig for FlashPort {
    type Data = <CacheComponent as AHBPortConfig>::Data;
    type Component = MainComponent;
    const TAG: &'static str = "FlashPort";
}

impl AHBMasterPortInput for FlashPort {
    fn on_ahb_input(
        comp: &mut MainComponent,
        ctx: &mut Context,
        msg: SlaveToMasterWires<Self::Data>,
    ) {
        let this = CacheComponent::component_to_member_mut(comp);
        this.flash_track.set_last_reply(msg.meta);

        debug_assert!(msg.meta.HRESP() != ERROR, "Unhandled errors: {msg:?}");
        if *this.pending_request_type == PendingRequestType::Flash {
            if msg.meta.is_done() {
                this.cycles_since_last_flash_read.set_next(1);
                let PendingFlashRead {
                    is_prefetched,
                    set_was_full_according_to_tag_ram_when_tag_ram_was_read,
                } = *this.pending_flash_read;
                let DataBus::Quad(ref data) = msg.data else {
                    unreachable!()
                };

                this.pending_tag_update.set_next(PendingTagUpdate {
                    addr: *this.pending_addr,
                    data: data.to_le_bytes(),
                    is_prefetched,
                    set_was_full_according_to_tag_ram_when_tag_ram_was_read,
                });

                if this.request_this_tick != RequestThisTick::Flash {
                    this.pending_flash_read.unset_next();
                }
            }
            <CacheComponent as AHBMasterPortInput>::on_ahb_input(comp, ctx, msg);
        }
    }
}

make_port_struct!(pub(crate) CacheRAMPort);

impl AHBPortConfig for CacheRAMPort {
    type Data = <CacheComponent as AHBPortConfig>::Data;
    type Component = MainComponent;
    const TAG: &'static str = "CacheRAMPort";
}

impl AHBMasterPortInput for CacheRAMPort {
    fn on_ahb_input(
        comp: &mut MainComponent,
        ctx: &mut Context,
        msg: SlaveToMasterWires<Self::Data>,
    ) {
        let mut this = CacheComponent::get_proxy(comp);
        this.ram_track.set_last_reply(msg.meta);

        if this.mode == Mode::Cache
            && matches!(
                *this.pending_request_type,
                PendingRequestType::Cache | PendingRequestType::None
            )
        {
            #[cfg(debug_assertions)]
            if *this.pending_request_type == PendingRequestType::Cache {
                let DataBus::Quad(ref data) = msg.data else {
                    unreachable!()
                };
                FlashProxy.assert_addr_content(ctx, *this.pending_addr, data.to_le_bytes());
            }

            <CacheComponent as AHBMasterPortInput>::on_ahb_input(this.component_mut(), ctx, msg);
        } else if this.mode == Mode::GPRAM {
            // Just forward
            <CacheComponent as AHBMasterPortInput>::on_ahb_input(this.component_mut(), ctx, msg);
        } else {
            // We didn't ask for that!
            debug_assert!(
                msg.meta.is_done() && !msg.data.is_present(),
                "Cache in state {:?} did not forward {msg:?}",
                this.mode
            );
        }
    }
}
