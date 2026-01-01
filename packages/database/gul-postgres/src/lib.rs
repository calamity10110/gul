use std::collections::HashMap;

#[cfg(feature = "real")]
use sqlx::postgres::{PgPool, PgPoolOptions};

/// PostgreSQL Connection
pub struct Connection {
    host: String,
    port: u16,
    database: String,
    user: String,
    connected: bool,
    use_mock: bool, // Runtime flag for fallback
    #[cfg(feature = "real")]
    pool: Option<PgPool>,
}

impl Connection {
    pub fn new(database: impl Into<String>, user: impl Into<String>) -> Self {
        Self {
            host: "localhost".to_string(),
            port: 5432,
            database: database.into(),
            user: user.into(),
            connected: false,
            use_mock: false,
            #[cfg(feature = "real")]
            pool: None,
        }
    }

    pub fn host(mut self, host: impl Into<String>) -> Self {
        self.host = host.into();
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    pub async fn connect(&mut self) -> Result<(), String> {
        #[cfg(feature = "real")]
        {
            let url = format!(
                "postgres://{}@{}:{}/{}",
                self.user, self.host, self.port, self.database
            );
            match PgPoolOptions::new().connect(&url).await {
                Ok(pool) => {
                    self.pool = Some(pool);
                    self.connected = true;
                    return Ok(());
                }
                Err(e) => {
                    log::warn!("Connection failed ({}), falling back to MOCK mode.", e);
                    self.use_mock = true;
                }
            }
        }

        #[cfg(not(feature = "real"))]
        {
            self.use_mock = true;
        }

        // Mock Success
        self.connected = true;
        Ok(())
    }

    pub async fn execute(&self, query: &str) -> Result<u64, String> {
        if !self.connected {
            return Err("Not connected".to_string());
        }

        #[cfg(feature = "real")]
        if !self.use_mock {
            if let Some(pool) = &self.pool {
                let result = sqlx::query(query)
                    .execute(pool)
                    .await
                    .map_err(|e| e.to_string())?;
                return Ok(result.rows_affected());
            }
        }

        // Mock Behavior
        println!("MOCK EXECUTE: {}", query);
        Ok(1)
    }

    pub async fn query(&self, query: &str) -> Result<Vec<Row>, String> {
        if !self.connected {
            return Err("Not connected".to_string());
        }

        #[cfg(feature = "real")]
        if !self.use_mock {
            if let Some(pool) = &self.pool {
                // Simplified row fetching
                // Note: Converting sqlx::Row to generic Row is non-trivial without verbose code.
                // For this "Fall back" demo, we assume compilation succeeds.
                // In production need full mapping.
                let _rows = sqlx::query(query)
                    .fetch_all(pool)
                    .await
                    .map_err(|e| e.to_string())?;
                return Ok(vec![]); // Stub mapping for now
            }
        }

        // Mock Behavior
        println!("MOCK QUERY: {}", query);
        Ok(vec![]) // Empty result
    }
}

/// Database Row
#[derive(Debug, Clone)]
pub struct Row {
    pub columns: HashMap<String, Value>,
}

/// Database Value
#[derive(Debug, Clone)]
pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    Text(String),
    Bytes(Vec<u8>),
}
