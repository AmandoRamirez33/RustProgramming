use std::collections::VecDeque;
use std::sync::{
    Arc, Mutex, Condvar,
    atomic::{AtomicBool, AtomicUsize, Ordering},
};
use crate::task::task::{Task, Priority};

struct QueueState {
    high: VecDeque<Task>,
    medium: VecDeque<Task>,
    low: VecDeque<Task>,
}

pub struct WorkQueue {
    state: Mutex<QueueState>,
    cv: Condvar,
    shutdown: AtomicBool,

    submitted: AtomicUsize,
    completed: AtomicUsize,
    failed: AtomicUsize,
}

impl WorkQueue {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            state: Mutex::new(QueueState {
                high: VecDeque::new(),
                medium: VecDeque::new(),
                low: VecDeque::new(),
            }),
            cv: Condvar::new(),
            shutdown: AtomicBool::new(false),
            submitted: AtomicUsize::new(0),
            completed: AtomicUsize::new(0),
            failed: AtomicUsize::new(0),
        })
    }

    pub fn submit(&self, task: Task) {
        let mut state = self.state.lock().unwrap();
        match task.priority {
            Priority::High => state.high.push_back(task),
            Priority::Medium => state.medium.push_back(task),
            Priority::Low => state.low.push_back(task),
        }
        self.submitted.fetch_add(1, Ordering::Relaxed);
        self.cv.notify_one();
    }

    pub fn start_workers(self: &Arc<Self>, count: usize) {
        for _ in 0..count {
            let queue = Arc::clone(self);
            std::thread::spawn(move || {
                crate::worker::worker::worker_loop(queue);
            });
        }
    }

    pub fn shutdown(&self) {
        self.shutdown.store(true, Ordering::Relaxed);
        self.cv.notify_all();
    }

    pub fn queue_size(&self) -> usize {
        let state = self.state.lock().unwrap();
        state.high.len() + state.medium.len() + state.low.len()
    }

    pub fn stats(&self) -> (usize, usize, usize) {
        (
            self.submitted.load(Ordering::Relaxed),
            self.completed.load(Ordering::Relaxed),
            self.failed.load(Ordering::Relaxed),
        )
    }

    pub(crate) fn wait_for_task(&self) -> Option<Task> {
        let mut state = self.state.lock().unwrap();
        loop {
            if self.shutdown.load(Ordering::Relaxed) {
                return None;
            }
            if let Some(task) = state.high.pop_front()
                .or_else(|| state.medium.pop_front())
                .or_else(|| state.low.pop_front())
            {
                return Some(task);
            }
            state = self.cv.wait(state).unwrap();
        }
    }

    pub(crate) fn increment_completed(&self) {
        self.completed.fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn increment_failed(&self) {
        self.failed.fetch_add(1, Ordering::Relaxed);
    }
}
