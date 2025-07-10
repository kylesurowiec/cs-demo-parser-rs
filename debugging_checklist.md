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
- [ ] Compare the behaviour with other demo files known to work, to determine if the issue is data specific or systemic.
- [ ] Consider validating the data after each read in `BitReader` to detect misaligned reads earlier.
- [ ] Review recent commits for changes to `BitReader` or the parser that may have introduced the issue.
- [ ] Create unit tests for `BitReader::read_int` when reading near EOF to ensure graceful error handling.
- [ ] Research upstream libraries or documentation (e.g., `demoinfocs-golang`) for known quirks in parsing this specific demo format.
- Observed `print_events` example failing with `UnexpectedEndOfDemo` after ~74k frames.
  Need to inspect frame reading logic in `parse_frame_s1` for late-demo cases.
  - Removed `reading_signon` state so the final DEM_Stop command ends parsing.

- [ ] Implement Source 1 packet parsing in `parse_frame_s1` to handle game events like `player_death`. Current implementation skips packet contents, so no kill events are dispatched.
  - New panic in `parse_packet_entities` after implementing initial packet reading. Likely due to incorrect netmessage decoding. Review demoinfocs-golang for proper S1 packet structure.
  - Temporarily skip `SvcPacketEntities` for `HL2DEMO` files to avoid overflow in
    `sendtables2::Parser::parse_packet_entities` until a correct implementation
    is available.
