use cs_demo_parser::parser::{Parser, ParserConfig};
use std::fs::File;
use std::io::Read;

fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("Usage: test_demo_header <demo_path>");

    // First, let's check if we can read the file at all
    println!("Testing file: {}", path);

    // Check raw file reading
    {
        let mut file = File::open(&path).expect("Failed to open file");
        let mut buffer = [0u8; 8];
        match file.read_exact(&mut buffer) {
            | Ok(_) => {
                println!("First 8 bytes (raw): {:?}", buffer);
                // Convert to string if possible
                if let Ok(s) = std::str::from_utf8(&buffer) {
                    println!("As string: '{}'", s);
                }
            },
            | Err(e) => {
                println!("Failed to read first 8 bytes: {}", e);
            },
        }
    }

    // Now try with the parser
    println!("\nTrying parser...");
    let file = File::open(&path).expect("Failed to open file");
    let mut parser = Parser::new(file);

    match parser.parse_header() {
        | Ok(header) => {
            println!("Header parsed successfully!");
            println!("  Filestamp: '{}'", header.filestamp);
            println!("  Is HL2DEMO: {}", header.filestamp == "HL2DEMO");
            println!("  Is PBDEMS2: {}", header.filestamp == "PBDEMS2");
            println!("  Protocol: {}", header.protocol);
            println!("  Network Protocol: {}", header.network_protocol);
            println!("  Map: {}", header.map_name);
        },
        | Err(e) => {
            println!("Failed to parse header: {:?}", e);
        },
    }

    // Try with a custom config that might be more forgiving
    println!("\nTrying with custom config...");
    let config = ParserConfig {
        ignore_packet_entities_panic: true,
        ..Default::default()
    };
    let file = File::open(&path).expect("Failed to open file");
    let mut parser = Parser::with_config(file, config);

    match parser.parse_header() {
        | Ok(_) => {
            println!("Header parsed with custom config");

            // Try parsing one frame
            println!("\nTrying to parse first frame...");
            match parser.parse_next_frame() {
                | Ok(true) => println!("First frame parsed successfully"),
                | Ok(false) => println!("Demo has no frames?"),
                | Err(e) => println!("Error parsing first frame: {:?}", e),
            }
        },
        | Err(e) => {
            println!("Failed with custom config too: {:?}", e);
        },
    }
}
