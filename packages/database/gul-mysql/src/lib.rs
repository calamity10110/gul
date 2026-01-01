use sqlx::mysql::{MySqlPool, MySqlPoolOptions};

pub struct MySqlManager {
    pool: MySqlPool,
}

impl MySqlManager {
    pub async fn new(url: &str) -> Result<Self, sqlx::Error> {
        let pool = MySqlPoolOptions::new().connect(url).await?;
        Ok(Self { pool })
    }
}
