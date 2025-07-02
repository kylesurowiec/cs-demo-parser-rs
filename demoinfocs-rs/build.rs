use std::path::{Path, PathBuf};

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::var_os("DEMOINFOCS_SKIP_PROTO").is_none() {
        compile("proto/msg", "msg")?;
        // The msgs2 protos are currently unused and fail to compile with modern
        // `protoc` versions. Skip generating Rust code for them until the
        // definitions are updated.
        // compile("proto/msgs2", "msgs2")?;
    }
    Ok(())
}
