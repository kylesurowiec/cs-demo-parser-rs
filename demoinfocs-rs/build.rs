use std::path::{Path, PathBuf};
use std::process::Command;

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
           } else if path.extension().map_or(false, |ext| ext == "proto") {
               files.push(path);
           }
       }
   }
   Ok(files)
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
           | _ => true,
       }
   });
   let mut mod_rs = String::new();
   for proto in &protos {
       println!("cargo:rerun-if-changed={} ", proto.display());
       if let Some(stem) = proto.file_stem().and_then(|s| s.to_str()) {
           mod_rs.push_str(&format!("pub mod {};\n", stem));
       }
   }
   std::fs::write(out_dir.join("mod.rs"), mod_rs)?;
   let mut config = prost_build::Config::new();
   config.out_dir(&out_dir);
   config.compile_protos(&protos, &[in_dir.as_path(), Path::new("/usr/include")])?;
   Ok(())
}

/// Sets up CS2 demo files by initializing the demos submodule and downloading LFS files.
///
/// This function:
/// 1. Initializes the demos-external submodule if not already present
/// 2. Pulls Git LFS files if they haven't been downloaded
/// 3. Optionally extracts .7z archives to the demos/ directory
///
/// # Environment Variables
/// - `DEMOINFOCS_SKIP_DEMOS` - Set to skip demo setup entirely
///
/// # Errors
/// Returns errors for critical failures but prints warnings for non-critical issues.
/// Demo setup failures won't fail the build.
fn setup_demos() -> Result<(), Box<dyn std::error::Error>> {
   // Skip if explicitly disabled
   if std::env::var_os("DEMOINFOCS_SKIP_DEMOS").is_some() {
       return Ok(());
   }

   println!("cargo:rerun-if-changed=demos-external");
   
   // Check if submodule is initialized
   if !Path::new("demos-external/.git").exists() {
       println!("cargo:warning=Initializing demos submodule...");
       
       let output = Command::new("git")
           .args(&["submodule", "update", "--init", "--recursive"])
           .output()?;
           
       if !output.status.success() {
           eprintln!("Warning: Failed to initialize submodule: {}", 
                    String::from_utf8_lossy(&output.stderr));
           // Don't fail the build, just warn
           return Ok(());
       }
   }
   
   // Check if LFS files need to be pulled
   let needs_lfs = Path::new("demos-external")
       .read_dir()
       .map(|entries| {
           entries.filter_map(Result::ok)
               .any(|entry| {
                   entry.path().extension()
                       .map(|ext| ext == "7z")
                       .unwrap_or(false)
                       && entry.metadata()
                           .map(|m| m.len() < 1000) // LFS pointer files are tiny
                           .unwrap_or(false)
               })
       })
       .unwrap_or(true);
   
   if needs_lfs {
       println!("cargo:warning=Downloading demo files via Git LFS...");
       
       let output = Command::new("git")
           .current_dir("demos-external")
           .args(&["lfs", "pull"])
           .output()?;
           
       if !output.status.success() {
           eprintln!("Warning: Failed to pull LFS files: {}", 
                    String::from_utf8_lossy(&output.stderr));
           eprintln!("cargo:warning=Demos may not be available. Run 'git lfs pull' in demos-external/");
       }
   }
   
   // Optional: Extract demos if 7z is available
   if !Path::new("demos").exists() || is_demos_empty() {
       if let Err(e) = extract_demos() {
           eprintln!("cargo:warning=Failed to extract demos: {}", e);
           eprintln!("cargo:warning=Please extract .7z files from demos-external/ manually");
       }
   }
   
   Ok(())
}

/// Checks if the demos directory exists but is empty.
///
/// # Returns
/// `true` if the demos directory doesn't exist or contains no entries
fn is_demos_empty() -> bool {
   Path::new("demos")
       .read_dir()
       .map(|mut entries| entries.next().is_none())
       .unwrap_or(true)
}

/// Extracts .dem files from all .7z archives in the demos-external directory.
///
/// # Requirements
/// - 7-Zip must be installed and available in PATH or standard locations
///
/// # Errors
/// Returns an error if 7-Zip is not found or if extraction fails
fn extract_demos() -> Result<(), Box<dyn std::error::Error>> {
   // Create demos directory
   std::fs::create_dir_all("demos")?;
   
   // Try to find 7z
   let seven_zip_paths = if cfg!(windows) {
       vec![
           "7z",
           "C:\\Program Files\\7-Zip\\7z.exe",
           "C:\\Program Files (x86)\\7-Zip\\7z.exe",
       ]
   } else {
       vec!["7z", "/usr/bin/7z", "/usr/local/bin/7z"]
   };
   
   let seven_zip = seven_zip_paths.iter()
       .find(|path| {
           Command::new(path)
               .arg("--help")
               .output()
               .map(|o| o.status.success())
               .unwrap_or(false)
       });
   
   if let Some(cmd) = seven_zip {
       println!("cargo:warning=Extracting demo archives...");
       
       // Extract each 7z file
       if let Ok(entries) = Path::new("demos-external").read_dir() {
           for entry in entries.filter_map(Result::ok) {
               let path = entry.path();
               if path.extension().map(|e| e == "7z").unwrap_or(false) {
                   Command::new(cmd)
                       .args(&[
                           "x",
                           "-y",
                           path.to_str().unwrap(),
                           "-odemos/",
                           "*.dem"
                       ])
                       .status()?;
               }
           }
       }
   } else {
       return Err("7z not found. Install from https://www.7-zip.org/".into());
   }
   
   Ok(())
}

/// Build script main function that compiles proto files and sets up demo files.
///
/// # Environment Variables
/// - `DEMOINFOCS_SKIP_PROTO` - Skip protocol buffer compilation
/// - `DEMOINFOCS_SKIP_DEMOS` - Skip demo file setup
fn main() -> Result<(), Box<dyn std::error::Error>> {
   if std::env::var_os("DEMOINFOCS_SKIP_PROTO").is_none() {
       compile("proto/msg", "msg")?;
       // The msgs2 protos are currently unused and fail to compile with modern
       // `protoc` versions. Skip generating Rust code for them until the
       // definitions are updated.
       // compile("proto/msgs2", "msgs2")?;
   }
   
   // Setup demos (non-critical - don't fail build if it fails)
   if let Err(e) = setup_demos() {
       eprintln!("cargo:warning=Demo setup failed: {}", e);
   }
   
   Ok(())
}