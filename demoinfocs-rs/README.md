# demoinfocs-rs

`demoinfocs-rs` is an experimental Rust crate for parsing Counter-Strike demo files. It currently lives inside this repository and is not published on crates.io.

## Building

Use Cargo to build the crate from within this directory:

```bash
cargo build
```

### Requirements

The build script requires `protoc` to generate Rust types from the protocol
buffer definitions. Install it via your package manager, e.g. on Debian/Ubuntu:

```bash
sudo apt-get install protobuf-compiler
```

If the build script cannot locate `google/protobuf/descriptor.proto`, install
`libprotobuf-dev` and set the `PROTOC_INCLUDE` environment variable to the
directory containing that file. When using the system packages on
Debian/Ubuntu, this is `/usr/include` and no environment variable is needed.

## Examples

Several small examples are available under `examples/`. To run one of them, supply the demo path via the `-demo` flag. For example:

```bash
cargo run --example print_events -- -demo /path/to/demo.dem
```

You can adjust queue sizes or provide decryption keys via `ParserConfig`:

```rust
use demoinfocs_rs::parser::{Parser, ParserConfig};
use std::fs::File;

let file = File::open("demo.dem")?;
let config = ParserConfig { decryption_key: Some(b"0123456789ABCDEF".to_vec()), ..Default::default() };
let mut parser = Parser::with_config(file, config);
```

## Tests

Run the unit tests from the repository root with:

```bash
cargo test --manifest-path demoinfocs-rs/Cargo.toml
```

### Demo files

Tests make use of demo files provided through the `demos-external` submodule.
Fetch them with `git submodule update --init` or set
`DEMOINFOCS_SKIP_DEMOS=1` to skip demo setup.

