# Types

**Version**: 0.13.0 | **Syntax**: v3.2 | **Updated**: 2025-12-28

---

# Type System Reference

GUL features a rich, flexible type system that supports static typing, type inference, generics, and advanced type features. This reference covers all aspects of the GUL type system.

## 🎯 Overview

The GUL type system provides:

- **Static Typing**: Type safety at compile time
- **Type Inference**: Automatic type deduction
- **Generics**: Parametric polymorphism
- **Traits**: Interface-like contracts
- **Union Types**: Multiple possible types
- **Optional Types**: Null safety
- **Type Aliases**: Custom type names

## 📚 Primitive Types

### Integer Types

```gul
# Signed integers
# Signed integers
let i8 = @int8(-128)
let i16 = @int16(-32768)
let i32 = @int32(-2147483648)
let i64 = @int64(-9223372036854775808)
let val = @int(-42)          # Platform-dependent (i64)

# Unsigned integers
let u8 = @uint8(255)
let u16 = @uint16(65535)
let u32 = @uint32(4294967295)
let u64 = @uint64(18446744073709551615)
let uval = @uint(42)         # Platform-dependent
```

### Floating-Point Types

```gul
let f32 = @float32(3.14)
let f64 = @float64(2.718281828)
let f = @float(1.23)     # Defaults to float64
```

### Boolean Type

```gul
is_valid: bool = True
is_active: bool = False
```

### Character and String Types

```gul
let ch = @char('A')
let name = @str("GUL")
let multiline = @str("""
    Multi-line
    string
    """)
```

### Unit Type

```gul
# The unit type () represents no value
@fn do_something():
    print("Done")
    # Implicitly returns ()
```

## 📦 Compound Types

### Tuples

```gul
# Fixed-size ordered collections
let point = (10, 20)
let person = ("Alice", 30, true)

# Accessing tuple elements
x = point.0  # 10
y = point.1  # 20

# Tuple destructuring
(name, age, _) = person
```

### Lists (Vectors)

```gul
# Dynamic arrays
let numbers = @list[1, 2, 3, 4, 5]
let names = @list["Alice", "Bob", "Charlie"]

# Type inference
let auto_typed = @list[1, 2, 3]  # vec[int]

# Nested lists
let matrix = @list[
    @list[1, 2, 3],
    @list[4, 5, 6],
    @list[7, 8, 9]
]
```

### Maps (Dictionaries)

```gul
# Key-value pairs
scores: map[str, int] = map{
    "Alice": 95,
    "Bob": 87,
    "Charlie": 92
}

# Type inference
config = map{
    "host": "localhost",
    "port": 8080
}  # map[str, Any]
```

### Sets

```gul
# Unique collections
unique_numbers: set[int] = set{1, 2, 3, 4, 5}
tags: set[str] = set{"gul", "programming", "language"}
```

## 🏗️ Structs

### Basic Structs

```gul
struct Point:
    x: int
    y: int

struct Person:
    name: str
    age: int
    email: str

# Creating instances
let p = Point { x: 10, y: 20 }
let person = Person {
    name: "Alice",
    age: 30,
    email: "alice@example.com"
}
```

### Tuple Structs

```gul
struct Color(int, int, int)

red = Color(255, 0, 0)
green = Color(0, 255, 0)

# Access fields by index
r = red.0  # 255
```

### Unit Structs

```gul
struct Marker

# Used for type-level programming
marker = Marker
```

### Generic Structs

```gul
struct Container[T]:
    value: T

    @fn new(val: T) -> Container[T]:
        return Container { value: val }

    @fn get(self) -> T:
        return self.value

# Usage
let int_container = Container[int].new(42)
let str_container = Container[str].new("hello")
```

## 🔀 Enums

### Basic Enums

```gul
enum Status:
    Pending
    Active
    Completed
    Failed

current_status: Status = Status.Active

match current_status:
    Status.Pending: print("Waiting...")
    Status.Active: print("Running")
    Status.Completed: print("Done")
    Status.Failed: print("Error")
```

### Enums with Data

```gul
enum Result[T, E]:
    Ok(T)
    Err(E)

enum Option[T]:
    Some(T)
    None

# Using Option
# Using Option
@fn find_user(id: int) -> Option[User]:
    if user_exists(id):
        return Option.Some(get_user(id))
    else:
        return Option.None

# Using Result
@fn divide(a: float, b: float) -> Result[float, str]:
    if b == 0:
        return Result.Err("Division by zero")
    else:
        return Result.Ok(a / b)
```

### Complex Enums

```gul
enum Message:
    Quit
    Move { x: int, y: int }
    Write(str)
    ChangeColor(int, int, int)

fn handle_message(msg: Message):
    match msg:
        Message.Quit:
            print("Quitting...")
        Message.Move { x, y }:
            print(f"Moving to ({x}, {y})")
        Message.Write(text):
            print(f"Writing: {text}")
        Message.ChangeColor(r, g, b):
            print(f"Changing color to RGB({r}, {g}, {b})")
```

## 🔤 Type Aliases

```gul
# Simple aliases
type UserId = int
type Email = str
type JsonData = map[str, Any]

# Generic aliases
type Result[T] = Result[T, str]
type HashMap[K, V] = map[K, V]

# Function type aliases
type Handler = fn(Request): Response
type Callback[T] = fn(T): void
```

## ❓ Optional Types

```gul
# Optional values with ?
name: str? = None
age: int? = Some(30)

# Safe unwrapping
if age is not None:
    print(f"Age is {age}")

# Default values
actual_age = age.unwrap_or(0)

# Chaining optional operations
result = get_user(42)?.get_email()?.validate()
```

## 🔗 Union Types

```gul
# Multiple possible types
fn process(value: int | str | bool):
    match type(value):
        int: print(f"Integer: {value}")
        str: print(f"String: {value}")
        bool: print(f"Boolean: {value}")

# Complex unions
type JsonValue = str | int | float | bool | vec[JsonValue] | map[str, JsonValue] | None
```

## 🎭 Generics

### Generic Functions

```gul
# Single type parameter
fn identity[T](x: T): T:
    return x

# Multiple type parameters
# Multiple type parameters
@fn pair[A, B](a: A, b: B) -> (A, B):
    return (a, b)

# Generic with constraints
fn compare[T: Comparable](a: T, b: T): bool:
    return a > b

# Usage
result1 = identity[int](42)
result2 = identity("hello")  # Type inference
pair_result = pair(10, "ten")
```

### Generic Structs

```gul
struct Pair[A, B]:
    first: A
    second: B

    fn new(a: A, b: B): Pair[A, B]:
        return Pair { first: a, second: b }

    fn swap(self): Pair[B, A]:
        return Pair[B, A].new(self.second, self.first)

# Usage
int_str = Pair[int, str].new(1, "one")
swapped = int_str.swap()  # Pair[str, int]
```

### Generic Enums

```gul
enum Tree[T]:
    Leaf(T)
    Node {
        value: T,
        left: Box[Tree[T]],
        right: Box[Tree[T]]
    }
```

## 🎯 Traits

### Defining Traits

```gul
trait Drawable:
    @fn draw(self)
    @fn area(self) -> float

trait Comparable:
    fn compare(self, other: Self): int

trait Display:
    fn to_string(self): str
```

### Implementing Traits

```gul
struct Circle:
    radius: float

impl Drawable for Circle:
    @fn draw(self):
        print(f"Drawing circle with radius {self.radius}")

    @fn area(self) -> float:
        return 3.14159 * self.radius ** 2

struct Rectangle:
    width: float
    height: float

impl Drawable for Rectangle:
    fn draw(self):
        print(f"Drawing {self.width}x{self.height} rectangle")

    fn area(self): float:
        return self.width * self.height
```

### Generic Traits

```gul
trait Container[T]:
    fn add(mut self, item: T)
    fn get(self, index: int): Option[T]
    fn size(self): int

struct Vec[T]:
    items: vec[T]

impl[T] Container[T] for Vec[T]:
    fn add(mut self, item: T):
        self.items.push(item)

    fn get(self, index: int): Option[T]:
        if index < len(self.items):
            return Some(self.items[index])
        else:
            return None

    fn size(self): int:
        return len(self.items)
```

### Trait Bounds

```gul
# Function with trait bounds
@fn print_all[T: Display](items: list[T]):
    for item in items:
        print(item.to_string())

# Multiple trait bounds
@fn process[T: Display + Comparable](value: T):
    print(value.to_string())
    # Can also call compare()

# Where clauses for complex bounds
@fn complex_function[T, U](a: T, b: U): void
where
    T: Display + Comparable,
    U: Container[T]:
    # Implementation
    pass
```

### Associated Types

```gul
trait Iterator:
    type Item

    fn next(mut self): Option[Self.Item]

struct VecIterator[T]:
    items: vec[T]
    index: int

impl[T] Iterator for VecIterator[T]:
    type Item = T

    fn next(mut self): Option[T]:
        if self.index < len(self.items):
            result = self.items[self.index]
            self.index += 1
            return Some(result)
        else:
            return None
```

## 🔄 Type Inference

### Automatic Inference

```gul
# Type inferred from value
x = 42                  # int
y = 3.14                # float
z = "hello"             # str
vec = vec[1, 2, 3]      # vec[int]

# Inference from usage
fn process(x):          # Type inferred from usage
    return x + 1
```

### Explicit Type Annotations

```gul
# Sometimes explicit types are needed
values: vec[int] = vec[]  # Empty vector needs type
result: Option[str] = None  # None needs type context
```

## 🔍 Type Checking

### Static Type Checking

```gul
fn add(a: int, b: int): int:
    return a + b

# Type error: cannot pass string to int parameter
# add("hello", 5)  # Compile error
```

### Dynamic Type Checking

```gul
import std.types

fn process(value: Any):
    if types.is_instance(value, int):
        print(f"Integer: {value}")
    elif types.is_instance(value, str):
        print(f"String: {value}")
    else:
        print("Unknown type")
```

### Type Guards

```gul
fn handle_value(value: int | str):
    if value is int:
        # value is type int here
        print(value + 1)
    else:
        # value is type str here
        print(value.upper())
```

## 🎨 Advanced Type Features

### Phantom Types

```gul
# Types that exist only at compile time
struct Validated[T]:
    value: T

struct Email
struct Url

fn validate_email(s: str): Validated[Email]:
    # Validation logic
    return Validated { value: s }

fn validate_url(s: str): Validated[Url]:
    # Validation logic
    return Validated { value: s }
```

### Higher-Kinded Types

```gul
# Types that abstract over type constructors
trait Functor[F[_]]:
    fn map[A, B](fa: F[A], f: fn(A): B): F[B]

impl Functor[Option]:
    fn map[A, B](opt: Option[A], f: fn(A): B): Option[B]:
        match opt:
            Some(a): return Some(f(a))
            None: return None
```

### Type-Level Programming

```gul
# Compile-time type computations
trait Add[A, B]:
    type Output

    fn add(a: A, b: B): Self.Output

impl Add[int, int]:
    type Output = int

    fn add(a: int, b: int): int:
        return a + b

impl Add[str, str]:
    type Output = str

    fn add(a: str, b: str): str:
        return a + b
```

## 🔐 Type Safety Features

### Newtype Pattern

```gul
# Wrap primitive types for type safety
struct Kilometers(float)
struct Miles(float)

fn convert(km: Kilometers): Miles:
    return Miles(km.0 * 0.621371)

# Type error: cannot pass Miles where Kilometers expected
distance = Kilometers(100)
# convert(Miles(50))  # Compile error
```

### Branded Types

```gul
struct UserId(int)
struct ProductId(int)

fn get_user(id: UserId): User:
    # Implementation
    pass

# Type safe - cannot mix up IDs
user_id = UserId(42)
# product_id = ProductId(42)
# get_user(product_id)  # Compile error
```

## 📊 Type Conversion

### Implicit Conversions

```gul
# Widening conversions (automatic)
i: i32 = 42
l: i64 = i  # i32 -> i64 (safe)

f32_val: f32 = 3.14
f64_val: f64 = f32_val  # f32 -> f64 (safe)
```

### Explicit Conversions

```gul
# Narrowing conversions (explicit)
l: i64 = 1000
i: i32 = l as i32  # Explicit cast

f: float = 3.14
int_val: int = f as int  # 3
```

### From/Into Traits

```gul
trait From[T]:
    fn from(value: T): Self

trait Into[T]:
    fn into(self): T

struct UserId(int)

impl From[int] for UserId:
    fn from(value: int): UserId:
        return UserId(value)

# Usage
id = UserId.from(42)
id2: UserId = 42.into()
```

## 📚 Best Practices

### 1. Use Type Inference When Obvious

```gul
# ✅ Good: obvious types
names = vec["Alice", "Bob"]
count = 42

# ❌ Bad: unnecessary verbosity
names: vec[str] = vec["Alice", "Bob"]
count: int = 42
```

### 2. Annotate Function Signatures

```gul
# ✅ Good: clear signatures
fn calculate_total(prices: vec[float], tax_rate: float): float:
    # Implementation

# ❌ Bad: unclear types
fn calculate_total(prices, tax_rate):
    # Implementation
```

### 3. Use Newtype for Domain Types

```gul
# ✅ Good: type-safe
struct Email(str)
struct Age(int)

# ❌ Bad: primitive obsession
fn create_user(email: str, age: int):
    pass
```

### 4. Prefer Option/Result Over Null

```gul
# ✅ Good: explicit error handling
fn find_user(id: int): Option[User]:
    # Implementation

# ❌ Bad: null confusion
fn find_user(id: int): User?:
    # Implementation
```

## 📖 See Also

- [Ownership Model](ownership.md)
- [Language Specification](specification.md)
- [Generics Tutorial](../tutorials/generics.md)

---

**Last Updated**: 2025-12-28  
**Version: 0.13.0  
**License**: MIT
