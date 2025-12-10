# GUL Language Specification v2.0

Version: 0.13.0  
Date: 2025-12-10  
Status: Production Ready

---

## Table of Contents

### Implemented Sections

1. [Language Overview](#1-language-overview)
2. [Language Structure](#2-language-structure)
3. [Lexical Structure](#3-lexical-structure)
4. [Types](#4-types)

### Future Sections (Planned)

The following sections are planned for future releases and will be based on the comprehensive [knowledgebase](knowledgebase.md):

- Ownership Model
- Variables
- Expressions
- Statements
- Definition Files (.def)
- Lists, Tuples & 4-Dimensional Data
- Classes & Objects
- Functions (.fnc)
- Control Flow
- Async Model
- Pattern System
- Error System
- Modules & Imports
- Foreign Language Integration
- UI Language
- Scientific & Engineering Systems
- Security System
- Standard Library
- Toolchain
- Compiler Architecture
- Formal Grammar

---

This specification is based on the comprehensive GUL knowledgebase and defines the complete language syntax, semantics, and behavior. For implementation details, see the [Compiler Guide](../guides/compiler.md).

For a quick reference, see [SYNTAX.md](syntax.md).

---

## 1. Language Overview

### 1.1 Purpose of GUL

GUL aims to be a "universal" language, suitable for:

- Systems programming
- Web development
- Data science
- Scientific computing
- Embedded systems
- IoT applications

### 1.2 Design Philosophy

**Core Principles:**

- **Readability First**: Syntax should be obvious and clean
- **Safety by Default**: Memory safety without GC pause times
- **Interoperability**: Seamless integration with existing ecosystems (Python, Rust, C, JavaScript)
- **Performance**: Native compilation for maximum speed
- **Simplicity**: Python-like syntax with Rust-like safety

### 1.3 Key Philosophies

- **Simplicity:** Indentation-based, readable syntax
- **Performance:** Native compilation via Rust-based toolchain
- **Safety:** Strong ownership model (`own`, `ref`, `copy`) without garbage collection
- **Autonomy:** Built-in AI and self-organization features
- **Universality:** Runs on Desktop, Web (WASM), and Embedded devices

### 1.4 Compilation Pipeline Summary

```text
Source → Lexer → Parser → AST → Semantic Analysis → IR → CodeGen (LLVM/Rust/WASM)
```

### 1.5 Execution Model

Compiled to native machine code or WebAssembly. No VM required for pure GUL code.

### 1.6 Supported Paradigms

- **Imperative**: Procedural programming with clear control flow
- **Functional**: Lisp-style list operations, first-class functions
- **Object-Oriented**: Structs/Classes with methods
- **Async/Concurrent**: Native async/await support

### 1.7 Static vs Dynamic Features

- Statically typed with type inference
- Dynamic features available via `any` type or foreign blocks
- Compile-time type checking with optional runtime type information

### 1.8 Use Cases

- High-performance web servers
- Scientific simulations (native units)
- Embedded systems (no GC, deterministic memory)
- Data analysis and machine learning
- System utilities and CLI tools

### 1.9 Integration Ecosystem

Built-in support for embedding:

- Rust (via FFI)
- Python (via PyO3)
- JavaScript (via QuickJS/V8)
- C (via libloading)
- SQL (embedded queries)

### 1.10 Toolchain Overview

- **`gul`**: The primary CLI tool (compiler, runner, package manager)
- **`gul build`**: Compile projects
- **`gul run`**: Execute GUL programs
- **`gul test`**: Run test suites
- **`gul fmt`**: Format code
- **`gul lint`**: Lint and check code quality

---

## 2. Language Structure

### 2.1 File Types (v2.0)

GUL uses distinct file types for different purposes:

- **`.gul`**: General purpose files (can contain any valid GUL code)
- **`.def`**: Definition files - Contains `const`, `mut`, `struct`, `@global`, `import`. No control flow
- **`.fnc`**: Function files - Contains `fn`, `async` functions. Pure logic
- **`.mn`**: Main files - Contains `main():` entry point and orchestrates execution
- **`.scrt`**: Secret files - Encrypted storage for API keys/credentials
- **`.cs`**: Cross-script files - Foreign code blocks (embedded C, Rust, Python, etc.)

### 2.2 Module System

Modules map to file paths:

- `import std.math` imports `std/math.gul` or the `std/math` package
- Folders are packages
- Each package can have a `package.toml` manifest

### 2.3 Naming Rules

- **Variables/Functions**: `snake_case` (e.g., `my_variable`, `calculate_sum`)
- **Types/Structs**: `PascalCase` (e.g., `MyStruct`, `UserData`)
- **Constants**: `SCREAMING_SNAKE_CASE` (e.g., `MAX_RETRY`, `API_VERSION`)
- **Private**: Prefix with `_` (e.g., `_internal_function`)

### 2.4 Unicode Support

- Source code must be UTF-8 encoded
- Identifiers can contain Unicode characters
- String literals support Unicode
- Comments support Unicode

### 2.5 Code Blocks & Indentation

GUL uses significant indentation (4 spaces recommended). Blocks are denoted by `:`.

```gul
if x > 0:
    print("Positive")
    process(x)
```

### 2.6 Comments

- **Single line**: `#` comment
- **Multi-line**: `#[ ... ]#` block comment

```gul
# This is a single-line comment

#[
This is a
multi-line comment
]#
```

### 2.7 Documentation Blocks

Triple-quoted strings `""" ... """` at the start of functions/modules act as docstrings:

```gul
fn calculate_area(width: float, height: float) -> float:
    """
    Calculate the area of a rectangle.

    Args:
        width: The width of the rectangle
        height: The height of the rectangle

    Returns:
        The area (width * height)
    """
    return width * height
```

### 2.8 Metadata Headers

Files can start with metadata comments:

```gul
# @version: 1.0.0
# @author: GUL Team
# @license: MIT
```

---

## 3. Lexical Structure

### 3.1 Character Set

UTF-8 encoding is required for all source files.

### 3.2 Tokens

The lexer produces the following token types:

- Keywords
- Identifiers
- Literals (String, Number, Boolean)
- Operators
- Delimiters
- Special (UI components, units)

### 3.3 Identifiers

- Start with letter or `_`
- Followed by letters, numbers, or `_`
- Case-sensitive
- Unicode characters allowed

### 3.4 Keywords (v2.0)

**Primary Keywords:**

- `import`, `const`, `mut`, `fn`, `async`, `extern`, `main`, `struct`
- `global`, `static`, `local`

**Legacy Keywords (deprecated but supported):**

- `imp`, `def`, `asy`, `cs`, `mn`

**Common Keywords:**

- `own`, `ref`, `copy`
- `await`, `loop`, `if`, `elif`, `else`, `for`, `while`
- `return`, `break`, `continue`
- `try`, `catch`, `finally`
- `match`, `case`

### 3.5 Literals

**String Literals:**

```gul
"hello"
'world'
"""multi-line
string"""
f"formatted {variable}"
```

**Numeric Literals:**

```gul
123          # Integer
12.34        # Float
0xFF         # Hexadecimal
0b1010       # Binary
0o755        # Octal
```

**Unit Literals:**

```gul
10 kg        # Mass
9.8 m/s^2    # Acceleration
100 N        # Force
```

**Collection Literals:**

```gul
(1, 2)              # Tuple
[1, 2, 3]           # List
{key: value}        # Dictionary
```

### 3.6 Operators

**Arithmetic:**

- `+`, `-`, `*`, `/`, `%`, `^` (power)

**Comparison:**

- `==`, `!=`, `<`, `>`, `<=`, `>=`

**Logical:**

- `and`, `or`, `not`

**Bitwise:**

- `&`, `|`, `~`, `<<`, `>>`

**Assignment:**

- `=`, `+=`, `-=`, `*=`, `/=`

### 3.7 Delimiters

- `(`, `)` - Parentheses
- `[`, `]` - Brackets
- `{`, `}` - Braces
- `,` - Comma
- `:` - Colon
- `.` - Dot
- `->` - Arrow (return type)

### 3.8 Escape Rules

Standard C-style escapes:

- `\n` - Newline
- `\t` - Tab
- `\"` - Double quote
- `\'` - Single quote
- `\\` - Backslash
- `\r` - Carriage return
- `\0` - Null character

---

## 4. Types

### 4.1 Primitive Types

- **`int`**: 64-bit signed integer
- **`float`**: 64-bit IEEE 754 floating-point
- **`bool`**: Boolean (`true`, `false`)
- **`str`**: UTF-8 string
- **`char`**: Unicode scalar value

### 4.2 Compound Types

- **Structs**: User-defined record types
- **Enums**: Tagged unions (planned for v2.1)

### 4.3 Tuple Types

Fixed-size, mixed-type collections:

```gul
def point: (int, int) = (10, 20)
def user: (str, int, bool) = ("Alice", 25, true)
```

### 4.4 List Types (4-D)

Dynamic arrays with support for up to 4 dimensions:

```gul
def numbers: [int] = [1, 2, 3]
def matrix: [[int]] = [[1, 2], [3, 4]]
def volume: [[[int]]] = [[[1, 2], [3, 4]], [[5, 6], [7, 8]]]
def spacetime: [[[[int]]]] = ...  # 4D tensor
```

Native support for scientific data: `x`, `y`, `z`, `t` (time) dimensions.

### 4.5 Dictionary Types

Hash maps with key-value pairs:

```gul
def config: {str: int} = {"port": 8080, "timeout": 30}
def data: {str: any} = {"name": "Alice", "age": 25}
```

### 4.6 Range Types

```gul
0..10       # Exclusive: 0 to 9
0..=10      # Inclusive: 0 to 10
```

### 4.7 Unit Types (Scientific Units)

Types can include physical dimensions:

```gul
def speed: float<m/s> = 10 m/s
def force: float<N> = 100 N
```

Arithmetic operations enforce dimensional consistency.

### 4.8 Time-Aware Types

Variables can track history if enabled (planned):

```gul
@history def position = 0 m
# Automatically tracks position over time
```

### 4.9 Optional Types

```gul
def maybe_value: int? = None
def user: Option<User> = Some(user_data)
```

### 4.10 Type Inference

GUL infers types where possible:

```gul
const x = 10             # Inferred as int
const y = 10 m           # Inferred as float<m>
const name = "Alice"     # Inferred as str
```

Explicit types can override:

```gul
const x: float = 10      # Explicitly float
```

---

## 5. Ownership Model

### 5.1 Ownership Overview

GUL uses a move-by-default system inspired by Rust. Variables are transferred unless specified otherwise, providing memory safety without garbage collection.

### 5.2 `own` Semantics

```gul
const own x = value  # Explicitly claims ownership

fn take(own x):
    # Function takes full ownership of x
    # Caller cannot use x after this call
    process(x)
```

Usage of `x` in caller after transfer is a compile error.

### 5.3 `ref` Semantics

```gul
fn peek(ref x):
    # Borrows x immutably
    print(x)
    # x is still owned by caller

fn mut_peek(ref mut x):
    # Borrows x mutably (planned)
    x.value = x.value + 1
```

### 5.4 `copy` Semantics

```gul
const y = copy x  # Creates a deep clone of x
```

Primitive types (`int`, `bool`, `float`) are `copy` by default (implicit copy on assignment).

### 5.5 Borrow Checker Rules

- **One mutable reference** OR **multiple immutable references**
- References cannot outlive the owner
- Compile-time enforcement prevents data races

### 5.6 Move Semantics

Assignment transfers ownership for non-copy types:

```gul
const list1 = [1, 2, 3]
const list2 = list1  # list1 is now invalid (moved)
# print(list1)  # ERROR: value moved
```

### 5.7 Lifetimes

Lifetimes are currently inferred. Explicit lifetime syntax (e.g., `'a`) is reserved for future advanced usage.

### 5.8 Safe & Unsafe Blocks

Normal GUL code is safe. `unsafe` blocks (planned) will allow manual pointer manipulation for systems programming.

### 5.9 Ownership in Collections

Collections own their elements. Moving a collection moves all elements.

### 5.10 Ownership + Async Interactions

Async tasks must own their data or act on `ref` strictly scoped execution contexts.

---

## 6. Variables

### 6.1 Declarations

```gul
const name = value           # Standard declaration. Inferred type
const name: type = value     # Explicit type
```

### 6.2 Mutability

Variables are immutable by default:

```gul
const x = 10         # Immutable
mut x = 10           # Mutable local variable
x = 20               # Assignment to mutable variable
```

### 6.3 Shadowing

Variables can be shadowed in inner scopes:

```gul
const x = 10
if true:
    const x = 20  # Shadows outer x
    print(x)      # Prints 20
print(x)          # Prints 10
```

### 6.4 Global Variables

Declared with `@global` annotation:

```gul
@global mut config = {}
```

### 6.5 Static Variables

`@static` annotation keeps variable alive across function calls:

```gul
fn counter():
    @static mut count = 0
    count = count + 1
    return count
```

### 6.6 Lazy Variables

```gul
lazy const x = expensive_computation()  # Evaluated on first access (planned)
```

### 6.7 Compile-Time Constants

```gul
const CONSTANT_NAME = value  # Treated as constant if uppercase
```

### 6.8 Variable Units

```gul
const distance = 100 m  # Variable holds both value 100 and unit m
```

### 6.9 Time-Dimension Variables

Variables in 4-D lists can automatically track values over `t` (time axis).

### 6.10 Scoped Variables

`@local`: Explicitly local scope (redundant in `.fnc` but clear).

---

## 7. Expressions

### 7.1 Expression Grammar

```text
expr ::= primary | unary | binary | call | await
```

### 7.2 Precedence Rules

Standard PEMDAS. `^` (power) is highest arithmetic. `and`/`or` are lower than comparison.

### 7.3 Arithmetic Expressions

```gul
1 + 2
x * y
10 m / 2 s  # Results in 5 m/s (unit arithmetic)
```

### 7.4 Logical Expressions

```gul
true and false
not valid
a == b
```

### 7.5 Unit Expressions

```gul
x + 5 kg  # Error if x is not mass unit
```

### 7.6 Range Expressions

```gul
0..10   # Generates sequence 0 to 9
0..=10  # Inclusive: 0 to 10
```

Used in `for` loops.

### 7.7 List/Tuple Expressions

```gul
[1, 2, 3]           # List
(1, "a")            # Tuple
{key: "value"}      # Dictionary
```

### 7.8 Time Expressions

`t` is a reserved variable in time-series contexts representing current time step.

### 7.9 Generator Expressions

```gul
(x * 2 for x in list)  # Lazy generator (planned)
```

### 7.10 Pattern-Based Expressions

```gul
match x:
    1: "one"
    _: "other"
```

---

## 8. Statements

### 8.1 Expression Statements

Statements that evaluate to a value but return is ignored:

```gul
calculate()  # Return value ignored
```

### 8.2 Variable Bindings

```gul
const x = 10
mut y = 20
```

### 8.3 Assignment

```gul
name = value  # Target must be mutable
```

### 8.4 Ownership Transfer Statements

```gul
return own x  # Explicit transfer
```

### 8.5 If Blocks

```gul
if condition:
    # ...
elif other:
    # ...
else:
    # ...
```

### 8.6 Then and Else Semantics

`else` block executes if condition is false.

### 8.7 While

```gul
while condition:
    # ...
```

### 8.8 For

```gul
for item in collection:
    print(item)

for i in 0..10:
    print(i)
```

### 8.9 Match

```gul
match value:
    1: print("One")
    2: print("Two")
    _: print("Other")
```

### 8.10 Try/Catch/Finally

```gul
try:
    dangerous_operation()
catch e:
    handle_error(e)
finally:
    cleanup()
```

---

## 9. Definition Files (.def)

### 9.1 Overview

`.def` files define the structure of a program or module. They cannot contain executable logic blocks.

### 9.2 Imports

```gul
import std{io, math}
```

### 9.3 Type Definitions

```gul
struct Point:
    x: float
    y: float
```

### 9.4 List & Tuple Definitions

```gul
const DATA = [1, 2, 3]
```

### 9.5 Class Definitions

```gul
struct User:
    name: str
    age: int

    fn greet(self):
        return "Hello, " + self.name
```

### 9.6 Initialization Order

Definitions are initialized in order of appearance.

### 9.7 Default Modules

`std` core types are implicitly imported.

### 9.8 Compile-Time Definitions

Macros or constants evaluated during compilation.

### 9.9 Multi-File Projects

`.def` files can reference other `.def` files via imports.

### 9.10 Namespace Rules

Definitions in `module.def` are accessed via `module.Name`.

---

## 10. Lists, Tuples & 4-Dimensional Data

### 10.1 Basic Lists

```gul
const numbers: [int] = [1, 2, 3, 4, 5]
const mixed = [1, "a"]  # Typed as [any]
```

### 10.2 Basic Tuples

```gul
const point = (10, 20)
const user = ("Alice", 25, true)
```

### 10.3 List/Tuple Functions (Lisp-Style)

```gul
car(list)         # Head
cdr(list)         # Tail
cons(item, list)  # Prepend
map(fn, list)     # Apply function
fold(fn, init, list)  # Reduce
```

### 10.4 2D Lists

```gul
const matrix = [[1, 2], [3, 4]]
```

Matrix operations supported via `std.math`.

### 10.5 3D Lists

Volumetric data:

```gul
const volume = [[[1, 2], [3, 4]], [[5, 6], [7, 8]]]
```

### 10.6 4D Lists (x-y-z-time)

Specialized for physics simulations:

```gul
const spacetime = [[[[1, 2], [3, 4]], [[5, 6], [7, 8]]]]
# Access via data[t][z][y][x]
```

### 10.7 Mutable vs Static Lists

```gul
const own list = [...]  # Mutable (growable)
const list = [...]      # Immutable view by default
```

### 10.8 Predictive Timelines

Using ML to predict `t+1` in 4D lists (integrated with `std.ml`).

### 10.9 Memory Rules for 4-D Lists

Large 4D lists use sparse storage or disk-backed tensors.

### 10.10 Interpolation & Projection

```gul
interpolate(list, t=1.5)  # Built-in interpolation
```

---

## 11. Classes & Objects

### 11.1 Class Syntax

GUL v2 uses `struct` for data and method definitions:

```gul
struct User:
    name: str
    age: int

    fn new(name: str, age: int) -> User:
        return User{name: name, age: age}

    fn grow_older(self):
        self.age = self.age + 1
```

### 11.2 Fields

Defined in `struct` block. All fields are typed.

### 11.3 Methods

Defined within struct body:

```gul
struct Calculator:
    value: float

    fn add(self, x: float):
        self.value = self.value + x
```

### 11.4 Constructors

Static method `new` is conventional:

```gul
const user = User.new("Bob", 30)
# Or struct literal:
const user = User{name: "Bob", age: 30}
```

### 11.5 Destructors

`drop` method (planned) for cleanup.

### 11.6 Inheritance

Composition over inheritance. Use trait system for shared behavior.

### 11.7 Mixins

Via traits (planned).

### 11.8 Traits

Define shared behavior:

```gul
trait Drawable:
    fn draw(self)

impl Drawable for Circle:
    fn draw(self):
        # Implementation
```

### 11.9 Async Classes

Structs can have async methods:

```gul
struct ApiClient:
    base_url: str

    async fn fetch(self, endpoint: str):
        return await http.get(self.base_url + endpoint)
```

### 11.10 Ownership of Fields

Structs own their fields. Accessing a non-copy field moves it unless borrowed.

---

## 12. Functions (.fnc)

### 12.1 Function Block Structure

```gul
fn name(params) -> return_type:
    # Function body
    return value
```

### 12.2 Async Functions

```gul
async fn name():
    # Async function body
    result = await some_async_operation()
    return result
```

### 12.3 Non-Async Functions

Standard `fn`. Execution blocks until return.

### 12.4 Return Semantics

Explicit `return value`. Implicit return of last expression (planned).

### 12.5 Argument Ownership

```gul
fn foo(own x):  # Moves x
    # Use x here

fn bar(ref x):  # Borrows x
    # Read x here
```

### 12.6 Variadic Functions

```gul
fn log(*args):  # Planned for v2.1
    for arg in args:
        print(arg)
```

### 12.7 Unit-Aware Functions

```gul
fn speed(d: float<m>, t: float<s>) -> float<m/s>:
    return d / t
```

Signatures enforce units.

### 12.8 Time-Dimensional Functions

Functions can operate on `t` axis of 4D inputs naturally.

### 12.9 Multi-Dispatch

Overloading based on types (planned).

### 12.10 Function Overloading

Multiple definitions with different signatures allowed via traits.

---

## 13. Control Flow

### 13.1 Conditions

```gul
if condition:
    # ...
elif other:
    # ...
else:
    # ...
```

### 13.2 Pattern Matching

```gul
match value:
    1: "one"
    2: "two"
    _: "other"
```

Exhaustive checking required.

### 13.3 Comprehensions

```gul
[x * 2 for x in list]  # Planned
```

### 13.4 Early Returns

```gul
fn check(x):
    if x < 0:
        return "negative"
    if x == 0:
        return "zero"
    return "positive"
```

### 13.5 Loops

```gul
loop:
    # Infinite loop
    if condition:
        break

while condition:
    # Conditional loop

for x in iterator:
    # Iterator loop
```

### 13.6 Loop Control

```gul
break          # Exit loop
continue       # Skip iteration
break value    # Loop expression (planned)
```

### 13.7 Error Propagation

```gul
try:
    risky_operation()
catch e:
    handle(e)
```

Result type `Result<T, E>` prefers explicit handling.

### 13.8 Deferrable Execution

```gul
defer cleanup()  # Planned - executes at scope end
```

### 13.9 Event-Driven Flow

Callbacks and async streams:

```gul
server.on("request", async (req):
    # Handle request
)
```

### 13.10 Reactive Flow

Variables bound to streams update automatically (Reactive Extensions).

---

## 14. Async Model

### 14.1 Overview

Cooperative multitasking via `async`/`await`.

### 14.2 Async Schedulers

Pluggable schedulers. Default is multi-threaded work-stealing executor (Tokio-based).

### 14.3 Async Classes

Actors pattern supported via ownership + async methods.

### 14.4 Continuous Execution Blocks

```gul
loop:
    result = await fetch_data()
    process(result)
```

### 14.5 Await Mechanism

```gul
result = await future
```

### 14.6 Ownership in Async

Moved values stay in task. Shared values must be wrapped:

```gul
@shareable mut data = {}
```

### 14.7 Time-Based Async

```gul
await time.sleep(1000)  # Sleep for 1 second
```

### 14.8 Parallelism

`async` is concurrency. Parallelism via `std.thread` or parallel iterators.

### 14.9 Data Streams

```gul
Stream<T> trait
async for item in stream:  # Planned
    process(item)
```

### 14.10 Deadlock Prevention

Ownership model prevents many data races.

---

## 15. Pattern System

### 15.1 Literal Patterns

```gul
match x:
    1: "one"
    "hello": "greeting"
```

### 15.2 Type Patterns

```gul
match value:
    x: int: handle_int(x)
    s: str: handle_str(s)
```

### 15.3 Structure Patterns

```gul
match point:
    Point{x: 0, y: 0}: "origin"
    Point{x, y}: f"({x}, {y})"
```

### 15.4 Ownership Patterns

```gul
match option:
    Some(own x): process_owned(x)
    Some(ref x): read_borrowed(x)
    None: default()
```

### 15.5 Unit Patterns

```gul
match measurement:
    1 m: "one meter"
    _ m: "some meters"
```

### 15.6 Time Patterns

Match time steps in signal processing.

### 15.7 Wildcards

```gul
match x:
    _: "matches anything"
```

### 15.8 Match Guards

```gul
match x:
    n if n > 10: "large"
    n if n > 0: "small"
    _: "zero or negative"
```

### 15.9 Pattern Failures

Runtime error if no match (should be compile-time exhaustive).

### 15.10 Pattern-Based Loops

```gul
for Point{x, y} in points:
    print(f"({x}, {y})")
```

---

## 16. Error System

### 16.1 Error Types

`Result<T, E>` is the core primitive:

```gul
Result<int, Error>
```

`Option<T>` for nullability:

```gul
Option<int>  # Some(value) or None
```

### 16.2 Throwing Errors

Return `Err`:

```gul
fn divide(a: int, b: int) -> Result<int, str>:
    if b == 0:
        return Err("Division by zero")
    return Ok(a / b)
```

### 16.3 Catching Errors

```gul
try:
    result = risky_operation()
catch e:
    print(f"Error: {e}")
```

### 16.4 Custom Error Types

```gul
struct MyError:
    message: str
    code: int
```

### 16.5 Try Blocks

Block evaluates to Result:

```gul
result = try:
    operation1()
    operation2()
```

### 16.6 Panic Semantics

Unrecoverable error. Aborts thread/process:

```gul
panic("Fatal error occurred")
```

### 16.7 Ownership and Errors

Error handling must respect ownership.

### 16.8 Async Errors

Errors propagate through Futures:

```gul
async fn fetch() -> Result<Data, Error>:
    data = await http.get(url)?
    return Ok(data)
```

### 16.9 Compile-Time Errors

Type mismatches, borrow violations detected at compile-time.

### 16.10 Warning Types

Unused variables, dead code detected by linter.

---

## 17. Modules & Imports

### 17.1 Module Structure

Files map to modules:

- `src/math.gul` is module `math`
- Folders are packages

### 17.2 Import Syntax

```gul
import std{io, math}
import python{numpy, pandas}
import [std.math, std.io]
```

### 17.3 Circular Imports

Allowed but discouraged. Resolved at link time.

### 17.4 Importing Classes

```gul
import module{StructName}
```

### 17.5 Importing Functions

```gul
import module{func_name}
```

### 17.6 Importing Ownership

Imports do not transfer ownership of global state, only references.

### 17.7 Importing Lists/Tuples

Constants defined in `.def` files can be imported.

### 17.8 External Dependency System

`gul_packages/` directory stores dependencies:

```bash
gul install package-name
```

### 17.9 Build Caching

Incremental compilation caches intermediate artifacts in `target/`.

### 17.10 Automatic Tree-Shaking

Unused imports are stripped during optimization.

---

## 18. Foreign Language Integration

### 18.1 Foreign Block Overview

Embed code using `extern` syntax:

```gul
extern rust {
    fn fast_compute(n: u64) -> u64 {
        n * n
    }
}
```

### 18.2 Python Integration

```gul
extern python {
    fn analyze(data):
        import numpy as np
        return np.mean(data)
}
```

Via PyO3 FFI.

### 18.3 Rust Integration

```gul
extern rust {
    fn process(data: &[u8]) -> Vec<u8> {
        data.to_vec()
    }
}
```

Compiles as inline Rust modules.

### 18.4 JavaScript Integration

```gul
extern js {
    function formatDate(date) {
        return date.toLocaleDateString();
    }
}
```

For WebAssembly targets via QuickJS.

### 18.5 SQL Integration

```gul
extern sql {
    SELECT * FROM users WHERE active = true
}
```

Compile-time checked embedded queries.

### 18.6 Cross-Language Ownership

Ownership boundaries must be respected. `own` passed to C becomes a pointer.

### 18.7 Concurrency with Foreign Code

Foreign blocks run in GUL scheduler. Blocking FFI calls should be marked async.

### 18.8 Memory Safety Rules

Foreign code inside `unsafe` boundaries.

### 18.9 Inline Code Execution

Executed by respective interpreters or compilers.

### 18.10 Linking & Packaging

Foreign dependencies specified in `package.toml`.

---

## 19. UI Language

### 19.1 Syntax for UI Blocks

GUL supports UI components as first-class syntax:

```gul
ui.print(^&^[button{text="Click Me", color="blue"}])
ui.print(^&^[slider{min=0, max=100, value=50}])
```

### 19.2 Button Components

```gul
^&^[button{
    text: "Submit",
    on_click: handle_click,
    color: "primary"
}]
```

### 19.3 Chart Components

```gul
^&^[chart{
    type: "line",
    data: my_data,
    labels: labels
}]
```

### 19.4 Data Binding

```gul
mut value = 50
^&^[slider{
    value: value,  # Two-way binding
    on_change: (new_val) => value = new_val
}]
```

### 19.5 Table Components

```gul
^&^[table{
    headers: ["Name", "Age", "Status"],
    rows: user_data
}]
```

### 19.6 Events

```gul
on_click, on_hover, on_change
```

### 19.7 Ownership in UI State

UI components own their local state. Global state is borrowed.

### 19.8 Animation Rules

CSS-like transitions defined in properties.

### 19.9 UI Layout Engine

Flexbox-inspired layout model.

### 19.10 Async UI Interaction

Event handlers can be `async fn`.

---

## 20. Scientific & Engineering Systems

### 20.1 Unit Grammar

```gul
value <unit>
9.8 <m/s^2>
```

### 20.2 Derived Units

Units derived from base SI units (m, kg, s, A, K, mol, cd).

### 20.3 Unit Conversion Rules

Automatic conversion if dimensions match:

```gul
1 km + 100 m  # Results in 1100 m
```

### 20.4 Dimensional Analysis

Compile-time check:

```gul
length(m) + time(s)  # ERROR: incompatible dimensions
```

### 20.5 Physical Constants

```gul
import std.science{c, G, h}
const speed_of_light = c  # 299792458 m/s
const gravity = G         # 6.67430e-11 m^3/kg/s^2
```

### 20.6 Scientific Functions

```gul
sin(angle)
cos(angle)
fft(signal)  # Fast Fourier Transform
```

### 20.7 Matrix Systems

Native matrix/tensor types via 4D lists.

### 20.8 Time-Series Data

Native support for `t` axis in arrays:

```gul
const signal[t][x] = ...
```

### 20.9 Simulation Environment

Step-based simulation loop:

```gul
for t in 0..1000:
    update_physics(t)
```

### 20.10 Error Propagation in Units

Uncertainty propagation:

```gul
measurement = 10.0 ± 0.1 m  # Planned
```

---

## 21. Security System

### 21.1 .scrt Files

Encrypted config files. Decrypted only in memory.

```scrt
[api]
key = "sk_live_abc123"
```

### 21.2 Credential Loading

```gul
import secrets
const api_key = secrets.API_KEY
```

### 21.3 Encryption Rules

AES-256 by default for `.scrt` files.

### 21.4 Secure Ownership

`own` data in `.scrt` is zeroed on drop.

### 21.5 Memory Zeroing

Sensitive variables marked `@secure` are zeroed:

```gul
@secure mut password = "..."
```

### 21.6 Safe APIs

`std.net` defaults to TLS 1.3.

### 21.7 Sandboxed Execution

WASM sandbox for untrusted modules.

### 21.8 Permissions Model

Capability-based security (planned).

### 21.9 Integrity Checks

Package signing verification using SHA-256.

### 21.10 Anti-Injection Design

SQL strings are strictly parameterized.

---

## 22. Standard Library

### 22.1 Core

`std.io`, `std.types`, `std.debug`

### 22.2 Collections

`std.collections` - Map, Set, Queue, Stack, LinkedList

### 22.3 Math

`std.math` - sqrt, abs, trig functions, constants

### 22.4 Units

`std.units` - Unit definitions and conversions

### 22.5 Filesystem

`std.fs` - read, write, path operations

### 22.6 Networking

`std.net` - http, tcp, udp sockets

### 22.7 Time

`std.time` - now, duration, sleep, formatting

### 22.8 Concurrency

`std.sync`, `std.thread` - Mutex, Channel, Thread

### 22.9 Compression

`std.compress` - zip, gzip

### 22.10 Cryptography

`std.crypto` - hash (SHA-256, MD5), encrypt, random

---

## 23. Toolchain

### 23.1 Compiler

```bash
gul build [--release] [--target <target>]
```

### 23.2 Runtime

```bash
gul run <file>
```

### 23.3 Package Manager

```bash
gul install <package>
gul publish
```

### 23.4 Build System

`package.toml` configuration:

```toml
[package]
name = "my-project"
version = "1.0.0"

[dependencies]
std = "1.0"
```

### 23.5 Debugger

```bash
gul debug <file>
```

GDB/LLDB wrapper with GUL-aware features.

### 23.6 Formatter

```bash
gul fmt <file>
```

Auto-formats code to standard style.

### 23.7 Linter

```bash
gul lint <file>
```

Checks for style issues, unused code.

### 23.8 Static Analyzer

```bash
gul check
```

Type checking without building.

### 23.9 Test Runner

```bash
gul test
```

Runs all tests in `tests/` directory.

### 23.10 Documentation Generator

```bash
gul doc
```

Generates HTML documentation from code.

---

## 24. Compiler Architecture

### 24.1 High-Level Overview

Rust-based modular crate structure:

```text
Lexer → Parser → AST → Semantic Analysis → IR → CodeGen
```

### 24.2 Lexer

Produces token stream using Logos crate. Supports v2 tokens (`@`, `?`, units).

### 24.3 Parser

Recursive descent parser. Context-aware for different file types (`.def` vs `.fnc`).

### 24.4 AST

Typed Abstract Syntax Tree with ownership nodes:

```rust
enum Expression {
    Literal(Literal),
    Binary(Box<Expression>, Operator, Box<Expression>),
    Call(String, Vec<Expression>),
    // ...
}
```

### 24.5 Symbol Resolver

Scoped symbol table. Resolves globals and imports.

### 24.6 Type Checker

Infers types and checks compatibility including physical units.

### 24.7 Ownership Checker

Validates move/borrow semantics at compile-time.

### 24.8 Unit Checker

Validates dimensional analysis for scientific computing.

### 24.9 Optimizer

- Dead code elimination
- Constant folding
- Inlining
- Loop optimizations

### 24.10 Backend

Generates:

- Native code via LLVM
- WebAssembly via wasm-pack
- Direct interpretation for REPL

---

## 25. Formal Grammar

### 25.1 Program Structure

```ebnf
program ::= statement*
```

### 25.2 Statements

```ebnf
statement ::= import | definition | function | struct |
              if_stmt | loop_stmt | for_stmt | while_stmt |
              match_stmt | try_stmt | expression_stmt
```

### 25.3 Imports

```ebnf
import ::= "import" (grouped_import | simple_import)
grouped_import ::= ident "{" ident_list "}"
simple_import ::= ident ("." ident)*
```

### 25.4 Definitions

```ebnf
definition ::= ("const" | "mut") ident [":" type] "=" expr
```

### 25.5 Functions

```ebnf
function ::= ["async"] "fn" ident "(" params ")" ["->" type] ":" block
params ::= [param ("," param)*]
param ::= ident [":" type]
```

### 25.6 Structs

```ebnf
struct ::= "struct" ident ":" INDENT field* method* DEDENT
field ::= ident ":" type
method ::= function
```

### 25.7 Expressions

```ebnf
expr ::= literal | ident | binary_expr | unary_expr |
         call_expr | index_expr | field_access |
         list_expr | dict_expr | tuple_expr
```

### 25.8 Binary Expressions

```ebnf
binary_expr ::= expr operator expr
operator ::= "+" | "-" | "*" | "/" | "%" | "^" |
             "==" | "!=" | "<" | ">" | "<=" | ">=" |
             "and" | "or"
```

### 25.9 Control Flow

```ebnf
if_stmt ::= "if" expr ":" block ["elif" expr ":" block]* ["else" ":" block]
while_stmt ::= "while" expr ":" block
for_stmt ::= "for" ident "in" expr ":" block
loop_stmt ::= "loop" ":" block
match_stmt ::= "match" expr ":" INDENT case+ DEDENT
case ::= pattern ":" expr
```

### 25.10 Blocks

```ebnf
block ::= INDENT statement+ DEDENT
INDENT ::= <increase in indentation>
DEDENT ::= <decrease in indentation>
```

---

## See Also

- [Syntax Quick Reference](syntax.md)
- [Project Structure Guide](structure.md)
- [Compiler Architecture](../guides/compiler.md)
- [Standard Library Reference](../api/standard-library.md)
- [Complete Knowledgebase](knowledgebase.md)

---

**Note**: This is the complete language specification. For quick reference and common patterns, see the Syntax Guide. For implementation details, consult the Compiler Guide.
