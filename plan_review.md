The problem is that the `.github/workflows/package-testing.yml` script compiles the Rust code with `cargo build --release` but then runs `cargo test --all-features` in line 91. According to the `bolt.md` memory log: "Running `cargo test` immediately after a `cargo build --release` causes Cargo to recompile the entire project from scratch in debug mode, destroying the caching benefits of the previous step and drastically increasing CI execution time." And the action is "always append the `--release` flag (e.g., `cargo test --release`) to ensure Cargo reuses the compiled release artifacts instead of starting a redundant debug build."

So the plan is:
1. Modify `.github/workflows/package-testing.yml` to replace `cargo test --all-features` with `cargo test --all-features --release`
