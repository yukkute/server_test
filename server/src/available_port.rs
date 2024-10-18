use std::net::TcpListener;

pub fn get_available_port() -> Option<u16> {
	let Ok(listener) = TcpListener::bind("127.0.0.1:0") else {
		return None;
	};

	match listener.local_addr() {
		Ok(a) => Some(a.port()),
		Err(_) => None,
	}
}
