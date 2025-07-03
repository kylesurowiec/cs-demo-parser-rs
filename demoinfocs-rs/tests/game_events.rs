use demoinfocs_rs::events;
use demoinfocs_rs::parser::Parser;
use demoinfocs_rs::proto::msg::all as msg;
use std::io::Cursor;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

#[test]
fn dispatch_basic_game_events() {
    let mut parser = Parser::new(Cursor::new(Vec::<u8>::new()));

    let ms = Arc::new(AtomicUsize::new(0));
    let rs = Arc::new(AtomicUsize::new(0));
    let re = Arc::new(AtomicUsize::new(0));

    let ms_c = ms.clone();
    parser.register_event_handler::<events::MatchStart, _>(move |_| {
        ms_c.fetch_add(1, Ordering::SeqCst);
    });
    let rs_c = rs.clone();
    parser.register_event_handler::<events::RoundStart, _>(move |_| {
        rs_c.fetch_add(1, Ordering::SeqCst);
    });
    let re_c = re.clone();
    parser.register_event_handler::<events::RoundEnd, _>(move |_| {
        re_c.fetch_add(1, Ordering::SeqCst);
    });

    let list = msg::CsvcMsgGameEventList {
        descriptors: vec![
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(1),
                name: Some("begin_new_match".into()),
                keys: vec![],
            },
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(2),
                name: Some("round_start".into()),
                keys: vec![],
            },
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(3),
                name: Some("round_end".into()),
                keys: vec![],
            },
        ],
    };
    parser.on_game_event_list(&list);

    parser.on_game_event(&msg::CsvcMsgGameEvent {
        event_name: None,
        eventid: Some(1),
        keys: vec![],
        passthrough: None,
    });
    parser.on_game_event(&msg::CsvcMsgGameEvent {
        event_name: None,
        eventid: Some(2),
        keys: vec![],
        passthrough: None,
    });
    parser.on_game_event(&msg::CsvcMsgGameEvent {
        event_name: None,
        eventid: Some(3),
        keys: vec![],
        passthrough: None,
    });

    thread::sleep(std::time::Duration::from_millis(20));

    assert!(ms.load(Ordering::SeqCst) >= 1);
    assert!(rs.load(Ordering::SeqCst) >= 1);
    assert!(re.load(Ordering::SeqCst) >= 1);
}
