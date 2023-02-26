use thiserror::Error;

pub type Result<T> = std::result::Result<T, IdunError>;

#[derive(Debug, Error)]
pub enum IdunError {
    // TODO move StateSocket away from lib OR use Config::save_state?
    #[error("Parsing error: {0}")]
    SerdeParse(#[from] serde_json::error::Error),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}
