pub struct AzureBlobClient {
    account: String,
    container: String,
}

impl AzureBlobClient {
    pub fn new(account: &str, container: &str) -> Self {
        Self {
            account: account.to_string(),
            container: container.to_string(),
        }
    }

    pub async fn put_blob(&self, blob: &str, data: &[u8]) -> Result<(), String> {
        // Mock
        Ok(())
    }
}
