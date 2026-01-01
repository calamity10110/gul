use async_trait::async_trait;
use bytes::Bytes;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Permission denied")]
    PermissionDenied,
    #[error("Unknown error: {0}")]
    Unknown(String),
}

#[async_trait]
pub trait Storage: Send + Sync {
    async fn read(&self, path: &str) -> Result<Bytes, StorageError>;
    async fn write(&self, path: &str, content: Bytes) -> Result<(), StorageError>;
    async fn delete(&self, path: &str) -> Result<(), StorageError>;
    async fn list(&self, prefix: &str) -> Result<Vec<String>, StorageError>;
}

// --- Local File System Implementation ---

pub struct LocalStorage {
    root: PathBuf,
}

impl LocalStorage {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }
}

#[async_trait]
impl Storage for LocalStorage {
    async fn read(&self, path: &str) -> Result<Bytes, StorageError> {
        let full_path = self.root.join(path);
        let content = tokio::fs::read(full_path).await?;
        Ok(Bytes::from(content))
    }

    async fn write(&self, path: &str, content: Bytes) -> Result<(), StorageError> {
        let full_path = self.root.join(path);
        if let Some(parent) = full_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        tokio::fs::write(full_path, content).await?;
        Ok(())
    }

    async fn delete(&self, path: &str) -> Result<(), StorageError> {
        let full_path = self.root.join(path);
        tokio::fs::remove_file(full_path).await?;
        Ok(())
    }

    async fn list(&self, _prefix: &str) -> Result<Vec<String>, StorageError> {
        // Simplified list stub
        Ok(vec![])
    }
}

// --- AWS S3 Implementation (Gated) ---

#[cfg(feature = "aws")]
pub mod s3 {
    use super::*;
    use aws_config::meta::region::RegionProviderChain;
    use aws_sdk_s3::Client;

    pub struct S3Storage {
        client: Client,
        bucket: String,
    }

    impl S3Storage {
        pub async fn new(bucket: &str) -> Self {
            let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
            let config = aws_config::from_env().region(region_provider).load().await;
            let client = Client::new(&config);
            Self {
                client,
                bucket: bucket.to_string(),
            }
        }
    }

    #[async_trait]
    impl Storage for S3Storage {
        async fn read(&self, path: &str) -> Result<Bytes, StorageError> {
            let resp = self
                .client
                .get_object()
                .bucket(&self.bucket)
                .key(path)
                .send()
                .await
                .map_err(|e| StorageError::Unknown(e.to_string()))?;

            let data = resp
                .body
                .collect()
                .await
                .map_err(|e| StorageError::Unknown(e.to_string()))?;
            Ok(data.into_bytes())
        }

        async fn write(&self, path: &str, content: Bytes) -> Result<(), StorageError> {
            self.client
                .put_object()
                .bucket(&self.bucket)
                .key(path)
                .body(content.into())
                .send()
                .await
                .map_err(|e| StorageError::Unknown(e.to_string()))?;
            Ok(())
        }

        async fn delete(&self, path: &str) -> Result<(), StorageError> {
            self.client
                .delete_object()
                .bucket(&self.bucket)
                .key(path)
                .send()
                .await
                .map_err(|e| StorageError::Unknown(e.to_string()))?;
            Ok(())
        }

        async fn list(&self, _prefix: &str) -> Result<Vec<String>, StorageError> {
            // Simplified
            Ok(vec![])
        }
    }
}
