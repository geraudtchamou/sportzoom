use redis::Client as RedisClient;
use sqlx::postgres::PgPool;
use std::sync::Arc;

/// Application configuration
#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub redis_url: String,
    pub jwt_secret: String,
    pub port: u16,
    pub host: String,
}

impl Config {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self, config::ConfigError> {
        dotenvy::dotenv().ok();

        let config = config::Config::builder()
            .add_source(config::Environment::default())
            .build()?;

        Ok(Config {
            database_url: config.get_string("DATABASE_URL")
                .unwrap_or_else(|_| "postgresql://postgres:password@localhost:5432/sportzoom".to_string()),
            redis_url: config.get_string("REDIS_URL")
                .unwrap_or_else(|_| "redis://localhost:6379".to_string()),
            jwt_secret: config.get_string("JWT_SECRET")
                .unwrap_or_else(|_| "change-this-secret-in-production".to_string()),
            port: config.get::<u16>("PORT").unwrap_or(8080),
            host: config.get_string("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
        })
    }
}

/// Shared application state
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub redis: RedisClient,
    pub jwt_secret: String,
}

impl AppState {
    pub fn new(db: PgPool, redis: RedisClient, jwt_secret: String) -> Arc<Self> {
        Arc::new(AppState {
            db,
            redis,
            jwt_secret,
        })
    }
}
