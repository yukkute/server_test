use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let original_out_dir = PathBuf::from("./src/generated");
    tonic_build::configure()
        .file_descriptor_set_path(original_out_dir.join("descriptor.bin"))
        .compile(&["../data.proto"], &["../"])?;

    Ok(())
}
