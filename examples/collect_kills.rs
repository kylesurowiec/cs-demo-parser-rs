use demoinfocs_rs::events::Kill;
use demoinfocs_rs::parser::Parser;
use std::env;
use std::fs::File;

/// Collects all kill events of the provided parser.
fn collect_kills(
    parser: &mut Parser<File>,
) -> Result<Vec<Kill>, demoinfocs_rs::parser::ParserError> {
    use std::sync::{Arc, Mutex};

    let kills = Arc::new(Mutex::new(Vec::new()));
    let handler_kills = Arc::clone(&kills);
    parser.register_event_handler::<Kill, _>(move |k| {
        handler_kills.lock().unwrap().push(k.clone());
    });
    parser.parse_to_end()?;
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
        | Ok(kills) => println!("collected {} kills", kills.len()),
        | Err(e) => eprintln!("error: {:?}", e),
    }
}
