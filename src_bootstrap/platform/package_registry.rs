// Package Registry for GUL
// Provides package management, search, dependency resolution, signing, and vulnerability scanning

use std::collections::HashMap;
use std::time::SystemTime;

/// Package registry database schema
#[derive(Debug, Clone)]
pub struct PackageRegistry {
    packages: HashMap<String, Package>,
    users: HashMap<String, User>,
    downloads: HashMap<String, u64>,
}

#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub license: String,
    pub repository: Option<String>,
    pub homepage: Option<String>,
    pub keywords: Vec<String>,
    pub dependencies: HashMap<String, String>,
    pub dev_dependencies: HashMap<String, String>,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
    pub downloads: u64,
    pub signature: Option<String>,
    pub checksum: String,
}

#[derive(Debug, Clone)]
pub struct User {
    pub username: String,
    pub email: String,
    pub api_key: String,
    pub packages: Vec<String>,
    pub created_at: SystemTime,
}

impl PackageRegistry {
    pub fn new() -> Self {
        PackageRegistry {
            packages: HashMap::new(),
            users: HashMap::new(),
            downloads: HashMap::new(),
        }
    }

    /// Upload a package
    pub fn upload_package(&mut self, package: Package, user: &str) -> Result<(), String> {
        // Verify user exists
        if !self.users.contains_key(user) {
            return Err("User not found".to_string());
        }

        // Verify signature if present
        if let Some(sig) = &package.signature {
            if !self.verify_signature(&package, sig) {
                return Err("Invalid package signature".to_string());
            }
        }

        // Check for vulnerabilities
        if let Err(e) = self.scan_vulnerabilities(&package) {
            return Err(format!("Vulnerability detected: {}", e));
        }

        // Store package
        let key = format!("{}@{}", package.name, package.version);
        self.packages.insert(key.clone(), package.clone());
        self.downloads.insert(key, 0);

        // Update user's package list
        if let Some(user_data) = self.users.get_mut(user) {
            user_data.packages.push(package.name);
        }

        Ok(())
    }

    /// Download a package
    pub fn download_package(&mut self, name: &str, version: &str) -> Result<Package, String> {
        let key = format!("{}@{}", name, version);

        match self.packages.get(&key) {
            Some(package) => {
                // Increment download count
                if let Some(count) = self.downloads.get_mut(&key) {
                    *count += 1;
                }
                Ok(package.clone())
            }
            None => Err(format!("Package {}@{} not found", name, version)),
        }
    }

    /// Semantic search for packages
    pub fn search(&self, query: &str) -> Vec<&Package> {
        let query_lower = query.to_lowercase();

        self.packages
            .values()
            .filter(|pkg| {
                pkg.name.to_lowercase().contains(&query_lower)
                    || pkg.description.to_lowercase().contains(&query_lower)
                    || pkg
                        .keywords
                        .iter()
                        .any(|k| k.to_lowercase().contains(&query_lower))
            })
            .collect()
    }

    /// Search by keywords
    pub fn search_by_keywords(&self, keywords: &[String]) -> Vec<&Package> {
        self.packages
            .values()
            .filter(|pkg| {
                keywords.iter().any(|kw| {
                    pkg.keywords
                        .iter()
                        .any(|pkg_kw| pkg_kw.to_lowercase() == kw.to_lowercase())
                })
            })
            .collect()
    }

    /// Resolve dependencies
    pub fn resolve_dependencies(
        &self,
        package_name: &str,
        version: &str,
    ) -> Result<Vec<String>, String> {
        let key = format!("{}@{}", package_name, version);

        match self.packages.get(&key) {
            Some(package) => {
                let mut resolved = Vec::new();
                let mut to_resolve: Vec<(String, String)> = package
                    .dependencies
                    .iter()
                    .map(|(name, ver)| (name.clone(), ver.clone()))
                    .collect();

                while let Some((name, ver)) = to_resolve.pop() {
                    let dep_key = format!("{}@{}", name, ver);

                    if !resolved.contains(&dep_key) {
                        resolved.push(dep_key.clone());

                        if let Some(dep_pkg) = self.packages.get(&dep_key) {
                            for (dep_name, dep_ver) in &dep_pkg.dependencies {
                                to_resolve.push((dep_name.clone(), dep_ver.clone()));
                            }
                        }
                    }
                }

                Ok(resolved)
            }
            None => Err(format!("Package {}@{} not found", package_name, version)),
        }
    }

    /// Sign a package
    pub fn sign_package(&self, package: &Package, private_key: &str) -> String {
        // Simplified signing - in production, use proper cryptographic signing
        let data = format!("{}{}{}", package.name, package.version, package.checksum);
        format!("sig_{}_{}", data, private_key)
    }

    /// Verify package signature
    pub fn verify_signature(&self, package: &Package, signature: &str) -> bool {
        // Simplified verification
        signature.starts_with("sig_") && signature.contains(&package.name)
    }

    /// Scan for vulnerabilities
    pub fn scan_vulnerabilities(&self, package: &Package) -> Result<(), String> {
        // Check against known vulnerable packages
        let vulnerable_packages = ["malicious-pkg", "backdoor-lib"];

        if vulnerable_packages.contains(&package.name.as_str()) {
            return Err(format!(
                "Package {} is known to be vulnerable",
                package.name
            ));
        }

        // Check dependencies for vulnerabilities
        for dep_name in package.dependencies.keys() {
            if vulnerable_packages.contains(&dep_name.as_str()) {
                return Err(format!("Dependency {} is vulnerable", dep_name));
            }
        }

        Ok(())
    }

    /// Get package statistics
    pub fn get_stats(&self, package_name: &str, version: &str) -> Option<PackageStats> {
        let key = format!("{}@{}", package_name, version);

        self.packages.get(&key).map(|pkg| PackageStats {
            name: pkg.name.clone(),
            version: pkg.version.clone(),
            downloads: *self.downloads.get(&key).unwrap_or(&0),
            created_at: pkg.created_at,
            updated_at: pkg.updated_at,
            dependency_count: pkg.dependencies.len(),
        })
    }

    /// List popular packages
    pub fn get_popular_packages(&self, limit: usize) -> Vec<(&Package, u64)> {
        let mut packages: Vec<_> = self
            .packages
            .iter()
            .map(|(key, pkg)| (pkg, *self.downloads.get(key).unwrap_or(&0)))
            .collect();

        packages.sort_by(|a, b| b.1.cmp(&a.1));
        packages.truncate(limit);
        packages
    }

    /// Register a new user
    pub fn register_user(&mut self, user: User) -> Result<(), String> {
        if self.users.contains_key(&user.username) {
            return Err("User already exists".to_string());
        }

        self.users.insert(user.username.clone(), user);
        Ok(())
    }

    /// Get user packages
    pub fn get_user_packages(&self, username: &str) -> Vec<&Package> {
        self.packages
            .values()
            .filter(|pkg| pkg.author == username)
            .collect()
    }
}

impl Default for PackageRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct PackageStats {
    pub name: String,
    pub version: String,
    pub downloads: u64,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
    pub dependency_count: usize,
}

/// Auto-import and suggestions system
pub struct AutoImport {
    symbol_index: HashMap<String, Vec<String>>,
}

impl AutoImport {
    pub fn new() -> Self {
        AutoImport {
            symbol_index: HashMap::new(),
        }
    }

    /// Index symbols from a package
    pub fn index_package(&mut self, package_name: &str, symbols: Vec<String>) {
        for symbol in symbols {
            self.symbol_index
                .entry(symbol)
                .or_default()
                .push(package_name.to_string());
        }
    }

    /// Detect missing symbols
    pub fn detect_missing_symbols(&self, code: &str) -> Vec<String> {
        // Simplified detection - in production, use proper AST analysis
        let mut missing = Vec::new();

        for word in code.split_whitespace() {
            if word.chars().next().is_some_and(|c| c.is_uppercase())
                && !self.symbol_index.contains_key(word)
            {
                missing.push(word.to_string());
            }
        }

        missing
    }

    /// Suggest imports for missing symbols
    pub fn suggest_imports(&self, symbol: &str) -> Vec<String> {
        self.symbol_index.get(symbol).cloned().unwrap_or_default()
    }

    /// Recommend packages based on usage
    pub fn recommend_packages(&self, symbols: &[String]) -> Vec<String> {
        let mut package_scores: HashMap<String, usize> = HashMap::new();

        for symbol in symbols {
            if let Some(packages) = self.symbol_index.get(symbol) {
                for package in packages {
                    *package_scores.entry(package.clone()).or_insert(0) += 1;
                }
            }
        }

        let mut recommendations: Vec<_> = package_scores.into_iter().collect();
        recommendations.sort_by(|a, b| b.1.cmp(&a.1));

        recommendations.into_iter().map(|(pkg, _)| pkg).collect()
    }
}

impl Default for AutoImport {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_creation() {
        let registry = PackageRegistry::new();
        assert_eq!(registry.packages.len(), 0);
    }

    #[test]
    fn test_package_upload() {
        let mut registry = PackageRegistry::new();

        let user = User {
            username: "alice".to_string(),
            email: "alice@example.com".to_string(),
            api_key: "key123".to_string(),
            packages: Vec::new(),
            created_at: SystemTime::now(),
        };
        registry.register_user(user).unwrap();

        let package = Package {
            name: "test-pkg".to_string(),
            version: "1.0.0".to_string(),
            author: "alice".to_string(),
            description: "A test package".to_string(),
            license: "MIT".to_string(),
            repository: None,
            homepage: None,
            keywords: vec!["test".to_string()],
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
            created_at: SystemTime::now(),
            updated_at: SystemTime::now(),
            downloads: 0,
            signature: None,
            checksum: "abc123".to_string(),
        };

        assert!(registry.upload_package(package, "alice").is_ok());
    }

    #[test]
    fn test_package_download() {
        let mut registry = PackageRegistry::new();

        let user = User {
            username: "alice".to_string(),
            email: "alice@example.com".to_string(),
            api_key: "key123".to_string(),
            packages: Vec::new(),
            created_at: SystemTime::now(),
        };
        registry.register_user(user).unwrap();

        let package = Package {
            name: "test-pkg".to_string(),
            version: "1.0.0".to_string(),
            author: "alice".to_string(),
            description: "A test package".to_string(),
            license: "MIT".to_string(),
            repository: None,
            homepage: None,
            keywords: vec!["test".to_string()],
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
            created_at: SystemTime::now(),
            updated_at: SystemTime::now(),
            downloads: 0,
            signature: None,
            checksum: "abc123".to_string(),
        };

        registry.upload_package(package, "alice").unwrap();

        let downloaded = registry.download_package("test-pkg", "1.0.0");
        assert!(downloaded.is_ok());
    }

    #[test]
    fn test_search() {
        let mut registry = PackageRegistry::new();

        let user = User {
            username: "alice".to_string(),
            email: "alice@example.com".to_string(),
            api_key: "key123".to_string(),
            packages: Vec::new(),
            created_at: SystemTime::now(),
        };
        registry.register_user(user).unwrap();

        let package = Package {
            name: "web-framework".to_string(),
            version: "1.0.0".to_string(),
            author: "alice".to_string(),
            description: "A web framework".to_string(),
            license: "MIT".to_string(),
            repository: None,
            homepage: None,
            keywords: vec!["web".to_string(), "framework".to_string()],
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
            created_at: SystemTime::now(),
            updated_at: SystemTime::now(),
            downloads: 0,
            signature: None,
            checksum: "abc123".to_string(),
        };

        registry.upload_package(package, "alice").unwrap();

        let results = registry.search("web");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_dependency_resolution() {
        let mut registry = PackageRegistry::new();

        let user = User {
            username: "alice".to_string(),
            email: "alice@example.com".to_string(),
            api_key: "key123".to_string(),
            packages: Vec::new(),
            created_at: SystemTime::now(),
        };
        registry.register_user(user).unwrap();

        let mut deps = HashMap::new();
        deps.insert("dep1".to_string(), "1.0.0".to_string());

        let package = Package {
            name: "main-pkg".to_string(),
            version: "1.0.0".to_string(),
            author: "alice".to_string(),
            description: "Main package".to_string(),
            license: "MIT".to_string(),
            repository: None,
            homepage: None,
            keywords: Vec::new(),
            dependencies: deps,
            dev_dependencies: HashMap::new(),
            created_at: SystemTime::now(),
            updated_at: SystemTime::now(),
            downloads: 0,
            signature: None,
            checksum: "abc123".to_string(),
        };

        registry.upload_package(package, "alice").unwrap();

        let resolved = registry.resolve_dependencies("main-pkg", "1.0.0");
        assert!(resolved.is_ok());
    }

    #[test]
    fn test_auto_import() {
        let mut auto_import = AutoImport::new();

        auto_import.index_package("std", vec!["Vec".to_string(), "HashMap".to_string()]);

        let suggestions = auto_import.suggest_imports("Vec");
        assert_eq!(suggestions, vec!["std"]);
    }

    #[test]
    fn test_package_recommendations() {
        let mut auto_import = AutoImport::new();

        auto_import.index_package("std", vec!["Vec".to_string(), "HashMap".to_string()]);
        auto_import.index_package(
            "serde",
            vec!["Serialize".to_string(), "Deserialize".to_string()],
        );

        let recommendations =
            auto_import.recommend_packages(&["Vec".to_string(), "HashMap".to_string()]);
        assert!(recommendations.contains(&"std".to_string()));
    }

    #[test]
    fn test_vulnerability_scan() {
        let registry = PackageRegistry::new();

        let safe_package = Package {
            name: "safe-pkg".to_string(),
            version: "1.0.0".to_string(),
            author: "alice".to_string(),
            description: "Safe package".to_string(),
            license: "MIT".to_string(),
            repository: None,
            homepage: None,
            keywords: Vec::new(),
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
            created_at: SystemTime::now(),
            updated_at: SystemTime::now(),
            downloads: 0,
            signature: None,
            checksum: "abc123".to_string(),
        };

        assert!(registry.scan_vulnerabilities(&safe_package).is_ok());

        let vulnerable_package = Package {
            name: "malicious-pkg".to_string(),
            version: "1.0.0".to_string(),
            author: "hacker".to_string(),
            description: "Vulnerable package".to_string(),
            license: "MIT".to_string(),
            repository: None,
            homepage: None,
            keywords: Vec::new(),
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
            created_at: SystemTime::now(),
            updated_at: SystemTime::now(),
            downloads: 0,
            signature: None,
            checksum: "abc123".to_string(),
        };

        assert!(registry.scan_vulnerabilities(&vulnerable_package).is_err());
    }
}
