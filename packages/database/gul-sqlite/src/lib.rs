use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};

pub struct SqliteManager {
    pool: SqlitePool,
}

impl SqliteManager {
    pub async fn new(url: &str) -> Result<Self, sqlx::Error> {
        let pool = SqlitePoolOptions::new().connect(url).await?;
        Ok(Self { pool })
    }
}
