# GUL v3.2 Quick Reference

**Version**: 0.13.0 | **Syntax**: v3.2 | **Updated**: 2025-12-28

---

## Table of Contents

1. [Type System](#type-system)
2. [Variables](#variables)
3. [Collections](#collections)
4. [Functions](#functions)
5. [Structs](#structs)
6. [Control Flow](#control-flow)
7. [Foreign Code](#foreign-code)
8. [MCP Commands](#mcp-commands)

---

## Type System

### Primitive Types (@ prefix)

```gul
let name = @str("Alice")      # String
let age = @int(30)            # Integer
let score = @float(95.5)      # Float
let active = @bool(true)      # Boolean
```

### Collection Types

```gul
let numbers = @list[1, 2, 3]           # List
let point = @tuple(10, 20)             # Tuple
let tags = @set{"a", "b", "c"}         # Set
let user = @dict{name: "Bob", age: 25} # Dictionary
```

---

## Variables

```gul
let x = @int(10)     # Immutable
var y = @int(20)     # Mutable

y = 30               # OK
# x = 15             # Error: x is immutable
```

---

## Collections

### Lists

```gul
let nums = @list[1, 2, 3, 4, 5]
nums.append(6)
nums.remove(1)
let first = nums[0]
```

### Tuples

```gul
let coords = @tuple(10.5, 20.3, 30.1)
let x = coords.0
let y = coords.1
```

### Sets

```gul
let tags = @set{"rust", "python", "js"}
tags.add("go")
tags.remove("js")
```

### Dictionaries

```gul
let config = @dict{
    host: "localhost",
    port: 8080,
    debug: true
}
let port = config["port"]
```

---

## Functions

### Basic Functions

```gul
fn @int add(a, b):
    return a + b

fn @str greet(name):
    return "Hello, " + name
```

### Arrow Functions

```gul
let double = (x) => x * 2
let add = (a, b) => a + b
```

### Async Functions

```gul
async @dict fetch_data(url):
    let response = await http.get(url)
    return response.json()
```

---

## Structs

```gul
struct Point:
    x: @float
    y: @float

    fn @float distance(self):
        return math.sqrt(self.x^2 + self.y^2)

    fn @str to_string(self):
        return "Point(" + str(self.x) + ", " + str(self.y) + ")"

# Usage
let p = Point{x: 3.0, y: 4.0}
print(p.distance())  # 5.0
```

---

## Control Flow

### If/Else

```gul
if x > 10:
    print("Large")
elif x > 5:
    print("Medium")
else:
    print("Small")
```

### Loops

```gul
# While loop
while count < 10:
    print(count)
    count = count + 1

# For loop
for item in items:
    print(item)

# Range
for i in range(10):
    print(i)
```

### Match

```gul
match value:
    1 => print("One")
    2 => print("Two")
    _ => print("Other")
```

---

## Foreign Code

### Python

```gul
@python {
    import pandas as pd
    import numpy as np

    df = pd.read_csv("data.csv")
    mean = df['age'].mean()
}

let result = python.mean
```

### Rust

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

let fib = rust.fast_fibonacci(20)
```

### SQL

```gul
@sql {
    SELECT * FROM users WHERE age > 18
}
```

---

## MCP Commands

### Code Generation

```bash
# Generate from description
gul-mcp generate "REST API for users" --type application

# Create package
gul-mcp create my-app --type web

# Run code
gul-mcp run main.mn

# Install dependencies
gul-mcp install pandas numpy tensorflow
```

### Auto-Maintenance

```bash
# Individual tasks
gul-mcp auto lint      # cargo clippy
gul-mcp auto fmt       # cargo fmt
gul-mcp auto check     # cargo check
gul-mcp auto audit     # cargo audit

# All at once
gul-mcp auto all
```

### Workflows

```bash
# List workflows
gul-mcp workflow list

# Run workflow
gul-mcp workflow run ci_workflow

# Add custom workflow
gul-mcp workflow add deploy deploy.json
```

### Scheduling

```bash
# List schedules
gul-mcp schedule list

# Enable/disable
gul-mcp schedule enable auto_lint
gul-mcp schedule disable weekly_deps
```

### Server

```bash
# Start MCP server
gul-mcp serve --port 3000

# TUI dashboard
gul-mcp tui

# Web UI
gul-mcp webui --port 8080

# Status
gul-mcp status
gul-mcp tools
```

---

## Import System

```gul
@imp std.io                    # Single module
@imp std{io, math, http}       # Multiple modules
@imp python{pandas, numpy}     # Foreign modules
```

---

## Main Entry Point

```gul
mn:
    print("Hello, World!")

# Or with function
fn main():
    print("Hello!")

mn:
    main()
```

---

## Common Patterns

### Web Server

```gul
@imp std.http

fn @dict handler(req):
    return @dict{status: "ok"}

mn:
    http.listen(8080, handler)
```

### Data Analysis

```gul
@imp python{pandas}

@python {
    df = pd.read_csv("data.csv")
    stats = {
        "mean": float(df.mean().mean()),
        "count": len(df)
    }
}
```

### Async Operations

```gul
async main():
    let data = await fetch("https://api.example.com")
    print(data)

mn:
    await main()
```

---

## Error Handling

```gul
try:
    let result = risky_operation()
catch error:
    print("Error:", error)
finally:
    cleanup()
```

---

## Ownership Modes

```gul
fn process(data: borrow @list):    # Borrow
    # Can read but not modify

fn mutate(data: ref @list):        # Mutable reference
    data.append(10)

fn consume(data: move @list):      # Move ownership
    # Takes ownership

fn keep(data: kept @list):         # Keep copy
    # Makes a copy
```

---

## Quick Start Example

```gul
@imp std.io
@imp python{numpy, pandas}

struct DataProcessor:
    filepath: @str

    fn @dict analyze(self):
        @python {
            df = pd.read_csv(self.filepath)
            return {
                "count": len(df),
                "mean": float(df.mean().mean())
            }
        }
        return python.result

fn main():
    let processor = DataProcessor{
        filepath: "data.csv"
    }
    let results = processor.analyze()
    print("Results:", results)

mn:
    main()
```

---

**More**: See [complete documentation](README.md)  
**MCP Guide**: [MCP_QUICKSTART.md](guides/MCP_QUICKSTART.md)  
**Advanced**: [MCP_ADVANCED.md](../MCP_ADVANCED.md)
