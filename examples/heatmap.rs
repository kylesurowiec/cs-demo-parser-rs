use demoinfocs_rs::parser::Parser;
use std::env;
use std::fs::File;

fn demo_path_from_args() -> String {
    let mut args = env::args().skip(1);
    match (args.next(), args.next()) {
        | (Some(flag), Some(path)) if flag == "-demo" => path,
        | _ => panic!("Usage: cargo run --example heatmap -- -demo <path>"),
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

    parser.register_event_handler::<u8, _>(|_| {
        // handle events here
    });
    parser.register_net_message_handler::<u8, _>(|_| {
        // handle net messages here
    });

    if let Err(e) = parser.parse_to_end() {
        println!("error while parsing: {:?}", e);
    }
    println!("parsed {} frames", parser.current_frame());
}
