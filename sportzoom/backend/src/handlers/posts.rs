use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;
use validator::Validate;

use crate::config::AppState;
use crate::db::models::{CreatePostRequest, CreateCommentRequest, Post, Comment, Like, PaginationParams};
use crate::middleware::auth::Claims;
use crate::utils::errors::{AppError, Result};
use std::sync::Arc;

#[axum::debug_handler]
pub async fn list_posts(
    State(state): State<Arc<AppState>>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<Vec<Post>>> {
    let limit = pagination.limit.unwrap_or(20) as i64;
    let offset = ((pagination.page.unwrap_or(1) - 1) * pagination.limit.unwrap_or(20)) as i64;

    let posts = sqlx::query_as::<_, Post>(
        "SELECT * FROM posts ORDER BY created_at DESC LIMIT $1 OFFSET $2"
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(posts))
}

#[axum::debug_handler]
pub async fn create_post(
    State(state): State<Arc<AppState>>,
    extensions: axum::extract::Extension<Claims>,
    Json(payload): Json<CreatePostRequest>,
) -> Result<(StatusCode, Json<Post>)> {
    payload.validate()?;

    let post_id = Uuid::new_v4();
    
    let post = sqlx::query_as::<_, Post>(
        r#"
        INSERT INTO posts (id, user_id, event_id, content_type, media_url, thumbnail_url, caption, duration_seconds)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING *
        "#
    )
    .bind(post_id)
    .bind(extensions.sub)
    .bind(payload.event_id)
    .bind(&payload.content_type)
    .bind(&payload.media_url)
    .bind(&payload.thumbnail_url)
    .bind(&payload.caption)
    .bind(payload.duration_seconds)
    .fetch_one(&state.db)
    .await?;

    Ok((StatusCode::CREATED, Json(post)))
}

#[axum::debug_handler]
pub async fn like_post(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    extensions: axum::extract::Extension<Claims>,
) -> Result<Json<Like>> {
    // Check if already liked
    let existing = sqlx::query_as::<_, Like>(
        "SELECT * FROM likes WHERE post_id = $1 AND user_id = $2"
    )
    .bind(id)
    .bind(extensions.sub)
    .fetch_optional(&state.db)
    .await?;

    if existing.is_some() {
        return Err(AppError::conflict("Already liked"));
    }

    let like_id = Uuid::new_v4();
    let like = sqlx::query_as::<_, Like>(
        "INSERT INTO likes (id, user_id, post_id) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(like_id)
    .bind(extensions.sub)
    .bind(id)
    .fetch_one(&state.db)
    .await?;

    // Update like count
    sqlx::query("UPDATE posts SET likes_count = likes_count + 1 WHERE id = $1")
        .bind(id)
        .execute(&state.db)
        .await?;

    Ok(Json(like))
}

#[axum::debug_handler]
pub async fn unlike_post(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    extensions: axum::extract::Extension<Claims>,
) -> Result<StatusCode> {
    sqlx::query("DELETE FROM likes WHERE post_id = $1 AND user_id = $2")
        .bind(id)
        .bind(extensions.sub)
        .execute(&state.db)
        .await?;

    sqlx::query("UPDATE posts SET likes_count = GREATEST(likes_count - 1, 0) WHERE id = $1")
        .bind(id)
        .execute(&state.db)
        .await?;

    Ok(StatusCode::OK)
}

#[axum::debug_handler]
pub async fn add_comment(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    extensions: axum::extract::Extension<Claims>,
    Json(payload): Json<CreateCommentRequest>,
) -> Result<(StatusCode, Json<Comment>)> {
    payload.validate()?;

    let comment_id = Uuid::new_v4();
    
    let comment = sqlx::query_as::<_, Comment>(
        r#"
        INSERT INTO comments (id, post_id, user_id, content)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#
    )
    .bind(comment_id)
    .bind(id)
    .bind(extensions.sub)
    .bind(&payload.content)
    .fetch_one(&state.db)
    .await?;

    // Update comment count
    sqlx::query("UPDATE posts SET comments_count = comments_count + 1 WHERE id = $1")
        .bind(id)
        .execute(&state.db)
        .await?;

    Ok((StatusCode::CREATED, Json(comment)))
}

#[axum::debug_handler]
pub async fn get_post_comments(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<Vec<Comment>>> {
    let limit = pagination.limit.unwrap_or(20) as i64;
    let offset = ((pagination.page.unwrap_or(1) - 1) * pagination.limit.unwrap_or(20)) as i64;

    let comments = sqlx::query_as::<_, Comment>(
        "SELECT * FROM comments WHERE post_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3"
    )
    .bind(id)
    .bind(limit)
    .bind(offset)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(comments))
}
