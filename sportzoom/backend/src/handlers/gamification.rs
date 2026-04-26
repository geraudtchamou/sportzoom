use axum::{
    extract::{Path, Query, State},
    Json,
};
use uuid::Uuid;

use crate::config::AppState;
use crate::db::models::{LeaderboardEntry, UserPublic, Badge, UserBadge};
use crate::middleware::auth::Claims;
use crate::utils::errors::{AppError, Result};
use std::sync::Arc;

#[axum::debug_handler]
pub async fn get_leaderboard(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<LeaderboardEntry>>> {
    let entries = sqlx::query_as::<_, (i64, Uuid, String, Option<String>, Option<String>, Option<String>, i32, i32)>(
        r#"
        SELECT ROW_NUMBER() OVER (ORDER BY points DESC) as rank,
               id, username, display_name, avatar_url, country, points, level
        FROM users
        ORDER BY points DESC
        LIMIT 100
        "#
    )
    .fetch_all(&state.db)
    .await?;

    let leaderboard: Vec<LeaderboardEntry> = entries
        .into_iter()
        .map(|(rank, id, username, display_name, avatar_url, country, points, level)| {
            LeaderboardEntry {
                rank,
                user: UserPublic {
                    id,
                    username,
                    display_name,
                    avatar_url,
                    bio: None,
                    country,
                    points,
                    level,
                },
                points,
            }
        })
        .collect();

    Ok(Json(leaderboard))
}

#[axum::debug_handler]
pub async fn get_user_points(
    State(state): State<Arc<AppState>>,
    extensions: axum::extract::Extension<Claims>,
) -> Result<Json<serde_json::Value>> {
    let user = sqlx::query("SELECT points, level, streak FROM users WHERE id = $1")
        .bind(extensions.sub)
        .fetch_one(&state.db)
        .await?;

    let points: i32 = user.get("points");
    let level: i32 = user.get("level");
    let streak: i32 = user.get("streak");

    Ok(Json(serde_json::json!({
        "points": points,
        "level": level,
        "streak": streak,
        "next_level_at": level * 1000
    })))
}

#[axum::debug_handler]
pub async fn get_user_badges(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<Vec<(Badge, UserBadge)>>> {
    let badges = sqlx::query_as::<_, (Badge, UserBadge)>(
        r#"
        SELECT b.*, ub.*
        FROM badges b
        JOIN user_badges ub ON b.id = ub.badge_id
        WHERE ub.user_id = $1
        ORDER BY ub.earned_at DESC
        "#
    )
    .bind(user_id)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(badges))
}

#[axum::debug_handler]
pub async fn get_all_badges(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Badge>>> {
    let badges = sqlx::query_as::<_, Badge>("SELECT * FROM badges ORDER BY points_required")
        .fetch_all(&state.db)
        .await?;

    Ok(Json(badges))
}

#[axum::debug_handler]
pub async fn get_achievements(
    State(_state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>> {
    // Return predefined achievements structure
    Ok(Json(serde_json::json!([
        {
            "id": "top_predictor",
            "name": "Top Predictor",
            "description": "Make 100 correct predictions",
            "icon": "🎯",
            "points": 500
        },
        {
            "id": "event_king",
            "name": "Event King",
            "description": "Attend 50 events",
            "icon": "👑",
            "points": 300
        },
        {
            "id": "content_creator",
            "name": "Content Creator",
            "description": "Post 100 videos",
            "icon": "🎬",
            "points": 400
        },
        {
            "id": "social_star",
            "name": "Social Star",
            "description": "Get 1000 likes on posts",
            "icon": "⭐",
            "points": 600
        }
    ])))
}
