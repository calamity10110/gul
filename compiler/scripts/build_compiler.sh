#!/bin/bash
# Build the GUL compiler from GUL source

set -e

echo "🚀 Building GUL Compiler from GUL Source"
echo "========================================"
echo ""

# Step 1: Transpile GUL to Rust
echo "Step 1: Transpiling GUL → Rust..."
python3 compiler/scripts/bootstrap_transpiler.py

echo ""
echo "Step 2: Creating Rust project structure..."

# Create Cargo.toml for the transpiled compiler
cat > compiler_rust/Cargo.toml << 'EOF'
[workspace]

[package]
name = "gul-compiler"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "gul-compile"
path = "main.rs"

[dependencies]
EOF

echo "✅ Cargo.toml created"

echo ""
echo "Step 3: Compiling Rust code..."
cd compiler_rust

# Try to build
if cargo build --release 2>&1 | tee build.log; then
    echo ""
    echo "🎉 SUCCESS! GUL compiler built successfully!"
    echo ""
    echo "Compiler location: compiler_rust/target/release/gul-compile"
    echo ""
    echo "Test it with:"
    echo "  ./target/release/gul-compile --help"
else
    echo ""
    echo "⚠️  Build had errors. Check build.log for details."
    echo ""
    echo "This is expected - the bootstrap transpiler is minimal."
    echo "Manual fixes may be needed in the generated Rust code."
fi

cd ..

echo ""
echo "Build process complete!"
