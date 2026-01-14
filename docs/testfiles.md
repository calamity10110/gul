# GUL Test Files Reference

This document describes the test files used to verify GUL compiler functionality.

## Test Locations

| Location | Compiler | Description |
|----------|----------|-------------|
| `compilers/shared/tests/` | Both | Cross-compiler compatibility tests |
| `compilers/nightly/tests/` | Nightly | Nightly-specific feature tests |
| `compilers/stable/tests/` | Stable | Stable-specific tests (TBD) |

---

## Shared Tests (`compilers/shared/tests/`)

Tests that should pass on **both** `gul_stable` and `gul_nightly`.

### sanity_check.mn

Basic compilation verification.

```gul
@fn mn:
    let x = 10
    print("Sanity Check passed")
    print(x)
    
    if x > 5:
        print("x is greater than 5")
    
    var i = 0
    while i < 3:
        print(i)
        i = i + 1
```

**Expected Output:**

```
Sanity Check passed
10
x is greater than 5
0
1
2
```

### control_flow.mn

Tests conditionals and loops.

- `if/else` statements
- `while` loops

### collections.mn

Tests collection operations.

- List creation and indexing
- `push()` method
- `.len` property

### functions.mn

Tests function definitions and calls.

- Function parameters
- Return values
- Multiple function calls

---

## Nightly Tests (`compilers/nightly/tests/`)

Comprehensive test suite for `gul_nightly` featuring 21 test files.

| Test File | Feature Tested |
|-----------|----------------|
| `test_arrays.gul` | Array creation, indexing |
| `test_arrays_str.gul` | String arrays |
| `test_array_set.gul` | Array set operations |
| `test_completion.gul` | Boolean comparisons |
| `test_control_flow.gul` | If/else, while |
| `test_dict.gul` | Dictionary operations |
| `test_dict_add.gul` | Dictionary add method |
| `test_floats.gul` | Float literals and math |
| `test_fstring.gul` | F-string interpolation |
| `test_list_insert.gul` | List insert method |
| `test_list_methods.gul` | List pop/push |
| `test_list_pop.gul` | List pop method |
| `test_list_remove.gul` | List remove method |
| `test_match.gul` | Match expressions |
| `test_missing_methods.gul` | Edge case handling |
| `test_multidim.gul` | Multi-dimensional arrays |
| `test_ownership.gul` | Ownership semantics |
| `test_sets.gul` | Set operations |
| `test_string.gul` | String operations |
| `test_tuple.gul` | Tuple support |

### Running Nightly Tests

```bash
cd compilers/nightly
python3 tests/run_tests.py
```

---

## CI Integration

### GitHub Actions

Tests run automatically via `.github/workflows/ci.yml`:

```yaml
jobs:
  compilers:
    # Build and test both compilers
    
  sanity-tests:
    # Run shared sanity tests
    
  nightly-integration:
    # Run full nightly test suite
```

### Local Testing with `act`

```bash
# List available jobs
./bin/act -l

# Run compiler tests (dry run)
./bin/act -W .github/workflows/ci.yml -j compilers --dryrun

# Run compiler tests (actual)
./bin/act -W .github/workflows/ci.yml -j compilers
```

---

## Writing Tests

### Test File Format

```gul
@fn mn:
    # Test code here
    print(result)

# EXPECT: expected_output
```

### Naming Convention

- `test_<feature>.gul` - Feature-specific test
- `test_<feature>_<subfeature>.gul` - Sub-feature test

### Adding New Tests

1. Create test file in appropriate location
2. Include `# EXPECT:` comments for expected output
3. Add to `run_tests.py` if needed for nightly
4. Update this document
