# CI/CD Automation Guide

This document describes the comprehensive CI/CD automation system for the GUL project.

## 🎯 Overview

The GUL project uses GitHub Actions for continuous integration and deployment with the following workflows:

1. **Core CI** (`ci.yml`) - Basic testing and quality checks
2. **Package Testing** (`package-testing.yml`) - Package ecosystem validation
3. **Interop Testing** (`interop-testing.yml`) - Multi-language compatibility
4. **Documentation** (`documentation.yml`) - Auto-documentation generation

## 📋 Workflows

### 1. Core CI Workflow

**Triggers**: Push to main/develop, Pull requests

**Jobs**:

- **test**: Run tests on Linux, macOS, Windows
- **test-packages**: Test individual package crates
- **test-no-std**: Test embedded targets
- **lint**: Code formatting and clippy checks
- **security**: Security audit with cargo-audit

**Key Features**:

- Cross-platform testing (Linux, macOS, Windows)
- Caching for faster builds
- Comprehensive test coverage

### 2. Package Ecosystem Testing

**Triggers**: Changes to packages/, Daily schedule

**Jobs**:

- **validate-packages**: Validate package metadata and structure
- **test-compatibility**: Test package compatibility matrix
- **test-gul-packages**: Test GUL standard library packages
- **test-installation**: Test package installation on all platforms
- **package-benchmarks**: Performance regression tests
- **generate-package-docs**: Auto-generate package documentation

**Key Features**:

- Validates 4,128+ packages
- Tests cross-package compatibility
- Ensures packages work together
- Performance monitoring

### 3. Multi-Language Interop Testing

**Triggers**: Changes to interop code, Twice daily

**Jobs**:

- **test-python-ffi**: Test Python 3.9, 3.10, 3.11, 3.12
- **test-rust-ffi**: Test Rust stable, beta, nightly
- **test-c-ffi**: Test C/C++ with GCC and Clang
- **test-js-ts-ffi**: Test Node.js 18, 20, 21
- **test-go-ffi**: Test Go 1.20, 1.21
- **test-multi-lang-integration**: Comprehensive integration tests
- **test-language-updates**: Monitor language version compatibility

**Key Features**:

- Tests FFI across multiple language versions
- Ensures backward compatibility
- Detects breaking changes early

### 4. Documentation Automation

**Triggers**: Changes to docs/, Daily schedule

**Jobs**:

- **validate-docs**: Check markdown, validate links, verify examples
- **generate-api-docs**: Auto-generate API documentation
- **build-docs-site**: Build MkDocs site
- **deploy-docs**: Deploy to GitHub Pages
- **generate-changelog**: Auto-update changelog
- **generate-screenshots**: Create tutorial screenshots

**Key Features**:

- Automated documentation generation
- Link validation
- Code example verification
- Auto-deployment to docs.mn-lang.org

## 🔧 Automation Scripts

### Package Validation

```bash
python scripts/ci/validate_packages.py
```

Validates:

- Package metadata (Cargo.toml, package.json)
- Required fields (name, version, description)
- Directory structure
- Dependencies
- Documentation

### Package Compatibility Testing

```bash
python scripts/ci/test_package_compatibility.py --category web
```

Tests:

- Individual package builds
- Package pair compatibility
- Dependency conflicts
- Version compatibility

### Language Version Testing

```bash
python scripts/ci/test_language_versions.py
```

Tests:

- Python 3.9-3.12 compatibility
- Node.js 18, 20, 21 compatibility
- Rust stable/beta/nightly compatibility
- FFI functionality across versions

### Documentation Validation

```bash
python scripts/ci/validate_code_examples.py
```

Validates:

- Code syntax in documentation
- Runnable examples
- Language-specific validation (GUL, Python, Rust, JS)

## 📊 Reports Generated

All CI runs generate detailed reports:

1. **Compatibility Report** (`target/compatibility_report.json`)

   - Package test results
   - Compatibility matrix
   - Dependency conflicts

2. **Language Compatibility Report** (`target/language_compatibility_report.json`)

   - Language version results
   - FFI test results
   - Version-specific issues

3. **Documentation Report**
   - Link validation results
   - Code example validation
   - Coverage statistics

## 🚀 Running Locally

### Run Core Tests

```bash
cargo test --all-features
cargo clippy --all-targets
cargo fmt -- --check
```

### Run Package Tests

```bash
# Test all packages
python scripts/ci/validate_packages.py

# Test specific category
python scripts/ci/test_package_compatibility.py --category web

# Test installation
cargo install --path .
gul install http
gul test tests/package_install_test.mn
```

### Run Interop Tests

```bash
# Build GUL
cargo build --release

# Test Python FFI
./target/release/gul test tests/interop/python_ffi.mn

# Test all languages
python scripts/ci/test_language_versions.py
```

### Validate Documentation

```bash
# Check links
npx markdown-link-check docs/**/*.md

# Validate code examples
python scripts/ci/validate_code_examples.py

# Build docs site
cd docs && mkdocs build
```

## 🔐 Secrets Required

For full CI/CD functionality, configure these secrets in GitHub:

- `GITHUB_TOKEN` - Automatic (for deployments)
- `CODECOV_TOKEN` - Code coverage reporting
- `DOCS_DEPLOY_KEY` - Documentation deployment (optional)

## 📅 Schedule

- **Daily (2 AM UTC)**: Core tests with full matrix
- **Daily (3 AM UTC)**: Package ecosystem tests
- **Twice Daily (12h)**: Language interop tests
- **Daily (4 AM UTC)**: Documentation updates

## 🎯 Quality Gates

Pull requests must pass:

1. ✅ All tests on all platforms
2. ✅ Code formatting (cargo fmt)
3. ✅ No clippy warnings
4. ✅ Security audit passes
5. ✅ Package validation succeeds
6. ✅ Documentation builds successfully
7. ✅ Code examples are valid

## 🔄 Continuous Deployment

On merge to `main`:

1. Documentation automatically deploys to GitHub Pages
2. Changelog auto-updates
3. Package registry updates (if configured)
4. Release artifacts generated (on tagged releases)

## 📈 Monitoring

Track CI/CD health:

- **GitHub Actions**: View workflow runs
- **Codecov**: Track test coverage
- **Documentation Site**: Monitor doc deployment
- **Package Reports**: Review compatibility matrices

## 🛠️ Maintenance

### Adding New Package Tests

1. Add package to appropriate category in `packages/`
2. CI automatically detects and tests it
3. Update `scripts/ci/test_package_compatibility.py` for specific tests

### Adding New Language Support

1. Create FFI tests in `tests/interop/`
2. Add language version matrix in `interop-testing.yml`
3. Update `test_language_versions.py` with new language

### Updating Documentation

1. Edit files in `docs/`
2. CI validates and deploys automatically
3. Check generated reports for issues

## 📚 Best Practices

1. **Test locally** before pushing
2. **Keep workflows fast** - use caching
3. **Fail early** - run quick checks first
4. **Parallel testing** - maximize concurrency
5. **Clear reporting** - generate detailed logs
6. **Automate everything** - reduce manual work

## 🆘 Troubleshooting

### CI Fails on Package Tests

```bash
# Check package locally
cd packages/category/package-name
cargo test --all-features

# Validate metadata
python scripts/ci/validate_packages.py
```

### Interop Tests Fail

```bash
# Test specific language
./target/release/gul test tests/interop/python_ffi.mn

# Check language version
python3 --version
node --version
rustc --version
```

### Documentation Build Fails

```bash
# Validate links
npx markdown-link-check docs/**/*.md

# Test code examples
python scripts/ci/validate_code_examples.py

# Build locally
cd docs && mkdocs serve
```

---

**Last Updated**: 2025-12-10  
**Maintained By**: GUL CI/CD Team  
**License**: MIT
