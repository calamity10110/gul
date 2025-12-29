pub struct CassandraClient {
    host: String,
}

impl CassandraClient {
    pub fn new(host: &str) -> Self {
        Self { host: host.to_string() }
    }

    pub async fn execute(&self, query: &str) -> Result<(), String> {
        // Mock
        Ok(())
    }
}
