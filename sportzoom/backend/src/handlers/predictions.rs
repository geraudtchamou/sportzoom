use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::Utc;
use uuid::Uuid;
use validator::Validate;

use crate::config::AppState;
use crate::db::models::{CreatePredictionRequest, Prediction, PredictionType, UserPublic, PaginationParams};
use crate::middleware::auth::Claims;
use crate::utils::errors::{AppError, Result};
use std::sync::Arc;

/// Get all predictions with pagination
#[axum::debug_handler]
pub async fn list_predictions(
    State(state): State<Arc<AppState>>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<Vec<Prediction>>> {
    let limit = pagination.limit.unwrap_or(20) as i64;
    let offset = ((pagination.page.unwrap_or(1) - 1) * pagination.limit.unwrap_or(20)) as i64;

    let predictions = sqlx::query_as::<_, Prediction>(
        "SELECT * FROM predictions ORDER BY created_at DESC LIMIT $1 OFFSET $2"
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(predictions))
}

/// Create a new prediction
#[axum::debug_handler]
pub async fn create_prediction(
    State(state): State<Arc<AppState>>,
    extensions: axum::extract::Extension<Claims>,
    Json(payload): Json<CreatePredictionRequest>,
) -> Result<(StatusCode, Json<Prediction>)> {
    payload.validate()?;

    // Verify event exists
    let event_exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM events WHERE id = $1)"
    )
    .bind(payload.event_id)
    .fetch_one(&state.db)
    .await?;

    if !event_exists {
        return Err(AppError::not_found("Event not found"));
    }

    let prediction_id = Uuid::new_v4();
    
    let prediction = sqlx::query_as::<_, Prediction>(
        r#"
        INSERT INTO predictions (id, user_id, event_id, prediction_type, predicted_value)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *
        "#
    )
    .bind(prediction_id)
    .bind(extensions.sub)
    .bind(payload.event_id)
    .bind(&payload.prediction_type)
    .bind(&payload.predicted_value)
    .fetch_one(&state.db)
    .await?;

    // Award points for making a prediction (gamification)
    sqlx::query("UPDATE users SET points = points + 5 WHERE id = $1")
        .bind(extensions.sub)
        .execute(&state.db)
        .await?;

    Ok((StatusCode::CREATED, Json(prediction)))
}

/// Get prediction by ID
#[axum::debug_handler]
pub async fn get_prediction(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Prediction>> {
    let prediction = sqlx::query_as::<_, Prediction>(
        "SELECT * FROM predictions WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::not_found("Prediction not found"))?;

    Ok(Json(prediction))
}

/// Get predictions for a specific event
#[axum::debug_handler]
pub async fn get_event_predictions(
    State(state): State<Arc<AppState>>,
    Path(event_id): Path<Uuid>,
) -> Result<Json<Vec<Prediction>>> {
    let predictions = sqlx::query_as::<_, Prediction>(
        "SELECT * FROM predictions WHERE event_id = $1 ORDER BY created_at DESC"
    )
    .bind(event_id)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(predictions))
}

/// Get user's predictions
#[axum::debug_handler]
pub async fn get_user_predictions(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<Uuid>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<Vec<Prediction>>> {
    let limit = pagination.limit.unwrap_or(20) as i64;
    let offset = ((pagination.page.unwrap_or(1) - 1) * pagination.limit.unwrap_or(20)) as i64;

    let predictions = sqlx::query_as::<_, Prediction>(
        "SELECT * FROM predictions WHERE user_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3"
    )
    .bind(user_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(predictions))
}

/// AI-based prediction suggestions (placeholder for ML integration)
#[derive(Debug, Serialize)]
pub struct PredictionSuggestion {
    pub prediction_type: PredictionType,
    pub suggested_value: String,
    pub confidence: f64,
    pub reasoning: String,
}

#[axum::debug_handler]
pub async fn get_prediction_suggestions(
    State(state): State<Arc<AppState>>,
    Path(event_id): Path<Uuid>,
) -> Result<Json<Vec<PredictionSuggestion>>> {
    // In production, this would call an ML service
    // For now, return mock suggestions based on event data
    
    let event = sqlx::query("SELECT * FROM events WHERE id = $1")
        .bind(event_id)
        .fetch_optional(&state.db)
        .await?
        .ok_or_else(|| AppError::not_found("Event not found"))?;

    // Mock suggestions - in production, use actual ML model
    let suggestions = vec![
        PredictionSuggestion {
            prediction_type: PredictionType::MatchWinner,
            suggested_value: "Team A".to_string(),
            confidence: 0.65,
            reasoning: "Based on recent form and head-to-head record".to_string(),
        },
        PredictionSuggestion {
            prediction_type: PredictionType::ExactScore,
            suggested_value: "2-1".to_string(),
            confidence: 0.35,
            reasoning: "Average goals scored in last 5 matches".to_string(),
        },
    ];

    Ok(Json(suggestions))
}

/// Update prediction result (admin only)
#[derive(Debug, Deserialize)]
pub struct UpdatePredictionResultRequest {
    pub is_correct: bool,
    pub points_awarded: i32,
}

#[axum::debug_handler]
pub async fn update_prediction_result(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdatePredictionResultRequest>,
) -> Result<Json<Prediction>> {
    // Update prediction
    let prediction = sqlx::query_as::<_, Prediction>(
        r#"
        UPDATE predictions 
        SET is_correct = $1, points_awarded = $2
        WHERE id = $3
        RETURNING *
        "#
    )
    .bind(payload.is_correct)
    .bind(payload.points_awarded)
    .bind(id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::not_found("Prediction not found"))?;

    // If correct, award points to user
    if payload.is_correct && payload.points_awarded > 0 {
        sqlx::query("UPDATE users SET points = points + $1 WHERE id = $2")
            .bind(payload.points_awarded)
            .bind(prediction.user_id)
            .execute(&state.db)
            .await?;
    }

    Ok(Json(prediction))
}
