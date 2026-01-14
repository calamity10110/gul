# GUL v3.2 Collections Runtime API

## Overview

The GUL runtime provides low-level collection primitives implemented in C (`stdlib.c`).
These are called by the compiler's generated code.

## List API (`GulList`)

```c
int64_t gul_list_alloc(int64_t initial_capacity)  // Create list
void    gul_list_free(int64_t list_ptr)           // Free list
int64_t gul_list_len(int64_t list_ptr)            // Get length
void    gul_list_push(int64_t list_ptr, int64_t value)  // Append
int64_t gul_list_pop(int64_t list_ptr)            // Remove last
int64_t gul_list_get(int64_t list_ptr, int64_t idx)     // Get by index
void    gul_list_set(int64_t list_ptr, int64_t idx, int64_t value)  // Set by index
void    gul_list_clear(int64_t list_ptr)          // Clear all
int64_t gul_list_contains(int64_t list_ptr, int64_t value)  // Check membership
void    gul_list_insert_before(int64_t list_ptr, int64_t idx, int64_t value)
void    gul_list_insert_after(int64_t list_ptr, int64_t idx, int64_t value)
void    gul_list_remove(int64_t list_ptr, int64_t idx)  // Remove at index
```

## Dict API (`GulDict`)

```c
int64_t gul_dict_alloc(int64_t capacity)          // Create dict
void    gul_dict_free(int64_t dict_ptr)           // Free dict
int64_t gul_dict_len(int64_t dict_ptr)            // Get length
void    gul_dict_set(int64_t dict_ptr, int64_t key_ptr, int64_t value)  // Set key
int64_t gul_dict_get(int64_t dict_ptr, int64_t key_ptr)  // Get value
int64_t gul_dict_contains(int64_t dict_ptr, int64_t key_ptr)  // Check key
void    gul_dict_remove(int64_t dict_ptr, int64_t key_ptr)  // Remove key
void    gul_dict_clear(int64_t dict_ptr)          // Clear all
```

## Set API (`GulSet`)

```c
int64_t gul_set_alloc(int64_t capacity)           // Create set
void    gul_set_free(int64_t set_ptr)             // Free set
int64_t gul_set_len(int64_t set_ptr)              // Get length
void    gul_set_add(int64_t set_ptr, int64_t value)  // Add value
int64_t gul_set_contains(int64_t set_ptr, int64_t value)  // Check membership
void    gul_set_remove(int64_t set_ptr, int64_t value)  // Remove value
void    gul_set_clear(int64_t set_ptr)            // Clear all
```

## Table API (`GulTable`)

```c
int64_t gul_table_alloc(int64_t col_count, int64_t row_count)  // Create table
void    gul_table_free(int64_t table_ptr)         // Free table
void    gul_table_set_col_name(int64_t table_ptr, int64_t idx, int64_t name_ptr)
void    gul_table_set_row(int64_t table_ptr, int64_t idx, int64_t name_ptr, int64_t values_ptr)
int64_t gul_table_get_cell(int64_t table_ptr, int64_t row_idx, int64_t col_idx)
```

## Memory Model

- All collections are heap-allocated
- Pointers are passed as `int64_t` for ABI compatibility
- Memory must be explicitly freed (no GC)
- Collections resize automatically when capacity is exceeded

## Ownership Model

Each symbol has an `ownership_mode` field:

- `"owned"` - The variable owns the data (default)
- `"borrow"` - Temporary read access
- `"ref"` - Reference to mutable data
- `"move"` - Ownership transferred
- `"kept"` - Retained after transfer

## Version

GUL Runtime v0.14.0-dev (v3.2 Nightly)
