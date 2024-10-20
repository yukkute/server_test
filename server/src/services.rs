use std::{
	pin::Pin,
	sync::{Arc, Mutex},
};

use log::info;
use tokio::time::Duration;
use tonic::{Request, Response, Status};

use crate::{
	data::ServerData,
	pb::{
		mo_auth_server::MoAuth, mo_talking_server::MoTalking, Empty, MoClientDatagram,
		MoServerDatagram, SessionCredentials, UserCredentials,
	},
	runtime::TOKIO_RUNTIME,
};

pub struct GrpcServer {
	port: u16,
	clock: tokio::sync::broadcast::Sender<()>,
	server_data: Mutex<ServerData>,
	counter: Arc<Mutex<u8>>,
}

impl GrpcServer {
	pub fn new(port: u16) -> Self {
		let (tx, _) = tokio::sync::broadcast::channel::<()>(16);

		let clock = tx.clone();

		let grpc = Self {
			port,
			clock,
			server_data: Mutex::default(),
			counter: Arc::default(),
		};

		let c = grpc.counter.clone();

		TOKIO_RUNTIME.spawn(async move {
			loop {
				tokio::time::sleep(Duration::from_millis(50)).await;

				let mut num = c.lock().unwrap();
				*num = num.wrapping_add(1);

				let _ = tx.send(());
			}
		});

		grpc
	}

	fn session_valid(&self, s: Option<SessionCredentials>) -> bool {
		let Some(s) = s else { return false };

		let mut server_data = self.server_data.lock().unwrap();

		server_data.userdata.has_valid_session(&s.username, &s.id)
	}
}

#[tonic::async_trait]
impl MoTalking for GrpcServer {
	type RequestServerClockStream =
		Pin<Box<dyn futures_util::Stream<Item = Result<Empty, Status>> + Send>>;

	async fn request_server_clock(
		&self,
		request: Request<Empty>,
	) -> Result<Response<Self::RequestServerClockStream>, Status> {
		info!("Requested clock!");
		info!("{request:?}");

		let (tx, rx) = tokio::sync::mpsc::channel::<Result<Empty, Status>>(16);

		let mut clock_reciever = self.clock.subscribe();

		TOKIO_RUNTIME.spawn(async move {
			loop {
				clock_reciever.recv().await.unwrap();

				if tx.send(Ok(Empty {})).await.is_err() {
					break;
				}
			}
		});

		let stream = tokio_stream::wrappers::ReceiverStream::new(rx);
		Ok(Response::new(Box::pin(stream)))
	}

	async fn get_data(
		&self,
		request: Request<MoClientDatagram>,
	) -> Result<Response<MoServerDatagram>, Status> {
		if !self.session_valid(request.into_inner().session_id) {
			return Err(Status::unauthenticated("authentication failed"));
		};

		//let counter = rand::thread_rng().gen_range(0..100);
		let response = MoServerDatagram {
			counter: u32::from(*self.counter.lock().unwrap()),
		};

		Ok(Response::new(response))
	}
}

#[tonic::async_trait]
impl MoAuth for GrpcServer {
	async fn register(&self, request: Request<UserCredentials>) -> Result<Response<Empty>, Status> {
		let request = &request.into_inner();
		let a = &mut self.server_data.lock().unwrap().userdata;

		if let Err(e) = a.register(&request.username, &request.password) {
			return Err(Status::unauthenticated(format!(
				"registration failed with error: {e:?}"
			)));
		}

		return Ok(Response::new(Empty {}));
	}

	async fn authenticate(
		&self,
		request: Request<UserCredentials>,
	) -> Result<Response<SessionCredentials>, Status> {
		let request = &request.into_inner();
		let a = &mut self.server_data.lock().unwrap().userdata;

		match a.authenticate(&request.username, &request.password) {
			Ok(id) => Ok(Response::new(SessionCredentials {
				username: request.username.clone(),
				id,
			})),
			Err(e) => Err(Status::unauthenticated(format!(
				"auth failed with error: {e:?}"
			))),
		}
	}
}
