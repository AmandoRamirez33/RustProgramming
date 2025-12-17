use std::sync::atomic::AtomicBool;
use std::time::Instant;

pub type TaskId = u64;

#[derive(Clone, Copy, Debug)]
pub enum Priority {
    High,
    Medium,
    Low,
}

pub struct TaskResult;
pub struct TaskError;

pub trait TaskPayload: Send + 'static {
    fn execute(&self) -> Result<TaskResult, TaskError>;
    fn cancel(&self) -> Result<(), TaskError>;
}

pub struct TaskMetadata {
    pub submitted_at: Instant,
    pub attempts: usize,
}

pub struct Task {
    pub id: TaskId,
    pub priority: Priority,
    pub payload: Box<dyn TaskPayload>,
    pub metadata: TaskMetadata,
    pub cancelled: AtomicBool,
}
