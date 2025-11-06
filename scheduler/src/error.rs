use thiserror::Error;

#[derive(Debug, Error)]
pub enum ScheduleError {
    #[error("{0}")]
    Channel(String)
}