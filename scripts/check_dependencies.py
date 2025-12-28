#!/usr/bin/env python3
"""
Dependency monitoring script for GUL project.
Checks for unmaintained, deprecated, or vulnerable dependencies.
"""

import subprocess
import json
import sys
from datetime import datetime

def run_command(cmd):
    """Run shell command and return output"""
    try:
        result = subprocess.run(
            cmd,
            shell=True,
            capture_output=True,
            text=True,
            check=True
        )
        return result.stdout
    except subprocess.CalledProcessError as e:
        return e.stderr

def check_cargo_audit():
    """Run cargo audit and parse results"""
    print("🔍 Running cargo audit...")
    output = run_command("cargo audit --json 2>/dev/null || cargo audit")
    
    try:
        data = json.loads(output)
        vulnerabilities = data.get("vulnerabilities", {}).get("list", [])
        warnings = data.get("warnings", {}).get("unmaintained", [])
        
        return {
            "vulnerabilities": len(vulnerabilities),
            "warnings": len(warnings),
            "details": {
                "vulnerabilities": vulnerabilities,
                "warnings": warnings
            }
        }
    except json.JSONDecodeError:
        # Fall back to text parsing
        vuln_count = output.count("Vulnerability")
        warn_count = output.count("unmaintained")
        return {
            "vulnerabilities": vuln_count,
            "warnings": warn_count,
            "output": output
        }

def check_outdated():
    """Check for outdated dependencies"""
    print("📦 Checking for outdated dependencies...")
    output = run_command("cargo update --dry-run 2>&1")
    
    if "Locking 0 packages" in output:
        return {"outdated": 0, "message": "All dependencies up to date"}
    
    # Count packages that would be updated
    lines = [l for l in output.split('\n') if 'Updating' in l]
    return {"outdated": len(lines), "packages": lines}

def generate_report():
    """Generate dependency monitoring report"""
    print("=" * 60)
    print("GUL Dependency Monitoring Report")
    print(f"Generated: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")
    print("=" * 60)
    
    # Check security
    audit_results = check_cargo_audit()
    print(f"\n🔒 Security Audit:")
    print(f"   Vulnerabilities: {audit_results['vulnerabilities']}")
    print(f"   Warnings: {audit_results['warnings']}")
    
    if audit_results['vulnerabilities'] > 0:
        print("   ⚠️  ACTION REQUIRED: Security vulnerabilities found!")
        sys.exit(1)
    elif audit_results['warnings'] > 0:
        print(f"   ℹ️  {audit_results['warnings']} unmaintained dependencies (monitored)")
    else:
        print("   ✅ No security issues found")
    
    # Check outdated
    outdated_results = check_outdated()
    print(f"\n📦 Dependency Status:")
    if outdated_results['outdated'] == 0:
        print("   ✅ All dependencies up to date")
    else:
        print(f"   📊 {outdated_results['outdated']} packages can be updated")
    
    print("\n" + "=" * 60)
    print("Status: PASS ✅" if audit_results['vulnerabilities'] == 0 else "Status: FAIL ❌")
    print("=" * 60)
    
    return audit_results['vulnerabilities'] == 0

if __name__ == "__main__":
    success = generate_report()
    sys.exit(0 if success else 1)
