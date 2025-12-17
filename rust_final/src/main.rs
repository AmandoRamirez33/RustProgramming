use std::sync::atomic::AtomicBool;
use std::time::{Duration, Instant};
use std::thread;

use rust_final::{WorkQueue, Priority, Task};
use rust_final::task::task::{TaskPayload, TaskResult, TaskError, TaskMetadata};

struct DemoTask {
    id: usize,
}

impl TaskPayload for DemoTask {
    fn execute(&self) -> Result<TaskResult, TaskError> {
        println!("[{:?}] Executing task {}", thread::current().id(), self.id);
        thread::sleep(Duration::from_millis(50));
        Ok(TaskResult)
    }

    fn cancel(&self) -> Result<(), TaskError> {
        println!("Task {} cancelled", self.id);
        Ok(())
    }
}

fn main() {
    let queue = WorkQueue::new();
    queue.start_workers(4);

    for i in 0..20 {
        let priority = if i % 2 == 0 { Priority::High } else { Priority::Low };

        queue.submit(Task {
            id: i as u64,
            priority,
            payload: Box::new(DemoTask { id: i }), 
            metadata: TaskMetadata {
                submitted_at: Instant::now(),
                attempts: 0,
            },
            cancelled: AtomicBool::new(false),
        });
    }

    thread::sleep(Duration::from_secs(3));

    let (submitted, completed, failed) = queue.stats();
    println!("Submitted: {}, Completed: {}, Failed: {}", submitted, completed, failed);

    queue.shutdown();
    thread::sleep(Duration::from_millis(500));
}
