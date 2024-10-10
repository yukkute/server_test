pub(crate) mod scheme {
    tonic::include_proto!("scheme");
}

use futures_util::StreamExt;
use rand::Rng;
use scheme::{MoClientDatagram, MoServerDatagram, Tick};
use std::{
    pin::Pin,
    sync::{Mutex, Once},
};
use tokio::time::Duration;
use tonic::{Request, Response, Status};

use self::scheme::mo_talking_server::{MoTalking, MoTalkingServer};

use super::available_port::get_available_port;

static SERVICES_INITIALIZATION: Once = Once::new();
static PORT: Mutex<Option<u16>> = Mutex::new(None);
static MO_TALKING_SERVER: Mutex<Option<MoTalkingServer<GrpcServer>>> =
    Mutex::new(None);

fn init() {
    SERVICES_INITIALIZATION.call_once(|| {
        // Acquire port
        let Some(port) = get_available_port() else {
            println!("no port found");
            panic!();
        };
        PORT.lock().unwrap().replace(port);

        let talking_service = MoTalkingServer::new(GrpcServer { port });
        MO_TALKING_SERVER
            .lock()
            .unwrap()
            .replace(talking_service);
    });
}

pub fn get_talking_service() -> MoTalkingServer<GrpcServer> {
    init();
    let Some(ref s) = *MO_TALKING_SERVER.lock().unwrap() else {
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
impl MoTalking for GrpcServer {
    type RequestServerClockStream = Pin<Box<dyn futures_util::Stream<Item = Result<Tick, Status>> + Send>>;

    async fn request_server_clock(
        &self,
        _request: Request<scheme::Empty>,
    ) -> Result<Response<Self::RequestServerClockStream>, Status> {

        let port = self.port;
        let stream = tokio_stream::iter(0..).then(move |_| {
            let port = port.to_string();
            async move {

                tokio::time::sleep(Duration::from_millis(50)).await;
                
                let tick = Tick { port: port.clone() };
                Ok(tick)
            }
        });
        Ok(Response::new(Box::pin(stream)))
    }

    async fn get_data(
        &self,
        request: Request<MoClientDatagram>,
    ) -> Result<Response<MoServerDatagram>, Status> {
        
        let version = request.into_inner().version;

        // Generate a random counter value for the response
        let counter = rand::thread_rng().gen_range(0..100);

        // Create a MoServerDatagram response
        let response = MoServerDatagram { counter };

        // You can add logic based on the version if needed
        println!("Received version: {}, responding with counter: {}", version, counter);

        // Send the response back to the client
        Ok(Response::new(response))
    }
}
