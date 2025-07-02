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
