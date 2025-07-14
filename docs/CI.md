# Continuous Integration (CI)

This project uses GitHub Actions to automatically verify code quality on every push and pull request to the `develop` and `main` branches.

## What the CI checks

- **Code formatting:** Ensures the code follows Rust’s formatting standards using `cargo fmt -- --check`.
- **Linting:** Runs `cargo clippy` to catch common mistakes and enforce coding best practices.
- **Build:** Compiles the project with `cargo build` to ensure no compilation errors.
- **Testing:** Runs all tests with `cargo test` to verify correctness.

## When does CI run?

- On every push to the `develop` and `main` branches.
- On every pull request targeting `develop` or `main`.

## How to run checks locally

Before pushing your code, it’s recommended to run these commands locally to catch issues early:

```bash
cargo fmt -- --check        # Check formatting
cargo clippy -- -D warnings # Run linter and treat warnings as errors
cargo build                 # Build the project
cargo test                  # Run tests