
use sqlx::{PgPool, Error};

/// Repository for database operations
pub struct Repository {
    pool: PgPool,
}

impl Repository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Health check
    pub async fn health_check(&self) -> Result<bool, Error> {
        sqlx::query("SELECT 1")
            .fetch_one(&self.pool)
            .await
            .map(|_| true)
    }
}
