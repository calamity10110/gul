pub struct GCSClient {
    bucket: String,
}

impl GCSClient {
    pub fn new(bucket: &str) -> Self {
        Self {
            bucket: bucket.to_string(),
        }
    }

    pub async fn upload(&self, object: &str, data: &[u8]) -> Result<(), String> {
        // Mock
        println!("Uploading to gs://{}/{}", self.bucket, object);
        Ok(())
    }
}
