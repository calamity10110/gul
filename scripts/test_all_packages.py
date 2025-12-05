#!/usr/bin/env python3
"""
Test all GUL packages and generate examples
"""

import subprocess
import sys
from pathlib import Path

def test_package(pkg_path):
    """Test a single package"""
    print(f"Testing {pkg_path.name}...", end=" ")
    try:
        result = subprocess.run(
            ["cargo", "test", "--quiet"],
            cwd=pkg_path,
            capture_output=True,
            timeout=30
        )
        if result.returncode == 0:
            print("✓ PASS")
            return True
        else:
            print("✗ FAIL")
            return False
    except subprocess.TimeoutExpired:
        print("✗ TIMEOUT")
        return False
    except Exception as e:
        print(f"✗ ERROR: {e}")
        return False

def create_example(pkg_path, pkg_name):
    """Create example for package"""
    examples_dir = pkg_path / "examples"
    examples_dir.mkdir(exist_ok=True)
    
    example_file = examples_dir / "basic.rs"
    if example_file.exists():
        return  # Already has example
    
    # Generate basic example
    example_code = f"""// Example usage of {pkg_name}

fn main() {{
    println!("{pkg_name} - Basic Example");
    
    // TODO: Add actual usage example
    println!("Package is ready to use!");
}}
"""
    
    example_file.write_text(example_code)
    print(f"  Created example for {pkg_name}")

def main():
    packages_dir = Path("packages")
    
    if not packages_dir.exists():
        print("Error: packages directory not found")
        sys.exit(1)
    
    # Find all packages
    packages = []
    for category_dir in packages_dir.iterdir():
        if category_dir.is_dir():
            for pkg_dir in category_dir.iterdir():
                if pkg_dir.is_dir() and (pkg_dir / "Cargo.toml").exists():
                    packages.append(pkg_dir)
    
    print(f"Found {len(packages)} packages\n")
    
    # Test all packages
    print("=" * 60)
    print("TESTING PACKAGES")
    print("=" * 60)
    
    passed = 0
    failed = 0
    
    for pkg in sorted(packages):
        if test_package(pkg):
            passed += 1
        else:
            failed += 1
    
    print(f"\n{'=' * 60}")
    print(f"Test Results: {passed} passed, {failed} failed")
    print(f"{'=' * 60}\n")
    
    # Create examples
    print("=" * 60)
    print("CREATING EXAMPLES")
    print("=" * 60)
    
    for pkg in sorted(packages):
        create_example(pkg, pkg.name)
    
    print(f"\n{'=' * 60}")
    print(f"✅ Complete! {len(packages)} packages tested and examples created")
    print(f"{'=' * 60}")

if __name__ == "__main__":
    main()
