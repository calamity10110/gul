#!/usr/bin/env python3
"""
Generate package compatibility report.

This script generates a compatibility matrix showing which packages
are compatible with each other.
"""

import os
import sys
import json
import tomllib
from pathlib import Path
from datetime import datetime


def load_toml(path: Path) -> dict | None:
    """Load and parse a TOML file."""
    try:
        with open(path, "rb") as f:
            return tomllib.load(f)
    except Exception as e:
        return None


def get_package_info(project_root: Path) -> list[dict]:
    """Get information about all packages."""
    packages = []
    
    # Scan gul_packages directory
    gul_packages_dir = project_root / "gul_packages"
    if gul_packages_dir.exists():
        for pkg_dir in gul_packages_dir.iterdir():
            if not pkg_dir.is_dir():
                continue
            
            info = {
                "name": pkg_dir.name,
                "path": str(pkg_dir),
                "type": "gul",
                "version": "0.1.0",
                "dependencies": [],
                "status": "unknown"
            }
            
            # Try to load manifest
            for manifest_name in ["gul.toml", "package.toml"]:
                manifest_path = pkg_dir / manifest_name
                if manifest_path.exists():
                    data = load_toml(manifest_path)
                    if data:
                        pkg_data = data.get("package", {})
                        info["version"] = pkg_data.get("version", "0.1.0")
                        info["description"] = pkg_data.get("description", "")
                        
                        deps = data.get("dependencies", {})
                        info["dependencies"] = list(deps.keys())
                    break
            
            # Check for test files
            test_files = list(pkg_dir.glob("*_test.mn")) + list(pkg_dir.glob("tests/*.mn"))
            info["has_tests"] = len(test_files) > 0
            
            packages.append(info)
    
    return packages


def generate_compatibility_matrix(packages: list[dict]) -> dict:
    """Generate a compatibility matrix for packages."""
    matrix = {}
    package_names = [p["name"] for p in packages]
    
    for pkg in packages:
        matrix[pkg["name"]] = {}
        for other_name in package_names:
            if pkg["name"] == other_name:
                matrix[pkg["name"]][other_name] = "self"
            elif other_name in pkg["dependencies"]:
                matrix[pkg["name"]][other_name] = "depends"
            else:
                matrix[pkg["name"]][other_name] = "compatible"
    
    return matrix


def generate_report(packages: list[dict], matrix: dict) -> str:
    """Generate a markdown compatibility report."""
    report = []
    report.append("# GUL Package Compatibility Report")
    report.append("")
    report.append(f"Generated: {datetime.now().isoformat()}")
    report.append("")
    
    # Summary
    report.append("## Summary")
    report.append("")
    report.append(f"- **Total packages**: {len(packages)}")
    with_tests = sum(1 for p in packages if p.get("has_tests"))
    report.append(f"- **Packages with tests**: {with_tests}")
    report.append("")
    
    # Package list
    report.append("## Packages")
    report.append("")
    report.append("| Package | Version | Dependencies | Tests |")
    report.append("|---------|---------|--------------|-------|")
    
    for pkg in sorted(packages, key=lambda x: x["name"]):
        deps = ", ".join(pkg["dependencies"]) if pkg["dependencies"] else "-"
        tests = "✅" if pkg.get("has_tests") else "❌"
        report.append(f"| {pkg['name']} | {pkg['version']} | {deps} | {tests} |")
    
    report.append("")
    
    # Dependency graph
    report.append("## Dependency Graph")
    report.append("")
    report.append("```mermaid")
    report.append("graph TD")
    
    for pkg in packages:
        if pkg["dependencies"]:
            for dep in pkg["dependencies"]:
                report.append(f"    {pkg['name']} --> {dep}")
    
    report.append("```")
    report.append("")
    
    return "\n".join(report)


def main():
    """Main entry point."""
    project_root = Path(__file__).parent.parent.parent
    
    # Get package information
    packages = get_package_info(project_root)
    
    if not packages:
        print("No packages found")
        sys.exit(0)
    
    print(f"Found {len(packages)} packages")
    
    # Generate compatibility matrix
    matrix = generate_compatibility_matrix(packages)
    
    # Generate report
    report = generate_report(packages, matrix)
    
    # Save report
    report_path = project_root / "docs" / "api" / "compatibility_report.md"
    report_path.parent.mkdir(parents=True, exist_ok=True)
    
    with open(report_path, "w") as f:
        f.write(report)
    
    print(f"✅ Compatibility report generated: {report_path}")
    
    # Also save JSON data
    json_path = project_root / "docs" / "api" / "compatibility_matrix.json"
    with open(json_path, "w") as f:
        json.dump({
            "packages": packages,
            "matrix": matrix,
            "generated": datetime.now().isoformat()
        }, f, indent=2)
    
    print(f"✅ Compatibility matrix saved: {json_path}")


if __name__ == "__main__":
    main()
