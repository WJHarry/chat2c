use std::net::UdpSocket;
use std::thread;
use std::sync::Arc;

pub mod udp_client;
pub mod udp_server;

/// Start UDP server
pub fn start_udp_daemon(arc_socket: &Arc<UdpSocket>) {
    let arc_socket_clone = Arc::clone(&arc_socket);
    thread::spawn(||udp_server::start(arc_socket_clone));
    log::info!("udp server started");
}

pub fn bind_udp_socket() -> UdpSocket {
    let bind_address = "0.0.0.0:18000";
    let socket = UdpSocket::bind(bind_address)
        .expect(&*format!("cannot bind to address {}", bind_address));
    socket.set_nonblocking(true).unwrap();
    socket
}