# WebAssembly (WASM)

This example shows how to build the parser for [WebAssembly](https://webassembly.org/) and expose a
simple API. The `parse_demo` function takes the raw demo bytes and returns a list
of player names as a JavaScript value.

## Building

```
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --example web_assembly
```

After compilation run `wasm-bindgen` on the produced `.wasm` file to generate the
JavaScript bindings:

```
wasm-bindgen --target web target/wasm32-unknown-unknown/debug/examples/web_assembly.wasm --out-dir out
```

The complete browser integration is shown in a dedicated repository:
<https://github.com/markus-wa/demoinfocs-wasm>
