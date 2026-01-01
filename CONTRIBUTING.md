# Contributing to GUL

Thank you for your interest in contributing to GUL! This document provides guidelines and instructions for contributing.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Coding Standards](#coding-standards)
- [Testing](#testing)
- [Documentation](#documentation)
- [Submitting Changes](#submitting-changes)

---

## Code of Conduct

We are committed to providing a welcoming and inclusive experience for everyone. Please be respectful and constructive in all interactions.

---

## Getting Started

### Prerequisites

- **Rust**: 1.70 or later
- **Cargo**: Latest stable
- **Git**: For version control

### Setup

1. **Fork** the repository
2. **Clone** your fork:

   ```bash
   git clone https://github.com/YOUR_USERNAME/gul.git
   cd gul
   ```

3. **Build** the project:

   ```bash
   cargo build
   ```

4. **Run tests**:

   ```bash
   cargo test --lib
   ```

---

## Development Workflow

### 1. Create a Branch

```bash
git checkout -b feature/your-feature-name
```

Branch naming conventions:

- `feature/` - New features
- `fix/` - Bug fixes
- `docs/` - Documentation changes
- `refactor/` - Code refactoring
- `test/` - Test additions/changes

### 2. Make Changes

- Write clean, readable code
- Follow the coding standards below
- Add tests for new functionality
- Update documentation as needed

### 3. Test Your Changes

```bash
# Run all tests
cargo test --lib

# Run specific tests
cargo test --lib test_name

# Run clippy
cargo clippy

# Format code
cargo fmt
```

### 4. Commit Changes

```bash
git add .
git commit -m "type: brief description

Detailed explanation of changes (if needed)

Fixes #issue_number (if applicable)"
```

Commit message types:

- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation changes
- `style:` - Code style changes (formatting)
- `refactor:` - Code refactoring
- `test:` - Test changes
- `chore:` - Build/tooling changes

### 5. Push and Create Pull Request

```bash
git push origin feature/your-feature-name
```

Then create a Pull Request on GitHub.

---

## Coding Standards

### Rust Code

- **Follow Rust conventions**: Use `cargo fmt` and `cargo clippy`
- **Naming**:
  - `snake_case` for variables and functions
  - `PascalCase` for types and structs
  - `SCREAMING_SNAKE_CASE` for constants
- **Comments**:
  - Use `///` for doc comments
  - Use `//` for inline comments
  - Explain *why*, not *what*
- **Error Handling**:
  - Use `Result<T, E>` for recoverable errors
  - Use `panic!` only for unrecoverable errors
  - Provide meaningful error messages

### GUL Code (v3.2 Syntax)

- Use **`let`** for immutable variables
- Use **`var`** for mutable variables
- Use **`@imp`** for imports
- Use **`@type`** constructors (e.g., `@int`, `@str`)
- Use **`mn:`** for main entry point
- Use **`async`** (not `asy`) for async functions

Example:

```gul
@imp std.io

let name = "GUL"
var count = 0

fn greet(name: str) -> str:
    return "Hello, " + name

mn:
    print(greet(name))
```

---

## Testing

### Unit Tests

Place tests in the same file or a `tests` module:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature() {
        let result = my_function();
        assert_eq!(result, expected);
    }
}
```

### Integration Tests

Place in `tests/` directory:

```rust
// tests/integration_test.rs
use gul_lang::*;

#[test]
fn test_integration() {
    // Test code
}
```

### Test Coverage

- **Aim for 80%+ coverage** for new code
- Test edge cases and error conditions
- Include both positive and negative tests

---

## Documentation

### Code Documentation

- Add doc comments (`///`) to all public items
- Include examples in doc comments:

```rust
/// Adds two numbers together.
///
/// # Examples
///
/// ```
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

### Documentation Files

- **README.md** - Project overview
- **docs/** - Comprehensive guides
- **CHANGELOG.md** - Version history
- **API docs** - Module and function documentation

When adding new features:

1. Update relevant documentation
2. Add examples to `examples/`
3. Update CHANGELOG.md
4. Add rustdoc comments

---

## Submitting Changes

### Pull Request Checklist

Before submitting a PR, ensure:

- [ ] Code compiles: `cargo build`
- [ ] Tests pass: `cargo test --lib`
- [ ] No warnings: `cargo clippy`
- [ ] Code is formatted: `cargo fmt`
- [ ] Documentation is updated
- [ ] CHANGELOG.md is updated
- [ ] Commit messages follow conventions
- [ ] PR description explains changes

### Pull Request Template

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
How was this tested?

## Checklist
- [ ] Tests pass
- [ ] Documentation updated
- [ ] CHANGELOG.md updated
```

### Review Process

1. **Automated checks** run (tests, clippy, fmt)
2. **Code review** by maintainers
3. **Feedback** addressed
4. **Merge** when approved

---

## Development Tips

### Building Documentation

```bash
# Generate Rust docs
cargo doc --open

# View project docs
cd docs && python -m http.server 8000
```

### Running Examples

```bash
# Run specific example
gul run examples/hello_world.mn

# With cargo
cargo run -- run examples/hello_world.mn
```

### Debugging

```bash
# Run with debug output
RUST_LOG=debug cargo run

# Use rust-gdb or rust-lldb
rust-gdb target/debug/gul
```

### Performance Profiling

```bash
# Run benchmarks
cargo bench

# Profile with perf
perf record ./target/release/gul run script.mn
perf report
```

---

## Getting Help

- **GitHub Issues**: Report bugs or request features
- **Discussions**: Ask questions or share ideas
- **Documentation**: Check [docs/](docs/) directory

---

## License

By contributing to GUL, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing to GUL! 🚀
