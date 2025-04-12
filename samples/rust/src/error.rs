use thiserror::Error;

#[derive(Error, Debug)]
pub enum A2AError {
    #[error("JSON serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("HTTP request error: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("Task not found: {0}")]
    TaskNotFound(String),

    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error("Method not found: {0}")]
    MethodNotFound(String),

    #[error("Internal server error: {0}")]
    InternalServerError(String),

    #[error("Task cannot be canceled: {0}")]
    TaskNotCancelable(String),

    #[error("Unsupported operation: {0}")]
    UnsupportedOperation(String),
}

pub type A2AResult<T> = Result<T, A2AError>;
