use demoinfocs_rs::parser::{Parser, ParserError};
use std::fs::File;
use std::path::PathBuf;

fn legacy_demo_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("demos")
        .join(name)
}

#[test]
fn parse_source1_demo() {
    let path = legacy_demo_path("starseries_ot.dem");
    if let Ok(f) = File::open(&path) {
        let mut parser = Parser::new(f);
        let err = parser.parse_to_end().unwrap_err();
        assert!(matches!(
            err,
            ParserError::UnexpectedEndOfDemo | ParserError::InvalidFileType
        ));
    }
}
