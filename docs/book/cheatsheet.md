# GUL v3.2 Cheatsheet 🚀

## Entry Point

```gul
mn:
    print("Hello, GUL!")
```

## Variables

```gul
let x = 10          # Immutable (Constant)
var y = 20          # Mutable
let z: int = 30     # Explicit Type
let s = @str("hi")  # Explicit constructor
```

## Types

- `@int`, `@float`, `@str`, `@bool`
- `@list`, `@dict`, `@set`, `@tabl`

## Control Flow

```gul
if x > 5:
    print("Big")
else:
    print("Small")

loop:               # Infinite loop
    break

for i in 0..10:
    print(i)

for item in my_list:
    print(item)
```

## Functions

```gul
@fn add(a: int, b: int) -> int:
    return a + b

# Async Function
@async fetch_data(url: str):
    await http.get(url)

# Lambdas
let double = (x) => x * 2
```

## Ownership

```gul
@fn process(move data: list):   # Takes ownership
    pass

@fn view(borrow data: list):    # Immutable reference
    pass

@fn modify(ref data: list):     # Mutable reference
    pass

@fn duplicate(kept data: list): # Makes a copy
    pass
```

## Autograd (v0.14+)

```gul
autograd_begin()
let x = make_var(10.0)
let z = var_mul(x, x)
backward(z)
print(var_grad(x)) # 20.0
autograd_end()
```

## Standard Library

```gul
@imp std{io, math, http, json, fs}

len(items)          # Polymorphic length
print(x)            # Global print
input("?")          # Global input
```

## AI & Tooling

```gul
@ai(model="claude-3")
@fn summarize(text: str) -> str

@python {
    import pandas as pd
}
```
