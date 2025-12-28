# Ownership

**Version**: 0.13.0 | **Syntax**: v3.2 | **Updated**: 2025-12-28

---

# Ownership Model Deep Dive

GUL implements a sophisticated ownership model that ensures memory safety while providing flexibility for different programming patterns. This guide explores the ownership system in detail.

## 🎯 Overview

The GUL ownership model is inspired by Rust but adapted for a more dynamic, multi-paradigm language. It provides:

- **Memory Safety**: Prevents data races and memory leaks
- **Zero-Cost Abstractions**: No runtime overhead
- **Flexibility**: Supports multiple paradigms (functional, OOP, procedural)
- **Automatic Management**: Reduces cognitive load

## 📚 Core Concepts

### Ownership Rules

GUL follows three fundamental ownership rules:

1. Each value has a single owner
2. When the owner goes out of scope, the value is dropped
3. Values can be borrowed (referenced) but not owned by multiple parties simultaneously

```gul
# Ownership example
mn:
    x = "Hello"          # x owns the string
    y = x                # ownership transfers to y
    # print(x)           # Error: x no longer owns the value
    print(y)             # OK: y owns the value
```

### Ownership Transfer (Move Semantics)

```gul
fn take_ownership(s: str):
    print(s)             # s owns the string here
    # s is dropped when function ends

mn:
    text = "GUL"
    take_ownership(text) # ownership moves to function parameter
    # print(text)        # Error: text no longer owns the value
```

## 🔄 Borrowing

### Immutable Borrowing

```gul
fn calculate_length(s: &str): int:
    return len(s)        # Can read, but not modify

mn:
    text = "Hello, GUL!"
    length = calculate_length(&text)  # Borrow immutably
    print(f"'{text}' has {length} characters")  # text still valid
```

### Mutable Borrowing

```gul
fn append_text(s: &mut str, suffix: str):
    s += suffix          # Can modify borrowed value

mn:
    var text = "Hello"
    append_text(&var text, ", World!")
    print(text)          # Output: "Hello, World!"
```

### Borrowing Rules

```gul
mn:
    var x = vec[1, 2, 3]

    # Multiple immutable borrows allowed
    r1 = &x
    r2 = &x
    print(r1, r2)        # OK

    # Can't have mutable borrow while immutable borrows exist
    # r3 = &var x        # Error: already borrowed as immutable

    # Can have one mutable borrow
    {
        r_mut = &var x
        r_mut.push(4)
    }                    # r_mut goes out of scope

    # Now we can borrow again
    print(x)             # OK
```

## 📦 Value Categories

### Stack Values

Small, fixed-size values are stored on the stack:

```gul
# Stack-allocated values
x: int = 42
y: float = 3.14
z: bool = True
point = (10, 20)         # Tuple on stack
```

### Heap Values

Larger or dynamically-sized values use the heap:

```gul
# Heap-allocated values
vec = vec[1, 2, 3, 4, 5]
map = map{"key": "value"}
text = "A longer string that needs heap allocation"
```

### Copy Types

Some types implement `Copy` trait and can be copied instead of moved:

```gul
# Types that implement Copy
x: int = 42
y = x                    # x is copied, not moved
print(x, y)              # Both valid: Output: 42 42

# Copy types in GUL:
# - int, float, bool
# - char
# - Tuples of Copy types
```

### Clone

For non-Copy types, explicit cloning is required:

```gul
# Clone for types that don't implement Copy
original = vec[1, 2, 3]
copy = original.clone()  # Explicit deep copy
print(original)          # OK: original still owns its data
print(copy)              # OK: copy owns its own data
```

## 🔗 References and Lifetimes

### Reference Basics

```gul
fn longest(x: &str, y: &str): &str:
    if len(x) > len(y):
        return x
    else:
        return y

mn:
    s1 = "short"
    s2 = "much longer string"
    result = longest(&s1, &s2)
    print(f"Longest: {result}")
```

### Lifetime Annotations

For complex reference patterns, GUL uses lifetime annotations:

```gul
# Lifetime annotation syntax
fn longest<'a>(x: &'a str, y: &'a str): &'a str:
    if len(x) > len(y):
        return x
    else:
        return y

# The 'a lifetime means: the returned reference
# is valid as long as both input references are valid
```

### Lifetime in Structs

```gul
# Struct with lifetime annotation
struct TextAnalyzer<'a>:
    text: &'a str

    fn word_count(self): int:
        return len(self.text.split())

mn:
    content = "Hello world from GUL"
    analyzer = TextAnalyzer { text: &content }
    print(analyzer.word_count())
```

### Multiple Lifetimes

```gul
fn first_word<'a, 'b>(s1: &'a str, s2: &'b str): &'a str:
    return s1.split()[0]

# Different lifetimes for different parameters
```

### Lifetime Elision

GUL automatically infers lifetimes in simple cases:

```gul
# Explicit lifetimes
fn first<'a>(s: &'a str): &'a str:
    return s.split()[0]

# Implicit (lifetime elision)
fn first(s: &str): &str:
    return s.split()[0]
```

## 🏗️ Smart Pointers

### Box (Heap Allocation)

```gul
import std.box

# Allocate on heap
boxed = Box.new(vec[1, 2, 3, 4, 5])
print(*boxed)            # Dereference to access value
```

### Rc (Reference Counted)

For multiple ownership scenarios:

```gul
import std.rc

# Shared ownership with reference counting
data = vec[1, 2, 3]
owner1 = Rc.new(data)
owner2 = owner1.clone()  # Increment ref count
owner3 = owner1.clone()  # Increment ref count

print(Rc.strong_count(owner1))  # Output: 3
```

### Arc (Atomic Reference Counted)

Thread-safe reference counting:

```gul
import std.arc
import std.thread

# Shared data across threads
shared_data = Arc.new(vec[1, 2, 3, 4, 5])

fn process_data(data: Arc[vec[int]]):
    for item in *data:
        print(item)

# Clone Arc for each thread
thread1 = thread.spawn(|| process_data(shared_data.clone()))
thread2 = thread.spawn(|| process_data(shared_data.clone()))

thread1.join()
thread2.join()
```

### RefCell (Interior Mutability)

Allows mutation through immutable references:

```gul
import std.refcell

# Interior mutability pattern
counter = RefCell.new(0)

fn increment(r: &RefCell[int]):
    var value = r.borrow_mut()
    *value += 1

increment(&counter)
increment(&counter)
print(*counter.borrow())  # Output: 2
```

### Combining Smart Pointers

```gul
import std.rc
import std.refcell

# Rc + RefCell for shared mutable state
shared_state = Rc.new(RefCell.new(map{
    "count": 0,
    "total": 0
}))

# Multiple owners can mutate
fn update_stats(state: Rc[RefCell[map]]):
    var s = state.borrow_mut()
    s["count"] += 1
    s["total"] += 100

state1 = shared_state.clone()
state2 = shared_state.clone()

update_stats(state1)
update_stats(state2)

print(*shared_state.borrow())  # {"count": 2, "total": 200}
```

## 🔒 Thread Safety

### Send and Sync Traits

```gul
# Send: Can transfer ownership between threads
# Sync: Can share references between threads

# Types implementing Send can be moved to threads
fn spawn<T: Send>(f: fn(): T):
    thread.spawn(f)

# Types implementing Sync can be accessed from multiple threads
fn share<T: Sync>(value: &T):
    # Can safely share reference
    pass
```

### Mutex for Shared Mutable State

```gul
import std.sync.mutex

# Thread-safe mutable state
counter = Mutex.new(0)

fn increment_counter(m: &Mutex[int]):
    var guard = m.lock()
    *guard += 1
    # Lock automatically released when guard goes out of scope

# Use across threads
threads = []
for i in range(10):
    t = thread.spawn(|| increment_counter(&counter))
    threads.push(t)

for t in threads:
    t.join()

print(*counter.lock())  # Output: 10
```

### RwLock (Read-Write Lock)

```gul
import std.sync.rwlock

# Multiple readers, single writer
data = RwLock.new(map{"key": "value"})

# Multiple reads simultaneously
fn read_data(lock: &RwLock[map]):
    guard = lock.read()
    print(guard["key"])

# Single write
fn write_data(lock: &RwLock[map]):
    var guard = lock.write()
    guard["key"] = "new value"
```

## 🌊 Drop and RAII

### Automatic Resource Management

```gul
struct FileHandler:
    path: str
    handle: File

    fn new(path: str): FileHandler:
        return FileHandler {
            path: path,
            handle: File.open(path)
        }

    # Drop automatically called when out of scope
    fn drop(self):
        print(f"Closing file: {self.path}")
        self.handle.close()

mn:
    {
        file = FileHandler.new("data.txt")
        # Use file...
    }  # file.drop() called here automatically
```

### Manual Drop

```gul
import std.mem

mn:
    resource = allocate_resource()

    # Use resource...

    # Explicitly drop before end of scope
    mem.drop(resource)

    # resource is now invalid
}
```

## 💡 Advanced Patterns

### Builder Pattern with Ownership

```gul
struct QueryBuilder:
    table: str
    conditions: vec[str]
    order_by: str?

    fn new(table: str): QueryBuilder:
        return QueryBuilder {
            table: table,
            conditions: vec[],
            order_by: None
        }

    fn where(mut self, condition: str): QueryBuilder:
        self.conditions.push(condition)
        return self

    fn order(mut self, column: str): QueryBuilder:
        self.order_by = Some(column)
        return self

    fn build(self): str:
        var query = f"SELECT * FROM {self.table}"

        if len(self.conditions) > 0:
            query += " WHERE " + " AND ".join(self.conditions)

        if self.order_by is not None:
            query += f" ORDER BY {self.order_by}"

        return query

# Usage with method chaining
query = QueryBuilder.new("users")
    .where("age > 18")
    .where("active = true")
    .order("created_at")
    .build()
```

### Option and Result with Ownership

```gul
# Option type
fn find_user(id: int): Option[User]:
    if user_exists(id):
        return Some(get_user(id))
    else:
        return None

# Pattern matching with ownership
match find_user(42):
    Some(user):
        process_user(user)  # user owned here
    None:
        print("User not found")

# Result type
fn parse_config(path: str): Result[Config, Error]:
    try:
        content = File.read(path)
        config = parse(content)
        return Ok(config)
    catch e:
        return Err(e)

# Handle Result with ownership
match parse_config("config.json"):
    Ok(config):
        use_config(config)  # config owned here
    Err(error):
        handle_error(error)
```

### Custom Drop Implementation

```gul
import std.mem

struct DatabaseConnection:
    url: str
    pool: ConnectionPool

    fn drop(self):
        # Custom cleanup logic
        print("Closing database connections")
        self.pool.close_all()
        self.pool.drain()
```

## 🔍 Debugging Ownership

### Ownership Diagnostics

```gul
import std.mem

fn debug_ownership():
    x = vec[1, 2, 3]

    # Check if value is moved
    print(mem.is_owned(x))  # true

    y = x
    # print(mem.is_owned(x))  # Error: x moved
    print(mem.is_owned(y))  # true

    # Get memory address
    print(mem.address_of(y))
```

### Borrow Checker Messages

GUL provides helpful error messages:

```gul
mn:
    x = "hello"
    y = x
    print(x)
}

# Error message:
# Error: value used after move
#   --> main.mn:3:11
#   |
# 2 |     y = x
#   |         - value moved here
# 3 |     print(x)
#   |           ^ value used after move
#   |
# help: consider using a reference instead: &x
```

## 📚 Best Practices

### 1. Prefer Borrowing

```gul
# ❌ Bad: Unnecessary ownership transfer
fn process(data: vec[int]):
    for item in data:
        print(item)

# ✅ Good: Borrow instead
fn process(data: &vec[int]):
    for item in data:
        print(item)
```

### 2. Use Clone Sparingly

```gul
# ❌ Bad: Excessive cloning
fn expensive_clone(data: vec[LargeStruct]):
    copy1 = data.clone()
    copy2 = data.clone()
    copy3 = data.clone()

# ✅ Good: Use references
fn efficient_borrows(data: &vec[LargeStruct]):
    process_ref1(data)
    process_ref2(data)
    process_ref3(data)
```

### 3. Explicit Lifetimes When Needed

```gul
# ✅ Good: Clear lifetime relationships
struct Parser<'a>:
    input: &'a str
    position: int

    fn parse(self): Result[Token<'a>, Error]:
        # Token borrows from input
        ...
```

### 4. Smart Pointers for Shared Ownership

```gul
# ✅ Good: Use Rc for shared ownership
import std.rc

shared_data = Rc.new(expensive_resource())
consumer1 = Consumer.new(shared_data.clone())
consumer2 = Consumer.new(shared_data.clone())
```

## 📖 See Also

- [Type System Reference](types.md)
- [Language Specification](specification.md)
- [Memory Management](../api/memory.md)

---

**Last Updated**: 2025-12-28  
**Version: 0.13.0  
**License**: MIT
