# GLOB Language - Complete Syntax Guide (v0.11.0)

**GLOB** (Global Language for Optimized Building) - A modern, multi-paradigm programming language

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

### What is GLOB?

GLOB combines simplicity with power:

- **Python-like** syntax (easy to read)
- **Rust-like** safety (memory safe)
- **JavaScript-like** async (non-blocking I/O)
- **Scientific notation** (physics, chemistry, math)
- **Multi-language** (embed Python, Rust, JS, SQL)

### Your First Program

```glob
# hello.mn - Your first GLOB program

mn main():
    print("Hello, GLOB!")
```

Run it: `glob run hello.mn`

---

## Variables and Mutability

### Immutable Variables (Default)

```glob
# Immutable - cannot be changed after creation
def name = "Alice"
def age = 25
def PI = 3.14159

# This would cause an error:
# age = 26  # ERROR: Cannot modify immutable variable
```

### Mutable Variables (? prefix)

```glob
# Mutable - can be changed
?count = 0
?total = 100
?name = "Bob"

# Now we can change them
?count = ?count + 1  # OK: count is now 1
?total = ?total - 50  # OK: total is now 50
?name = "Alice"       # OK: name is now "Alice"
```

### When to Use Mutable vs Immutable

```glob
# Use immutable for constants
def MAX_USERS = 1000
def API_URL = "https://api.example.com"

# Use mutable for counters, accumulators
?counter = 0
?sum = 0

fn count_items(items):
    ?count = 0
    for item in items:
        ?count = ?count + 1
    return ?count
```

### Global vs Static Variables

```glob
# Global variables (managed by async functions)
@global ?app_state = {
    users: [],
    sessions: {}
}

# Static variables (managed by all functions)
@static cache = {}
@static config = load_config()

asy update_state(user):
    # Async functions can modify global state
    @global ?app_state.users.append(user)

fn get_cached(key):
    # All functions can access static variables
    return @static cache.get(key)
```

---

## Flexible Import System

GLOB supports multiple equivalent import syntaxes. Choose the style you prefer!

### Format 1: Individual Imports

```glob
# Import one at a time
imp std.io
imp std.http
imp python: numpy
imp python: matplotlib
imp rust: tokio
imp my_package
```

### Format 2: Grouped Imports with Brackets

```glob
# Group related imports
imp [
    python: (numpy, matplotlib, pandas),
    rust: (tokio, serde),
    my_package,
    other_package
]
```

### Format 3: Grouped Imports with Braces

```glob
# Using braces (equivalent to brackets)
imp python: {numpy, matplotlib}
imp rust: {tokio, serde}
imp {my_package, other_package}
```

### Format 4: Grouped Imports with Parentheses

```glob
# Using parentheses (also equivalent)
imp python: (numpy, matplotlib)
imp rust: (tokio, serde)
imp (my_package, other_package)
```

### Format 5: Mixed Styles

```glob
# Mix and match - all valid!
imp {
    python: [numpy, matplotlib],
    rust: (tokio, serde),
    my_package
}

# Or even:
imp [python: {numpy, matplotlib}, rust: (tokio)]
```

### Bracket Equivalence Rule

**Important:** `[]`, `{}`, and `()` work the same for grouping, as long as they match:

```glob
# All of these are valid:
imp [a, b, c]
imp {a, b, c}
imp (a, b, c)

# These are also valid:
imp python: [numpy, pandas]
imp python: {numpy, pandas}
imp python: (numpy, pandas)

# But these are ERRORS (mismatched):
# imp [a, b, c}    # ERROR: [ doesn't match }
# imp {a, b, c]    # ERROR: { doesn't match ]
```

### Import Examples

```glob
# Data science project
imp [
    python: (numpy, pandas, matplotlib, scipy),
    std: (io, math, stats)
]

# Web server project
imp {
    std: {http, io, json},
    rust: {tokio, serde},
    js: (express, axios)
}

# IoT project
imp (
    embedded: (gpio, i2c, spi),
    std: (time, io),
    rust: tokio
)
```

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
# Basic async function
@asy fetch_data(url):
    response = await http.get(url)
    return response.json()

# Async with error handling
@asy safe_fetch(url):
    try:
        data = await fetch_data(url)
        return data
    catch error:
        print("Error:", error)
        return null
```

---

## Annotations

GLOB uses `@` prefix for annotations. Annotations provide type hints, optimization hints, and semantic information.

### Type Annotations

```glob
# Basic types
@int age = 25
@str name = "Alice"
@float price = 19.99
@bool is_active = true
@char letter = 'A'

# Collection types
@lst numbers = [1, 2, 3, 4, 5]
@map person = {name: "Bob", age: 30}
@set unique_ids = {1, 2, 3}

# Special types
@null empty = null
@any value = anything
@void no_return = undefined
```

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
    # Caller can't use data anymore

# Ref - Borrow reference
fn read_data(@ref data):
    print(data)
    # Caller still owns data

# Copy - Explicit duplicate
fn backup_data(@copy data):
    return data
    # Both copies exist
```

---

## UI Components

```glob
# Inline UI components with ^÷^ syntax
ui.print(^÷^[button{text="Click Me", color="blue"}])
ui.print(^÷^[slider{min=0, max=100, value=50}])
ui.print(^÷^[table{headers=["Name", "Age"], rows=data}])
ui.print(^÷^[chart{type="bar", data=values}])
```

---

## Multi-Language Integration

### Python Integration

```glob
imp python: (numpy, pandas)

@cs python:
    import numpy as np
    def analyze(data):
        return np.mean(data)

mn main():
    data = [1, 2, 3, 4, 5]
    result = analyze(data)
    print(result)
```

### Rust Integration

```glob
imp rust: tokio

@cs rust:
    fn fast_compute(n: u64) -> u64 {
        n * n
    }

mn main():
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
# Complete GLOB program with new features

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

**Ready to code in GLOB? Check out the examples in `examples/` directory!**
