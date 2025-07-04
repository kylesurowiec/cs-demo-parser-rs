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
fn dispatch_round_state_events() {
    let mut parser = Parser::new(Cursor::new(Vec::<u8>::new()));

    let final_c = Arc::new(AtomicUsize::new(0));
    let last_half_c = Arc::new(AtomicUsize::new(0));
    let match_point_c = Arc::new(AtomicUsize::new(0));
    let match_start_c = Arc::new(AtomicUsize::new(0));
    let warmup_c = Arc::new(AtomicUsize::new(0));
    let upload_stats_c = Arc::new(AtomicUsize::new(0));
    let mvp_c = Arc::new(AtomicUsize::new(0));
    let freeze_end_c = Arc::new(AtomicUsize::new(0));
    let official_c = Arc::new(AtomicUsize::new(0));

    parser.register_event_handler::<events::RoundAnnounceFinal, _>(move |_| {
        final_c.fetch_add(1, Ordering::SeqCst);
    });
    parser.register_event_handler::<events::RoundAnnounceLastRoundHalf, _>(move |_| {
        last_half_c.fetch_add(1, Ordering::SeqCst);
    });
    parser.register_event_handler::<events::RoundAnnounceMatchPoint, _>(move |_| {
        match_point_c.fetch_add(1, Ordering::SeqCst);
    });
    parser.register_event_handler::<events::RoundAnnounceMatchStart, _>(move |_| {
        match_start_c.fetch_add(1, Ordering::SeqCst);
    });
    parser.register_event_handler::<events::RoundAnnounceWarmup, _>(move |_| {
        warmup_c.fetch_add(1, Ordering::SeqCst);
    });
    parser.register_event_handler::<events::RoundEndUploadStats, _>(move |_| {
        upload_stats_c.fetch_add(1, Ordering::SeqCst);
    });
    parser.register_event_handler::<events::RoundMVPAnnouncement, _>(move |_| {
        mvp_c.fetch_add(1, Ordering::SeqCst);
    });
    parser.register_event_handler::<events::RoundFreezetimeEnd, _>(move |_| {
        freeze_end_c.fetch_add(1, Ordering::SeqCst);
    });
    parser.register_event_handler::<events::RoundEndOfficial, _>(move |_| {
        official_c.fetch_add(1, Ordering::SeqCst);
    });

    let list = msg::CsvcMsgGameEventList {
        descriptors: vec![
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(1),
                name: Some("round_announce_final".into()),
                keys: vec![],
            },
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(2),
                name: Some("round_announce_last_round_half".into()),
                keys: vec![],
            },
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(3),
                name: Some("round_announce_match_point".into()),
                keys: vec![],
            },
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(4),
                name: Some("round_announce_match_start".into()),
                keys: vec![],
            },
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(5),
                name: Some("round_announce_warmup".into()),
                keys: vec![],
            },
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(6),
                name: Some("round_end_upload_stats".into()),
                keys: vec![],
            },
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(7),
                name: Some("round_mvp".into()),
                keys: vec![],
            },
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(8),
                name: Some("round_freeze_end".into()),
                keys: vec![],
            },
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(9),
                name: Some("round_officially_ended".into()),
                keys: vec![],
            },
        ],
    };
    parser.on_game_event_list(&list);

    for id in 1..=9 {
        parser.on_game_event(&msg::CsvcMsgGameEvent {
            event_name: None,
            eventid: Some(id),
            keys: vec![],
            passthrough: None,
        });
    }

    thread::sleep(std::time::Duration::from_millis(20));

    assert!(final_c.load(Ordering::SeqCst) >= 1);
    assert!(last_half_c.load(Ordering::SeqCst) >= 1);
    assert!(match_point_c.load(Ordering::SeqCst) >= 1);
    assert!(match_start_c.load(Ordering::SeqCst) >= 1);
    assert!(warmup_c.load(Ordering::SeqCst) >= 1);
    assert!(upload_stats_c.load(Ordering::SeqCst) >= 1);
    assert!(mvp_c.load(Ordering::SeqCst) >= 1);
    assert!(freeze_end_c.load(Ordering::SeqCst) >= 1);
    assert!(official_c.load(Ordering::SeqCst) >= 1);
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

    parser.register_event_handler::<events::BombPlantBegin, _>(move |_| {
        beginplant.fetch_add(1, Ordering::SeqCst);
    });
    parser.register_event_handler::<events::BombDefuseStart, _>(move |_| {
        begindefuse.fetch_add(1, Ordering::SeqCst);
    });
    parser.register_event_handler::<events::BombDefused, _>(move |_| {
        defused.fetch_add(1, Ordering::SeqCst);
    });
    parser.register_event_handler::<events::BombExplode, _>(move |_| {
        exploded.fetch_add(1, Ordering::SeqCst);
    });
    parser.register_event_handler::<events::BombDropped, _>(move |_| {
        dropped.fetch_add(1, Ordering::SeqCst);
    });
    parser.register_event_handler::<events::BombPickup, _>(move |_| {
        pickup.fetch_add(1, Ordering::SeqCst);
    });
    parser.register_event_handler::<events::BombPlanted, _>(move |_| {
        planted.fetch_add(1, Ordering::SeqCst);
    });
    parser.register_event_handler::<events::BombBeep, _>(move |_| {
        beep.fetch_add(1, Ordering::SeqCst);
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
