// This creates a wrapper around the demo file that logs all read operations
// Save this as examples/debug_parse.rs

use std::fs::File;
use std::io::{Read, Result};
use cs_demo_parser::parser::Parser;

struct DebugReader {
    inner: File,
    bytes_read: usize,
    read_count: usize,
}

impl DebugReader {
    fn new(file: File) -> Self {
        Self {
            inner: file,
            bytes_read: 0,
            read_count: 0,
        }
    }
}

impl Read for DebugReader {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let result = self.inner.read(buf);
        
        if let Ok(n) = result {
            self.bytes_read += n;
            self.read_count += 1;
            
            if self.read_count <= 10 || n == 0 {
                println!("Read #{}: requested {} bytes, got {} bytes (total: {} bytes)", 
                    self.read_count, buf.len(), n, self.bytes_read);
                
                if n > 0 && n <= 32 {
                    print!("  Data: ");
                    for i in 0..n {
                        print!("{:02x} ", buf[i]);
                    }
                    println!();
                }
                
                if n == 0 {
                    println!("  EOF reached!");
                }
            }
        } else if let Err(e) = &result {
            println!("Read #{} failed: {}", self.read_count, e);
        }
        
        result
    }
}

fn main() {
    let path = std::env::args().nth(1).expect("Usage: debug_parse <demo_path>");
    
    println!("Opening demo: {}", path);
    
    // Get file size
    let metadata = std::fs::metadata(&path).expect("Failed to get metadata");
    println!("File size: {} bytes", metadata.len());
    
    let file = File::open(&path).expect("Failed to open file");
    let debug_reader = DebugReader::new(file);
    let mut parser = Parser::new(debug_reader);
    
    println!("\nParsing header...");
    match parser.parse_header() {
        Ok(header) => {
            println!("\nHeader parsed successfully!");
            println!("  Filestamp: {}", header.filestamp);
            println!("  Map: {}", header.map_name);
            
            println!("\nTrying to parse first frame...");
            match parser.parse_next_frame() {
                Ok(true) => println!("First frame parsed!"),
                Ok(false) => println!("No frames?"),
                Err(e) => println!("Frame parse error: {:?}", e),
            }
        }
        Err(e) => {
            println!("Header parse failed: {:?}", e);
        }
    }
}