use crate::command::{ScheduleSpec, TaskId};
use std::cmp::Ordering;
use std::time::Instant;

pub(super) struct ScheduledTask {
    task_id: TaskId,
    when: Instant,
    spec: ScheduleSpec,
}

impl ScheduledTask {
    pub fn new(task_id: TaskId, spec: ScheduleSpec) -> Self {
        let when = spec.when();
        ScheduledTask {
            task_id,
            when,
            spec,
        }
    }

    pub fn task_id(&self) -> &TaskId {
        &self.task_id
    }

    // pub fn timeout(&self, now: Instant) -> bool {
    //     self.when <= now
    // }

    pub fn when(&self) -> Instant {
        self.when
    }

    pub fn execute(self) -> Option<Self> {
        let ScheduledTask { task_id, spec, .. } = self;
        match spec {
            ScheduleSpec::OnceAt(_, cb) => {
                cb(task_id);
                None
            }
            ScheduleSpec::Repeat(duration, mut cb) => {
                cb(task_id);
                let now = Instant::now();
                let when = now + duration;
                let task_id = self.task_id;
                let spec = ScheduleSpec::Repeat(duration, cb);
                let new_task = ScheduledTask {
                    task_id,
                    when,
                    spec,
                };
                Some(new_task)
            }
        }
    }

    // pub fn re_schedule(self) -> Option<Self> {
    //     todo!("re schedule")
    // }
}

impl Ord for ScheduledTask {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .when
            .cmp(&self.when)
            .then_with(|| self.task_id.cmp(&other.task_id))
    }
}

impl PartialOrd for ScheduledTask {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for ScheduledTask {}

impl PartialEq for ScheduledTask {
    fn eq(&self, other: &Self) -> bool {
        self.task_id == other.task_id
    }
}
