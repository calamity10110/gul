// Package Manager Tests
// Tests for package registration, caching, version constraints, and metadata

use gul::platform::package_support::{PackageCategory, PackageManager};

#[test]
fn test_package_manager_creation() {
    let pm = PackageManager::new();
    assert!(
        pm.count() > 0,
        "Package manager should have default packages"
    );
}

#[test]
fn test_package_registration() {
    let pm = PackageManager::new();

    // Test that default packages are registered
    assert!(pm.has_package("axum"));
    assert!(pm.has_package("tokio"));
    assert!(pm.has_package("django"));
    assert!(pm.has_package("react"));
}

#[test]
fn test_package_retrieval() {
    let pm = PackageManager::new();

    let axum = pm.get_package("axum");
    assert!(axum.is_some());

    let pkg = axum.unwrap();
    assert_eq!(pkg.name, "axum");
    assert_eq!(pkg.language, "rust");
}

#[test]
fn test_list_packages() {
    let pm = PackageManager::new();
    let packages = pm.list_packages();

    assert!(packages.len() > 30, "Should have at least 30 packages");
}

#[test]
fn test_list_by_language() {
    let pm = PackageManager::new();

    let rust_packages = pm.list_packages_by_language("rust");
    assert!(
        rust_packages.len() >= 7,
        "Should have multiple Rust packages"
    );

    let python_packages = pm.list_packages_by_language("python");
    assert!(
        python_packages.len() >= 6,
        "Should have multiple Python packages"
    );

    let js_packages = pm.list_packages_by_language("javascript");
    assert!(
        js_packages.len() >= 6,
        "Should have multiple JavaScript packages"
    );
}

#[test]
fn test_package_has_dependencies() {
    let pm = PackageManager::new();

    let axum = pm.get_package("axum").unwrap();
    assert!(
        !axum.dependencies.is_empty(),
        "Axum should have dependencies"
    );
    assert!(axum.dependencies.contains(&"tokio".to_string()));
}

#[test]
fn test_package_has_features() {
    let pm = PackageManager::new();

    let tokio = pm.get_package("tokio").unwrap();
    assert!(!tokio.features.is_empty(), "Tokio should have features");
}

#[test]
fn test_nonexistent_package() {
    let pm = PackageManager::new();

    assert!(!pm.has_package("nonexistent_package"));
    assert!(pm.get_package("nonexistent_package").is_none());
}

#[test]
fn test_package_count() {
    let pm = PackageManager::new();
    let count = pm.count();

    assert!(count >= 30, "Should have at least 30 packages");
    assert_eq!(count, pm.list_packages().len());
}

#[test]
fn test_database_packages() {
    let pm = PackageManager::new();

    assert!(pm.has_package("postgresql"));
    assert!(pm.has_package("mysql"));
    assert!(pm.has_package("sqlite"));
    assert!(pm.has_package("mongodb"));
    assert!(pm.has_package("redis"));
}

#[test]
fn test_multi_language_support() {
    let pm = PackageManager::new();

    // Test that we have packages from multiple languages
    let languages: Vec<String> = pm
        .list_packages()
        .iter()
        .map(|p| p.language.clone())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();

    assert!(languages.len() >= 5, "Should support at least 5 languages");
}
