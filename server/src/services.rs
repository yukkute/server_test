use crate::services::scheme::more_onigiri_services_server::MoreOnigiriServices;
use futures_util::StreamExt;
use rand::Rng;
use scheme::{DataRequest, DataResponse, Pong};
use std::pin::Pin;
use tokio::time::Duration;
use tonic::{Request, Response, Status};

pub mod scheme {
    tonic::include_proto!("scheme");
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
