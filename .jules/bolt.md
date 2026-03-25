## 2024-05-19 - [Avoid Redundant String Allocations with HashSet<&str>]
**Learning:** During symbol extraction (`detect_missing_symbols`), allocating a new `String` directly and returning it often results in duplicate instances of the same string, creating unnecessary heap pressure. Directly converting it via `std::collections::HashSet::new()` and then collecting introduces non-deterministic order output (flaky tests risk).
**Action:** When extracting multiple distinct strings, preserve iteration order and prevent redundant `to_string()` allocations by tracking `&str` references in a `HashSet<&str>` using `seen.insert(word)` as a conditional guard before inserting elements into a traditional `Vec`.

## 2026-03-22 - [Avoid Redundant String Allocations in Hot Loops with HashMap::entry]
**Learning:** Using `.entry(key.clone()).or_insert()` inside frequent loops allocates a `String` on every iteration, bypassing the benefits of a hash map lookup.
**Action:** When working with frequent lookups in hot loops, use `get_mut()` or `contains_key()` to avoid cloning keys unless an insertion is actually needed.

## $(date +%Y-%m-%d) - [Fold CI/CD Interop Jobs]
**Learning:** Provisioning separate CI runners for tightly related tests (like FFI tests for different languages) forces redundant toolchain downloads, codebase compilations, and dependency fetching, which eats up massive CI execution time.
**Action:** Always fold related CI test jobs into a single pipeline block when they require the exact same pre-requisite build artifacts, drastically reducing redundant CI build overhead.

## 2024-03-24 - CI/CD Pipeline Fragmentation Overhead
**Learning:** Splitting logically related tests and documentation generation steps into multiple dependent or sequential jobs in GitHub Actions causes massive overhead. Each job redundantly provisions a runner, reinstalls system dependencies, fetches toolchains, caches Rust, and performs identical `cargo build --release` steps.
**Action:** Fold related tests and pipeline steps into a single primary job per workflow to eliminate duplicate setup and build times, using simple bash loops instead of matrix strategies when testing fast, related components.

## 2024-05-20 - Folding Matrix-Dependent CI Jobs Safely
**Learning:** When folding fast sequential jobs (like integration tests, docs, and registry publishing) that depend on a natively parallelized matrix job (like test coverage across Python versions), attempting to fold the matrix itself into a bash loop breaks coverage artifact generation and increases test wall-clock time.
**Action:** Preserve matrix strategies for parallel testing tasks that generate versioned artifacts, but aggressively fold all subsequent dependent jobs into a single combined job to eliminate massive redundant runner provisioning overhead.
