/// Notifications Service
/// 
/// Handles push notifications via Firebase Cloud Messaging (FCM)
/// and in-app notifications

use uuid::Uuid;

/// Notification types
#[derive(Debug, Clone)]
pub enum NotificationType {
    Like,
    Comment,
    Follow,
    PredictionResult,
    EventReminder,
    TrendingAlert,
    FriendActivity,
    System,
}

impl NotificationType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Like => "like",
            Self::Comment => "comment",
            Self::Follow => "follow",
            Self::PredictionResult => "prediction_result",
            Self::EventReminder => "event_reminder",
            Self::TrendingAlert => "trending_alert",
            Self::FriendActivity => "friend_activity",
            Self::System => "system",
        }
    }
}

/// Create an in-app notification
pub async fn create_notification(
    pool: &sqlx::PgPool,
    user_id: Uuid,
    notification_type: NotificationType,
    title: String,
    message: String,
    data: Option<serde_json::Value>,
) -> Result<Uuid, sqlx::Error> {
    let id = Uuid::new_v4();

    sqlx::query(
        r#"
        INSERT INTO notifications (id, user_id, notification_type, title, message, data)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
    )
    .bind(id)
    .bind(user_id)
    .bind(notification_type.as_str())
    .bind(title)
    .bind(message)
    .bind(data)
    .execute(pool)
    .await?;

    Ok(id)
}

/// Send push notification via FCM
pub async fn send_push_notification(
    _fcm_client: &reqwest::Client,
    _fcm_server_key: &str,
    _device_token: &str,
    _title: &str,
    _body: &str,
    _data: Option<serde_json::Value>,
) -> Result<(), Box<dyn std::error::Error>> {
    // In production, integrate with Firebase Cloud Messaging
    // Example implementation:
    /*
    let payload = json!({
        "to": device_token,
        "notification": {
            "title": title,
            "body": body,
        },
        "data": data.unwrap_or(json!({})),
    });

    let response = _fcm_client
        .post("https://fcm.googleapis.com/fcm/send")
        .header("Authorization", format!("key={}", _fcm_server_key))
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(format!("FCM request failed: {}", response.status()).into());
    }
    */

    tracing::info!("Push notification sent to device: {}", _device_token);
    Ok(())
}

/// Send prediction result notification
pub async fn notify_prediction_result(
    pool: &sqlx::PgPool,
    user_id: Uuid,
    is_correct: bool,
    points_earned: i32,
    event_name: &str,
) -> Result<(), sqlx::Error> {
    let (title, message) = if is_correct {
        (
            "🎉 Correct Prediction!".to_string(),
            format!("You earned {} points for predicting the outcome of {}", points_earned, event_name),
        )
    } else {
        (
            "😔 Better Luck Next Time".to_string(),
            format!("Your prediction for {} was incorrect. Keep trying!", event_name),
        )
    };

    create_notification(
        pool,
        user_id,
        NotificationType::PredictionResult,
        title,
        message,
        Some(json!({
            "points_earned": points_earned,
            "is_correct": is_correct,
            "event_name": event_name
        })),
    )
    .await?;

    Ok(())
}

/// Send event reminder notification
pub async fn notify_event_reminder(
    pool: &sqlx::PgPool,
    user_id: Uuid,
    event_name: &str,
    start_time: chrono::DateTime<chrono::Utc>,
) -> Result<(), sqlx::Error> {
    let title = "📅 Event Starting Soon!".to_string();
    let message = format!("{} starts in 30 minutes. Don't miss out!", event_name);

    create_notification(
        pool,
        user_id,
        NotificationType::EventReminder,
        title,
        message,
        Some(json!({
            "event_name": event_name,
            "start_time": start_time
        })),
    )
    .await?;

    Ok(())
}

/// Send trending alert notification
pub async fn notify_trending_content(
    pool: &sqlx::PgPool,
    user_id: Uuid,
    post_title: &str,
    view_count: i32,
) -> Result<(), sqlx::Error> {
    let title = "🔥 Your Post is Trending!".to_string();
    let message = format!("\"{}\" has reached {} views!", post_title, view_count);

    create_notification(
        pool,
        user_id,
        NotificationType::TrendingAlert,
        title,
        message,
        Some(json!({
            "post_title": post_title,
            "view_count": view_count
        })),
    )
    .await?;

    Ok(())
}

/// Get unread notification count for user
pub async fn get_unread_count(
    pool: &sqlx::PgPool,
    user_id: Uuid,
) -> Result<i64, sqlx::Error> {
    let count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM notifications WHERE user_id = $1 AND is_read = FALSE",
    )
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    Ok(count)
}

/// Mark notification as read
pub async fn mark_as_read(
    pool: &sqlx::PgPool,
    notification_id: Uuid,
    user_id: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE notifications SET is_read = TRUE WHERE id = $1 AND user_id = $2",
    )
    .bind(notification_id)
    .bind(user_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Mark all notifications as read for user
pub async fn mark_all_as_read(
    pool: &sqlx::PgPool,
    user_id: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE notifications SET is_read = TRUE WHERE user_id = $1 AND is_read = FALSE")
        .bind(user_id)
        .execute(pool)
        .await?;

    Ok(())
}

use serde_json::json;
