use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;
use validator::Validate;

use crate::config::AppState;
use crate::db::models::{StartLiveStreamRequest, LiveStream, PaginationParams};
use crate::middleware::auth::Claims;
use crate::utils::errors::{AppError, Result};
use std::sync::Arc;

#[axum::debug_handler]
pub async fn list_live_streams(
    State(state): State<Arc<AppState>>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<Vec<LiveStream>>> {
    let limit = pagination.limit.unwrap_or(20) as i64;
    let offset = ((pagination.page.unwrap_or(1) - 1) * pagination.limit.unwrap_or(20)) as i64;

    let streams = sqlx::query_as::<_, LiveStream>(
        "SELECT * FROM live_streams WHERE is_live = true ORDER BY viewer_count DESC LIMIT $1 OFFSET $2"
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(streams))
}

#[axum::debug_handler]
pub async fn start_stream(
    State(state): State<Arc<AppState>>,
    extensions: axum::extract::Extension<Claims>,
    Json(payload): Json<StartLiveStreamRequest>,
) -> Result<(StatusCode, Json<LiveStream>)> {
    payload.validate()?;

    let stream_id = Uuid::new_v4();
    
    // In production, integrate with Agora/LiveKit/WebRTC for actual streaming URL
    let stream_url = Some(format!("rtmp://live.sportzoom.app/live/{}", stream_id));

    let stream = sqlx::query_as::<_, LiveStream>(
        r#"
        INSERT INTO live_streams (id, user_id, event_id, title, description, stream_url, is_live, viewer_count)
        VALUES ($1, $2, $3, $4, $5, $6, true, 0)
        RETURNING *
        "#
    )
    .bind(stream_id)
    .bind(extensions.sub)
    .bind(payload.event_id)
    .bind(&payload.title)
    .bind(&payload.description)
    .bind(&stream_url)
    .fetch_one(&state.db)
    .await?;

    Ok((StatusCode::CREATED, Json(stream)))
}

#[axum::debug_handler]
pub async fn end_stream(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    extensions: axum::extract::Extension<Claims>,
) -> Result<Json<LiveStream>> {
    let stream = sqlx::query_as::<_, LiveStream>(
        r#"
        UPDATE live_streams 
        SET is_live = false, ended_at = NOW()
        WHERE id = $1 AND user_id = $2
        RETURNING *
        "#
    )
    .bind(id)
    .bind(extensions.sub)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::not_found("Stream not found or not authorized"))?;

    Ok(Json(stream))
}

#[axum::debug_handler]
pub async fn get_stream(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<LiveStream>> {
    let stream = sqlx::query_as::<_, LiveStream>("SELECT * FROM live_streams WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.db)
        .await?
        .ok_or_else(|| AppError::not_found("Stream not found"))?;

    Ok(Json(stream))
}

#[axum::debug_handler]
pub async fn increment_viewer_count(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode> {
    sqlx::query("UPDATE live_streams SET viewer_count = viewer_count + 1 WHERE id = $1 AND is_live = true")
        .bind(id)
        .execute(&state.db)
        .await?;

    Ok(StatusCode::OK)
}
