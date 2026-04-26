use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
};
use serde::Serialize;

use crate::AppState;
use crate::db::models::LiveStream;

#[derive(Debug, Serialize)]
pub struct LiveStreamResponse {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub title: String,
    pub description: Option<String>,
    pub stream_url: String,
    pub viewer_count: i32,
    pub like_count: i32,
    pub is_active: bool,
    pub started_at: chrono::DateTime<chrono::Utc>,
}

/// List active live streams
pub async fn list_live_streams(
    State(state): State<AppState>,
) -> Result<Json<Vec<LiveStreamResponse>>, StatusCode> {
    let streams = sqlx::query_as::<_, LiveStream>(
        "SELECT * FROM live_streams WHERE is_active = TRUE ORDER BY viewer_count DESC LIMIT 50",
    )
    .fetch_all(&state.db.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response: Vec<LiveStreamResponse> = streams
        .into_iter()
        .map(|s| LiveStreamResponse {
            id: s.id,
            user_id: s.user_id,
            title: s.title,
            description: s.description,
            stream_url: s.stream_url,
            viewer_count: s.viewer_count,
            like_count: s.like_count,
            is_active: s.is_active,
            started_at: s.started_at,
        })
        .collect();

    Ok(Json(response))
}

/// Start a new live stream
pub async fn start_stream(
    State(state): State<AppState>,
) -> Result<Json<LiveStreamResponse>, StatusCode> {
    // In production, integrate with Agora/LiveKit for actual streaming
    // This is a simplified implementation
    
    let user_id = uuid::Uuid::new_v4(); // Get from JWT in production
    
    let stream = sqlx::query_as::<_, LiveStream>(
        r#"
        INSERT INTO live_streams (user_id, title, stream_url)
        VALUES ($1, $2, $3)
        RETURNING *
        "#,
    )
    .bind(user_id)
    .bind("Live Stream")
    .bind("rtmp://stream.example.com/live")
    .fetch_one(&state.db.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(LiveStreamResponse {
        id: stream.id,
        user_id: stream.user_id,
        title: stream.title,
        description: stream.description,
        stream_url: stream.stream_url,
        viewer_count: stream.viewer_count,
        like_count: stream.like_count,
        is_active: stream.is_active,
        started_at: stream.started_at,
    }))
}

/// End a live stream
pub async fn end_stream(
    State(state): State<AppState>,
) -> Result<StatusCode, StatusCode> {
    let stream_id = uuid::Uuid::new_v4(); // Get from request body in production

    sqlx::query("UPDATE live_streams SET is_active = FALSE, ended_at = NOW() WHERE id = $1")
        .bind(stream_id)
        .execute(&state.db.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}

/// Get a specific live stream
pub async fn get_stream(
    State(state): State<AppState>,
) -> Result<Json<LiveStreamResponse>, StatusCode> {
    let stream_id = uuid::Uuid::new_v4(); // Get from path params in production

    let stream = sqlx::query_as::<_, LiveStream>("SELECT * FROM live_streams WHERE id = $1")
        .bind(stream_id)
        .fetch_optional(&state.db.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(LiveStreamResponse {
        id: stream.id,
        user_id: stream.user_id,
        title: stream.title,
        description: stream.description,
        stream_url: stream.stream_url,
        viewer_count: stream.viewer_count,
        like_count: stream.like_count,
        is_active: stream.is_active,
        started_at: stream.started_at,
    }))
}
