use std::time::{Duration, Instant};
use channel::async_channel::OneshotSender;

pub type ScheduleId = u64;

pub type RepeatCallback = Box<dyn FnMut(ScheduleId) + Send + 'static>;
pub type OnceCallback = Box<dyn FnOnce(ScheduleId) + Send + 'static>;

pub enum ScheduleSpec {
    // OnceAfter(Duration, OnceCallback),
    OnceAt(Instant, OnceCallback),
    Repeat(Duration, RepeatCallback),
}

impl ScheduleSpec {
    pub fn once_at(instant: Instant, cb: OnceCallback) -> Self {
        ScheduleSpec::OnceAt(instant, cb)
    }

    pub fn once_after(duration: Duration, cb: OnceCallback) -> Self {
        let instant = Instant::now() + duration;
        ScheduleSpec::OnceAt(instant, cb)
    }

    pub fn repeat(interval: Duration, cb: RepeatCallback) -> Self {
        ScheduleSpec::Repeat(interval, cb)
    }

    pub fn when(&self) -> Instant {
        match self {
            // ScheduleSpec::OnceAfter(delay, _) => Instant::now() + *delay,
            ScheduleSpec::OnceAt(instant, _) => *instant,
            ScheduleSpec::Repeat(interval, _) => Instant::now() + *interval,
        }
    }

    // pub fn is_repeat(&self) -> bool {
    //     match self {
    //         ScheduleSpec::Repeat(_, _) => true,
    //         _ => false,
    //     }
    // }
}

pub enum Command {
    Add(ScheduleSpec, OneshotSender<ScheduleId>),
    Remove(ScheduleId),
}
