pub struct S3Client {
    region: String,
    access_key: String,
    secret_key: String,
}

impl S3Client {
    pub fn new(region: &str, access_key: &str, secret_key: &str) -> Self {
        Self {
            region: region.to_string(),
            access_key: access_key.to_string(),
            secret_key: secret_key.to_string(),
        }
    }

    pub fn put_object(&self, bucket: &str, key: &str, _data: &[u8]) -> Result<(), String> {
        // In a real implementation, this would sign the request and use reqwest to PUT
        println!(
            "MOCK: Uploading to s3://{}/{} in {}",
            bucket, key, self.region
        );
        Ok(())
    }

    pub fn get_object(&self, bucket: &str, key: &str) -> Result<Vec<u8>, String> {
        println!("MOCK: Downloading from s3://{}/{}", bucket, key);
        Ok(vec![0; 10]) // Mock data
    }
}
