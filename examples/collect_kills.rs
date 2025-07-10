use cs_demo_parser::events::Kill;
use cs_demo_parser::parser::Parser;
use std::collections::HashMap;
use std::env;
use std::fs::File;

/// Collects all kill events of the provided parser.
fn collect_kills(
    parser: &mut Parser<File>,
) -> Result<Vec<Kill>, cs_demo_parser::parser::ParserError> {
    use std::sync::{Arc, Mutex};

    let kills = Arc::new(Mutex::new(Vec::new()));
    let handler_kills = Arc::clone(&kills);
    let handler_id = parser.register_event_handler::<Kill, _>(move |k| {
        handler_kills.lock().unwrap().push(k.clone());
    });
    parser.parse_to_end()?;
    parser.unregister_event_handler(handler_id);
    Ok(Arc::try_unwrap(kills).unwrap().into_inner().unwrap())
}

fn demo_path_from_args() -> String {
    let mut args = env::args().skip(1);
    match (args.next(), args.next()) {
        | (Some(flag), Some(path)) if flag == "-demo" => path,
        | _ => panic!("Usage: cargo run --example collect_kills -- -demo <path>"),
    }
}

fn main() {
    let path = demo_path_from_args();
    let file = File::open(&path).expect("failed to open demo");
    let mut parser = Parser::new(file);
    match collect_kills(&mut parser) {
        | Ok(kills) => {
            println!("collected {} kills", kills.len());
            let mut scoreboard: HashMap<String, (u32, u32)> = HashMap::new();
            for k in &kills {
                if k.killer.is_some() {
                    let entry = scoreboard.entry("Unknown".to_string()).or_insert((0, 0));
                    entry.0 += 1;
                }
                if k.victim.is_some() {
                    let entry = scoreboard.entry("Unknown".to_string()).or_insert((0, 0));
                    entry.1 += 1;
                }
            }
            println!("scoreboard:");
            for (player, (kills, deaths)) in scoreboard {
                println!("{}: {} kills / {} deaths", player, kills, deaths);
            }
        },
        | Err(e) => eprintln!("error: {:?}", e),
    }
}
