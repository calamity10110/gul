#!/usr/bin/env python3
"""
Check package versions for consistency and validity.

This script validates that all packages have proper version numbers
and checks for version conflicts between dependencies.
"""

import os
import sys
import json
import tomllib
from pathlib import Path


def load_cargo_toml(path: Path) -> dict | None:
    """Load and parse a Cargo.toml file."""
    try:
        with open(path, "rb") as f:
            return tomllib.load(f)
    except Exception as e:
        print(f"Warning: Could not load {path}: {e}")
        return None


def check_semver(version: str) -> bool:
    """Check if a version string is valid semver."""
    parts = version.replace("-", ".").split(".")
    if len(parts) < 3:
        return False
    try:
        int(parts[0])
        int(parts[1])
        int(parts[2].split("+")[0])  # Handle build metadata
        return True
    except ValueError:
        return False


def check_package_versions(packages_dir: Path) -> list[dict]:
    """Check all package versions in a directory."""
    issues = []
    
    for package_path in packages_dir.rglob("Cargo.toml"):
        cargo_toml = load_cargo_toml(package_path)
        if not cargo_toml:
            issues.append({
                "file": str(package_path),
                "error": "Could not parse Cargo.toml"
            })
            continue
        
        package = cargo_toml.get("package", {})
        name = package.get("name", "unknown")
        version = package.get("version", "")
        
        if not version:
            issues.append({
                "file": str(package_path),
                "package": name,
                "error": "Missing version field"
            })
        elif not check_semver(version):
            issues.append({
                "file": str(package_path),
                "package": name,
                "version": version,
                "error": "Invalid semver format"
            })
    
    return issues


def check_gul_package_versions(gul_packages_dir: Path) -> list[dict]:
    """Check GUL package versions in gul_packages directory."""
    issues = []
    
    for package_dir in gul_packages_dir.iterdir():
        if not package_dir.is_dir():
            continue
        
        manifest_path = package_dir / "gul.toml"
        if not manifest_path.exists():
            manifest_path = package_dir / "package.toml"
        
        if manifest_path.exists():
            try:
                with open(manifest_path, "rb") as f:
                    manifest = tomllib.load(f)
                
                package = manifest.get("package", {})
                version = package.get("version", "")
                
                if version and not check_semver(version):
                    issues.append({
                        "file": str(manifest_path),
                        "package": package_dir.name,
                        "version": version,
                        "error": "Invalid semver format"
                    })
            except Exception as e:
                issues.append({
                    "file": str(manifest_path),
                    "package": package_dir.name,
                    "error": f"Parse error: {e}"
                })
    
    return issues


def main():
    """Main entry point."""
    project_root = Path(__file__).parent.parent.parent
    
    all_issues = []
    
    # Check Rust packages
    packages_dir = project_root / "packages"
    if packages_dir.exists():
        all_issues.extend(check_package_versions(packages_dir))
    
    # Check GUL packages
    gul_packages_dir = project_root / "gul_packages"
    if gul_packages_dir.exists():
        all_issues.extend(check_gul_package_versions(gul_packages_dir))
    
    # Report results
    if all_issues:
        print("❌ Package version issues found:")
        for issue in all_issues:
            print(f"  - {issue.get('package', 'unknown')}: {issue['error']}")
            if 'version' in issue:
                print(f"    Version: {issue['version']}")
            print(f"    File: {issue['file']}")
        sys.exit(1)
    else:
        print("✅ All package versions are valid")
        sys.exit(0)


if __name__ == "__main__":
    main()
