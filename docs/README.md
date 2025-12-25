# GUL Documentation

**Version**: 0.13.0 | **Syntax**: v3.0

---

## Quick Links

- [Quick Reference](QUICK_REFERENCE.md) - Cheat sheet
- [Syntax Guide](reference/syntax.md) - Complete syntax
- [Standard Library](api/standard-library.md) - 13 modules, 110+ functions
- [Data-Flow Guide](guides/dataflow.md) - Node-based programming

---

## Getting Started

```gul
@imp std.io

mn:
    print("Hello, GUL!")
```

**Run**: `gul run hello.mn`

---

## Documentation Structure

### Guides

- [Introduction](guides/introduction.md) - Language overview
- [Data-Flow](guides/dataflow.md) - Contract-based programming
- [Installation](guides/installation.md) - Setup
- [TUI IDE](guides/tui.md) - Terminal IDE

### Reference

- [Syntax](reference/syntax.md) - Complete syntax
- [Types](reference/types.md) - Type system
- [Ownership](reference/ownership.md) - Memory model
- [Language Spec](language_spec.md) - **GUL 101** - Complete specification

### API

- [Standard Library](api/standard-library.md) - Core modules

### History

- [Development History](devhistory.md) - v1.0 → v3.0 evolution

---

## v3.0 Syntax

```gul
let name = "value"    # Immutable
var count = 0         # Mutable
mn:                   # Main entry
@imp std.http         # Import
@python { }           # Foreign code
```

---

**Last Updated**: 2025-12-18
