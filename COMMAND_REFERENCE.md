# GUL Command Reference v3.2

**Version**: 0.14.0-dev | **Syntax**: v3.2 | **Updated**: 2026-01-08

**Complete command reference for GUL Universal Language**

---

## Table of Contents

1. [CLI Commands](#cli-commands)
2. [Language Features](#language-features)
3. [Type System](#type-system)
4. [Collection Types](#collection-types)
5. [Built-in Functions](#built-in-functions)
6. [Standard Library](#standard-library)
7. [Advanced Features](#advanced-features)

---

## CLI Commands

### Compiler Commands

```bash
# Compile a GUL program
gul-compile <file.mn>                    # Compile nightly
gul-compile-stable <file.mn>             # Compile stable

# Run directly
gul run <file.mn>                        # Compile and run
gul run --release <file.mn>              # Optimized build

# Build project
gul build                                # Development build  
gul build --release                      # Production build

# Check code
gul check                                # Type check only
gul fmt                                  # Format code
gul lint                                 # Lint code
```

### Package Management

```bash
# List packages (180+ available)
gul package list
gul package list --language rust
gul package list --category web

# Search and info
gul package search <query>
gul package info <package-name>

# Install/update/remove
gul package install <package-name>
gul package update <package-name>
gul package remove <package-name>

# Maintenance
gul package audit                        # Security audit
gul package outdated                     # Check for updates
```

### Development Tools

```bash
# IDEs
gul ide                                  # TUI IDE
gul ide --mode web                       # Web IDE
gul repl                                 # Interactive REPL

# MCP Server (AI Integration)
gul-mcp serve --port 3000
gul-mcp tui
gul-mcp webui --port 8080

# Code generation
gul-mcp generate "REST API"
gul-mcp create my-app --type web

# Auto-maintenance
gul-mcp auto fmt                         # Auto-format
gul-mcp auto lint                        # Auto-lint
gul-mcp auto check                       # Auto type-check
gul-mcp auto all                         # All checks
```

### Testing

```bash
# Run tests
gul test                                 # All tests
gul test <file.mn>                       # Specific file
cargo test --lib                         # Rust unit tests

# Benchmarks
cargo bench                              # Run benchmarks
cargo bench --bench <name>               # Specific benchmark
```

### Documentation

```bash
# Generate documentation
gul doc                                  # Generate docs
cargo doc --open                         # Rust API docs
```

### Runtime Operations

```bash
# Execute foreign code
gul runtime python "print('Hello')"
gul runtime js "console.log('Hello')"

# Load Rust libraries
gul runtime load-lib path/to/lib.so
```

---

## Language Features

### Variables

```gul
# Immutable (default)
const name = "Alice"
const age = 25
const pi = 3.14159
name2 = "Bob" (default)

# Mutable
var count = 0
count = count + 1

# Type annotations
const name: str = "Bob"
const age: int = 30
const price: float = 99.99

# Auto literal conversion (NEW in v3.2)
const s: str = 123                  # Int -> Str auto-conversion
const s2: str = 3.14                # Float -> Str auto-conversion
const b: bool = "true"              # Str -> Bool auto-conversion
```

### Functions

```gul
# Regular function
@fn add(a: int, b: int) -> int:
    return a + b

# Arrow function
const double = (x) => x * 2

# Async shortcut (no 'fn' keyword)
async fetch_data(url: str)(res):
    res = await http.get(url)

# Decorators
@flow fn compute(x):              # Flow variable
    return x * 2
@ui button{text: "Click", color: "blue"} # UI with properties
```

### Control Flow

```gul
# If-else
if condition:
    # code
elif other_condition:
    # code
else:
    # code

# For loops
for i in 0..10:
    print(i)

# Parallel for (v3.2)
also_for i in 0..1000:
    process(i)

# Parallel while (v3.2)
also_while condition:
    process_next()

# While loops
while condition:
    # code

# Match expressions
match value:
    1 => print("One")
    2 => print("Two")
    _ => print("Other")
```

### Structs

```gul
struct User:
    name: str
    email: str
    age: int
    
    @fn greet(self) -> str:
        return "Hello, " + self.name

# Create instance
const user = User{name: "Alice", email: "alice@example.com", age: 25}
print(user.greet())
```

### Imports

```gul
# Single module
@imp std.io

# Multiple modules
@imp std{io, http, json}

# Python packages
@imp python{pandas, numpy}

# Block style
@imp:
    std.io,
    std.http,
    python{pandas}
```

---

## Type System

### Primitive Types

```gul
# Integer
const count: int = 42
constx = @int(input())              # Type constructor

# Float
const price: float = 99.99
const y = @flt(input())              # Type constructor

# String
const name: str = "Alice"
const s = @str(123)                  # Type conversion

# Boolean
const flag: bool = true
const b = @bool("true")              # Type conversion
```

### Type Constructors

```gul
@int()          # Integer constructor
@flt()          # Float constructor
@str()          # String constructor  
@bool()         # Boolean constructor
@list[]         # List constructor
@dict{}         # Dictionary constructor
@set{}          # Set constructor
@tabl{}         # Table constructor
@frame{}        # DataFrame constructor (NEW)
@tensor[]       # Tensor constructor (NEW)
@chan<T>        # Channel constructor (NEW)
```

### Type Annotations

```gul
# Variable annotations
const x: int = 10
const name: str = "Alice"

# Function annotations
@fn process(data: list, count: int) -> dict:
    return @dict{result: data}

# Optional types  
const value: int? = None
```

---

## Collection Types

### List

```gul
# Create list
const numbers = [1, 2, 3, 4, 5]
const empty = @list[]

# Access elements
const first = numbers[0]
const last = numbers[-1]

# Operations
numbers.push(6)
numbers.insert(0, 0)
numbers.remove(3)
const length = len(numbers)

# Iteration
for num in numbers:
    print(num)
```

### Dictionary

```gul
# Create dictionary
const user = {"name": "Alice", "age": 25}
const empty = @dict{}

# Access elements
const name = user["name"]
const age = user.get("age", 0)

# Operations
user["email"] = "alice@example.com"
user.remove("age")
const keys = user.keys()
const values = user.values()
const length = len(user)

# Iteration
for key, val in user:
    print(key, val)
```

### Set

```gul
# Create set
const unique = {1, 2, 3, 4, 5}
const empty = @set{}

# Operations
unique.add(6)
unique.remove(1)
const has_two = 2 in unique

# Set operations
const a = {1, 2, 3}
const b = {3, 4, 5}
const union = a | b
const intersection = a & b
const difference = a - b
```

### Tuple

```gul
# Create tuple
const point = (10, 20)
const person = ("Alice", 25, "alice@example.com")

# Access elements
const x = point[0]
const y = point[1]

# Unpacking
const (name, age, email) = person
const length = len(person)
```

### Table (NEW)

```gul
# Create table
const data = @tabl{
    columns: ("id", "name", "age"),
    rows: [
        {name: "row1", values: [1, "Alice", 25]},
        {name: "row2", values: [2, "Bob", 30]}
    ]
}

# Access
const cell = data[0][1]              # "Alice"
```

### DataFrame (NEW in v3.2)

```gul
# Create DataFrame
const df = @frame{
    columns: ("name", "age", "city"),
    data: [
        ["Alice", 25, "NYC"],
        ["Bob", 30, "LA"],
        ["Charlie", 35, "SF"]
    ]
}

# Column access
const names = df.name
const ages = df["age"]

# Operations
const filtered = df.filter(row => row.age > 25)
const grouped = df.group_by("city")
const length = len(df)
```

---

## Built-in Functions

### I/O Functions

```gul
print(value)                       # Print to stdout
print(v1, v2, v3)                  # Print multiple values

input()                            # Read string from stdin
@int(input())                      # Read integer
@flt(input())                      # Read float
```

### String Functions

```gul
str.len(s)                         # String length
str.upper(s)                       # Uppercase
str.lower(s)                       # Lowercase
str.split(s, delimiter)            # Split string
str.join(list, separator)          # Join strings
str.substr(s, start, length)       # Substring
str.replace(s, old, new)           # Replace
```

### Math Functions

```gul
math.abs(x)                        # Absolute value
math.sqrt(x)                       # Square root
math.pow(x, y)                     # Power
math.floor(x)                      # Floor
math.ceil(x)                       # Ceiling
math.round(x)                      # Round
math.sin(x)                        # Sine
math.cos(x)                        # Cosine
math.tan(x)                        # Tangent
math.log(x)                        # Natural log
math.exp(x)                        # Exponent
```

### Collection Functions

```gul
len(collection)                    # Length
map(fn, collection)                # Map  
filter(fn, collection)             # Filter
reduce(fn, collection, init)       # Reduce
sum(collection)                    # Sum
min(collection)                    # Minimum
max(collection)                    # Maximum
sorted(collection)                 # Sort
reversed(collection)               # Reverse
```

---

## Standard Library

### std.io - File I/O

```gul
@imp std.io

# Read/write files
const content = io.read_file("data.txt")
io.write_file("output.txt", content)

# File operations
io.exists("file.txt")
io.delete("file.txt")
io.copy("src.txt", "dst.txt")
io.move("old.txt", "new.txt")

# Directory operations
io.mkdir("dirname")
io.rmdir("dirname")
io.listdir("path")
```

### std.http - HTTP Client/Server

```gul
@imp std.http

# HTTP client
const response = await http.get("https://api.example.com")
const data = response.json()

const response = await http.post("https://api.example.com", {
    json: {key: "value"}
})

# HTTP server
@async handle_request(req):
    return {status: 200, body: "Hello"}

http.listen(8080, handle_request)
```

### std.json - JSON Processing

```gul
@imp std.json

# Parse JSON
const obj = json.parse('{"key": "value"}')

# Stringify
const string = json.stringify({key: "value"})
```

### std.db - Database

```gul
@imp std.db

# Connect to database
const conn = await db.connect("postgresql://localhost/mydb")

# Execute queries
const results = await db.query("SELECT * FROM users")

# Transactions
await db.transaction():
    await db.execute("INSERT INTO users ...")
    await db.execute("UPDATE accounts ...")
```

### std.math - Mathematics

```gul
@imp std.math

# Constants
math.PI
math.E

# Functions
math.sqrt(16)
math.pow(2, 8)
math.sin(math.PI / 2)
```

### std.collections - Data Structures

```gul
@imp std.collections

# Stack
const stack = collections.Stack()
stack.push(1)
const top = stack.pop()

# Queue
const queue = collections.Queue()
queue.enqueue(1)
const first = queue.dequeue()

# Heap
const heap = collections.Heap()
heap.push(5)
const min = heap.pop()
```

### std.crypto - Cryptography

```gul
@imp std.crypto

# Hashing
const hash = crypto.sha256("password")
const hash = crypto.md5("data")

# Encryption
const encrypted = crypto.encrypt(data, key)
const decrypted = crypto.decrypt(encrypted, key)
```

### std.time - Date/Time

```gul
@imp std.time

# Current time
const now = time.now()

# Formatting
const formatted = time.format(now, "%Y-%m-%d")

# Parsing
const parsed = time.parse("2024-01-01", "%Y-%m-%d")

# Operations
time.sleep(1000)                   # Sleep 1 second
```

---

## Advanced Features

### Pipeline Operator

```gul
# Chain function calls
const result = data 
    |> filter(x => x > 0)
    |> map(x => x * 2)
    |> sum()

# Equivalent to:
const result = sum(map(filter(data, x => x > 0), x => x * 2))
```

### Flow Variables (NEW in v3.2)

```gul
# Reactive flow variable
@flow var x = 10
@flow var y = x * 2                # Auto-updates when x changes

x = 20                             # y automatically becomes 40
```

### Channels (NEW in v3.2)

```gul
# Create channel
const ch = @chan<int>(10)            # Buffered channel, capacity 10

# Send/receive
ch.send(42)
const value = ch.recv()

# Use in goroutines
async producer(ch):
    for i in 0..100:
        await ch.send(i)

async consumer(ch):
    while true:
        const val = await ch.recv()
        print(val)
```

### Parallel Processing (NEW in v3.2)

```gul
# Parallel for (also_for)
also_for i in 0..1000:
    process_item(i)

# Parallel map
const results = parallel_map(data, x => heavy_computation(x))

# Parallel while (also_while)
also_while condition:
    process_item(next())
```

### Auto-Differentiation

```gul
# Gradient tracking
autograd_begin()
const x = make_var(2.0)
const y = var_add(var_mul(x, x), var_add(var_mul(make_var(3.0), x), make_var(1.0)))
backward(y)
const grad_x = var_grad(x)          # dy/dx = 2x + 3 = 7
autograd_end()
```

### Tensor Operations (NEW)

```gul
# Create tensor
const t = @tensor<float>[3, 3]

# Operations
const a = tensor.zeros([2, 2])
const b = tensor.ones([2, 2])
const c = a + b
const d = tensor.matmul(a, b)

# Shapes
const shape = t.shape()
const reshaped = t.reshape([9])
```

### Foreign Code Integration

```gul
# Python
@python {
    import pandas as pd
    df = pd.read_csv("data.csv")
    result = df['age'].mean()
}

# Rust
@rust {
    fn fast_compute(n: i64) -> i64 {
        (0..n).sum()
    }
}

# JavaScript
@js {
    const result = Array.from({length: 100}, (_, i) => i * i);
}

# SQL
@sql {
    SELECT * FROM users WHERE age > 18
}
```

### Pattern Matching

```gul
match value:
    1 => print("One")
    2 | 3 => print("Two or three")
    x if x > 10 => print("Greater than 10")
    [head, ...tail] => print("List pattern")
    {name, age} => print("Dict pattern")
    _ => print("Default")
```

### Ownership System

```gul
# Modes
const x = 10                         # owned (default)
const y = borrow x                   # borrowed reference
const z = ref x                      # mutable reference
const w = move x                     # transfer ownership
const k = kept x                     # keep original

# In functions
fn process(data: borrow list):     # Borrow parameter
    # Can read but not modify
    print(data[0])

fn modify(data: ref list):         # Mutable reference
    data.append(10)
```

---

## Type Conversion Reference

### Automatic Conversions

```gul
# Int/Float -> String (automatic)
const s1: str = 42                   # "42"
const s2: str = 3.14                 # "3.14"

# String -> Bool (automatic for "true"/"false")
const b1: bool = "true"              # true
const b2: bool = "false"             # false
```

### Explicit Conversions

```gul
# To Integer
@int("42")                         # 42
@int(3.14)                         # 3
@int(true)                         # 1

# To Float
@flt("3.14")                       # 3.14
@flt(42)                           # 42.0

# To String
@str(42)                           # "42"
@str(3.14)                         # "3.14"
@str(true)                         # "true"

# To Boolean
@bool(1)                           # true
@bool(0)                           # false
@bool("true")                      # true
```

---

## Compilation Examples

```bash
# Compile and run simple program
gul-compile tests/hello.mn && ./tests/hello

# Compile with nightly features
cargo run --release --bin gul-compile --manifest-path compilers/nightly/Cargo.toml -- myprogram.mn

# Compile with stable
cargo run --release --bin gul-compile --manifest-path compilers/stable/Cargo.toml -- myprogram.mn

# Run tests
cargo run --release --bin gul-compile --manifest-path compilers/nightly/Cargo.toml -- tests/collections.mn && ./tests/collections
```

---

## Quick Reference

### Hello World

```gul
mn:
    print("Hello, GUL v3.2!")
```

### Complete Example

```gul
@imp std{io, http, json}

struct User:
    name: str
    age: int
    
    fn greet(self) -> str:
        return f"Hello, {self.name}!"

fn process_data(items: list) -> dict:
    const filtered = items |> filter(x => x > 0) |> map(x => x * 2)
    return @dict{result: filtered, count: len(filtered)}

async fetch_users() -> list:
    const response = await http.get("https://api.example.com/users")
    return response.json()

mn:
    const user = User{name: "Alice", age: 25}
    print(user.greet())
    
    const data = [1, -2, 3, -4, 5]
    const result = process_data(data)
    print(result)
    
    const users = await fetch_users()
    for u in users:
        print(u["name"])
```

---

**For more information**: [Full Documentation](docs/) | [Examples](examples/) | [GitHub](https://github.com/calamity10110/gul)
