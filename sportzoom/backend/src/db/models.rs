use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// User account model
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
    pub points: i32,
    pub level: i32,
    pub streak: i32,
    pub last_active: DateTime<Utc>,
    pub role: UserRole,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, FromRow, Serialize, Deserialize)]
#[sqlx(type_name = "user_role")]
pub enum UserRole {
    User,
    Admin,
    Moderator,
}

/// Event model (sports matches, concerts, etc.)
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Event {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub event_type: EventType,
    pub category: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub location: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub thumbnail_url: Option<String>,
    pub status: EventStatus,
    pub attendee_count: i32,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, FromRow, Serialize, Deserialize)]
#[sqlx(type_name = "event_type")]
pub enum EventType {
    Sports,
    Concert,
    Festival,
    Local,
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq, FromRow, Serialize, Deserialize)]
#[sqlx(type_name = "event_status")]
pub enum EventStatus {
    Scheduled,
    Live,
    Completed,
    Cancelled,
}

/// Prediction model
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Prediction {
    pub id: Uuid,
    pub user_id: Uuid,
    pub event_id: Uuid,
    pub prediction_type: PredictionType,
    pub predicted_value: String,
    pub points_awarded: Option<i32>,
    pub is_correct: Option<bool>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, FromRow, Serialize, Deserialize)]
#[sqlx(type_name = "prediction_type")]
pub enum PredictionType {
    MatchWinner,
    ExactScore,
    FirstGoal,
    Assist,
    Other,
}

/// Post model (social feed content)
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Post {
    pub id: Uuid,
    pub user_id: Uuid,
    pub event_id: Option<Uuid>,
    pub content_type: ContentType,
    pub media_url: String,
    pub thumbnail_url: Option<String>,
    pub caption: Option<String>,
    pub likes_count: i32,
    pub comments_count: i32,
    pub shares_count: i32,
    pub views_count: i32,
    pub duration_seconds: Option<i32>,
    pub is_trending: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, FromRow, Serialize, Deserialize)]
#[sqlx(type_name = "content_type")]
pub enum ContentType {
    Video,
    Image,
    Story,
}

/// Comment model
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Comment {
    pub id: Uuid,
    pub post_id: Uuid,
    pub user_id: Uuid,
    pub content: String,
    pub likes_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Like model
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Like {
    pub id: Uuid,
    pub user_id: Uuid,
    pub post_id: Option<Uuid>,
    pub comment_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

/// Badge model
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Badge {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub icon_url: String,
    pub points_required: i32,
    pub created_at: DateTime<Utc>,
}

/// User badge association
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct UserBadge {
    pub id: Uuid,
    pub user_id: Uuid,
    pub badge_id: Uuid,
    pub earned_at: DateTime<Utc>,
}

/// Live stream model
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct LiveStream {
    pub id: Uuid,
    pub user_id: Uuid,
    pub event_id: Option<Uuid>,
    pub title: String,
    pub description: Option<String>,
    pub viewer_count: i32,
    pub is_live: bool,
    pub stream_url: Option<String>,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
}

/// Notification model
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Notification {
    pub id: Uuid,
    pub user_id: Uuid,
    pub notification_type: NotificationType,
    pub title: String,
    pub message: String,
    pub is_read: bool,
    pub related_entity_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, FromRow, Serialize, Deserialize)]
#[sqlx(type_name = "notification_type")]
pub enum NotificationType {
    Like,
    Comment,
    Follow,
    PredictionResult,
    EventReminder,
    TrendingAlert,
    System,
}

/// Analytics tracking model
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Analytics {
    pub id: Uuid,
    pub event_type: AnalyticsEventType,
    pub user_id: Option<Uuid>,
    pub entity_id: Option<Uuid>,
    pub entity_type: EntityType,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, FromRow, Serialize, Deserialize)]
#[sqlx(type_name = "analytics_event_type")]
pub enum AnalyticsEventType {
    View,
    Like,
    Comment,
    Share,
    WatchTime,
    Click,
    Conversion,
}

#[derive(Debug, Clone, PartialEq, Eq, FromRow, Serialize, Deserialize)]
#[sqlx(type_name = "entity_type")]
pub enum EntityType {
    Post,
    Event,
    Prediction,
    Stream,
    User,
}

/// Event attendee model
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct EventAttendee {
    pub id: Uuid,
    pub event_id: Uuid,
    pub user_id: Uuid,
    pub status: AttendeeStatus,
    pub checked_in: bool,
    pub joined_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, FromRow, Serialize, Deserialize)]
#[sqlx(type_name = "attendee_status")]
pub enum AttendeeStatus {
    Interested,
    Attending,
    Maybe,
}

// Request/Response DTOs
#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
    pub display_name: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub user: UserPublic,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserPublic {
    pub id: Uuid,
    pub username: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub country: Option<String>,
    pub points: i32,
    pub level: i32,
}

impl From<User> for UserPublic {
    fn from(user: User) -> Self {
        UserPublic {
            id: user.id,
            username: user.username,
            display_name: user.display_name,
            avatar_url: user.avatar_url,
            bio: user.bio,
            country: user.country,
            points: user.points,
            level: user.level,
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreatePredictionRequest {
    pub event_id: Uuid,
    pub prediction_type: PredictionType,
    #[validate(length(max = 255))]
    pub predicted_value: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreatePostRequest {
    pub event_id: Option<Uuid>,
    pub content_type: ContentType,
    #[validate(url)]
    pub media_url: String,
    pub thumbnail_url: Option<String>,
    pub caption: Option<String>,
    pub duration_seconds: Option<i32>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateCommentRequest {
    #[validate(length(min = 1, max = 1000))]
    pub content: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateEventRequest {
    #[validate(length(min = 1, max = 255))]
    pub title: String,
    pub description: Option<String>,
    pub event_type: EventType,
    pub category: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub location: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub thumbnail_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct JoinEventRequest {
    pub status: AttendeeStatus,
}

#[derive(Debug, Deserialize, Validate)]
pub struct StartLiveStreamRequest {
    pub event_id: Option<Uuid>,
    #[validate(length(min = 1, max = 255))]
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct FeedPost {
    pub post: Post,
    pub user: UserPublic,
    pub is_liked: bool,
}

#[derive(Debug, Serialize)]
pub struct LeaderboardEntry {
    pub rank: i64,
    pub user: UserPublic,
    pub points: i32,
}

#[derive(Debug, Serialize)]
pub struct DashboardStats {
    pub total_users: i64,
    pub active_users_24h: i64,
    pub total_posts: i64,
    pub total_events: i64,
    pub live_streams: i64,
    pub total_predictions: i64,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProfileRequest {
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub country: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

impl Default for PaginationParams {
    fn default() -> Self {
        PaginationParams {
            page: Some(1),
            limit: Some(20),
        }
    }
}
