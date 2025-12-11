#!/usr/bin/env python3
"""
Validate all GUL packages for correctness and completeness.
Checks: metadata, dependencies, version compatibility, and structure.
"""

import os
import sys
import json
import toml
from pathlib import Path
from typing import Dict, List, Set

class PackageValidator:
    def __init__(self, root_dir: str):
        self.root_dir = Path(root_dir)
        self.errors: List[str] = []
        self.warnings: List[str] = []
        self.validated_packages: Set[str] = set()
    
    def validate_all(self) -> bool:
        """Validate all packages in the project"""
        print("🔍 Validating GUL packages...")
        
        # Validate Rust packages
        self._validate_rust_packages()
        
        # Validate GUL standard library packages
        self._validate_gul_packages()
        
        # Print results
        self._print_results()
        
        return len(self.errors) == 0
    
    def _validate_rust_packages(self):
        """Validate Rust package structure"""
        packages_dir = self.root_dir / "packages"
        
        if not packages_dir.exists():
            self.errors.append(f"Packages directory not found: {packages_dir}")
            return
        
        for category_dir in packages_dir.iterdir():
            if not category_dir.is_dir():
                continue
            
            for package_dir in category_dir.iterdir():
                if not package_dir.is_dir():
                    continue
                
                self._validate_rust_package(package_dir)
    
    def _validate_rust_package(self, package_dir: Path):
        """Validate a single Rust package"""
        package_name = package_dir.name
        
        # Check for Cargo.toml
        cargo_toml = package_dir / "Cargo.toml"
        if not cargo_toml.exists():
            self.errors.append(f"{package_name}: Missing Cargo.toml")
            return
        
        try:
            cargo_data = toml.load(cargo_toml)
        except Exception as e:
            self.errors.append(f"{package_name}: Invalid Cargo.toml: {e}")
            return
        
        # Validate package metadata
        if "package" not in cargo_data:
            self.errors.append(f"{package_name}: Missing [package] section")
            return
        
        pkg_info = cargo_data["package"]
        
        # Check required fields
        required_fields = ["name", "version", "edition"]
        for field in required_fields:
            if field not in pkg_info:
                self.errors.append(f"{package_name}: Missing required field '{field}'")
        
        # Check recommended fields
        recommended_fields = ["description", "license", "authors"]
        for field in recommended_fields:
            if field not in pkg_info:
                self.warnings.append(f"{package_name}: Missing recommended field '{field}'")
        
        # Check for README
        if not (package_dir / "README.md").exists():
            self.warnings.append(f"{package_name}: Missing README.md")
        
        # Check for src directory
        if not (package_dir / "src").exists():
            self.errors.append(f"{package_name}: Missing src directory")
        
        # Check for tests
        if not (package_dir / "tests").exists() and not (package_dir / "src").glob("**/test*.rs"):
            self.warnings.append(f"{package_name}: No tests found")
        
        # Validate dependencies
        if "dependencies" in cargo_data:
            self._validate_dependencies(package_name, cargo_data["dependencies"])
        
        self.validated_packages.add(package_name)
    
    def _validate_gul_packages(self):
        """Validate GUL standard library packages"""
        gul_packages_dir = self.root_dir / "gul_packages"
        
        if not gul_packages_dir.exists():
            self.warnings.append(f"GUL packages directory not found: {gul_packages_dir}")
            return
        
        for package_dir in gul_packages_dir.iterdir():
            if not package_dir.is_dir():
                continue
            
            self._validate_gul_package(package_dir)
    
    def _validate_gul_package(self, package_dir: Path):
        """Validate a single GUL package"""
        package_name = package_dir.name
        
        # Check for package.gul or main entry point
        main_files = list(package_dir.glob("*.gul"))
        if not main_files:
            self.warnings.append(f"{package_name}: No .gul files found")
        
        # Check for package metadata
        metadata_file = package_dir / "package.json"
        if metadata_file.exists():
            try:
                with open(metadata_file) as f:
                    metadata = json.load(f)
                
                # Validate metadata structure
                if "name" not in metadata:
                    self.errors.append(f"{package_name}: Missing 'name' in package.json")
                if "version" not in metadata:
                    self.errors.append(f"{package_name}: Missing 'version' in package.json")
            except Exception as e:
                self.errors.append(f"{package_name}: Invalid package.json: {e}")
        else:
            self.warnings.append(f"{package_name}: Missing package.json metadata")
        
        self.validated_packages.add(f"gul:{package_name}")
    
    def _validate_dependencies(self, package_name: str, dependencies: Dict):
        """Validate package dependencies"""
        for dep_name, dep_spec in dependencies.items():
            if isinstance(dep_spec, dict):
                # Complex dependency specification
                if "version" not in dep_spec and "path" not in dep_spec and "git" not in dep_spec:
                    self.warnings.append(
                        f"{package_name}: Dependency '{dep_name}' has no version/path/git specification"
                    )
            elif isinstance(dep_spec, str):
                # Simple version specification
                if dep_spec == "*":
                    self.warnings.append(
                        f"{package_name}: Dependency '{dep_name}' uses wildcard version '*'"
                    )
    
    def _print_results(self):
        """Print validation results"""
        print(f"\n{'='*60}")
        print(f"Validation Results")
        print(f"{'='*60}")
        print(f"✅ Validated packages: {len(self.validated_packages)}")
        print(f"⚠️  Warnings: {len(self.warnings)}")
        print(f"❌ Errors: {len(self.errors)}")
        
        if self.warnings:
            print(f"\n⚠️  Warnings:")
            for warning in self.warnings:
                print(f"   - {warning}")
        
        if self.errors:
            print(f"\n❌ Errors:")
            for error in self.errors:
                print(f"   - {error}")
        
        print(f"{'='*60}\n")

def main():
    root_dir = os.getcwd()
    validator = PackageValidator(root_dir)
    
    success = validator.validate_all()
    
    if success:
        print("✅ All packages validated successfully!")
        sys.exit(0)
    else:
        print("❌ Package validation failed!")
        sys.exit(1)

if __name__ == "__main__":
    main()
