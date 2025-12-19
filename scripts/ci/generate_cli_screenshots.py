#!/usr/bin/env python3
"""Generate CLI screenshots for documentation."""

import os
import subprocess
import sys


def run_command(cmd: str) -> str:
    """Run a shell command and capture output."""
    try:
        result = subprocess.run(cmd, shell=True, capture_output=True, text=True)
        return result.stdout + result.stderr
    except Exception as e:
        return f"Error: {e}"


def generate_screenshot(name: str, command: str, output_dir: str):
    """Generate a screenshot of a CLI command."""
    # For CI, just log what would be captured
    print(f"Would capture screenshot for: {command}")
    
    output_path = os.path.join(output_dir, f"{name}.txt")
    output = run_command(command)
    
    os.makedirs(output_dir, exist_ok=True)
    with open(output_path, 'w') as f:
        f.write(f"$ {command}\n{output}")
    
    print(f"Saved output to {output_path}")


def main():
    """Main entry point."""
    output_dir = os.environ.get('OUTPUT_DIR', 'docs/assets/screenshots/cli')
    
    commands = [
        ("help", "cargo run -- --help"),
        ("version", "cargo run -- --version"),
        ("check", "cargo run -- check examples/hello.mn 2>&1 || true"),
    ]
    
    for name, cmd in commands:
        generate_screenshot(name, cmd, output_dir)
    
    print("CLI screenshots generated!")


if __name__ == "__main__":
    main()
