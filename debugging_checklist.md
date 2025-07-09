# Debugging Steps for Unexpected EOF Panic

The `debug_dump` example panics with `UnexpectedEof` in `bitreader.rs` when reading the demo file. The following checklist outlines investigation steps to resolve the issue.

- [ ] Re-run `cargo run --example debug_dump` with `RUST_BACKTRACE=1` to capture a full stack trace of the panic.
- [ ] Validate that `BitReader` correctly handles EOF by inspecting `bitreader.rs` around `read_int` and verifying return values instead of unwrapping.
- [ ] Investigate whether the demo file is truncated or corrupted by checking its size against the header values (playback frames, signon length, lump table sizes).
- [ ] Examine the lump table parsing in `debug_dump.rs` and compare with the official Valve demo specification to ensure offsets are computed correctly.
- [ ] Add logging around `Parser::parse_next_frame` to identify which frame causes the EOF and what command was expected.
- [ ] Compare the behaviour with other demo files known to work, to determine if the issue is data specific or systemic.
- [ ] Consider validating the data after each read in `BitReader` to detect misaligned reads earlier.
- [ ] Review recent commits for changes to `BitReader` or the parser that may have introduced the issue.
- [ ] Create unit tests for `BitReader::read_int` when reading near EOF to ensure graceful error handling.
- [ ] Research upstream libraries or documentation (e.g., `demoinfocs-golang`) for known quirks in parsing this specific demo format.

