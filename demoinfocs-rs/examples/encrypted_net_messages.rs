use demoinfocs_rs::matchinfo::match_info_decryption_key;
use demoinfocs_rs::parser::Parser;
use std::env;
use std::fs;
use std::fs::File;

fn args() -> (String, String) {
    let mut args = env::args().skip(1);
    match (args.next(), args.next(), args.next(), args.next()) {
        | (Some(flag1), Some(demo), Some(flag2), Some(info))
            if flag1 == "-demo" && flag2 == "-info" =>
        {
            (demo, info)
        },
        | _ => {
            panic!("Usage: cargo run --example encrypted_net_messages -- -demo <demo> -info <info>")
        },
    }
}

fn main() {
    let (demo_path, info_path) = args();
    let info_bytes = fs::read(info_path).expect("failed to read info file");
    let key = match_info_decryption_key(&info_bytes).expect("failed to parse key");
    println!("decryption key: {}", String::from_utf8_lossy(&key));

    let file = File::open(&demo_path).expect("failed to open demo file");
    let mut parser = Parser::new(file);
    if let Err(e) = parser.parse_to_end() {
        eprintln!("error while parsing demo: {:?}", e);
    }
}
