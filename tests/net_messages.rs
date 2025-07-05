use demoinfocs_rs::parser::Parser;
use demoinfocs_rs::proto::msg::cs_demo_parser_rs as msg;
use std::io::Cursor;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

macro_rules! net_msg_test {
    ($name:ident, $typ:ty) => {
        #[test]
        fn $name() {
            let mut parser = Parser::new(Cursor::new(Vec::<u8>::new()));
            let cnt = Arc::new(AtomicUsize::new(0));
            let c = cnt.clone();
            parser.register_net_message_handler::<$typ, _>(move |_| {
                c.fetch_add(1, Ordering::SeqCst);
            });
            parser.dispatch_net_message(<$typ>::default());
            std::thread::sleep(std::time::Duration::from_millis(10));
            assert_eq!(1, cnt.load(Ordering::SeqCst));
        }
    };
}

net_msg_test!(server_info, msg::CsvcMsgServerInfo);
net_msg_test!(send_table, msg::CsvcMsgSendTable);
net_msg_test!(class_info, msg::CsvcMsgClassInfo);
net_msg_test!(set_pause, msg::CsvcMsgSetPause);
net_msg_test!(create_string_table, msg::CsvcMsgCreateStringTable);
net_msg_test!(update_string_table, msg::CsvcMsgUpdateStringTable);
net_msg_test!(voice_init, msg::CsvcMsgVoiceInit);
net_msg_test!(voice_data, msg::CsvcMsgVoiceData);
net_msg_test!(print_msg, msg::CsvcMsgPrint);
net_msg_test!(sounds_msg, msg::CsvcMsgSounds);
net_msg_test!(set_view, msg::CsvcMsgSetView);
net_msg_test!(fix_angle, msg::CsvcMsgFixAngle);
net_msg_test!(crosshair_angle, msg::CsvcMsgCrosshairAngle);
net_msg_test!(bsp_decal, msg::CsvcMsgBspDecal);
net_msg_test!(split_screen, msg::CsvcMsgSplitScreen);
net_msg_test!(user_message, msg::CsvcMsgUserMessage);
net_msg_test!(entity_message, msg::CsvcMsgEntityMsg);
net_msg_test!(game_event, msg::CsvcMsgGameEvent);
net_msg_test!(packet_entities, msg::CsvcMsgPacketEntities);
net_msg_test!(temp_entities, msg::CsvcMsgTempEntities);
net_msg_test!(prefetch, msg::CsvcMsgPrefetch);
net_msg_test!(menu, msg::CsvcMsgMenu);
net_msg_test!(game_event_list, msg::CsvcMsgGameEventList);
net_msg_test!(get_cvar_value, msg::CsvcMsgGetCvarValue);
net_msg_test!(paintmap_data, msg::CsvcMsgPaintmapData);
net_msg_test!(cmd_key_values, msg::CsvcMsgCmdKeyValues);
net_msg_test!(encrypted_data, msg::CsvcMsgEncryptedData);
net_msg_test!(hltv_replay, msg::CsvcMsgHltvReplay);
net_msg_test!(broadcast_command, msg::CsvcMsgBroadcastCommand);
net_msg_test!(net_nop, msg::CnetMsgNop);
net_msg_test!(net_disconnect, msg::CnetMsgDisconnect);
net_msg_test!(net_file, msg::CnetMsgFile);
net_msg_test!(net_split_screen_user, msg::CnetMsgSplitScreenUser);
net_msg_test!(net_tick, msg::CnetMsgTick);
net_msg_test!(net_string_cmd, msg::CnetMsgStringCmd);
net_msg_test!(net_set_con_var, msg::CnetMsgSetConVar);
net_msg_test!(net_signon_state, msg::CnetMsgSignonState);
net_msg_test!(net_player_avatar_data, msg::CnetMsgPlayerAvatarData);
