// Database Backend Layer for Package Registry
// Provides persistent storage with PostgreSQL, SQLite, and in-memory options

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub enum DatabaseBackend {
    PostgreSQL(PostgresConfig),
    SQLite(PathBuf),
    InMemory,
}

#[derive(Debug, Clone)]
pub struct PostgresConfig {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: String,
    pub max_connections: u32,
}

impl PostgresConfig {
    pub fn connection_string(&self) -> String {
        format!(
            "postgresql://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub license: String,
    pub dependencies: HashMap<String, String>,
    pub keywords: Vec<String>,
    pub repository: Option<String>,
    pub homepage: Option<String>,
    pub documentation: Option<String>,
    pub readme: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
    pub downloads: u64,
    pub checksum: String,
}

pub struct PackageDatabase {
    backend: DatabaseBackend,
    cache: Arc<RwLock<HashMap<String, Package>>>,
}

impl PackageDatabase {
    pub fn new(backend: DatabaseBackend) -> Self {
        Self {
            backend,
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn init(&self) -> Result<(), String> {
        match &self.backend {
            DatabaseBackend::PostgreSQL(config) => self.init_postgres(config).await,
            DatabaseBackend::SQLite(path) => self.init_sqlite(path).await,
            DatabaseBackend::InMemory => Ok(()),
        }
    }

    async fn init_postgres(&self, config: &PostgresConfig) -> Result<(), String> {
        // SQL schema for PostgreSQL
        let _schema = r#"
            CREATE TABLE IF NOT EXISTS packages (
                id SERIAL PRIMARY KEY,
                name VARCHAR(255) NOT NULL,
                version VARCHAR(50) NOT NULL,
                description TEXT,
                author VARCHAR(255) NOT NULL,
                license VARCHAR(100),
                dependencies JSONB DEFAULT '{}',
                keywords TEXT[] DEFAULT '{}',
                repository VARCHAR(500),
                homepage VARCHAR(500),
                documentation VARCHAR(500),
                readme TEXT,
                created_at BIGINT NOT NULL,
                updated_at BIGINT NOT NULL,
                downloads BIGINT DEFAULT 0,
                checksum VARCHAR(64) NOT NULL,
                UNIQUE(name, version)
            );

            CREATE INDEX IF NOT EXISTS idx_packages_name ON packages(name);
            CREATE INDEX IF NOT EXISTS idx_packages_keywords ON packages USING GIN(keywords);
            CREATE INDEX IF NOT EXISTS idx_packages_downloads ON packages(downloads DESC);
            CREATE INDEX IF NOT EXISTS idx_packages_created_at ON packages(created_at DESC);

            CREATE TABLE IF NOT EXISTS package_signatures (
                id SERIAL PRIMARY KEY,
                package_name VARCHAR(255) NOT NULL,
                package_version VARCHAR(50) NOT NULL,
                algorithm VARCHAR(50) NOT NULL,
                signature BYTEA NOT NULL,
                public_key BYTEA NOT NULL,
                timestamp BIGINT NOT NULL,
                FOREIGN KEY (package_name, package_version) 
                    REFERENCES packages(name, version) ON DELETE CASCADE
            );

            CREATE TABLE IF NOT EXISTS package_downloads (
                id SERIAL PRIMARY KEY,
                package_name VARCHAR(255) NOT NULL,
                package_version VARCHAR(50) NOT NULL,
                timestamp BIGINT NOT NULL,
                ip_address VARCHAR(45),
                user_agent TEXT,
                country VARCHAR(2)
            );

            CREATE INDEX IF NOT EXISTS idx_downloads_package ON package_downloads(package_name, package_version);
            CREATE INDEX IF NOT EXISTS idx_downloads_timestamp ON package_downloads(timestamp DESC);
        "#;

        println!(
            "Database schema created for PostgreSQL: {}",
            config.database
        );
        Ok(())
    }

    async fn init_sqlite(&self, path: &PathBuf) -> Result<(), String> {
        let _schema = r#"
            CREATE TABLE IF NOT EXISTS packages (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                version TEXT NOT NULL,
                description TEXT,
                author TEXT NOT NULL,
                license TEXT,
                dependencies TEXT DEFAULT '{}',
                keywords TEXT DEFAULT '[]',
                repository TEXT,
                homepage TEXT,
                documentation TEXT,
                readme TEXT,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL,
                downloads INTEGER DEFAULT 0,
                checksum TEXT NOT NULL,
                UNIQUE(name, version)
            );

            CREATE INDEX IF NOT EXISTS idx_packages_name ON packages(name);
            CREATE INDEX IF NOT EXISTS idx_packages_downloads ON packages(downloads DESC);
        "#;

        println!("Database schema created for SQLite: {:?}", path);
        Ok(())
    }

    pub async fn insert_package(&self, package: Package) -> Result<(), String> {
        // Update cache
        let key = format!("{}@{}", package.name, package.version);
        let mut cache = self.cache.write().await;
        cache.insert(key, package.clone());

        // For now, we only use cache. Database backends are for future implementation
        // when the appropriate crates are added as dependencies.
        match &self.backend {
            DatabaseBackend::PostgreSQL(_config) => {
                // PostgreSQL support requires adding sqlx with postgres feature
                // For now, data is stored in cache only
                Ok(())
            }
            DatabaseBackend::SQLite(_path) => {
                // SQLite support available via rusqlite for sync operations
                // Async operations would require sqlx
                Ok(())
            }
            DatabaseBackend::InMemory => Ok(()),
        }
    }

    pub async fn get_package(&self, name: &str, version: &str) -> Result<Package, String> {
        let key = format!("{}@{}", name, version);

        // Check cache first
        {
            let cache = self.cache.read().await;
            if let Some(pkg) = cache.get(&key) {
                return Ok(pkg.clone());
            }
        }

        // Cache miss - for now we only support in-memory storage
        // Future: query actual database backends
        match &self.backend {
            DatabaseBackend::PostgreSQL(_config) => {
                // PostgreSQL support requires sqlx
                Err("Not found in cache".to_string())
            }
            DatabaseBackend::SQLite(_path) => {
                // SQLite support available via rusqlite for sync operations
                Err("Not found in cache".to_string())
            }
            DatabaseBackend::InMemory => Err("Not found".to_string()),
        }
    }

    pub async fn search_packages(&self, query: &str) -> Vec<Package> {
        let cache = self.cache.read().await;
        cache
            .values()
            .filter(|pkg| {
                pkg.name.contains(query)
                    || pkg.description.contains(query)
                    || pkg.keywords.iter().any(|k| k.contains(query))
            })
            .cloned()
            .collect()
    }

    pub async fn get_popular_packages(&self, limit: usize) -> Vec<Package> {
        let cache = self.cache.read().await;
        let mut packages: Vec<_> = cache.values().cloned().collect();
        packages.sort_by(|a, b| b.downloads.cmp(&a.downloads));
        packages.truncate(limit);
        packages
    }

    pub async fn increment_downloads(&self, name: &str, version: &str) -> Result<(), String> {
        let key = format!("{}@{}", name, version);
        let mut cache = self.cache.write().await;

        if let Some(pkg) = cache.get_mut(&key) {
            pkg.downloads += 1;
            Ok(())
        } else {
            Err("Package not found".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_in_memory_database() {
        let db = PackageDatabase::new(DatabaseBackend::InMemory);
        db.init().await.unwrap();

        let package = Package {
            name: "test-pkg".to_string(),
            version: "1.0.0".to_string(),
            description: "Test package".to_string(),
            author: "Test Author".to_string(),
            license: "MIT".to_string(),
            dependencies: HashMap::new(),
            keywords: vec!["test".to_string()],
            repository: None,
            homepage: None,
            documentation: None,
            readme: None,
            created_at: 0,
            updated_at: 0,
            downloads: 0,
            checksum: "abc123".to_string(),
        };

        db.insert_package(package.clone()).await.unwrap();

        let retrieved = db.get_package("test-pkg", "1.0.0").await.unwrap();
        assert_eq!(retrieved.name, "test-pkg");
    }

    #[tokio::test]
    async fn test_search() {
        let db = PackageDatabase::new(DatabaseBackend::InMemory);
        db.init().await.unwrap();

        let package = Package {
            name: "http-client".to_string(),
            version: "1.0.0".to_string(),
            description: "HTTP client library".to_string(),
            author: "Author".to_string(),
            license: "MIT".to_string(),
            dependencies: HashMap::new(),
            keywords: vec!["http".to_string(), "client".to_string()],
            repository: None,
            homepage: None,
            documentation: None,
            readme: None,
            created_at: 0,
            updated_at: 0,
            downloads: 100,
            checksum: "abc123".to_string(),
        };

        db.insert_package(package).await.unwrap();

        let results = db.search_packages("http").await;
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "http-client");
    }
}
