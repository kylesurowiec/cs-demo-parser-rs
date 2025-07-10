# Debugging Steps for Unexpected EOF Panic

**Latest panic stack trace**

```
thread 'main' panicked at src/sendtables2/mod.rs:161:28:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::panicking::panic_const::panic_const_add_overflow
   3: cs_demo_parser::sendtables2::Parser::parse_packet_entities
   4: cs_demo_parser::parser::Parser::<R>::handle_svc_message
   5: cs_demo_parser::parser::Parser::<R>::parse_packet_s1
   6: cs_demo_parser::parser::Parser::<R>::parse_frame_s1
   7: cs_demo_parser::parser::Parser::<R>::parse_next_frame::{{closure}}
   8: core::ops::function::FnOnce::call_once
   9: <core::panic::unwind_safe::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
  10: std::panicking::try::do_call
  11: __rust_try
  12: std::panicking::try
  13: std::panic::catch_unwind
  14: cs_demo_parser::parser::Parser::<R>::parse_next_frame
  15: debug_dump::main
  16: core::ops::function::FnOnce::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
Compile error trying to add scoreboard: `Player` struct has no `name` field, see `error[E0609]`.

The `debug_dump` example panics with `UnexpectedEof` in `bitreader.rs` when reading the demo file. The following checklist outlines investigation steps to resolve the issue.

- [x] Re-run `cargo run --example debug_dump` with `RUST_BACKTRACE=1` to capture a full stack trace of the panic.
  - Panic originates from `BitReader::read_int` when EOF is reached.
- [x] Validate that `BitReader` correctly handles EOF by inspecting `bitreader.rs` around `read_int` and verifying return values instead of unwrapping.
  - Added `catch_unwind` around header parsing so EOF now returns `UnexpectedEndOfDemo` instead of panicking.
- [x] Investigate whether the demo file is truncated or corrupted by checking its size against the header values (playback frames, signon length, lump table sizes).
- The demo files in `demos/` are only ~130 bytes and start with the text "version". They are Git LFS pointer files, so the real demos were not downloaded. Run `git lfs pull` to fetch them before continuing.
  - `git lfs pull` fails because no remote is configured. Need to copy real demo files manually for future runs.
- [x] Examine the lump table parsing in `debug_dump.rs` and compare with the official Valve demo specification to ensure offsets are computed correctly.
  - Source 1 demos include a lump table but each entry is only four `u32` values.
    The previous code assumed eight values and consumed too many bytes, offsetting
    subsequent frame reads. Updated both `lumps.rs` and `debug_dump.rs` to read
    the correct format and only parse lumps for PBDEMS2 demos.
- [x] Remove premature skipping of `signon_length` bytes in `Parser::parse_header` and set `reading_signon` accordingly.
  - Header parsing no longer consumes signon data, preventing misaligned frame reads.
- [x] Correct packet header skip length in `parse_packet_s1`.
  - Source 1 packets include 160 **bytes** of header data. The code should skip
    `(152 + 4 + 4) * 8` bits. Setting it to just `160` bits caused the reader to
    fall out of sync and trigger EOF errors.
- [x] Skip lump parsing in `debug_dump` for HL2DEMO demos to avoid panicking on unexpected magic.
  - Lumps are only present in PBDEMS2 demos. Source 1 demos should ignore the table entirely.
- [x] Add logging around `Parser::parse_next_frame` to identify which frame causes the EOF and what command was expected.
  - Inserted `println!` calls in `parse_next_frame`, `parse_frame_s1` and
    `parse_frame_s2` to print the frame index and command type during parsing.
 - [x] Implement Source 1 packet parsing in `parse_frame_s1` to handle game events like `player_death`. Kill events are now dispatched.
  - New panic in `parse_packet_entities` after implementing initial packet reading. Likely due to incorrect netmessage decoding. Review demoinfocs-golang for proper S1 packet structure.
  - Temporarily skip `SvcPacketEntities` for `HL2DEMO` files to avoid overflow in
    `sendtables2::Parser::parse_packet_entities` until a correct implementation
    is available.

Debug notes:
- `debug_dump` runs successfully with the full demo path argument (no `-demo`).
- `collect_kills` works on `2015-08-23-ESLOneCologne2015-fnatic-vs-virtuspro-mirage.dem` and reports 154 kills.
- Player names are not decoded from events yet, so generating a scoreboard from
  kill events isn't possible. Need to parse `player_info` data or string tables
  in future iterations.
- Decoding of `CSVCMsg_GameEvent` fields still missing. Kill events contain no player IDs or names, leaving scoreboard empty. Implement descriptor lookup and key parsing in `GameEventHandler` to fill event structs in the next iteration.
- Attempted to build scoreboard by parsing kill events, but compile error: Player struct has no name field.
