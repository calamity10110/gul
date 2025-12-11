#!/usr/bin/env python3
"""
Test package compatibility across the GUL ecosystem.
Ensures packages work together and don't have conflicting dependencies.
"""

import os
import sys
import subprocess
import json
from pathlib import Path
from typing import List, Dict, Tuple
import argparse

class CompatibilityTester:
    def __init__(self, category: str = None):
        self.root_dir = Path(os.getcwd())
        self.category = category
        self.test_results: Dict[str, bool] = {}
        self.compatibility_matrix: Dict[Tuple[str, str], bool] = {}
    
    def run_tests(self) -> bool:
        """Run all compatibility tests"""
        print(f"🧪 Testing package compatibility...")
        
        packages = self._get_packages()
        
        # Test individual packages
        for package in packages:
            result = self._test_package(package)
            self.test_results[package] = result
        
        # Test package combinations
        self._test_package_combinations(packages)
        
        # Generate report
        self._generate_report()
        
        return all(self.test_results.values())
    
    def _get_packages(self) -> List[str]:
        """Get list of packages to test"""
        packages = []
        packages_dir = self.root_dir / "packages"
        
        if self.category:
            category_dir = packages_dir / self.category
            if category_dir.exists():
                packages.extend([f"{self.category}/{p.name}" for p in category_dir.iterdir() if p.is_dir()])
        else:
            for category_dir in packages_dir.iterdir():
                if category_dir.is_dir():
                    packages.extend([f"{category_dir.name}/{p.name}" for p in category_dir.iterdir() if p.is_dir()])
        
        return packages
    
    def _test_package(self, package: str) -> bool:
        """Test a single package"""
        print(f"  Testing {package}...")
        
        package_dir = self.root_dir / "packages" / package
        
        try:
            # Run package tests
            result = subprocess.run(
                ["cargo", "test", "--all-features"],
                cwd=package_dir,
                capture_output=True,
                text=True,
                timeout=300
            )
            
            if result.returncode == 0:
                print(f"    ✅ {package} tests passed")
                return True
            else:
                print(f"    ❌ {package} tests failed")
                print(f"    Error: {result.stderr[:200]}")
                return False
        
        except subprocess.TimeoutExpired:
            print(f"    ⏱️  {package} tests timed out")
            return False
        except Exception as e:
            print(f"    ❌ {package} error: {e}")
            return False
    
    def _test_package_combinations(self, packages: List[str]):
        """Test package combinations for compatibility"""
        print(f"\n🔗 Testing package combinations...")
        
        # Test common package pairs
        common_pairs = [
            ("web/gul-http", "database/gul-postgres"),
            ("web/gul-http", "utils/gul-json"),
            ("tui/gul-tui", "utils/gul-datetime"),
            ("data-science/gul-ml", "database/gul-postgres"),
        ]
        
        for pkg1, pkg2 in common_pairs:
            if pkg1 in packages and pkg2 in packages:
                compatible = self._test_package_pair(pkg1, pkg2)
                self.compatibility_matrix[(pkg1, pkg2)] = compatible
    
    def _test_package_pair(self, pkg1: str, pkg2: str) -> bool:
        """Test if two packages are compatible"""
        print(f"  Testing {pkg1} + {pkg2}...")
        
        # Create a temporary test project
        test_dir = self.root_dir / "target" / "compat_test"
        test_dir.mkdir(parents=True, exist_ok=True)
        
        # Create Cargo.toml with both dependencies
        cargo_toml = test_dir / "Cargo.toml"
        with open(cargo_toml, "w") as f:
            f.write(f"""
[package]
name = "compat_test"
version = "0.1.0"
edition = "2021"

[dependencies]
{pkg1.split('/')[-1]} = {{ path = "../../packages/{pkg1}" }}
{pkg2.split('/')[-1]} = {{ path = "../../packages/{pkg2}" }}
""")
        
        # Create dummy main.rs
        src_dir = test_dir / "src"
        src_dir.mkdir(exist_ok=True)
        with open(src_dir / "main.rs", "w") as f:
            f.write("fn main() {}\n")
        
        try:
            # Try to build
            result = subprocess.run(
                ["cargo", "build"],
                cwd=test_dir,
                capture_output=True,
                timeout=120
            )
            
            if result.returncode == 0:
                print(f"    ✅ Compatible")
                return True
            else:
                print(f"    ❌ Incompatible")
                return False
        
        except Exception as e:
            print(f"    ❌ Error: {e}")
            return False
    
    def _generate_report(self):
        """Generate compatibility report"""
        report = {
            "timestamp": subprocess.check_output(["date", "-u", "+%Y-%m-%dT%H:%M:%SZ"]).decode().strip(),
            "individual_tests": self.test_results,
            "compatibility_matrix": {
                f"{p1}+{p2}": result
                for (p1, p2), result in self.compatibility_matrix.items()
            }
        }
        
        report_file = self.root_dir / "target" / "compatibility_report.json"
        with open(report_file, "w") as f:
            json.dump(report, f, indent=2)
        
        print(f"\n📊 Compatibility report saved to {report_file}")
        
        # Print summary
        passed = sum(1 for r in self.test_results.values() if r)
        total = len(self.test_results)
        print(f"\n{'='*60}")
        print(f"Compatibility Test Results")
        print(f"{'='*60}")
        print(f"Individual packages: {passed}/{total} passed")
        
        if self.compatibility_matrix:
            compat_passed = sum(1 for r in self.compatibility_matrix.values() if r)
            compat_total = len(self.compatibility_matrix)
            print(f"Package pairs: {compat_passed}/{compat_total} compatible")
        
        print(f"{'='*60}\n")

def main():
    parser = argparse.ArgumentParser(description="Test package compatibility")
    parser.add_argument("--category", help="Test only packages in this category")
    args = parser.parse_args()
    
    tester = CompatibilityTester(category=args.category)
    success = tester.run_tests()
    
    sys.exit(0 if success else 1)

if __name__ == "__main__":
    main()
