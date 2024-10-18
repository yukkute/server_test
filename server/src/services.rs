use crate::pb::{mo_talking_server::MoTalking, Empty, MoClientDatagram, MoServerDatagram, Tick};
use futures_util::StreamExt;
use rand::Rng;
use std::pin::Pin;
use tokio::time::Duration;
use tonic::{Request, Response, Status};

pub struct GrpcServer {
	pub port: u16,
}

#[tonic::async_trait]
impl MoTalking for GrpcServer {
	type RequestServerClockStream =
		Pin<Box<dyn futures_util::Stream<Item = Result<Tick, Status>> + Send>>;

	async fn request_server_clock(
		&self,
		_request: Request<Empty>,
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
		println!(
			"Received version: {}, responding with counter: {}",
			version, counter
		);

		// Send the response back to the client
		Ok(Response::new(response))
	}
}
