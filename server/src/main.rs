use crate::{
    multicast::start_multicasting,
    services::{scheme::more_onigiri_services_server::MoreOnigiriServicesServer, GrpcServer},
};
use std::net::{Ipv4Addr, SocketAddr, TcpListener};

mod multicast;
mod services;

fn get_available_port() -> Option<u16> {
    let Ok(listener) = TcpListener::bind("127.0.0.1:0") else {
        return None;
    };

    match listener.local_addr() {
        Ok(a) => Some(a.port()),
        Err(_) => None,
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let loopback = Ipv4Addr::UNSPECIFIED;

    let Some(port) = get_available_port() else {
        println!("no port found");
        panic!();
    };

    start_multicasting(port);

    let addr: SocketAddr = format!("{}:{}", loopback, port).parse()?;
    debug_assert!(addr.is_ipv4());

    println!("server listening on {:?}", addr);

    let data_request = MoreOnigiriServicesServer::new(GrpcServer { port });

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(include_bytes!("generated/descriptor.bin"))
        .build()?;

    tonic::transport::Server::builder()
        .add_service(data_request)
        .add_service(reflection_service)
        .serve(addr)
        .await?;

    Ok(())
}
