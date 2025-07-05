use reqwest::blocking::get;
use sevenz_rust::decompress_file;
use std::io::Write;
use std::path::{Path, PathBuf};

/// Recursively collects all .proto files from the given directory and its subdirectories.
///
/// # Arguments
/// * `dir` - The directory to search for .proto files
///
/// # Returns
/// A vector of paths to all .proto files found
fn collect_proto_files(dir: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut files = Vec::new();
    if dir.is_dir() {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                files.extend(collect_proto_files(&path)?);
            } else if path.extension().is_some_and(|ext| ext == "proto") {
                files.push(path);
            }
        }
    }
    Ok(files)
}

fn ensure_descriptor_proto(vendor_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let descriptor_path = vendor_dir.join("google/protobuf/descriptor.proto");
    if descriptor_path.exists() {
        return Ok(());
    }
    // Download descriptor.proto from the official protobuf repo
    let url = "https://raw.githubusercontent.com/protocolbuffers/protobuf/main/src/google/protobuf/descriptor.proto";
    println!("cargo:warning=Downloading descriptor.proto from {}...", url);
    let resp = reqwest::blocking::get(url)?;
    if !resp.status().is_success() {
        return Err(format!("Failed to download descriptor.proto: HTTP {}", resp.status()).into());
    }
    std::fs::create_dir_all(descriptor_path.parent().unwrap())?;
    let mut file = std::fs::File::create(&descriptor_path)?;
    file.write_all(&resp.bytes()?)?;
    println!("cargo:warning=Downloaded descriptor.proto to {}", descriptor_path.display());
    Ok(())
}

/// Compiles protocol buffer files from the input directory to Rust code in the output directory.
///
/// # Arguments
/// * `input` - Relative path to the directory containing .proto files
/// * `output` - Relative path within src/proto/ where generated Rust files will be placed
///
/// # Notes
/// - Skips certain conflicting proto files (base_gcmessages_csgo.proto, steamworkssdk.proto, etc.)
/// - Generates a mod.rs file that exports all compiled proto modules
fn compile(input: &str, output: &str) -> Result<(), Box<dyn std::error::Error>> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let in_dir = manifest_dir.join(input);
    let out_dir = manifest_dir.join("src").join("proto").join(output);
    let vendor_dir = manifest_dir.join("third_party");
    ensure_descriptor_proto(&vendor_dir)?;
    std::fs::create_dir_all(&out_dir)?;
    let mut protos = collect_proto_files(&in_dir)?;
    // `base_gcmessages_csgo.proto` duplicates several definitions from
    // `base_gcmessages.proto` and compiling both at the same time causes
    // conflicts in `prost-build`. Skip the CS:GO specific variant for now.
    protos.retain(|p| {
        let name = p.file_name().and_then(|f| f.to_str());
        match name {
            | Some("base_gcmessages_csgo.proto") => false,
            | Some(n) if n.ends_with("steamworkssdk.proto") => false,
            | Some("enums_clientserver.proto") => false,
            | Some("steammessages_base.proto") => false, // skip problematic proto
            | _ => true,
        }
    });
    // Only generate a minimal mod.rs for the consolidated protobuf module
    let mod_rs = r#"#[path = "cs_demo_parser_rs.rs"]
pub mod cs_demo_parser_rs;
pub use cs_demo_parser_rs::*;
"#;
    std::fs::write(out_dir.join("mod.rs"), mod_rs)?;
    let mut config = prost_build::Config::new();
    config.out_dir(&out_dir);
    let mut includes: Vec<PathBuf> = vec![in_dir.clone(), vendor_dir.clone()];
    if let Ok(protoc_include) = std::env::var("PROTOC_INCLUDE") {
        includes.push(PathBuf::from(protoc_include));
    }
    // Remove the check for system descriptor.proto, since we now vendor it
    config.compile_protos(&protos, &includes)?;
    // Older prost versions generate a file named `_.rs`. Rename it for
    // consistency so the module is always `cs_demo_parser_rs`.
    let generated = out_dir.join("_.rs");
    if generated.exists() {
        let target = out_dir.join("cs_demo_parser_rs.rs");
        std::fs::rename(generated, target)?;
    }
    Ok(())
}

/// Sets up CS2 demo files by downloading and extracting demo archives.
///
/// If the environment variable FETCH_LATEST_DEMOS is set to "1", any missing demo archives will be downloaded.
/// If the environment variable SYNC_LATEST_DEMOS is set to "1", all demo archives will be re-downloaded (overwrite existing).
/// Otherwise, only the presence of .7z files is checked and no downloads are performed automatically.
///
/// # Environment Variables
/// - `DEMOINFOCS_SKIP_DEMOS` - Skip demo setup entirely
/// - `FETCH_LATEST_DEMOS` - Download any missing demo archives if set to "1"
/// - `SYNC_LATEST_DEMOS` - Force re-download of all demo archives if set to "1"
#[allow(dead_code)]
fn setup_demos() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:warning=setup_demos() starting...");

    if std::env::var_os("DEMOINFOCS_SKIP_DEMOS").is_some() {
        println!("cargo:warning=DEMOINFOCS_SKIP_DEMOS is set, skipping demo setup");
        return Ok(());
    }

    println!("cargo:rerun-if-changed=demos-external");

    // Debug current directory
    if let Ok(cwd) = std::env::current_dir() {
        println!("cargo:warning=Current directory: {}", cwd.display());
    }

    // Check demos-external directory, create if missing
    let demos_external = Path::new("../demos-external");
    if !demos_external.exists() {
        println!("cargo:warning=demos-external directory doesn't exist, creating it...");
        std::fs::create_dir_all(demos_external)?;
    }

    let _fetch_latest = std::env::var("FETCH_LATEST_DEMOS").ok().as_deref() == Some("1");
    let _sync_latest = std::env::var("SYNC_LATEST_DEMOS").ok().as_deref() == Some("1");
    let _demo_archives = [
        "default.7z",
        "broken.7z",
        "overtime-demos.7z",
        "regression-set.7z",
        "retake_unknwon_bombsite_index.7z",
        "s2.7z",
        "unexpected_end_of_demo.7z",
        "valve_matchmaking.7z",
    ];
    let mut has_7z = false;
    if let Ok(entries) = demos_external.read_dir() {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.extension().map(|e| e == "7z").unwrap_or(false) {
                has_7z = true;
                break;
            }
        }
    }
    // Only allow downloads if a flag is set
    // --- DOWNLOAD FUNCTIONALITY DISABLED ---
    /*
    if fetch_latest || sync_latest {
        use std::sync::Arc;
        use std::thread;
        println!("cargo:warning={} demo archives in parallel...", if sync_latest {"Syncing (re-downloading)"} else {"Fetching missing"});
        let demos_external = Arc::new(demos_external.to_path_buf());
        let handles: Vec<_> = demo_archives.iter().map(|filename| {
            let demos_external = Arc::clone(&demos_external);
            let filename = filename.to_string();
            let do_download = sync_latest || !demos_external.join(&filename).exists();
            thread::spawn(move || {
                let dest = demos_external.join(&filename);
                if do_download {
                    let url = format!("https://gitlab.com/markus-wa/cs-demos-2/-/raw/master/{}", filename);
                    println!("cargo:warning=Downloading {}...", filename);
                    match get(&url) {
                        Ok(resp) if resp.status().is_success() => {
                            let bytes = match resp.bytes() {
                                Ok(b) => b,
                                Err(e) => {
                                    println!("cargo:warning=Failed to read bytes for {}: {}", filename, e);
                                    return;
                                }
                            };
                            if let Ok(mut file) = std::fs::File::create(&dest) {
                                if let Err(e) = file.write_all(&bytes) {
                                    println!("cargo:warning=Failed to write {}: {}", filename, e);
                                } else {
                                    println!("cargo:warning=Downloaded {} ({} bytes)", filename, bytes.len());
                                }
                            } else {
                                println!("cargo:warning=Failed to create file {}", dest.display());
                            }
                        }
                        Ok(resp) => {
                            println!("cargo:warning=Failed to download {}: HTTP {}", filename, resp.status());
                        }
                        Err(e) => {
                            println!("cargo:warning=Failed to download {}: {}", filename, e);
                        }
                    }
                }
            })
        }).collect();
        for handle in handles {
            let _ = handle.join();
        }
    } else {
    */
    if !has_7z {
        println!(
            "cargo:warning=No .7z files found in demos-external. Set FETCH_LATEST_DEMOS=1 to \
             fetch missing or SYNC_LATEST_DEMOS=1 to re-download all demos. No downloads will \
             occur unless a flag is set."
        );
    }
    //}

    println!("cargo:warning=Listing demos-external contents:");
    if let Ok(entries) = demos_external.read_dir() {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            let metadata = entry.metadata().ok();
            let size = metadata.as_ref().map(|m| m.len()).unwrap_or(0);
            println!("cargo:warning=  {} ({}bytes)", path.display(), size);
        }
    }

    // Extract demos (downloads real .7z files if needed)
    println!("cargo:warning=Calling extract_demos()...");
    if let Err(e) = extract_demos() {
        eprintln!("cargo:warning=extract_demos failed: {}", e);
    }

    println!("cargo:warning=setup_demos() complete");
    Ok(())
}

/// Downloads the real .7z file from GitLab if the file is a small LFS pointer.
///
/// This is needed because sometimes only a pointer file is present instead of the actual archive.
#[allow(dead_code)]
fn download_real_7z_if_pointer(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let filename = path.file_name().unwrap_or_default().to_string_lossy();
    let size = path.metadata()?.len();
    if size >= 1000 {
        // Already a real file
        return Ok(());
    }
    println!(
        "cargo:warning=Attempting to download real {} from GitLab...",
        filename
    );
    let url = format!(
        "https://gitlab.com/markus-wa/cs-demos-2/-/raw/master/{}",
        filename
    );
    let resp = get(&url)?;
    if !resp.status().is_success() {
        println!(
            "cargo:warning=Failed to download {}: HTTP {}",
            filename,
            resp.status()
        );
        return Err(format!("Failed to download {}: HTTP {}", filename, resp.status()).into());
    }
    let mut file = std::fs::File::create(path)?;
    let bytes = resp.bytes()?;
    file.write_all(&bytes)?;
    println!(
        "cargo:warning=Downloaded {} ({} bytes)",
        filename,
        bytes.len()
    );
    Ok(())
}

/// Extracts all .dem files from .7z archives in the demos-external directory.
///
/// Ensures the output directory exists, checks for .7z files, and attempts to download
/// the real archive if only a pointer is present. Uses sevenz_rust for extraction.
#[allow(dead_code)]
fn extract_demos() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:warning=extract_demos() starting...");
    let demos_dir = Path::new("demos");
    std::fs::create_dir_all(demos_dir)?;
    let demos_external = Path::new("../demos-external");
    if !demos_external.exists() {
        println!("cargo:warning=demos-external directory missing!");
        return Err("demos-external directory missing".into());
    }
    let entries = demos_external.read_dir()?;
    let mut found_7z = false;
    let mut any_error = false;
    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();
        println!("cargo:warning=Checking file: {}", path.display());
        if path.extension().map(|e| e == "7z").unwrap_or(false) {
            found_7z = true;
            let filename = path.file_name().unwrap_or_default().to_string_lossy();
            if let Ok(metadata) = path.metadata() {
                println!(
                    "cargo:warning=Found 7z: {} ({} bytes)",
                    filename,
                    metadata.len()
                );
            }
            // --- DOWNLOAD FUNCTIONALITY DISABLED ---
            /*
            // Download the real file if it's a pointer
            if let Err(e) = download_real_7z_if_pointer(&path) {
                println!("cargo:warning=Could not auto-download {}: {}", filename, e);
            }
            */
            println!("cargo:warning=Attempting to extract {}...", filename);
            match decompress_file(&path, demos_dir) {
                | Ok(_) => println!("cargo:warning=Successfully extracted {}", filename),
                | Err(e) => {
                    println!("cargo:warning=Failed to extract {}: {}", filename, e);
                    any_error = true;
                },
            }
        }
    }
    if !found_7z {
        println!("cargo:warning=No .7z files found in demos-external");
        return Err("No .7z files found".into());
    }
    if any_error {
        Err("Some extractions failed".into())
    } else {
        println!("cargo:warning=All demos extracted successfully");
        Ok(())
    }
}

/// Build script main function that compiles proto files and sets up demo files.
///
/// # Environment Variables
/// - `DEMOINFOCS_SKIP_PROTO` - Skip protocol buffer compilation
/// - `DEMOINFOCS_SKIP_DEMOS` - Skip demo file setup
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:warning=Build script starting...");

    //if std::env::var_os("DEMOINFOCS_SKIP_PROTO").is_none() {
    compile("proto/msg", "msg")?;
    compile("proto/msgs2", "msgs2")?;
    //}

    // Setup demos (non-critical - don't fail build if it fails)

    println!("cargo:warning=Build script complete");
    Ok(())
}
