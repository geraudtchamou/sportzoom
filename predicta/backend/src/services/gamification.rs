/// Gamification Service
/// 
/// Handles points, levels, badges, and achievements

use uuid::Uuid;

/// Points configuration
pub struct PointsConfig {
    pub prediction_correct: i32,
    pub prediction_exact: i32,
    pub daily_login: i32,
    pub post_created: i32,
    pub post_trending: i32,
    pub referral_signup: i32,
    pub streak_bonus: i32,
}

impl Default for PointsConfig {
    fn default() -> Self {
        Self {
            prediction_correct: 100,
            prediction_exact: 500,
            daily_login: 10,
            post_created: 20,
            post_trending: 200,
            referral_signup: 100,
            streak_bonus: 50,
        }
    }
}

/// Calculate user level based on total points
/// Level formula: level = floor(sqrt(points / 100)) + 1
pub fn calculate_level(total_points: i64) -> i32 {
    ((total_points as f64 / 100.0).sqrt().floor() as i32) + 1
}

/// Award points to a user
pub async fn award_points(
    pool: &sqlx::PgPool,
    user_id: Uuid,
    points: i32,
    reason: &str,
) -> Result<(i64, i32), sqlx::Error> {
    // Add points to user
    let user = sqlx::query!(
        r#"
        UPDATE users 
        SET 
            points = points + $1,
            level = floor(sqrt((points + $1)::float / 100.0))::int + 1,
            updated_at = NOW()
        WHERE id = $2
        RETURNING points, level
        "#,
        points as i64,
        user_id,
    )
    .fetch_one(pool)
    .await?;

    // Log points transaction
    sqlx::query!(
        "INSERT INTO analytics (metric_type, metric_name, value) VALUES ($1, $2, $3)",
        "points_awarded",
        reason,
        points as i64,
    )
    .execute(pool)
    .await?;

    Ok((user.points.unwrap_or(0), user.level.unwrap_or(1)))
}

/// Check and award badges to user
pub async fn check_and_award_badges(
    pool: &sqlx::PgPool,
    user_id: Uuid,
) -> Result<Vec<Uuid>, sqlx::Error> {
    let mut earned_badges = Vec::new();

    // Get user stats
    let stats = sqlx::query!(
        r#"
        SELECT 
            u.points,
            u.level,
            COUNT(DISTINCT p.id) as post_count,
            COUNT(DISTINCT pr.id) as prediction_count,
            COUNT(DISTINCT CASE WHEN pr.is_correct = TRUE THEN pr.id END) as correct_predictions
        FROM users u
        LEFT JOIN posts p ON p.user_id = u.id
        LEFT JOIN predictions pr ON pr.user_id = u.id
        WHERE u.id = $1
        GROUP BY u.id
        "#,
        user_id,
    )
    .fetch_optional(pool)
    .await?;

    if let Some(stats) = stats {
        // Check for "Top Predictor" badge (50+ correct predictions)
        if stats.correct_predictions.unwrap_or(0) >= 50 {
            if let Ok(badge_id) = get_badge_by_name(pool, "Top Predictor").await {
                if !has_badge(pool, user_id, badge_id).await? {
                    award_badge(pool, user_id, badge_id).await?;
                    earned_badges.push(badge_id);
                }
            }
        }

        // Check for "Event King" badge (100+ event participations)
        // Implementation depends on event_attendees table
        
        // Check for "Content Creator" badge (50+ posts)
        if stats.post_count.unwrap_or(0) >= 50 {
            if let Ok(badge_id) = get_badge_by_name(pool, "Content Creator").await {
                if !has_badge(pool, user_id, badge_id).await? {
                    award_badge(pool, user_id, badge_id).await?;
                    earned_badges.push(badge_id);
                }
            }
        }
    }

    Ok(earned_badges)
}

/// Get badge by name
async fn get_badge_by_name(pool: &sqlx::PgPool, name: &str) -> Result<Uuid, sqlx::Error> {
    let badge = sqlx::query_scalar::<_, Uuid>("SELECT id FROM badges WHERE name = $1")
        .bind(name)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| sqlx::Error::RowNotFound)?;
    
    Ok(badge)
}

/// Check if user has a badge
async fn has_badge(
    pool: &sqlx::PgPool,
    user_id: Uuid,
    badge_id: Uuid,
) -> Result<bool, sqlx::Error> {
    let exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM user_badges WHERE user_id = $1 AND badge_id = $2)",
    )
    .bind(user_id)
    .bind(badge_id)
    .fetch_one(pool)
    .await?;
    
    Ok(exists)
}

/// Award badge to user
async fn award_badge(
    pool: &sqlx::PgPool,
    user_id: Uuid,
    badge_id: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO user_badges (user_id, badge_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
    )
    .bind(user_id)
    .bind(badge_id)
    .execute(pool)
    .await?;
    
    Ok(())
}

/// Track daily login streak
pub async fn track_daily_login(
    pool: &sqlx::PgPool,
    user_id: Uuid,
) -> Result<(i32, i32), sqlx::Error> {
    let today = chrono::Utc::now().date_naive();
    
    // Get last login date and current streak
    let result = sqlx::query!(
        r#"
        WITH last_login AS (
            SELECT created_at 
            FROM analytics 
            WHERE metric_type = 'daily_login' 
              AND metric_name = $1::text
            ORDER BY created_at DESC 
            LIMIT 1
        )
        SELECT 
            CASE 
                WHEN MAX(created_at)::date = $2 - INTERVAL '1 day' THEN 
                    (SELECT COUNT(*) FROM analytics WHERE metric_type = 'daily_login' AND metric_name = $1::text AND created_at > NOW() - INTERVAL '30 days')::int
                WHEN MAX(created_at)::date = $2 THEN 1
                ELSE 1
            END as streak
        FROM last_login
        "#,
        user_id.to_string(),
        today,
    )
    .fetch_optional(pool)
    .await?;

    let streak = result.and_then(|r| r.streak).unwrap_or(1) as i32;

    // Log daily login
    sqlx::query!(
        "INSERT INTO analytics (metric_type, metric_name, value) VALUES ('daily_login', $1, 1)",
        user_id.to_string(),
    )
    .execute(pool)
    .await?;

    // Calculate bonus points based on streak
    let bonus = if streak >= 7 { 100 } else if streak >= 3 { 50 } else { 10 };

    Ok((streak, bonus))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_level() {
        assert_eq!(calculate_level(0), 1);
        assert_eq!(calculate_level(100), 2);
        assert_eq!(calculate_level(400), 3);
        assert_eq!(calculate_level(900), 4);
        assert_eq!(calculate_level(10000), 11);
    }
}
