use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
};
use serde::Serialize;
use uuid::Uuid;

use crate::AppState;
use crate::db::models::LeaderboardEntry;

#[derive(Debug, Serialize)]
pub struct LeaderboardResponse {
    pub entries: Vec<LeaderboardEntry>,
    pub total_count: i64,
}

#[derive(Debug, Serialize)]
pub struct UserPointsResponse {
    pub user_id: Uuid,
    pub points: i64,
    pub level: i32,
    rank: i64,
}

/// Get global leaderboard
pub async fn get_leaderboard(
    State(state): State<AppState>,
) -> Result<Json<LeaderboardResponse>, StatusCode> {
    let entries = sqlx::query_as::<_, LeaderboardEntry>(
        r#"
        SELECT 
            ROW_NUMBER() OVER (ORDER BY points DESC) as rank,
            id as user_id,
            username,
            avatar_url,
            points,
            level,
            country
        FROM users
        WHERE is_banned = FALSE
        ORDER BY points DESC
        LIMIT 100
        "#,
    )
    .fetch_all(&state.db.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let total_count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users WHERE is_banned = FALSE")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or(0);

    Ok(Json(LeaderboardResponse {
        entries,
        total_count,
    }))
}

/// Get user points
pub async fn get_user_points(
    State(state): State<AppState>,
) -> Result<Json<UserPointsResponse>, StatusCode> {
    let user_id = Uuid::new_v4(); // Get from JWT in production

    let user = sqlx::query!(
        r#"
        SELECT 
            id,
            points,
            level,
            (SELECT COUNT(*) + 1 FROM users WHERE points > u.points AND is_banned = FALSE) as rank
        FROM users u
        WHERE id = $1
        "#,
    )
    .bind(user_id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(UserPointsResponse {
        user_id,
        points: user.points.unwrap_or(0),
        level: user.level.unwrap_or(1),
        rank: user.rank.unwrap_or(0),
    }))
}

/// Get user badges
pub async fn get_user_badges(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let user_id = Uuid::new_v4(); // Get from JWT in production

    let badges = sqlx::query(
        r#"
        SELECT b.* FROM badges b
        JOIN user_badges ub ON b.id = ub.badge_id
        WHERE ub.user_id = $1
        ORDER BY ub.earned_at DESC
        "#,
    )
    .bind(user_id)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({
        "badges": badges,
        "count": badges.len()
    })))
}

/// Get achievements
pub async fn get_achievements(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Return all available badges/achievements
    let achievements = sqlx::query("SELECT * FROM badges ORDER BY name")
        .fetch_all(&state.db.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({
        "achievements": achievements,
        "total": achievements.len()
    })))
}
