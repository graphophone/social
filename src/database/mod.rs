use sqlx::{Connection, postgres::PgPoolOptions};
use anyhow::Result;
use std::time::Duration;

pub mod likes;

pub struct SocialDb {
    pool: sqlx::PgPool,
}

impl SocialDb {
    pub async fn build(conf: &crate::config::PostgresConfig) -> Result<Self> {
        let db_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            conf.username,
            conf.password,
            conf.host,
            conf.port,
            conf.database,
        );
        let pool = PgPoolOptions::new()
            .max_connections(50)
            .acquire_timeout(Duration::from_secs(3))
            .idle_timeout(Duration::from_secs(10))
            .connect(&db_url)
            .await?;

        sqlx::migrate!("./migrations")
            .run(&pool)
            .await?;

        return Ok(SocialDb {
            pool,
        })
    }

    pub async fn ping(&self) -> Result<()> {
        let mut conn = self.pool.acquire()
            .await?;
        conn.ping().await?;
        Ok(())
    }
}