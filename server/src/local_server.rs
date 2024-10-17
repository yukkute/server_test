use crate::{
    available_port::get_available_port, pb::mo_talking_server::MoTalkingServer,
    services::GrpcServer,
};
use log::{error, info};
use simplelog::*;
use std::net::{Ipv4Addr, SocketAddr};
use std::sync::OnceLock;
use tokio::runtime::Runtime;

static TOKIO_RUNTIME: OnceLock<Runtime> = OnceLock::new();

#[no_mangle]
pub extern "C" fn start_local_server() -> u16 {
    let runtime = TOKIO_RUNTIME.get_or_init(|| {
        let log_config = ConfigBuilder::default()
            .set_time_format_custom(format_description!(
                "[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:3]"
            ))
            .build();

        let log_filter = LevelFilter::Info;

        let color_choice = if atty::is(atty::Stream::Stdout) {
            ColorChoice::Auto
        } else {
            ColorChoice::Never
        };

        CombinedLogger::init(vec![TermLogger::new(
            log_filter,
            log_config,
            TerminalMode::Mixed,
            color_choice,
        )])
        .unwrap();

        Runtime::new().unwrap()
    });

    let Some(port) = get_available_port() else {
        error!("could not bind a port");
        panic!();
    };

    let loopback = Ipv4Addr::LOCALHOST;
    let addr: SocketAddr = format!("{}:{}", loopback, port).parse().unwrap();

    debug_assert!(addr.is_ipv4());

    let reflection_service = tonic_reflection::server::Builder::configure()
        .include_reflection_service(true)
        .register_encoded_file_descriptor_set(include_bytes!("generated/descriptor.bin"))
        .build_v1alpha()
        .unwrap();

    let talking_service = MoTalkingServer::new(GrpcServer { port });

    let well_built_server = tonic::transport::Server::builder()
        .add_service(talking_service)
        .add_service(reflection_service);

    // Use the runtime to spawn the server
    runtime.spawn(async move { well_built_server.serve(addr).await });

    info!("Personal server listening on {}:{}", loopback, port);

    port
}
