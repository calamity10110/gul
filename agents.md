# Agent Guidelines for GUL Language Repository

**Version**: 0.13.0 | **Syntax**: v3.2 | **Updated**: 2025-12-28

## Build/Test Commands

### Building

```bash
cargo build --verbose                    # Debug build
cargo build --release                    # Release build
cargo check --all-features              # Quick check
```

### Testing

```bash
cargo test --all-features --verbose     # All tests
cargo test <test_name>                  # Single test
cargo test --lib                        # Library tests only
cargo test --doc --verbose              # Doc tests
```

### Code Quality

```bash
cargo clippy --all-targets --all-features -- -D warnings  # Lint
cargo fmt                                                 # Format code
cargo fmt -- --check                                      # Check formatting
cargo audit                                              # Security audit
```

### MCP Server

```bash
gul-mcp serve --port 3000               # Start MCP server
gul-mcp auto all                        # Run all maintenance
gul-mcp workflow run ci_workflow        # Execute workflow
gul-mcp schedule list                   # View schedules
```

---

## GUL Language Syntax (v3.2)

### Core Syntax

#### Type System (@ prefix)

```gul
# Primitives
let name = @str("Alice")
let age = @int(30)
let score = @float(95.5)
let active = @bool(true)

# Collections
let numbers = @list[1, 2, 3]
let point = @tuple(10, 20)
let tags = @set{"a", "b", "c"}
let config = @dict{key: "value"}
```

#### Variables

```gul
let x = @int(10)     # Immutable
var y = @int(20)     # Mutable
```

#### Functions

```gul
# With return type
fn @int add(a, b):
    return a + b

fn @str greet(name):
    return "Hello, " + name

# Async
async @dict fetch_data(url):
    let response = await http.get(url)
    return response.json()
```

#### Imports

```gul
@imp std.io                      # Single module
@imp std{io, math, http}         # Multiple modules
@imp python{numpy, pandas}       # Foreign modules
```

#### Foreign Code Blocks

```gul
@python {
    import pandas as pd
    df = pd.read_csv("data.csv")
}

@rust {
    fn fast_calc(x: i64) -> i64 {
        x * x
    }
}

@sql {
    SELECT * FROM users WHERE age > 18
}

@c {
    #include <stdio.h>
    void hello() {
        printf("Hello from C\n");
    }
}
```

#### Structs

```gul
struct User:
    name: @str
    age: @int
    email: @str

    fn @str display(self):
        return self.name + " (" + str(self.age) + ")"
```

#### Entry Point

```gul
mn:
    print("Hello, World!")
```

#### Ownership Modes

```gul
fn process(data: borrow @list):   # Borrow (read-only)
fn mutate(data: ref @list):       # Mutable reference
fn consume(data: move @list):     # Take ownership
fn keep(data: kept @list):        # Keep copy
```

---

## File Types

| Extension          | Purpose      | Example                               |
| ------------------ | ------------ | ------------------------------------- |
| `.mn`              | Main files   | Entry points, orchestration           |
| `.def`             | Definitions  | Types, constants, imports             |
| `.fnc`             | Functions    | Pure logic functions                  |
| `.scrt`            | Secrets      | Credentials, API keys (never commit!) |
| `.py`, `.rs`, etc. | Foreign code | Language-specific files               |

---

## Project Structure

```
gul/
├── src/
│   ├── lib.rs              # Library root
│   ├── main.rs             # CLI entry
│   ├── ast.rs              # AST definitions
│   ├── lexer.rs            # Tokenization
│   ├── parser.rs           # Parsing
│   ├── interpreter.rs      # Execution
│   ├── ai/                 # AI module
│   ├── mcp/                # MCP server
│   │   ├── server.rs
│   │   ├── scheduler.rs
│   │   ├── workflow.rs
│   │   ├── auto_maintenance.rs
│   │   ├── cli.rs
│   │   ├── tui.rs
│   │   └── webui.rs
│   └── ...
├── examples/               # Example programs
├── tests/                  # Integration tests
├── docs/                   # Documentation
└── templates/              # Project templates
```

---

## Common Patterns

### Web Server

```gul
@imp std.http

fn @dict handler(req):
    return @dict{status: "ok", message: "Hello!"}

mn:
    http.listen(8080, handler)
```

### Data Analysis

```gul
@imp python{pandas, numpy}

@python {
    df = pd.read_csv("data.csv")
    stats = df.describe().to_dict()
}

mn:
    print(python.stats)
```

### Machine Learning

```gul
@imp python{tensorflow}

struct MLModel:
    input_shape: @int
    num_classes: @int

    fn train(self, data, labels):
        @python {
            model = tf.keras.Sequential([
                tf.keras.layers.Dense(128, activation='relu'),
                tf.keras.layers.Dense(self.num_classes, activation='softmax')
            ])
            model.fit(data, labels, epochs=10)
        }
```

---

## MCP Integration

### Code Generation

```bash
# Generate from description
gul-mcp generate "create a REST API for users" --type application

# AI will:
# 1. Analyze requirements
# 2. Generate v3.2 code with @ prefix
# 3. Create tests
# 4. Run auto-format
# 5. Verify compilation
```

### Automation

```bash
# Auto-maintenance
gul-mcp auto lint    # cargo clippy
gul-mcp auto fmt     # cargo fmt
gul-mcp auto check   # cargo check
gul-mcp auto audit   # cargo audit
gul-mcp auto all     # All of the above

# Workflows
gul-mcp workflow list
gul-mcp workflow run ci_workflow

# Scheduling
gul-mcp schedule enable auto_lint    # Run on commit
gul-mcp schedule enable auto_test    # Run on push
```

---

## Testing Guidelines

### Write Tests

```rust
#[test]
fn test_feature() {
    // Test implementation
    assert_eq!(expected, actual);
}
```

### Run Tests

```bash
cargo test                    # All tests
cargo test test_name         # Specific test
cargo test --lib             # Library tests
cargo test -- --nocapture    # Show output
```

---

## Documentation

- **Main Docs**: `/docs/README.md`
- **Quick Ref**: `/docs/QUICK_REFERENCE.md`
- **API Docs**: `/docs/api/`
- **Guides**: `/docs/guides/`
- **Reference**: `/docs/reference/`

---

## Git Workflow

```bash
# Before committing
cargo fmt                          # Format
cargo clippy --fix                 # Fix lints
cargo test                         # Test
gul-mcp auto all                   # Auto-maintenance

# Commit
git add -A
git commit -m "feat: description"
git push origin master
```

---

## Key Points for AI Agents

1. **Always use v3.2 syntax** - @ prefix for all types
2. **Run tests** before committing
3. **Format code** with `cargo fmt`
4. **Check lints** with `cargo clippy`
5. **Use MCP** for automation
6. **Update docs** when adding features
7. **Never commit** `.scrt` files

---

## Quick Reference

```gul
# Variables
let x = @int(10)        # Immutable
var y = @str("hi")      # Mutable

# Collections
@list[1, 2, 3]
@tuple(x, y)
@set{1, 2, 3}
@dict{key: "value"}

# Functions
fn @int add(a, b): a + b
async @dict fetch(url): await http.get(url)

# Structs
struct Point:
    x: @float
    y: @float

# Imports
@imp std.io
@imp python{pandas}

# Foreign code
@python { import pandas as pd }
@rust { fn calc() {} }

# Entry
mn: print("Hello!")
```

---

**Last Updated**: 2025-12-28  
**For**: AI agents working with GUL  
**Version**: v3.2
