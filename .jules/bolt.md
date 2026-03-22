## 2024-05-19 - [Avoid Redundant String Allocations with HashSet<&str>]
**Learning:** During symbol extraction (`detect_missing_symbols`), allocating a new `String` directly and returning it often results in duplicate instances of the same string, creating unnecessary heap pressure. Directly converting it via `std::collections::HashSet::new()` and then collecting introduces non-deterministic order output (flaky tests risk).
**Action:** When extracting multiple distinct strings, preserve iteration order and prevent redundant `to_string()` allocations by tracking `&str` references in a `HashSet<&str>` using `seen.insert(word)` as a conditional guard before inserting elements into a traditional `Vec`.

## 2026-03-22 - [Avoid Redundant String Allocations in Hot Loops with HashMap::entry]
**Learning:** Using `.entry(key.clone()).or_insert()` inside frequent loops allocates a `String` on every iteration, bypassing the benefits of a hash map lookup.
**Action:** When working with frequent lookups in hot loops, use `get_mut()` or `contains_key()` to avoid cloning keys unless an insertion is actually needed.
