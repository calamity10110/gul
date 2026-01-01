// Build Cache for Package Registry
// Provides pre-built binary caching with CDN support

use serde::{Deserialize, Serialize};

use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Target {
    pub os: String,      // linux, macos, windows
    pub arch: String,    // x86_64, aarch64, arm
    pub variant: String, // gnu, musl, msvc
}

impl Target {
    pub fn new(os: impl Into<String>, arch: impl Into<String>, variant: impl Into<String>) -> Self {
        Self {
            os: os.into(),
            arch: arch.into(),
            variant: variant.into(),
        }
    }

    pub fn cache_key(&self) -> String {
        format!("{}-{}-{}", self.os, self.arch, self.variant)
    }

    pub fn linux_x86_64_gnu() -> Self {
        Self::new("linux", "x86_64", "gnu")
    }

    pub fn linux_aarch64_gnu() -> Self {
        Self::new("linux", "aarch64", "gnu")
    }

    pub fn macos_x86_64() -> Self {
        Self::new("macos", "x86_64", "")
    }

    pub fn macos_aarch64() -> Self {
        Self::new("macos", "aarch64", "")
    }

    pub fn windows_x86_64_msvc() -> Self {
        Self::new("windows", "x86_64", "msvc")
    }
}

#[derive(Debug, Clone)]
pub struct Binary {
    pub data: Vec<u8>,
    pub checksum: String,
    pub size: usize,
}

pub struct BuildCache {
    // In production: S3 client or CDN
    cache_dir: PathBuf,
    cdn_url: Option<String>,
    supported_targets: Vec<Target>,
}

impl BuildCache {
    pub fn new(cache_dir: PathBuf) -> Self {
        Self {
            cache_dir,
            cdn_url: None,
            supported_targets: vec![
                Target::linux_x86_64_gnu(),
                Target::linux_aarch64_gnu(),
                Target::macos_x86_64(),
                Target::macos_aarch64(),
                Target::windows_x86_64_msvc(),
            ],
        }
    }

    pub fn with_cdn(mut self, cdn_url: String) -> Self {
        self.cdn_url = Some(cdn_url);
        self
    }

    /// Get cache key for package and target
    fn cache_key(&self, package: &str, version: &str, target: &Target) -> String {
        format!("{}-{}-{}", package, version, target.cache_key())
    }

    /// Check if binary is cached
    pub async fn has_cached(&self, package: &str, version: &str, target: &Target) -> bool {
        let key = self.cache_key(package, version, target);
        let path = self.cache_dir.join(&key);
        path.exists()
    }

    /// Get cached binary
    pub async fn get_cached(
        &self,
        package: &str,
        version: &str,
        target: &Target,
    ) -> Result<Binary, String> {
        let key = self.cache_key(package, version, target);

        // In production: Check CDN first
        if let Some(cdn_url) = &self.cdn_url {
            let url = format!("{}/{}", cdn_url, key);
            // Download from CDN
            println!("Would download from CDN: {}", url);
        }

        // Check local cache
        let path = self.cache_dir.join(&key);
        if path.exists() {
            // Read binary
            Ok(Binary {
                data: vec![], // Placeholder
                checksum: "abc123".to_string(),
                size: 0,
            })
        } else {
            Err(format!("Binary not found in cache: {}", key))
        }
    }

    /// Cache a binary
    pub async fn cache_binary(
        &self,
        package: &str,
        version: &str,
        target: &Target,
        binary: Binary,
    ) -> Result<(), String> {
        let key = self.cache_key(package, version, target);

        // In production: Upload to S3/CDN
        if let Some(cdn_url) = &self.cdn_url {
            let url = format!("{}/{}", cdn_url, key);
            println!("Would upload to CDN: {}", url);
        }

        // Save to local cache
        let path = self.cache_dir.join(&key);
        std::fs::create_dir_all(&self.cache_dir).map_err(|e| e.to_string())?;
        std::fs::write(path, &binary.data).map_err(|e| e.to_string())?;

        Ok(())
    }

    /// Get CDN URL for binary
    pub fn get_cdn_url(&self, package: &str, version: &str, target: &Target) -> Option<String> {
        self.cdn_url
            .as_ref()
            .map(|cdn| format!("{}/{}", cdn, self.cache_key(package, version, target)))
    }

    /// List supported targets
    pub fn supported_targets(&self) -> &[Target] {
        &self.supported_targets
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[tokio::test]
    async fn test_cache_key() {
        let target = Target::linux_x86_64_gnu();
        assert_eq!(target.cache_key(), "linux-x86_64-gnu");
    }

    #[tokio::test]
    async fn test_build_cache() {
        let temp_dir = env::temp_dir().join("gul-test-cache");
        let cache = BuildCache::new(temp_dir.clone());

        let target = Target::linux_x86_64_gnu();
        let binary = Binary {
            data: vec![1, 2, 3, 4],
            checksum: "test123".to_string(),
            size: 4,
        };

        // Cache binary
        cache
            .cache_binary("test-pkg", "1.0.0", &target, binary)
            .await
            .unwrap();

        // Check if cached
        assert!(cache.has_cached("test-pkg", "1.0.0", &target).await);

        // Clean up
        std::fs::remove_dir_all(temp_dir).ok();
    }

    #[test]
    fn test_supported_targets() {
        let cache = BuildCache::new(PathBuf::from("/tmp"));
        let targets = cache.supported_targets();

        assert!(targets.len() >= 5);
        assert!(targets.contains(&Target::linux_x86_64_gnu()));
        assert!(targets.contains(&Target::macos_aarch64()));
    }

    #[test]
    fn test_cdn_url() {
        let cache =
            BuildCache::new(PathBuf::from("/tmp")).with_cdn("https://cdn.example.com".to_string());

        let target = Target::linux_x86_64_gnu();
        let url = cache.get_cdn_url("pkg", "1.0.0", &target);

        assert!(url.is_some());
        assert!(url.unwrap().contains("cdn.example.com"));
    }
}
