use actix_web::{ResponseError, http::StatusCode};
use redis::RedisError;
use serde_json;
use thiserror::Error;
use tokio_tungstenite;

#[derive(Error, Debug)]
pub enum HermesError {
    #[error("RedisError: {0}")]
    Redis(#[from] RedisError),

    #[error("IoError: {0}")]
    IO(#[from] std::io::Error),

    #[error("JsonError: {0}")]
    Json(#[from] serde_json::Error),

    #[error("ConfigError: {0}")]
    Config(#[from] toml::de::Error),

    #[error("WebSocket error: {0}")]
    WebSocket(#[from] tokio_tungstenite::tungstenite::Error),

    #[error("Invalid database operation: {op}")]
    InvalidOperation { op: String },

    #[error("Network error: {0}")]
    Network(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Redis connection pool error: {0}")]
    RedisPool(#[from] bb8::RunError<RedisError>),

    #[error("Key Not Found: {0}")]
    KeyNotFound(String),
}

impl ResponseError for HermesError {
    fn status_code(&self) -> StatusCode {
        match self {
            HermesError::InvalidOperation { .. } => StatusCode::BAD_REQUEST,
            HermesError::Json(_) => StatusCode::BAD_REQUEST,
            HermesError::Config(_) => StatusCode::INTERNAL_SERVER_ERROR,
            HermesError::Redis(_) => StatusCode::INTERNAL_SERVER_ERROR,
            HermesError::IO(_) => StatusCode::INTERNAL_SERVER_ERROR,
            HermesError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            HermesError::WebSocket(_) => StatusCode::BAD_GATEWAY,
            HermesError::Network(_) => StatusCode::SERVICE_UNAVAILABLE,
            HermesError::RedisPool(_) => StatusCode::INTERNAL_SERVER_ERROR,
            &HermesError::KeyNotFound(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
