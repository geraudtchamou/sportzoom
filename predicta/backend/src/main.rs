use axum::{
    routing::{get, post},
    Router,
};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod db;
mod handlers;
mod middleware;
mod services;
mod utils;
mod websocket;

use config::Config;
use db::Database;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "predicta_backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::from_env()?;
    tracing::info!("Configuration loaded successfully");

    // Initialize database connection
    let db = Database::new(&config.database_url).await?;
    tracing::info!("Database connection established");

    // Initialize Redis connection
    let redis_pool = redis::Client::open(config.redis_url.as_str())?
        .get_tokio_connection_manager()
        .await?;
    tracing::info!("Redis connection established");

    // Create shared application state
    let app_state = Arc::new(AppState {
        db,
        redis: redis_pool,
        config: config.clone(),
    });

    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build router
    let app = Router::new()
        // Health check
        .route("/health", get(handlers::health))
        // Authentication routes
        .nest(
            "/api/v1/auth",
            Router::new()
                .route("/register", post(handlers::auth::register))
                .route("/login", post(handlers::auth::login))
                .route("/refresh", post(handlers::auth::refresh_token))
                .route("/logout", post(handlers::auth::logout)),
        )
        // User routes
        .nest(
            "/api/v1/users",
            Router::new()
                .route("/:id", get(handlers::users::get_user))
                .route("/profile", get(handlers::users::get_profile))
                .route("/profile", put(handlers::users::update_profile)),
        )
        // Prediction routes
        .nest(
            "/api/v1/predictions",
            Router::new()
                .route("/", get(handlers::predictions::list_predictions))
                .route("/", post(handlers::predictions::create_prediction))
                .route("/:id", get(handlers::predictions::get_prediction))
                .route("/events/:event_id", get(handlers::predictions::get_event_predictions)),
        )
        // Event routes
        .nest(
            "/api/v1/events",
            Router::new()
                .route("/", get(handlers::events::list_events))
                .route("/", post(handlers::events::create_event))
                .route("/:id", get(handlers::events::get_event))
                .route("/:id/join", post(handlers::events::join_event))
                .route("/:id/room", get(handlers::events::get_event_room)),
        )
        // Feed routes
        .nest(
            "/api/v1/feed",
            Router::new()
                .route("/", get(handlers::feed::get_feed))
                .route("/trending", get(handlers::feed::get_trending))
                .route("/recommendations", get(handlers::feed::get_recommendations)),
        )
        // Post routes
        .nest(
            "/api/v1/posts",
            Router::new()
                .route("/", get(handlers::posts::list_posts))
                .route("/", post(handlers::posts::create_post))
                .route("/:id", get(handlers::posts::get_post))
                .route("/:id/like", post(handlers::posts::like_post))
                .route("/:id/comment", post(handlers::posts::add_comment)),
        )
        // Live streaming routes
        .nest(
            "/api/v1/live",
            Router::new()
                .route("/", get(handlers::live::list_live_streams))
                .route("/start", post(handlers::live::start_stream))
                .route("/end", post(handlers::live::end_stream))
                .route("/:id", get(handlers::live::get_stream)),
        )
        // Gamification routes
        .nest(
            "/api/v1/gamification",
            Router::new()
                .route("/leaderboard", get(handlers::gamification::get_leaderboard))
                .route("/points", get(handlers::gamification::get_user_points))
                .route("/badges", get(handlers::gamification::get_user_badges))
                .route("/achievements", get(handlers::gamification::get_achievements)),
        )
        // WebSocket endpoint for real-time features
        .route("/ws", get(websocket::ws_handler))
        // Admin routes (protected)
        .nest(
            "/api/v1/admin",
            Router::new()
                .route("/users", get(handlers::admin::list_users))
                .route("/content", get(handlers::admin::moderate_content))
                .route("/analytics", get(handlers::admin::get_analytics)),
        )
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);

    // Start server
    let addr = format!("0.0.0.0:{}", config.port);
    tracing::info!("🚀 Predicta backend starting on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// Shared application state
pub struct AppState {
    pub db: Database,
    pub redis: redis::aio::ConnectionManager,
    pub config: Config,
}
