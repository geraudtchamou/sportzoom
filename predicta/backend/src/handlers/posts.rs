use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::AppState;
use crate::db::models::Post;

/// Create post request
#[derive(Debug, Deserialize)]
pub struct CreatePostRequest {
    pub content_type: String,
    pub media_url: String,
    pub thumbnail_url: Option<String>,
    pub caption: Option<String>,
    pub duration_seconds: Option<i32>,
    pub event_id: Option<Uuid>,
}

#[derive(Debug, Serialize)]
pub struct PostResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub username: String,
    pub user_avatar: Option<String>,
    pub content_type: String,
    pub media_url: String,
    pub thumbnail_url: Option<String>,
    pub caption: Option<String>,
    pub duration_seconds: Option<i32>,
    pub like_count: i32,
    pub comment_count: i32,
    pub share_count: i32,
    pub view_count: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// List posts
pub async fn list_posts(
    State(state): State<AppState>,
) -> Result<Json<Vec<PostResponse>>, StatusCode> {
    let posts = sqlx::query_as::<_, Post>("SELECT * FROM posts ORDER BY created_at DESC LIMIT 50")
        .fetch_all(&state.db.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response: Vec<PostResponse> = posts
        .into_iter()
        .map(|p| PostResponse {
            id: p.id,
            user_id: p.user_id,
            username: "user".to_string(),
            user_avatar: None,
            content_type: p.content_type,
            media_url: p.media_url,
            thumbnail_url: p.thumbnail_url,
            caption: p.caption,
            duration_seconds: p.duration_seconds,
            like_count: p.like_count,
            comment_count: p.comment_count,
            share_count: p.share_count,
            view_count: p.view_count,
            created_at: p.created_at,
        })
        .collect();

    Ok(Json(response))
}

/// Create a new post
pub async fn create_post(
    State(state): State<AppState>,
    Json(payload): Json<CreatePostRequest>,
) -> Result<Json<PostResponse>, StatusCode> {
    // In real implementation, get user_id from JWT claims
    let user_id = Uuid::new_v4(); // Placeholder

    // Calculate initial AI score (will be updated based on engagement)
    let ai_score = 0.0;

    let post = sqlx::query_as::<_, Post>(
        r#"
        INSERT INTO posts (user_id, event_id, content_type, media_url, thumbnail_url, caption, duration_seconds, ai_score)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING *
        "#,
    )
    .bind(user_id)
    .bind(payload.event_id)
    .bind(&payload.content_type)
    .bind(&payload.media_url)
    .bind(&payload.thumbnail_url)
    .bind(&payload.caption)
    .bind(payload.duration_seconds)
    .bind(ai_score)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create post: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(PostResponse {
        id: post.id,
        user_id: post.user_id,
        username: "user".to_string(),
        user_avatar: None,
        content_type: post.content_type,
        media_url: post.media_url,
        thumbnail_url: post.thumbnail_url,
        caption: post.caption,
        duration_seconds: post.duration_seconds,
        like_count: post.like_count,
        comment_count: post.comment_count,
        share_count: post.share_count,
        view_count: post.view_count,
        created_at: post.created_at,
    }))
}

/// Get a specific post
pub async fn get_post(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<PostResponse>, StatusCode> {
    let post = sqlx::query_as::<_, Post>("SELECT * FROM posts WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.db.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(PostResponse {
        id: post.id,
        user_id: post.user_id,
        username: "user".to_string(),
        user_avatar: None,
        content_type: post.content_type,
        media_url: post.media_url,
        thumbnail_url: post.thumbnail_url,
        caption: post.caption,
        duration_seconds: post.duration_seconds,
        like_count: post.like_count,
        comment_count: post.comment_count,
        share_count: post.share_count,
        view_count: post.view_count,
        created_at: post.created_at,
    }))
}

/// Like a post
pub async fn like_post(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    // In real implementation, get user_id from JWT claims
    let user_id = Uuid::new_v4(); // Placeholder

    // Add like
    sqlx::query(
        "INSERT INTO likes (user_id, post_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
    )
    .bind(user_id)
    .bind(id)
    .execute(&state.db.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Increment like count
    sqlx::query("UPDATE posts SET like_count = like_count + 1 WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Update AI score: Score = (likes × 3) + (comments × 5) + (watch_time × 2)
    sqlx::query(
        "UPDATE posts SET ai_score = (like_count * 3.0) + (comment_count * 5.0) + (total_watch_time_ms * 0.002) WHERE id = $1",
    )
    .bind(id)
    .execute(&state.db.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}

/// Add comment to post
pub async fn add_comment(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // In real implementation, get user_id from JWT claims
    let user_id = Uuid::new_v4(); // Placeholder
    
    let content = payload["content"].as_str().ok_or(StatusCode::BAD_REQUEST)?;

    let comment = sqlx::query(
        "INSERT INTO comments (post_id, user_id, content) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(id)
    .bind(user_id)
    .bind(content)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Increment comment count
    sqlx::query("UPDATE posts SET comment_count = comment_count + 1 WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({
        "comment": comment,
        "message": "Comment added successfully"
    })))
}
