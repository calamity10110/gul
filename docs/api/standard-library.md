# GUL Standard Library Reference v3.0

The GUL Standard Library provides 13 comprehensive modules for common programming tasks.

**Version**: 0.13.0 | **Status**: Production Ready

## Overview

GUL's standard library includes:

- **7 Core Modules**: fs, path, env, time, process, random, crypto
- **3 Data Modules**: collections, string, bytes
- **3 Network Modules**: http, websocket, tcp, udp (experimental)

Total: **110+ functions** across all modules

---

## Core Modules

### std.fs - File System

File system operations for reading, writing, and managing files.

```gul
import std{fs}

mn:
    # Check if file exists
    if fs.exists("config.json"):
        print("Config found")

    # Read/write files
    content = fs.read_file("data.txt")
    fs.write_file("output.txt", "Hello!")

    # Directory operations
    fs.mkdir("output")
    files = fs.list_dir(".")
```

### std.path - Path Manipulation

Cross-platform path handling.

```gul
import std{path}

mn:
    let full_path = path.join("dir", "file.txt")
    let filename = path.basename(full_path)
    let extension = path.extension(full_path)
```

### std.env - Environment

Access environment variables and system information.

```gul
import std{env}

mn:
    let api_key = env.get("API_KEY", "default")
    let home = env.home_dir()
    let cwd = env.current_dir()
```

### std.time - Time and Dates

Time, duration, and date functionality.

```gul
import std{time}

mn:
    let now = time.now()
    await time.sleep(1000)  # Sleep 1 second
    let formatted = time.format(now, "%Y-%m-%d")
```

### std.process - Process Management

Spawn and manage system processes.

```gul
import std{process}

mn:
    let output = process.run("ls", ["-la"])
    print(output.stdout)
```

### std.random - Random Numbers

Generate random numbers and UUIDs.

```gul
import std{random}

mn:
    let num = random.int(1, 100)
    let uuid = random.uuid()
    let choice = random.choice([1, 2, 3])
```

### std.crypto - Cryptography

Hashing and encryption functions.

```gul
import std{crypto}

mn:
    let hash = crypto.sha256("password")
    let encoded = crypto.base64_encode("data")
```

---

## Data Modules

### std.collections - Data Structures

Advanced data structures (Set, Queue, Stack, Heap).

```gul
import std{collections}

mn:
    let set = collections.Set()
    set.insert(1)
    set.insert(2)

    let queue = collections.Queue()
    queue.push(1)
    let item = queue.pop()
```

### std.string - String Utilities

String manipulation and formatting.

```gul
import std{string}

mn:
    let parts = string.split("a,b,c", ",")
    let joined = string.join(parts, "-")
    let upper = string.to_upper("hello")
```

### std.bytes - Binary Data (NEW in v0.13.0)

Binary data operations.

```gul
import std{bytes}

mn:
    let data = bytes.from_string("Hello")
    let hex = bytes.to_hex(data)
    let b64 = bytes.to_base64(data)
    let concat = bytes.concat(data1, data2)
```

**Functions** (10 total):

- `from_string(s: String) -> Bytes`
- `to_string(b: Bytes) -> String`
- `from_hex(hex: String) -> Bytes`
- `to_hex(b: Bytes) -> String`
- `from_base64(b64: String) -> Bytes`
- `to_base64(b: Bytes) -> String`
- `concat(b1: Bytes, b2: Bytes) -> Bytes`
- `slice(b: Bytes, start: Number, end: Number) -> Bytes`
- `length(b: Bytes) -> Number`
- `equals(b1: Bytes, b2: Bytes) -> Boolean`

---

## Network Modules (Experimental)

### std.http - HTTP Client/Server

HTTP requests and server functionality.

```gul
import std{http}

async fetch_data():
    let response = await http.get("https://api.example.com")
    return response.json()

mn:
    let data = await fetch_data()
    print(data)
```

### std.websocket - WebSocket (9 functions)

WebSocket client and server.

```gul
import std{websocket}

mn:
    let ws = websocket.connect("ws://localhost:8080")
    websocket.send(ws, "Hello")
    let msg = websocket.receive(ws)
```

**Functions**:

- `connect(url: String) -> WebSocket`
- `send(ws: WebSocket, data: String)`
- `receive(ws: WebSocket) -> String`
- `listen(port: Number) -> WebSocketServer`
- `accept(server: WebSocketServer) -> WebSocket`
- `ping(ws: WebSocket)`
- `pong(ws: WebSocket)`
- `close(ws: WebSocket)`
- `is_open(ws: WebSocket) -> Boolean`

### std.tcp - TCP Sockets (13 functions)

TCP client and server operations.

```gul
import std{tcp}

mn:
    let socket = tcp.connect("localhost", 8080)
    tcp.send(socket, "Hello")
    let data = tcp.receive(socket)
```

### std.udp - UDP Sockets (14 functions)

UDP socket operations with multicast support.

```gul
import std{udp}

mn:
    let socket = udp.bind("0.0.0.0", 8080)
    udp.send_to(socket, "Hello", "localhost", 9000)
```

---

## Module Loading

```gul
# Single module
import std.fs

# Multiple modules
import std{fs, path, env}

# Aliasing
import std.collections as col
```

---

## See Also

- [Syntax Reference](../reference/syntax.md) - v3.0 syntax guide
- [Package Catalog](../reference/package-catalog.md) - 58 available packages
- [Quick Reference](../QUICK_REFERENCE.md) - Cheat sheet

---

**Last Updated**: 2025-12-18  
**Version**: 0.13.0
