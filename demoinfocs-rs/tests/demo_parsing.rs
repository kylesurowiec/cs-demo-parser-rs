use demoinfocs_rs::parser::{Parser, ParserError};
use std::fs::File;
use std::io::Cursor;
use std::path::PathBuf;

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../test/cs-demos")
        .join(name)
}

#[test]
fn parse_default_demo() {
    let path = fixture_path("default.dem");
    let file = File::open(&path).expect("failed to open demo");
    let mut parser = Parser::new(file);
    let err = parser.parse_to_end().unwrap_err();
    assert!(matches!(err, ParserError::UnexpectedEndOfDemo));
}

#[test]
fn invalid_file_type() {
    let data = vec![0u8; 2048];
    let mut parser = Parser::new(Cursor::new(data));
    let err = parser.parse_header().expect_err("expected error");
    matches!(err, ParserError::InvalidFileType);
}

#[test]
fn example_print_events_runs() {
    let path = fixture_path("s2/s2.dem");
    let file = File::open(&path).expect("failed to open demo");
    let mut parser = Parser::new(file);
    parser.register_event_handler::<u8, _>(|_| {});
    let err = parser.parse_to_end().unwrap_err();
    assert!(matches!(err, ParserError::UnexpectedEndOfDemo));
}
