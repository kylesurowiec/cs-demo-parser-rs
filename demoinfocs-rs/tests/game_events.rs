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
fn dispatch_equipment_and_server_events() {
    let mut parser = Parser::new(Cursor::new(Vec::<u8>::new()));

    let ammo = Arc::new(AtomicUsize::new(0));
    let equip = Arc::new(AtomicUsize::new(0));
    let pickup = Arc::new(AtomicUsize::new(0));
    let slerp = Arc::new(AtomicUsize::new(0));
    let remove = Arc::new(AtomicUsize::new(0));
    let inspect = Arc::new(AtomicUsize::new(0));
    let cvar = Arc::new(AtomicUsize::new(0));
    let vote = Arc::new(AtomicUsize::new(0));
    let reward = Arc::new(AtomicUsize::new(0));

    let c = ammo.clone();
    parser.register_event_handler::<events::AmmoPickup, _>(move |_| {
        c.fetch_add(1, Ordering::SeqCst);
    });
    let c = equip.clone();
    parser.register_event_handler::<events::ItemEquip, _>(move |_| {
        c.fetch_add(1, Ordering::SeqCst);
    });
    let c = pickup.clone();
    parser.register_event_handler::<events::ItemPickup, _>(move |_| {
        c.fetch_add(1, Ordering::SeqCst);
    });
    let c = slerp.clone();
    parser.register_event_handler::<events::ItemPickupSlerp, _>(move |_| {
        c.fetch_add(1, Ordering::SeqCst);
    });
    let c = remove.clone();
    parser.register_event_handler::<events::ItemRemove, _>(move |_| {
        c.fetch_add(1, Ordering::SeqCst);
    });
    let c = inspect.clone();
    parser.register_event_handler::<events::InspectWeapon, _>(move |_| {
        c.fetch_add(1, Ordering::SeqCst);
    });
    let c = cvar.clone();
    parser.register_event_handler::<events::ServerCvar, _>(move |_| {
        c.fetch_add(1, Ordering::SeqCst);
    });
    let c = vote.clone();
    parser.register_event_handler::<events::VoteCast, _>(move |_| {
        c.fetch_add(1, Ordering::SeqCst);
    });
    let c = reward.clone();
    parser.register_event_handler::<events::TournamentReward, _>(move |_| {
        c.fetch_add(1, Ordering::SeqCst);
    });

    let list = msg::CsvcMsgGameEventList {
        descriptors: vec![
            msg::csvc_msg_game_event_list::DescriptorT { eventid: Some(1), name: Some("ammo_pickup".into()), keys: vec![] },
            msg::csvc_msg_game_event_list::DescriptorT { eventid: Some(2), name: Some("item_equip".into()), keys: vec![] },
            msg::csvc_msg_game_event_list::DescriptorT { eventid: Some(3), name: Some("item_pickup".into()), keys: vec![] },
            msg::csvc_msg_game_event_list::DescriptorT { eventid: Some(4), name: Some("item_pickup_slerp".into()), keys: vec![] },
            msg::csvc_msg_game_event_list::DescriptorT { eventid: Some(5), name: Some("item_remove".into()), keys: vec![] },
            msg::csvc_msg_game_event_list::DescriptorT { eventid: Some(6), name: Some("inspect_weapon".into()), keys: vec![] },
            msg::csvc_msg_game_event_list::DescriptorT { eventid: Some(7), name: Some("server_cvar".into()), keys: vec![] },
            msg::csvc_msg_game_event_list::DescriptorT { eventid: Some(8), name: Some("vote_cast".into()), keys: vec![] },
            msg::csvc_msg_game_event_list::DescriptorT { eventid: Some(9), name: Some("tournament_reward".into()), keys: vec![] },
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

    assert!(ammo.load(Ordering::SeqCst) >= 1);
    assert!(equip.load(Ordering::SeqCst) >= 1);
    assert!(pickup.load(Ordering::SeqCst) >= 1);
    assert!(slerp.load(Ordering::SeqCst) >= 1);
    assert!(remove.load(Ordering::SeqCst) >= 1);
    assert!(inspect.load(Ordering::SeqCst) >= 1);
    assert!(cvar.load(Ordering::SeqCst) >= 1);
    assert!(vote.load(Ordering::SeqCst) >= 1);
    assert!(reward.load(Ordering::SeqCst) >= 1);
}
