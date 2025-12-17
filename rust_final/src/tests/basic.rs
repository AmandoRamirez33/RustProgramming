use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;

use rust_final::{WorkQueue, Task, Priority};
use rust_final::task::payload::{TaskPayload, TaskResult, TaskError};
use rust_final::task::task::TaskMetadata;

struct TestTask {
    ran: AtomicBool,
}

impl TaskPayload for TestTask {
    fn execute(&self) -> Result<TaskResult, TaskError> {
        self.ran.store(true, Ordering::Relaxed);
        Ok(TaskResult)
    }

    fn cancel(&self) -> Result<(), TaskError> {
        Ok(())
    }
}

#[test]
fn task_executes() {
    let queue = WorkQueue::new();
    queue.start_workers(1);

    queue.submit(Task {
        id: 1,
        priority: Priority::High,
        payload: Box::new(TestTask {
            ran: AtomicBool::new(false),
        }),
        metadata: TaskMetadata {
            submitted_at: Instant::now(),
            attempts: 0,
        },
        cancelled: AtomicBool::new(false),
    });

    std::thread::sleep(std::time::Duration::from_millis(100));
    queue.shutdown();

    let (_, completed, _) = queue.stats();
    assert_eq!(completed, 1);
}
