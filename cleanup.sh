#!/bin/bash
# GUL Project Cleanup Script
# Version: 1.0
# Date: 2025-12-30

set -e

echo "🧹 Starting GUL Project Cleanup..."
echo ""

# 1. Move misplaced test files
echo "📦 Moving test files..."
[ -f simple_test.mn ] && mv simple_test.mn tests/ && echo "  ✅ Moved simple_test.mn → tests/"
[ -f std_test.mn ] && mv std_test.mn tests/ && echo "  ✅ Moved std_test.mn → tests/"

# 2. Move documentation
echo "📚 Organizing documentation..."
mkdir -p docs/development
[ -f new_pkg_functions.txt ] && mv new_pkg_functions.txt docs/development/ && echo "  ✅ Moved new_pkg_functions.txt → docs/development/"
[ -f agents.md ] && mv agents.md .github/AGENTS.md && echo "  ✅ Moved agents.md → .github/AGENTS.md"

# 3. Handle std.http (PostScript image - not needed in root)
echo "🖼️  Moving std.http..."
mkdir -p docs/assets
[ -f std.http ] && mv std.http docs/assets/ && echo "  ✅ Moved std.http → docs/assets/"

# 4. Clean build artifacts
echo "🗑️  Cleaning build artifacts (saves ~3.6GB)..."
cargo clean
echo "  ✅ Build artifacts cleaned"

echo ""
echo "============================================"
echo "✅ Cleanup Phase 1 Complete!"
echo "============================================"
echo ""
echo "📊 Summary:"
echo "  - Test files moved to tests/"
echo "  - Documentation organized in docs/"
echo "  - Build artifacts cleaned (freed ~3.6GB)"
echo ""
echo "Next: Run ./create_missing_files.sh to add LICENSE, CHANGELOG, etc."
echo ""
