use demoinfocs_rs::parser::Parser;
use demoinfocs_rs::proto::msg::all::CsvcMsgBspDecal;
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

    parser.register_net_message_handler::<CsvcMsgBspDecal, _>(|msg| {
        if let Some(pos) = &msg.pos {
            println!(
                "bullet decal at x={} y={} z={}",
                pos.x.as_ref().map(|v| v.to_string()).unwrap_or_default(),
                pos.y.as_ref().map(|v| v.to_string()).unwrap_or_default(),
                pos.z.as_ref().map(|v| v.to_string()).unwrap_or_default()
            );
        }
    });

    if let Err(e) = parser.parse_to_end() {
        println!("error while parsing: {:?}", e);
    }
}
