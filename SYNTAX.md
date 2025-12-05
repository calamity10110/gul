# GUL Language - Complete Syntax Guide (v0.11.0)

**GUL** (Glob Universal Language) - A modern, multi-paradigm programming language

---

## Table of Contents

1. [Basic Concepts](#basic-concepts)
2. [Variables and Mutability](#variables-and-mutability)
3. [Flexible Import System](#flexible-import-system)
4. [Functions](#functions)
5. [Annotations](#annotations)
6. [Control Flow](#control-flow)
7. [Ownership Model](#ownership-model)
8. [UI Components](#ui-components)
9. [Multi-Language Integration](#multi-language-integration)
10. [Scientific Computing](#scientific-computing)
11. [Best Practices](#best-practices)

---

## Basic Concepts

### What is GUL?

GUL combines simplicity with power:

- **Python-like** syntax (easy to read)
- **Rust-like** safety (memory safe)
- **JavaScript-like** async (non-blocking I/O)
- **Scientific notation** (physics, chemistry, math)
- **Multi-language** (embed Python, Rust, JS, SQL)

### Your First Program

```glob
# hello.mn - Your first GUL program

mn main():
    print("Hello, GUL!")
```

Run it: `glob run hello.mn`

---

## Variables and Mutability

### Immutable Variables (Default)

```glob
# Immutable - cannot be changed after creation
name = "Alice"        # Immutable by default
age = 25              # Immutable by default
const PI = 3.14159    # Explicit immutable (recommended)
def MAX = 100         # Legacy syntax (still supported)
```

### Mutable Variables

GUL supports **two syntaxes** for mutable variables:

```glob
# Option 1: mut keyword (recommended for new code)
mut count = 0
mut total = 100

# Option 2: ? prefix (legacy, still supported)
?counter = 0
?sum = 100

# Both work the same way:
mut count = count + 1   # OK: count is now 1
?counter = ?counter + 1 # OK: counter is now 1
```

**Recommendation:** Use `mut` for new code as it's clearer and more familiar to developers from other languages.

### When to Use Mutable vs Immutable

```glob
# Use immutable for constants
const MAX_USERS = 1000
const API_URL = "https://api.example.com"

# Use mutable for counters, accumulators
mut counter = 0
mut sum = 0

fn count_items(items):
    mut count = 0
    for item in items:
        count = count + 1
    return count
```

### Global vs Static Variables

```glob
# Global variables (managed by async functions)
@global ?app_state = {
    users: [],
    sessions: {}
}
@global @?list(app_state) = {
    users: [],
    sessions: {}
}
# Static variables (managed by all functions)
@static cache = {}
@static cache = {@list(state, state2)}
@static config = load_config()
@static config = @list(config, config2)

asy update_state(user):
    # Async functions can modify global state
    @global ?app_state.users.append(user)

fn get_cached(key):
    # All functions can access static variables
    return @static cache.get(key)
```

---

## Import System

GUL supports flexible import syntax. Use `import` keyword (recommended) or `imp` (legacy).

### Individual Imports

```glob
# Recommended: import keyword
import std.io
import std.http
import python.numpy
import python.matplotlib
import rust.tokio

# Legacy: imp keyword (still supported)
imp std.io
imp python.numpy
```

### Grouped Imports

```glob
# Group related imports with braces
import {
    std.io,
    std.http,
    python.numpy,
    python.pandas
}

# Shorthand for language packages
import python{numpy, pandas, matplotlib}
import rust{tokio, serde}
```

### Mixed Syntax

```glob
# Combine individual and grouped
import std.io
import std.http
import {
    python{numpy, pandas},
    rust{tokio, serde}
}
```

### Import Examples

```glob
# Data science project
import {
    std.io,
    std.math,
    python{numpy, pandas, matplotlib, scipy}
}

# Web server project
import {
    std{http, io, json},
    rust{tokio, serde},
    js{express, axios}
}

# IoT project
import {
    embedded{gpio, i2c, spi},
    std{time, io},
    rust.tokio
}
```

**Recommendation:** Use `import` keyword for new code. The `imp` keyword is maintained for backward compatibility.

---

## Functions

### Synchronous Functions

```glob
# Basic function
fn greet(name):
    return "Hello, " + name

# Function with type annotations
@fn calculate(@int x, @int y) -> @int:
    return x + y

# Function with mutable parameter
fn increment(@mut ?value):
    ?value = ?value + 1
    return ?value
```

### Async Functions

```glob
# Async function (recommended syntax)
async fetch_data(url):
    response = await http.get(url)
    return response.json()

# Legacy syntax (still supported)
@asy fetch_data_legacy(url):
    response = await http.get(url)
    return response.json()

# Async with error handling
async safe_fetch(url):
    try:
        data = await fetch_data(url)
        return data
    catch error:
        print("Error:", error)
        return null
```

**Recommendation:** Use `async` keyword for new code. The `@asy` syntax is maintained for backward compatibility.

---

## Annotations

GUL supports **two syntaxes** for type annotations:

### Type Annotations

```glob
# Option 1: Colon syntax (recommended)
age: int = 25
name: str = "Alice"
price: float = 19.99
is_active: bool = true

# Option 2: @ prefix (legacy, still supported)
@int age = 25
@str name = "Alice"
@float price = 19.99
@bool is_active = true

# Collection types
numbers: list = [1, 2, 3, 4, 5]
person: map = {name: "Bob", age: 30}
unique_ids: set = {1, 2, 3}

# Or with @ prefix:
@list numbers = [1, 2, 3, 4, 5]
@map person = {name: "Bob", age: 30}
@set unique_ids = {1, 2, 3}
```

**Recommendation:** Use `:` syntax for new code as it's more familiar. The `@` prefix is faster to type and still supported.

### Mutable Type Annotations

```glob
# Combine @ and ? for mutable typed variables
@?int counter = 0
@?str message = "Hello"
@?lst items = []

# Use in functions
fn add_item(@?lst items, item):
    items.append(item)
```

### Function Annotations

```glob
# Annotate function type
@asy download(url):
    return await http.get(url)

@fn calculate(x, y):
    return x + y

# Custom language blocks
@cs python:
    def analyze(data):
        return sum(data) / len(data)
```

### Ownership Annotations

```glob
# Reference (borrow)
fn print_data(@ref data):
    print(data)
    # data is borrowed, not owned

# Own (take ownership)
fn consume(@own data):
    process(data)
    # data is moved, caller loses access

# Copy (explicit duplicate)
fn backup(@copy data):
    return data
    # data is copied

# Move (transfer ownership)
fn transfer(@move resource):
    return resource
    # resource is moved
```

### Comparison Operator Annotations

```glob
# Use @ prefix for operator functions
result = @less(5, 10)        # 5 < 10 → true
result = @more(10, 5)        # 10 > 5 → true
result = @equal(5, 5)        # 5 == 5 → true
result = @not_equal(5, 10)   # 5 != 10 → true
result = @less_equal(5, 5)   # 5 <= 5 → true
result = @more_equal(10, 5)  # 10 >= 5 → true
```

### Logical Operator Annotations

```glob
# Logical operations
result = @and(true, false)   # true && false → false
result = @or(true, false)    # true || false → true
result = @not(true)          # !true → false
result = @xor(true, false)   # true XOR false → true
```

### Statistical Function Annotations

```glob
# Statistical operations
numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]

total = @sum(numbers)           # Sum of all numbers
average = @mean(numbers)        # Mean/average
middle = @median(numbers)       # Median value
most_common = @mode(numbers)    # Mode (most frequent)

# Advanced statistics
std = @stddev(numbers)          # Standard deviation
var = @variance(numbers)        # Variance
corr = @correlation(x, y)       # Correlation coefficient
cov = @covariance(x, y)         # Covariance

# Data operations
sorted_data = @sort(numbers)    # Sort ascending
reversed_data = @reverse(numbers)  # Reverse order
length = @len(numbers)          # Length/count
maximum = @max(numbers)         # Maximum value
minimum = @min(numbers)         # Minimum value
```

### Mathematical Function Annotations

```glob
# Logarithms and exponentials
result = @log(100, 10)    # Log base 10
result = @ln(2.718)       # Natural log
result = @exp(1)          # e^1

# Roots and powers
result = @sqrt(16)        # Square root
result = @cbrt(27)        # Cube root

# Rounding and truncation
result = @abs(-5)         # Absolute value
result = @floor(3.7)      # Floor (3)
result = @ceil(3.2)       # Ceiling (4)
result = @round(3.5)      # Round (4)
result = @trunc(3.9)      # Truncate (3)
result = @frac(3.7)       # Fractional part (0.7)

# Sign functions
result = @sign(-5)        # Sign (-1, 0, or 1)
result = @signum(10)      # Signum function
```

### Control Flow Annotations

```glob
# Annotated control flow (for clarity)
@if condition:
    do_something()
@elif other_condition:
    do_other()
@else:
    do_default()

# Annotated loops
@for item in collection:
    process(item)

@while condition:
    update()

@loop:
    if done:
        @break
    if skip:
        @continue
@return:
     @return(number) in [1, 2, 3, 4, 5].
     @return(result) in [calculation_fn].

```

---

## Control Flow

### If-Elif-Else

```glob
if age >= 18:
    print("Adult")
elif age >= 13:
    print("Teenager")
else:
    print("Child")
```

### Loops

```glob
# For loop
for number in [1, 2, 3, 4, 5]:
    print(number)

# While loop
?count = 0
while ?count < 10:
    print(?count)
    ?count = ?count + 1

# Infinite loop
loop:
    user_input = input("Enter 'quit' to exit: ")
    if user_input == "quit":
        break
```

---

## Ownership Model

```glob
# Own - Transfer ownership
fn process_data(@own data):
    transform(data)
    return data as @global data
    # Caller can't use data anymore and data is global owned by variable

# Ref - Borrow reference
fn read_data(@ref data):
    print(data)
    # data is borrowed, Caller still owns data

# Copy - Explicit duplicate
fn backup_data(@copy data):
    return data
    # Both copies exist
```

---

## UI Components

```glob
# Template literal syntax (recommended)
ui.render(`<button text="Click Me" color="blue">`)
ui.render(`<slider min=0 max=100 value=50>`)
ui.render(`<table headers=["Name", "Age"] rows=data>`)
ui.render(`<chart type="bar" data=values>`)

# Legacy syntax (still supported)
ui.print(^÷^[button{text="Click Me", color="blue"}])
```

**Recommendation:** Use template literals with backticks for new code.

---

## Multi-Language Integration

### Python Integration

```glob
import python{numpy, pandas}

# Recommended: extern syntax
extern python {
    fn analyze(data: list) -> float {
        import numpy as np
        return np.mean(data)
    }
}

# Legacy syntax (still supported)
@cs python:
    import numpy as np
    def analyze_legacy(data):
        return np.mean(data)

main():
    data = [1, 2, 3, 4, 5]
    result = analyze(data)
    print(result)
```

### Rust Integration

```glob
import rust.tokio

extern rust {
    fn fast_compute(n: u64) -> u64 {
        n * n
    }
}

main():
    result = fast_compute(100)
    print(result)
```

### JavaScript Integration

```glob
imp js: axios

@cs js:
    export function formatDate(date) {
        return new Date(date).toLocaleDateString();
    }

mn main():
    formatted = formatDate("2024-01-15")
    print(formatted)
```

---

## Scientific Computing

```glob
# Physics
def speed = 10 m/s
def acceleration = 9.81 m/s^2
def energy = m * c^2

# Chemistry
def pH = -log10([H+])
def molarity = 2.5 mol/L

# Use in calculations
@fn kinetic_energy(@float mass, @float velocity) -> @float:
    return 0.5 * mass * velocity^2
```

---

## Best Practices

### 1. Use Immutable by Default

```glob
# Good: Immutable unless you need to change it
def MAX_RETRIES = 3
def API_URL = "https://api.example.com"

# Only use mutable when necessary
?retry_count = 0
```

### 2. Choose Clear Import Style

```glob
# Good: Grouped by category
imp [
    python: (numpy, pandas),
    std: (io, http),
    my_utils
]

# Also good: One per line for clarity
imp python: numpy
imp python: pandas
imp std.io
imp std.http
```

### 3. Use Annotations for Clarity

```glob
# Good: Annotated types
@fn calculate(@int x, @int y) -> @int:
    return x + y

# Good: Annotated ownership
@fn process(@ref data):
    for item in data:
        print(item)
```

### 4. Prefer @ Functions for Statistics

```glob
# Good: Use @ annotations
average = @mean(numbers)
total = @sum(values)

# Instead of manual calculation
# average = sum(numbers) / len(numbers)
```

---

## Complete Example

```glob
# Complete GUL program with new features

# Flexible imports
imp [
    python: (numpy, pandas),
    std: (io, http),
    rust: tokio
]

# Constants (immutable)
def API_URL = "https://api.example.com"
def MAX_RETRIES = 3

# Global mutable state
@global ?app_state = {
    users: [],
    request_count: 0
}

# Async function with annotations
@asy fetch_users(endpoint):
    ?retries = 0

    @while ?retries < MAX_RETRIES:
        try:
            response = await http.get(API_URL + endpoint)
            return response.json()
        catch error:
            ?retries = ?retries + 1

    return null

# Sync function with type annotations
@fn analyze_data(@ref data) -> @map:
    @int count = @len(data)
    @float average = @mean(data)
    @float total = @sum(data)

    return {
        count: count,
        average: average,
        total: total
    }

# Python integration
@cs python:
    import pandas as pd
    def create_dataframe(data):
        return pd.DataFrame(data)

# Main entry point
mn main():
    # Fetch data
    users = await fetch_users("/users")

    if users != null:
        # Analyze
        stats = analyze_data(@ref users)

        # Display
        ui.print(^÷^[table{data=stats}])

        # Update global state
        @global ?app_state.users = users
        @global ?app_state.request_count = @global ?app_state.request_count + 1

        print("Analysis complete!")
    else:
        print("Failed to fetch users")
```

---

## Quick Reference

### Keywords

- `def` - Immutable variable
- `?variable` - Mutable variable
- `fn` - Synchronous function
- `asy` - Async function
- `mn` - Main entry point
- `imp` - Import
- `cs` - Custom language block
- `await` - Wait for async
- `if`, `elif`, `else` - Conditionals
- `for`, `while`, `loop` - Loops
- `break`, `continue` - Loop control
- `return` - Return value
- `try`, `catch` - Error handling

### Annotations (@ prefix)

- Types: `@int`, `@str`, `@float`, `@bool`, `@lst`, `@map`, `@set`
- Ownership: `@ref`, `@own`, `@copy`, `@move`
- Functions: `@asy`, `@fn`, `@cs`
- Operators: `@less`, `@more`, `@equal`, `@and`, `@or`, `@not`
- Statistics: `@sum`, `@mean`, `@median`, `@stddev`, `@correlation`
- Math: `@log`, `@ln`, `@exp`, `@sqrt`, `@abs`, `@floor`, `@ceil`
- Scope: `@global`, `@static`, `@local`

### Mutability

- `def x = 5` - Immutable
- `?x = 5` - Mutable
- `@?int x = 5` - Mutable with type annotation

### Import Formats

- Individual: `imp package`
- Grouped: `imp [pkg1, pkg2]` or `imp {pkg1, pkg2}` or `imp (pkg1, pkg2)`
- Language-specific: `imp python: numpy` or `imp python: {numpy, pandas}`

---

**Ready to code in GUL? Check out the examples in `examples/` directory!**
