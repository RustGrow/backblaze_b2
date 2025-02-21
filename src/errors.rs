use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct B2ErrorResponse {
    pub status: u32,
    pub code: String,
    pub message: String,
}

impl fmt::Display for B2ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Backblaze B2 API error (status {}): [{}] {}",
            self.status, self.code, self.message
        )
    }
}

#[derive(Error, Debug)]
pub enum B2Error {
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("API error: {0}")]
    ApiError(B2ErrorResponse),

    #[error("Configuration error: {0}")]
    ConfigError(#[from] std::env::VarError),

    #[error("Invalid response: {0}")]
    InvalidResponse(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, B2Error>;
