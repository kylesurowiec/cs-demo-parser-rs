# Debugging Steps for Unexpected EOF Panic

**Latest panic stack trace**

```
thread 'main' panicked at /workspace/cs-demo-parser-rs/src/bitreader.rs:60:31: called `Result::unwrap()` on an `Err` value: Error { kind: UnexpectedEof, message: "failed to fill whole buffer" }
stack backtrace:
   0: <std::sys::backtrace::BacktraceLock::print::DisplayBacktrace as core::fmt::Display>::fmt
   1: core::fmt::write
   2: std::io::Write::write_fmt
   3: std::sys::backtrace::BacktraceLock::print
   4: std::panicking::default_hook::{{closure}}
   5: std::panicking::default_hook
   6: std::panicking::rust_panic_with_hook
   7: std::panicking::begin_panic_handler::{{closure}}
   8: std::sys::backtrace::__rust_end_short_backtrace
   9: __rustc::rust_begin_unwind
  10: core::panicking::panic_fmt
  11: core::result::unwrap_failed
  12: cs_demo_parser::parser::Parser::<R>::parse_to_end
  13: print_events::main
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
