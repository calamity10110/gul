# GUL Language Knowledgebase

**Version**: 0.13.0 | **Syntax**: v3.2 |

## Quick Start

**Keywords**: getting-started, hello-world, first-program

### Hello World

```gul
mn:
    print("Hello, GUL!")
```

### Simple Program

```gul
@imp std.io

@fn greet(name: str) -> str:
    return "Hello, " + name
# @fn @str(greet(name,("Hello, " + name)))
mn:
    let message = greet("World")
    print(message)
```

**Related**: [First Program Guide](../guides/first-program.md)

---

## Language Overview

**Keywords**: introduction, philosophy, design-principles

### What is GUL?

GUL (GUL Universal Language) is a modern, multi-paradigm programming language that combines:

- **Python's readability** - Clean, indentation-based syntax
- **Rust's safety** - Strong ownership model without garbage collection
- **Universal deployment** - Desktop, Web (WASM), and Embedded devices

### Core Design Principles

1. **Simplicity First**: Clear, readable syntax without unnecessary complexity
2. **Safety by Default**: Memory safety through ownership, not garbage collection  
3. **Performance**: Native compilation for maximum speed
4. **Interoperability**: Seamless integration with Python, Rust, JavaScript, SQL
5. **Universality**: One language for all platforms

### Key Features

| Feature | Description |
| --------- | ------------- |
| **Syntax v3.2** | Modern `let`/`var` syntax with `@` type prefixes |
| **Ownership** | Rust-like move semantics with explicit `borrow`/`move`/`kept` |
| **Foreign Code** | Embed Python, Rust, SQL directly in GUL code |
| **Type System** | Gradual typing with inference and `@type` constructors |
| **Async/Await** | Built-in cooperative multitasking |
| **MCP Server** | AI agent integration via Model Context Protocol |

**See Also**: [Specification](specification.md), [Quick Reference](../QUICK_REFERENCE.md)

---

## Syntax v3.2

**Keywords**: syntax, grammar, v3.2, conventions

### File Extension

*.mn  - All GUL files use .mn extension

### Comments

```gul
# Single line comment

#[
Multi-line
comment  
]#
```

### Keywords

**Variables**: `let` (immutable), `var` (mutable)  
**Functions**: `@fn`, `@async`, `return`  
**Entry Point**: `mn:`  
**Control Flow**: `if`, `elif`, `else`, `for`, `while`, `loop`, `break`, `continue`, `match`  
**Error Handling**: `try`, `catch`, `finally`, `throw`  
**Imports**: `@imp`  
**Foreign Code**: `@python`, `@rust`, `@sql`, `@js`  
**Async**: `@async`, `await`  

### Operators

**Arithmetic**: `+`, `-`, `*`, `/`, `%`, `^` (power)  
**Comparison**: `==`, `!=`, `<`, `>`, `<=`, `>=`  
**Logical**: `&&`, `||`, `!`  
**Bitwise**: `&`, `|`, `<<`, `>>`  

**See Also**: [Syntax Reference](syntax.md)

---

## Variables and Types

**Keywords**: variables, types, let, var, immutable, mutable

### Variable Declaration

```gul
# Immutable (default)
let name = "Alice"
let age = 25

# Mutable  
var count = 0
count = count + 1
```

### Type System

#### Primitive Types

```gul
let name = @str("Alice")      # String
let age = @int(30)            # Integer (64-bit signed)
let score = @float(95.5)      # Float (64-bit IEEE), also accepts @flt(24)
let active = @bool(true)      # Boolean
let msg = f"Score: {score}"   # F-String interpolation
```

#### Collections

**Lists** - Position-based, ordered, allows duplicates:

```gul
let numbers = @list[1, 2, 3]        # Literal immutable
var numbers = @list[1, 2, 3]        # Literal mutable
let matrix = @list[[1, 2], [3, 4]]  # Multi-dimensional immutable
var matrix = @list[[1, 2], [3, 4]]  # Multi-dimensional mutable

# Methods
numbers.insertbefore(99) # Insert at begin
numbers.insertbefore(0, 99) # Insert at index, count from begin
numbers.insertafter(0, 99) # Insert at index, count from end 
numbers.insertafter(99) # insert to end
numbers.remove(2)           # Remove by value
numbers.pop()               # Remove from end
numbers.pop(0)              # Remove at index
numbers.clear()             # Empty list
numbers.contain("C", "found")     # Membership verify
numbers.len()               # Length property
numbers[0]  # First element
numbers[-1]  # Last element
```

**Sets** - Unordered, unique elements:

```gul
let tags = @set{"a", "b"}
var tags = @set{"a", "b"}

# Methods
tags.add("c")             # Add element
tag.contain("C", found")  # Membership verify
tags.remove("b")          # Remove element
tags.clear()              # Empty set
tag.len()                 # Length property
```

**Dictionaries** - Ordered key-value pairs, unique keys:

```gul
let config = @dict{host: "localhost", port: 8080}  # Immutable
var cfg = @dict{host: "localhost", port: 8080}  # Mutable

# Methods
config.contain("port", "found")     # Membership verify
config.len()                # Length property

# Access
cfg[key]  # By identifier
cfg["key"]  # By string

# Insertion methods (maintains order)
cfg.insertbefore(position, key: value)  # Insert at position/default begin
cfg.insertbefore(target_key, key: value)  # Insert before key
cfg.insertafter(key: value)  # Insert at end (default)
cfg.insertafter(target_key, key: value)  # Insert after key
cfg.add(key: value)  # Append at end

# Removal
cfg.remove(position)  # By position
cfg.remove(key)  # By key
cfg.remove(key: value)  # By key-value pair

```

#### Type Annotations

```gul
let name: str = "Alice"
var counter: int = 0
let data: list = [1, 2, 3]
```

#### Gradual Typing

```gul
var result: any = 42
result = "now a string"  # OK with 'any' type
```

### Type Quick Reference

| Type | Constructor | Example |
| ------ | ------------- | --------- |
| String (default) | `@str/@string(...)` | `@str("hello")` or `@string("hello")` or `"hello"` |
| Integer | `@int(...)` | `@int(42)` |
| Float | `@flt/@float(...)` | `@flt(3.14)` or `@float(3.14)` |
| Boolean | `@bool(...)` | `@bool(true)` |
| List | `@list[...]` | `@list[1, 2, 3]` |
| Tuple | `(...)` | `(1, 2)` (MVP: treated as immutable List) |
| Set | `@set{...}` | `@set{1, 2, 3}` |
| Dict | `@dict{...}` | `@dict{key: val}` |

**See Also**: [Types Reference](types.md)

---

## Functions

**Keywords**: functions, fn, async, parameters, return

### Basic Functions

```gul
@fn add(a: int, b: int) -> int:
    return a + b
# @fn @int(add((a, b), (a + b)))

@fn greet(name: str) -> str:
    return "Hello, " + name
# @fn @str(greet((name), ("Hello, " + name)))

# Call functions
let sum = add(5, 3)
let message = greet("Alice")
```

### Arrow Functions (Lambdas)

```gul
let double = (x) => x * 2
let add = (a, b) => a + b

# Usage
let result = double(5)  # 10
```

### Async Functions

```gul
@async fetch_data(url: str) -> dict:
    let response = await http.get(url)
    return response.json()

# Call async function
mn:
    let data = await fetch_data("https://api.example.com")
    print(data)
### Function Features

| Feature | Syntax | Example |
| --------- | -------- | --------- |
| Type annotations | `fn name(x: type) -> type:` | `fn add(a: int, b: int) -> int:` |
| Optional types | `x?: type` | `fn greet(name?: str):` |
| Default values | `x = value` | `fn greet(name = "Guest"):` |
| Variadic args | `*args` | `fn sum(*numbers):` |

**See Also**: [Functions Guide](../book/02_functions.md)

---

## Control Flow

**Keywords**: if, else, for, while, loop, match, control-flow

### Conditional Statements

```gul
if age >= 18:
    print("Adult")
elif age >= 13:
    print("Teenager")
else:
    print("Child")
```

### For Loops

```gul
# Iterate over range
for i in 0..10:
    print(i)

# Iterate over collection
for item in items:
    print(item)

# With range function
for i in range(10):
    print(i)
```

### While Loops

```gul
while count < 10:
    print(count)
    count = count + 1
```

### Infinite Loop

```gul
loop:
    if done:
        break
    # Do work
```

### Match Expressions

```gul
match value:
    1 => print("One")
    2 => print("Two")
    val => print("Captured value: " + val)
    _ => print("Other")

# Match with result
let description = match status_code:
    200 => "OK"
    404 => "Not Found"
    500 => "Server Error"
    _ => "Unknown"
```

## Modules and Imports

**Keywords**: imports, modules, @imp, packages

### Import Syntax

```gul
# Single module
@imp std.io

# Multiple modules from same package
@imp std{io, http, math}

# Python packages
@imp python{pandas, numpy, tensorflow}

# Rust crates
@imp rust{tokio, serde}

# Block-style imports
@imp:
    std.io,
    std.http,
    python{pandas}
```

### Module Structure

```directory
project/
├── main.mn              # Entry point
├── utils.mn             # Utilities module  
├── models/
│   ├── user.mn          # User model
│   └── product.mn       # Product model
└── config.mn            # Configuration
```

### Usage

```gul
# In main.mn
@imp utils
@imp models{user, product}

mn:
    let helper = utils.format_date()
    let u = user.create("Alice")
```

**See Also**: [Modules Guide](../book/03_modules.md), [Package Catalog](package-catalog.md)

---

## Foreign Language Integration

**Keywords**: foreign-code, python, rust, sql, javascript, interop

### Python Integration

```gul
@imp python{pandas, numpy}

@python {
    import pandas as pd
    import numpy as np
    
    df = pd.read_csv("data.csv")
    mean_age = df['age'].mean()
    
    result = {
        "count": len(df),
        "mean": float(mean_age)
    }
}

mn:
    print(python.result)
```

### Rust Integration

```gul
@rust {
    fn fast_fibonacci(n: u64) -> u64 {
        if n <= 1 { return n; }
        let mut a = 0;
        let mut b = 1;
        for _ in 2..=n {
            let c = a + b;
            a = b;
            b = c;
        }
        b
    }
}

mn:
    let fib20 = rust.fast_fibonacci(20)
    print("Fibonacci(20):", fib20)
```

### SQL Integration

```gul
@sql {
    SELECT * FROM users 
    WHERE age > 18 
    ORDER BY created_at DESC
}

mn:
    for user in sql.results:
        print(user.name)
```

### JavaScript Integration (WebAssembly)

```gul
@js {
    function calculateTotal(items) {
        return items.reduce((sum, item) => sum + item.price, 0);
    }
}

mn:
    let total = js.calculateTotal(cart_items)
```

**See Also**: [Integration Guide](../docs/api/integration.md)

---

## Async Programming

**Keywords**: @async, await, concurrency, futures, tasks

### Async Function

```gul
@async fetch_user(id: int) -> dict:
    let response = await http.get(f"https://api.com/users/{id}")
    return response.json()

@async main():
    let user = await fetch_user(123)
    print("User:", user.name)

mn:
    await main()
```

### Parallel Execution

```gul
@async fetch_all():
    let [user1, user2, user3] = await parallel([
        fetch_user(1),
        fetch_user(2),
        fetch_user(3)
    ])
    return [user1, user2, user3]

### Async Patterns

| Pattern | Description | Example |
| --|---------|-------------|---------|
| **Sequential** | One after another | `let a = await f1(); let b = await f2()` |
| **Parallel** | All at once | `await parallel([f1(), f2(), f3()])` |
| **Race** | First to complete | `await race([f1(), f2()])` |

**See Also**: [Async Guide](../book/04_advanced.md#async-programming)

---

## Ownership Model

**Keywords**: ownership, borrow, move, kept, memory-management

### Ownership Modes

```gul
# Borrow (read-only reference)
@fn read_data(data: borrow @list):
    print(data[0])  # Can only read
# @fn @read(read_data((data: borrow @list), print(data[0])))
# Mutable reference
@fn modify_data(data: ref @list):
    data.append(10)  # Can modify
# @fn @ref(modify_data((data: ref @list), data.append(10)))

# Move ownership
@fn consume_data(data: move @list):
    # Takes full ownership
    # Original variable becomes invalid
# @fn @move(consume_data((data: move @list), data.append(10)))
# Keep (make a copy)
@fn keep_data(data: kept @list):
    # Creates a copy
    # Original remains valid
# @fn @keep(keep_data((data: kept @list), data.append(10)))
```

### Example

```gul
let original = @list[1, 2, 3]

# Borrow - original still valid
read_data(borrow original)
print(original)  # OK

# Move - original becomes invalid  
consume_data(move original)
# print(original)  # ERROR: value moved

# Kept - makes a copy
let data2 = @list[4, 5, 6]
keep_data(kept data2)
print(data2)  # OK - still valid
```

### Ownership Quick Reference

| Mode | Keyword | Can Read? | Can Modify? | Original Valid After? |
| ---- | ------- | --------- | ----------- | --------------------- |
| **Borrow** | `borrow` | ✅ | ❌ | ✅ |
| **Reference** | `ref` | ✅ | ✅ | ✅ |
| **Move** | `move` | ✅ | ✅ | ❌ |
| **Keep** | `kept` | ✅ | ✅ | ✅ (copy made) |

**See Also**: [Ownership Reference](ownership.md)

---

## Error Handling

**Keywords**: errors, exceptions, try, catch, finally, result

### Try-Catch

```gul
try:
    let result = risky_operation()
    print("Success:", result)
catch error:
    print("Error:", error)
finally:
    cleanup()  # Always runs
```

### Result Type (Rust-style)

```gul
@fn divide(a: int, b: int) -> Result<int, str>:
    if b == 0:
        return Err("Division by zero")
    return Ok(a / b)

mn:
    match divide(10, 2):
        Ok(value) => print("Result:", value)
        Err(msg) => print("Error:", msg)
```

### Error Propagation

```gul
@async fetch_and_process() -> Result<dict, str>:
    let data = await fetch_data()?  # ? propagates errors
    let processed = process(data)?
    return Ok(processed)
```

---

## Standard Library

**Keywords**: stdlib, standard-library, modules, packages

### Core Modules

| Module | Description | Key Functions |
|--------|-------------|---------------|
| `std.io` | Input/Output | `print()`, `read_file()`, `write_file()` |
| `std.http` | HTTP client/server | `get()`, `post()`, `listen()` |
| `std.math` | Mathematics | `sqrt()`, `sin()`, `cos()`, `pow()` |
| `std.json` | JSON handling | `parse()`, `stringify()` |
| `std.db` | Database | `query()`, `execute()`, `connect()` |
| `std.collections` | Data structures | `Map`, `Set`, `Queue`, `Stack` |
| `std.crypto` | Cryptography | `hash()`, `encrypt()`, `decrypt()` |
| `std.time` | Date/Time | `now()`, `sleep()`, `format()` |

### Example Usage

```gul
@imp std{io, http, json, math}

# File I/O
let content = io.read_file("data.txt")
io.write_file("output.txt", "Hello")

# HTTP
let response = await http.get("https://api.com/data")
let data = json.parse(response.body)

# Math
let result = math.sqrt(16)  # 4.0
let angle = math.sin(math.PI / 2)  # 1.0
```

## Toolchain

**Keywords**: compiler, tools, cli, gul-command

### CLI Commands

```bash
# Create new project
gul new my-project

# Build project
gul build
gul build --release  # Optimized build

# Run code
gul run main.mn
gul run --watch main.mn  # Auto-rebuild on changes

# Test
gul test

# Format code
gul fmt

# Lint code
gul lint

# Package management
gul install pandas numpy  # Install dependencies
gul publish  # Publish package

# Documentation
gul doc  # Generate docs

# IDE/REPL
gul ide  # Launch TUI IDE
gul repl  # Interactive REPL
```

### MCP Server Commands

```bash
# Start MCP server
gul-mcp serve --port 3000

# AI code generation
gul-mcp generate "create REST API"

# Auto-maintenance
gul-mcp auto fmt  # Format code
gul-mcp auto lint  # Run linter
gul-mcp auto all  # All checks

# Dashboard
gul-mcp tui     # Terminal UI
gul-mcp webui   # Web interface
```

## FAQ and Troubleshooting

**Keywords**: faq, troubleshooting, common-issues, help

### Q: How do I choose between `let` and `var`?

**A**: Use `let` by default for immutable variables. Only use `var` when you need to modify the value.

```gul
let API_KEY = "abc123"  # Never changes - use let
var counter = 0         # Changes - use var
counter = counter + 1
```

### Q: When should I use `borrow` vs `move` vs `kept`?

**A**:

- **`borrow`**: When you only need to read the data (most common)
- **`ref`**: When you need to modify the data  
- **`move`**: When you're transferring ownership (less common)
- **`kept`**: When you need a copy (careful - performance cost)

```gul
read_data(borrow list)    # Just reading
modify_data(ref list)     # Need to change it
consume_data(move list)   # Transferring ownership
backup_data(kept list)    # Need a copy
```

### Q: How do I import Python packages?

**A**: Use `@imp python{package1, package2}` syntax:

```gul
@imp python{pandas, numpy, matplotlib}

@python {
    import pandas as pd
    df = pd.read_csv("data.csv")
}
```

### Q: Error: "value moved here"

**Context**: Trying to use a variable after it was moved

```gul
let data = @list[1, 2, 3]
process(move data)  # Ownership moved
print(data)  # ❌ Error: value moved
```

**Solution**: Use `borrow` instead:

```gul
let data = @list[1, 2, 3]
process(borrow data)  # ✅ Borrow instead
print(data)  # ✅ Still valid
```

### Q: How do I run async code in `mn:` block?

**A**: Use `await` directly or wrap in async function:

```gul
# Option 1: Direct await
mn:
    let data = await fetch_data()
    print(data)

# Option 2: Async main
@async main():
    let data = await fetch_data()
    print(data)

mn:
    await main()
```

### Common Errors

| Error | Cause | Solution |
|-------|-------|----------|
| "value moved here" | Using variable after `move` | Use `borrow` or `kept` |
| "cannot modify immutable variable" | Trying to change `let` variable | Use `var` instead |
| "type mismatch" | Wrong type passed | Check function signature |
| "module not found" | Missing import | Add `@imp` statement |

**See Also**: [Guides](../docs/guides/), [Book](../docs/book/)

---

## Appendix A: Syntax Comparison

### v2.0 → v3.2 Migration

| Feature | v2.0 (Deprecated) | v3.2 (Current) |
|---------|-------------------|----------------|
| Variables | `def x = 10` | `let x = 10` or `var x = 10` |
| Functions | `fn:` block | `fn name():` |
| Imports | `imp std.io` | `@imp std.io` |
| Main | `main:` | `mn:` |
| Async | `asy fetch()` | `async fetch()` |
| Foreign | `cs python:` | `@python { }` |

---

## Appendix B: Keywords Reference

### Current (v3.2)

```
let, var, @fn, @async, await, mn, @imp
if, elif, else, for, while, loop, break, continue, return
try, catch, finally, throw
match, struct, enum
borrow, ref, move, kept
@python, @rust, @sql, @js
true, false, null
```

### Deprecated (v2.0) - Still supported

```
def, imp, main, asy, cs
own, copy (use borrow/move/kept instead)
```

---

## Appendix C: Quick Reference Card

```gul
# Variables
let x = 10      # Immutable
var y = 20      # Mutable

# Types
@int(42)        @str("hello")    @bool(true)
@list[1,2,3]    @dict{k: v}      @tuple(1,2)

# Functions
@fn add(a, b):
    return a + b

@async fetch(url):
    return await http.get(url)

# Control Flow
if cond:
    ...
elif other:
    ...
else:
    ...

for i in range(10):
    print(i)

match value:
    1 => "one"
    _ => "other"

# Imports
@imp std.io
@imp python{pandas}

# Foreign Code
@python {
    import numpy as np
    result = np.mean([1,2,3])
}

# Main Entry
mn:
    print("Hello!")
```

---

## Testing & Verification

### Test Files

GUL includes comprehensive test suites:

| Location | Description |
|----------|-------------|
| `compilers/shared/tests/` | Cross-compiler tests |
| `compilers/nightly/tests/` | Nightly feature tests |

### Running Tests

```bash
# Nightly test suite
cd compilers/nightly
python3 tests/run_tests.py

# Single test
./target/release/gulc test.gul -o test && ./test
```

### CI/CD

See `.github/workflows/ci.yml` for automated testing.

**Full Documentation**: [testfiles.md](../testfiles.md)
