use demoinfocs_rs::parser::Parser;
use demoinfocs_rs::proto::msg::all::*;
use demoinfocs_rs::proto::msg::{self};
use prost::Message;
use std::io::Cursor;
use std::sync::{Arc, Mutex};

#[test]
fn user_message_saytext() {
    let parser = Parser::new(Cursor::new(Vec::<u8>::new()));
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
    let parser = Parser::new(Cursor::new(Vec::<u8>::new()));
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
fn user_message_server_rank_update() {
    let parser = Parser::new(Cursor::new(Vec::<u8>::new()));
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
