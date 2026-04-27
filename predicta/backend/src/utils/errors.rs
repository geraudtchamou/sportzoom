use thiserror::Error;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

/// Application errors
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),

    #[error("JWT error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match &self {
            AppError::Database(e) => {
                tracing::error!("Database error: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error occurred")
            }
            AppError::Auth(e) => {
                tracing::warn!("Authentication error: {}", e);
                (StatusCode::UNAUTHORIZED, e.as_str())
            }
            AppError::Validation(e) => {
                tracing::warn!("Validation error: {}", e);
                (StatusCode::BAD_REQUEST, e.as_str())
            }
            AppError::NotFound(e) => {
                tracing::info!("Not found: {}", e);
                (StatusCode::NOT_FOUND, e.as_str())
            }
            AppError::BadRequest(e) => {
                tracing::warn!("Bad request: {}", e);
                (StatusCode::BAD_REQUEST, e.as_str())
            }
            AppError::Internal(e) => {
                tracing::error!("Internal error: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "An internal error occurred")
            }
            AppError::Redis(e) => {
                tracing::error!("Redis error: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Cache service unavailable")
            }
            AppError::Jwt(e) => {
                tracing::warn!("JWT error: {}", e);
                (StatusCode::UNAUTHORIZED, "Invalid token")
            }
            AppError::Io(e) => {
                tracing::error!("IO error: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "IO error occurred")
            }
        };

        let body = Json(json!({
            "error": {
                "code": status.as_u16(),
                "message": error_message,
                "type": format!("{:?}", self).split('(').next().unwrap_or("Unknown")
            }
        }));

        (status, body).into_response()
    }
}

/// Result type alias
pub type Result<T> = std::result::Result<T, AppError>;
