use cs_demo_parser::parser::Parser;
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

    parser.parse_to_end().expect("parse error");

    println!("\nPlayers:");
    for player in parser.game_state().participants().connected() {
        println!(
            "{} scoped={} ducking={} bomb_zone={}",
            player.name,
            player.is_scoped(),
            player.is_ducking(),
            player.is_in_bomb_zone()
        );
    }
}
