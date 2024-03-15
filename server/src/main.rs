use crate::scheme::more_onigiri_services_server::{MoreOnigiriServices, MoreOnigiriServicesServer};
use futures_util::StreamExt;
use rand::Rng;
use scheme::{DataRequest, DataResponse, Pong};
use std::{net::TcpListener, pin::Pin};
use tokio::time::Duration;
use tonic::{transport::Server, Request, Response, Status};
use tonic_reflection::server::Builder;

pub mod scheme {
    tonic::include_proto!("scheme"); // Path to your .proto file
}

fn get_available_port() -> Option<u16> {
    (8000..9000).find(|port| TcpListener::bind(("127.0.0.1", *port)).is_ok())
}

pub struct RsServer {
    port: u16,
}

#[tonic::async_trait]
impl MoreOnigiriServices for RsServer {
    type GetDataStream = Pin<
        Box<dyn tokio_stream::Stream<Item = Result<DataResponse, Status>> + Send + Sync + 'static>,
    >;

    async fn send_ping(
        &self,
        _request: tonic::Request<scheme::Empty>,
    ) -> Result<Response<Pong>, Status> {
        let reply = Pong {
            port: self.port.to_string(),
        };

        Ok(tonic::Response::new(reply))
    }

    async fn get_data(
        &self,
        _request: Request<DataRequest>,
    ) -> Result<Response<Self::GetDataStream>, Status> {
        let stream = tokio_stream::wrappers::IntervalStream::new(tokio::time::interval(
            Duration::from_millis(1000 / 3),
        ))
        .map(|_| {
            let counter = rand::thread_rng().gen_range(1000..=9999);
            Ok(DataResponse { counter })
        });

        Ok(Response::new(Box::pin(stream)))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Some(port) = get_available_port() else {
        println!("no port found");
        panic!();
    };

    let addr = format!("[::1]:{:4}", port).parse()?;
    println!("server running on port {:?}", addr);
    let data_request = MoreOnigiriServicesServer::new(RsServer { port });

    let reflection_service = Builder::configure()
        .register_encoded_file_descriptor_set(include_bytes!("generated/descriptor.bin"))
        .build()?;

    Server::builder()
        .add_service(data_request)
        .add_service(reflection_service)
        .serve(addr)
        .await?;

    Ok(())
}
