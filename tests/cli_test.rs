// CLI command tests
use std::process::Command;

#[test]
fn test_cli_package_list() {
    let output = Command::new("cargo")
        .args(["run", "--bin", "gul", "--", "package", "list"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Available packages") || stdout.contains("●"));
}

#[test]
fn test_cli_package_info() {
    let output = Command::new("cargo")
        .args(["run", "--bin", "gul", "--", "package", "info", "actix-web"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("actix-web") || stdout.contains("Package"));
}

#[test]
fn test_cli_package_search() {
    let output = Command::new("cargo")
        .args(["run", "--bin", "gul", "--", "package", "search", "web"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("packages found") || stdout.contains("Searching"));
}

#[test]
fn test_cli_ai_status() {
    let output = Command::new("cargo")
        .args(["run", "--bin", "gul", "--", "ai", "status"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Provider") || stdout.contains("AI"));
}

#[test]
fn test_cli_package_audit() {
    let output = Command::new("cargo")
        .args(["run", "--bin", "gul", "--", "package", "audit"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Auditing") || stdout.contains("Scanned"));
}

#[test]
fn test_cli_package_outdated() {
    let output = Command::new("cargo")
        .args(["run", "--bin", "gul", "--", "package", "outdated"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Checking") || stdout.contains("packages checked"));
}
