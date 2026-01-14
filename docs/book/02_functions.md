# Chapter 2: Functions & Ownership

## Defining Functions

Functions are defined using the `fn` keyword.

```gul
@fn greet(name: str):
    print("Hello, " + name)
```

Values are returned using `return`. If no return type is specified, it returns `void` (unit).

```gul
@fn add(a: int, b: int) -> int:
    return a + b
```

## Async Functions

GUL has first-class support for concurrency via `@async` and `await`.

```gul
@async fetch_url(url: str) -> str:
    let response = await http.get(url)
    return response.text()
```

## Ownership System

GUL enforces memory safety without a Garbage Collector using Ownership. This is similar to Rust but with simplified keywords.

### Keywords

1. **`borrow`**: The function borrows the value (Read-only).

    ```gul
    @fn print_ticket(ticket: borrow Ticket):
        print(ticket.id)
    ```

2. **`ref`**: The function takes a mutable reference/copy.

    ```gul
    @fn stamp(ticket: ref Ticket):
        ticket.stamped = true
    ```

3. **`move`**: The function takes full ownership. Caller loses access.

    ```gul
    @fn consume(ticket: move Ticket):
        # ticket is destroyed at end of function
    ```

4. **`kept`**: The function takes a copy (clone), original stays valid.

    ```gul
    @fn archive(ticket: kept Ticket):
        store(ticket)
    ```

### Example

```gul
let file = File.open("data.txt")

# Read from file (Borrow)
read_data(borrow file)

# Close file (Move/Consume)
close_file(move file)

# file.read()  <-- Error! file has been moved.
```

## Built-in Functions

GUL includes many global built-in functions that don't require imports:

- `print(...)`, `println(...)`: Output to console.
- `input(...)`: Get user input.
- `len(obj)`: Get length of lists, strings, or dicts.
- `autograd_begin()`, `make_var(v)`: Machine learning primitives.
