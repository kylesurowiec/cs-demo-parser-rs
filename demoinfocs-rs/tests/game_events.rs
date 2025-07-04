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
fn dispatch_grenade_game_events() {
    let mut parser = Parser::new(Cursor::new(Vec::<u8>::new()));

    let flash = Arc::new(AtomicUsize::new(0));
    let he = Arc::new(AtomicUsize::new(0));
    let decoy_start = Arc::new(AtomicUsize::new(0));
    let decoy_end = Arc::new(AtomicUsize::new(0));
    let smoke_start = Arc::new(AtomicUsize::new(0));
    let smoke_end = Arc::new(AtomicUsize::new(0));
    let fire_start = Arc::new(AtomicUsize::new(0));
    let fire_end = Arc::new(AtomicUsize::new(0));

    let flash_c = flash.clone();
    parser.register_event_handler::<events::FlashExplode, _>(move |_| {
        flash_c.fetch_add(1, Ordering::SeqCst);
    });
    let he_c = he.clone();
    parser.register_event_handler::<events::HeExplode, _>(move |_| {
        he_c.fetch_add(1, Ordering::SeqCst);
    });
    let decoy_start_c = decoy_start.clone();
    parser.register_event_handler::<events::DecoyStart, _>(move |_| {
        decoy_start_c.fetch_add(1, Ordering::SeqCst);
    });
    let decoy_end_c = decoy_end.clone();
    parser.register_event_handler::<events::DecoyExpired, _>(move |_| {
        decoy_end_c.fetch_add(1, Ordering::SeqCst);
    });
    let smoke_start_c = smoke_start.clone();
    parser.register_event_handler::<events::SmokeStart, _>(move |_| {
        smoke_start_c.fetch_add(1, Ordering::SeqCst);
    });
    let smoke_end_c = smoke_end.clone();
    parser.register_event_handler::<events::SmokeExpired, _>(move |_| {
        smoke_end_c.fetch_add(1, Ordering::SeqCst);
    });
    let fire_start_c = fire_start.clone();
    parser.register_event_handler::<events::FireGrenadeStart, _>(move |_| {
        fire_start_c.fetch_add(1, Ordering::SeqCst);
    });
    let fire_end_c = fire_end.clone();
    parser.register_event_handler::<events::FireGrenadeExpired, _>(move |_| {
        fire_end_c.fetch_add(1, Ordering::SeqCst);
    });

    let list = msg::CsvcMsgGameEventList {
        descriptors: vec![
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(1),
                name: Some("flashbang_detonate".into()),
                keys: vec![],
            },
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(2),
                name: Some("hegrenade_detonate".into()),
                keys: vec![],
            },
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(3),
                name: Some("decoy_started".into()),
                keys: vec![],
            },
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(4),
                name: Some("decoy_detonate".into()),
                keys: vec![],
            },
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(5),
                name: Some("smokegrenade_detonate".into()),
                keys: vec![],
            },
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(6),
                name: Some("smokegrenade_expired".into()),
                keys: vec![],
            },
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(7),
                name: Some("inferno_startburn".into()),
                keys: vec![],
            },
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(8),
                name: Some("inferno_expire".into()),
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

    assert!(flash.load(Ordering::SeqCst) >= 1);
    assert!(he.load(Ordering::SeqCst) >= 1);
    assert!(decoy_start.load(Ordering::SeqCst) >= 1);
    assert!(decoy_end.load(Ordering::SeqCst) >= 1);
    assert!(smoke_start.load(Ordering::SeqCst) >= 1);
    assert!(smoke_end.load(Ordering::SeqCst) >= 1);
    assert!(fire_start.load(Ordering::SeqCst) >= 1);
    assert!(fire_end.load(Ordering::SeqCst) >= 1);
}
