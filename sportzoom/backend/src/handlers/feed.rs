use axum::{
    extract::{Query, State},
    Json,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::config::AppState;
use crate::db::models::{FeedPost, Post, UserPublic};
use crate::middleware::auth::Claims;
use crate::utils::errors::{AppError, Result};
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct FeedParams {
    pub limit: Option<i64>,
    pub cursor: Option<String>,
}

#[axum::debug_handler]
pub async fn get_personalized_feed(
    State(state): State<Arc<AppState>>,
    extensions: Option<axum::extract::Extension<Claims>>,
    Query(params): Query<FeedParams>,
) -> Result<Json<Vec<FeedPost>>> {
    let limit = params.limit.unwrap_or(20);
    
    // Get user's interests for personalization (simplified)
    let user_id = extensions.map(|e| e.sub);
    
    let posts = if let Some(uid) = user_id {
        // Personalized feed based on user's followed events and interactions
        sqlx::query_as::<_, Post>(
            r#"
            SELECT p.*, 
                   CASE WHEN l.user_id IS NOT NULL THEN true ELSE false END as is_liked_temp
            FROM posts p
            LEFT JOIN likes l ON p.id = l.post_id AND l.user_id = $1
            WHERE p.user_id != $1
            ORDER BY 
                (p.is_trending DESC),
                (p.likes_count * 3 + p.comments_count * 5 + p.views_count * 2) DESC,
                p.created_at DESC
            LIMIT $2
            "#
        )
        .bind(uid)
        .bind(limit)
        .fetch_all(&state.db)
        .await?
    } else {
        // Public feed for non-authenticated users
        sqlx::query_as::<_, Post>(
            "SELECT * FROM posts WHERE content_type != 'story' ORDER BY created_at DESC LIMIT $1"
        )
        .bind(limit)
        .fetch_all(&state.db)
        .await?
    };

    // Enrich posts with user data and like status
    let mut feed_posts = Vec::new();
    for post in posts {
        let user = sqlx::query_as::<_, UserPublic>(
            "SELECT id, username, display_name, avatar_url, bio, country, points, level FROM users WHERE id = $1"
        )
        .bind(post.user_id)
        .fetch_optional(&state.db)
        .await?
        .unwrap_or(UserPublic {
            id: post.user_id,
            username: "unknown".to_string(),
            display_name: None,
            avatar_url: None,
            bio: None,
            country: None,
            points: 0,
            level: 1,
        });

        let is_liked = user_id.map(|uid| {
            // Check if user liked this post (simplified - would use cache in production)
            false
        }).unwrap_or(false);

        feed_posts.push(FeedPost {
            post,
            user,
            is_liked,
        });
    }

    Ok(Json(feed_posts))
}

#[axum::debug_handler]
pub async fn get_trending_feed(
    State(state): State<Arc<AppState>>,
    Query(params): Query<FeedParams>,
) -> Result<Json<Vec<FeedPost>>> {
    let limit = params.limit.unwrap_or(20);

    let posts = sqlx::query_as::<_, Post>(
        r#"
        SELECT * FROM posts 
        WHERE is_trending = true AND content_type != 'story'
        ORDER BY (likes_count * 3 + comments_count * 5 + views_count * 2) DESC, created_at DESC
        LIMIT $1
        "#
    )
    .bind(limit)
    .fetch_all(&state.db)
    .await?;

    let mut feed_posts = Vec::new();
    for post in posts {
        let user = sqlx::query_as::<_, UserPublic>(
            "SELECT id, username, display_name, avatar_url, bio, country, points, level FROM users WHERE id = $1"
        )
        .bind(post.user_id)
        .fetch_one(&state.db)
        .await?;

        feed_posts.push(FeedPost {
            post,
            user,
            is_liked: false,
        });
    }

    Ok(Json(feed_posts))
}

#[axum::debug_handler]
pub async fn get_recommendations(
    State(state): State<Arc<AppState>>,
    extensions: axum::extract::Extension<Claims>,
    Query(params): Query<FeedParams>,
) -> Result<Json<Vec<FeedPost>>> {
    let limit = params.limit.unwrap_or(10);
    
    // AI-powered recommendations (simplified - would use ML in production)
    // This queries posts from similar users or related to user's interests
    let posts = sqlx::query_as::<_, Post>(
        r#"
        WITH user_interactions AS (
            SELECT entity_id, COUNT(*) as interaction_count
            FROM analytics
            WHERE user_id = $1 AND entity_type = 'post'
            GROUP BY entity_id
            ORDER BY interaction_count DESC
            LIMIT 10
        )
        SELECT DISTINCT p.*
        FROM posts p
        JOIN user_interactions ui ON p.event_id = (SELECT event_id FROM posts WHERE id = ui.entity_id)
        WHERE p.user_id != $1
        ORDER BY p.created_at DESC
        LIMIT $2
        "#
    )
    .bind(extensions.sub)
    .bind(limit)
    .fetch_all(&state.db)
    .await
    .unwrap_or_else(|_| vec![]);

    // Fallback to trending if no recommendations found
    let posts = if posts.is_empty() {
        sqlx::query_as::<_, Post>(
            "SELECT * FROM posts WHERE content_type != 'story' ORDER BY likes_count DESC LIMIT $1"
        )
        .bind(limit)
        .fetch_all(&state.db)
        .await?
    } else {
        posts
    };

    let mut feed_posts = Vec::new();
    for post in posts {
        let user = sqlx::query_as::<_, UserPublic>(
            "SELECT id, username, display_name, avatar_url, bio, country, points, level FROM users WHERE id = $1"
        )
        .bind(post.user_id)
        .fetch_one(&state.db)
        .await?;

        feed_posts.push(FeedPost {
            post,
            user,
            is_liked: false,
        });
    }

    Ok(Json(feed_posts))
}
