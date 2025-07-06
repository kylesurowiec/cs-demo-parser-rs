use demoinfocs_rs::parser::Parser;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("Usage: debug_dump <demo_path>");

    println!("Opening demo: {}", path);
    let mut file = File::open(&path).expect("Failed to open file");
    let size = file.metadata().map(|m| m.len()).unwrap_or(0);
    println!("File size: {} bytes", size);

    // Read raw header bytes for inspection
    let mut header_bytes = vec![0u8; 1072];
    if let Err(e) = file.read_exact(&mut header_bytes) {
        eprintln!("Failed to read header bytes: {}", e);
        return;
    }

    println!("\nRaw header preview:");
    let filestamp = std::str::from_utf8(&header_bytes[0..8]).unwrap_or("<invalid>");
    println!("  Filestamp: {}", filestamp);
    let playback_frames = i32::from_le_bytes([
        header_bytes[1064],
        header_bytes[1065],
        header_bytes[1066],
        header_bytes[1067],
    ]);
    println!("  Playback frames: {}", playback_frames);

    // Reset reader for parser
    file.seek(SeekFrom::Start(0)).expect("seek back");
    let mut parser = Parser::new(file);

    println!("\nParsing with library...");
    if let Err(e) = parser.parse_header() {
        eprintln!("Header error: {:?}", e);
        return;
    }

    for i in 0..200 {
        match parser.parse_next_frame() {
            | Ok(true) => {
                println!(
                    "Frame {} parsed at tick {}",
                    i,
                    parser.game_state().ingame_tick()
                );
            },
            | Ok(false) => {
                println!("Demo finished after {} frames", i);
                return;
            },
            | Err(e) => {
                eprintln!("Error on frame {}: {:?}", i, e);
                let progress = parser.progress();
                println!("Progress: {:.2}%", progress * 100.0);
                if let Ok(mut f) = File::open(&path) {
                    let pos = (size as f32 * progress) as i64;
                    let start = if pos > 16 { pos - 16 } else { 0 };
                    if f.seek(SeekFrom::Start(start as u64)).is_ok() {
                        let mut buf = [0u8; 32];
                        if let Ok(n) = f.read(&mut buf) {
                            print!("Bytes around position {}:", pos);
                            for b in &buf[..n] {
                                print!(" {:02x}", b);
                            }
                            println!();
                        }
                    }
                }
                return;
            },
        }
    }
}
