    use std::path::{Path, PathBuf};
use std::process::Command;
use sevenz_rust::decompress_file;

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
   
   // Check if submodule exists
   let submodule_git = Path::new("demos-external/.git");
   println!("cargo:warning=Checking for {}", submodule_git.display());
   
   if !submodule_git.exists() {
       println!("cargo:warning=.git not found, initializing submodule...");
       
       let output = Command::new("git")
           .args(&["submodule", "update", "--init", "--recursive", "demos-external"])
           .output()?;
           
       println!("cargo:warning=Submodule init stdout: {}", String::from_utf8_lossy(&output.stdout));
       println!("cargo:warning=Submodule init stderr: {}", String::from_utf8_lossy(&output.stderr));
       println!("cargo:warning=Submodule init status: {}", output.status);
           
       if !output.status.success() {
           return Ok(());
       }
   } else {
       println!("cargo:warning=Submodule .git exists");
   }

   // Check demos-external directory
   let demos_external = Path::new("demos-external");
   if !demos_external.exists() {
       println!("cargo:warning=demos-external directory doesn't exist!");
       return Ok(());
   }
   
   println!("cargo:warning=Listing demos-external contents:");
   if let Ok(entries) = demos_external.read_dir() {
       for entry in entries.filter_map(Result::ok) {
           let path = entry.path();
           let metadata = entry.metadata().ok();
           let size = metadata.as_ref().map(|m| m.len()).unwrap_or(0);
           println!("cargo:warning=  {} ({}bytes)", path.display(), size);
       }
   }

   // Check if we need LFS
   let mut needs_lfs = false;
   if let Ok(entries) = demos_external.read_dir() {
       for entry in entries.filter_map(Result::ok) {
           let path = entry.path();
           if path.extension().map(|e| e == "7z").unwrap_or(false) {
               if let Ok(metadata) = entry.metadata() {
                   if metadata.len() < 1000 {
                       println!("cargo:warning=Found small .7z file (LFS pointer): {} ({} bytes)", 
                               path.display(), metadata.len());
                       needs_lfs = true;
                   }
               }
           }
       }
   }

   if needs_lfs {
       println!("cargo:warning=Need to download LFS files");
       
       // Install LFS
       println!("cargo:warning=Running git lfs install...");
       let install_output = Command::new("git")
           .args(&["lfs", "install"])
           .output()?;
       println!("cargo:warning=LFS install status: {}", install_output.status);

       // Pull LFS files
       println!("cargo:warning=Running git lfs pull in demos-external...");
       let pull_output = Command::new("git")
           .args(&["-C", "demos-external", "lfs", "pull"])
           .output()?;
           
       println!("cargo:warning=LFS pull stdout: {}", String::from_utf8_lossy(&pull_output.stdout));
       println!("cargo:warning=LFS pull stderr: {}", String::from_utf8_lossy(&pull_output.stderr));
       println!("cargo:warning=LFS pull status: {}", pull_output.status);
   } else {
       println!("cargo:warning=LFS files appear to be downloaded already");
   }
   
   // Extract demos
   println!("cargo:warning=Calling extract_demos()...");
   if let Err(e) = extract_demos() {
       eprintln!("cargo:warning=extract_demos failed: {}", e);
   }
   
   println!("cargo:warning=setup_demos() complete");
   Ok(())
}

/// Extracts .dem files from all .7z archives in the demos-external directory.
///
/// # Requirements
/// - Uses sevenz_rust to decompress files
///
/// # Errors
/// Returns an error if extraction fails
fn extract_demos() -> Result<(), Box<dyn std::error::Error>> {
   println!("cargo:warning=extract_demos() starting...");
   
   let demos_dir = Path::new("demos");
   println!("cargo:warning=Creating demos directory...");
   std::fs::create_dir_all(demos_dir)?;
   
   let demos_external = Path::new("demos-external");
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
               println!("cargo:warning=Found 7z: {} ({} bytes)", filename, metadata.len());
           }
           
           println!("cargo:warning=Attempting to extract {}...", filename);
           
           match decompress_file(&path, demos_dir) {
               Ok(_) => println!("cargo:warning=Successfully extracted {}", filename),
               Err(e) => {
                   println!("cargo:warning=Failed to extract {}: {}", filename, e);
                   any_error = true;
               }
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
   
   if std::env::var_os("DEMOINFOCS_SKIP_PROTO").is_none() {
       compile("proto/msg", "msg")?;
   }
   
   // Setup demos (non-critical - don't fail build if it fails)
   if let Err(e) = setup_demos() {
       eprintln!("cargo:warning=Demo setup failed: {}", e);
   }
   
   println!("cargo:warning=Build script complete");
   Ok(())
}