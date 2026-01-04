
# Walkthrough - Implement Arrays and Lists

This task implemented support for Array/List types, literal syntax, indexing, and assignment.

## Changes

### AST (`src/ast/mod.rs`)

- Added `Type::List(Box<Type>)` to support typed lists.
- Added `ExpressionKind::List(Vec<Expression>)` for literals `[1, 2, 3]`.
- Added `ExpressionKind::Index { target, index }` for `arr[0]`.
- Added `Statement::SetIndex { target, index, value }` for `arr[0] = 5`.

### Parser (`src/parser/mod.rs`)

- Updated `parse_primary` to parse list literals `[...]`.
- Updated `parse_call` to handle postfix index operator `[...]`.
- Updated `parse_expression_or_assignment` to parse `arr[0] = val`.

### Semantic Analysis (`src/semantic/mod.rs`)

- Implemented type inference for `List`. Infers type from the first element.
- Implemented type resolution for `Index` (extracts inner type of List).
- Implemented analysis for `SetIndex`.

### IR Generator (`src/ir/mod.rs`)

- Added `malloc` declaration from system/runtime.
- Implemented `List` generation:
  - Allocates `(size + 1) * 8` bytes.
  - Stores length at offset 0.
  - Stores elements at offsets `8 * (i + 1)`.
- Implemented `Index` generation:
  - Calculates address `base + 8 + (index * 8)`.
  - Loads value.
- Implemented `SetIndex` generation:
  - Calculates address.
  - Stores value.

## Verification

### Automated Tests (`tests/run_tests.py`)

- **`test_arrays.gul`**: Creating int arrays, reading indices, summing elements.
- **`test_arrays_str.gul`**: Creating string arrays, string concatenation.
- **`test_array_set.gul`**: Modifying array elements.

All 8 tests passed.

## Next Steps

1. **Structs**: Implement struct definition and member access (`.` operator).
2. **Memory Management**: Implement GC or more robust memory handling (current `malloc` leaks).
3. **For Loops**: Sugar for `while` loops over Iterators (Lists).
