use std::net::UdpSocket;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::message::Message;
use crate::network::udp::udp_client;
use crate::MESSAGES;

pub fn do_send(udp_socket: Arc<UdpSocket>, target_addr: String, message : String) {
    udp_client::send(Arc::clone(&udp_socket), target_addr.clone(), format!("text_message#{}", message)).unwrap();
    MESSAGES.insert(Message{content: message, timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis(), from: 0}, 0);
}

pub fn receive_message(udp_socket: Arc<UdpSocket>, content: String) {
    MESSAGES.insert(Message{content: content, timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis(), from: 1}, 0);
}