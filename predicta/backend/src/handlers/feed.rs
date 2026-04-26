use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::AppState;
use crate::db::models::Post;

/// Feed query parameters
#[derive(Debug, Deserialize)]
pub struct FeedQuery {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub event_id: Option<Uuid>,
}

#[derive(Debug, Serialize)]
pub struct PostResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub username: String,
    pub user_avatar: Option<String>,
    pub event_id: Option<Uuid>,
    pub content_type: String,
    pub media_url: String,
    pub thumbnail_url: Option<String>,
    pub caption: Option<String>,
    pub duration_seconds: Option<i32>,
    pub like_count: i32,
    pub comment_count: i32,
    pub share_count: i32,
    pub view_count: i32,
    pub is_liked: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Get personalized feed (AI-recommended)
pub async fn get_feed(
    State(state): State<AppState>,
    Query(params): Query<FeedQuery>,
) -> Result<Json<Vec<PostResponse>>, StatusCode> {
    let limit = params.limit.unwrap_or(20);
    let offset = params.offset.unwrap_or(0);

    // AI-powered feed ranking algorithm:
    // Score = (likes × 3) + (comments × 5) + (watch_time × 2)
    // This is a simplified version - in production, use ML models
    
    let posts = sqlx::query_as::<_, Post>(
        r#"
        SELECT * FROM posts 
        WHERE TRUE
        ORDER BY ai_score DESC, created_at DESC
        LIMIT $1 OFFSET $2
        "#,
    )
    .bind(limit as i64)
    .bind(offset as i64)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // In real implementation, join with users table to get username/avatar
    let response: Vec<PostResponse> = posts
        .into_iter()
        .map(|p| PostResponse {
            id: p.id,
            user_id: p.user_id,
            username: "user".to_string(), // Placeholder - join with users table
            user_avatar: None,
            event_id: p.event_id,
            content_type: p.content_type,
            media_url: p.media_url,
            thumbnail_url: p.thumbnail_url,
            caption: p.caption,
            duration_seconds: p.duration_seconds,
            like_count: p.like_count,
            comment_count: p.comment_count,
            share_count: p.share_count,
            view_count: p.view_count,
            is_liked: false, // Would check against current user
            created_at: p.created_at,
        })
        .collect();

    Ok(Json(response))
}

/// Get trending posts (last 24 hours)
pub async fn get_trending(
    State(state): State<AppState>,
    Query(params): Query<FeedQuery>,
) -> Result<Json<Vec<PostResponse>>, StatusCode> {
    let limit = params.limit.unwrap_or(20);

    // Get trending posts from last 24 hours
    let posts = sqlx::query_as::<_, Post>(
        r#"
        SELECT * FROM posts 
        WHERE is_trending = TRUE 
          AND created_at > NOW() - INTERVAL '24 hours'
        ORDER BY ai_score DESC, view_count DESC
        LIMIT $1
        "#,
    )
    .bind(limit as i64)
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
            event_id: p.event_id,
            content_type: p.content_type,
            media_url: p.media_url,
            thumbnail_url: p.thumbnail_url,
            caption: p.caption,
            duration_seconds: p.duration_seconds,
            like_count: p.like_count,
            comment_count: p.comment_count,
            share_count: p.share_count,
            view_count: p.view_count,
            is_liked: false,
            created_at: p.created_at,
        })
        .collect();

    Ok(Json(response))
}

/// Get AI recommendations for user
pub async fn get_recommendations(
    State(state): State<AppState>,
) -> Result<Json<Vec<PostResponse>>, StatusCode> {
    // In production, this would use collaborative filtering and behavior-based learning
    // For now, return high-scoring posts
    
    let posts = sqlx::query_as::<_, Post>(
        r#"
        SELECT * FROM posts 
        WHERE ai_score > 50.0
        ORDER BY ai_score DESC
        LIMIT 20
        "#,
    )
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
            event_id: p.event_id,
            content_type: p.content_type,
            media_url: p.media_url,
            thumbnail_url: p.thumbnail_url,
            caption: p.caption,
            duration_seconds: p.duration_seconds,
            like_count: p.like_count,
            comment_count: p.comment_count,
            share_count: p.share_count,
            view_count: p.view_count,
            is_liked: false,
            created_at: p.created_at,
        })
        .collect();

    Ok(Json(response))
}
