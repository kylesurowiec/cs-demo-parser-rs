#[path = "mock_parser.rs"]
mod mock_parser;

use demoinfocs_rs::parser::{Parser, ParserError};
use mock_parser::MockParser;
use std::io::Cursor;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

#[test]
fn parse_mock_demo() {
    let mut parser = MockParser::new();
    let ev_cnt = Arc::new(AtomicUsize::new(0));
    let ev_c = ev_cnt.clone();
    parser.register_event_handler::<u8, _>(move |v| {
        ev_c.fetch_add(*v as usize, Ordering::SeqCst);
    });
    let msg_cnt = Arc::new(AtomicUsize::new(0));
    let msg_c = msg_cnt.clone();
    parser.register_net_message_handler::<u16, _>(move |m| {
        msg_c.fetch_add(*m as usize, Ordering::SeqCst);
    });
    parser.feed_event(1u8);
    parser.feed_net_message(2u16);
    parser.parse_to_end();
    std::thread::sleep(std::time::Duration::from_millis(10));
    assert_eq!(1, ev_cnt.load(Ordering::SeqCst));
    assert_eq!(2, msg_cnt.load(Ordering::SeqCst));
}

#[test]
fn invalid_file_type() {
    let data = vec![0u8; 2048];
    let mut parser = Parser::new(Cursor::new(data));
    let err = parser.parse_header().expect_err("expected error");
    matches!(err, ParserError::InvalidFileType);
}

#[test]
fn git_lfs_pointer() {
    let data = b"version https://git-lfs.github.com/spec/v1\n".to_vec();
    let mut parser = Parser::new(Cursor::new(data));
    let err = parser.parse_header().expect_err("expected error");
    matches!(err, ParserError::GitLfsPointer);
}

#[test]
fn example_print_events_runs() {
    let mut parser = MockParser::new();
    let called = Arc::new(AtomicUsize::new(0));
    let c = called.clone();
    parser.register_event_handler::<u8, _>(move |_| {
        c.fetch_add(1, Ordering::SeqCst);
    });
    parser.feed_event(42u8);
    parser.parse_to_end();
    std::thread::sleep(std::time::Duration::from_millis(10));
    assert_eq!(1, called.load(Ordering::SeqCst));
}
