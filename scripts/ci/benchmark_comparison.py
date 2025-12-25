#!/usr/bin/env python3
"""
Compare benchmark results with baseline.

This script compares current benchmark results against a stored baseline
to detect performance regressions.
"""

import os
import sys
import json
from pathlib import Path
from datetime import datetime


def load_criterion_results(criterion_dir: Path) -> dict:
    """Load benchmark results from Criterion output."""
    results = {}
    
    if not criterion_dir.exists():
        return results
    
    for bench_dir in criterion_dir.iterdir():
        if not bench_dir.is_dir():
            continue
        
        # Look for estimates.json in new format
        for estimates_file in bench_dir.rglob("estimates.json"):
            try:
                with open(estimates_file) as f:
                    data = json.load(f)
                
                bench_name = bench_dir.name
                results[bench_name] = {
                    "mean": data.get("mean", {}).get("point_estimate"),
                    "median": data.get("median", {}).get("point_estimate"),
                    "std_dev": data.get("std_dev", {}).get("point_estimate"),
                }
            except Exception as e:
                print(f"Warning: Could not load {estimates_file}: {e}")
    
    return results


def load_baseline(baseline_path: Path) -> dict:
    """Load baseline benchmark results."""
    if baseline_path.exists():
        try:
            with open(baseline_path) as f:
                return json.load(f)
        except Exception:
            pass
    return {}


def save_baseline(baseline_path: Path, results: dict):
    """Save current results as new baseline."""
    baseline_path.parent.mkdir(parents=True, exist_ok=True)
    
    with open(baseline_path, "w") as f:
        json.dump({
            "timestamp": datetime.now().isoformat(),
            "results": results
        }, f, indent=2)


def compare_results(current: dict, baseline: dict, threshold: float = 0.10) -> list[dict]:
    """Compare current results against baseline."""
    regressions = []
    
    baseline_results = baseline.get("results", {})
    
    for bench_name, current_data in current.items():
        if bench_name not in baseline_results:
            continue
        
        baseline_data = baseline_results[bench_name]
        
        if not current_data.get("mean") or not baseline_data.get("mean"):
            continue
        
        current_mean = current_data["mean"]
        baseline_mean = baseline_data["mean"]
        
        # Calculate percentage change (positive = slower)
        change = (current_mean - baseline_mean) / baseline_mean
        
        if change > threshold:
            regressions.append({
                "benchmark": bench_name,
                "baseline_ns": baseline_mean,
                "current_ns": current_mean,
                "change_percent": change * 100,
                "threshold_percent": threshold * 100
            })
    
    return regressions


def format_duration(ns: float) -> str:
    """Format nanoseconds as human-readable duration."""
    if ns < 1000:
        return f"{ns:.2f}ns"
    elif ns < 1_000_000:
        return f"{ns/1000:.2f}µs"
    elif ns < 1_000_000_000:
        return f"{ns/1_000_000:.2f}ms"
    else:
        return f"{ns/1_000_000_000:.2f}s"


def main():
    """Main entry point."""
    project_root = Path(__file__).parent.parent.parent
    criterion_dir = project_root / "target" / "criterion"
    baseline_path = project_root / ".benchmark_baseline.json"
    
    # Load current results
    current_results = load_criterion_results(criterion_dir)
    
    if not current_results:
        print("No benchmark results found in target/criterion")
        print("Run `cargo bench` first to generate benchmark results")
        sys.exit(0)
    
    print(f"Found {len(current_results)} benchmark results")
    
    # Load baseline
    baseline = load_baseline(baseline_path)
    
    if not baseline:
        print("No baseline found, saving current results as baseline")
        save_baseline(baseline_path, current_results)
        sys.exit(0)
    
    print(f"Baseline from: {baseline.get('timestamp', 'unknown')}")
    
    # Compare results
    threshold = float(os.environ.get("BENCHMARK_THRESHOLD", "0.10"))
    regressions = compare_results(current_results, baseline, threshold)
    
    # Report results
    if regressions:
        print("")
        print(f"❌ Performance regressions detected (threshold: {threshold*100:.0f}%):")
        print("")
        
        for reg in regressions:
            print(f"  {reg['benchmark']}:")
            print(f"    Baseline: {format_duration(reg['baseline_ns'])}")
            print(f"    Current:  {format_duration(reg['current_ns'])}")
            print(f"    Change:   +{reg['change_percent']:.1f}%")
            print("")
        
        # Don't fail the build, just warn
        print("⚠️  Please review these regressions before merging")
        sys.exit(0)  # Change to sys.exit(1) to fail on regressions
    else:
        print("✅ No performance regressions detected")
        
        # Update baseline on success (optional)
        if os.environ.get("UPDATE_BASELINE", "").lower() == "true":
            save_baseline(baseline_path, current_results)
            print("Baseline updated with current results")


if __name__ == "__main__":
    main()
