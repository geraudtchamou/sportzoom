mod config;
mod db;
mod handlers;
mod middleware;
mod utils;

use axum::{
    extract::State,
    http::StatusCode,
    middleware as axum_middleware,
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::{AppState, Config};
use crate::db::{init_db, init_redis};
use crate::handlers::*;
use crate::middleware::auth::{admin_middleware, auth_middleware};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "sportzoom_backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::from_env()?;
    
    tracing::info!("Starting SportZoom backend...");
    tracing::info!("Database: {}", config.database_url);
    tracing::info!("Redis: {}", config.redis_url);
    tracing::info!("Port: {}", config.port);

    // Initialize database
    let db_pool = init_db(&config.database_url).await?;
    
    // Initialize Redis
    let redis_client = init_redis(&config.redis_url).await?;

    // Create shared state
    let state = AppState::new(db_pool, redis_client, config.jwt_secret);

    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build router
    let app = create_router(state).layer(cors);

    // Start server
    let addr = format!("{}:{}", config.host, config.port);
    tracing::info!("🚀 Server listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        // Health check
        .route("/health", get(health_check))
        
        // API v1 routes
        .nest(
            "/api/v1",
            Router::new()
                // Auth routes
                .route("/auth/register", post(register))
                .route("/auth/login", post(login))
                .route("/auth/refresh", post(refresh_token))
                .route("/auth/logout", post(logout))
                
                // User routes
                .route("/users/:id", get(get_user_profile))
                .route("/users/profile", get(get_current_user_profile))
                .route("/users/profile", put(update_profile))
                
                // Prediction routes
                .route("/predictions", get(list_predictions).post(create_prediction))
                .route("/predictions/:id", get(get_prediction))
                .route("/predictions/events/:event_id", get(get_event_predictions))
                .route("/predictions/users/:user_id", get(get_user_predictions))
                .route("/predictions/:id/result", post(update_prediction_result))
                .route("/predictions/events/:event_id/suggestions", get(get_prediction_suggestions))
                
                // Event routes
                .route("/events", get(list_events).post(create_event))
                .route("/events/:id", get(get_event))
                .route("/events/:id/join", post(join_event))
                .route("/events/:id/attendees", get(get_event_attendees))
                
                // Feed routes
                .route("/feed", get(get_personalized_feed))
                .route("/feed/trending", get(get_trending_feed))
                .route("/feed/recommendations", get(get_recommendations))
                
                // Post routes
                .route("/posts", get(list_posts).post(create_post))
                .route("/posts/:id/like", post(like_post))
                .route("/posts/:id/unlike", post(unlike_post))
                .route("/posts/:id/comments", get(get_post_comments).post(add_comment))
                
                // Live streaming routes
                .route("/live", get(list_live_streams))
                .route("/live/start", post(start_stream))
                .route("/live/:id/end", post(end_stream))
                .route("/live/:id", get(get_stream))
                .route("/live/:id/viewers", post(increment_viewer_count))
                
                // Gamification routes
                .route("/gamification/leaderboard", get(get_leaderboard))
                .route("/gamification/points", get(get_user_points))
                .route("/gamification/badges", get(get_all_badges))
                .route("/gamification/badges/:user_id", get(get_user_badges))
                .route("/gamification/achievements", get(get_achievements))
                
                // Admin routes (protected)
                .route("/admin/stats", get(get_dashboard_stats))
                .route("/admin/users", get(list_all_users))
                .route("/admin/users/:id/ban", post(ban_user))
                .route("/admin/users/:id/admin", post(make_admin))
                .route("/admin/posts/:id", delete(delete_post))
        )
        .layer(axum_middleware::from_fn_with_state(state.clone(), auth_middleware))
        .with_state(state)
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}
