use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::AppState;
use crate::db::models::Event;

/// Create event request
#[derive(Debug, Deserialize)]
pub struct CreateEventRequest {
    pub title: String,
    pub description: Option<String>,
    pub event_type: String,
    pub category: String,
    pub location: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Serialize)]
pub struct EventResponse {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub event_type: String,
    pub category: String,
    pub location: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    pub organizer_id: Uuid,
    pub attendee_count: i32,
    pub is_live: bool,
    pub status: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<Event> for EventResponse {
    fn from(event: Event) -> Self {
        Self {
            id: event.id,
            title: event.title,
            description: event.description,
            event_type: event.event_type,
            category: event.category,
            location: event.location,
            latitude: event.latitude,
            longitude: event.longitude,
            start_time: event.start_time,
            end_time: event.end_time,
            organizer_id: event.organizer_id,
            attendee_count: event.attendee_count,
            is_live: event.is_live,
            status: event.status,
            created_at: event.created_at,
        }
    }
}

/// List events with filters
pub async fn list_events(
    State(state): State<AppState>,
) -> Result<Json<Vec<EventResponse>>, StatusCode> {
    let events = sqlx::query_as::<_, Event>(
        "SELECT * FROM events WHERE status = 'upcoming' OR status = 'live' ORDER BY start_time ASC LIMIT 100",
    )
    .fetch_all(&state.db.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response: Vec<EventResponse> = events.into_iter().map(EventResponse::from).collect();

    Ok(Json(response))
}

/// Create a new event
pub async fn create_event(
    State(state): State<AppState>,
    Json(payload): Json<CreateEventRequest>,
) -> Result<Json<EventResponse>, StatusCode> {
    // In real implementation, get organizer_id from JWT claims
    let organizer_id = Uuid::new_v4(); // Placeholder

    let event = sqlx::query_as::<_, Event>(
        r#"
        INSERT INTO events (title, description, event_type, category, location, latitude, longitude, start_time, end_time, organizer_id)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        RETURNING *
        "#,
    )
    .bind(&payload.title)
    .bind(&payload.description)
    .bind(&payload.event_type)
    .bind(&payload.category)
    .bind(&payload.location)
    .bind(payload.latitude)
    .bind(payload.longitude)
    .bind(payload.start_time)
    .bind(payload.end_time)
    .bind(organizer_id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create event: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(EventResponse::from(event)))
}

/// Get a specific event
pub async fn get_event(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<EventResponse>, StatusCode> {
    let event = sqlx::query_as::<_, Event>("SELECT * FROM events WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.db.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(EventResponse::from(event)))
}

/// Join an event (mark as attending/interested)
pub async fn join_event(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    // In real implementation, get user_id from JWT claims
    let user_id = Uuid::new_v4(); // Placeholder

    sqlx::query(
        r#"
        INSERT INTO event_attendees (user_id, event_id, status)
        VALUES ($1, $2, 'attending')
        ON CONFLICT (user_id, event_id) DO UPDATE SET status = 'attending'
        "#,
    )
    .bind(user_id)
    .bind(id)
    .execute(&state.db.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Increment attendee count
    sqlx::query("UPDATE events SET attendee_count = attendee_count + 1 WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}

/// Get event room (for real-time chat)
pub async fn get_event_room(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Verify event exists
    let _event = sqlx::query_as::<_, Event>("SELECT * FROM events WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.db.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    // Return room info (WebSocket connection details would be handled separately)
    Ok(Json(json!({
        "event_id": id,
        "room_id": format!("event-{}", id),
        "max_participants": 10000,
        "features": ["chat", "reactions", "predictions"]
    })))
}
