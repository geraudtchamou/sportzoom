use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// User model
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub country: Option<String>,
    pub points: i64,
    pub level: i32,
    pub is_verified: bool,
    pub is_banned: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Prediction types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum PredictionType {
    MatchWinner { team_a: String, team_b: String },
    ExactScore { team_a: String, team_b: String },
    PlayerEvent { player_id: Uuid, event_type: String },
}

/// Prediction model
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Prediction {
    pub id: Uuid,
    pub user_id: Uuid,
    pub event_id: Uuid,
    pub prediction_type: PredictionType,
    pub predicted_outcome: String,
    pub points_awarded: i32,
    pub is_correct: Option<bool>,
    pub created_at: DateTime<Utc>,
}

/// Event model
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Event {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub event_type: String, // sports, concert, festival, etc.
    pub category: String,
    pub location: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub organizer_id: Uuid,
    pub attendee_count: i32,
    pub is_live: bool,
    pub status: String, // upcoming, live, completed, cancelled
    pub created_at: DateTime<Utc>,
}

/// Post model (for social feed)
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Post {
    pub id: Uuid,
    pub user_id: Uuid,
    pub event_id: Option<Uuid>,
    pub content_type: String, // video, image, text
    pub media_url: String,
    pub thumbnail_url: Option<String>,
    pub caption: Option<String>,
    pub duration_seconds: Option<i32>,
    pub like_count: i32,
    pub comment_count: i32,
    pub share_count: i32,
    pub view_count: i32,
    pub total_watch_time_ms: i64,
    pub is_trending: bool,
    pub ai_score: f64,
    pub created_at: DateTime<Utc>,
}

/// Comment model
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Comment {
    pub id: Uuid,
    pub post_id: Uuid,
    pub user_id: Uuid,
    pub parent_comment_id: Option<Uuid>,
    pub content: String,
    pub like_count: i32,
    pub created_at: DateTime<Utc>,
}

/// Live stream model
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct LiveStream {
    pub id: Uuid,
    pub user_id: Uuid,
    pub event_id: Option<Uuid>,
    pub title: String,
    pub description: Option<String>,
    pub stream_url: String,
    pub viewer_count: i32,
    pub like_count: i32,
    pub is_active: bool,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
}

/// Badge model
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Badge {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub icon_url: String,
    pub criteria: String,
}

/// User badge (many-to-many)
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct UserBadge {
    pub user_id: Uuid,
    pub badge_id: Uuid,
    pub earned_at: DateTime<Utc>,
}

/// Leaderboard entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardEntry {
    pub rank: i64,
    pub user_id: Uuid,
    pub username: String,
    pub avatar_url: Option<String>,
    pub points: i64,
    pub level: i32,
    pub country: Option<String>,
}

/// Notification model
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Notification {
    pub id: Uuid,
    pub user_id: Uuid,
    pub notification_type: String,
    pub title: String,
    pub message: String,
    pub data: Option<serde_json::Value>,
    pub is_read: bool,
    pub created_at: DateTime<Utc>,
}

/// Analytics model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Analytics {
    pub date: DateTime<Utc>,
    pub metric_type: String,
    pub metric_name: String,
    pub value: i64,
}
