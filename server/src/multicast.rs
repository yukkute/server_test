use std::net::SocketAddrV4;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::net::UdpSocket;
use tokio::time::{interval, Duration};

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

        // let multicast_addr_ipv6: SocketAddrV6 = SocketAddrV6::new(
        //     Ipv6Addr::new(0xff02, 0, 0, 0, 0, 0xae, 0x21, 0x88),
        //     4445,
        //     0,
        //     0,
        // );
        // let _ = socket
        //     .join_multicast_v6(multicast_addr_ipv6.ip(), 0)
        //     .is_ok();

        let mut interval = interval(Duration::from_millis(500));
        loop {
            interval.tick().await;

            if !MULTICASTING_RUNNING.load(Ordering::SeqCst) {
                break;
            }

            let message = format!("mo_{}", grpc_port);
            debug_assert!(message.is_ascii());

            let _ = socket.send_to(message.as_bytes(), multicast_addr).await;

            // let _ = socket
            //     .send_to(message.as_bytes(), multicast_addr_ipv6)
            //     .await;
        }
    });
}

#[allow(dead_code)]
pub fn stop_multicasting() {
    MULTICASTING_RUNNING.store(false, Ordering::SeqCst);
}
