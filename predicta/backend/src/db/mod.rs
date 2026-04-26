pub mod models;
pub mod repositories;

use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;

/// Database connection wrapper
#[derive(Clone)]
pub struct Database {
    pub pool: PgPool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(20)
            .min_connections(5)
            .acquire_timeout(Duration::from_secs(30))
            .idle_timeout(Duration::from_secs(600))
            .connect(database_url)
            .await?;

        Ok(Self { pool })
    }

    /// Run database migrations
    pub async fn migrate(&self) -> Result<(), sqlx::Error> {
        // In production, use sqlx migrate or flyway
        // For now, we'll create tables manually
        self.create_tables().await
    }

    /// Create all necessary tables
    async fn create_tables(&self) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            -- Users table
            CREATE TABLE IF NOT EXISTS users (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                username VARCHAR(50) UNIQUE NOT NULL,
                email VARCHAR(255) UNIQUE NOT NULL,
                password_hash VARCHAR(255) NOT NULL,
                display_name VARCHAR(100),
                avatar_url TEXT,
                bio TEXT,
                country VARCHAR(100),
                points BIGINT DEFAULT 0,
                level INTEGER DEFAULT 1,
                is_verified BOOLEAN DEFAULT FALSE,
                is_banned BOOLEAN DEFAULT FALSE,
                created_at TIMESTAMPTZ DEFAULT NOW(),
                updated_at TIMESTAMPTZ DEFAULT NOW()
            );

            -- Events table
            CREATE TABLE IF NOT EXISTS events (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                title VARCHAR(255) NOT NULL,
                description TEXT,
                event_type VARCHAR(50) NOT NULL,
                category VARCHAR(100) NOT NULL,
                location VARCHAR(255),
                latitude DOUBLE PRECISION,
                longitude DOUBLE PRECISION,
                start_time TIMESTAMPTZ NOT NULL,
                end_time TIMESTAMPTZ,
                organizer_id UUID REFERENCES users(id),
                attendee_count INTEGER DEFAULT 0,
                is_live BOOLEAN DEFAULT FALSE,
                status VARCHAR(50) DEFAULT 'upcoming',
                created_at TIMESTAMPTZ DEFAULT NOW()
            );

            -- Predictions table
            CREATE TABLE IF NOT EXISTS predictions (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                user_id UUID REFERENCES users(id) ON DELETE CASCADE,
                event_id UUID REFERENCES events(id) ON DELETE CASCADE,
                prediction_type JSONB NOT NULL,
                predicted_outcome VARCHAR(255) NOT NULL,
                points_awarded INTEGER DEFAULT 0,
                is_correct BOOLEAN,
                created_at TIMESTAMPTZ DEFAULT NOW()
            );

            -- Posts table (social feed)
            CREATE TABLE IF NOT EXISTS posts (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                user_id UUID REFERENCES users(id) ON DELETE CASCADE,
                event_id UUID REFERENCES events(id),
                content_type VARCHAR(50) NOT NULL,
                media_url TEXT NOT NULL,
                thumbnail_url TEXT,
                caption TEXT,
                duration_seconds INTEGER,
                like_count INTEGER DEFAULT 0,
                comment_count INTEGER DEFAULT 0,
                share_count INTEGER DEFAULT 0,
                view_count INTEGER DEFAULT 0,
                total_watch_time_ms BIGINT DEFAULT 0,
                is_trending BOOLEAN DEFAULT FALSE,
                ai_score DOUBLE PRECISION DEFAULT 0.0,
                created_at TIMESTAMPTZ DEFAULT NOW()
            );

            -- Comments table
            CREATE TABLE IF NOT EXISTS comments (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                post_id UUID REFERENCES posts(id) ON DELETE CASCADE,
                user_id UUID REFERENCES users(id) ON DELETE CASCADE,
                parent_comment_id UUID REFERENCES comments(id) ON DELETE CASCADE,
                content TEXT NOT NULL,
                like_count INTEGER DEFAULT 0,
                created_at TIMESTAMPTZ DEFAULT NOW()
            );

            -- Live streams table
            CREATE TABLE IF NOT EXISTS live_streams (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                user_id UUID REFERENCES users(id),
                event_id UUID REFERENCES events(id),
                title VARCHAR(255) NOT NULL,
                description TEXT,
                stream_url TEXT NOT NULL,
                viewer_count INTEGER DEFAULT 0,
                like_count INTEGER DEFAULT 0,
                is_active BOOLEAN DEFAULT TRUE,
                started_at TIMESTAMPTZ DEFAULT NOW(),
                ended_at TIMESTAMPTZ
            );

            -- Badges table
            CREATE TABLE IF NOT EXISTS badges (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                name VARCHAR(100) NOT NULL,
                description TEXT NOT NULL,
                icon_url TEXT NOT NULL,
                criteria TEXT NOT NULL
            );

            -- User badges (many-to-many)
            CREATE TABLE IF NOT EXISTS user_badges (
                user_id UUID REFERENCES users(id) ON DELETE CASCADE,
                badge_id UUID REFERENCES badges(id) ON DELETE CASCADE,
                earned_at TIMESTAMPTZ DEFAULT NOW(),
                PRIMARY KEY (user_id, badge_id)
            );

            -- Notifications table
            CREATE TABLE IF NOT EXISTS notifications (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                user_id UUID REFERENCES users(id) ON DELETE CASCADE,
                notification_type VARCHAR(50) NOT NULL,
                title VARCHAR(255) NOT NULL,
                message TEXT NOT NULL,
                data JSONB,
                is_read BOOLEAN DEFAULT FALSE,
                created_at TIMESTAMPTZ DEFAULT NOW()
            );

            -- Likes table (for posts and comments)
            CREATE TABLE IF NOT EXISTS likes (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                user_id UUID REFERENCES users(id) ON DELETE CASCADE,
                post_id UUID REFERENCES posts(id) ON DELETE CASCADE,
                comment_id UUID REFERENCES comments(id) ON DELETE CASCADE,
                created_at TIMESTAMPTZ DEFAULT NOW(),
                UNIQUE(user_id, post_id, comment_id)
            );

            -- Event attendees (many-to-many)
            CREATE TABLE IF NOT EXISTS event_attendees (
                user_id UUID REFERENCES users(id) ON DELETE CASCADE,
                event_id UUID REFERENCES events(id) ON DELETE CASCADE,
                status VARCHAR(50) DEFAULT 'interested', -- interested, attending, checked_in
                joined_at TIMESTAMPTZ DEFAULT NOW(),
                PRIMARY KEY (user_id, event_id)
            );

            -- Analytics table
            CREATE TABLE IF NOT EXISTS analytics (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                date TIMESTAMPTZ DEFAULT NOW(),
                metric_type VARCHAR(50) NOT NULL,
                metric_name VARCHAR(100) NOT NULL,
                value BIGINT NOT NULL
            );

            -- Indexes for performance
            CREATE INDEX IF NOT EXISTS idx_posts_ai_score ON posts(ai_score DESC);
            CREATE INDEX IF NOT EXISTS idx_posts_created_at ON posts(created_at DESC);
            CREATE INDEX IF NOT EXISTS idx_posts_trending ON posts(is_trending) WHERE is_trending = TRUE;
            CREATE INDEX IF NOT EXISTS idx_predictions_user ON predictions(user_id);
            CREATE INDEX IF NOT EXISTS idx_predictions_event ON predictions(event_id);
            CREATE INDEX IF NOT EXISTS idx_events_start_time ON events(start_time);
            CREATE INDEX IF NOT EXISTS idx_events_location ON events(latitude, longitude);
            CREATE INDEX IF NOT EXISTS idx_notifications_user ON notifications(user_id, is_read);
            CREATE INDEX IF NOT EXISTS idx_live_streams_active ON live_streams(is_active) WHERE is_active = TRUE;
            "#,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
