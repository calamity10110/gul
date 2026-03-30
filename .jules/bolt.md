## 2024-05-19 - [Avoid Redundant String Allocations with HashSet<&str>]
**Learning:** During symbol extraction (`detect_missing_symbols`), allocating a new `String` directly and returning it often results in duplicate instances of the same string, creating unnecessary heap pressure. Directly converting it via `std::collections::HashSet::new()` and then collecting introduces non-deterministic order output (flaky tests risk).
**Action:** When extracting multiple distinct strings, preserve iteration order and prevent redundant `to_string()` allocations by tracking `&str` references in a `HashSet<&str>` using `seen.insert(word)` as a conditional guard before inserting elements into a traditional `Vec`.

## 2026-03-22 - [Avoid Redundant String Allocations in Hot Loops with HashMap::entry]
**Learning:** Using `.entry(key.clone()).or_insert()` inside frequent loops allocates a `String` on every iteration, bypassing the benefits of a hash map lookup.
**Action:** When working with frequent lookups in hot loops, use `get_mut()` or `contains_key()` to avoid cloning keys unless an insertion is actually needed.

## 2026-03-29 - [Fold CI/CD Interop Jobs]
**Learning:** Provisioning separate CI runners for tightly related tests (like FFI tests for different languages) forces redundant toolchain downloads, codebase compilations, and dependency fetching, which eats up massive CI execution time.
**Action:** Always fold related CI test jobs into a single pipeline block when they require the exact same pre-requisite build artifacts, drastically reducing redundant CI build overhead.

## 2024-03-24 - CI/CD Pipeline Fragmentation Overhead
**Learning:** Splitting logically related tests and documentation generation steps into multiple dependent or sequential jobs in GitHub Actions causes massive overhead. Each job redundantly provisions a runner, reinstalls system dependencies, fetches toolchains, caches Rust, and performs identical `cargo build --release` steps.
**Action:** Fold related tests and pipeline steps into a single primary job per workflow to eliminate duplicate setup and build times, using simple bash loops instead of matrix strategies when testing fast, related components.

## 2024-05-20 - Folding Matrix-Dependent CI Jobs Safely
**Learning:** When folding fast sequential jobs (like integration tests, docs, and registry publishing) that depend on a natively parallelized matrix job (like test coverage across Python versions), attempting to fold the matrix itself into a bash loop breaks coverage artifact generation and increases test wall-clock time.
**Action:** Preserve matrix strategies for parallel testing tasks that generate versioned artifacts, but aggressively fold all subsequent dependent jobs into a single combined job to eliminate massive redundant runner provisioning overhead.

## 2024-05-21 - [Avoid Format String Allocation in Graph Traversal]
**Learning:** In deep graph traversal (like package dependency resolution), using `format!` to generate a key *before* checking if the node has been visited causes massive redundant heap allocations (O(edges) instead of O(nodes)).
**Action:** Use a `HashSet` containing non-allocating primitive references (like `(&str, &str)` tuples) for cycle detection, and only perform heavy `String` allocations like `format!` when inserting a newly discovered node.

## 2024-05-21 - [Avoid String Lowercasing inside Nested Filtering Loops]
**Learning:** Using `String::to_lowercase()` inside a hot nested loop for string comparisons forces a heap allocation for every permutation, severely degrading search or filter performance.
**Action:** When performing case-insensitive exact string comparisons inside hot loops in Rust, use the non-allocating standard library method `.eq_ignore_ascii_case()` instead.
## 2026-03-29 - [Reuse Release Artifacts in Cargo Test]
**Learning:** Running `cargo test` immediately after a `cargo build --release` causes Cargo to recompile the entire project from scratch in debug mode, destroying the caching benefits of the previous step and drastically increasing CI execution time.
**Action:** In CI workflows, if `cargo test` directly follows a `cargo build --release` step for the same target, always append the `--release` flag (e.g., `cargo test --release`) to ensure Cargo reuses the compiled release artifacts instead of starting a redundant debug build.

## 2024-05-18 - [Eliminate String Allocations in Package Search]
**Learning:** Using `.to_lowercase().contains(&query_lower)` inside hot loops like the package registry's `search` function creates massive performance overhead due to O(N) memory allocations for every checked string field (name, description, keywords).
**Action:** Replace `to_lowercase().contains()` with a custom case-insensitive byte window search using `eq_ignore_ascii_case()` (e.g., `contains_ignore_ascii_case`) to perform zero-allocation substring matching. This eliminates garbage creation and yields a >2x performance improvement.
