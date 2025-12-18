// Package Support Module for GUL
// Provides integration with popular frameworks and libraries across multiple languages

use std::collections::HashMap;

/// Package metadata and integration information
#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,
    pub language: String,
    pub version: String,
    pub description: String,
    pub dependencies: Vec<String>,
    pub features: Vec<String>,
}

/// Package manager for GUL
pub struct PackageManager {
    packages: HashMap<String, Package>,
}

impl PackageManager {
    pub fn new() -> Self {
        let mut manager = PackageManager {
            packages: HashMap::new(),
        };
        manager.register_default_packages();
        manager
    }

    /// Register all default supported packages
    fn register_default_packages(&mut self) {
        // Rust packages
        self.register_axum();
        self.register_tokio();
        self.register_serde();
        self.register_dioxus();
        self.register_rustea();
        self.register_tauri();
        self.register_leptos();

        // Batch 1: New Rust packages
        self.register_actix_web();
        self.register_sqlx();
        self.register_regex();
        self.register_rayon();
        self.register_diesel();

        // Batch 2: More Rust packages
        self.register_warp();
        self.register_tracing();
        self.register_anyhow();

        // Python packages
        self.register_django();
        self.register_flask();
        self.register_fastapi();
        self.register_pydantic();
        self.register_numpy();
        self.register_pandas();

        // JavaScript packages
        self.register_react();
        self.register_angular();
        self.register_vue();
        self.register_nodejs();
        self.register_express();
        self.register_d3js();

        // Batch 3: New JavaScript packages
        self.register_nextjs();
        self.register_svelte();
        self.register_prisma();
        self.register_jest();
        self.register_webpack();
        self.register_tailwindcss();
        self.register_axios();
        self.register_socketio();
        self.register_graphql();
        self.register_nestjs();

        // Multi-language support
        self.register_cpp_support();
        self.register_java_support();
        self.register_go_support();
        self.register_csharp_support();
        self.register_typescript_support();
        self.register_ruby_support();

        // Database support
        self.register_database_support();
    }

    // Rust Framework Registrations

    fn register_axum(&mut self) {
        self.packages.insert(
            "axum".to_string(),
            Package {
                name: "axum".to_string(),
                language: "rust".to_string(),
                version: "0.7".to_string(),
                description:
                    "Ergonomic and modular web framework built with Tokio, Tower, and Hyper"
                        .to_string(),
                dependencies: vec![
                    "tokio".to_string(),
                    "tower".to_string(),
                    "hyper".to_string(),
                ],
                features: vec![
                    "routing".to_string(),
                    "extractors".to_string(),
                    "middleware".to_string(),
                    "websockets".to_string(),
                ],
            },
        );
    }

    fn register_tokio(&mut self) {
        self.packages.insert(
            "tokio".to_string(),
            Package {
                name: "tokio".to_string(),
                language: "rust".to_string(),
                version: "1.0".to_string(),
                description: "Asynchronous runtime for Rust".to_string(),
                dependencies: vec![],
                features: vec![
                    "async_runtime".to_string(),
                    "io".to_string(),
                    "net".to_string(),
                    "time".to_string(),
                    "sync".to_string(),
                ],
            },
        );
    }

    fn register_serde(&mut self) {
        self.packages.insert(
            "serde".to_string(),
            Package {
                name: "serde".to_string(),
                language: "rust".to_string(),
                version: "1.0".to_string(),
                description: "Serialization framework for Rust".to_string(),
                dependencies: vec![],
                features: vec![
                    "serialize".to_string(),
                    "deserialize".to_string(),
                    "json".to_string(),
                    "derive".to_string(),
                ],
            },
        );
    }

    fn register_dioxus(&mut self) {
        self.packages.insert(
            "dioxus".to_string(),
            Package {
                name: "dioxus".to_string(),
                language: "rust".to_string(),
                version: "0.5".to_string(),
                description: "Portable, performant, and ergonomic framework for building cross-platform user interfaces in Rust".to_string(),
                dependencies: vec![],
                features: vec![
                    "web".to_string(),
                    "desktop".to_string(),
                    "mobile".to_string(),
                    "ssr".to_string(),
                ],
            },
        );
    }

    fn register_rustea(&mut self) {
        self.packages.insert(
            "rustea".to_string(),
            Package {
                name: "rustea".to_string(),
                language: "rust".to_string(),
                version: "0.1".to_string(),
                description: "TUI framework with crossterm and ratatui support".to_string(),
                dependencies: vec!["crossterm".to_string(), "ratatui".to_string()],
                features: vec![
                    "tui".to_string(),
                    "terminal".to_string(),
                    "widgets".to_string(),
                    "events".to_string(),
                ],
            },
        );
    }

    fn register_tauri(&mut self) {
        self.packages.insert(
            "tauri".to_string(),
            Package {
                name: "tauri".to_string(),
                language: "rust".to_string(),
                version: "2.0".to_string(),
                description: "Build smaller, faster, and more secure desktop applications"
                    .to_string(),
                dependencies: vec![],
                features: vec![
                    "desktop".to_string(),
                    "webview".to_string(),
                    "native_api".to_string(),
                    "system_tray".to_string(),
                ],
            },
        );
    }

    fn register_leptos(&mut self) {
        self.packages.insert(
            "leptos".to_string(),
            Package {
                name: "leptos".to_string(),
                language: "rust".to_string(),
                version: "0.6".to_string(),
                description: "Full-stack, isomorphic Rust web framework".to_string(),
                dependencies: vec![],
                features: vec![
                    "ssr".to_string(),
                    "csr".to_string(),
                    "reactive".to_string(),
                    "routing".to_string(),
                ],
            },
        );
    }

    // Python Framework Registrations

    fn register_django(&mut self) {
        self.packages.insert(
            "django".to_string(),
            Package {
                name: "django".to_string(),
                language: "python".to_string(),
                version: "5.0".to_string(),
                description: "High-level Python web framework".to_string(),
                dependencies: vec![],
                features: vec![
                    "orm".to_string(),
                    "admin".to_string(),
                    "auth".to_string(),
                    "templates".to_string(),
                ],
            },
        );
    }

    fn register_flask(&mut self) {
        self.packages.insert(
            "flask".to_string(),
            Package {
                name: "flask".to_string(),
                language: "python".to_string(),
                version: "3.0".to_string(),
                description: "Lightweight WSGI web application framework".to_string(),
                dependencies: vec![],
                features: vec![
                    "routing".to_string(),
                    "templates".to_string(),
                    "sessions".to_string(),
                    "blueprints".to_string(),
                ],
            },
        );
    }

    fn register_fastapi(&mut self) {
        self.packages.insert(
            "fastapi".to_string(),
            Package {
                name: "fastapi".to_string(),
                language: "python".to_string(),
                version: "0.110".to_string(),
                description: "Modern, fast web framework for building APIs with Python".to_string(),
                dependencies: vec!["pydantic".to_string()],
                features: vec![
                    "async".to_string(),
                    "validation".to_string(),
                    "openapi".to_string(),
                    "dependency_injection".to_string(),
                ],
            },
        );
    }

    fn register_pydantic(&mut self) {
        self.packages.insert(
            "pydantic".to_string(),
            Package {
                name: "pydantic".to_string(),
                language: "python".to_string(),
                version: "2.0".to_string(),
                description: "Data validation using Python type annotations".to_string(),
                dependencies: vec![],
                features: vec![
                    "validation".to_string(),
                    "serialization".to_string(),
                    "json_schema".to_string(),
                ],
            },
        );
    }

    fn register_numpy(&mut self) {
        self.packages.insert(
            "numpy".to_string(),
            Package {
                name: "numpy".to_string(),
                language: "python".to_string(),
                version: "1.26".to_string(),
                description: "Fundamental package for scientific computing with Python".to_string(),
                dependencies: vec![],
                features: vec![
                    "arrays".to_string(),
                    "linear_algebra".to_string(),
                    "fft".to_string(),
                    "random".to_string(),
                ],
            },
        );
    }

    fn register_pandas(&mut self) {
        self.packages.insert(
            "pandas".to_string(),
            Package {
                name: "pandas".to_string(),
                language: "python".to_string(),
                version: "2.2".to_string(),
                description: "Powerful data structures for data analysis".to_string(),
                dependencies: vec!["numpy".to_string()],
                features: vec![
                    "dataframes".to_string(),
                    "series".to_string(),
                    "io".to_string(),
                    "groupby".to_string(),
                ],
            },
        );
    }

    // JavaScript Framework Registrations

    fn register_react(&mut self) {
        self.packages.insert(
            "react".to_string(),
            Package {
                name: "react".to_string(),
                language: "javascript".to_string(),
                version: "18.0".to_string(),
                description: "JavaScript library for building user interfaces".to_string(),
                dependencies: vec![],
                features: vec![
                    "components".to_string(),
                    "hooks".to_string(),
                    "jsx".to_string(),
                    "virtual_dom".to_string(),
                ],
            },
        );
    }

    fn register_angular(&mut self) {
        self.packages.insert(
            "angular".to_string(),
            Package {
                name: "angular".to_string(),
                language: "javascript".to_string(),
                version: "17.0".to_string(),
                description: "Platform for building mobile and desktop web applications"
                    .to_string(),
                dependencies: vec!["typescript".to_string()],
                features: vec![
                    "components".to_string(),
                    "dependency_injection".to_string(),
                    "routing".to_string(),
                    "forms".to_string(),
                ],
            },
        );
    }

    fn register_vue(&mut self) {
        self.packages.insert(
            "vue".to_string(),
            Package {
                name: "vue".to_string(),
                language: "javascript".to_string(),
                version: "3.4".to_string(),
                description: "Progressive JavaScript framework for building user interfaces"
                    .to_string(),
                dependencies: vec![],
                features: vec![
                    "reactive".to_string(),
                    "components".to_string(),
                    "composition_api".to_string(),
                    "router".to_string(),
                ],
            },
        );
    }

    fn register_nodejs(&mut self) {
        self.packages.insert(
            "nodejs".to_string(),
            Package {
                name: "nodejs".to_string(),
                language: "javascript".to_string(),
                version: "20.0".to_string(),
                description: "JavaScript runtime built on Chrome's V8 engine".to_string(),
                dependencies: vec![],
                features: vec![
                    "async".to_string(),
                    "fs".to_string(),
                    "http".to_string(),
                    "streams".to_string(),
                ],
            },
        );
    }

    fn register_express(&mut self) {
        self.packages.insert(
            "express".to_string(),
            Package {
                name: "express".to_string(),
                language: "javascript".to_string(),
                version: "4.18".to_string(),
                description: "Fast, unopinionated, minimalist web framework for Node.js"
                    .to_string(),
                dependencies: vec!["nodejs".to_string()],
                features: vec![
                    "routing".to_string(),
                    "middleware".to_string(),
                    "templating".to_string(),
                ],
            },
        );
    }

    fn register_d3js(&mut self) {
        self.packages.insert(
            "d3".to_string(),
            Package {
                name: "d3".to_string(),
                language: "javascript".to_string(),
                version: "7.0".to_string(),
                description: "JavaScript library for manipulating documents based on data"
                    .to_string(),
                dependencies: vec![],
                features: vec![
                    "visualization".to_string(),
                    "svg".to_string(),
                    "scales".to_string(),
                    "transitions".to_string(),
                ],
            },
        );
    }

    // Multi-language Support

    fn register_cpp_support(&mut self) {
        self.packages.insert(
            "cpp_std".to_string(),
            Package {
                name: "cpp_std".to_string(),
                language: "cpp".to_string(),
                version: "20".to_string(),
                description: "C++ Standard Library support".to_string(),
                dependencies: vec![],
                features: vec![
                    "stl".to_string(),
                    "templates".to_string(),
                    "smart_pointers".to_string(),
                ],
            },
        );
    }

    fn register_java_support(&mut self) {
        self.packages.insert(
            "java_jdk".to_string(),
            Package {
                name: "java_jdk".to_string(),
                language: "java".to_string(),
                version: "21".to_string(),
                description: "Java Development Kit support".to_string(),
                dependencies: vec![],
                features: vec![
                    "collections".to_string(),
                    "streams".to_string(),
                    "concurrency".to_string(),
                ],
            },
        );
    }

    fn register_go_support(&mut self) {
        self.packages.insert(
            "go_std".to_string(),
            Package {
                name: "go_std".to_string(),
                language: "go".to_string(),
                version: "1.22".to_string(),
                description: "Go standard library support".to_string(),
                dependencies: vec![],
                features: vec![
                    "goroutines".to_string(),
                    "channels".to_string(),
                    "http".to_string(),
                ],
            },
        );
    }

    fn register_csharp_support(&mut self) {
        self.packages.insert(
            "dotnet".to_string(),
            Package {
                name: "dotnet".to_string(),
                language: "csharp".to_string(),
                version: "8.0".to_string(),
                description: ".NET runtime and libraries support".to_string(),
                dependencies: vec![],
                features: vec![
                    "linq".to_string(),
                    "async".to_string(),
                    "collections".to_string(),
                ],
            },
        );
    }

    fn register_typescript_support(&mut self) {
        self.packages.insert(
            "typescript".to_string(),
            Package {
                name: "typescript".to_string(),
                language: "typescript".to_string(),
                version: "5.3".to_string(),
                description: "Typed superset of JavaScript".to_string(),
                dependencies: vec![],
                features: vec![
                    "types".to_string(),
                    "interfaces".to_string(),
                    "generics".to_string(),
                ],
            },
        );
    }

    fn register_ruby_support(&mut self) {
        self.packages.insert(
            "ruby_std".to_string(),
            Package {
                name: "ruby_std".to_string(),
                language: "ruby".to_string(),
                version: "3.3".to_string(),
                description: "Ruby standard library support".to_string(),
                dependencies: vec![],
                features: vec![
                    "gems".to_string(),
                    "metaprogramming".to_string(),
                    "blocks".to_string(),
                ],
            },
        );
    }

    fn register_database_support(&mut self) {
        // PostgreSQL
        self.packages.insert(
            "postgresql".to_string(),
            Package {
                name: "postgresql".to_string(),
                language: "sql".to_string(),
                version: "16.0".to_string(),
                description: "PostgreSQL database support".to_string(),
                dependencies: vec![],
                features: vec![
                    "acid".to_string(),
                    "json".to_string(),
                    "full_text_search".to_string(),
                ],
            },
        );

        // MySQL
        self.packages.insert(
            "mysql".to_string(),
            Package {
                name: "mysql".to_string(),
                language: "sql".to_string(),
                version: "8.0".to_string(),
                description: "MySQL database support".to_string(),
                dependencies: vec![],
                features: vec!["transactions".to_string(), "replication".to_string()],
            },
        );

        // SQLite
        self.packages.insert(
            "sqlite".to_string(),
            Package {
                name: "sqlite".to_string(),
                language: "sql".to_string(),
                version: "3.45".to_string(),
                description: "SQLite embedded database support".to_string(),
                dependencies: vec![],
                features: vec!["embedded".to_string(), "zero_config".to_string()],
            },
        );

        // MongoDB
        self.packages.insert(
            "mongodb".to_string(),
            Package {
                name: "mongodb".to_string(),
                language: "nosql".to_string(),
                version: "7.0".to_string(),
                description: "MongoDB NoSQL database support".to_string(),
                dependencies: vec![],
                features: vec![
                    "document".to_string(),
                    "aggregation".to_string(),
                    "sharding".to_string(),
                ],
            },
        );

        // Redis
        self.packages.insert(
            "redis".to_string(),
            Package {
                name: "redis".to_string(),
                language: "nosql".to_string(),
                version: "7.2".to_string(),
                description: "Redis in-memory data structure store".to_string(),
                dependencies: vec![],
                features: vec![
                    "cache".to_string(),
                    "pub_sub".to_string(),
                    "streams".to_string(),
                ],
            },
        );
    }

    // Public API methods

    pub fn get_package(&self, name: &str) -> Option<&Package> {
        self.packages.get(name)
    }

    pub fn list_packages(&self) -> Vec<&Package> {
        self.packages.values().collect()
    }

    pub fn list_packages_by_language(&self, language: &str) -> Vec<&Package> {
        self.packages
            .values()
            .filter(|p| p.language == language)
            .collect()
    }

    pub fn has_package(&self, name: &str) -> bool {
        self.packages.contains_key(name)
    }

    pub fn get_package_count(&self) -> usize {
        self.packages.len()
    }

    // Batch 1: New Rust Packages
    fn register_actix_web(&mut self) {
        self.packages.insert(
            "actix-web".to_string(),
            Package {
                name: "actix-web".to_string(),
                language: "rust".to_string(),
                version: "4.5".to_string(),
                description: "Powerful, pragmatic, and extremely fast web framework".to_string(),
                dependencies: vec!["tokio".to_string()],
                features: vec!["routing".to_string(), "middleware".to_string()],
            },
        );
    }

    fn register_sqlx(&mut self) {
        self.packages.insert(
            "sqlx".to_string(),
            Package {
                name: "sqlx".to_string(),
                language: "rust".to_string(),
                version: "0.7".to_string(),
                description: "Async SQL toolkit with compile-time checked queries".to_string(),
                dependencies: vec!["tokio".to_string()],
                features: vec!["postgres".to_string(), "mysql".to_string()],
            },
        );
    }

    fn register_regex(&mut self) {
        self.packages.insert(
            "regex".to_string(),
            Package {
                name: "regex".to_string(),
                language: "rust".to_string(),
                version: "1.10".to_string(),
                description: "Regular expressions for Rust".to_string(),
                dependencies: vec![],
                features: vec!["unicode".to_string()],
            },
        );
    }

    fn register_rayon(&mut self) {
        self.packages.insert(
            "rayon".to_string(),
            Package {
                name: "rayon".to_string(),
                language: "rust".to_string(),
                version: "1.8".to_string(),
                description: "Data parallelism library for Rust".to_string(),
                dependencies: vec![],
                features: vec!["parallel-iterators".to_string()],
            },
        );
    }

    fn register_diesel(&mut self) {
        self.packages.insert(
            "diesel".to_string(),
            Package {
                name: "diesel".to_string(),
                language: "rust".to_string(),
                version: "2.1".to_string(),
                description: "Safe, extensible ORM and query builder".to_string(),
                dependencies: vec![],
                features: vec!["postgres".to_string(), "mysql".to_string()],
            },
        );
    }

    // Batch 2: More Rust Packages
    fn register_warp(&mut self) {
        self.packages.insert(
            "warp".to_string(),
            Package {
                name: "warp".to_string(),
                language: "rust".to_string(),
                version: "0.3".to_string(),
                description: "Composable web framework with filters".to_string(),
                dependencies: vec!["tokio".to_string()],
                features: vec!["filters".to_string(), "websockets".to_string()],
            },
        );
    }

    fn register_tracing(&mut self) {
        self.packages.insert(
            "tracing".to_string(),
            Package {
                name: "tracing".to_string(),
                language: "rust".to_string(),
                version: "0.1".to_string(),
                description: "Application-level tracing for Rust".to_string(),
                dependencies: vec![],
                features: vec!["async".to_string(), "log".to_string()],
            },
        );
    }

    fn register_anyhow(&mut self) {
        self.packages.insert(
            "anyhow".to_string(),
            Package {
                name: "anyhow".to_string(),
                language: "rust".to_string(),
                version: "1.0".to_string(),
                description: "Flexible error handling library".to_string(),
                dependencies: vec![],
                features: vec!["backtrace".to_string(), "std".to_string()],
            },
        );
    }

    // Batch 3: JavaScript Packages
    fn register_nextjs(&mut self) {
        self.packages.insert(
            "next.js".to_string(),
            Package {
                name: "next.js".to_string(),
                language: "javascript".to_string(),
                version: "14.1".to_string(),
                description: "React framework for production".to_string(),
                dependencies: vec!["react".to_string()],
                features: vec!["ssr".to_string(), "routing".to_string()],
            },
        );
    }

    fn register_svelte(&mut self) {
        self.packages.insert(
            "svelte".to_string(),
            Package {
                name: "svelte".to_string(),
                language: "javascript".to_string(),
                version: "4.2".to_string(),
                description: "Cybernetically enhanced web apps".to_string(),
                dependencies: vec![],
                features: vec!["reactive".to_string(), "compiler".to_string()],
            },
        );
    }

    fn register_prisma(&mut self) {
        self.packages.insert(
            "prisma".to_string(),
            Package {
                name: "prisma".to_string(),
                language: "javascript".to_string(),
                version: "5.9".to_string(),
                description: "Next-generation ORM for Node.js and TypeScript".to_string(),
                dependencies: vec![],
                features: vec!["migrations".to_string(), "type-safety".to_string()],
            },
        );
    }

    fn register_jest(&mut self) {
        self.packages.insert(
            "jest".to_string(),
            Package {
                name: "jest".to_string(),
                language: "javascript".to_string(),
                version: "29.7".to_string(),
                description: "Delightful JavaScript testing framework".to_string(),
                dependencies: vec![],
                features: vec!["snapshots".to_string(), "mocking".to_string()],
            },
        );
    }

    fn register_webpack(&mut self) {
        self.packages.insert(
            "webpack".to_string(),
            Package {
                name: "webpack".to_string(),
                language: "javascript".to_string(),
                version: "5.90".to_string(),
                description: "Module bundler for modern JavaScript applications".to_string(),
                dependencies: vec![],
                features: vec!["bundling".to_string(), "code-splitting".to_string()],
            },
        );
    }

    fn register_tailwindcss(&mut self) {
        self.packages.insert(
            "tailwindcss".to_string(),
            Package {
                name: "tailwindcss".to_string(),
                language: "javascript".to_string(),
                version: "3.4".to_string(),
                description: "Utility-first CSS framework".to_string(),
                dependencies: vec![],
                features: vec!["jit".to_string(), "dark-mode".to_string()],
            },
        );
    }

    fn register_axios(&mut self) {
        self.packages.insert(
            "axios".to_string(),
            Package {
                name: "axios".to_string(),
                language: "javascript".to_string(),
                version: "1.6".to_string(),
                description: "Promise-based HTTP client".to_string(),
                dependencies: vec![],
                features: vec!["interceptors".to_string(), "cancellation".to_string()],
            },
        );
    }

    fn register_socketio(&mut self) {
        self.packages.insert(
            "socket.io".to_string(),
            Package {
                name: "socket.io".to_string(),
                language: "javascript".to_string(),
                version: "4.6".to_string(),
                description: "Real-time bidirectional event-based communication".to_string(),
                dependencies: vec![],
                features: vec!["websockets".to_string(), "rooms".to_string()],
            },
        );
    }

    fn register_graphql(&mut self) {
        self.packages.insert(
            "graphql".to_string(),
            Package {
                name: "graphql".to_string(),
                language: "javascript".to_string(),
                version: "16.8".to_string(),
                description: "Query language for APIs".to_string(),
                dependencies: vec![],
                features: vec!["schema".to_string(), "resolvers".to_string()],
            },
        );
    }

    fn register_nestjs(&mut self) {
        self.packages.insert(
            "nestjs".to_string(),
            Package {
                name: "nestjs".to_string(),
                language: "javascript".to_string(),
                version: "10.3".to_string(),
                description:
                    "Progressive Node.js framework for building efficient server-side applications"
                        .to_string(),
                dependencies: vec!["express".to_string()],
                features: vec!["dependency-injection".to_string(), "modules".to_string()],
            },
        );
    }
}

impl Default for PackageManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_package_manager_creation() {
        let pm = PackageManager::new();
        assert!(
            pm.get_package_count() > 0,
            "Should have registered packages"
        );
    }

    #[test]
    fn test_rust_packages() {
        let pm = PackageManager::new();
        assert!(pm.has_package("axum"));
        assert!(pm.has_package("tokio"));
        assert!(pm.has_package("serde"));
        assert!(pm.has_package("dioxus"));
        assert!(pm.has_package("tauri"));
        assert!(pm.has_package("leptos"));
    }

    #[test]
    fn test_python_packages() {
        let pm = PackageManager::new();
        assert!(pm.has_package("django"));
        assert!(pm.has_package("flask"));
        assert!(pm.has_package("fastapi"));
        assert!(pm.has_package("numpy"));
        assert!(pm.has_package("pandas"));
    }

    #[test]
    fn test_javascript_packages() {
        let pm = PackageManager::new();
        assert!(pm.has_package("react"));
        assert!(pm.has_package("angular"));
        assert!(pm.has_package("vue"));
        assert!(pm.has_package("nodejs"));
        assert!(pm.has_package("express"));
        assert!(pm.has_package("d3"));
    }

    #[test]
    fn test_database_support() {
        let pm = PackageManager::new();
        assert!(pm.has_package("postgresql"));
        assert!(pm.has_package("mysql"));
        assert!(pm.has_package("sqlite"));
        assert!(pm.has_package("mongodb"));
        assert!(pm.has_package("redis"));
    }

    #[test]
    fn test_list_by_language() {
        let pm = PackageManager::new();
        let rust_packages = pm.list_packages_by_language("rust");
        assert!(
            rust_packages.len() >= 7,
            "Should have multiple Rust packages"
        );
    }
}
