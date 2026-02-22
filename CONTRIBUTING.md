# Contributing

## Prerequisites

- Rust stable toolchain
- `cargo` in PATH

## Local Quality Checks

Run all checks before opening a pull request:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets --all-features
```

## Pull Requests

- Keep PRs focused and small.
- Include tests for behavior changes.
- Update `README.md` when changing CLI behavior or config format.
