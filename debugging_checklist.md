# Debugging Steps for Unexpected EOF Panic

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
- [ ] Add logging around `Parser::parse_next_frame` to identify which frame causes the EOF and what command was expected.
- [ ] Compare the behaviour with other demo files known to work, to determine if the issue is data specific or systemic.
- [ ] Consider validating the data after each read in `BitReader` to detect misaligned reads earlier.
- [ ] Review recent commits for changes to `BitReader` or the parser that may have introduced the issue.
- [ ] Create unit tests for `BitReader::read_int` when reading near EOF to ensure graceful error handling.
- [ ] Research upstream libraries or documentation (e.g., `demoinfocs-golang`) for known quirks in parsing this specific demo format.

