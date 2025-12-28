# Creating Packages Tutorial

Learn how to create, publish, and maintain GUL packages for the community.

## 📦 What is a Package?

A GUL package is a reusable library that can be shared and installed via the GUL package registry.

## 🎯 Creating Your First Package

### Step 1: Initialize Package

```bash
mkdir my-package
cd my-package
gul package init
```

This creates:

```
my-package/
├── gul.toml          # Package metadata
├── README.md         # Package documentation
├── src/
│   └── lib.mn       # Main library code
├── tests/
│   └── test_lib.mn  # Tests
└── examples/
    └── basic.mn     # Usage examples
```

### Step 2: Configure Package

Edit `gul.toml`:

```toml
[package]
name = "my-package"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
description = "A useful GUL package"
license = "MIT"
repository = "https://github.com/yourusername/my-package"
keywords = ["utility", "helper"]

[dependencies]
# Add dependencies here

[dev-dependencies]
# Test dependencies
```

### Step 3: Write Library Code

Edit `src/lib.mn`:

```gul
# Public API
export fn greet(name: str): str:
    return f"Hello, {name}!"

export fn add(a: int, b: int): int:
    return a + b

export struct Point:
    x: float
    y: float

    fn distance(self, other: Point): float:
        dx = self.x - other.x
        dy = self.y - other.y
        return math.sqrt(dx ** 2 + dy ** 2)

# Private helper (not exported)
fn internal_helper():
    # Only used within package
    pass
```

### Step 4: Write Tests

Edit `tests/test_lib.mn`:

```gul
import my_package

@test
fn test_greet():
    result = my_package.greet("World")
    assert result == "Hello, World!"

@test
fn test_add():
    assert my_package.add(2, 3) == 5
    assert my_package.add(-1, 1) == 0

@test
fn test_point_distance():
    p1 = my_package.Point { x: 0.0, y: 0.0 }
    p2 = my_package.Point { x: 3.0, y: 4.0 }

    assert p1.distance(p2) == 5.0
```

Run tests:

```bash
gul test
```

### Step 5: Add Examples

Edit `examples/basic.mn`:

```gul
import my_package

main:
    # Greet function
    message = my_package.greet("Alice")
    print(message)

    # Math operations
    sum = my_package.add(10, 20)
    print(f"10 + 20 = {sum}")

    # Point distance
    p1 = my_package.Point { x: 0.0, y: 0.0 }
    p2 = my_package.Point { x: 3.0, y: 4.0 }
    dist = p1.distance(p2)
    print(f"Distance: {dist}")
```

Run example:

```bash
gul run examples/basic.mn
```

## 📝 Documentation

### Write README.md

````markdown
# My Package

A useful GUL package providing greeting and math utilities.

## Installation

```bash
gul install my-package
```
````

## Usage

```gul
import my_package

main:
message = my_package.greet("World")
print(message) # "Hello, World!"
```

## API

### Functions

- `greet(name: str): str` - Returns a greeting message
- `add(a: int, b: int): int` - Adds two integers

### Structs

- `Point { x: float, y: float }` - 2D point with distance calculation

## License

MIT

````

### Add DocStrings

```gul
Greets a person by name

# Arguments
* `name` - The name to greet

# Returns
A greeting message string

# Example
```gul
message = greet("Alice")
print(message)  # "Hello, Alice!"

export fn greet(name: str): str:
    return f"Hello, {name}!"
````

## 🚀 Publishing

### Step 1: Build and Test

```bash
# Run all tests
gul test

# Check code quality
gul lint
gul fmt --check

# Build package
gul build --release
```

### Step 2: Version Management

Follow semantic versioning (MAJOR.MINOR.PATCH):

```bash
# Patch release (bug fixes)
gul package version patch  # 0.1.0 -> 0.1.1

# Minor release (new features)
gul package version minor  # 0.1.1 -> 0.2.0

# Major release (breaking changes)
gul package version major  # 0.2.0 -> 1.0.0
```

### Step 3: Publish to Registry

```bash
# Login to package registry
gul login

# Publish package
gul publish

# Verify publication
gul package info my-package
```

## 🔄 Maintaining Packages

### Updating Dependencies

```bash
# Update all dependencies
gul update

# Update specific dependency
gul update dependency-name
```

### Deprecation

```gul
/// @deprecated Use new_function() instead
export fn old_function():
    print("This function is deprecated")
```

### Multiple Versions

Maintain compatibility:

```toml
[package]
version = "2.0.0"

[compatibility]
min_gul_version = "0.12.0"
```

## 📚 Best Practices

1. **Semantic Versioning**: Follow SemVer strictly
2. **Changelog**: Keep a CHANGELOG.md file
3. **Tests**: Cover all public APIs
4. **Documentation**: Document all public functions
5. **Examples**: Provide clear usage examples
6. **License**: Include appropriate license
7. **CI/CD**: Automate testing and publishing

## 🎯 Advanced Features

### Conditional Compilation

```gul
#[cfg(target_os = "linux")]
fn linux_specific():
    # Linux-only code
    pass

#[cfg(feature = "advanced")]
export fn advanced_feature():
    # Optional feature
    pass
```

### Feature Flags

In `gul.toml`:

```toml
[features]
default = ["basic"]
basic = []
advanced = ["dep:advanced-lib"]
full = ["basic", "advanced"]
```

---

**Last Updated**: 2025-12-28  
**Version**: 1.0.0  
**License**: MIT
