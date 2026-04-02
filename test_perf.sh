#!/bin/bash
echo "Looking for common CI/CD performance anti-patterns in .github/workflows..."

grep -n "cargo build" .github/workflows/*.yml | grep -v "--release"
echo "---"
grep -n "cargo test" .github/workflows/*.yml | grep "--release"
echo "---"
grep -n "npm install" .github/workflows/*.yml | grep -v "ci"
echo "---"
grep -n "pip install" .github/workflows/*.yml
