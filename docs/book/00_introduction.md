# Chapter 0: Introduction & Installation

## What is GUL?

**GUL** (General Utility Language) is a modern systems programming language that feels like Python but runs like Rust. It is designed for the AI era, with first-class support for machine learning, strict ownership semantics for safety, and an easy-to-read syntax.

## Key Features

- **Zero Cost Abstractions**: Like C++ and Rust.
- **Memory Safety**: Ownership system without Garbage Collection.
- **AI Native**: Built-in keywords for AI models.
- **Developer Experience**: Great error messages and tooling.

## Installation

### From Source

1. Clone the repository:

   ```bash
   git clone https://github.com/gul-lang/gul.git
   cd gul
   ```

2. Build the compiler:

   ```bash
   cargo build --release
   ```

3. Add to PATH:

   ```bash
   export PATH=$PATH:$(pwd)/target/release
   ```

## Hello World

Create a file named `hello.mn`:

```gul
mn:
    std.io.println("Hello, World!")
```

Run it:

```bash
gul hello.mn
```
