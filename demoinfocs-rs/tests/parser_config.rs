use demoinfocs_rs::parser::{Parser, ParserConfig};
use std::io::Cursor;

#[test]
fn tick_rate_override_is_used() {
    let cfg = ParserConfig {
        tick_rate_override: Some(64.0),
        ..Default::default()
    };
    let parser = Parser::with_config(Cursor::new(Vec::<u8>::new()), cfg);
    assert_eq!(parser.tick_rate(), 64.0);
    assert_eq!(parser.tick_time(), std::time::Duration::from_secs_f64(1.0 / 64.0));
}

#[test]
fn ignore_missing_decryption_key_flag() {
    let cfg = ParserConfig {
        ignore_missing_decryption_key: true,
        ..Default::default()
    };
    assert!(cfg.ignore_missing_decryption_key);
}
