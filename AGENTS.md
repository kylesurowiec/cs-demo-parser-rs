# Repository Contribution Guidelines

## Testing
Run all Rust tests from the repository root:

```bash
cargo test --manifest-path demoinfocs-rs/Cargo.toml
```

## Style Checks
Verify formatting and run lints before committing:

```bash
cargo fmt -- --check
cargo clippy
```

## Deprecated Go Code
The old Go implementation inside `pkg/` and various example folders is no longer maintained. Only modify the Rust crate in `demoinfocs-rs`.

## Continuous Integration
CI builds the crate, checks formatting, runs clippy and tests. It also expects the examples under `demoinfocs-rs/examples` to compile.
