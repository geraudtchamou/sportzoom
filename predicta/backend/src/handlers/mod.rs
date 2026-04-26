pub mod auth;
pub mod events;
pub mod feed;
pub mod gamification;
pub mod live;
pub mod posts;
pub mod predictions;
pub mod users;
pub mod admin;

use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
};
use serde_json::json;

use crate::AppState;

/// Health check endpoint
pub async fn health() -> Json<serde_json::Value> {
    Json(json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now()
    }))
}
