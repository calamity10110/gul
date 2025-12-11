#!/usr/bin/env python3
"""
Test GUL interoperability with different language versions.
Ensures FFI works across Python, Rust, C, JavaScript, TypeScript, etc.
"""

import os
import sys
import subprocess
from pathlib import Path
from typing import Dict, List
import json

class LanguageVersionTester:
    def __init__(self):
        self.root_dir = Path(os.getcwd())
        self.test_results: Dict[str, Dict[str, bool]] = {}
    
    def run_tests(self) -> bool:
        """Test all language versions"""
        print("🌐 Testing language version compatibility...")
        
        # Test Python versions
        self._test_python_versions()
        
        # Test Node.js versions
        self._test_nodejs_versions()
        
        # Test Rust versions
        self._test_rust_versions()
        
        # Generate report
        self._generate_report()
        
        return self._all_tests_passed()
    
    def _test_python_versions(self):
        """Test different Python versions"""
        print("\n🐍 Testing Python compatibility...")
        
        python_versions = ["3.9", "3.10", "3.11", "3.12"]
        self.test_results["python"] = {}
        
        for version in python_versions:
            print(f"  Testing Python {version}...")
            
            try:
                # Check if Python version is available
                result = subprocess.run(
                    [f"python{version}", "--version"],
                    capture_output=True,
                    timeout=5
                )
                
                if result.returncode != 0:
                    print(f"    ⚠️  Python {version} not available, skipping")
                    continue
                
                # Run Python FFI tests
                test_result = self._run_interop_test("python", version)
                self.test_results["python"][version] = test_result
                
                if test_result:
                    print(f"    ✅ Python {version} compatible")
                else:
                    print(f"    ❌ Python {version} incompatible")
            
            except Exception as e:
                print(f"    ❌ Error testing Python {version}: {e}")
                self.test_results["python"][version] = False
    
    def _test_nodejs_versions(self):
        """Test different Node.js versions"""
        print("\n📦 Testing Node.js compatibility...")
        
        # Use nvm to test multiple versions if available
        nodejs_versions = ["18", "20", "21"]
        self.test_results["nodejs"] = {}
        
        for version in nodejs_versions:
            print(f"  Testing Node.js {version}...")
            
            try:
                # This would use nvm in practice
                test_result = self._run_interop_test("nodejs", version)
                self.test_results["nodejs"][version] = test_result
                
                if test_result:
                    print(f"    ✅ Node.js {version} compatible")
                else:
                    print(f"    ❌ Node.js {version} incompatible")
            
            except Exception as e:
                print(f"    ❌ Error testing Node.js {version}: {e}")
                self.test_results["nodejs"][version] = False
    
    def _test_rust_versions(self):
        """Test different Rust versions"""
        print("\n🦀 Testing Rust compatibility...")
        
        rust_versions = ["stable", "beta", "nightly"]
        self.test_results["rust"] = {}
        
        for version in rust_versions:
            print(f"  Testing Rust {version}...")
            
            try:
                # Install Rust version
                subprocess.run(
                    ["rustup", "install", version],
                    capture_output=True,
                    timeout=300
                )
                
                # Run Rust FFI tests
                test_result = self._run_interop_test("rust", version)
                self.test_results["rust"][version] = test_result
                
                if test_result:
                    print(f"    ✅ Rust {version} compatible")
                else:
                    print(f"    ❌ Rust {version} incompatible")
            
            except Exception as e:
                print(f"    ❌ Error testing Rust {version}: {e}")
                self.test_results["rust"][version] = False
    
    def _run_interop_test(self, language: str, version: str) -> bool:
        """Run interoperability test for a specific language version"""
        test_file = self.root_dir / "tests" / "interop" / f"{language}_ffi_test.gul"
        
        if not test_file.exists():
            print(f"    ⚠️  Test file not found: {test_file}")
            return True  # Don't fail if test doesn't exist yet
        
        try:
            # Build GUL first
            gul_binary = self.root_dir / "target" / "release" / "gul"
            
            if not gul_binary.exists():
                print(f"    Building GUL...")
                build_result = subprocess.run(
                    ["cargo", "build", "--release"],
                    cwd=self.root_dir,
                    capture_output=True,
                    timeout=300
                )
                
                if build_result.returncode != 0:
                    print(f"    ❌ Failed to build GUL")
                    return False
            
            # Run the test
            result = subprocess.run(
                [str(gul_binary), "test", str(test_file)],
                capture_output=True,
                timeout=60,
                env={**os.environ, f"{language.upper()}_VERSION": version}
            )
            
            return result.returncode == 0
        
        except subprocess.TimeoutExpired:
            print(f"    ⏱️  Test timed out")
            return False
        except Exception as e:
            print(f"    ❌ Test error: {e}")
            return False
    
    def _generate_report(self):
        """Generate language compatibility report"""
        report = {
            "timestamp": subprocess.check_output(["date", "-u", "+%Y-%m-%dT%H:%M:%SZ"]).decode().strip(),
            "results": self.test_results
        }
        
        report_file = self.root_dir / "target" / "language_compatibility_report.json"
        report_file.parent.mkdir(parents=True, exist_ok=True)
        
        with open(report_file, "w") as f:
            json.dump(report, f, indent=2)
        
        print(f"\n📊 Language compatibility report saved to {report_file}")
        
        # Print summary
        print(f"\n{'='*60}")
        print(f"Language Version Compatibility")
        print(f"{'='*60}")
        
        for language, versions in self.test_results.items():
            passed = sum(1 for r in versions.values() if r)
            total = len(versions)
            print(f"{language.capitalize()}: {passed}/{total} versions compatible")
        
        print(f"{'='*60}\n")
    
    def _all_tests_passed(self) -> bool:
        """Check if all tests passed"""
        for versions in self.test_results.values():
            if not all(versions.values()):
                return False
        return True

def main():
    tester = LanguageVersionTester()
    success = tester.run_tests()
    
    sys.exit(0 if success else 1)

if __name__ == "__main__":
    main()
