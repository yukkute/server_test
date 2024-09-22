use std::net::{Ipv4Addr, SocketAddr};

use super::services::{get_more_onigiri_services, get_port};

pub async fn start_local_server() -> Option<u16> {
    let port = get_port();

    let loopback = Ipv4Addr::LOCALHOST;
    let addr: SocketAddr = format!("{}:{}", loopback, port)
        .parse()
        .expect("address is valid");

    debug_assert!(addr.is_ipv4());

    // TODO: move reflection to services as static variable, currently not
    // possible cause depends on "reflection_service" type to be named
    //
    //     ServerReflectionServer<impl ServerReflection>
    //     ↓
    let Ok(reflection_service) = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(include_bytes!("../generated/descriptor.bin"))
        .build_v1()
    else {
        return None;
    };

    let more_onigiri_services = get_more_onigiri_services();
    //= crate::mo_server::services::scheme::more_onigiri_services_server::MoreOnigiriServicesServer::new(crate::mo_server::services::GrpcServer { port });

    let s = tonic::transport::Server::builder()
        .add_service(more_onigiri_services)
        .add_service(reflection_service);

    tokio::spawn(async move {
        match s.serve(addr).await {
            Ok(_) => {}
            Err(e) => {
                dbg!(e);
            }
        };
    });

    println!("rust: local server listening on {}", port);
    std::io::Write::flush(&mut std::io::stdout()).unwrap(); // flush

    Some(port)
}
