use demoinfocs_rs::parser::Parser;
use std::env;
use std::fs::File;

fn demo_path_from_args() -> String {
    let mut args = env::args().skip(1);
    match (args.next(), args.next()) {
        | (Some(flag), Some(path)) if flag == "-demo" => path,
        | _ => panic!("Usage: cargo run --example entities -- -demo <path>"),
    }
}

fn main() {
    let path = demo_path_from_args();
    let file = File::open(&path).expect("failed to open demo file");
    let mut parser = Parser::new(file);

    // Parse until server classes become available
    while parser.server_classes().is_empty() {
        if !parser.parse_next_frame().expect("parse error") {
            break;
        }
    }

    for class in parser.server_classes() {
        println!("ServerClass: id={} name={}", class.id(), class.name());
    }
}
