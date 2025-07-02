use demoinfocs_rs::dispatcher::{Dispatcher, EventDispatcher};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

#[test]
fn test_dispatcher_event() {
    let disp = EventDispatcher::new();
    let count = Arc::new(AtomicUsize::new(0));
    let c = count.clone();
    disp.register_handler::<u32, _>(move |v| {
        assert_eq!(*v, 42);
        c.fetch_add(1, Ordering::SeqCst);
    });
    disp.dispatch(42u32);
    thread::sleep(std::time::Duration::from_millis(10));
    assert_eq!(1, count.load(Ordering::SeqCst));
}
