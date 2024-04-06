use std::io::ErrorKind;
use std::net::UdpSocket;
use std::sync::Arc;
use std::thread;
use crate::service::{connect::{connect_callback, connect_test_callback}, message_service::receive_message};
use crate::service::file_service::receiver_file;

pub fn start(arc_socket: Arc<UdpSocket>) -> std::io::Result<()> {
    log::info!("waiting for UDP request on {}", arc_socket.local_addr().unwrap());

    loop {
        let mut buf = [0u8; 1500];
        let result = arc_socket.recv_from(&mut buf);
        match result {
            Ok( (_usize, _socket_addr) ) => {
                let buf = &mut buf[..result.unwrap().0];
                let content = String::from_utf8_lossy(buf).to_string();
                let arc_socket_clone = Arc::clone(&arc_socket);
                log::info!("received message {}", content);
                thread::spawn(move || {
                    request_handler(arc_socket_clone, content);
                });
            },
            Err(e) => {
                if e.kind() != ErrorKind::WouldBlock {
                    log::error!("Error while receiving message {:?}", e);
                }
                continue;
            }
        }
    }
}

fn request_handler(arc_socket: Arc<UdpSocket>, content: String) {
    let content_seps: Vec<&str> = content.split('#').collect();
    log::info!("{}", content_seps[0]);
    match content_seps[0] {
        "connect_callback" => { connect_callback(arc_socket, String::from(content_seps[1])); },
        "connect_test" => { connect_test_callback(arc_socket, String::from(content_seps[1])); },
        "text_message" => { receive_message(arc_socket, String::from(content_seps[1]));},
        "file" => { receiver_file(arc_socket, content.replacen(content_seps[0], "", 1)); }
        _ => {}
    }
}