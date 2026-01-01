#![allow(dead_code)]
// Native package support for popular frameworks and libraries
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum PackageEcosystem {
    Rust,
    Python,
    JavaScript,
    TypeScript,
}

#[derive(Debug, Clone)]
pub struct NativePackage {
    pub name: String,
    pub ecosystem: PackageEcosystem,
    pub version: String,
    pub features: Vec<String>,
    pub dependencies: Vec<String>,
}

pub struct PackageRegistry {
    packages: HashMap<String, NativePackage>,
}

impl Default for PackageRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl PackageRegistry {
    pub fn new() -> Self {
        let mut registry = PackageRegistry {
            packages: HashMap::new(),
        };

        // Register popular Rust packages
        registry.register_rust_packages();

        // Register popular Python packages
        registry.register_python_packages();

        // Register popular JavaScript packages
        registry.register_js_packages();

        registry
    }

    fn register_rust_packages(&mut self) {
        // Axum - Web framework
        self.add_package(NativePackage {
            name: "axum".to_string(),
            ecosystem: PackageEcosystem::Rust,
            version: "0.7".to_string(),
            features: vec!["web".to_string(), "http".to_string(), "routing".to_string()],
            dependencies: vec!["tokio".to_string()],
        });

        // Tokio - Async runtime
        self.add_package(NativePackage {
            name: "tokio".to_string(),
            ecosystem: PackageEcosystem::Rust,
            version: "1.35".to_string(),
            features: vec!["async".to_string(), "runtime".to_string()],
            dependencies: vec![],
        });

        // Serde - Serialization
        self.add_package(NativePackage {
            name: "serde".to_string(),
            ecosystem: PackageEcosystem::Rust,
            version: "1.0".to_string(),
            features: vec!["serialization".to_string(), "json".to_string()],
            dependencies: vec![],
        });

        // Dioxus - UI framework
        self.add_package(NativePackage {
            name: "dioxus".to_string(),
            ecosystem: PackageEcosystem::Rust,
            version: "0.4".to_string(),
            features: vec!["ui".to_string(), "reactive".to_string()],
            dependencies: vec!["tokio".to_string()],
        });

        // Tauri - Desktop apps
        self.add_package(NativePackage {
            name: "tauri".to_string(),
            ecosystem: PackageEcosystem::Rust,
            version: "1.5".to_string(),
            features: vec!["desktop".to_string(), "webview".to_string()],
            dependencies: vec!["tokio".to_string()],
        });

        // Leptos - Web framework
        self.add_package(NativePackage {
            name: "leptos".to_string(),
            ecosystem: PackageEcosystem::Rust,
            version: "0.5".to_string(),
            features: vec!["web".to_string(), "ssr".to_string()],
            dependencies: vec!["tokio".to_string()],
        });
    }

    fn register_python_packages(&mut self) {
        // Django - Web framework
        self.add_package(NativePackage {
            name: "django".to_string(),
            ecosystem: PackageEcosystem::Python,
            version: "5.0".to_string(),
            features: vec!["web".to_string(), "orm".to_string(), "admin".to_string()],
            dependencies: vec![],
        });

        // Flask - Micro-framework
        self.add_package(NativePackage {
            name: "flask".to_string(),
            ecosystem: PackageEcosystem::Python,
            version: "3.0".to_string(),
            features: vec!["web".to_string(), "routing".to_string()],
            dependencies: vec![],
        });

        // FastAPI - Async API framework
        self.add_package(NativePackage {
            name: "fastapi".to_string(),
            ecosystem: PackageEcosystem::Python,
            version: "0.109".to_string(),
            features: vec!["web".to_string(), "async".to_string(), "api".to_string()],
            dependencies: vec!["pydantic".to_string()],
        });

        // Pydantic - Data validation
        self.add_package(NativePackage {
            name: "pydantic".to_string(),
            ecosystem: PackageEcosystem::Python,
            version: "2.5".to_string(),
            features: vec!["validation".to_string(), "serialization".to_string()],
            dependencies: vec![],
        });

        // NumPy - Numerical computing
        self.add_package(NativePackage {
            name: "numpy".to_string(),
            ecosystem: PackageEcosystem::Python,
            version: "1.26".to_string(),
            features: vec!["numerical".to_string(), "arrays".to_string()],
            dependencies: vec![],
        });

        // Pandas - Data analysis
        self.add_package(NativePackage {
            name: "pandas".to_string(),
            ecosystem: PackageEcosystem::Python,
            version: "2.1".to_string(),
            features: vec!["data".to_string(), "analysis".to_string()],
            dependencies: vec!["numpy".to_string()],
        });
    }

    fn register_js_packages(&mut self) {
        // React - UI library
        self.add_package(NativePackage {
            name: "react".to_string(),
            ecosystem: PackageEcosystem::JavaScript,
            version: "18.2".to_string(),
            features: vec!["ui".to_string(), "components".to_string()],
            dependencies: vec![],
        });

        // Angular - Web framework
        self.add_package(NativePackage {
            name: "angular".to_string(),
            ecosystem: PackageEcosystem::JavaScript,
            version: "17.0".to_string(),
            features: vec!["web".to_string(), "framework".to_string()],
            dependencies: vec![],
        });

        // Vue.js - Progressive framework
        self.add_package(NativePackage {
            name: "vue".to_string(),
            ecosystem: PackageEcosystem::JavaScript,
            version: "3.4".to_string(),
            features: vec!["web".to_string(), "reactive".to_string()],
            dependencies: vec![],
        });

        // Node.js (runtime reference)
        self.add_package(NativePackage {
            name: "node".to_string(),
            ecosystem: PackageEcosystem::JavaScript,
            version: "20.0".to_string(),
            features: vec!["runtime".to_string(), "server".to_string()],
            dependencies: vec![],
        });

        // Express.js - Web framework
        self.add_package(NativePackage {
            name: "express".to_string(),
            ecosystem: PackageEcosystem::JavaScript,
            version: "4.18".to_string(),
            features: vec!["web".to_string(), "routing".to_string()],
            dependencies: vec![],
        });

        // D3.js - Data visualization
        self.add_package(NativePackage {
            name: "d3".to_string(),
            ecosystem: PackageEcosystem::JavaScript,
            version: "7.8".to_string(),
            features: vec!["visualization".to_string(), "charts".to_string()],
            dependencies: vec![],
        });
    }

    pub fn add_package(&mut self, package: NativePackage) {
        self.packages.insert(package.name.clone(), package);
    }

    pub fn get_package(&self, name: &str) -> Option<&NativePackage> {
        self.packages.get(name)
    }

    pub fn list_packages(&self) -> Vec<&NativePackage> {
        self.packages.values().collect()
    }

    pub fn search_packages(&self, query: &str) -> Vec<&NativePackage> {
        self.packages
            .values()
            .filter(|pkg| {
                pkg.name.contains(query) || pkg.features.iter().any(|f| f.contains(query))
            })
            .collect()
    }

    pub fn get_by_ecosystem(&self, ecosystem: &PackageEcosystem) -> Vec<&NativePackage> {
        self.packages
            .values()
            .filter(|pkg| &pkg.ecosystem == ecosystem)
            .collect()
    }

    pub fn resolve_dependencies(&self, package_name: &str) -> Result<Vec<String>, String> {
        let mut resolved = Vec::new();
        let mut to_resolve = vec![package_name.to_string()];
        let mut visited = std::collections::HashSet::new();

        while let Some(name) = to_resolve.pop() {
            if visited.contains(&name) {
                continue;
            }

            visited.insert(name.clone());
            resolved.push(name.clone());

            if let Some(pkg) = self.get_package(&name) {
                for dep in &pkg.dependencies {
                    if !visited.contains(dep) {
                        to_resolve.push(dep.clone());
                    }
                }
            } else {
                return Err(format!("Package '{}' not found", name));
            }
        }

        Ok(resolved)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_creation() {
        let registry = PackageRegistry::new();
        assert!(!registry.packages.is_empty());
    }

    #[test]
    fn test_get_package() {
        let registry = PackageRegistry::new();

        assert!(registry.get_package("axum").is_some());
        assert!(registry.get_package("tokio").is_some());
        assert!(registry.get_package("react").is_some());
        assert!(registry.get_package("django").is_some());
    }

    #[test]
    fn test_search_packages() {
        let registry = PackageRegistry::new();

        let web_packages = registry.search_packages("web");
        assert!(!web_packages.is_empty());

        let async_packages = registry.search_packages("async");
        assert!(!async_packages.is_empty());
    }

    #[test]
    fn test_get_by_ecosystem() {
        let registry = PackageRegistry::new();

        let rust_packages = registry.get_by_ecosystem(&PackageEcosystem::Rust);
        assert!(rust_packages.len() >= 6);

        let python_packages = registry.get_by_ecosystem(&PackageEcosystem::Python);
        assert!(python_packages.len() >= 6);

        let js_packages = registry.get_by_ecosystem(&PackageEcosystem::JavaScript);
        assert!(js_packages.len() >= 6);
    }

    #[test]
    fn test_resolve_dependencies() {
        let registry = PackageRegistry::new();

        let deps = registry.resolve_dependencies("axum").unwrap();
        assert!(deps.contains(&"tokio".to_string()));

        let deps = registry.resolve_dependencies("fastapi").unwrap();
        assert!(deps.contains(&"pydantic".to_string()));
    }

    #[test]
    fn test_package_features() {
        let registry = PackageRegistry::new();

        let axum = registry.get_package("axum").unwrap();
        assert!(axum.features.contains(&"web".to_string()));

        let numpy = registry.get_package("numpy").unwrap();
        assert!(numpy.features.contains(&"numerical".to_string()));
    }
}
