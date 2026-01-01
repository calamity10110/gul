# Chapter 3: Modules & Projects

## The `mn` Entry Point

Every runnable GUL program starts execution at the `mn` block.

```gul
mn:
    app.start()
```

## Imports (`imp`)

Use `imp` to include other packages or standard library modules.

```gul
imp std.io
imp std.fs as file_system
imp my_package.utils
```

### Grouped Imports

```gul
imp {
    std.io,
    std.net,
    package.module
}
```

## Package Structure

A typical GUL project looks like this:

```
my-project/
├── Gul.toml        # Manifest (like Cargo.toml)
├── src/
│   ├── main.mn     # Entry point
│   ├── lib.mn      # Library root
│   └── utils.mn    # Sub-module
└── README.md
```

## Annotations

GUL uses `@` for annotations (decorators).

- `@test`: Marks a function as a test.
- `@global`: Defines a global variable.
- `@ai`: Integrates implementation via AI.

```gul
@test
fn test_addition():
    assert(1 + 1 == 2)
```
