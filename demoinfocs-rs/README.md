# demoinfocs-rs

`demoinfocs-rs` is an experimental Rust crate for parsing Counter-Strike demo files. It currently lives inside this repository and is not published on crates.io.

## Building

Use Cargo to build the crate from within this directory:

```bash
cargo build
```

## Examples

Several small examples are available under `examples/`. To run one of them, supply the demo path via the `-demo` flag. For example:

```bash
cargo run --example print_events -- -demo /path/to/demo.dem
```

## Tests

Run the unit tests with:

```bash
cargo test
```

