use crate::scheme::data_request_server::{DataRequest, DataRequestServer};
use crate::scheme::{CounterRequest, CounterResponse};
use rand::Rng;
use tonic::transport::Server;
use tonic_reflection::server::Builder;

pub mod scheme {
    tonic::include_proto!("scheme"); // Path to your .proto file
}

use std::net::TcpListener;

fn get_available_port() -> Option<u16> {
    (8000..9000).find(|port| TcpListener::bind(("127.0.0.1", *port)).is_ok())
}

#[derive(Default)]
pub struct MyDataRequest {}

#[tonic::async_trait]
impl DataRequest for MyDataRequest {
    async fn get_counter(
        &self,
        request: tonic::Request<CounterRequest>,
    ) -> Result<tonic::Response<CounterResponse>, tonic::Status> {
        println!("Got a request: {:?}", request);
        let reply = CounterResponse {
            counter: rand::thread_rng().gen_range(1..=100), // Example counter value
        };
        Ok(tonic::Response::new(reply))
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
    let data_request = DataRequestServer::new(MyDataRequest::default());

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
