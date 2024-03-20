pub(crate) mod scheme {
    tonic::include_proto!("scheme");
}

use futures_util::StreamExt;
use rand::Rng;
use scheme::{DataRequest, DataResponse, Pong};
use std::{
    pin::Pin,
    sync::{Mutex, Once},
};
use tokio::time::Duration;
use tonic::{Request, Response, Status};

use self::scheme::more_onigiri_services_server::{MoreOnigiriServices, MoreOnigiriServicesServer};

use super::available_port::get_available_port;

static SERVICES_INITIALIZATION: Once = Once::new();
static PORT: Mutex<Option<u16>> = Mutex::new(None);
static MORE_ONIGIRI_SERVICES: Mutex<Option<MoreOnigiriServicesServer<GrpcServer>>> =
    Mutex::new(None);

fn init() {
    SERVICES_INITIALIZATION.call_once(|| {
        // Acquire port
        let Some(port) = get_available_port() else {
            println!("no port found");
            panic!();
        };
        PORT.lock().unwrap().replace(port);

        // Initialize MOServices
        let more_onigiri_services = MoreOnigiriServicesServer::new(GrpcServer { port });
        MORE_ONIGIRI_SERVICES
            .lock()
            .unwrap()
            .replace(more_onigiri_services);
    });
}

pub fn get_more_onigiri_services() -> MoreOnigiriServicesServer<GrpcServer> {
    init();
    let Some(ref s) = *MORE_ONIGIRI_SERVICES.lock().unwrap() else {
        // Safety: init called
        unreachable!();
    };
    s.clone()
}

pub fn get_port() -> u16 {
    init();
    let Some(p) = *PORT.lock().unwrap() else {
        // Safety: init called
        unreachable!();
    };
    p
}

pub struct GrpcServer {
    pub port: u16,
}

#[tonic::async_trait]
impl MoreOnigiriServices for GrpcServer {
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
