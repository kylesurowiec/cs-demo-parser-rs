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
}

impl DebugReader {
    fn new(file: File, pos: Arc<AtomicU64>) -> Self {
        Self { inner: file, pos }
    }
}

impl Read for DebugReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let n = self.inner.read(buf)?;
        let p = self.pos.load(Ordering::SeqCst);
        self.pos.store(p + n as u64, Ordering::SeqCst);
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
    let pos = Arc::new(AtomicU64::new(0));
    let reader = DebugReader::new(file, pos.clone());
    let mut parser = Parser::new(reader);

    println!("\nParsing with library...");
    if let Err(e) = parser.parse_header() {
        eprintln!("Header error: {:?}", e);
        return;
    }

    for i in 0..200 {
        match parser.parse_next_frame() {
            | Ok(true) => {
                println!(
                    "Frame {} parsed at tick {} (offset {} bytes)",
                    i,
                    parser.game_state().ingame_tick(),
                    pos.load(Ordering::SeqCst)
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
