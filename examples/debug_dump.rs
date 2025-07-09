use cs_demo_parser::parser::Parser;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};

/// Wrapper that tracks how many bytes have been read from the underlying file.
struct DebugReader {
    inner: File,
    pos: Arc<AtomicU64>,
    reads: Arc<AtomicU64>,
}

impl DebugReader {
    fn new(file: File, pos: Arc<AtomicU64>, reads: Arc<AtomicU64>) -> Self {
        Self {
            inner: file,
            pos,
            reads,
        }
    }
}

impl Read for DebugReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let n = self.inner.read(buf)?;
        let p = self.pos.load(Ordering::SeqCst);
        self.pos.store(p + n as u64, Ordering::SeqCst);

        let rc = self.reads.fetch_add(1, Ordering::SeqCst) + 1;
        if rc <= 10 || n == 0 {
            println!(
                "Read #{}: requested {} bytes, got {} bytes (offset {})",
                rc,
                buf.len(),
                n,
                self.pos.load(Ordering::SeqCst)
            );
            if n > 0 && n <= 32 {
                print!("  Data:");
                for b in &buf[..n] {
                    print!(" {:02x}", b);
                }
                println!();
            }
            if n == 0 {
                println!("  EOF reached!");
            }
        }
        Ok(n)
    }
}

impl Seek for DebugReader {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        let new = self.inner.seek(pos)?;
        self.pos.store(new, Ordering::SeqCst);
        Ok(new)
    }
}

fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("Usage: debug_dump <demo_path>");

    println!("Opening demo: {}", path);
    let mut file = File::open(&path).expect("Failed to open file");
    let size = file.metadata().map(|m| m.len()).unwrap_or(0);
    println!("File size: {} bytes", size);

    // Peek the filestamp to detect Git LFS pointer files early
    let mut filestamp_buf = [0u8; 8];
    if let Err(e) = file.read_exact(&mut filestamp_buf) {
        eprintln!("Failed to read filestamp: {}", e);
        return;
    }
    let filestamp_str = std::str::from_utf8(&filestamp_buf).unwrap_or("<invalid>");
    if filestamp_str.trim_start() == "version" {
        eprintln!(
            "Input looks like a Git LFS pointer. Run `git lfs pull` to fetch the actual demo file."
        );
        return;
    }
    // Rewind and attempt to read the full header
    file.seek(SeekFrom::Start(0)).expect("seek back to start");
    if size < 1072 {
        eprintln!(
            "File is too small to contain a complete header ({} < 1072 bytes)",
            size
        );
    }

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

    // Parse lump table for extra insight
    let mut lump_file = File::open(&path).expect("Failed to reopen for lumps");
    lump_file.seek(SeekFrom::Start(1072)).expect("seek lumps");
    let mut lump_reader = cs_demo_parser::bitreader::BitReader::new_small(lump_file);
    let magic = lump_reader.read_int(32);
    let lump_info = cs_demo_parser::parser::lumps::LumpInfo::parse(&mut lump_reader);
    println!("  Lump magic: 0x{:08x}", magic);
    println!("  Lump data size: {} bytes", lump_info.data_size);

    // Reset reader for parser
    file.seek(SeekFrom::Start(0)).expect("seek back");
    let pos = Arc::new(AtomicU64::new(0));
    let reads = Arc::new(AtomicU64::new(0));
    let reader = DebugReader::new(file, pos.clone(), reads.clone());
    let mut parser = Parser::new(reader);

    println!("\nParsing with library...");
    let header = match parser.parse_header() {
        | Ok(h) => h,
        | Err(e) => {
            eprintln!("Header error: {:?}", e);
            return;
        },
    };
    println!("\nParsed header:");
    println!("  Protocol: {}", header.protocol);
    println!("  Network protocol: {}", header.network_protocol);
    println!("  Server: {}", header.server_name);
    println!("  Client: {}", header.client_name);
    println!("  Map: {}", header.map_name);
    println!("  Game dir: {}", header.game_directory);
    println!("  Playback ticks: {}", header.playback_ticks);
    println!("  Playback time: {}", header.playback_time);
    println!("  Signon length: {}", header.signon_length);
    println!("  Lump skip size: {}", parser.lump_size());

    for i in 0..200 {
        let frame_start = pos.load(Ordering::SeqCst);
        match parser.parse_next_frame() {
            | Ok(true) => {
                let frame_end = pos.load(Ordering::SeqCst);
                let mut cmd_byte = [0u8; 1];
                if let Ok(mut pf) = File::open(&path) {
                    if pf.seek(SeekFrom::Start(frame_start)).is_ok() {
                        let _ = pf.read_exact(&mut cmd_byte);
                    }
                }
                let cmd_name = match cmd_byte[0] {
                    | 0 => "Signon",
                    | 1 => "Packet",
                    | 2 => "SyncTick",
                    | 3 => "ConsoleCmd",
                    | 4 => "UserCmd",
                    | 5 => "DataTables",
                    | 6 => "Stop",
                    | 7 => "CustomData",
                    | 8 => "StringTables",
                    | _ => "Other",
                };
                println!(
                    "Frame {} parsed at tick {} ({}-{} len {} bytes) cmd {} ({}) progress {:.2}% reads {}",
                    i,
                    parser.game_state().ingame_tick(),
                    frame_start,
                    frame_end,
                    frame_end - frame_start,
                    cmd_byte[0],
                    cmd_name,
                    parser.progress() * 100.0,
                    reads.load(Ordering::SeqCst)
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
                println!("Total reads: {}", reads.load(Ordering::SeqCst));
                let pos_bytes = pos.load(Ordering::SeqCst) as i64;
                if let Ok(mut f) = File::open(&path) {
                    let start = if pos_bytes > 16 { pos_bytes - 16 } else { 0 };
                    if f.seek(SeekFrom::Start(start as u64)).is_ok() {
                        let mut buf = [0u8; 32];
                        if let Ok(n) = f.read(&mut buf) {
                            print!("Bytes around position {}:", pos_bytes);
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
