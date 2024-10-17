#![allow(dead_code)]

use std::{
    net::SocketAddrV4,
    sync::atomic::{AtomicBool, Ordering},
};
use tokio::{
    net::UdpSocket,
    time::{interval, Duration},
};

static MULTICASTING_RUNNING: AtomicBool = AtomicBool::new(false);

pub fn start_multicasting(grpc_port: u16) {
    if MULTICASTING_RUNNING.swap(true, Ordering::SeqCst) {
        return;
    }

    tokio::spawn(async move {
        let socket = UdpSocket::bind("0.0.0.0:0").await.unwrap();

        let multicast_addr: SocketAddrV4 = "233.252.0.0:4445"
            .parse()
            .expect("hardcoded multicast address is valid");

        let mut interval = interval(Duration::from_millis(500));
        loop {
            interval.tick().await;

            if !MULTICASTING_RUNNING.load(Ordering::SeqCst) {
                break;
            }

            let message = format!("mo_{}", grpc_port);
            debug_assert!(message.is_ascii());

            let _ = socket.send_to(message.as_bytes(), multicast_addr).await;
        }
    });
}

pub fn stop_multicasting() {
    MULTICASTING_RUNNING.store(false, Ordering::SeqCst);
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::net::SocketAddrV4;
    use tokio::{
        net::UdpSocket,
        time::{timeout, Duration},
    };

    #[tokio::test]
    async fn get_port_from_multicast() {
        let target_port = 50051;

        // -----

        let multicast_addr: SocketAddrV4 = "233.252.0.0:4445".parse().unwrap();
        let listener = UdpSocket::bind("0.0.0.0:4445").await.unwrap();
        listener
            .join_multicast_v4(*multicast_addr.ip(), "0.0.0.0".parse().unwrap())
            .unwrap();

        // -----

        start_multicasting(target_port);

        let message = {
            let mut buf = [0u8; 8];
            let _ = timeout(Duration::from_secs(2), listener.recv_from(&mut buf)).await;
            String::from_utf8(buf.to_vec()).unwrap()
        };

        stop_multicasting();
        assert_eq!(message, format!("mo_{}", target_port));
    }
}
