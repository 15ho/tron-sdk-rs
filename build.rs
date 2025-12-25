use std::path::{Path, PathBuf};

use walkdir::WalkDir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let protos = find_proto_files(Path::new("proto/tron"));

    let protos: Vec<&str> = protos
        .iter()
        .map(|p| p.to_str().expect("path to string conversion err"))
        .collect();

    let protos: &[&str] = &protos;

    tonic_prost_build::configure()
        .build_server(false)
        .out_dir("src/tron")
        .compile_protos(protos, &["proto/googleapis", "proto/tron"])?;
    Ok(())
}

fn find_proto_files(root_dir: &Path) -> Vec<PathBuf> {
    let mut proto_files = Vec::new();
    for entry in WalkDir::new(root_dir) {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                eprintln!("Error accessing entry: {}", e);
                continue;
            }
        };

        let path = entry.path();
        if path.extension().is_some_and(|ext| ext == "proto")
            && let Ok(relative_path) = path.strip_prefix(root_dir)
        {
            proto_files.push(relative_path.to_path_buf());
        }
    }
    proto_files
}
