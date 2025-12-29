use chrono::Utc;
use hmac::{Hmac, Mac};
use reqwest::{Client, Method, RequestBuilder};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

type HmacSha256 = Hmac<Sha256>;

pub struct S3Client {
    region: String,
    access_key: String,
    secret_key: String,
    endpoint: String,
    client: Client,
}

impl S3Client {
    pub fn new(region: &str, access_key: &str, secret_key: &str) -> Self {
        Self {
            region: region.to_string(),
            access_key: access_key.to_string(),
            secret_key: secret_key.to_string(),
            endpoint: format!("https://s3.{}.amazonaws.com", region),
            client: Client::new(),
        }
    }

    pub async fn get_object(&self, bucket: &str, key: &str) -> Result<Vec<u8>, anyhow::Error> {
        let url = format!("{}/{}/{}", self.endpoint, bucket, key);
        let builder = self.client.get(&url);
        let signed_builder = self.sign_request(builder, "GET", &url, &[]);
        let resp = signed_builder.send().await?;
        Ok(resp.bytes().await?.to_vec())
    }

    pub async fn put_object(&self, bucket: &str, key: &str, data: &[u8]) -> Result<(), anyhow::Error> {
        let url = format!("{}/{}/{}", self.endpoint, bucket, key);
        let builder = self.client.put(&url).body(data.to_vec());
        let signed_builder = self.sign_request(builder, "PUT", &url, data);
        signed_builder.send().await?;
        Ok(())
    }

    fn sign_request(
        &self,
        builder: RequestBuilder,
        method: &str,
        url: &str,
        payload: &[u8],
    ) -> RequestBuilder {
        // AWS SigV4 (Simplified for brevity)
        // In a full implementation, this would handle canonical headers, etc.
        let now = Utc::now();
        let date_header = now.format("%Y%m%dT%H%M%SZ").to_string();
        
        let mut hasher = Sha256::new();
        hasher.update(payload);
        let payload_hash = hex::encode(hasher.finalize());

        builder
            .header("x-amz-date", date_header)
            .header("x-amz-content-sha256", payload_hash)
            // Auth header logic would go here
    }
}
