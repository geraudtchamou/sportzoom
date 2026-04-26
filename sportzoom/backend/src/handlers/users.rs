use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::config::AppState;
use crate::db::models::{UserPublic, UpdateProfileRequest};
use crate::middleware::auth::Claims;
use crate::utils::errors::{AppError, Result};
use std::sync::Arc;

#[axum::debug_handler]
pub async fn get_user_profile(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<UserPublic>> {
    let user = sqlx::query_as::<_, UserPublic>(
        "SELECT id, username, display_name, avatar_url, bio, country, points, level FROM users WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::not_found("User not found"))?;

    Ok(Json(user))
}

#[axum::debug_handler]
pub async fn get_current_user_profile(
    State(state): State<Arc<AppState>>,
    extensions: axum::extract::Extension<Claims>,
) -> Result<Json<UserPublic>> {
    let user = sqlx::query_as::<_, UserPublic>(
        "SELECT id, username, display_name, avatar_url, bio, country, points, level FROM users WHERE id = $1"
    )
    .bind(extensions.sub)
    .fetch_one(&state.db)
    .await?;

    Ok(Json(user))
}

#[axum::debug_handler]
pub async fn update_profile(
    State(state): State<Arc<AppState>>,
    extensions: axum::extract::Extension<Claims>,
    Json(payload): Json<UpdateProfileRequest>,
) -> Result<Json<UserPublic>> {
    let user = sqlx::query_as::<_, UserPublic>(
        r#"
        UPDATE users 
        SET display_name = COALESCE($1, display_name),
            bio = COALESCE($2, bio),
            country = COALESCE($3, country),
            updated_at = NOW()
        WHERE id = $4
        RETURNING id, username, display_name, avatar_url, bio, country, points, level
        "#
    )
    .bind(payload.display_name)
    .bind(payload.bio)
    .bind(payload.country)
    .bind(extensions.sub)
    .fetch_one(&state.db)
    .await?;

    Ok(Json(user))
}
