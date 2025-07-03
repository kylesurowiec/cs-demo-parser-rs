# cs-demo-parser

This repository contains a Counter‑Strike demo parser that is now **entirely
implemented in Rust**. The previous Go implementation has been removed.

## Building

Build the crate using `cargo` from the repository root and pointing to the
crate's manifest:

```bash
cargo build --manifest-path demoinfocs-rs/Cargo.toml
```

This builds the library and all examples.

## Testing

Run the tests with:

```bash
cargo test --manifest-path demoinfocs-rs/Cargo.toml
```

Running `cargo test` from the repository root fails because there is no
`Cargo.toml` at that location.

## Examples

Several short examples live under `demoinfocs-rs/examples`. Execute any of them
by passing the demo path with the `-demo` flag:

```bash
cargo run --example heatmap -- -demo demo.dem
```

Replace `heatmap` with any other example name such as `nade_trajectories` or
`print_events`.
