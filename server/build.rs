use std::{
	error::Error,
	fs,
	path::{Path, PathBuf},
};

fn main() -> Result<(), Box<dyn Error>> {
	let proto_files: Vec<PathBuf> = fs::read_dir(Path::new("../proto"))?
		.flatten()
		.filter_map(|entry| {
			let path = entry.path();
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
