use std::{
	error::Error,
	fs,
	path::{Path, PathBuf},
};

fn main() -> Result<(), Box<dyn Error>> {
	let proto_files: Vec<PathBuf> = fs::read_dir(Path::new("../proto"))?
		.flatten()
		.map(|e| e.path())
		.filter(|p| p.extension().unwrap() == "proto")
		.collect();

	tonic_build::configure()
		.build_client(false)
		.build_server(true)
		.file_descriptor_set_path(PathBuf::from("./src/generated/descriptor.bin"))
		.compile_protos(&proto_files, &[PathBuf::from("../proto/")])?;

	Ok(())
}
