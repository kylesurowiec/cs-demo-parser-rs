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
fn dispatch_player_events() {
    let mut parser = Parser::new(Cursor::new(Vec::<u8>::new()));

    let connect = Arc::new(AtomicUsize::new(0));
    let disconnect = Arc::new(AtomicUsize::new(0));
    let name_change = Arc::new(AtomicUsize::new(0));
    let spawn = Arc::new(AtomicUsize::new(0));
    let spawned = Arc::new(AtomicUsize::new(0));
    let team = Arc::new(AtomicUsize::new(0));
    let ping = Arc::new(AtomicUsize::new(0));
    let ping_stop = Arc::new(AtomicUsize::new(0));
    let fall_damage = Arc::new(AtomicUsize::new(0));
    let given_c4 = Arc::new(AtomicUsize::new(0));
    let jump = Arc::new(AtomicUsize::new(0));
    let footstep = Arc::new(AtomicUsize::new(0));

    let c = connect.clone();
    parser.register_event_handler::<events::PlayerConnect, _>(move |_| {
        c.fetch_add(1, Ordering::SeqCst);
    });
    let d = disconnect.clone();
    parser.register_event_handler::<events::PlayerDisconnected, _>(move |_| {
        d.fetch_add(1, Ordering::SeqCst);
    });
    let n = name_change.clone();
    parser.register_event_handler::<events::PlayerNameChange, _>(move |_| {
        n.fetch_add(1, Ordering::SeqCst);
    });
    let s = spawn.clone();
    parser.register_event_handler::<events::PlayerSpawn, _>(move |_| {
        s.fetch_add(1, Ordering::SeqCst);
    });
    let sp = spawned.clone();
    parser.register_event_handler::<events::PlayerSpawned, _>(move |_| {
        sp.fetch_add(1, Ordering::SeqCst);
    });
    let t = team.clone();
    parser.register_event_handler::<events::PlayerTeam, _>(move |_| {
        t.fetch_add(1, Ordering::SeqCst);
    });
    let p = ping.clone();
    parser.register_event_handler::<events::PlayerPing, _>(move |_| {
        p.fetch_add(1, Ordering::SeqCst);
    });
    let ps = ping_stop.clone();
    parser.register_event_handler::<events::PlayerPingStop, _>(move |_| {
        ps.fetch_add(1, Ordering::SeqCst);
    });
    let fd = fall_damage.clone();
    parser.register_event_handler::<events::PlayerFallDamage, _>(move |_| {
        fd.fetch_add(1, Ordering::SeqCst);
    });
    let gc4 = given_c4.clone();
    parser.register_event_handler::<events::PlayerGivenC4, _>(move |_| {
        gc4.fetch_add(1, Ordering::SeqCst);
    });
    let j = jump.clone();
    parser.register_event_handler::<events::PlayerJump, _>(move |_| {
        j.fetch_add(1, Ordering::SeqCst);
    });
    let f = footstep.clone();
    parser.register_event_handler::<events::Footstep, _>(move |_| {
        f.fetch_add(1, Ordering::SeqCst);
    });

    let list = msg::CsvcMsgGameEventList {
        descriptors: vec![
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(1),
                name: Some("player_connect".into()),
                keys: vec![],
            },
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(2),
                name: Some("player_connect_full".into()),
                keys: vec![],
            },
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(3),
                name: Some("player_disconnect".into()),
                keys: vec![],
            },
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(4),
                name: Some("player_changename".into()),
                keys: vec![],
            },
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(5),
                name: Some("player_spawn".into()),
                keys: vec![],
            },
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(6),
                name: Some("player_spawned".into()),
                keys: vec![],
            },
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(7),
                name: Some("player_team".into()),
                keys: vec![],
            },
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(8),
                name: Some("player_ping".into()),
                keys: vec![],
            },
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(9),
                name: Some("player_ping_stop".into()),
                keys: vec![],
            },
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(10),
                name: Some("player_falldamage".into()),
                keys: vec![],
            },
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(11),
                name: Some("player_given_c4".into()),
                keys: vec![],
            },
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(12),
                name: Some("player_jump".into()),
                keys: vec![],
            },
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(13),
                name: Some("player_footstep".into()),
                keys: vec![],
            },
        ],
    };
    parser.on_game_event_list(&list);

    for id in 1..=13 {
        parser.on_game_event(&msg::CsvcMsgGameEvent {
            event_name: None,
            eventid: Some(id),
            keys: vec![],
            passthrough: None,
        });
    }

    thread::sleep(std::time::Duration::from_millis(20));

    assert!(connect.load(Ordering::SeqCst) >= 2);
    assert!(disconnect.load(Ordering::SeqCst) >= 1);
    assert!(name_change.load(Ordering::SeqCst) >= 1);
    assert!(spawn.load(Ordering::SeqCst) >= 1);
    assert!(spawned.load(Ordering::SeqCst) >= 1);
    assert!(team.load(Ordering::SeqCst) >= 1);
    assert!(ping.load(Ordering::SeqCst) >= 1);
    assert!(ping_stop.load(Ordering::SeqCst) >= 1);
    assert!(fall_damage.load(Ordering::SeqCst) >= 1);
    assert!(given_c4.load(Ordering::SeqCst) >= 1);
    assert!(jump.load(Ordering::SeqCst) >= 1);
    assert!(footstep.load(Ordering::SeqCst) >= 1);
}
