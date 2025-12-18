// Integration tests for GUL package manager
use gul::platform::package_support::PackageManager;

#[test]
fn test_package_installation_flow() {
    let pm = PackageManager::new();

    // Test package exists
    assert!(pm.has_package("actix-web"));
    assert!(pm.has_package("next.js"));

    // Test package retrieval
    let pkg = pm.get_package("actix-web");
    assert!(pkg.is_some());
    assert_eq!(pkg.unwrap().language, "rust");
}

#[test]
fn test_cross_language_packages() {
    let pm = PackageManager::new();

    let rust_pkgs = pm.list_packages_by_language("rust");
    let js_pkgs = pm.list_packages_by_language("javascript");
    let py_pkgs = pm.list_packages_by_language("python");

    assert!(
        rust_pkgs.len() >= 10,
        "Should have at least 10 Rust packages"
    );
    assert!(
        js_pkgs.len() >= 6,
        "Should have at least 6 JavaScript packages"
    );
    assert!(py_pkgs.len() >= 6, "Should have at least 6 Python packages");
}

#[test]
fn test_package_dependencies() {
    let pm = PackageManager::new();

    let pkg = pm.get_package("actix-web").unwrap();
    assert!(pkg.dependencies.contains(&"tokio".to_string()));
}

#[test]
fn test_package_features() {
    let pm = PackageManager::new();

    let pkg = pm.get_package("sqlx").unwrap();
    assert!(pkg.features.contains(&"postgres".to_string()));
    assert!(pkg.features.contains(&"mysql".to_string()));
}
