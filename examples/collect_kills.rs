use cs_demo_parser::bitreader::BitReader;
use std::env;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

fn demo_path_from_args() -> String {
    let mut args = env::args().skip(1);
    match (args.next(), args.next()) {
        | (Some(flag), Some(path)) if flag == "-demo" => path,
        | _ => panic!("Usage: cargo run --bin enhanced_debug -- -demo <path>"),
    }
}

fn debug_header_byte_by_byte(file_path: &str) {
    println!("=== BYTE-BY-BYTE HEADER ANALYSIS ===");
    let mut file = File::open(file_path).expect("Failed to open file");
    let mut buffer = vec![0u8; 1072]; // Full header size
    let bytes_read = file.read(&mut buffer).expect("Failed to read file");
    
    println!("Read {} bytes from file", bytes_read);
    
    if bytes_read < 1072 {
        println!("⚠ Warning: File smaller than expected header size (1072 bytes)");
        println!("This might be a truncated or corrupted demo file");
        return;
    }
    
    // File stamp (8 bytes)
    let filestamp = String::from_utf8_lossy(&buffer[0..8]);
    println!("File stamp: '{}'", filestamp);
    
    if !filestamp.starts_with("HL2DEMO") && !filestamp.starts_with("PBDEMS2") {
        println!("❌ Invalid file stamp - not a valid demo file");
        return;
    }
    
    // Protocol (4 bytes, offset 8)
    let protocol = i32::from_le_bytes([buffer[8], buffer[9], buffer[10], buffer[11]]);
    println!("Protocol: {}", protocol);
    
    // Network protocol (4 bytes, offset 12)
    let net_protocol = i32::from_le_bytes([buffer[12], buffer[13], buffer[14], buffer[15]]);
    println!("Network protocol: {}", net_protocol);
    
    // Server name (260 bytes, offset 16)
    let server_name_end = buffer[16..276].iter().position(|&x| x == 0).unwrap_or(260) + 16;
    let server_name = String::from_utf8_lossy(&buffer[16..server_name_end]);
    println!("Server name: '{}' (length: {})", server_name, server_name.len());
    
    // Client name (260 bytes, offset 276)
    let client_name_end = buffer[276..536].iter().position(|&x| x == 0).unwrap_or(260) + 276;
    let client_name = String::from_utf8_lossy(&buffer[276..client_name_end]);
    println!("Client name: '{}' (length: {})", client_name, client_name.len());
    
    // Map name (260 bytes, offset 536)
    let map_name_end = buffer[536..796].iter().position(|&x| x == 0).unwrap_or(260) + 536;
    let map_name = String::from_utf8_lossy(&buffer[536..map_name_end]);
    println!("Map name: '{}' (length: {})", map_name, map_name.len());
    
    // Game directory (260 bytes, offset 796)
    let game_dir_end = buffer[796..1056].iter().position(|&x| x == 0).unwrap_or(260) + 796;
    let game_dir = String::from_utf8_lossy(&buffer[796..game_dir_end]);
    println!("Game directory: '{}' (length: {})", game_dir, game_dir.len());
    
    // Playback time (4 bytes float, offset 1056)
    let playback_time = f32::from_le_bytes([buffer[1056], buffer[1057], buffer[1058], buffer[1059]]);
    println!("Playback time: {:.2}s", playback_time);
    
    // Playback ticks (4 bytes int, offset 1060)
    let playback_ticks = i32::from_le_bytes([buffer[1060], buffer[1061], buffer[1062], buffer[1063]]);
    println!("Playback ticks: {}", playback_ticks);
    
    // Playback frames (4 bytes int, offset 1064)
    let playback_frames = i32::from_le_bytes([buffer[1064], buffer[1065], buffer[1066], buffer[1067]]);
    println!("Playback frames: {}", playback_frames);
    
    // Signon length (4 bytes int, offset 1068)
    let signon_length = i32::from_le_bytes([buffer[1068], buffer[1069], buffer[1070], buffer[1071]]);
    println!("Signon length: {}", signon_length);
    
    // Validate fields
    println!("\n=== HEADER VALIDATION ===");
    if playback_time <= 0.0 {
        println!("❌ Invalid playback time: {}", playback_time);
    } else {
        println!("✓ Playback time looks valid: {:.2}s", playback_time);
    }
    
    if playback_ticks <= 0 {
        println!("❌ Invalid playback ticks: {}", playback_ticks);
    } else {
        println!("✓ Playback ticks looks valid: {}", playback_ticks);
    }
    
    if playback_frames <= 0 {
        println!("❌ Invalid playback frames: {}", playback_frames);
    } else {
        println!("✓ Playback frames looks valid: {}", playback_frames);
    }
    
    if map_name.trim().is_empty() {
        println!("⚠ Map name is empty - this might be normal for some demo types");
    } else {
        println!("✓ Map name present: '{}'", map_name);
    }
    
    // Calculate expected demo duration
    if playback_ticks > 0 && playback_time > 0.0 {
        let tick_rate = playback_ticks as f32 / playback_time;
        println!("✓ Calculated tick rate: {:.1} ticks/second", tick_rate);
        
        if tick_rate < 60.0 || tick_rate > 130.0 {
            println!("⚠ Unusual tick rate - typical CS:GO demos are 64 or 128 tick");
        }
    }
}

fn debug_bitreader_step_by_step(file_path: &str) {
    println!("\n=== BITREADER STEP-BY-STEP DEBUG ===");
    
    let file = File::open(file_path).expect("Failed to open file");
    let mut reader = BitReader::new_large(file);
    
    // Read each field step by step and show exactly what we get
    println!("Step 1: Reading file stamp (8 bytes)...");
    let filestamp = reader.read_c_string(8);
    println!("  Result: '{}'", filestamp);
    
    println!("Step 2: Reading protocol (32-bit signed int)...");
    let protocol = reader.read_signed_int(32);
    println!("  Result: {}", protocol);
    
    println!("Step 3: Reading network protocol (32-bit signed int)...");
    let net_protocol = reader.read_signed_int(32);
    println!("  Result: {}", net_protocol);
    
    println!("Step 4: Reading server name (C string, max 260 bytes)...");
    let server_name = reader.read_c_string(260);
    println!("  Result: '{}' (length: {})", server_name, server_name.len());
    
    println!("Step 5: Reading client name (C string, max 260 bytes)...");
    let client_name = reader.read_c_string(260);
    println!("  Result: '{}' (length: {})", client_name, client_name.len());
    
    println!("Step 6: Reading map name (C string, max 260 bytes)...");
    let map_name = reader.read_c_string(260);
    println!("  Result: '{}' (length: {})", map_name, map_name.len());
    
    println!("Step 7: Reading game directory (C string, max 260 bytes)...");
    let game_directory = reader.read_c_string(260);
    println!("  Result: '{}' (length: {})", game_directory, game_directory.len());
    
    println!("Step 8: Reading playback time (32-bit float)...");
    let playback_time = reader.read_float();
    println!("  Result: {:.6}s", playback_time);
    
    println!("Step 9: Reading playback ticks (32-bit signed int)...");
    let playback_ticks = reader.read_signed_int(32);
    println!("  Result: {}", playback_ticks);
    
    println!("Step 10: Reading playback frames (32-bit signed int)...");
    let playback_frames = reader.read_signed_int(32);
    println!("  Result: {}", playback_frames);
    
    println!("Step 11: Reading signon length (32-bit signed int)...");
    let signon_length = reader.read_signed_int(32);
    println!("  Result: {}", signon_length);
    
    println!("\n=== BITREADER VALIDATION ===");
    if playback_time <= 0.0 || playback_ticks <= 0 || playback_frames <= 0 {
        println!("❌ Header parsing failed - critical fields are zero or negative");
        println!("   This suggests the BitReader is not aligned correctly or the file is corrupted");
    } else {
        println!("✓ Header parsing succeeded - all critical fields have valid values");
    }
}

fn check_file_integrity(file_path: &str) {
    println!("\n=== FILE INTEGRITY CHECK ===");
    
    let mut file = File::open(file_path).expect("Failed to open file");
    let metadata = file.metadata().expect("Failed to get file metadata");
    let file_size = metadata.len();
    
    println!("File size: {} bytes ({:.2} MB)", file_size, file_size as f64 / 1_048_576.0);
    
    if file_size < 1072 {
        println!("❌ File is smaller than minimum header size (1072 bytes)");
        println!("   This file is definitely corrupted or incomplete");
        return;
    }
    
    // Read first 16 bytes to check magic numbers
    let mut magic = [0u8; 16];
    file.read_exact(&mut magic).expect("Failed to read magic bytes");
    
    println!("Magic bytes: {:02x?}", &magic);
    
    // Check if this looks like a valid demo file
    if &magic[0..7] == b"HL2DEMO" {
        println!("✓ Valid Source 1 demo file signature");
    } else if &magic[0..7] == b"PBDEMS2" {
        println!("✓ Valid Source 2 demo file signature");
    } else {
        println!("❌ Invalid demo file signature");
        println!("   Expected 'HL2DEMO' or 'PBDEMS2', got '{}'", String::from_utf8_lossy(&magic[0..8]));
        return;
    }
    
    // Seek to end to check if file ends abruptly
    file.seek(SeekFrom::End(-16)).expect("Failed to seek to end");
    let mut end_bytes = [0u8; 16];
    file.read_exact(&mut end_bytes).expect("Failed to read end bytes");
    
    println!("Last 16 bytes: {:02x?}", end_bytes);
    
    // Check if the file appears to end naturally (most demo files end with specific patterns)
    let all_zeros = end_bytes.iter().all(|&b| b == 0);
    if all_zeros {
        println!("⚠ File ends with all zeros - might be truncated");
    }
}

fn main() {
    let path = demo_path_from_args();
    
    println!("Enhanced CS:GO Demo Debug Tool");
    println!("Demo file: {}", path);
    println!("========================================");
    
    // Run all checks
    check_file_integrity(&path);
    debug_header_byte_by_byte(&path);
    debug_bitreader_step_by_step(&path);
    
    println!("\n=== RECOMMENDATIONS ===");
    println!("1. If the byte-by-byte analysis shows valid data but BitReader fails,");
    println!("   there might be an issue with the BitReader implementation.");
    println!("2. If both analyses show invalid data, the demo file is likely corrupted.");
    println!("3. Try with a different demo file to see if the issue is file-specific.");
    println!("4. Check if this is a newer demo format that requires different parsing.");
}