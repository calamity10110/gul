# GUL Standard Library Reference

The GUL Standard Library provides a comprehensive set of modules for common programming tasks.

## Core Modules

### std.io - Input/Output

File reading, writing, and console I/O operations.

```gul
import std{io}

# Read file
content = io.read_file("config.txt")

# Write file
io.write_file("output.txt", "Hello, World!")

# Print to console
io.println("Message")
```

**Functions:**

- `read_file(path: str) -> str` - Read entire file as string
- `write_file(path: str, content: str)` - Write string to file
- `read_lines(path: str) -> [str]` - Read file as list of lines
- `println(message: str)` - Print with newline
- `print(message: str)` - Print without newline

---

### std.http - HTTP Client/Server

HTTP requests and server functionality.

```gul
import std{http}

# HTTP Client
async fn fetch_data():
    response = await http.get("https://api.example.com/data")
    if response.status == 200:
        data = response.json()
        return data

# HTTP Server
const PORT = 8080

async handle_request(req):
    return {
        status: 200,
        headers: {"Content-Type": "application/json"},
        body: {message: "Hello!"}
    }

main():
    server = http.Server(PORT)
    server.on("request", handle_request)
    await server.start()
```

**Client Functions:**

- `get(url: str, headers: {str: str}?) -> Response` - GET request
- `post(url: str, data: any, headers: {str: str}?) -> Response` - POST request
- `put(url: str, data: any, headers: {str: str}?) -> Response` - PUT request
- `delete(url: str, headers: {str: str}?) -> Response` - DELETE request

**Server Classes:**

- `Server(port: int)` - Create HTTP server
- `server.on(event: str, handler: fn)` - Register event handler
- `server.start()` - Start server (async)

**Response Object:**

- `status: int` - HTTP status code
- `headers: {str: str}` - Response headers
- `body: str` - Response body
- `json() -> any` - Parse JSON response

---

### std.json - JSON Parsing

Parse and stringify JSON data.

```gul
import std{json}

# Parse JSON
data = json.parse('{"name": "Alice", "age": 25}')
print(data.name)  # "Alice"

# Stringify object
obj = {name: "Bob", age: 30}
json_str = json.stringify(obj)
print(json_str)  # {"name":"Bob","age":30}
```

**Functions:**

- `parse(text: str) -> any` - Parse JSON string to object
- `stringify(obj: any, pretty: bool = false) -> str` - Convert object to JSON

---

### std.db - Database Interface

Unified interface for SQL databases.

```gul
import std{db}

# Connect to database
conn = await db.connect("sqlite:///data.db")

# Execute query
users = await conn.query("SELECT * FROM users WHERE active = ?", [true])

# Execute with named parameters
user = await conn.query_one(
    "SELECT * FROM users WHERE id = :id",
    {id: 123}
)

# Execute statement
await conn.execute(
    "INSERT INTO users (name, email) VALUES (?, ?)",
    ["Alice", "alice@example.com"]
)

# Transactions
tx = await conn.begin()
try:
    await tx.execute("UPDATE users SET balance = balance - 100 WHERE id = 1")
    await tx.execute("UPDATE users SET balance = balance + 100 WHERE id = 2")
    await tx.commit()
catch e:
    await tx.rollback()
```

**Functions:**

- `connect(url: str) -> Connection` - Connect to database
- `conn.query(sql: str, params: [any]) -> [[any]]` - Execute query, return all rows
- `conn.query_one(sql: str, params: [any]) -> [any]` - Execute query, return one row
- `conn.execute(sql: str, params: [any])` - Execute statement
- `conn.begin() -> Transaction` - Begin transaction
- `tx.commit()` - Commit transaction
- `tx.rollback()` - Rollback transaction

---

### std.math - Mathematics

Mathematical functions and constants.

```gul
import std{math}

# Constants
print(math.PI)     # 3.141592653589793
print(math.E)      # 2.718281828459045

# Basic math
result = math.sqrt(16)  # 4.0
power = math.pow(2, 8)  # 256.0

# Trigonometry
sin_val = math.sin(math.PI / 2)  # 1.0
cos_val = math.cos(0)            # 1.0
tan_val = math.tan(math.PI / 4)  # 1.0

# Logarithms
log_val = math.log(math.E)    # 1.0
log10_val = math.log10(100)   # 2.0

# Rounding
ceil_val = math.ceil(4.3)     # 5.0
floor_val = math.floor(4.7)   # 4.0
round_val = math.round(4.5)   # 5.0
```

**Constants:**

- `PI: float` - π (3.14159...)
- `E: float` - Euler's number (2.71828...)
- `TAU: float` - 2π (6.28318...)

**Functions:**

- `sqrt(x: float) -> float` - Square root
- `pow(x: float, y: float) -> float` - Power
- `abs(x: float) -> float` - Absolute value
- `sin(x: float) -> float` - Sine
- `cos(x: float) -> float` - Cosine
- `tan(x: float) -> float` - Tangent
- `asin(x: float) -> float` - Arcsine
- `acos(x: float) -> float` - Arccosine
- `atan(x: float) -> float` - Arctangent
- `log(x: float) -> float` - Natural logarithm
- `log10(x: float) -> float` - Base-10 logarithm
- `ceil(x: float) -> float` - Round up
- `floor(x: float) -> float` - Round down
- `round(x: float) -> float` - Round to nearest

---

### std.time - Time and Dates

Time, duration, and date functionality.

```gul
import std{time}

# Current time
now = time.now()
print(now)  # 2025-12-10T10:30:00Z

# Sleep (async)
await time.sleep(1000)  # Sleep for 1 second (1000ms)

# Duration
start = time.now()
# ... do work ...
elapsed = time.now() - start
print(f"Took {elapsed}ms")

# Format time
formatted = time.format(now, "%Y-%m-%d %H:%M:%S")
print(formatted)  # 2025-12-10 10:30:00

# Parse time
parsed = time.parse("2025-12-10 10:30:00", "%Y-%m-%d %H:%M:%S")
```

**Functions:**

- `now() -> Time` - Get current time
- `sleep(ms: int)` - Sleep for milliseconds (async)
- `format(time: Time, fmt: str) -> str` - Format time as string
- `parse(text: str, fmt: str) -> Time` - Parse time from string

---

### std.fs - File System

File system operations.

```gul
import std{fs}

# Check if file exists
if fs.exists("config.json"):
    print("Config found")

# Create directory
fs.mkdir("output")
fs.mkdir_all("path/to/nested/dir")  # Create all parent directories

# List directory
files = fs.list_dir(".")
for file in files:
    print(file)

# Copy file
fs.copy("source.txt", "destination.txt")

# Move/rename file
fs.move("old_name.txt", "new_name.txt")

# Delete file
fs.remove("temp.txt")

# Delete directory
fs.remove_dir("temp_folder")
fs.remove_dir_all("folder_with_contents")  # Recursive delete

# Get file info
info = fs.stat("file.txt")
print(info.size)      # File size in bytes
print(info.modified)  # Last modified time
```

**Functions:**

- `exists(path: str) -> bool` - Check if path exists
- `is_file(path: str) -> bool` - Check if path is a file
- `is_dir(path: str) -> bool` - Check if path is a directory
- `mkdir(path: str)` - Create directory
- `mkdir_all(path: str)` - Create directory and parents
- `list_dir(path: str) -> [str]` - List directory contents
- `copy(src: str, dst: str)` - Copy file
- `move(src: str, dst: str)` - Move/rename file
- `remove(path: str)` - Delete file
- `remove_dir(path: str)` - Delete empty directory
- `remove_dir_all(path: str)` - Delete directory recursively
- `stat(path: str) -> FileInfo` - Get file information

---

### std.collections - Data Structures

Advanced data structures.

```gul
import std{collections}

# HashMap
map = collections.HashMap()
map.insert("key", "value")
val = map.get("key")
map.remove("key")

# HashSet
set = collections.HashSet()
set.insert(1)
set.insert(2)
if set.contains(1):
    print("Found")

# Queue
queue = collections.Queue()
queue.push(1)
queue.push(2)
item = queue.pop()  # FIFO

# Stack
stack = collections.Stack()
stack.push(1)
stack.push(2)
item = stack.pop()  # LIFO
```

**Classes:**

- `HashMap<K, V>` - Hash table
- `HashSet<T>` - Hash set
- `Queue<T>` - FIFO queue
- `Stack<T>` - LIFO stack
- `LinkedList<T>` - Doubly-linked list
- `BinaryTree<T>` - Binary tree

---

### std.crypto - Cryptography

Hashing and encryption functions.

```gul
import std{crypto}

# Hashing
sha256 = crypto.sha256("Hello, World!")
md5 = crypto.md5("data")

# Base64
encoded = crypto.base64_encode("text")
decoded = crypto.base64_decode(encoded)

# Random
random_bytes = crypto.random_bytes(32)
random_int = crypto.random_int(1, 100)
```

**Functions:**

- `sha256(data: str) -> str` - SHA-256 hash
- `md5(data: str) -> str` - MD5 hash
- `base64_encode(data: str) -> str` - Base64 encode
- `base64_decode(data: str) -> str` - Base64 decode
- `random_bytes(n: int) -> [int]` - Generate random bytes
- `random_int(min: int, max: int) -> int` - Generate random integer

---

### std.regex - Regular Expressions

Pattern matching with regular expressions.

```gul
import std{regex}

# Match
if regex.match(r"\d+", "123"):
    print("Is a number")

# Find all matches
matches = regex.find_all(r"\w+", "Hello World")
print(matches)  # ["Hello", "World"]

# Replace
result = regex.replace(r"\d+", "NUMBER", "I have 5 apples and 3 oranges")
print(result)  # "I have NUMBER apples and NUMBER oranges"
```

**Functions:**

- `match(pattern: str, text: str) -> bool` - Test if pattern matches
- `find(pattern: str, text: str) -> str?` - Find first match
- `find_all(pattern: str, text: str) -> [str]` - Find all matches
- `replace(pattern: str, replacement: str, text: str) -> str` - Replace matches

---

## See Also

- [HTTP Module Deep Dive](http.md)
- [Database Module Guide](database.md)
- [File System Operations](filesystem.md)
- [Math & Science Library](math-science.md)

---

**Note**: This is a high-level overview. For detailed API documentation, see the individual module pages.
