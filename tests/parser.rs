use demoinfocs_rs::parser::Parser;
use std::io::Cursor;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

#[test]
fn test_parser_handlers() {
    let mut p = Parser::new(Cursor::new(Vec::<u8>::new()));
    let ev_count = Arc::new(AtomicUsize::new(0));
    let ev_c = ev_count.clone();
    p.register_event_handler::<u8, _>(move |v| {
        ev_c.fetch_add(*v as usize, Ordering::SeqCst);
    });

    let msg_count = Arc::new(AtomicUsize::new(0));
    let msg_c = msg_count.clone();
    p.register_net_message_handler::<u8, _>(move |m| {
        msg_c.fetch_add(*m as usize, Ordering::SeqCst);
    });

    p.dispatch_event(1u8);
    p.dispatch_net_message(2u8);
    thread::sleep(std::time::Duration::from_millis(10));
    assert_eq!(1, ev_count.load(Ordering::SeqCst));
    assert_eq!(2, msg_count.load(Ordering::SeqCst));
}
