# GUL v3.0 Quick Reference

## Keywords

### Variables

- `let` - Immutable variable
- `var` - Mutable variable

### Functions

- `fn` - Function declaration
- `async` - Async function
- `return` - Return value

### Control Flow

- `if`, `elif`, `else` - Conditionals
- `for`, `while`, `loop` - Loops
- `in` - Iterator
- `break`, `continue` - Loop control

### Imports

- `import` - Import modules
- `use` - Alternative import syntax

### Entry Point

- `mn:` - Main entry block

### Error Handling

- `try`, `catch`, `finally` - Exception handling
- `throw` - Throw exception
- `await` - Await async

## Syntax Examples

### Variables

```gul
let x = 10          # Immutable
var y = 20          # Mutable
y = y + 1
```

### Functions

```gul
fn add(a, b):
    return a + b

fn typed(x: int) -> int:
    return x * 2

async fetch(url):
    return await http.get(url)
```

### Control Flow

```gul
if x > 10:
    print("big")
elif x > 5:
    print("medium")
else:
    print("small")

for i in 0..10:
    print(i)

while x < 100:
    x = x * 2
```

### Data Structures

```gul
let list = [1, 2, 3]
let dict = {name: "Alice", age: 25}
```

### Imports

```gul
import std{http, math}
import python{numpy, pandas}
```

### Main Entry

```gul
mn:
    print("Hello, GUL!")
    await run_app()
```

## CLI Commands

### Package Management

```bash
gul package list                    # List all packages
gul package list --language rust    # Filter by language
gul package info <name>             # Package details
gul package search <query>          # Search packages
gul package install <name>          # Install package
gul package update <name>           # Update package
gul package remove <name>           # Remove package
gul package audit                   # Security audit
gul package outdated                # Check updates
```

### AI Commands

```bash
gul ai status                       # Show AI config
gul ai set-provider openai          # Set provider
gul ai set-model gpt-4              # Set model
gul ai set-key <key>                # Set API key
```

### Runtime Commands

```bash
gul runtime python "print('hi')"    # Run Python
gul runtime js "console.log('hi')"  # Run JavaScript
gul runtime load-lib lib.so         # Load Rust lib
```

## Standard Library (13 Modules)

### Core

- `std.fs` - File system
- `std.path` - Path manipulation
- `std.env` - Environment
- `std.time` - Time/date
- `std.process` - Process management
- `std.random` - Random numbers
- `std.crypto` - Cryptography

### Data

- `std.collections` - Data structures
- `std.string` - String utilities
- `std.bytes` - Binary data

### Networking

- `std.http` - HTTP client/server
- `std.websocket` - WebSocket (9 functions)
- `std.tcp` - TCP sockets (13 functions)
- `std.udp` - UDP sockets (14 functions)

## Package Ecosystem (58 Packages)

### Rust (15)

actix-web, axum, warp, tokio, serde, sqlx, diesel, dioxus, tauri, leptos, regex, rayon, tracing, anyhow, clap, reqwest

### Python (6)

django, flask, fastapi, numpy, pandas, pydantic

### JavaScript (16)

react, angular, vue, next.js, svelte, nestjs, nodejs, express, d3.js, jest, webpack, tailwindcss, axios, socket.io, graphql, prisma

### Multi-language (21)

C++, Java, Go, C#, TypeScript, Ruby, PostgreSQL, MySQL, SQLite, MongoDB, Redis, etc.

## Migration from v2.0

```gul
# v2.0              # v3.0
const x = 5         let x = 5
mut y = 10          var y = 10
def add(a, b):      fn add(a, b):
asy fetch():        async fetch():
main():             mn:
```

## File Extensions

- `.gul` - GUL source files (recommended)
- `.mn` - Main entry files (legacy)

## Version

Current: **v0.13.0** (Production Ready)
