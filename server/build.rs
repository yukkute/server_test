use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let proto_files: Vec<PathBuf> = fs::read_dir(Path::new("../proto"))?
        .filter(|e| e.is_ok())
        .filter_map(|entry| {
            let path = entry.unwrap().path();
            if path.extension()? == "proto" {
                return Some(path);
            }
            None
        })
        .collect();

    tonic_build::configure()
        .build_client(false)
        .build_server(true)
        .file_descriptor_set_path(PathBuf::from("./src/generated/descriptor.bin"))
        .compile_protos(&proto_files, &["../"])?;

    Ok(())
}
