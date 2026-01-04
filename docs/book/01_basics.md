# Chapter 1: Basic Syntax

This chapter covers the building blocks of GUL. By the end, you will be able to write simple scripts and understand how GUL handles data.

## Comments

GUL uses the hash symbol `#` for comments, similar to Python.

```gul
# This is a line comment
let x = 10  # Comment at end of line
```

## Variables & Mutability

Safety is a core principle of GUL. By default, variables are **immutable**.

### 1. Immutable Bindings (`let`)

Use `let` to declare constants. The compiler will prevent you from changing them.

```gul
let pi = 3.14159
# pi = 3.14  <-- Compile Error!
```

### 2. Mutable Variables (`var`)

Use `var` when you need to change a value.

```gul
var count = 0
count = count + 1
```

### 4. Shadowing

You can redeclare a variable with the same name. This is called "shadowing" and is useful for type conversions.

```gul
let x = "100"       # x is a string
let x = x.to_int()  # x is now an int. The string is dropped.
```

## Data Types

### Scalar Types

- **`int`**: 64-bit signed integer (`42`).
- **`float`**: 64-bit floating point (`3.14`).
- **`bool`**: Boolean (`true`, `false`).
- **`str`**: UTF-8 String ("Hello").
- **F-String**: Interpolated String (`f"Val: {x}"`).
- **`char`**: Single Unicode character ('A').

### Compound Types

- **Tuple**: Fixed-size group of mixed types.

    ```gul
    let cords = (10, 20)
    print(cords[0]) # 10
    ```

- **List**: Growable array of same type.

    ```gul
    let numbers = @list[1, 2, 3]
    # numbers.push(4) # Lists are immutable by default
    var mut_nums = @list[1, 2, 3]
    mut_nums.add(4)
    ```

- **Map**: Key-Value dictionary.

    ```gul
    let ages = @dict{"Alice": 30, "Bob": 25}
    ```

## Operators

### Arithmetic

`+`, `-`, `*`, `/`, `%` (modulo), `**` (power).

### Logical

`&&`, `||`, `!`.

```gul
if x > 0 && x < 10:
    pass
```

### Comparison

`==`, `!=`, `<`, `>`, `<=`, `>=`.

## Control Flow

### If Expressions

In GUL, `if` is an expression. It returns a value.

```gul
let status = if age >= 18: "Adult" else: "Minor"
```

### Match (Switch)

GUL has powerful pattern matching.

```gul
let x = 1
match x:
    1 => print("One")
    2 => print("Two")
    _ => print("Other")
```

### Loops

#### `loop`

Infinite loop. Use `break` to exit.

```gul
loop:
    print("Spinning...")
    if done: break
```

#### `while`

```gul
while count < 10:
    count += 1
```

#### `for`

Iterate over collections or ranges.

```gul
for i in range(0, 5):
    print(i)  
# Prints 0, 1, 2, 3, 4
```
