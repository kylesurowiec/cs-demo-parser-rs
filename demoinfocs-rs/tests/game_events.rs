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

#[test]
fn dispatch_additional_game_events() {
    let mut parser = Parser::new(Cursor::new(Vec::<u8>::new()));

    let announce_phase_end = Arc::new(AtomicUsize::new(0));
    let buytime_ended = Arc::new(AtomicUsize::new(0));
    let choppers_incoming_warning = Arc::new(AtomicUsize::new(0));
    let cs_intermission = Arc::new(AtomicUsize::new(0));
    let cs_match_end_restart = Arc::new(AtomicUsize::new(0));
    let cs_pre_restart = Arc::new(AtomicUsize::new(0));
    let cs_round_final_beep = Arc::new(AtomicUsize::new(0));
    let cs_round_start_beep = Arc::new(AtomicUsize::new(0));
    let cs_win_panel_match = Arc::new(AtomicUsize::new(0));
    let cs_win_panel_round = Arc::new(AtomicUsize::new(0));
    let enter_bombzone = Arc::new(AtomicUsize::new(0));
    let exit_bombzone = Arc::new(AtomicUsize::new(0));
    let enter_buyzone = Arc::new(AtomicUsize::new(0));
    let exit_buyzone = Arc::new(AtomicUsize::new(0));
    let entity_visible = Arc::new(AtomicUsize::new(0));
    let firstbombs_incoming_warning = Arc::new(AtomicUsize::new(0));
    let hltv_chase = Arc::new(AtomicUsize::new(0));
    let hltv_fixed = Arc::new(AtomicUsize::new(0));
    let hltv_message = Arc::new(AtomicUsize::new(0));
    let hltv_status = Arc::new(AtomicUsize::new(0));
    let hostage_follows = Arc::new(AtomicUsize::new(0));
    let hostname_changed = Arc::new(AtomicUsize::new(0));
    let jointeam_failed = Arc::new(AtomicUsize::new(0));
    let other_death = Arc::new(AtomicUsize::new(0));
    let player_blind = Arc::new(AtomicUsize::new(0));
    let show_survival_respawn_status = Arc::new(AtomicUsize::new(0));
    let survival_paradrop_spawn = Arc::new(AtomicUsize::new(0));
    let switch_team = Arc::new(AtomicUsize::new(0));
    let weapon_fire_on_empty = Arc::new(AtomicUsize::new(0));
    let weapon_zoom = Arc::new(AtomicUsize::new(0));
    let weapon_zoom_rifle = Arc::new(AtomicUsize::new(0));

    let c = announce_phase_end.clone();
    parser.register_event_handler::<events::AnnouncePhaseEnd, _>(move |_| {
        c.fetch_add(1, Ordering::SeqCst);
    });
    let c = buytime_ended.clone();
    parser.register_event_handler::<events::BuytimeEnded, _>(move |_| {
        c.fetch_add(1, Ordering::SeqCst);
    });
    let c = choppers_incoming_warning.clone();
    parser.register_event_handler::<events::ChoppersIncomingWarning, _>(move |_| {
        c.fetch_add(1, Ordering::SeqCst);
    });
    let c = cs_intermission.clone();
    parser.register_event_handler::<events::CsIntermission, _>(move |_| {
        c.fetch_add(1, Ordering::SeqCst);
    });
    let c = cs_match_end_restart.clone();
    parser.register_event_handler::<events::CsMatchEndRestart, _>(move |_| {
        c.fetch_add(1, Ordering::SeqCst);
    });
    let c = cs_pre_restart.clone();
    parser.register_event_handler::<events::CsPreRestart, _>(move |_| {
        c.fetch_add(1, Ordering::SeqCst);
    });
    let c = cs_round_final_beep.clone();
    parser.register_event_handler::<events::CsRoundFinalBeep, _>(move |_| {
        c.fetch_add(1, Ordering::SeqCst);
    });
    let c = cs_round_start_beep.clone();
    parser.register_event_handler::<events::CsRoundStartBeep, _>(move |_| {
        c.fetch_add(1, Ordering::SeqCst);
    });
    let c = cs_win_panel_match.clone();
    parser.register_event_handler::<events::CsWinPanelMatch, _>(move |_| {
        c.fetch_add(1, Ordering::SeqCst);
    });
    let c = cs_win_panel_round.clone();
    parser.register_event_handler::<events::CsWinPanelRound, _>(move |_| {
        c.fetch_add(1, Ordering::SeqCst);
    });
    let c = enter_bombzone.clone();
    parser.register_event_handler::<events::EnterBombzone, _>(move |_| {
        c.fetch_add(1, Ordering::SeqCst);
    });
    let c = exit_bombzone.clone();
    parser.register_event_handler::<events::ExitBombzone, _>(move |_| {
        c.fetch_add(1, Ordering::SeqCst);
    });
    let c = enter_buyzone.clone();
    parser.register_event_handler::<events::EnterBuyzone, _>(move |_| {
        c.fetch_add(1, Ordering::SeqCst);
    });
    let c = exit_buyzone.clone();
    parser.register_event_handler::<events::ExitBuyzone, _>(move |_| {
        c.fetch_add(1, Ordering::SeqCst);
    });
    let c = entity_visible.clone();
    parser.register_event_handler::<events::EntityVisible, _>(move |_| {
        c.fetch_add(1, Ordering::SeqCst);
    });
    let c = firstbombs_incoming_warning.clone();
    parser.register_event_handler::<events::FirstBombsIncomingWarning, _>(move |_| {
        c.fetch_add(1, Ordering::SeqCst);
    });
    let c = hltv_chase.clone();
    parser.register_event_handler::<events::HltvChase, _>(move |_| {
        c.fetch_add(1, Ordering::SeqCst);
    });
    let c = hltv_fixed.clone();
    parser.register_event_handler::<events::HltvFixed, _>(move |_| {
        c.fetch_add(1, Ordering::SeqCst);
    });
    let c = hltv_message.clone();
    parser.register_event_handler::<events::HltvMessage, _>(move |_| {
        c.fetch_add(1, Ordering::SeqCst);
    });
    let c = hltv_status.clone();
    parser.register_event_handler::<events::HltvStatus, _>(move |_| {
        c.fetch_add(1, Ordering::SeqCst);
    });
    let c = hostage_follows.clone();
    parser.register_event_handler::<events::HostageFollows, _>(move |_| {
        c.fetch_add(1, Ordering::SeqCst);
    });
    let c = hostname_changed.clone();
    parser.register_event_handler::<events::HostnameChanged, _>(move |_| {
        c.fetch_add(1, Ordering::SeqCst);
    });
    let c = jointeam_failed.clone();
    parser.register_event_handler::<events::JoinTeamFailed, _>(move |_| {
        c.fetch_add(1, Ordering::SeqCst);
    });
    let c = other_death.clone();
    parser.register_event_handler::<events::OtherDeath, _>(move |_| {
        c.fetch_add(1, Ordering::SeqCst);
    });
    let c = player_blind.clone();
    parser.register_event_handler::<events::PlayerBlind, _>(move |_| {
        c.fetch_add(1, Ordering::SeqCst);
    });
    let c = show_survival_respawn_status.clone();
    parser.register_event_handler::<events::ShowSurvivalRespawnStatus, _>(move |_| {
        c.fetch_add(1, Ordering::SeqCst);
    });
    let c = survival_paradrop_spawn.clone();
    parser.register_event_handler::<events::SurvivalParadropSpawn, _>(move |_| {
        c.fetch_add(1, Ordering::SeqCst);
    });
    let c = switch_team.clone();
    parser.register_event_handler::<events::SwitchTeam, _>(move |_| {
        c.fetch_add(1, Ordering::SeqCst);
    });
    let c = weapon_fire_on_empty.clone();
    parser.register_event_handler::<events::WeaponFireOnEmpty, _>(move |_| {
        c.fetch_add(1, Ordering::SeqCst);
    });
    let c = weapon_zoom.clone();
    parser.register_event_handler::<events::WeaponZoom, _>(move |_| {
        c.fetch_add(1, Ordering::SeqCst);
    });
    let c = weapon_zoom_rifle.clone();
    parser.register_event_handler::<events::WeaponZoomRifle, _>(move |_| {
        c.fetch_add(1, Ordering::SeqCst);
    });

    let list = msg::CsvcMsgGameEventList {
        descriptors: vec![
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(1),
                name: Some("announce_phase_end".into()),
                keys: vec![],
            },
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(2),
                name: Some("buytime_ended".into()),
                keys: vec![],
            },
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(3),
                name: Some("choppers_incoming_warning".into()),
                keys: vec![],
            },
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(4),
                name: Some("cs_intermission".into()),
                keys: vec![],
            },
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(5),
                name: Some("cs_match_end_restart".into()),
                keys: vec![],
            },
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(6),
                name: Some("cs_pre_restart".into()),
                keys: vec![],
            },
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(7),
                name: Some("cs_round_final_beep".into()),
                keys: vec![],
            },
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(8),
                name: Some("cs_round_start_beep".into()),
                keys: vec![],
            },
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(9),
                name: Some("cs_win_panel_match".into()),
                keys: vec![],
            },
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(10),
                name: Some("cs_win_panel_round".into()),
                keys: vec![],
            },
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(11),
                name: Some("enter_bombzone".into()),
                keys: vec![],
            },
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(12),
                name: Some("exit_bombzone".into()),
                keys: vec![],
            },
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(13),
                name: Some("enter_buyzone".into()),
                keys: vec![],
            },
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(14),
                name: Some("exit_buyzone".into()),
                keys: vec![],
            },
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(15),
                name: Some("entity_visible".into()),
                keys: vec![],
            },
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(16),
                name: Some("firstbombs_incoming_warning".into()),
                keys: vec![],
            },
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(17),
                name: Some("hltv_chase".into()),
                keys: vec![],
            },
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(18),
                name: Some("hltv_fixed".into()),
                keys: vec![],
            },
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(19),
                name: Some("hltv_message".into()),
                keys: vec![],
            },
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(20),
                name: Some("hltv_status".into()),
                keys: vec![],
            },
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(21),
                name: Some("hostage_follows".into()),
                keys: vec![],
            },
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(22),
                name: Some("hostname_changed".into()),
                keys: vec![],
            },
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(23),
                name: Some("jointeam_failed".into()),
                keys: vec![],
            },
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(24),
                name: Some("other_death".into()),
                keys: vec![],
            },
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(25),
                name: Some("player_blind".into()),
                keys: vec![],
            },
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(26),
                name: Some("show_survival_respawn_status".into()),
                keys: vec![],
            },
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(27),
                name: Some("survival_paradrop_spawn".into()),
                keys: vec![],
            },
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(28),
                name: Some("switch_team".into()),
                keys: vec![],
            },
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(29),
                name: Some("weapon_fire_on_empty".into()),
                keys: vec![],
            },
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(30),
                name: Some("weapon_zoom".into()),
                keys: vec![],
            },
            msg::csvc_msg_game_event_list::DescriptorT {
                eventid: Some(31),
                name: Some("weapon_zoom_rifle".into()),
                keys: vec![],
            },
        ],
    };
    parser.on_game_event_list(&list);

    for id in 1..=31 {
        parser.on_game_event(&msg::CsvcMsgGameEvent {
            event_name: None,
            eventid: Some(id),
            keys: vec![],
            passthrough: None,
        });
    }

    thread::sleep(std::time::Duration::from_millis(20));

    assert!(announce_phase_end.load(Ordering::SeqCst) >= 1);
    assert!(buytime_ended.load(Ordering::SeqCst) >= 1);
    assert!(choppers_incoming_warning.load(Ordering::SeqCst) >= 1);
    assert!(cs_intermission.load(Ordering::SeqCst) >= 1);
    assert!(cs_match_end_restart.load(Ordering::SeqCst) >= 1);
    assert!(cs_pre_restart.load(Ordering::SeqCst) >= 1);
    assert!(cs_round_final_beep.load(Ordering::SeqCst) >= 1);
    assert!(cs_round_start_beep.load(Ordering::SeqCst) >= 1);
    assert!(cs_win_panel_match.load(Ordering::SeqCst) >= 1);
    assert!(cs_win_panel_round.load(Ordering::SeqCst) >= 1);
    assert!(enter_bombzone.load(Ordering::SeqCst) >= 1);
    assert!(exit_bombzone.load(Ordering::SeqCst) >= 1);
    assert!(enter_buyzone.load(Ordering::SeqCst) >= 1);
    assert!(exit_buyzone.load(Ordering::SeqCst) >= 1);
    assert!(entity_visible.load(Ordering::SeqCst) >= 1);
    assert!(firstbombs_incoming_warning.load(Ordering::SeqCst) >= 1);
    assert!(hltv_chase.load(Ordering::SeqCst) >= 1);
    assert!(hltv_fixed.load(Ordering::SeqCst) >= 1);
    assert!(hltv_message.load(Ordering::SeqCst) >= 1);
    assert!(hltv_status.load(Ordering::SeqCst) >= 1);
    assert!(hostage_follows.load(Ordering::SeqCst) >= 1);
    assert!(hostname_changed.load(Ordering::SeqCst) >= 1);
    assert!(jointeam_failed.load(Ordering::SeqCst) >= 1);
    assert!(other_death.load(Ordering::SeqCst) >= 1);
    assert!(player_blind.load(Ordering::SeqCst) >= 1);
    assert!(show_survival_respawn_status.load(Ordering::SeqCst) >= 1);
    assert!(survival_paradrop_spawn.load(Ordering::SeqCst) >= 1);
    assert!(switch_team.load(Ordering::SeqCst) >= 1);
    assert!(weapon_fire_on_empty.load(Ordering::SeqCst) >= 1);
    assert!(weapon_zoom.load(Ordering::SeqCst) >= 1);
    assert!(weapon_zoom_rifle.load(Ordering::SeqCst) >= 1);
}
