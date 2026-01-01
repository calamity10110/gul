# Chapter 2: Functions & Ownership

## Defining Functions

Functions are defined using the `fn` keyword.

```gul
fn greet(name: str):
    std.io.println("Hello, " + name)
```

Values are returned using `return`. If no return type is specified, it returns `void` (unit).

```gul
fn add(a: int, b: int) -> int:
    return a + b
```

## Async Functions

GUL has first-class support for concurrency via `asy` (async) and `await`.

```gul
asy fetch_url(url: str) -> str:
    let response = await http.get(url)
    return response.text()
```

## Ownership System

GUL enforces memory safety without a Garbage Collector using Ownership. This is similar to Rust but with simplified keywords.

### Keywords

1.  **`own` (Move)**: The function takes full ownership of the value. The caller cannot use it afterwards.

    ```gul
    fn consume(own ticket: Ticket):
        # ticket is destroyed at end of function
    ```

2.  **`ref` (Borrow)**: The function borrows the value (Read-only).

    ```gul
    fn print_ticket(ref ticket: Ticket):
        print(ticket.id)
    ```

3.  **`mut` (Mutable Borrow)**: The function borrows the value and can modify it.
    ```gul
    fn stamp(mut ticket: Ticket):
        ticket.stamped = true
    ```

### Example

```gul
let file = File.open("data.txt")

# Read from file (Borrow)
read_data(ref file)

# Close file (Move/Consume)
close_file(own file)

# file.read()  <-- Error! file has been moved.
```
