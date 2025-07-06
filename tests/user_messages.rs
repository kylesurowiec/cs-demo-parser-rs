use cs_demo_parser::parser::Parser;
use cs_demo_parser::proto::msg::cs_demo_parser_rs::*;
use cs_demo_parser::proto::msg::{self};
use prost::Message;
use std::io::Cursor;
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicUsize, Ordering},
};

#[test]
fn user_message_saytext() {
    let mut parser = Parser::new(Cursor::new(Vec::<u8>::new()));
    let captured: Arc<Mutex<Option<CcsUsrMsgSayText>>> = Arc::new(Mutex::new(None));
    let cap = captured.clone();
    parser.register_user_message_handler::<CcsUsrMsgSayText, _>(move |m| {
        *cap.lock().unwrap() = Some(m.clone());
    });

    let msg = CcsUsrMsgSayText {
        ent_idx: Some(1),
        text: Some("glhf".into()),
        chat: Some(true),
        textallchat: Some(true),
    };
    let mut buf = Vec::new();
    msg.encode(&mut buf).unwrap();
    let um = CsvcMsgUserMessage {
        msg_type: Some(msg::ECstrike15UserMessages::CsUmSayText as i32),
        msg_data: Some(buf),
        passthrough: None,
    };
    parser.handle_user_message(&um);
    std::thread::sleep(std::time::Duration::from_millis(10));

    let got = captured.lock().unwrap().clone().unwrap();
    assert_eq!(got.ent_idx.unwrap(), 1);
    assert_eq!(got.text.unwrap(), "glhf");
    assert!(got.chat.unwrap());
    assert!(got.textallchat.unwrap());
}

#[test]
fn user_message_saytext2_generic() {
    let mut parser = Parser::new(Cursor::new(Vec::<u8>::new()));
    let captured: Arc<Mutex<Option<CcsUsrMsgSayText2>>> = Arc::new(Mutex::new(None));
    let cap = captured.clone();
    parser.register_user_message_handler::<CcsUsrMsgSayText2, _>(move |m| {
        *cap.lock().unwrap() = Some(m.clone());
    });

    let msg = CcsUsrMsgSayText2 {
        ent_idx: Some(1),
        chat: Some(true),
        msg_name: Some("#CSGO_Coach_Join_T".into()),
        params: vec!["hi".into(), "hello".into()],
        textallchat: Some(true),
    };
    let mut buf = Vec::new();
    msg.encode(&mut buf).unwrap();
    let um = CsvcMsgUserMessage {
        msg_type: Some(msg::ECstrike15UserMessages::CsUmSayText2 as i32),
        msg_data: Some(buf),
        passthrough: None,
    };
    parser.handle_user_message(&um);
    std::thread::sleep(std::time::Duration::from_millis(10));

    let got = captured.lock().unwrap().clone().unwrap();
    assert_eq!(got.ent_idx.unwrap(), 1);
    assert_eq!(got.msg_name.unwrap(), "#CSGO_Coach_Join_T");
    assert_eq!(got.params, vec!["hi", "hello"]);
    assert!(got.chat.unwrap());
    assert!(got.textallchat.unwrap());
}

#[test]
fn chat_message_event() {
    let mut parser = Parser::new(Cursor::new(Vec::<u8>::new()));
    let cnt = Arc::new(AtomicUsize::new(0));
    let c = cnt.clone();
    parser.register_event_handler::<cs_demo_parser::events::ChatMessage, _>(move |e| {
        if e.text == "hello" {
            c.fetch_add(1, Ordering::SeqCst);
        }
    });

    let msg = CcsUsrMsgSayText {
        ent_idx: Some(1),
        text: Some("hello".into()),
        chat: Some(true),
        textallchat: Some(true),
    };
    let mut buf = Vec::new();
    msg.encode(&mut buf).unwrap();
    let um = CsvcMsgUserMessage {
        msg_type: Some(msg::ECstrike15UserMessages::CsUmSayText as i32),
        msg_data: Some(buf),
        passthrough: None,
    };
    parser.handle_user_message(&um);
    std::thread::sleep(std::time::Duration::from_millis(10));

    assert_eq!(1, cnt.load(Ordering::SeqCst));
}

#[test]
fn user_message_server_rank_update() {
    let mut parser = Parser::new(Cursor::new(Vec::<u8>::new()));
    let captured: Arc<Mutex<Option<CcsUsrMsgServerRankUpdate>>> = Arc::new(Mutex::new(None));
    let cap = captured.clone();
    parser.register_user_message_handler::<CcsUsrMsgServerRankUpdate, _>(move |m| {
        *cap.lock().unwrap() = Some(m.clone());
    });

    let update = CcsUsrMsgServerRankUpdate {
        rank_update: vec![
            ccs_usr_msg_server_rank_update::RankUpdate {
                account_id: Some(123),
                rank_old: Some(1),
                rank_new: Some(2),
                num_wins: Some(5),
                rank_change: Some(1.0),
                rank_type_id: None,
            },
            ccs_usr_msg_server_rank_update::RankUpdate {
                account_id: Some(456),
                rank_old: Some(2),
                rank_new: Some(3),
                num_wins: Some(6),
                rank_change: Some(2.0),
                rank_type_id: None,
            },
        ],
    };
    let mut buf = Vec::new();
    update.encode(&mut buf).unwrap();
    let um = CsvcMsgUserMessage {
        msg_type: Some(msg::ECstrike15UserMessages::CsUmServerRankUpdate as i32),
        msg_data: Some(buf),
        passthrough: None,
    };
    parser.handle_user_message(&um);
    std::thread::sleep(std::time::Duration::from_millis(10));

    let got = captured.lock().unwrap().clone().unwrap();
    assert_eq!(got.rank_update.len(), 2);
    assert_eq!(got.rank_update[0].account_id.unwrap(), 123);
    assert_eq!(got.rank_update[1].rank_new.unwrap(), 3);
}

#[test]
fn rank_update_event() {
    let mut parser = Parser::new(Cursor::new(Vec::<u8>::new()));
    let cnt = Arc::new(AtomicUsize::new(0));
    let c = cnt.clone();
    parser.register_event_handler::<cs_demo_parser::events::RankUpdate, _>(move |_| {
        c.fetch_add(1, Ordering::SeqCst);
    });

    let update = CcsUsrMsgServerRankUpdate {
        rank_update: vec![ccs_usr_msg_server_rank_update::RankUpdate {
            account_id: Some(1),
            rank_old: Some(0),
            rank_new: Some(1),
            num_wins: Some(1),
            rank_change: Some(1.0),
            rank_type_id: None,
        }],
    };
    let mut buf = Vec::new();
    update.encode(&mut buf).unwrap();
    let um = CsvcMsgUserMessage {
        msg_type: Some(msg::ECstrike15UserMessages::CsUmServerRankUpdate as i32),
        msg_data: Some(buf),
        passthrough: None,
    };
    parser.handle_user_message(&um);
    std::thread::sleep(std::time::Duration::from_millis(10));

    assert_eq!(1, cnt.load(Ordering::SeqCst));
}

#[test]
fn user_message_text_msg() {
    let mut parser = Parser::new(Cursor::new(Vec::<u8>::new()));
    let captured: Arc<Mutex<Option<CcsUsrMsgTextMsg>>> = Arc::new(Mutex::new(None));
    let cap = captured.clone();
    parser.register_user_message_handler::<CcsUsrMsgTextMsg, _>(move |m| {
        *cap.lock().unwrap() = Some(m.clone());
    });

    let msg = CcsUsrMsgTextMsg {
        msg_dst: Some(1),
        params: vec!["hello".into(), "world".into()],
    };
    let mut buf = Vec::new();
    msg.encode(&mut buf).unwrap();
    let um = CsvcMsgUserMessage {
        msg_type: Some(msg::ECstrike15UserMessages::CsUmTextMsg as i32),
        msg_data: Some(buf),
        passthrough: None,
    };
    parser.handle_user_message(&um);
    std::thread::sleep(std::time::Duration::from_millis(10));

    let got = captured.lock().unwrap().clone().unwrap();
    assert_eq!(got.msg_dst.unwrap(), 1);
    assert_eq!(got.params, vec!["hello", "world"]);
}

#[test]
fn user_message_hint_text() {
    let mut parser = Parser::new(Cursor::new(Vec::<u8>::new()));
    let captured: Arc<Mutex<Option<CcsUsrMsgHintText>>> = Arc::new(Mutex::new(None));
    let cap = captured.clone();
    parser.register_user_message_handler::<CcsUsrMsgHintText, _>(move |m| {
        *cap.lock().unwrap() = Some(m.clone());
    });

    let msg = CcsUsrMsgHintText {
        text: Some("hint".into()),
    };
    let mut buf = Vec::new();
    msg.encode(&mut buf).unwrap();
    let um = CsvcMsgUserMessage {
        msg_type: Some(msg::ECstrike15UserMessages::CsUmHintText as i32),
        msg_data: Some(buf),
        passthrough: None,
    };
    parser.handle_user_message(&um);
    std::thread::sleep(std::time::Duration::from_millis(10));

    let got = captured.lock().unwrap().clone().unwrap();
    assert_eq!(got.text.unwrap(), "hint");
}

#[test]
fn user_message_round_impact_score_data() {
    let mut parser = Parser::new(Cursor::new(Vec::<u8>::new()));
    let captured: Arc<Mutex<Option<CcsUsrMsgRoundImpactScoreData>>> = Arc::new(Mutex::new(None));
    let cap = captured.clone();
    parser.register_user_message_handler::<CcsUsrMsgRoundImpactScoreData, _>(move |m| {
        *cap.lock().unwrap() = Some(m.clone());
    });

    let msg = CcsUsrMsgRoundImpactScoreData {
        init_conditions: None,
        all_ris_event_data: Vec::new(),
    };
    let mut buf = Vec::new();
    msg.encode(&mut buf).unwrap();
    let um = CsvcMsgUserMessage {
        msg_type: Some(msg::ECstrike15UserMessages::CsUmRoundImpactScoreData as i32),
        msg_data: Some(buf),
        passthrough: None,
    };
    parser.handle_user_message(&um);
    std::thread::sleep(std::time::Duration::from_millis(10));

    let got = captured.lock().unwrap().clone().unwrap();
    assert!(got.init_conditions.is_none());
    assert!(got.all_ris_event_data.is_empty());
}

#[test]
fn user_message_round_backup_filenames() {
    let mut parser = Parser::new(Cursor::new(Vec::<u8>::new()));
    let captured: Arc<Mutex<Option<cs_demo_parser::events::RoundBackupFilenames>>> =
        Arc::new(Mutex::new(None));
    let cap = captured.clone();
    parser.register_user_message_handler::<cs_demo_parser::events::RoundBackupFilenames, _>(
        move |m| {
            *cap.lock().unwrap() = Some(m.clone());
        },
    );

    let msg = CcsUsrMsgRoundBackupFilenames {
        count: Some(3),
        index: Some(2),
        filename: Some("backup_02.dem".into()),
        nicename: Some("backup round 2".into()),
    };
    let mut buf = Vec::new();
    msg.encode(&mut buf).unwrap();
    let um = CsvcMsgUserMessage {
        msg_type: Some(msg::ECstrike15UserMessages::CsUmRoundBackupFilenames as i32),
        msg_data: Some(buf),
        passthrough: None,
    };
    parser.handle_user_message(&um);
    std::thread::sleep(std::time::Duration::from_millis(10));

    let got = captured.lock().unwrap().clone().unwrap();
    assert_eq!(got.count, 3);
    assert_eq!(got.index, 2);
    assert_eq!(got.filename, "backup_02.dem");
    assert_eq!(got.nicename, "backup round 2");
}

#[test]
fn user_message_vgui_menu() {
    let mut parser = Parser::new(Cursor::new(Vec::<u8>::new()));
    let captured: Arc<Mutex<Option<cs_demo_parser::events::VguiMenu>>> = Arc::new(Mutex::new(None));
    let cap = captured.clone();
    parser.register_user_message_handler::<cs_demo_parser::events::VguiMenu, _>(move |m| {
        *cap.lock().unwrap() = Some(m.clone());
    });

    let msg = CcsUsrMsgVguiMenu {
        name: Some("main".into()),
        show: Some(true),
        subkeys: vec![
            ccs_usr_msg_vgui_menu::Subkey {
                name: Some("a".into()),
                str: Some("1".into()),
            },
            ccs_usr_msg_vgui_menu::Subkey {
                name: Some("b".into()),
                str: Some("2".into()),
            },
        ],
    };
    let mut buf = Vec::new();
    msg.encode(&mut buf).unwrap();
    let um = CsvcMsgUserMessage {
        msg_type: Some(msg::ECstrike15UserMessages::CsUmVguiMenu as i32),
        msg_data: Some(buf),
        passthrough: None,
    };
    parser.handle_user_message(&um);
    std::thread::sleep(std::time::Duration::from_millis(10));

    let got = captured.lock().unwrap().clone().unwrap();
    assert_eq!(got.name, "main");
    assert!(got.show);
    assert_eq!(got.keys.len(), 2);
    assert_eq!(got.keys[0], ("a".into(), "1".into()));
    assert_eq!(got.keys[1], ("b".into(), "2".into()));
}

#[test]
fn user_message_show_menu() {
    let mut parser = Parser::new(Cursor::new(Vec::<u8>::new()));
    let captured: Arc<Mutex<Option<cs_demo_parser::events::ShowMenu>>> = Arc::new(Mutex::new(None));
    let cap = captured.clone();
    parser.register_user_message_handler::<cs_demo_parser::events::ShowMenu, _>(move |m| {
        *cap.lock().unwrap() = Some(m.clone());
    });

    let msg = CcsUsrMsgShowMenu {
        bits_valid_slots: Some(7),
        display_time: Some(5),
        menu_string: Some("menu".into()),
    };
    let mut buf = Vec::new();
    msg.encode(&mut buf).unwrap();
    let um = CsvcMsgUserMessage {
        msg_type: Some(msg::ECstrike15UserMessages::CsUmShowMenu as i32),
        msg_data: Some(buf),
        passthrough: None,
    };
    parser.handle_user_message(&um);
    std::thread::sleep(std::time::Duration::from_millis(10));

    let got = captured.lock().unwrap().clone().unwrap();
    assert_eq!(got.bits_valid_slots, 7);
    assert_eq!(got.display_time, 5);
    assert_eq!(got.menu_string, "menu");
}

#[test]
fn user_message_bar_time() {
    let mut parser = Parser::new(Cursor::new(Vec::<u8>::new()));
    let captured: Arc<Mutex<Option<cs_demo_parser::events::BarTime>>> = Arc::new(Mutex::new(None));
    let cap = captured.clone();
    parser.register_user_message_handler::<cs_demo_parser::events::BarTime, _>(move |m| {
        *cap.lock().unwrap() = Some(m.clone());
    });

    let msg = CcsUsrMsgBarTime {
        time: Some("3".into()),
    };
    let mut buf = Vec::new();
    msg.encode(&mut buf).unwrap();
    let um = CsvcMsgUserMessage {
        msg_type: Some(msg::ECstrike15UserMessages::CsUmBarTime as i32),
        msg_data: Some(buf),
        passthrough: None,
    };
    parser.handle_user_message(&um);
    std::thread::sleep(std::time::Duration::from_millis(10));

    let got = captured.lock().unwrap().clone().unwrap();
    assert_eq!(got.time, "3");
}
