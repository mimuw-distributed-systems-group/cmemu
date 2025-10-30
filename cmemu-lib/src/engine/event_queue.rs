use super::time::{Duration, Timepoint};
use crate::build_data;
#[allow(unused_imports)]
use crate::proxy::event_data::{COMPONENTS_COUNT, EventData, HANDLERS_COUNT};
use enum_map::Enum;
use heapless::Deque as ArrayDeque;
use heapless::sorted_linked_list::{Min, SortedLinkedList};
use std::cmp::Ordering;
use std::mem::swap;
use std::ops::Not;

#[derive(Debug)]
pub(super) struct Event {
    timepoint: Timepoint,
    // unique-ish ID used for revocations
    event_id: u32,
    // Note: the alignment is a bit wasteful
    payload: EventData,
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.timepoint == other.timepoint
    }
}
impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        self.timepoint.cmp(&other.timepoint)
    }
}
impl Eq for Event {}

/// Revocation token for future-scheduled events
#[derive(Debug)]
pub(crate) struct EventRevokeToken {
    timepoint: Timepoint,
    // unique-ish ID used for revocations
    event_id: u32,
    // Note: it is best-effort only - maybe add more fields (e.g., future overflow epoch)
    // if that's actually an issue.
}

impl EventRevokeToken {
    pub(crate) fn revoke(self, ctx: &mut super::Context) {
        ctx.event_queue_mut().revoke_event(self);
    }
}

// TODO: what is that!?
const RX_EVENT_SPAM_EXPECTED: usize = 128 / 4 + 4;

/// The size of queue-now - calling ``#[handler]``-s as a way of decoupling.
/// It is important for this to be small and reside in a CPU cache, (this is noticeable few % of performance).
/// TODO: don't include CDL handlers in `HANDLERS_COUNT` when building without CDL.
///       then just use the second format. +5 is here just because currently the count a is power of two.
/// TODO: reduce it somehow?? this is almost surely an overshot
#[cfg(not(feature = "cycle-debug-logger"))]
const QUEUE_NOW_MAX_EVENTS: usize =
    (COMPONENTS_COUNT + 5 + RX_EVENT_SPAM_EXPECTED).next_power_of_two();
#[cfg(feature = "cycle-debug-logger")]
const QUEUE_NOW_MAX_EVENTS: usize = (HANDLERS_COUNT + RX_EVENT_SPAM_EXPECTED).next_power_of_two();

// We have wakeup for oscillators and possible external events
const QUEUE_FUTURE_MAX_EVENTS: usize = <build_data::Oscillators as Enum>::LENGTH + 4;
// Note: previously, the future queues were using a slab::Slab allocator + std::BinaryHeap,
// but we needed an option to revoke an event.
// heapless::SortedLinkedList uses linear insertion, and is memcpy-free for resorts, so we dropped slab.
// Consider implementing some priority-queue with revocations (e.g., based on log-structured merge trees).
// that is not allocating.
const _: () = assert!(
    QUEUE_FUTURE_MAX_EVENTS < 16,
    "Future queues use inefficient implementation, implement a better one before using  longer queues!"
);
type FutureQueue = SortedLinkedList<Event, Min, QUEUE_FUTURE_MAX_EVENTS, u8>;

// Intended use:
// events in the same tick should go with delay = 0; their order is preserved
// only ticks should have delay != 0; their order in same future timepoint is not preserved
pub(crate) struct EventQueue {
    current_time: Timepoint,
    queue_now: ArrayDeque<EventData, QUEUE_NOW_MAX_EVENTS>,
    queue_future: FutureQueue,
    overflow_buffer: FutureQueue,
    // Used for revocations
    events_counter: u32,
}

impl EventQueue {
    pub(super) fn new() -> Self {
        Self {
            current_time: Timepoint::from_picos(0),
            queue_now: ArrayDeque::new(),
            queue_future: FutureQueue::new_u8(),
            overflow_buffer: FutureQueue::new_u8(),
            events_counter: 0,
        }
    }

    pub(crate) fn add(&mut self, delay: Duration, payload: EventData) -> Option<EventRevokeToken> {
        // TODO: verify that it inlines enough or should we have a dedicated method?
        if delay == Duration::ZERO {
            self.queue_now
                .push_back(payload)
                .expect("queue_now overflowed, need to increase QUEUE_NOW_MAX_EVENTS");
            None
        } else {
            let (queue, timepoint) = match self.current_time.checked_add_duration(delay) {
                Some(t) => (&mut self.queue_future, t),
                None => (
                    &mut self.overflow_buffer,
                    self.current_time.wrapping_add_duration(delay),
                ),
            };
            self.events_counter = self.events_counter.wrapping_add(1);
            let token = EventRevokeToken {
                timepoint,
                event_id: self.events_counter,
            };
            queue.push(Event {
                timepoint,
                payload,
                event_id: self.events_counter,
            }).expect("future queue overflowed, need to increase QUEUE_FUTURE_MAX_EVENTS or make it dynamic!");
            Some(token)
        }
    }

    pub(super) fn pop_now(&mut self) -> Option<EventData> {
        self.queue_now.pop_front()
    }

    pub(super) fn pop(&mut self) -> Option<EventData> {
        if let ev @ Some(_) = self.queue_now.pop_front() {
            ev
        } else {
            if self.queue_future.is_empty() && !self.overflow_buffer.is_empty() {
                swap(&mut self.queue_future, &mut self.overflow_buffer);
            }
            self.queue_future.pop().map(|ev| {
                self.current_time = ev.timepoint;
                ev.payload
            })
        }
    }

    // Note: use EventRevokeToken::revoke as main API
    pub(super) fn revoke_event(&mut self, token: EventRevokeToken) {
        for queue in [&mut self.queue_future, &mut self.overflow_buffer] {
            if let Some(ev) =
                queue.find_mut(|e| e.timepoint == token.timepoint && e.event_id == token.event_id)
            {
                ev.pop();
            }
        }
    }

    pub(super) fn peek_timepoint(&self) -> Option<Timepoint> {
        self.queue_now
            .is_empty()
            .not()
            .then_some(self.current_time)
            .or_else(|| self.queue_future.peek().map(|i| i.timepoint))
            .or_else(|| self.overflow_buffer.peek().map(|i| i.timepoint))
    }
    pub(super) fn peek_payload(&self) -> Option<&EventData> {
        self.queue_now.front().or_else(|| {
            self.queue_future
                .peek()
                .or_else(|| self.overflow_buffer.peek())
                .map(|e| &e.payload)
        })
    }

    pub fn get_current_time(&self) -> Timepoint {
        self.current_time
    }

    #[cfg(test)]
    pub fn fake_time(&mut self, t: Timepoint) {
        // TODO: this assert no longer works, since some code may put calls to CDL
        assert!(
            //     self.queue_now.is_empty()
            self.queue_future.is_empty() && self.overflow_buffer.is_empty()
        );
        self.current_time = t;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overflowing_queue() {
        use crate::component::WakeupEvent;

        let mut q = EventQueue::new();

        let mut t = Timepoint::ZERO;
        q.fake_time(t);
        // Note: we need some EventData, quartz_tick is generated manually in the build script.
        let wake_up = || EventData::Wakeup(WakeupEvent::Radio);
        q.add(Duration::from_picos(1 << 63), wake_up());
        q.add(Duration::from_picos(1 << 62), wake_up());

        t = Timepoint::from_picos(1 << 62);
        assert_eq!(t, q.peek_timepoint().unwrap());
        assert!(matches!(q.pop(), Some(EventData::Wakeup(_))));
        assert_eq!(t, q.get_current_time());
        q.add(Duration::from_picos(0), EventData::Nop); // schedule for "now"
        assert_eq!(t, q.peek_timepoint().unwrap());
        assert!(matches!(q.peek_payload(), Some(EventData::Nop)));
        assert!(matches!(q.pop(), Some(EventData::Nop)));
        assert!(q.pop_now().is_none());
        assert_eq!(t, q.get_current_time());

        t = Timepoint::from_picos(1 << 63);
        assert_eq!(t, q.peek_timepoint().unwrap());
        assert!(matches!(q.pop(), Some(EventData::Wakeup(_))));
        assert!(q.pop().is_none());
        q.add(Duration::from_picos(0b110 << 61), wake_up()); // "overflow the time"
        q.add(Duration::from_picos(0b111 << 61), EventData::Nop); // "overflow the time"
        assert_eq!(t, q.get_current_time());

        t = Timepoint::from_picos(0b10 << 61);
        assert_eq!(t, q.peek_timepoint().unwrap());
        assert!(matches!(q.pop(), Some(EventData::Wakeup(_))));
        q.add(Duration::from_picos(0), wake_up()); // schedule for "now"
        assert_eq!(t, q.peek_timepoint().unwrap());
        assert!(matches!(q.pop_now(), Some(EventData::Wakeup(_))));
        assert!(q.pop_now().is_none());
        assert_eq!(t, q.get_current_time());

        t = Timepoint::from_picos(0b11 << 61);
        assert_eq!(t, q.peek_timepoint().unwrap());
        assert!(matches!(q.pop(), Some(EventData::Nop)));
        assert_eq!(t, q.get_current_time());
        assert!(q.pop().is_none());
    }
}
