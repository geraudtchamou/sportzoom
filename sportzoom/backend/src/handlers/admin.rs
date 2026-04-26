use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::config::AppState;
use crate::db::models::{DashboardStats, User, UserRole, PaginationParams};
use crate::utils::errors::{AppError, Result};
use std::sync::Arc;

#[axum::debug_handler]
pub async fn get_dashboard_stats(
    State(state): State<Arc<AppState>>,
) -> Result<Json<DashboardStats>> {
    let stats = sqlx::query_as::<_, (i64, i64, i64, i64, i64, i64)>(
        r#"
        SELECT 
            (SELECT COUNT(*) FROM users) as total_users,
            (SELECT COUNT(*) FROM users WHERE last_active > NOW() - INTERVAL '24 hours') as active_users_24h,
            (SELECT COUNT(*) FROM posts) as total_posts,
            (SELECT COUNT(*) FROM events) as total_events,
            (SELECT COUNT(*) FROM live_streams WHERE is_live = true) as live_streams,
            (SELECT COUNT(*) FROM predictions) as total_predictions
        "#
    )
    .fetch_one(&state.db)
    .await?;

    Ok(Json(DashboardStats {
        total_users: stats.0,
        active_users_24h: stats.1,
        total_posts: stats.2,
        total_events: stats.3,
        live_streams: stats.4,
        total_predictions: stats.5,
    }))
}

#[axum::debug_handler]
pub async fn list_all_users(
    State(state): State<Arc<AppState>>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<Vec<User>>> {
    let limit = pagination.limit.unwrap_or(20) as i64;
    let offset = ((pagination.page.unwrap_or(1) - 1) * pagination.limit.unwrap_or(20)) as i64;

    let users = sqlx::query_as::<_, User>(
        "SELECT * FROM users ORDER BY created_at DESC LIMIT $1 OFFSET $2"
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(users))
}

#[axum::debug_handler]
pub async fn ban_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode> {
    sqlx::query("UPDATE users SET role = 'user' WHERE id = $1")
        .bind(id)
        .execute(&state.db)
        .await?;

    Ok(StatusCode::OK)
}

#[axum::debug_handler]
pub async fn make_admin(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode> {
    sqlx::query("UPDATE users SET role = 'admin' WHERE id = $1")
        .bind(id)
        .execute(&state.db)
        .await?;

    Ok(StatusCode::OK)
}

#[axum::debug_handler]
pub async fn delete_post(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode> {
    sqlx::query("DELETE FROM posts WHERE id = $1")
        .bind(id)
        .execute(&state.db)
        .await?;

    Ok(StatusCode::OK)
}
