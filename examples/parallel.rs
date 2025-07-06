use cs_demo_parser::utils::parallel;
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let dir = env::args()
        .nth(1)
        .expect("Usage: cargo run --example parallel -- <demo-directory>");
    let demos: Vec<PathBuf> = fs::read_dir(dir)
        .expect("failed to read directory")
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().map(|e| e == "dem").unwrap_or(false))
        .collect();

    parallel::run(demos, |parser, path| {
        if let Ok(header) = parser.parse_header() {
            println!("{} -> {}", path.display(), header.map_name);
        }
        if let Err(e) = parser.parse_to_end() {
            eprintln!("{}: {:?}", path.display(), e);
        }
    });
}
