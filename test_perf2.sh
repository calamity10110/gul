#!/bin/bash
echo "cargo build checking"
grep -n "cargo build" .github/workflows/*.yml

echo "cargo test checking"
grep -n "cargo test" .github/workflows/*.yml
