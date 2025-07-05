# Remaining Work for Rust Port

The legacy Go library under `pkg/` exposed a large API surface. The current `demoinfocs-rs` crate only implements a subset. This document lists missing features and modules that still need to be implemented to achieve feature parity.

## Core Parser
- [x] **Source 1 demo support** – implement datatable and string table parsing similar to `pkg/demoinfocs/datatables.go` and `stringtables.go`.
- [x] **String tables** – decode `svc_CreateStringTable` and `svc_UpdateStringTable` messages and expose APIs for consumers. Use `parser.string_table(name)` to access tables and `parser.register_on_string_table` for update callbacks.
- [x] **Net message handling** – map all message types from `net_messages.go` and expose registration callbacks.
- [x] **Encrypted net messages** – initial decryption helpers and error handling implemented.
- [x] **Parser configuration** – complete all options found in the Go `ParserConfig`.
- [x] **Mock parser** – reimplement the `fake` package for unit testing.
  New options include skipping warnings for missing decryption keys and overriding the tick rate.

## Game State and Entities
- [x] **Complete entity tracking** – Source 1 entity tables are available and basic projectile ownership and dropped weapon tracking works for Source 2 demos.
- [x] **Full `Player` API** – port remaining helper methods (`IsInBombZone`, `IsDucking`, `IsScoped`, `IsSpottedBy`, etc.).
- [x] **Game rules and match info** – implement the structures and callbacks from `gamerules.go` and `matchinfo.go`.
- [x] **Inferno and grenade helpers** – replicate convex hull calculations and trajectory tracking from `inferno.go` and `grenade.go`.
- [x] **String table based equipment mapping** – parse item definitions for accurate equipment types.

## Events and Messages
- [ ] **All game events** – many event structs exist but not every event from `game_events.go` is decoded. Ensure every event descriptor is represented and dispatched.
- [x] **All user messages** – only a handful of `Cstrike15UserMessages` variants were originally handled. The parser now decodes every message generated from the protobuf definitions.
- Additional messages (`CS_UM_VGUIMenu`, `CS_UM_ShowMenu`, `CS_UM_BarTime`, `CS_UM_RoundBackupFilenames`) are now decoded.
- [x] **Round backup and restore** – support messages such as `CS_UM_RoundBackupFilenames` and `CS_UM_RoundImpactScoreData` with full data models.

## Examples and Utilities
- [x] **Voice capture example** – implemented using `Parser::register_net_message_handler`.
  Run `cargo run --example voice_capture -- -demo <demo> -out voice.raw` to dump the raw audio stream.
- [x] **WebAssembly bindings** – port the old WASM example and ensure the crate builds for `wasm32-unknown-unknown`.
- [x] **Parallel processing** – reintroduce the parallel parsing utilities for batch processing multiple demos.
- [ ] **Command helpers** – port the `s2_commands.go` helpers for crafting demo commands.

This checklist is meant as guidance for achieving feature parity with the old Go library. As functionality is added, update this document to track remaining work.
