#[derive(Debug)]
pub struct TaskResult;

#[derive(Debug)]
pub struct TaskError;

pub trait TaskPayload: Send + 'static {
    fn execute(&self) -> Result<TaskResult, TaskError>;
    fn cancel(&self) -> Result<(), TaskError>;
}
