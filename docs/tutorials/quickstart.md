# Quick Start Tutorial

Get up and running with GUL in just 5 minutes!

## ⚡ Installation

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and build GUL
git clone https://github.com/gul-lang/gul.git
cd gul
cargo build --release
cargo install --path .
```

## 🎯 Your First Program

Create a file called `hello.mn`:

```gul
main:
    print("Hello, GUL!")
```

Run it:

```bash
gul run hello.mn
```

Output:

```
Hello, GUL!
```

## 🔢 Variables and Types

```gul
main:
    # Variables with type inference
    name = "Alice"
    age = 30
    height = 5.7
    is_active = True

    # Explicit types
    score: int = 100
    price: float = 19.99

    print(f"{name} is {age} years old")
```

## 🔄 Control Flow

```gul
main:
    # If statements
    age = 25
    if age >= 18:
        print("Adult")
    else:
        print("Minor")

    # For loops
    for i in range(5):
        print(i)

    # While loops
    count = 0
    while count < 3:
        print(count)
        count += 1
```

## 📦 Functions

```gul
fn greet(name: str):
    print(f"Hello, {name}!")

fn add(a: int, b: int): int:
    return a + b

main:
    greet("Alice")
    result = add(5, 3)
    print(f"5 + 3 = {result}")
```

## 📊 Data Structures

```gul
main:
    # Lists
    numbers = vec[1, 2, 3, 4, 5]
    numbers.push(6)
    print(numbers[0])  # 1

    # Maps
    person = map{
        "name": "Bob",
        "age": 30,
        "city": "NYC"
    }
    print(person["name"])  # Bob

    # Sets
    tags = set{"python", "rust", "gul"}
    tags.add("javascript")
```

## 🌐 Simple Web Server

Create `server.mn`:

```gul
import std.http

server = http.Server(port=8080)

@server.get("/")
fn home(request):
    return "Welcome to GUL!"

@server.get("/hello/{name}")
fn hello(request, name: str):
    return f"Hello, {name}!"

main:
    print("Server running on http://localhost:8080")
    server.start()
```

Run it:

```bash
gul run server.mn
```

Visit http://localhost:8080 in your browser!

## 📁 File Operations

```gul
import std.filesystem as fs

main:
    # Write to file
    fs.write_text("message.txt", "Hello from GUL!")

    # Read from file
    content = fs.read_text("message.txt")
    print(content)

    # List directory
    files = fs.list_files(".")
    for file in files:
        print(file)
```

## 🔐 Working with Secrets

```gul
import std.secrets

secret api_key = env("API_KEY", default="dev-key")

main:
    print(f"Using API key: {api_key}")
```

## 🚀 Next Steps

1. **Learn More**: Read the [First Program Guide](first-program.md)
2. **Build a Web App**: Follow the [Web Server Tutorial](web-server.md)
3. **Explore Examples**: Check out [`examples/`](../../examples/)
4. **Read the Docs**: Browse the [Language Reference](../reference/syntax.md)

## 🎓 Key Concepts

- **main** block: Entry point of your program
- **Type inference**: Often optional type annotations
- **Imports**: Use standard library with `import std.*`
- **Functions**: Define with `fn name(params): return_type`
- **Routes**: Web endpoints with `@server.get("/path")`

## 💡 Tips

- Use `gul --help` to see all commands
- Try `gul ide` to launch the interactive IDE
- Run `gul fmt file.mn` to format your code
- Use `gul check file.mn` to type-check without running

## 📚 Resources

- [Language Specification](../reference/specification.md)
- [Standard Library API](../api/standard-library.md)
- [Web Development Guide](../guides/web-development.md)
- [Examples Repository](../../examples/)

---

**Congratulations!** 🎉 You've completed the GUL Quick Start. Happy coding!

---

**Last Updated**: 2025-12-10  
**Version**: 1.0.0  
**License**: MIT
