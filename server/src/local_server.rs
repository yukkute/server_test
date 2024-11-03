use std::{
	net::{Ipv4Addr, SocketAddr},
	sync::Arc,
};

use log::{error, info};

use crate::{
	available_port::get_available_port,
	pb::{mo_auth_server::MoAuthServer, mo_talking_server::MoTalkingServer},
	runtime::{init_runtime, TOKIO_RUNTIME},
	services::GrpcServer,
};

#[unsafe(no_mangle)]
unsafe extern "C" fn start_local_server() -> u16 {
	start_local_server_rust()
}

static mut STARTED: bool = false;
static mut PORT: u16 = 0;

fn start_local_server_rust() -> u16 {
	unsafe {
		if STARTED {
			return PORT;
		};
		STARTED = true;
	}

	init_runtime();

	let Some(port) = get_available_port() else {
		error!("could not bind a port");
		panic!();
	};

	unsafe { PORT = port };

	let loopback = Ipv4Addr::LOCALHOST;
	let addr: SocketAddr = format!("{loopback}:{port}").parse().unwrap();

	debug_assert!(addr.is_ipv4());

	let reflection_service = tonic_reflection::server::Builder::configure()
		.include_reflection_service(true)
		.register_encoded_file_descriptor_set(include_bytes!("generated/descriptor.bin"))
		.build_v1alpha()
		.unwrap();

	let grpc_server = Arc::new(GrpcServer::new(port));

	let talking_service = MoTalkingServer::from_arc(grpc_server.clone());
	let auth_service = MoAuthServer::from_arc(grpc_server.clone());

	let well_built_server = tonic::transport::Server::builder()
		.add_service(talking_service)
		.add_service(auth_service)
		.add_service(reflection_service);

	TOKIO_RUNTIME.spawn(async move { well_built_server.serve(addr).await });

	info!("Personal server listening on {loopback}:{port}");

	port
}
