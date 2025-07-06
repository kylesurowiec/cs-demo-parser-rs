use cs_demo_parser::parser::Parser;
use std::env;
use std::fs::File;

fn demo_path_from_args() -> String {
    let mut args = env::args().skip(1);
    match (args.next(), args.next()) {
        | (Some(flag), Some(path)) if flag == "-demo" => path,
        | _ => panic!("Usage: cargo run --example nade_trajectories -- -demo <path>"),
    }
}

fn main() {
    let path = demo_path_from_args();
    let file = File::open(&path).expect("failed to open demo file");
    let mut parser = Parser::new(file);

    if let Ok(header) = parser.parse_header() {
        println!("map: {}", header.map_name);
    }

    parser.register_event_handler::<u8, _>(|_| {
        // grenade events would be handled here
    });

    if let Err(e) = parser.parse_to_end() {
        println!("error while parsing: {:?}", e);
    }
    println!("frames parsed: {}", parser.current_frame());
}
