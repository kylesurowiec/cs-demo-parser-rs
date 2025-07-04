# Migrating from the Go API

This repository originally provided a Go library for demo parsing. The new Rust crate exposes a similar API. The following table lists common calls in the Go library and their equivalents in Rust.

| Go API (package `demoinfocs`) | Rust API (`demoinfocs_rs`) |
| ----------------------------- | -------------------------- |
| `demoinfocs.NewParser(r)` | `Parser::new(r)` |
| `Parser.ParseHeader()` | `parser.parse_header()` |
| `Parser.ParseNextFrame()` | `parser.parse_next_frame()` |
| `Parser.ParseToEnd()` | `parser.parse_to_end()` |
| `Parser.RegisterEventHandler(fn)` | `parser.register_event_handler::<T, _>(fn)` |
| `Parser.RegisterNetMessageHandler(fn)` | `parser.register_net_message_handler::<T, _>(fn)` |

Functions that do not have an equivalent yet will be added over time. The overall workflow of creating a parser, registering handlers and parsing frames remains the same.

## Missing features

Several parts of the Go API are not yet implemented in Rust:

* **Parser configuration** – use `Parser::with_config` together with
  `ParserConfig` to customize queue sizes, supply decryption keys or ignore
  certain errors.
* **Entity callbacks** – Go allows registering `OnEntity`/`OnEntityCreated`
  handlers to observe property changes. Rust only exposes a small `GameState`
  snapshot and does not emit entity events yet.

## Examples

Below are short snippets demonstrating common tasks with the Rust API.

Register a handler for all game events and parse the demo to the end:

```rust
use demoinfocs_rs::{events::GenericGameEvent, parser::Parser};
use std::fs::File;

let file = File::open("demo.dem")?;
let mut parser = Parser::new(file);
parser.register_event_handler::<GenericGameEvent, _>(|ev| {
    println!("{} {:?}", ev.name, ev.data);
});
parser.parse_to_end()?;
```

Iterate over players after parsing some frames:

```rust
let players = parser.game_state().participants().by_user_id();
for p in players.values() {
    println!("{}: {}", p.user_id, p.name);
}
```
