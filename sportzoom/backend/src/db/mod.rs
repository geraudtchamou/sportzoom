pub mod models;

use sqlx::postgres::{PgPool, PgPoolOptions};
use anyhow::Result;

pub use models::*;

pub async fn init_db(database_url: &str) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .acquire_timeout(std::time::Duration::from_secs(30))
        .connect(database_url)
        .await?;

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(|e| anyhow::anyhow!("Migration failed: {}", e))?;

    Ok(pool)
}

pub async fn init_redis(redis_url: &str) -> Result<redis::Client> {
    let client = redis::Client::open(redis_url)?;
    
    // Test connection
    let mut conn = client.get_tokio_connection().await?;
    redis::cmd("PING").query_async::<_, String>(&mut conn).await?;
    
    Ok(client)
}
