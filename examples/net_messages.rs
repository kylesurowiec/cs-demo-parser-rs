use demoinfocs_rs::parser::Parser;
use demoinfocs_rs::proto::msg::cs_demo_parser_rs::{
    CcsUsrMsgBarTime, CcsUsrMsgRoundBackupFilenames, CcsUsrMsgShowMenu, CcsUsrMsgVguiMenu,
};
use std::env;
use std::fs::File;

fn demo_path_from_args() -> String {
    let mut args = env::args().skip(1);
    match (args.next(), args.next()) {
        | (Some(flag), Some(path)) if flag == "-demo" => path,
        | _ => panic!("Usage: cargo run --example net_messages -- -demo <path>"),
    }
}

fn main() {
    let path = demo_path_from_args();
    let file = File::open(&path).expect("failed to open demo file");
    let mut parser = Parser::new(file);

    match parser.parse_header() {
        | Ok(header) => println!("map: {}", header.map_name),
        | Err(err) => println!("failed to parse header: {:?}", err),
    }

    parser.register_user_message_handler::<CcsUsrMsgVguiMenu, _>(|m| {
        println!(
            "VGUI menu: {} (show={})",
            m.name.clone().unwrap_or_default(),
            m.show.unwrap_or(false)
        );
    });
    parser.register_user_message_handler::<CcsUsrMsgShowMenu, _>(|m| {
        println!("Show menu: {}", m.menu_string.clone().unwrap_or_default());
    });
    parser.register_user_message_handler::<CcsUsrMsgBarTime, _>(|m| {
        println!("Bar time: {}", m.time.clone().unwrap_or_default());
    });
    parser.register_user_message_handler::<CcsUsrMsgRoundBackupFilenames, _>(|m| {
        println!("Round backup: {}", m.filename.clone().unwrap_or_default());
    });

    if let Err(e) = parser.parse_to_end() {
        println!("error while parsing: {:?}", e);
    }
    println!("frames parsed: {}", parser.current_frame());
}
