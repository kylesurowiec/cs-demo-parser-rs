# cs-demo-parser
Mostly vibe coded fork of https://github.com/markus-wa/demoinfocs-golang written in Rust.

Goals:
  - 1:1 functionality
  - Investigate speed improvements

## Building

Build the crate using `cargo` from the repository root:

```bash
cargo build
```

This builds the library and all examples.

### Requirements

The build script compiles several protocol buffer files using `protoc`.
Install the protobuf compiler before building. On Debian/Ubuntu systems:

```bash
sudo apt-get install protobuf-compiler libprotobuf-dev
```

The file `google/protobuf/descriptor.proto` is needed for code generation. It is
usually provided by the `libprotobuf-dev` package. If you installed protobuf to
a custom location, set the `PROTOC_INCLUDE` environment variable to the
directory containing this file.

If `PROTOC_INCLUDE` is set, the build script will use it to locate
`google/protobuf/descriptor.proto`. When `prost-build` is built with its
bundled `protoc`, this variable is provided automatically.
Otherwise the build script falls back to common system locations such as
`/usr/include` or `/usr/local/include`. Installing `libprotobuf-dev` on
Debian/Ubuntu provides the descriptor in `/usr/include`.

### Message definitions

The build script compiles two collections of protobuf files.
`proto/msg` holds the legacy Source&nbsp;1 (CS:GO) messages, whereas
`proto/msgs2` contains the Source&nbsp;2 definitions used by CS2 demos.
Both are emitted into `src/proto` as `msg` and `msgs2` modules.

### Demo files

The test suite expects a collection of Counter-Strike demo files located in the
`demos-external` submodule. Fetch them with:

```bash
git submodule update --init
```

If you don't want to download the demos, set the environment variable
`DEMOINFOCS_SKIP_DEMOS=1` before building or running the tests. The build script
also understands `FETCH_LATEST_DEMOS=1` to download any missing archives.

## Testing

Run the tests with:

```bash
cargo test
```

## Examples

Several short examples live under `examples`. Execute any of them
by passing the demo path with the `-demo` flag:

```bash
cargo run --example heatmap -- -demo demo.dem
```

Replace `heatmap` with any other example name such as `nade_trajectories` or
`print_events`.
