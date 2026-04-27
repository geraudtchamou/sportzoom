use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::AppState;
use crate::db::models::{Event, Prediction};

/// Create prediction request
#[derive(Debug, Deserialize)]
pub struct CreatePredictionRequest {
    pub event_id: Uuid,
    pub prediction_type: String, // match_winner, exact_score, player_event
    pub predicted_outcome: String,
    pub team_a: Option<String>,
    pub team_b: Option<String>,
    pub player_id: Option<Uuid>,
    pub event_type: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PredictionResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub event_id: Uuid,
    pub prediction_type: String,
    pub predicted_outcome: String,
    pub points_awarded: i32,
    pub is_correct: Option<bool>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// List predictions for a user
pub async fn list_predictions(
    State(state): State<AppState>,
) -> Result<Json<Vec<PredictionResponse>>, StatusCode> {
    let predictions = sqlx::query_as::<_, Prediction>(
        "SELECT * FROM predictions ORDER BY created_at DESC LIMIT 50",
    )
    .fetch_all(&state.db.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response: Vec<PredictionResponse> = predictions
        .into_iter()
        .map(|p| PredictionResponse {
            id: p.id,
            user_id: p.user_id,
            event_id: p.event_id,
            prediction_type: serde_json::to_string(&p.prediction_type).unwrap_or_default(),
            predicted_outcome: p.predicted_outcome,
            points_awarded: p.points_awarded,
            is_correct: p.is_correct,
            created_at: p.created_at,
        })
        .collect();

    Ok(Json(response))
}

/// Create a new prediction
pub async fn create_prediction(
    State(state): State<AppState>,
    Json(payload): Json<CreatePredictionRequest>,
) -> Result<Json<PredictionResponse>, StatusCode> {
    // Validate that event exists and is not started
    let event = sqlx::query_as::<_, Event>("SELECT * FROM events WHERE id = $1")
        .bind(payload.event_id)
        .fetch_optional(&state.db.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    if event.status != "upcoming" {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Build prediction type JSON
    let prediction_type_json = match payload.prediction_type.as_str() {
        "match_winner" => {
            json!({
                "type": "MatchWinner",
                "team_a": payload.team_a.ok_or(StatusCode::BAD_REQUEST)?,
                "team_b": payload.team_b.ok_or(StatusCode::BAD_REQUEST)?
            })
        }
        "exact_score" => {
            json!({
                "type": "ExactScore",
                "team_a": payload.team_a.ok_or(StatusCode::BAD_REQUEST)?,
                "team_b": payload.team_b.ok_or(StatusCode::BAD_REQUEST)?
            })
        }
        "player_event" => {
            json!({
                "type": "PlayerEvent",
                "player_id": payload.player_id.ok_or(StatusCode::BAD_REQUEST)?,
                "event_type": payload.event_type.ok_or(StatusCode::BAD_REQUEST)?
            })
        }
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    // Create prediction in database
    // Note: In real implementation, you'd need to pass the user_id from auth context
    let user_id = Uuid::new_v4(); // Placeholder - get from JWT claims

    let prediction = sqlx::query_as::<_, Prediction>(
        r#"
        INSERT INTO predictions (user_id, event_id, prediction_type, predicted_outcome)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
    )
    .bind(user_id)
    .bind(payload.event_id)
    .bind(prediction_type_json)
    .bind(payload.predicted_outcome)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create prediction: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(PredictionResponse {
        id: prediction.id,
        user_id: prediction.user_id,
        event_id: prediction.event_id,
        prediction_type: serde_json::to_string(&prediction.prediction_type).unwrap_or_default(),
        predicted_outcome: prediction.predicted_outcome,
        points_awarded: prediction.points_awarded,
        is_correct: prediction.is_correct,
        created_at: prediction.created_at,
    }))
}

/// Get a specific prediction
pub async fn get_prediction(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<PredictionResponse>, StatusCode> {
    let prediction = sqlx::query_as::<_, Prediction>("SELECT * FROM predictions WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.db.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(PredictionResponse {
        id: prediction.id,
        user_id: prediction.user_id,
        event_id: prediction.event_id,
        prediction_type: serde_json::to_string(&prediction.prediction_type).unwrap_or_default(),
        predicted_outcome: prediction.predicted_outcome,
        points_awarded: prediction.points_awarded,
        is_correct: prediction.is_correct,
        created_at: prediction.created_at,
    }))
}

/// Get all predictions for an event
pub async fn get_event_predictions(
    State(state): State<AppState>,
    Path(event_id): Path<Uuid>,
) -> Result<Json<Vec<PredictionResponse>>, StatusCode> {
    let predictions = sqlx::query_as::<_, Prediction>(
        "SELECT * FROM predictions WHERE event_id = $1 ORDER BY created_at DESC",
    )
    .bind(event_id)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response: Vec<PredictionResponse> = predictions
        .into_iter()
        .map(|p| PredictionResponse {
            id: p.id,
            user_id: p.user_id,
            event_id: p.event_id,
            prediction_type: serde_json::to_string(&p.prediction_type).unwrap_or_default(),
            predicted_outcome: p.predicted_outcome,
            points_awarded: p.points_awarded,
            is_correct: p.is_correct,
            created_at: p.created_at,
        })
        .collect();

    Ok(Json(response))
}

use serde_json::json;
