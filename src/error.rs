use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConversationError {
    #[error("API error: {0}")]
    ApiError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}