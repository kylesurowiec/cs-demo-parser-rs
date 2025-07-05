use demoinfocs_rs::{events::ConVarsUpdated, parser::Parser, proto::msg::cs_demo_parser_rs as msg};
use std::collections::HashMap;
use std::io::Cursor;

#[test]
fn server_info_sets_map_name() {
    let mut parser = Parser::new(Cursor::new(Vec::<u8>::new()));
    parser.dispatch_net_message(msg::CsvcMsgServerInfo {
        map_name: Some("testmap".into()),
        ..Default::default()
    });
    std::thread::sleep(std::time::Duration::from_millis(10));
    assert_eq!(parser.game_state().map_name(), Some("testmap"));
}

#[test]
fn convars_update_round_time() {
    let mut parser = Parser::new(Cursor::new(Vec::<u8>::new()));
    let mut vars = HashMap::new();
    vars.insert("mp_roundtime".into(), "60".into());
    parser.dispatch_event(ConVarsUpdated {
        updated_con_vars: vars,
    });
    assert_eq!(
        parser.game_state().round_time(),
        Some(std::time::Duration::from_secs(60))
    );
}
