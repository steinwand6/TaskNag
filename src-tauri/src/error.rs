use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Internal error: {0}")]
    Internal(String),
    
    #[error("Parse error: {0}")]
    ParseError(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl From<AppError> for ErrorResponse {
    fn from(err: AppError) -> Self {
        ErrorResponse {
            error: err.to_string(),
        }
    }
}

// Tauri command error conversion
impl From<AppError> for String {
    fn from(err: AppError) -> Self {
        err.to_string()
    }
}