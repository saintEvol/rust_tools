use crate::command::{Command, OnceCallback, RepeatCallback, ScheduleId, ScheduleSpec};
use crate::error::ScheduleError;
use crate::scheduled_task::ScheduledTask;
use channel::async_channel::{SendError, UnboundedReceiver, UnboundedSender, one_shot, unbounded};
#[cfg(feature = "dioxus")]
use dioxus::prelude::spawn;
use futures::FutureExt;
use std::collections::{BinaryHeap, HashSet};
use std::time::{Duration, Instant};
#[cfg(all(not(target_arch = "wasm32"), not(feature = "dioxus")))]
use tokio::spawn;
use tracing::{error, warn};

pub struct Scheduler {
    tx: UnboundedSender<Command>,
}

impl Scheduler {
    pub fn new() -> Self {
        let (tx, rx) = unbounded();
        Self::run(rx);
        Scheduler { tx }
    }

    pub async fn once_after(
        &mut self,
        duration: Duration,
        handle: impl FnOnce(ScheduleId) + Send + 'static,
    ) -> Result<ScheduleId, ScheduleError> {
        let handle = Box::new(handle) as OnceCallback;
        let spec = ScheduleSpec::once_after(duration, handle);
        self.add_schedule_spec(spec).await
    }

    pub async fn once_at(
        &mut self,
        instant: Instant,
        handle: impl FnOnce(ScheduleId) + Send + 'static,
    ) -> Result<ScheduleId, ScheduleError> {
        let handle = Box::new(handle) as OnceCallback;
        let spec = ScheduleSpec::once_at(instant, handle);
        self.add_schedule_spec(spec).await
    }

    pub async fn repeat(
        &mut self,
        interval: Duration,
        handle: impl FnMut(ScheduleId) + Send + 'static,
    ) -> Result<ScheduleId, ScheduleError> {
        let handle = Box::new(handle) as RepeatCallback;
        let spec = ScheduleSpec::repeat(interval, handle);
        self.add_schedule_spec(spec).await
    }

    pub fn remove(&mut self, id: ScheduleId) -> Result<(), SendError> {
        let cmd = Command::Remove(id);
        self.tx.send(cmd)
    }

    async fn add_schedule_spec(&mut self, spec: ScheduleSpec) -> Result<ScheduleId, ScheduleError> {
        let (tx, rx) = one_shot();
        let cmd = Command::Add(spec, tx);
        self.tx
            .send(cmd)
            .map_err(|e| ScheduleError::Channel(e.to_string()))?;
        let ret = rx
            .recv()
            .await
            .map_err(|e| ScheduleError::Channel(e.to_string()))?;
        Ok(ret)
    }

    fn run(mut rx: UnboundedReceiver<Command>) {
        let f = async move {
            let mut next_id: ScheduleId = 1;
            let mut deleting = HashSet::<ScheduleId>::new();
            let mut tasks = BinaryHeap::<ScheduledTask>::new();
            loop {
                let now = Instant::now();
                let delay_future =
                    if let Some(when) = peek_valid_task_deadline(&mut deleting, &mut tasks) {
                        if now >= when {
                            // 马上完成
                            futures::future::ready(()).left_future()
                        } else {
                            // 等待超时
                            timer::timer::sleep_until(when).right_future()
                        }
                    } else {
                        // 没有任务，等待一天
                        timer::timer::sleep_until(now + Duration::from_secs(86400)).right_future()
                    };

                futures::select! {
                    cmd = rx.recv().fuse() => {
                        match cmd {
                            None => {
                                // 通道已经关闭，退出循环
                                break;
                            }
                            Some(cmd) => {
                                match cmd {
                                    Command::Add(spec, notifier) => {
                                        let id = next_id;
                                        let task = ScheduledTask::new(id, spec);
                                        tasks.push(task);
                                        next_id += 1;
                                        if let Err(_) = notifier.send(id) {
                                            error!("发送[ScheduleId]失败: 接收方提前关闭");
                                        }
                                    }
                                    Command::Remove(id) => {
                                        deleting.insert(id);
                                    }
                                }
                            }
                        }
                    }
                    _ = delay_future.fuse() => {
                        let now = Instant::now();
                        // 取出可用任务，并检查是否超时
                        while let Some(when) = peek_valid_task_deadline(&mut deleting, &mut tasks) {
                            if now >= when {
                                if let Some(task) = tasks.pop() {
                                    if let Some(new_task) = task.execute() {
                                        tasks.push(new_task);
                                    }
                                }
                            }
                        }

                    }
                }
            }
            warn!("Scheduler已经退出");
        };

        fn peek_valid_task_deadline(
            deleting: &mut HashSet<ScheduleId>,
            tasks: &mut BinaryHeap<ScheduledTask>,
        ) -> Option<Instant> {
            while let Some((task_id, when)) = tasks.peek().map(|t| (*t.task_id(), t.when())) {
                if deleting.contains(&task_id) {
                    tasks.pop();
                    deleting.remove(&task_id);
                } else {
                    return Some(when);
                }
            }

            None
        }

        #[cfg(feature = "dioxus")]
        {
            spawn(f);
            return;
        }

        #[cfg(all(not(target_arch = "wasm32"), not(feature = "dioxus")))]
        {
            spawn(f);
            return;
        }

        // wasm,但是又没有启用dioxus,当前不会真正启动任务
        #[cfg(all(target_arch = "wasm32", not(feature = "dioxus")))]
        {
            println!(
                "编译目标为:wasm32,但是未启用feature: dioxus, 这种情况下当前没有可用异步运行时，因此不会真正启动[Scheduler]"
            );
            compile_error!(
                "编译目标为:wasm32,但是未启用feature: dioxus, 这种情况下当前没有可用异步运行时，因此不会真正启动[Scheduler]"
            );
            panic!(
                "编译目标为:wasm32,但是未启用feature: dioxus, 这种情况下当前没有可用异步运行时，因此不会真正启动[Scheduler]"
            );
        }
    }
}
