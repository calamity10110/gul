# Structure

**Version**: 0.13.0 | **Syntax**: v3.2 | **Updated**: 2025-12-28

---

# GUL Language - Project Structure Guide

**Understanding how GUL organizes your code**

---

## Table of Contents

1. [Introduction](#introduction)
2. [The Block System](#the-block-system)
3. [File Organization](#file-organization)
4. [New in v0.11.0](#new-in-v0110)
5. [Package Structure](#package-structure)
6. [Secret Management](#secret-management)
7. [Development Workflow](#development-workflow)
8. [Publishing Packages](#publishing-packages)
9. [Examples](#examples)

---

## Introduction

### What Makes GUL Different?

GUL uses an **automatic code organization system** that splits your code into logical blocks. This means:

✅ **You write everything in one file** (`main.mn`)  
✅ **Compiler automatically organizes** your code into structured blocks  
✅ **Clean separation** of concerns  
✅ **Easy to understand** project structure  
✅ **Safe secret management** built-in

Think of it like having an assistant that automatically organizes your desk!

---

## The Block System

### What are Blocks?

Blocks are separate files that contain specific types of code. GUL automatically creates these blocks from your main file.

### The Six Block Types

| Block File        | Purpose           | Contains                      |
| ----------------- | ----------------- | ----------------------------- |
| `imports.imp`     | Dependencies      | All `imp` statements          |
| `definitions.def` | Constants & Types | All `def` statements          |
| `async.asy`       | Async Functions   | All `asy` functions           |
| `functions.fnc`   | Sync Functions    | All `fn` functions            |
| `custom.cs`       | Foreign Code      | All `cs` language blocks      |
| `main.mn`         | Entry Point       | Only the `mn:` function |

### How It Works

```
┌─────────────────────────────────────┐
│  You Write: main.mn                 │
│                                     │
│  @imp std.io                         │
│  @imp std.http                       │
│                                     │
│  def API_URL = "https://..."        │
│  def MAX_RETRIES = 3                │
│                                     │
│  async fetch_data(url):               │
│      await http.get(url)     │
│                                     │
│  fn process(data):                  │
│      return transform(data)         │
│                                     │
│  mn:                         │
│      data = await fetch_data(url)   │
│      print(data)                    │
└─────────────────────────────────────┘
                  ↓
        Compiler Organizes
                  ↓
┌─────────────────────────────────────┐
│  Generated Files:                   │
│                                     │
│  imports.imp                        │
│  ├─ @imp std.io                      │
│  └─ @imp std.http                    │
│                                     │
│  definitions.def                    │
│  ├─ def API_URL = "https://..."     │
│  └─ def MAX_RETRIES = 3             │
│                                     │
│  async.async                          │
│  └─ async fetch_data(url): ...        │
│                                     │
│  functions.fnc                      │
│  └─ fn process(data): ...           │
│                                     │
│  main.mn (cleaned)                  │
│  └─ mn: ...                  │
└─────────────────────────────────────┘
```

---

## File Organization

### Basic Project Structure

```
my-project/
├── main.mn              # Your main file (you write this)
├── imports.imp          # Auto-generated imports
├── definitions.def      # Auto-generated definitions
├── async.async           # Auto-generated async functions
├── functions.fnc       # Auto-generated sync functions
├── custom.cs           # Auto-generated foreign code
├── package.toml        # Package metadata
├── scrt.def            # Secret annotations (safe to publish)
├── project.scrt        # Actual secrets (NEVER published)
└── README.md           # Documentation
```

### Detailed File Descriptions

#### 1. `main.mn` - Your Starting Point

This is where you write ALL your code initially:

```glob
# main.mn - You write everything here!

# Imports
@@imp std.io
@@imp std.http
@imp python: numpy

# Constants
def API_KEY = "your-key"
def MAX_USERS = 100

# Async functions
async fetch_users():
    await http.get("/users")

# Sync functions
fn calculate_total(items):
    return sum(items)

# Main entry point
mn:
    users = await fetch_users()
    print(users)
```

#### 2. `imports.imp` - Dependencies (Auto-generated)

Contains all your import statements:

```glob
# imports.imp - Auto-generated from main.mn

@@imp std.io
@@imp std.http
@imp python: numpy
```

#### 3. `definitions.def` - Constants & Types (Auto-generated)

Contains all your definitions:

```glob
# definitions.def - Auto-generated from main.mn

def API_KEY = "your-key"
def MAX_USERS = 100
```

#### 4. `async.asy` - Async Functions (Auto-generated)

Contains all async functions:

```glob
# async.async - Auto-generated from main.mn

async fetch_users():
    await http.get("/users")
```

#### 5. `functions.fnc` - Sync Functions (Auto-generated)

Contains all synchronous functions:

```glob
# functions.fnc - Auto-generated from main.mn

fn calculate_total(items):
    return sum(items)
```

#### 6. `custom.cs` - Foreign Code (Auto-generated)

Contains all embedded language blocks:

```glob
# custom.cs - Auto-generated from main.mn

cs python:
    import numpy as np
    fn analyze(data):
        return np.mean(data)

cs rust:
    fn fast_compute(n: u64) -> u64 {
        n * n
    }
```

#### 7. `main.mn` (cleaned) - Entry Point (Auto-generated)

Contains only the main function:

```glob
# main.mn - Auto-generated (cleaned version)

mn:
    users = await fetch_users()
    print(users)
```

---

## New in v0.11.0

### Flexible Import System

GUL v0.11.0 introduces multiple equivalent import syntaxes. Choose the style that works best for your project!

**Individual Imports:**

```glob
@@imp std.io
@imp python: numpy
@imp rust: tokio
```

**Grouped Imports:**

```glob
# Using brackets
@imp [python: (numpy, pandas), std: (io, http)]

# Using braces
@imp {python: {numpy, pandas}, std: {io, http}}

# Using parentheses
@imp (python: (numpy, pandas), std: (io, http))
```

**Key Feature:** `[]`, `{}`, and `()` are interchangeable!

### Mutability System

Clear distinction between mutable and immutable variables:

**Immutable (Default):**

```glob
def name = "Alice"      # Cannot be changed
name = "Bob"            # def is optional
```

**Mutable (? prefix):**

```glob
?count = 0              # Can be changed
?count = ?count + 1     # Modification allowed
```

**With Type Annotations:**

```glob
@?int counter = 0
@?str message = "Hello"
```

**Global vs Static:**

```glob
@global ?app_state = {}  # Managed by async functions
@static cache = {}       # Managed by all functions
```

### Annotation System

Comprehensive `@` prefix annotations for types, functions, and more:

**Type Annotations:**

```glob
@int age = 25
@str name = "Alice"
@lst numbers = [1, 2, 3]
@map data = {key: "value"}
```

**Function Annotations:**

```glob
@async fetch_data(url):
    await http.get(url)

@fn calculate(x, y):
    return x + y
```

**Ownership Annotations:**

```glob
@ref data      # Borrow
@own buffer    # Take ownership
@copy items    # Duplicate
```

**Statistical Annotations:**

```glob
average = @mean(numbers)
total = @sum(values)
std = @stddev(data)
```

---

## Package Structure

### Package Metadata - `package.toml`

Every GUL project has a `package.toml` file that describes the package:

```toml
[package]
name = "my-awesome-project"
version = "1.0.0"
authors = ["Your Name <you@example.com>"]
description = "A brief description of what this package does"
license = "MIT"
repository = "https://github.com/yourusername/my-awesome-project"

[dependencies]
# Standard library modules
std = "1.0"
ui = "0.5"
http = "2.0"

# External packages
data-utils = "1.2.3"
math-helpers = "0.8.0"

[dependencies.python]
# Python packages
numpy = "1.24.0"
pandas = "2.0.0"

[dependencies.js]
# JavaScript packages
express = "4.18.0"
axios = "1.4.0"

[blocks]
# Block file locations (usually auto-detected)
imports = "imports.imp"
definitions = "definitions.def"
async = "async.asy"
functions = "functions.fnc"
custom = "custom.cs"
main = "main.mn"
secrets = "scrt.def"

[build]
# Build configuration
target = "native"  # or "wasm", "embedded"
optimize = true
debug = false
```

### Multi-File Projects

For larger projects, you can split code across multiple files:

```
my-large-project/
├── main.mn              # Main entry point
├── src/
│   ├── utils.mn         # Utility functions
│   ├── models.mn        # Data models
│   ├── api.mn           # API functions
│   └── config.mn        # Configuration
├── tests/
│   ├── test_utils.mn
│   ├── test_models.mn
│   └── test_api.mn
├── package.toml
└── README.md
```

Import from other files:

```glob
# main.mn
@imp myproject.utils      # Imports from src/utils.mn
@imp myproject.models     # Imports from src/models.mn
@imp myproject.api        # Imports from src/api.mn

mn:
    user = models.User("Alice", 25)
    result = utils.process_data(user)
    print(result)
```

---

## Secret Management

### The Problem

How do you share code without exposing passwords, API keys, and other secrets?

### GUL's Solution

GUL has **three types of secret files**:

#### 1. `project.scrt` - Your Actual Secrets (NEVER PUBLISHED)

This file contains your real secrets:

```scrt
# project.scrt - NEVER commit this to git!

[api]
key = "sk_live_abc123xyz789"
secret = "your-secret-key"

[database]
host = "db.example.com"
username = "admin"
password = "super-secret-password"

[email]
smtp_password = "email-password-123"
```

**Important:**

- ❌ NEVER commit to git
- ❌ NEVER publish to package registry
- ✅ Automatically added to `.gitignore`
- ✅ Only exists on your local machine

#### 2. `secret.def` - Decrypted Secrets (LOCAL ONLY)

Runtime file with decrypted secrets:

```glob
# secret.def - Auto-generated at runtime

def API_KEY = "sk_live_abc123xyz789"
def DB_PASSWORD = "super-secret-password"
def SMTP_PASSWORD = "email-password-123"
```

**Important:**

- ❌ NEVER commit to git
- ❌ NEVER publish
- ✅ Auto-generated when you run your code
- ✅ Automatically deleted after use

#### 3. `scrt.def` - Annotations Only (SAFE TO PUBLISH)

Public file with placeholders:

```glob
# scrt.def - Safe to publish

# API Configuration
# @secret API_KEY: string - Your API key from provider.com
# @secret API_SECRET: string - Your API secret

# Database Configuration
# @secret DB_HOST: string - Database hostname
# @secret DB_USER: string - Database username
# @secret DB_PASSWORD: string - Database password

# Email Configuration
# @secret SMTP_PASSWORD: string - SMTP server password
```

**Important:**

- ✅ Safe to commit to git
- ✅ Safe to publish
- ✅ Documents what secrets are needed
- ✅ Helps other developers set up the project

### How to Use Secrets

```glob
# main.mn

# Load secrets (automatically decrypted from project.scrt)
@imp secrets

# Use secrets in your code
async connect_to_api():
    client = http.Client(
        api_key=secrets.API_KEY,
        api_secret=secrets.API_SECRET
    )
    return client

async connect_to_database():
    db = database.connect(
        host=secrets.DB_HOST,
        user=secrets.DB_USER,
        password=secrets.DB_PASSWORD
    )
    return db

mn:
    api = await connect_to_api()
    db = await connect_to_database()
    print("Connected successfully!")
```

### Secret Management Rules

1. **Only `.scrt` and `secret.def` may contain real secrets**
2. **`scrt.def` is auto-generated with annotations only**
3. **Linter blocks secrets in `scrt.def`**
4. **Auto-redaction on publish**
5. **Auto-gitignore maintenance**

---

## Development Workflow

### Step-by-Step Development Process

#### 1. Create a New Project

```bash
# Create project directory
glob new my-project
cd my-project

# This creates:
# - main.mn (empty template)
# - package.toml (with defaults)
# - README.md (template)
```

#### 2. Write Your Code

Edit `main.mn`:

```glob
# main.mn

@@imp std.io

def GREETING = "Hello, GUL!"

fn greet(name):
    return GREETING + " " + name

mn:
    message = greet("World")
    print(message)
```

#### 3. Save and Auto-Organize

When you save `main.mn`, GUL automatically:

- Creates `imports.imp` with your imports
- Creates `definitions.def` with your constants
- Creates `functions.fnc` with your functions
- Creates cleaned `main.mn` with just the main function

#### 4. Run Your Code

```bash
glob run main.mn
```

Output:

```
Hello, GUL! World
```

#### 5. Test Your Code

```bash
glob test
```

#### 6. Build for Production

```bash
glob build --release
```

### Live Development

GUL watches your files and auto-recompiles:

```bash
glob watch main.mn
```

Now every time you save, GUL:

1. Re-organizes blocks
2. Runs linter
3. Runs tests
4. Shows results

---

## Publishing Packages

### Preparing for Publication

#### 1. Update `package.toml`

```toml
[package]
name = "my-awesome-package"
version = "1.0.0"
authors = ["Your Name <you@example.com>"]
description = "A clear description of what your package does"
license = "MIT"
keywords = ["web", "api", "http"]
categories = ["web-programming", "network-programming"]
```

#### 2. Write Documentation

Create a good `README.md`:

```markdown
# My Awesome Package

Brief description of what this package does.

## Installation

\`\`\`bash
glob add my-awesome-package
\`\`\`

## Usage

\`\`\`glob
@imp my-awesome-package

mn:
result = my-awesome-package.do_something()
print(result)
\`\`\`

## Examples

See the `examples/` directory for more examples.

## License

MIT
```

#### 3. Check for Secrets

```bash
glob check-secrets
```

This ensures no secrets are in publishable files.

#### 4. Run Tests

```bash
glob test --all
```

#### 5. Publish

```bash
glob publish
```

### What Gets Published?

✅ **Included:**

- `imports.imp`
- `definitions.def`
- `async.asy`
- `functions.fnc`
- `custom.cs`
- `main.mn` (cleaned)
- `scrt.def` (annotations only)
- `package.toml`
- `README.md`
- `LICENSE`

❌ **Excluded:**

- `project.scrt` (secrets)
- `secret.def` (runtime secrets)
- `.git/` directory
- `target/` build directory
- Any files in `.gitignore`

---

## Examples

### Example 1: Simple Web API

```
web-api-project/
├── main.mn
├── package.toml
└── project.scrt

# main.mn
@@imp std.http

def PORT = 8080

async handle_request(request):
    return {
        status: 200,
        body: "Hello from GUL!"
    }

mn:
    server = http.Server(PORT)
    server.on("request", handle_request)
    await server.start()
    print(f"Server running on port {PORT}")
```

After compilation:

```
web-api-project/
├── imports.imp          # @imp std.http
├── definitions.def      # def PORT = 8080
├── async.async           # async handle_request(...)
├── main.mn             # mn: ...
└── package.toml
```

### Example 2: Data Analysis Tool

```
data-analysis/
├── main.mn
├── package.toml
└── project.scrt

# main.mn
@@imp std.io
@imp python: pandas
@imp python: matplotlib

def DATA_FILE = "data.csv"

cs python:
    import pandas as pd
    import matplotlib.pyplot as plt

    fn analyze(filename):
        df = pd.read_csv(filename)
        return df.describe()

    fn plot(data):
        plt.plot(data)
        plt.savefig('output.png')

fn load_data():
    return file.read(DATA_FILE)

mn:
    data = load_data()
    stats = analyze(DATA_FILE)
    print(stats)
    plot(data)
```

After compilation:

```
data-analysis/
├── imports.imp          # All imports
├── definitions.def      # def DATA_FILE = "data.csv"
├── custom.cs           # Python code block
├── functions.fnc       # fn load_data()
├── main.mn             # mn: ...
└── package.toml
```

### Example 3: IoT Device Controller

```
iot-controller/
├── main.mn
├── package.toml
└── project.scrt

# main.mn
@imp embedded.gpio
@@imp std.time

def LED_PIN = 13
def BLINK_INTERVAL = 1000  # milliseconds

fn setup_pin(pin):
    gpio.set_mode(pin, "output")

fn blink_led(pin, interval):
    loop:
        gpio.high(pin)
        time.sleep(interval)
        gpio.low(pin)
        time.sleep(interval)

mn:
    setup_pin(LED_PIN)
    blink_led(LED_PIN, BLINK_INTERVAL)
```

---

## Quick Reference

### File Types Summary

| File                | Purpose            | You Edit? | Published? |
| ------------------- | ------------------ | --------- | ---------- |
| `main.mn` (source)  | Your code          | ✅ Yes    | ❌ No      |
| `imports.imp`       | Imports            | ❌ Auto   | ✅ Yes     |
| `definitions.def`   | Constants          | ❌ Auto   | ✅ Yes     |
| `async.asy`         | Async functions    | ❌ Auto   | ✅ Yes     |
| `functions.fnc`     | Sync functions     | ❌ Auto   | ✅ Yes     |
| `custom.cs`         | Foreign code       | ❌ Auto   | ✅ Yes     |
| `main.mn` (cleaned) | Entry point        | ❌ Auto   | ✅ Yes     |
| `scrt.def`          | Secret annotations | ❌ Auto   | ✅ Yes     |
| `project.scrt`      | Actual secrets     | ✅ Yes    | ❌ NEVER   |
| `secret.def`        | Runtime secrets    | ❌ Auto   | ❌ NEVER   |
| `package.toml`      | Metadata           | ✅ Yes    | ✅ Yes     |

### Commands Quick Reference

```bash
glob new <name>          # Create new project
glob run <file>          # Run a GUL file
glob build               # Build project
glob build --release     # Build optimized
glob test                # Run tests
glob watch <file>        # Watch and auto-compile
glob check-secrets       # Check for exposed secrets
glob publish             # Publish to registry
glob add <package>       # Add dependency
glob remove <package>    # Remove dependency
```

---

**Now you understand how GUL organizes your code! Start building amazing projects!** 🚀
