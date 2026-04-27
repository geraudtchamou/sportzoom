use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
};
use serde::Serialize;

use crate::AppState;

#[derive(Debug, Serialize)]
pub struct UserAdminResponse {
    pub id: uuid::Uuid,
    pub username: String,
    pub email: String,
    pub is_verified: bool,
    pub is_banned: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// List all users (admin only)
pub async fn list_users(
    State(state): State<AppState>,
) -> Result<Json<Vec<UserAdminResponse>>, StatusCode> {
    let users = sqlx::query!(
        "SELECT id, username, email, is_verified, is_banned, created_at FROM users ORDER BY created_at DESC LIMIT 100",
    )
    .fetch_all(&state.db.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response: Vec<UserAdminResponse> = users
        .into_iter()
        .map(|u| UserAdminResponse {
            id: u.id,
            username: u.username,
            email: u.email,
            is_verified: u.is_verified,
            is_banned: u.is_banned,
            created_at: u.created_at,
        })
        .collect();

    Ok(Json(response))
}

/// Moderate content (admin only)
pub async fn moderate_content(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Get reported content
    let reported_posts = sqlx::query!(
        "SELECT * FROM posts WHERE is_trending = FALSE ORDER BY created_at DESC LIMIT 50",
    )
    .fetch_all(&state.db.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({
        "reported_content": reported_posts,
        "count": reported_posts.len()
    })))
}

/// Get analytics dashboard data (admin only)
pub async fn get_analytics(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Get user count
    let total_users: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or(0);

    let active_users: i64 = sqlx::query_scalar(
        "SELECT COUNT(DISTINCT user_id) FROM posts WHERE created_at > NOW() - INTERVAL '7 days'",
    )
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or(0);

    // Get post count
    let total_posts: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM posts")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or(0);

    // Get event count
    let total_events: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM events")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or(0);

    // Get live streams count
    let active_streams: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM live_streams WHERE is_active = TRUE")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or(0);

    // Get prediction count
    let total_predictions: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM predictions")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or(0);

    Ok(Json(json!({
        "users": {
            "total": total_users,
            "active_last_7_days": active_users
        },
        "content": {
            "total_posts": total_posts,
            "total_events": total_events,
            "active_live_streams": active_streams
        },
        "engagement": {
            "total_predictions": total_predictions
        },
        "timestamp": chrono::Utc::now()
    })))
}
