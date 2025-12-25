#!/usr/bin/env python3
"""
Validate package dependencies.

This script checks that all package dependencies are valid and resolvable.
"""

import os
import sys
import tomllib
from pathlib import Path
from collections import defaultdict


def load_toml(path: Path) -> dict | None:
    """Load and parse a TOML file."""
    try:
        with open(path, "rb") as f:
            return tomllib.load(f)
    except Exception as e:
        print(f"Warning: Could not load {path}: {e}")
        return None


def get_all_packages(project_root: Path) -> dict[str, Path]:
    """Get a mapping of all available package names to their paths."""
    packages = {}
    
    # Scan packages directory
    packages_dir = project_root / "packages"
    if packages_dir.exists():
        for cargo_toml in packages_dir.rglob("Cargo.toml"):
            data = load_toml(cargo_toml)
            if data and "package" in data:
                name = data["package"].get("name")
                if name:
                    packages[name] = cargo_toml.parent
    
    # Scan gul_packages directory
    gul_packages_dir = project_root / "gul_packages"
    if gul_packages_dir.exists():
        for pkg_dir in gul_packages_dir.iterdir():
            if pkg_dir.is_dir():
                packages[pkg_dir.name] = pkg_dir
    
    return packages


def check_rust_dependencies(cargo_toml: Path, available_packages: dict[str, Path]) -> list[dict]:
    """Check dependencies in a Cargo.toml file."""
    issues = []
    data = load_toml(cargo_toml)
    
    if not data:
        return issues
    
    package_name = data.get("package", {}).get("name", "unknown")
    
    for dep_type in ["dependencies", "dev-dependencies", "build-dependencies"]:
        deps = data.get(dep_type, {})
        for dep_name, dep_spec in deps.items():
            # Skip external crates (they have version, git, or registry)
            if isinstance(dep_spec, str):
                continue  # Just a version string - external crate
            if isinstance(dep_spec, dict):
                if "path" in dep_spec:
                    # Local path dependency - check if it exists
                    dep_path = cargo_toml.parent / dep_spec["path"]
                    if not dep_path.exists():
                        issues.append({
                            "package": package_name,
                            "dependency": dep_name,
                            "type": dep_type,
                            "error": f"Path dependency not found: {dep_spec['path']}"
                        })
                elif "version" in dep_spec or "git" in dep_spec or "registry" in dep_spec:
                    continue  # External crate
    
    return issues


def check_circular_dependencies(project_root: Path) -> list[dict]:
    """Check for circular dependencies between packages."""
    issues = []
    
    # Build dependency graph
    graph = defaultdict(set)
    packages_dir = project_root / "packages"
    
    if not packages_dir.exists():
        return issues
    
    for cargo_toml in packages_dir.rglob("Cargo.toml"):
        data = load_toml(cargo_toml)
        if not data or "package" not in data:
            continue
        
        package_name = data["package"].get("name")
        if not package_name:
            continue
        
        for dep_type in ["dependencies", "dev-dependencies"]:
            deps = data.get(dep_type, {})
            for dep_name in deps:
                graph[package_name].add(dep_name)
    
    # Simple cycle detection using DFS
    def find_cycle(node: str, visited: set, path: list) -> list | None:
        if node in path:
            return path[path.index(node):] + [node]
        if node in visited:
            return None
        visited.add(node)
        path.append(node)
        for neighbor in graph.get(node, []):
            if neighbor in graph:  # Only check internal packages
                cycle = find_cycle(neighbor, visited, path)
                if cycle:
                    return cycle
        path.pop()
        return None
    
    visited = set()
    for node in graph:
        cycle = find_cycle(node, visited, [])
        if cycle:
            issues.append({
                "error": "Circular dependency detected",
                "cycle": " -> ".join(cycle)
            })
            break  # Only report first cycle found
    
    return issues


def main():
    """Main entry point."""
    project_root = Path(__file__).parent.parent.parent
    
    all_issues = []
    available_packages = get_all_packages(project_root)
    
    print(f"Found {len(available_packages)} packages")
    
    # Check Rust package dependencies
    packages_dir = project_root / "packages"
    if packages_dir.exists():
        for cargo_toml in packages_dir.rglob("Cargo.toml"):
            issues = check_rust_dependencies(cargo_toml, available_packages)
            all_issues.extend(issues)
    
    # Check for circular dependencies
    circular_issues = check_circular_dependencies(project_root)
    all_issues.extend(circular_issues)
    
    # Report results
    if all_issues:
        print("❌ Dependency issues found:")
        for issue in all_issues:
            if "cycle" in issue:
                print(f"  - {issue['error']}: {issue['cycle']}")
            else:
                print(f"  - {issue['package']}: {issue['error']}")
                print(f"    Dependency: {issue['dependency']} ({issue['type']})")
        sys.exit(1)
    else:
        print("✅ All dependencies are valid")
        sys.exit(0)


if __name__ == "__main__":
    main()
