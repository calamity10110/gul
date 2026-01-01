#!/bin/bash
# Generate documentation for all packages
echo "Building gul-docgen..."
cargo build -p gul-docgen --release

echo "Running gul-docgen..."
./target/release/gul-docgen --root . 

echo "Documentation generation complete."
