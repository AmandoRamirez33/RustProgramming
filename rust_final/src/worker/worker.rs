use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::panic::AssertUnwindSafe;

use crate::queue::work_queue::WorkQueue;

pub fn worker_loop(queue: Arc<WorkQueue>) {
    loop {
        let task = match queue.wait_for_task() {
            Some(task) => task,
            None => return,
        };

        if task.cancelled.load(Ordering::Relaxed) {
            let _ = task.payload.cancel();
            continue;
        }

        let result = std::panic::catch_unwind(
            AssertUnwindSafe(|| task.payload.execute())
        );

        match result {
            Ok(Ok(_)) => queue.increment_completed(),
            Ok(Err(_)) | Err(_) => queue.increment_failed(),
        }
    }
}
