use std::net::UdpSocket;
use std::sync::Arc;

pub fn send(socket: Arc<UdpSocket>, target_addr: String, data: String) -> std::io::Result<()> {
    socket.send_to(data.as_bytes(), target_addr.clone())
        .expect("data send error");
    log::info!("Sent to {}, {}", target_addr, data);
    return Ok(())
}