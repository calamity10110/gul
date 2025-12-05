#!/bin/bash
# Package Generator Script for GUL

create_package() {
    local category=$1
    local name=$2
    local description=$3
    local keywords=$4
    
    local pkg_dir="packages/$category/$name"
    
    echo "Creating $name..."
    mkdir -p "$pkg_dir/src"
    
    # Create Cargo.toml
    cat > "$pkg_dir/Cargo.toml" <<EOF
[package]
name = "$name"
version = "0.1.0"
edition = "2021"
authors = ["GUL Team <team@gul-lang.org>"]
description = "$description"
license = "MIT"
repository = "https://github.com/gul-lang/packages"
keywords = [$keywords]
categories = ["development-tools"]

[dependencies]

[dev-dependencies]
EOF

    # Create basic lib.rs
    cat > "$pkg_dir/src/lib.rs" <<EOF
// $name - $description

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
EOF
}

# Batch 1: Utilities
create_package "utils" "gul-regex" "Regular expression support for GUL" '"regex", "pattern", "gul"'
create_package "utils" "gul-crypto" "Cryptography library for GUL" '"crypto", "encryption", "gul"'
create_package "utils" "gul-yaml" "YAML parsing for GUL" '"yaml", "parser", "gul"'
create_package "utils" "gul-toml" "TOML parsing for GUL" '"toml", "parser", "gul"'
create_package "utils" "gul-xml" "XML parsing for GUL" '"xml", "parser", "gul"'
create_package "utils" "gul-csv" "CSV parsing for GUL" '"csv", "parser", "gul"'
create_package "utils" "gul-compress" "Compression library for GUL" '"compress", "zip", "gul"'
create_package "utils" "gul-hash" "Hashing algorithms for GUL" '"hash", "crypto", "gul"'

echo "Batch 1 complete!"
