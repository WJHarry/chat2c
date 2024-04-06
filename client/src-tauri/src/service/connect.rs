use std::net::UdpSocket;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use crate::{CONNECTIONS, MY_UID, SERVER_ADDR};
use crate::connection;
use crate::network::udp::udp_client;

pub fn connect(udp_socket: Arc<UdpSocket>, target_uid: String) {
    // if (*CONNECTIONS).into_iter().any(|i| i.1.status != connection::ConnStatus::DISCONNECTED) {
    //     log::error!("There is another active connection.");
    //     return;
    // }
    log::info!("Connecting to {}", target_uid);
    let connect_result = udp_client::send(udp_socket, String::from(SERVER_ADDR), format!("connect#{}_{}", *MY_UID, target_uid));
    if connect_result.is_err() {
        log::error!("Error while connecting to {}, {}", target_uid, connect_result.err().unwrap());
        return;
    }
    CONNECTIONS.clear();
    CONNECTIONS.insert(target_uid.clone(), connection::Connection{target_uid: target_uid.clone(), target_addr: String::from(""), status: connection::ConnStatus::INVITED});

    log::info!("{}", (*CONNECTIONS).len());
    log::info!("Waiting response of connecting to {}", target_uid);

    sleep(Duration::from_secs(5));
    if CONNECTIONS.get(&target_uid).expect("Disconnected.").status <= connection::ConnStatus::INVITED {
        log::error!("Timeout to connect to {}, please check your network and try again later.", target_uid);
        return;
    }
}

pub fn connect_callback(udp_socket: Arc<UdpSocket>, content: String) {
    let content_seps: Vec<&str> = content.split('_').collect();
    let target_addr = String::from(content_seps[1]);
    let target_uid = String::from(content_seps[0]);

    let exist_conn = CONNECTIONS.get(&target_uid);
    if exist_conn.is_some() && exist_conn.unwrap().value().status == connection::ConnStatus::ESTABLISHED {
        log::info!("connection has established");
        return;
    }
    CONNECTIONS.insert(
        target_uid.clone(),
        connection::Connection{target_uid: target_uid.clone(), target_addr: target_addr.clone(), status: connection::ConnStatus::AGREED});
    for _ in 0..5 {
        udp_client::send(Arc::clone(&udp_socket), target_addr.clone(), format!("connect_test#{}", *MY_UID)).unwrap();
        sleep(Duration::from_millis(100));
    }
}

pub fn connect_test_callback(_udp_socket: Arc<UdpSocket>, content: String) {
    let conn = CONNECTIONS.get_mut(&content);
    conn.unwrap().value_mut().status = connection::ConnStatus::ESTABLISHED;

    let connections: Vec<String> = CONNECTIONS.iter().map(|c| {c.value().to_string()}).collect();
    log::info!("ctc {:?}", connections);
}