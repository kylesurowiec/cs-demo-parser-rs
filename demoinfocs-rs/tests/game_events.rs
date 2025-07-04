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

    let c = final_c.clone();
    parser.register_event_handler::<events::RoundAnnounceFinal, _>(move |_| {
        c.fetch_add(1, Ordering::SeqCst);
    });
    let c = last_half_c.clone();
    parser.register_event_handler::<events::RoundAnnounceLastRoundHalf, _>(move |_| {
        c.fetch_add(1, Ordering::SeqCst);
    });
    let c = match_point_c.clone();
    parser.register_event_handler::<events::RoundAnnounceMatchPoint, _>(move |_| {
        c.fetch_add(1, Ordering::SeqCst);
    });
    let c = match_start_c.clone();
    parser.register_event_handler::<events::RoundAnnounceMatchStart, _>(move |_| {
        c.fetch_add(1, Ordering::SeqCst);
    });
    let c = warmup_c.clone();
    parser.register_event_handler::<events::RoundAnnounceWarmup, _>(move |_| {
        c.fetch_add(1, Ordering::SeqCst);
    });
    let c = upload_stats_c.clone();
    parser.register_event_handler::<events::RoundEndUploadStats, _>(move |_| {
        c.fetch_add(1, Ordering::SeqCst);
    });
    let c = mvp_c.clone();
    parser.register_event_handler::<events::RoundMVPAnnouncement, _>(move |_| {
        c.fetch_add(1, Ordering::SeqCst);
    });
    let c = freeze_end_c.clone();
    parser.register_event_handler::<events::RoundFreezetimeEnd, _>(move |_| {
        c.fetch_add(1, Ordering::SeqCst);
    });
    let c = official_c.clone();
    parser.register_event_handler::<events::RoundEndOfficial, _>(move |_| {
        c.fetch_add(1, Ordering::SeqCst);
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
