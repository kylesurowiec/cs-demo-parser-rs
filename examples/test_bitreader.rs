use cs_demo_parser::parser::{Parser};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

fn main() {
    let path = std::env::args().nth(1).expect("Usage: debug_frame_parsing <demo_path>");
    
    println!("Analyzing demo: {}", path);
    
    // First, let's look at the raw header data
    {
        let mut file = File::open(&path).expect("Failed to open file");
        let mut header_bytes = vec![0u8; 1072]; // 8 + 4 + 4 + 260*4 + 4 + 4 + 4 + 4
        file.read_exact(&mut header_bytes).expect("Failed to read header bytes");
        
        println!("\nRaw header analysis:");
        println!("Signature: {}", std::str::from_utf8(&header_bytes[0..8]).unwrap_or("<invalid>"));
        
        let protocol = i32::from_le_bytes([header_bytes[8], header_bytes[9], header_bytes[10], header_bytes[11]]);
        let network_protocol = i32::from_le_bytes([header_bytes[12], header_bytes[13], header_bytes[14], header_bytes[15]]);
        
        println!("Protocol: {}", protocol);
        println!("Network Protocol: {}", network_protocol);
        
        // Server name at offset 16, 260 bytes
        let server_name_end = header_bytes[16..276].iter().position(|&b| b == 0).unwrap_or(260);
        let server_name = std::str::from_utf8(&header_bytes[16..16+server_name_end]).unwrap_or("<invalid>");
        println!("Server name: '{}'", server_name);
        
        // Client name at offset 276, 260 bytes
        let client_name_end = header_bytes[276..536].iter().position(|&b| b == 0).unwrap_or(260);
        let client_name = std::str::from_utf8(&header_bytes[276..276+client_name_end]).unwrap_or("<invalid>");
        println!("Client name: '{}'", client_name);
        
        // Map name at offset 536, 260 bytes
        let map_name_end = header_bytes[536..796].iter().position(|&b| b == 0).unwrap_or(260);
        let map_name = std::str::from_utf8(&header_bytes[536..536+map_name_end]).unwrap_or("<invalid>");
        println!("Map name: '{}'", map_name);
        
        // Game directory at offset 796, 260 bytes
        let game_dir_end = header_bytes[796..1056].iter().position(|&b| b == 0).unwrap_or(260);
        let game_dir = std::str::from_utf8(&header_bytes[796..796+game_dir_end]).unwrap_or("<invalid>");
        println!("Game directory: '{}'", game_dir);
        
        // Playback time (float) at offset 1056
        let playback_time = f32::from_le_bytes([header_bytes[1056], header_bytes[1057], header_bytes[1058], header_bytes[1059]]);
        println!("Playback time: {}", playback_time);
        
        // Playback ticks at offset 1060
        let playback_ticks = i32::from_le_bytes([header_bytes[1060], header_bytes[1061], header_bytes[1062], header_bytes[1063]]);
        println!("Playback ticks: {}", playback_ticks);
        
        // Playback frames at offset 1064
        let playback_frames = i32::from_le_bytes([header_bytes[1064], header_bytes[1065], header_bytes[1066], header_bytes[1067]]);
        println!("Playback frames: {}", playback_frames);
        
        // Signon length at offset 1068
        let signon_length = i32::from_le_bytes([header_bytes[1068], header_bytes[1069], header_bytes[1070], header_bytes[1071]]);
        println!("Signon length: {}", signon_length);
    }
    
    // Now let's parse with the library and see what happens
    println!("\n\nParsing with library:");
    let file = File::open(&path).expect("Failed to open file");
    let mut parser = Parser::new(file);
    
    match parser.parse_header() {
        Ok(header) => {
            println!("Header parsed successfully");
            println!("  Map from parser: '{}'", header.map_name);
            println!("  Frames from parser: {}", header.playback_frames);
            
            // Try to parse frames with detailed logging
            println!("\nParsing frames...");
            for i in 0..200 {
                match parser.parse_next_frame() {
                    Ok(true) => {
                        if i >= 190 || i < 10 {
                            println!("Frame {}: OK", i);
                        }
                    }
                    Ok(false) => {
                        println!("Frame {}: End of demo", i);
                        break;
                    }
                    Err(e) => {
                        println!("Frame {}: Error - {:?}", i, e);
                        
                        // Let's see the current state
                        println!("Current parser frame: {}", parser.current_frame());
                        println!("Progress: {:.2}%", parser.progress() * 100.0);
                        break;
                    }
                }
            }
        }
        Err(e) => {
            println!("Failed to parse header: {:?}", e);
        }
    }
    
    // Let's also check what's at the position where it fails
    println!("\n\nChecking file position after header (1072 bytes):");
    {
        let mut file = File::open(&path).expect("Failed to open file");
        file.seek(SeekFrom::Start(1072)).expect("Failed to seek");
        
        let mut buffer = vec![0u8; 64];
        match file.read(&mut buffer) {
            Ok(n) => {
                println!("Read {} bytes at position 1072:", n);
                for i in 0..n.min(32) {
                    if i % 16 == 0 {
                        print!("\n  ");
                    }
                    print!("{:02x} ", buffer[i]);
                }
                println!();
            }
            Err(e) => {
                println!("Failed to read at position 1072: {}", e);
            }
        }
    }
}