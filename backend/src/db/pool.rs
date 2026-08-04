use sqlx::PgPool;
use crate::config::Config;

pub async fn init_pool(config: &Config) -> Result<PgPool, sqlx::Error> {
    PgPool::connect(&config.database_url).await
}