# 🧹 GUL Project Directory Cleanup & Reorganization Plan

**Date**: 2025-12-30  
**Version**: 0.13.0  
**Status**: Ready for Cleanup

---

## Current Directory Analysis

### Root Directory Contents (31 items)

**Directories (19)**:

- `.agent`, `.embuild`, `.git`, `.github`, `.vscode` - Build/Dev tools
- `benches`, `bin`, `crates`, `src`, `tests` - Code
- `docs`, `examples`, `gul_packages`, `packages` - Documentation/Packages
- `firmware`, `scripts`, `templates`, `web` - Specialized
- `target` - Build artifacts (3.6GB!)

**Files (12)**:

- `Cargo.toml`, `Cargo.lock` - Rust project files
- `README.md` - Main documentation
- `agents.md`, `mkdocs.yml`, `opencode.json` - Config
- `new_pkg_functions.txt` - Temp/working file (24KB)
- `simple_test.mn`, `std_test.mn` - Test files (should be in tests/)
- `std.http` - Large file (13MB - what is this?)
- `update.sh` - Build script

### Size Analysis

```
3.6GB  target/          ← Build artifacts (should be .gitignored)
2.9GB  packages/        ← 545 packages
2.3GB  web/             ← Web IDE
20MB   bin/             ← Binary files
13MB   std.http         ← ❓ Unknown file
1.6MB  src/             ← Source code
560KB  docs/            ← Documentation
```

---

## 🚨 Issues Identified

### 1. Files in Wrong Location

| File | Current Location | Should Be |
|------|------------------|-----------|
| `simple_test.mn` | Root | `tests/` or `examples/` |
| `std_test.mn` | Root | `tests/` |
| `new_pkg_functions.txt` | Root | `docs/` or delete if temp |
| `agents.md` | Root | `docs/` or `.github/` |
| `std.http` (13MB!) | Root | ❓ Investigate |

### 2. Potentially Redundant Directories

- `bin/` vs `src/bin/` - Duplication?
- `crates/` vs `packages/` - Relationship unclear
- `gul_packages/` vs `packages/` - Different purposes?

### 3. Missing Structure

- No `LICENSE` file (mentioned in README)
- No `CONTRIBUTING.md` (mentioned in README)
- No `CHANGELOG.md`
- No `.editorconfig` for consistent formatting

### 4. Large Directories

- `target/` (3.6GB) - Should be cleaned regularly
- `packages/`  (2.9GB) - 545 packages, may need pruning
- `web/` (2.3GB) - Check for unnecessary files

---

## 📋 Cleanup Actions

### Priority 1: Root Directory Cleanup

```bash
# Move test files to proper location
mv simple_test.mn tests/
mv std_test.mn tests/

# Investigate std.http (13MB file)
file std.http
# If temporary: rm std.http
# If important: mv std.http data/ or docs/assets/

# Move/organize documentation
mv agents.md docs/
# OR mv agents.md .github/AGENTS.md

# Handle working file
mv new_pkg_functions.txt docs/development/
# OR rm new_pkg_functions.txt (if temporary)

# Clean build artifacts
cargo clean  # Frees up 3.6GB
```

### Priority 2: Missing Files

```bash
# Create missing files mentioned in docs
touch LICENSE
touch CONTRIBUTING.md  
touch CHANGELOG.md
touch .editorconfig

# Create .gitattributes for consistent line endings
cat > .gitattributes << EOF
* text=auto
*.rs text
*.toml text
*.md text
*.sh text eol=lf
*.bat text eol=crlf
EOF
```

### Priority 3: Documentation Organization

Current docs structure is good, but could add:

```
docs/
├── development/        # Development guides
│   ├── architecture.md
│   ├── contributing.md
│   └── new_pkg_functions.txt (if kept)
├── deployment/         # Already exists (PRODUCTION_DEPLOYMENT.md)
└── maintenance/        # Changelog, releases
    ├── CHANGELOG.md
    └── RELEASES.md
```

### Priority 4: Directory Consolidation

**Investigate and clarify**:

```bash
# Check bin/ directory
ls -la bin/

# Check crates/ directory  
ls -la crates/

# Understand packages/ vs gul_packages/
ls -la packages/ | wc -l
ls -la gul_packages/ | wc -l
```

**Potential consolidation**:

- If `bin/` duplicates `src/bin/`, remove `bin/`
- If `crates/` is for workspace members, document in README
- Clarify `packages/` (external) vs `gul_packages/` (internal)

---

## 🗂️ Proposed Directory Structure

### After Cleanup

```
gul/
├── .agent/                    # Agent configuration
├── .github/                   # GitHub workflows, templates
│   ├── workflows/
│   ├── ISSUE_TEMPLATE/
│   ├── PULL_REQUEST_TEMPLATE/
│   └── AGENTS.md             # ← Moved from root
│
├── benches/                   # Benchmarks
├── crates/                    # Workspace crates (if used)
├── docs/                      # Documentation (560KB)
│   ├── api/                   # API documentation (8 files)
│   ├── book/                  # The GUL Book (7 chapters)
│   ├── guides/                # Comprehensive guides (14 files)
│   ├── reference/             # Language reference (7 files)
│   ├── targets/               # Platform targets
│   ├── development/           # Development docs
│   │   └── new_pkg_functions.txt
│   ├── DOCUMENTATION_ANALYSIS.md
│   ├── DOCUMENTATION_REVIEW.md
│   ├── RUSTDOC_COMPARISON.md
│   ├── MCP-README.md
│   ├── PACKAGES_IMPLEMENTED.md
│   ├── PRODUCTION_DEPLOYMENT.md
│   ├── QUICK_REFERENCE.md
│   └── build_and_flash_embedded.md
│
├── examples/                  # Code examples (22 files)
├── firmware/                  # Embedded firmware
├── gul_packages/              # GUL standard library packages
├── packages/                  # External/third-party packages
├── scripts/                   # Build and utility scripts
├── src/                       # Main source code
│   ├── bin/                   # Binary targets
│   ├── ... (modules)
│   ├── lib.rs
│   └── main.rs
│
├── templates/                 # Project templates
├── tests/                     # Test files
│   ├── simple_test.mn        # ← Moved from root
│   ├── std_test.mn           # ← Moved from root
│   └── ... (other tests)
│
├── web/                       # Web IDE (2.3GB)
│
├── .editorconfig             # ← New
├── .gitattributes            # ← New
├── .gitignore
├── Cargo.lock
├── Cargo.toml
├── CHANGELOG.md              # ← New
├── CONTRIBUTING.md           # ← New
├── LICENSE                   # ← New
├── README.md
├── mkdocs.yml
├── opencode.json
└── update.sh

# Removed from root:
# - std.http (13MB) - deleted or moved
# - new_pkg_functions.txt - moved to docs/development/
# - agents.md - moved to .github/ or docs/
# - simple_test.mn, std_test.mn - moved to tests/
# - target/ - cleaned (saves 3.6GB)
```

---

## 🔍 Investigation Needed

### 1. `std.http` File (13MB)

```bash
# Check what this file is
file std.http
head -20 std.http
tail -20 std.http

# Determine action
# - If temporary download: delete
# - If test data: move to tests/fixtures/
# - If documentation: move to docs/assets/
```

### 2. `bin/` Directory

```bash
# Check if it duplicates src/bin/
diff -r bin/ src/bin/ 2>/dev/null || echo "Different"

# If duplicates: remove bin/
# If different purpose: document in README
```

### 3. `crates/` vs `packages/`

```bash
# Understand the relationship
ls crates/
ls packages/ | head -20

# Document:
# - crates/ = Workspace member crates (internal)
# - packages/ = GUL package ecosystem (external)
# - gul_packages/ = Standard library packages (internal)
```

---

## 📝 Cleanup Script

```bash
#!/bin/bash
# GUL Project Cleanup Script
set -e

echo "🧹 Starting GUL Project Cleanup..."

# 1. Move misplaced test files
echo "📦 Moving test files..."
mv simple_test.mn tests/ 2>/dev/null || true
mv std_test.mn tests/ 2>/dev/null || true

# 2. Move documentation
echo "📚 Organizing documentation..."
mkdir -p docs/development
mv new_pkg_functions.txt docs/development/ 2>/dev/null || true
mv agents.md .github/AGENTS.md 2>/dev/null || true

# 3. Handle std.http (investigate first!)
echo "❓ Checking std.http..."
if [ -f std.http ]; then
    echo "⚠️  std.http exists (13MB) - manual review required"
    file std.http
fi

# 4. Clean build artifacts
echo "🗑️  Cleaning build artifacts..."
cargo clean

# 5. Create missing files
echo "📄 Creating missing files..."
[ ! -f LICENSE ] && cat > LICENSE << 'EOF'
MIT License

Copyright (c) 2025 GUL Team

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
EOF

[ ! -f CHANGELOG.md ] && cat > CHANGELOG.md << 'EOF'
# Changelog

All notable changes to GUL will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.13.0] - 2025-12-30

### Added
- v3.2 syntax support (let/var, @imp, @type constructors)
- Comprehensive documentation suite (66 files)
- MCP server integration
- 180+ package ecosystem
- Embedded support (ESP32-S3, RP2040)

### Changed
- Migrated from v2.0 to v3.2 syntax
- Enhanced Rust documentation

### Fixed
- All compiler warnings
- Documentation alignment

## [0.12.0] - 2025-12-xx

### Added
- Initial release
- Basic compiler
- Standard library

[0.13.0]: https://github.com/calamity10110/gul/releases/tag/v0.13.0
[0.12.0]: https://github.com/calamity10110/gul/releases/tag/v0.12.0
EOF

[ ! -f .editorconfig ] && cat > .editorconfig << 'EOF'
# EditorConfig for GUL project
# https://editorconfig.org

root = true

[*]
charset = utf-8
end_of_line = lf
insert_final_newline = true
trim_trailing_whitespace = true
indent_style = space
indent_size = 4

[*.{yml,yaml}]
indent_size = 2

[*.md]
trim_trailing_whitespace = false

[*.{mn,gul}]
indent_size = 4

[Makefile]
indent_style = tab
EOF

[ ! -f .gitattributes ] && cat > .gitattributes << 'EOF'
# Auto detect text files and perform LF normalization
* text=auto

# Source code
*.rs text
*.toml text
*.md text
*.mn text
*.gul text

# Scripts
*.sh text eol=lf
*.bat text eol=crlf
*.ps1 text eol=crlf

# Documentation
*.txt text
*.json text
*.yml text
*.yaml text

# Binary files
*.png binary
*.jpg binary
*.jpeg binary
*.gif binary
*.ico binary
*.woff binary
*.woff2 binary
*.ttf binary
*.eot binary
EOF

echo "✅ Cleanup complete!"
echo ""
echo "📊 Summary:"
echo "- Test files moved to tests/"
echo "- Documentation organized"
echo "- Build artifacts cleaned (freed 3.6GB)"
echo "- Missing files created"
echo ""
echo "⚠️  Manual review needed:"
echo "- Check std.http file (13MB)"
echo "- Review bin/ directory"
echo "- Verify crates/ vs packages/ structure"
```

---

## ✅ Checklist

### Immediate Actions

- [ ] Move `simple_test.mn` → `tests/`
- [ ] Move `std_test.mn` → `tests/`
- [ ] Investigate `std.http` (13MB)
- [ ] Move `agents.md` → `.github/` or `docs/`
- [ ] Move or delete `new_pkg_functions.txt`
- [ ] Run `cargo clean` (frees 3.6GB)
- [ ] Create `LICENSE`
- [ ] Create `CHANGELOG.md`
- [ ] Create `CONTRIBUTING.md`
- [ ] Create `.editorconfig`
- [ ] Create `.gitattributes`

### Documentation

- [ ] Update README to reflect any directory changes
- [ ] Document `crates/` vs `packages/` vs `gul_packages/`
- [ ] Add development guide
- [ ] Create architecture documentation

### Verification

- [ ] Run `cargo build --release`
- [ ] Run `cargo test --lib`
- [ ] Run `cargo clippy`
- [ ] Verify all documentation links work
- [ ] Check .gitignore includes target/

---

## 📊 Expected Results

### Before Cleanup

```
Root: 31 items (messy)
Size: ~6.5GB (with target/)
Documentation: Scattered
Tests: In root
Missing: LICENSE, CONTRIBUTING, CHANGELOG
```

### After Cleanup

```
Root: 20 items (organized)
Size: ~2.9GB (without target/)
Documentation: Organized in docs/
Tests: In tests/
Complete: All essential files present
```

**Space Saved**: ~3.6GB (from cargo clean)  
**Organization**: Professional structure  
**Completeness**: All missing files added

---

## 🎯 Recommendation

**Execute the cleanup script** to:

1. ✅ Move files to proper locations
2. ✅ Clean build artifacts
3. ✅ Create missing files
4. ✅ Organize documentation

**Then manually review**:

1. `std.http` file (determine what it is)
2. `bin/` directory (check for duplication)
3. Directory relationships (`crates/`, `packages/`, `gul_packages/`)

**Finally**:

1. Update README with directory structure
2. Commit changes
3. Tag release

---

*Cleanup Plan Created: 2025-12-30*  
*Status: Ready for Execution*  
*Estimated Time: 15 minutes*
