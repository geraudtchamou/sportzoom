use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;
use validator::Validate;

use crate::config::AppState;
use crate::db::models::{CreateEventRequest, JoinEventRequest, Event, EventAttendee, AttendeeStatus, PaginationParams};
use crate::middleware::auth::Claims;
use crate::utils::errors::{AppError, Result};
use std::sync::Arc;

#[axum::debug_handler]
pub async fn list_events(
    State(state): State<Arc<AppState>>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<Vec<Event>>> {
    let limit = pagination.limit.unwrap_or(20) as i64;
    let offset = ((pagination.page.unwrap_or(1) - 1) * pagination.limit.unwrap_or(20)) as i64;

    let events = sqlx::query_as::<_, Event>(
        "SELECT * FROM events ORDER BY start_time DESC LIMIT $1 OFFSET $2"
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(events))
}

#[axum::debug_handler]
pub async fn create_event(
    State(state): State<Arc<AppState>>,
    extensions: axum::extract::Extension<Claims>,
    Json(payload): Json<CreateEventRequest>,
) -> Result<(StatusCode, Json<Event>)> {
    payload.validate()?;

    let event_id = Uuid::new_v4();
    
    let event = sqlx::query_as::<_, Event>(
        r#"
        INSERT INTO events (id, title, description, event_type, category, start_time, end_time, location, latitude, longitude, thumbnail_url, status, attendee_count, created_by)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, 'scheduled', 0, $12)
        RETURNING *
        "#
    )
    .bind(event_id)
    .bind(&payload.title)
    .bind(&payload.description)
    .bind(&payload.event_type)
    .bind(&payload.category)
    .bind(payload.start_time)
    .bind(payload.end_time)
    .bind(&payload.location)
    .bind(payload.latitude)
    .bind(payload.longitude)
    .bind(&payload.thumbnail_url)
    .bind(extensions.sub)
    .fetch_one(&state.db)
    .await?;

    Ok((StatusCode::CREATED, Json(event)))
}

#[axum::debug_handler]
pub async fn get_event(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Event>> {
    let event = sqlx::query_as::<_, Event>("SELECT * FROM events WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.db)
        .await?
        .ok_or_else(|| AppError::not_found("Event not found"))?;

    Ok(Json(event))
}

#[axum::debug_handler]
pub async fn join_event(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    extensions: axum::extract::Extension<Claims>,
    Json(payload): Json<JoinEventRequest>,
) -> Result<Json<EventAttendee>> {
    // Check if already joined
    let existing = sqlx::query_as::<_, EventAttendee>(
        "SELECT * FROM event_attendees WHERE event_id = $1 AND user_id = $2"
    )
    .bind(id)
    .bind(extensions.sub)
    .fetch_optional(&state.db)
    .await?;

    let attendee = if let Some(mut att) = existing {
        att.status = payload.status;
        sqlx::query_as::<_, EventAttendee>(
            "UPDATE event_attendees SET status = $1 WHERE event_id = $2 AND user_id = $3 RETURNING *"
        )
        .bind(&payload.status)
        .bind(id)
        .bind(extensions.sub)
        .fetch_one(&state.db)
        .await?
    } else {
        sqlx::query_as::<_, EventAttendee>(
            "INSERT INTO event_attendees (event_id, user_id, status) VALUES ($1, $2, $3) RETURNING *"
        )
        .bind(id)
        .bind(extensions.sub)
        .bind(&payload.status)
        .fetch_one(&state.db)
        .await?
    };

    // Update attendee count
    sqlx::query("UPDATE events SET attendee_count = (SELECT COUNT(*) FROM event_attendees WHERE event_id = $1) WHERE id = $2")
        .bind(id)
        .execute(&state.db)
        .await?;

    Ok(Json(attendee))
}

#[axum::debug_handler]
pub async fn get_event_attendees(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<EventAttendee>>> {
    let attendees = sqlx::query_as::<_, EventAttendee>(
        "SELECT ea.*, u.username, u.display_name, u.avatar_url FROM event_attendees ea JOIN users u ON ea.user_id = u.id WHERE event_id = $1"
    )
    .bind(id)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(attendees))
}
