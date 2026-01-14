# GUL Standard Library API Reference

**Version**: 0.14.0-dev | **Syntax**: v3.2 | **Updated**: 2026-01-08

Complete reference for GUL's 13 standard library modules with 110+ functions.

---

## Table of Contents

1. [Core Modules](#core-modules)
2. [I/O Module](#io-module)
3. [HTTP Module](#http-module)
4. [Math Module](#math-module)
5. [Database Module](#database-module)
6. [File System](#file-system)
7. [JSON Module](#json-module)
8. [Time Module](#time-module)
9. [Crypto Module](#crypto-module)
10. [Collections](#collections)
11. [String Operations](#string-operations)
12. [System Module](#system-module)
13. [Async Runtime](#async-runtime)

---

## Core Modules

### `std.io` - Input/Output

```gul
@imp std.io

# Print to console (Core Builtins)
print("Hello")                       # Simple print (global)
println("With newline")              # Print with newline (global)
printf("Name: %s", name)             # Formatted print (global)

# Input
let name = input("Enter name: ")     # Global input
let age = input_int("Age: ")         # Global int input
```

### `std.math` - Mathematics

```gul
@imp std.math

# Basic operations
let result = math.sqrt(@float(16.0))   # 4.0
let power = math.pow(@float(2.0), 3.0) # 8.0
let abs_val = math.abs(@int(-5))       # 5

# Trigonometry
let sin_val = math.sin(@float(1.57))
let cos_val = math.cos(@float(0.0))

# Constants
let pi = math.PI
let e = math.E
```

### `std.http` - HTTP Client/Server

```gul
@imp std.http

# GET request
@async fetch_data():
    let response = await http.get("https://api.example.com/data")
    let data = response.json()
    return data

# POST request
@async post_data():
    let data = @dict{name: "Alice", age: 30}
    let response = await http.post("https://api.example.com/users", data)
    return response.json()

# Server
@fn handler(request) -> dict:
    return @dict{status: "ok", message: "Hello!"}

mn:
    http.listen(8080, handler)
```

---

## Database Module

### `std.db` - Database Operations

```gul
@imp std.db

# Connect
let conn = db.connect("postgresql://localhost/mydb")

# Query
let users = conn.query("SELECT * FROM users WHERE age > ?", @list[18])

# Insert
conn.execute("INSERT INTO users (name, age) VALUES (?, ?)",
             @list["Alice", 30])

# Transaction
conn.begin()
try:
    conn.execute("UPDATE users SET age = ? WHERE id = ?", @list[31, 1])
    conn.commit()
catch error:
    conn.rollback()
```

---

## File System

### `std.fs` - File Operations

```gul
@imp std.fs

# Read file
let content = fs.read_file("data.txt")
let lines = fs.read_lines("data.txt")

# Write file
fs.write_file("output.txt", "Hello, World!")
fs.append_file("log.txt", "New log entry\n")

# File info
let size = fs.file_size("data.txt")
let exists = fs.exists("data.txt")

# Directory operations
fs.create_dir("new_folder")
let files = fs.list_dir(".")
fs.remove_file("temp.txt")
```

---

## JSON Module

### `std.json` - JSON Operations

```gul
@imp std.json

# Parse JSON
let data = json.parse('{"name": "Alice", "age": 30}')
let name = data["name"]

# Stringify
let obj = @dict{name: "Bob", age: 25}
let json_str = json.stringify(obj)

# Pretty print
let pretty = json.stringify_pretty(obj, indent=2)
```

---

## Time Module

### `std.time` - Time/Date Operations

```gul
@imp std.time

# Current time
let now = time.now()
let timestamp = time.timestamp()

# Formatting
let formatted = time.format(now, "%Y-%m-%d %H:%M:%S")

# Sleep
time.sleep(@int(1))              # Sleep 1 second
time.sleep_ms(@int(500))         # Sleep 500ms

# Parsing
let date = time.parse("2026-01-08", "%Y-%m-%d")
```

---

## Crypto Module

### `std.crypto` - Cryptography

```gul
@imp std.crypto

# Hashing
let hash = crypto.sha256("password")
let md5 = crypto.md5("data")

# Encoding
let encoded = crypto.base64_encode("Hello")
let decoded = crypto.base64_decode(encoded)

# Random
let random_bytes = crypto.random_bytes(@int(16))
let uuid = crypto.uuid()
```

---

## Collections

### `std.collections` - Advanced Collections

```gul
@imp std.collections

# Queue
let queue = collections.Queue()
queue.enqueue(@int(1))
let item = queue.dequeue()

# Stack
let stack = collections.Stack()
stack.push(@int(1))
let top = stack.pop()

# HashMap
let map = collections.HashMap()
map.set("key", "value")
let val = map.get("key")

# Common builtins for collections
let length = len(map)                # Get size of any collection
```

---

## String Operations

### `std.str` - String Utilities

```gul
@imp std.str

# Operations
let upper = str.to_upper("hello")     # "HELLO"
let lower = str.to_lower("WORLD")     # "world"
let trimmed = str.trim("  hi  ")      # "hi"

# Checking
let starts = str.starts_with("hello", "he")  # true
let contains = str.contains("hello", "ll")   # true

# Splitting/Joining
let parts = str.split("a,b,c", ",")   # ["a", "b", "c"]
let joined = str.join(@list["a", "b"], ",")  # "a,b"

# Replace
let replaced = str.replace("hello world", "world", "GUL")
```

---

## System Module

### `std.sys` - System Operations

```gul
@imp std.sys

# Environment
let path = sys.env("PATH")
sys.set_env("MY_VAR", "value")

# Process
let exit_code = sys.execute("ls -la")
let output = sys.capture_output("pwd")

# Platform info
let os = sys.os_name()           # "linux", "windows", "macos"
let arch = sys.arch()            # "x86_64", "arm64"
```

---

## Async Runtime

### `std.async` - Asynchronous Operations

```gul
@imp std.async

# Spawn task
async task1():
    await time.sleep(@int(1))
    return @str("Done")

async task2():
    await time.sleep(@int(2))
    return @str("Complete")

# Run concurrently
@@async main():
    let results = await async.gather(@list[task1(), task2()])
    print(results)

mn:
    await main()
```

---

## Complete Example

```gul
@imp std{io, http, json, db, fs, time}

struct User:
    id: int
    name: str
    email: str

    @fn to_dict(self) -> dict:
        return @dict{
            id: self.id,
            name: self.name,
            email: self.email
        }

@async fetch_users():
    # HTTP request
    let response = await http.get("https://api.example.com/users")
    let data = response.json()
    return data

@async save_to_db(users):
    # Database
    let conn = db.connect("postgresql://localhost/app")

    for user in users:
        conn.execute(
            "INSERT INTO users (name, email) VALUES (?, ?)",
            @list[user["name"], user["email"]]
        )

@async save_to_file(users):
    # File system
    let json_data = json.stringify_pretty(users)
    fs.write_file("users.json", json_data)

@@async main():
    print("Fetching users...")
    let users = await fetch_users()

    print("Saving to database...")
    await save_to_db(users)

    print("Saving to file...")
    await save_to_file(users)

    print("Done!")

mn:
    await main()
```

---

## Module Summary

| Module            | Functions | Purpose              |
| ----------------- | --------- | -------------------- |
| `std.io`          | 8+        | Console I/O          |
| `std.http`        | 12+       | HTTP client/server   |
| `std.math`        | 20+       | Mathematics          |
| `std.db`          | 10+       | Database operations  |
| `std.fs`          | 15+       | File system          |
| `std.json`        | 4+        | JSON parsing         |
| `std.time`        | 10+       | Time/date            |
| `std.crypto`      | 8+        | Cryptography         |
| `std.collections` | 12+       | Advanced collections |
| `std.str`         | 10+       | String operations    |
| `std.sys`         | 8+        | System operations    |
| `std.async`       | 5+        | Async runtime        |
| `std.ui`          | 8+        | UI components        |

**Total**: 13 modules, 110+ functions

---

## See Also

- [HTTP API](http.md) - Detailed HTTP reference
- [Database API](database.md) - Database guide
- [Math & Science](math-science.md) - Scientific computing
- [Quick Reference](../QUICK_REFERENCE.md) - Syntax cheat sheet

---

**Last Updated**: 2026-01-08  
**Version**: 0.14.0-dev  
**Syntax**: v3.2
