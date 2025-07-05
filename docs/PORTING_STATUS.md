# Remaining Work for Rust Port

The legacy Go library under `pkg/` exposed a large API surface. The current `demoinfocs-rs` crate only implements a subset. This document lists missing features and modules that still need to be implemented to achieve feature parity.

## Core Parser
- [x] **Source 1 demo support** – implement datatable and string table parsing similar to `pkg/demoinfocs/datatables.go` and `stringtables.go`.
- [x] **String tables** – decode `svc_CreateStringTable` and `svc_UpdateStringTable` messages and expose APIs for consumers. Use `parser.string_table(name)` to access tables and `parser.register_on_string_table` for update callbacks.
- [ ] **Net message handling** – map all message types from `net_messages.go` and expose registration callbacks.
- [ ] **Encrypted net messages** – port decryption helpers and error handling for encrypted messages.
- [ ] **Parser configuration** – complete all options found in the Go `ParserConfig`.
- [ ] **Mock parser** – reimplement the `fake` package for unit testing.

## Game State and Entities
- [ ] **Complete entity tracking** – add Source 1 entity tables and finish the Source 2 implementation (projectile ownership, dropped weapons, etc.).
- [ ] **Full `Player` API** – port remaining helper methods (`IsInBombZone`, `IsDucking`, `IsScoped`, `IsSpottedBy`, etc.).
- [ ] **Inferno and grenade helpers** – replicate convex hull calculations and trajectory tracking from `inferno.go` and `grenade.go`.
- [ ] **Game rules and match info** – implement the structures and callbacks from `gamerules.go` and `matchinfo.go`.
- [ ] **String table based equipment mapping** – parse item definitions for accurate equipment types.

## Events and Messages
- [ ] **All game events** – many event structs exist but not every event from `game_events.go` is decoded. Ensure every event descriptor is represented and dispatched.
- [ ] **All user messages** – only a handful of `Cstrike15UserMessages` variants are currently handled. Implement decoding for the remaining messages generated from the protobuf definitions.
- [ ] **Round backup and restore** – support messages such as `CS_UM_RoundBackupFilenames` and `CS_UM_RoundImpactScoreData` with full data models.

## Examples and Utilities
- [ ] **Voice capture example** – finish the example in `examples/voice-capture` once voice data parsing is implemented.
- [ ] **WebAssembly bindings** – port the old WASM example and ensure the crate builds for `wasm32-unknown-unknown`.
- [ ] **Parallel processing** – reintroduce the parallel parsing utilities for batch processing multiple demos.
- [ ] **Command helpers** – port the `s2_commands.go` helpers for crafting demo commands.

This checklist is meant as guidance for achieving feature parity with the old Go library. As functionality is added, update this document to track remaining work.
