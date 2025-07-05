#[path = "mock_parser.rs"]
mod mock_parser;

use mock_parser::MockParser;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

#[test]
fn parse_source1_demo() {
    let mut parser = MockParser::new();
    let hit = Arc::new(AtomicUsize::new(0));
    let h = hit.clone();
    parser.register_event_handler::<u32, _>(move |_| {
        h.fetch_add(1, Ordering::SeqCst);
    });
    parser.feed_event(123u32);
    parser.parse_to_end();
    std::thread::sleep(std::time::Duration::from_millis(10));
    assert_eq!(1, hit.load(Ordering::SeqCst));
}
