# cs-demo-parser

## Running the Rust tests

The Rust portion of this repository lives under `demoinfocs-rs`. To run its
tests, invoke `cargo` with an explicit manifest path:

```bash
cargo test --manifest-path demoinfocs-rs/Cargo.toml
```

Running `cargo test` from the repository root fails because there is no
`Cargo.toml` at that location.


## Running the Rust examples

Several short examples live under `demoinfocs-rs/examples`. Run them with
`cargo run` passing the demo path via the `-demo` flag:

```bash
cargo run --example heatmap -- -demo demo.dem
```

Replace `heatmap` with any of the other example names such as
`nade_trajectories` or `print_events`.
