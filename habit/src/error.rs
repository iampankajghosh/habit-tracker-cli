use thiserror::Error;

#[derive(Debug, Error)]
pub enum HabitError {
    #[error("habit not found: {0}")]
    NotFound(String),
    #[error("invalid habit name: {0}")]
    InvalidName(String),
    #[error("habit already completed for date: {0}")]
    AlreadyCompleted(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, HabitError>;

