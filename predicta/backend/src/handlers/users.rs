use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::AppState;
use crate::db::models::User;

#[derive(Debug, Serialize)]
pub struct UserProfileResponse {
    pub id: Uuid,
    pub username: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub country: Option<String>,
    pub points: i64,
    pub level: i32,
    pub is_verified: bool,
    pub follower_count: i32,
    pub following_count: i32,
    pub post_count: i32,
}

/// Get user by ID
pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<UserProfileResponse>, StatusCode> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1 AND is_banned = FALSE")
        .bind(id)
        .fetch_optional(&state.db.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    // Get counts
    let post_count: i32 = sqlx::query_scalar("SELECT COUNT(*) FROM posts WHERE user_id = $1")
        .bind(id)
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or(0);

    Ok(Json(UserProfileResponse {
        id: user.id,
        username: user.username,
        display_name: user.display_name,
        avatar_url: user.avatar_url,
        bio: user.bio,
        country: user.country,
        points: user.points,
        level: user.level,
        is_verified: user.is_verified,
        follower_count: 0, // Implement followers system
        following_count: 0,
        post_count,
    }))
}

/// Get current user profile
pub async fn get_profile(
    State(state): State<AppState>,
) -> Result<Json<UserProfileResponse>, StatusCode> {
    // In production, get user_id from JWT claims
    let user_id = Uuid::new_v4(); // Placeholder

    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1 AND is_banned = FALSE")
        .bind(user_id)
        .fetch_optional(&state.db.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let post_count: i32 = sqlx::query_scalar("SELECT COUNT(*) FROM posts WHERE user_id = $1")
        .bind(user_id)
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or(0);

    Ok(Json(UserProfileResponse {
        id: user.id,
        username: user.username,
        display_name: user.display_name,
        avatar_url: user.avatar_url,
        bio: user.bio,
        country: user.country,
        points: user.points,
        level: user.level,
        is_verified: user.is_verified,
        follower_count: 0,
        following_count: 0,
        post_count,
    }))
}

#[derive(Debug, Deserialize)]
pub struct UpdateProfileRequest {
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub country: Option<String>,
}

/// Update user profile
pub async fn update_profile(
    State(state): State<AppState>,
    Json(payload): Json<UpdateProfileRequest>,
) -> Result<Json<UserProfileResponse>, StatusCode> {
    // In production, get user_id from JWT claims
    let user_id = Uuid::new_v4(); // Placeholder

    let user = sqlx::query_as::<_, User>(
        r#"
        UPDATE users 
        SET 
            display_name = COALESCE($1, display_name),
            bio = COALESCE($2, bio),
            country = COALESCE($3, country),
            updated_at = NOW()
        WHERE id = $4
        RETURNING *
        "#,
    )
    .bind(payload.display_name)
    .bind(payload.bio)
    .bind(payload.country)
    .bind(user_id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let post_count: i32 = sqlx::query_scalar("SELECT COUNT(*) FROM posts WHERE user_id = $1")
        .bind(user_id)
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or(0);

    Ok(Json(UserProfileResponse {
        id: user.id,
        username: user.username,
        display_name: user.display_name,
        avatar_url: user.avatar_url,
        bio: user.bio,
        country: user.country,
        points: user.points,
        level: user.level,
        is_verified: user.is_verified,
        follower_count: 0,
        following_count: 0,
        post_count,
    }))
}
