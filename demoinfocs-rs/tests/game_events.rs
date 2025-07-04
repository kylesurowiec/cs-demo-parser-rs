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

#[test]
fn dispatch_bomb_game_events() {
    let mut parser = Parser::new(Cursor::new(Vec::<u8>::new()));

    let beginplant = Arc::new(AtomicUsize::new(0));
    let begindefuse = Arc::new(AtomicUsize::new(0));
    let defused = Arc::new(AtomicUsize::new(0));
    let exploded = Arc::new(AtomicUsize::new(0));
    let dropped = Arc::new(AtomicUsize::new(0));
    let pickup = Arc::new(AtomicUsize::new(0));
    let planted = Arc::new(AtomicUsize::new(0));
    let beep = Arc::new(AtomicUsize::new(0));

    let c = beginplant.clone();
    parser.register_event_handler::<events::BombPlantBegin, _>(move |_| {
        c.fetch_add(1, Ordering::SeqCst);
    });
    let c = begindefuse.clone();
    parser.register_event_handler::<events::BombDefuseStart, _>(move |_| {
        c.fetch_add(1, Ordering::SeqCst);
    });
    let c = defused.clone();
    parser.register_event_handler::<events::BombDefused, _>(move |_| {
        c.fetch_add(1, Ordering::SeqCst);
    });
    let c = exploded.clone();
    parser.register_event_handler::<events::BombExplode, _>(move |_| {
        c.fetch_add(1, Ordering::SeqCst);
    });
    let c = dropped.clone();
    parser.register_event_handler::<events::BombDropped, _>(move |_| {
        c.fetch_add(1, Ordering::SeqCst);
    });
    let c = pickup.clone();
    parser.register_event_handler::<events::BombPickup, _>(move |_| {
        c.fetch_add(1, Ordering::SeqCst);
    });
    let c = planted.clone();
    parser.register_event_handler::<events::BombPlanted, _>(move |_| {
        c.fetch_add(1, Ordering::SeqCst);
    });
    let c = beep.clone();
    parser.register_event_handler::<events::BombBeep, _>(move |_| {
        c.fetch_add(1, Ordering::SeqCst);
    });

    let list = msg::CsvcMsgGameEventList {
        descriptors: vec![
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(1),
                name: Some("bomb_beginplant".into()),
                keys: vec![],
            },
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(2),
                name: Some("bomb_begindefuse".into()),
                keys: vec![],
            },
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(3),
                name: Some("bomb_defused".into()),
                keys: vec![],
            },
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(4),
                name: Some("bomb_exploded".into()),
                keys: vec![],
            },
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(5),
                name: Some("bomb_dropped".into()),
                keys: vec![],
            },
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(6),
                name: Some("bomb_pickup".into()),
                keys: vec![],
            },
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(7),
                name: Some("bomb_planted".into()),
                keys: vec![],
            },
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(8),
                name: Some("bomb_beep".into()),
                keys: vec![],
            },
        ],
    };
    parser.on_game_event_list(&list);

    for id in 1..=8 {
        parser.on_game_event(&msg::CsvcMsgGameEvent {
            event_name: None,
            eventid: Some(id),
            keys: vec![],
            passthrough: None,
        });
    }

    thread::sleep(std::time::Duration::from_millis(20));

    assert!(beginplant.load(Ordering::SeqCst) >= 1);
    assert!(begindefuse.load(Ordering::SeqCst) >= 1);
    assert!(defused.load(Ordering::SeqCst) >= 1);
    assert!(exploded.load(Ordering::SeqCst) >= 1);
    assert!(dropped.load(Ordering::SeqCst) >= 1);
    assert!(pickup.load(Ordering::SeqCst) >= 1);
    assert!(planted.load(Ordering::SeqCst) >= 1);
    assert!(beep.load(Ordering::SeqCst) >= 1);
}
