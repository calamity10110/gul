# GUL (GUL Universal Language) Knowledgebase

Version: 0.13.0 (v2.0 Syntax)
Date: 2025-12-09

## 1. Project Overview

GUL is a modern, multi-paradigm programming language designed to combine Python's readability with Rust's safety and performance. It features built-in capabilities for scientific computing, UI development, and multi-language integration.

**Key Philosophies:**

- **Simplicity:** Indentation-based, readable syntax.
- **Performance:** Native compilation via Rust-based toolchain.
- **Safety:** Strong ownership model (`own`, `ref`, `copy`) without garbage collection.
- **Autonomy:** Built-in AI and self-organization features.
- **Universality:** Runs on Desktop, Web (WASM), and Embedded devices.

## 2. Package Ecosystem

The GUL ecosystem is organized into a modular package structure in `gul_packages/`.

### 2.1 Core Standard Library (`gul_packages/std/`)

- **http.mn**: HTTP client/server (Requests/Reqwest inspired).
- **json.mn**: JSON parsing and serialization (Serde inspired).
- **ml.mn**: Machine Learning primitives (Tensors, Models, Layers).
- **utils.mn**: Common utilities (Regex, DateTime, Crypto).

### 2.2 Extended Standard Library

- **net.mn**: Low-level networking (TCP/UDP sockets).
- **db.mn**: Database abstractions (PostgreSQL, SQLite).
- **tui.mn**: Terminal User Interface framework (Widgets, Layouts).
- **web.mn**: Web application framework (Routing, Middleware).

### 2.3 Domain Specific Libraries

- **data.mn**: Data science tools (DataFrames, Arrays).
- **science.mn**: Scientific modules (Physics, Chemistry, Biology, Math).
- **robotics.mn**: IoT and Robotics (GPIO, Sensors, ROS integration).

## 3. Language Features

### Syntax (v2.0)

- **File Extensions:** `.mn` (General), `.def` (Definitions), `.fnc` (Functions), `.mn` (Main).
- **Blocks:** Indentation-based (Python-style).
- **Comments:** `# Single line`, `#[ Multi-line ]#`.
- **Keywords:** `import`, `const`, `mut`, `fn`, `async`, `extern`, `main`, `struct`.

### Ownership Model (Unchanged)

- `own`: Transfers ownership (Move semantics).
- `ref`: Borrows a reference (Immutable default).
- `copy`: Creates a duplicate.

### Multi-Language Integration (`extern` blocks)

Embed foreign code directly:

- `extern python {...}`: Embed Python code.
- `extern rust {...}`: Embed Rust code.
- `extern js {...}`: Embed JavaScript.
- `extern sql {...}`: Embed SQL queries.

### UI Syntax

First-class UI component literals:

- `^÷^[button{text="Click"}]`
- `^÷^[chart{data=...}]`

### Scientific Computing

Native support for units:

- `def speed = 10 m/s`
- `def force = 100 N`

## 4. Directory Structure

```
/
├── gul_packages/           # GUL Implementation Packages
│   ├── std/                # Standard Library Modules
│   └── examples/           # GUL Code Examples
├── src/                    # Rust-based Compiler Source
│   ├── lexer/              # Tokenizer
│   ├── parser.rs           # Parser
│   ├── semantic.rs         # Semantic Analysis
│   └── compiler/           # Code generation
├── examples/               # Native Rust examples
├── docs/                   # Documentation (SYNTAX, STRUCTURE, etc.)
└── target/                 # Build artifacts
```

## 5. GUL — Reference Manual

0. Front Matter

0.1 Preface
This manual defines the GUL (GUL Universal Language) programming language, version 0.12.0. GUL is designed for modern development needs, blending Python's readability with Rust's performance and safety.

0.2 Scope of the Manual
This document covers the language syntax, semantics, standard library interfaces, and toolchain usage.

0.3 Conventions Used

- `code` fixed-width font for code.
- **Bold** for emphasized terms.
- `[ ... ]` denotes optional syntax in grammar sections.

  0.4 Versioning Scheme
  GUL follows Semantic Versioning 2.0.0.

  0.5 Glossary

- **Ownership**: The system by which memory is managed without a garbage collector.
- **Foreign Block**: A block of code in another language (e.g., Python, C) embedded within GUL.
- **Unit**: A scientific unit of measurement (e.g., `m/s`) attached to a numeric value.

  0.6 Notational Conventions
  Grammar is presented in simplified EBNF.

1. Language Overview

1.1 Purpose of GUL
GUL aims to be a "universal" language, suitable for systems programming, web development, data science, and scripting.

1.2 Design Philosophy

- **Readability First**: Syntax should be obvious and clean.
- **Safety by Default**: Memory safety without GC pause times.
- **Interoperability**: Seamless integration with existing ecosystems (Python, Rust, C).

  1.3 Core Principles

- Strict File Roles: logic (`.fnc`) is separated from definitions (`.def`) and orchestration (`.mn`).
- Explicit Ownership: `own`, `ref`, `copy` keywords make memory intent clear.

  1.4 Compilation Pipeline Summary
  Source -> Lexer -> AST -> Semantic Analysis -> IR -> CodeGen (LLVM/Rust/WASM).

  1.5 Execution Model
  Compiled to native machine code or WebAssembly. No VM required for pure GUL code.

  1.6 Supported Paradigms

- Imperative
- Functional (Lisp-style list ops, first-class functions)
- Object-Oriented (Structs/Classes with methods)
- Async/Concurrent

  1.7 Static vs Dynamic Features
  Statically typed with type inference. Dynamic features available via `any` type or foreign blocks.

  1.8 Use Cases

- High-performance web servers.
- Scientific simulations (native units).
- Embedded systems (no GC).

  1.9 Integration Ecosystem
  Built-in support for embedding Rust, Python, and C via `cs` blocks.

  1.10 Toolchain Overview

- `gul`: The primary CLI tool (compiler, runner, package manager).

2. Language Structure

2.1 File Types (v2.0)

- `.def`: Definition files. Contains `const`, `mut`, `struct`, `@global`, `import`. No control flow.
- `.fnc`: Function files. Contains `fn`, `async` functions. Pure logic.
- `.mn`: Main files. Contains `mn:` entry point and orchestrates execution.
- `.scrt`: Secret files. Encrypted storage for API keys/credentials.
- `.mn`: General purpose files (can contain any valid GUL code).

  2.2 Module System
  Modules map to file paths. `@imp std.math` imports `std/math.mn` or `std/math` package.

  2.3 Naming Rules

- Variables/Functions: `snake_case` (e.g., `my_variable`).
- Types/Structs: `PascalCase` (e.g., `MyStruct`).
- Constants: `SCREAMING_SNAKE_CASE` (e.g., `MAX_RETRY`).

  2.4 Unicode Support
  Source code must be UTF-8. Identifiers can contain Unicode characters.

  2.5 Code Blocks & Indentation
  GUL uses significant indentation (4 spaces recommended). Blocks are denoted by `:`.

```gul
if x > 0:
    print("Positive")
```

2.6 Comments

- `#`: Single line comment.
- `#[ ... ]#`: Multi-line comment.

  2.7 Documentation Blocks
  Triple-quoted strings `""" ... """` at the start of functions/modules act as docstrings.

  2.8 Metadata Headers
  Files can start with metadata comments:
  `# @version: 1.0`
  `# @author: Jane`

3. Lexical Structure

3.1 Character Set
UTF-8.

3.2 Tokens
See Lexer module. Includes keywords, identifiers, literals, operators, delimiters.

3.3 Identifiers
Start with letter or `_`, followed by letters, numbers, `_`.

3.4 Keywords (v2.0)
Primary: `import`, `const`, `mut`, `fn`, `async`, `extern`, `main`, `struct`, `global`, `static`, `local`.
Legacy (deprecated): `imp`, `def`, `asy`, `cs`, `mn`.
Common: `own`, `ref`, `copy`, `await`, `loop`, `if`, `elif`, `else`, `for`, `while`, `return`, `break`, `continue`, `try`, `catch`.

3.5 Literals

- **String**: `"hello"` or `'hello'`.
- **Number**: `123`, `12.34`, `0xFF`.
- **Unit**: `10 kg`, `9.8 m/s^2` (Number followed by Unit identifier).
- **Tuple**: `(1, 2)`.
- **List**: `[1, 2, 3]`.
- **Dict**: `{key: val}`.

  3.6 Operators
  `+`, `-`, `*`, `/`, `%`, `^` (power), `==`, `!=`, `<`, `>`, `<=`, `>=`.
  `and`, `or`, `not`.

  3.7 Delimiters
  `()`, `[]`, `{}`, `,`, `:`, `.`, `->`.

  3.8 Escape Rules
  Standard C-style escapes: `\n`, `\t`, `\"`, `\\`.

4. Types

4.1 Primitive Types

- `int` (64-bit signed).
- `float` (64-bit IEEE).
- `bool` (`true`, `false`).
- `str` (UTF-8 string).
- `char` (Unicode scalar).

  4.2 Compound Types

- **Structs**: User-defined records.
- **Enums**: Tagged unions (planned).

  4.3 Tuple Types
  `(int, str)` - Fixed size, mixed types.

  4.4 List Types (4-D)
  `[int]` - Dynamic arrays.
  Native support for up to 4 dimensions for scientific data: `x`, `y`, `z`, `t` (time).

  4.5 Dictionary Types
  `{str: int}` - Hash map.

  4.6 Range Types
  `0..10` (exclusive), `0..=10` (inclusive).

  4.7 Unit Types (Scientific Units)
  Types can include dimensions: `float<m/s>`.
  Arithmetic operations enforce dimensional consistency (e.g., `L / T = V`).

  4.8 Time-Aware Types
  Variables can track history if enabled: `?history` annotation (planned).

  4.9 Optional Types
  `int?` or `Option<int>`.

  4.10 Type Inference
  GUL infers types where possible:
  `def x = 10` (inferred as `int`).
  `def y = 10 m` (inferred as `float<m>`).

5. Ownership Model

5.1 Ownership Overview
GUL uses a move-by-default system. Variables are transferred unless specified otherwise.

5.2 own Semantics
`def own x = value`: Explicitly claims ownership.
`fn take(own x)`: Function takes full ownership of `x`. Usage of `x` in caller after this is a compile error.

5.3 ref Semantics
`fn peek(ref x)`: Borrows `x` immutably.
`fn mut_peek(ref mut x)`: Borrows `x` mutably (planned).

5.4 copy Semantics
`def y = copy x`: Creates a deep clone of `x`.
Primitive types (`int`, `bool`, `float`) are `copy` by default (implicit copy on assignment).

5.5 Borrow Checker Rules

- One mutable reference OR multiple immutable references.
- References cannot outlive the owner.

  5.6 Move Semantics
  Assignment transfers ownership for non-copy types:

```gul
def list1 = [1, 2, 3]
def list2 = list1  # list1 is now invalid
```

5.7 Lifetimes
Lifetimes are currently inferred. Explicit lifetime syntax (e.g., `'a`) is reserved for future advanced usage.

5.8 Safe & Unsafe Blocks
Normal GUL code is safe. `unsafe` blocks (planned) will allow manual pointer manipulation.

5.9 Ownership in Collections
Collections own their elements. Moving a collection moves all elements.

5.10 Ownership + Async Interactions
Async tasks must own their data or act on `ref` strictly scoped execution contexts.

6. Variables

6.1 Declarations
`def name = value`: Standard declaration. Inferred type.
`def name: type = value`: Explicit type.

6.2 Mutability
Variables are immutable by default.
`def ?x = 10`: Mutable local variable (syntactic sugar for mutable binding).
`?x = 20`: Assignment to mutable variable.

6.3 Shadowing
Variables can be shadowed in inner scopes:

```gul
def x = 10
if true:
    def x = 20  # Shadows outer x
```

6.4 Global Variables
Declared with `@global`.
`@global ?config = {}`.

6.5 Static Variables
`@static` annotation keeps variable alive across function calls (like C static locals).

6.6 Lazy Variables
`lazy def x = computation()`: Evaluated on first access (planned).

6.7 Compile-Time Constants
`def CONSTANT_NAME = value`: Treated as constant if uppercase.
`let X = 10`: Explicit constant keyword (v2).

6.8 Variable Units
`def distance = 100 m`: Variable holds both value `100` and unit `m`.

6.9 Time-Dimension Variables
Variables in 4-D lists can automatically track values over `t`.

6.10 Scoped Variables
`@local`: Explicitly local scope (redundant in `.fnc` but clear).

7. Expressions

7.1 Expression Grammar
`expr ::= primary | unary | binary | call | await`

7.2 Precedence Rules
Standard PEMDAS. `^` (power) is highest arithmetic. `and`/`or` are lower than comparison.

7.3 Arithmetic Expressions
`1 + 2`, `x * y`.
Supports unit arithmetic: `10 m / 2 s` results in `5 m/s`.

7.4 Logical Expressions
`true and false`, `not valid`, `a == b`.

7.5 Unit Expressions
`x + 5 kg`. Error if `x` is not `mass` unit.

7.6 Range Expressions
`0..10`: Generates sequence 0 to 9.
Used in `for` loops.

7.7 List/tuple expressions
`[1, 2, 3]`, `(1, "a")`.

7.8 Time Expressions
`t` is a reserved variable in time-series contexts representing current time step.

7.9 Generator Expressions
`(x * 2 for x in list)`: Lazy generator (planned).

7.10 Pattern-based expressions
`match x { 1 => "one", _ => "other" }` evaluates to a value.

8. Statements

8.1 Expression Statements
Statements that evaluate to a value but return ignored (e.g., function call).

8.2 Variable Bindings
`def`, `const`.

8.3 Assignment
`name = value`. Target must be mutable.

8.4 Ownership Transfer Statements
`return own x` (explicit transfer).

8.5 if Blocks

```gul
if condition:
    ...
elif other:
    ...
else:
    ...
```

8.6 then and else semantics
`else` block executes if condition is false.

8.7 while
`while condition: ...`

8.8 for
`for item in collection: ...`
`for i in 0..10: ...`

8.9 match

```gul
match value:
    1: print("One")
    2: print("Two")
    _: print("Other")
```

8.10 try/except/finally

```gul
try:
    danger()
catch e:
    handle(e)
```

9. Definition Files (.def)

9.1 Overview
`.def` files define the structure of a program or module. They cannot contain executable logic blocks (like `if` at top level).

9.2 Imports
`@imp std.io` works here to bring types into scope.

9.3 Type Definitions
`struct Point { x: float, y: float }`

9.4 List & Tuple Definitions
Defines global constants or data structures.
`def DATA = [1, 2, 3]`

9.5 Class Definitions
`class User { ... fields ... methods ... }` (v2 uses `struct` + `impl`).

9.6 Initialization Order
Definitions are initialized in order of appearance.

9.7 Default Modules
`std` core types are implicitly imported.

9.8 Compile-Time Definitions
Macros or constants evaluated during compilation.

9.9 Multi-file Projects
`.def` files can reference other `.def` files via imports.

9.10 Namespace Rules
Definitions in `module.def` are accessed via `module.Name`.

10. Lists, Tuples & 4-Dimensional Data

10.1 Basic Lists
Heterogeneous lists are allowed `[1, "a"]` (typed as `[any]`), but homogeneous `[int]` preferred.

10.2 Basic Tuples
Immutable fixed-size sequences `(1, 2)`.

10.3 List/Tuple Functions (LISP-style)

- `car(list)`: Head.
- `cdr(list)`: Tail.
- `cons(item, list)`: Prepend.

  10.4 2D Lists
  `[[1, 2], [3, 4]]`. Matrix operations supported via `std.math`.

  10.5 3D Lists
  Volumetric data.

  10.6 4D Lists (z-y-x-time)
  Specialized for physics simulations. Access via `data[t][z][y][x]`.

  10.7 Mutable vs Static Lists
  `def own list = [...]` is mutable (growable).
  `def list = [...]` is immutable view by default.

  10.8 Predictive Timelines
  Using ML to predict `t+1` in 4D lists (integrated with `std.ml`).

  10.9 Memory Rules for 4-D Lists
  Large 4D lists use sparse storage or disk-backed tensors.

  10.10 Interpolation & Projection
  Built-in `interpolate(list, t=1.5)` methods.

11. Classes & Objects

11.1 Class Syntax
GUL v2 uses `struct` for data and `impl` blocks (planned) for methods.

```gul
struct User:
    name: str
    age: int
```

Legacy `class` syntax is deprecated.

11.2 Fields
Defined in `struct` block. Typed.

11.3 Methods
Defined in `impl` block (or `struct` body in current parser version).

```gul
struct User:
    fn grow_older(self):
        self.age = self.age + 1
```

11.4 Constructors
Static method `new` is conventional.
`def u = User(name="Bob", age=30)` (struct literal).

11.5 Destructors
`drop` method (planned) for cleanup.

11.6 Inheritance
Composition over inheritance. `trait` system (planned) replaces inheritance.

11.7 Mixins
Via traits.

11.8 Traits
Define shared behavior. `impl Trait for Struct`.

11.9 Async Classes
Structs can have async methods.

11.10 Ownership of Fields
Structs own their fields. Accessing a non-copy field moves it unless borrowed.

12. Functions (.fnc)

12.1 Function Block Structure
`fn name(params) -> return_type:`

12.2 Async Functions (async "name")
`async name():` or `fn name() -> async:`
V2 keyword: `asy` or `async fn`.

12.3 Non-Async Functions
Standard `fn`. Execution blocks until return.

12.4 Return Semantics
Explicit `return value`. Implicit return of last expression (if block is expression-oriented, currently strict statement-based).

12.5 Argument Ownership
`fn foo(own x)`: Moves.
`fn bar(ref x)`: Borrows.

12.6 Variadic Functions
`fn log(*args)`: Not yet fully supported in parser, planned for v2.1.

12.7 Unit-Aware Functions
`fn speed(d: float<m>, t: float<s>) -> float<m/s>`: Signatures enforce units.

12.8 Time-Dimensional Functions
Functions can operate on `t` axis of 4D inputs naturally.

12.9 Multi-Dispatch
Overloading based on types.

12.10 Function Overloading
Multiple definitions with different signatures allowed in `.def` interfaces (implementation via traits).

13. Control Flow

13.1 Conditions
`if condition: ... elif: ... else: ...`

13.2 Pattern Matching
`match` statement. Exclusive exhaustive checking.

13.3 Comprehensions
`[x * 2 for x in list]` (planned).

13.4 Early Returns
`return` exits immediately.

13.5 Loops
`loop`: Infinite loop.
`while condition`: Conditional loop.
`for x in iter`: Iterator loop.

13.6 Loop Control
`break` exits loop.
`continue` skips iteration.
`break value` (loop expressions).

13.7 Error Propagation
`try/catch`. Result type `Result<T, E>` prefers explicit handling.

13.8 Deferrable Execution
`defer cleanup()`: (Planned).

13.9 Event-Driven Flow
Callbacks and async streams.

13.10 Reactive Flow
Variables bound to streams update automatically (Reactive Extensions).

14. Async Model

14.1 Overview
Cooperative multitasking via `async`/`await`.

14.2 Async Schedulers
Pluggable schedulers. Default is a multi-threaded work-stealing executor (Tokio-based).

14.3 Async Classes
Actors pattern supported via ownership + async methods.

14.4 Continuous Execution Blocks
`loop: ... await ...`

14.5 Await Mechanism
`await future`.

14.6 Ownership in Async
Moved values stay in task. Shared values must be `Arc`/`Mutex` (std.sync).

14.7 Time-Based Async
`await time.sleep(1 s)`.

14.8 Parallelism
`async` is concurrency. Parallelism via `std.thread` or parallel iterators.

14.9 Data Streams
`Stream<T>` trait. `async for` (planned).

14.10 Deadlock Prevention
Ownership model prevents many data races.

15. Pattern System

15.1 Literal Patterns
Match exact values: `1`, `"hello"`.

15.2 Type Patterns
Match types: `x: int`.

15.3 Structure Patterns
Destructuring: `Point { x, y }`.

15.4 Ownership Patterns
Match and move: `Some(own x)`.

15.5 Unit Patterns
Match units: `1 m`.

15.6 Time Patterns
Match time steps (in signal processing).

15.7 Wildcards
`_` matches anything.

15.8 Match Guards
`case x if x > 10`.

15.9 Pattern Failures
Runtime error if no match (should be compile time exhaustive).

15.10 Pattern-Based Loops
`for Point {x, y} in points:`

16. Error System

16.1 Error Types
`Result<T, E>` is the core primitive. `Option<T>` for nullability.

16.2 Throwing Errors
`raise Error("msg")` (legacy) or return `Err`.

16.3 Catching Errors
`try: ... catch e: ...` converts Result to flow.

16.4 Custom Error Types
Structs implementing `Error` trait.

16.5 Try Blocks
Block evaluates to Result.

16.6 Panic Semantics
Unrecoverable error. Aborts thread/process.

16.7 Ownership and Errors
Error handling must respect ownership (can't use moved value in catch).

16.8 Async Errors
Errors propagate through Futures.

16.9 Compile-Time Errors
Type mismatches, borrow violations.

16.10 Warning Types
Unused variables, dead code (linter).

17. Modules & Imports

17.1 Module Structure
Files map to modules. `src/math.mn` is `math`. Folders are packages.

17.2 Import Syntax
`@imp std.io` or `import std.io`.
`@imp [std.math, std.io]` (Grouped).

17.3 Circular Imports
Allowed but discouraged. Resolved at link time.

17.4 Importing Classes
`@imp module.StructName`.

17.5 Importing Functions
`@imp module.func_name`.

17.6 Importing Ownership
Imports do not transfer ownership of global state, only references.

17.7 Importing Lists/Tuples
Constants defined in `.def` files can be imported.

17.8 External Dependency System
`gul_packages/` directory stores dependencies managed by `gul install`.

17.9 Build Caching
Incremental compilation caches intermediate artifacts in `target/`.

17.10 Automatic Tree-Shaking
Unused imports are stripped during optimization.

18. Foreign Language Integration

18.1 Foreign Block Overview
Embed code using `cs language:` syntax.

```gul
cs rust:
    let x = 5;
```

18.2 Python Integration
`cs python:` executes Python code via FFI (PyO3).

18.3 Rust Integration
`cs rust:` compiles as inline Rust modules.

18.4 JavaScript Integration
`cs js:` for WebAssembly targets.

18.5 SQL Integration
`cs sql:` for embedded queries (compile-time checked).

18.6 Cross-Language Ownership
Ownership boundaries must be respected. `own` passed to C becomes a pointer that C must free (or transfer back).

18.7 Concurrency with Foreign Code
Foreign blocks run in the GUL scheduler. Blocking FFI calls should be marked async.

18.8 Memory Safety Rules
Foreign code inside `unsafe` boundaries.

18.9 Inline Code Execution
Executed by respective interpreters or compilers.

18.10 Linking & Packaging
Foreign dependencies must be specified in `gul.toml`.

19. UI Language

19.1 Syntax for UI Blocks
GUL supports two equivalent syntaxes for UI components: 1. Sprite Syntax: `^&^[ component_name { props } ]` 2. Macro Syntax: `ui![ component_name { props } ]`

    Both syntaxes are semantically identical.

19.2 Button
`button { text="Click", on_click=handler }`

19.3 Button Components
`^&^[button{text="Submit"}]`

19.4 Chart Components
`^&^[chart{type="line", data=my_list}]`

19.5 Data Binding
`value=?my_var` (Two-way binding).

20. WEB DEVELOPMENT
    GUL v2.0 includes a robust web server in `std.web`.

20.1 Basic Server
```gul
@@imp std.web

    mn:
        app = web.App()

        # Routing (Express/Flask style)
        app.get("/", async (req, res):
            return res.json({message: "Hello GUL Web!"})
        )

        # Path Parameters
        app.get("/users/:id", async (req, res):
            user_id = req.params.id
            return res.send(f"User {user_id}")
        )

        await app.listen(3000)
    ```

20.2 Database Integration (std.db)
The `std.db` module provides a unified interface for SQL and NoSQL.

    ```gul
    @@imp std.db

    # Query
    users = await db.query("SELECT * FROM users WHERE active = ?", [true])
    ```

20.3 JSON Handling
`std.json` is used for parsing and stringifying.
`data = json.parse(string)`
`string = json.stringify(object)`

21. TESTING
    GUL has a built-in test runner. Tests live in `tests/` directory or alongside code.

21.1 Writing Tests
Use the `test` keyword (or `@test` annotation in legacy).

    ```gul
    # In my_file.mn
    fn add(a, b): return a + b

    test "addition works":
        assert_eq(add(1, 2), 3)

    test "negative numbers":
        assert(add(-1, -1) == -2)
    ```

21.2 Running Tests
`gul test` -> Runs all tests.
`gul test my_file.mn` -> Runs tests in specific file.

21.3 Assertions
`assert(condition)`
`assert_eq(a, b)`
`assert_ne(a, b)`

22. DEPLOYMENT & DEVOPS
    Building for production requires specific flags.

22.1 Building for Production
Use the `--release` flag to optimize the binary.
`gul build --release`

    To build a static binary (for Alpine/Scratch containers):
    `gul build --release --target x86_64-unknown-linux-musl`

22.2 Environment Variables
GUL loads `.env` files automatically in development. In production, use system env vars.
`@imp std.env`
`db_host = env.get("DB_HOST", "localhost")`

22.3 Docker
Example Dockerfile:
```dockerfile
FROM gul-lang/builder:latest AS builder
COPY . .
RUN gul build --release

    FROM alpine:latest
    COPY --from=builder /app/dist/my_app /my_app
    CMD ["/my_app"]
    ```

22.4 Secrets
In Dev: Use `.scrt` files (encrypted).
In Prod: Use Environment Variables or Secret Managers. `.scrt` files are excluded from git.

23.6 Events
`on_click`, `on_hover`.

23.7 Ownership in UI State
UI components own their local state. Global state is borrowed.

19.8 Animation Rules
CSS-like transitions defined in properties.

19.9 UI Layout Engine
Flexbox-inspired layout model.

19.10 Async UI Interaction
Events handlers can be `async fn`.

20. Scientific & Engineering Systems

20.1 Unit Grammar
`value <unit>`. Ex: `9.8 <m/s^2>`.

20.2 Derived Units
Units derived from base SI units (m, kg, s, A, K, mol, cd).

20.3 Unit Conversion Rules
Automatic conversion if dimensions match. `1 km + 100 m` = `1100 m`.

20.4 Dimensional Analysis
Compile-time check. `len(m) + time(s)` is an error.

20.5 Physical Constants
`std.science.c` (speed of light), `G` (gravity).

20.6 Scientific Functions
`sin`, `cos`, `fft` (Fast Fourier Transform).

20.7 Matrix Systems
Native matrix/tensor types via 4D lists.

20.8 Time-Series Data
Native support for `t` axis in arrays.

20.9 Simulation Environment
Step-based simulation loop built-in.

20.10 Error Propagation in Units
Uncertainty propagation `10.0 ± 0.1 m` (planned).

21. Security System

21.1 .scrt Files
Encrypted config files. Decrypted only in memory.

21.2 Credential Loading
`std.security.load_key("api_key")`.

21.3 Encryption Rules
AES-256 by default.

21.4 Secure Ownership
`own` data in `.scrt` is zeroed on drop.

21.5 Memory Zeroing
Sensitive variables marked `@secure` are zeroed.

21.6 Safe APIs
`std.net` defaults to TLS 1.3.

21.7 Sandboxed Execution
WASM sandbox for untrusted modules.

21.8 Permissions Model
Capability-based security (planned).

21.9 Integrity Checks
Package signing verification.

21.10 Anti-Injection Design
SQL strings are strictly parameterized.

22. Standard Library

22.1 Core
`std.io`, `std.types`, `std.debug`.

22.2 Collections
`std.collections` (Map, Set, Queue).

22.3 Math
`std.math` (sqrt, abs, trig).

22.4 Units
`std.units` (unit definitions).

22.5 Filesystem
`std.fs` (read, write, path).

22.6 Networking
`std.net` (http, socket).

22.7 Time
`std.time` (now, duration, sleep).

22.8 Concurrency
`std.sync`, `std.thread`.

22.9 Compression
`std.compress` (zip, gzip).

22.10 Cryptography
`std.crypto` (hash, encrypt).

23. Toolchain

23.1 Compiler
`gul build`.

23.2 Runtime
`gul run`.

23.3 Package Manager
`gul install`, `gul publish`.

23.4 Build System
`gul.toml` configuration.

23.5 Debugger
`gul debug` (GDB/LLDB wrapper).

23.6 Formatter
`gul fmt`.

23.7 Linter
`gul lint`.

23.8 Static Analyzer
`gul check`.

23.9 Test Runner
`gul test`.

23.10 Documentation Generator
`gul doc`.

24. Compiler Architecture

24.1 High-Level Overview
Rust-based. Modular crate structure.

24.2 Lexer
Produces token stream. Supports v2 tokens (`@`, `?`).

24.3 Parser
Recursive descent. Context-aware (`.def` vs `.fnc`).

24.4 AST
Typed AST with Ownership nodes.

24.5 Symbol Resolver
Scoped symbol table. Resolves globals and imports.

24.6 Type Checker
Infers types and checks compatibility (including Units).

24.7 Ownership Checker
Validates move/borrow semantics.

24.8 Unit Checker
Validates dimensional analysis.

24.9 Optimizer
Dead code elimination, constant folding.

24.10 Backend
Generates native code (via LLVM) or interprets directly.

25. Formal Grammar (Simplified)

```ebnf
program ::= statement*
statement ::= import | definition | function | struct | if | loop
import ::= "imp" ("[" ident_list "]" | ident)
definition ::= "def" ident ["=" expr]
function ::= ["async"] "fn" ident "(" params ")" ":" block
block ::= indent statement* dedent
expr ::= literal | ident | binary_op | call
```

26–200. Detailed API Documentation
(See `gul_packages/std/README.md` and generated docs).

200. Package database and Manager
     Central registry at `registry.mn-lang.org` (mock).

201. Builtin features
     `print`, `len`, `range`, `car`, `cdr`, `cons`.

202. Builtin compiler
     Self-hosting (planned).

203. Supported targets
     Linux (x64, ARM), Windows, macOS, WASM.

204. Installation
     `curl https://gul-lang.org/install.sh | sh`

205. Usage
     `gul new my_project`
     `cd my_project`
     `gul run`

206. Best Practices

- Use `.def` for config, `.fnc` for logic.
- Prefer `own` for clear data flow.
- Use units for all physical quantities.

### GUL v2.0 Language Refactor (December 2025)

- **Core Goal**: Unified language specification with strict file type roles and stronger memory safety.
- **New File Types**:
  - `.def`: Definitions (variables, constants, external interfaces). No logic allowed.
  - `.fnc`: Functions (pure logic).
  - `.mn`: Main entry point (orchestration).
  - `.scrt`: Secrets (encrypted storage).
  - `.cs`: Foreign blocks (embedded C#, Rust, etc.).
- **Ownership Syntax**: Introduced `own`, `ref`, `copy` keywords for explicit memory management.
  - `def own data = [...]`
  - `fn process(ref config)`
- **List Operations**: Lisp-style builtins added: `car`, `cdr`, `cons`, plus `Expression::List` and `Expression::Dict` support.
- **Global State**: Mutable global variables now require `@global` annotation and `?` prefix for mutations (e.g., `?counter = 20`).
- **Foreign Blocks**: Syntax `cs language:` (e.g., `cs rust:`) allows embedding foreign code, parsed into `Statement::ForeignBlock`.
