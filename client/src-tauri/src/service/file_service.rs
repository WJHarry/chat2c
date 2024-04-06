use std::{fs, thread};
use std::io::{Read, Write};
use std::net::UdpSocket;
use std::path::Path;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use tauri::api::path::cache_dir;

use crate::CONNECTIONS;
use crate::network::udp::udp_client;

pub fn do_send(udp_socket: Arc<UdpSocket>, file_path: String, target_uid: String) {
    thread::spawn(move || {
        let file_result = fs::File::open(file_path.clone());
        if file_result.is_err() {
            log::error!("File read error {}", file_result.unwrap_err());
            return;
        }
        let mut index = 0;
        let mut file = file_result.unwrap();
        let file_name = Path::new(&file_path).file_name();
        const BUFFER_LEN: usize = 512;
        let mut buffer = [0u8; BUFFER_LEN];
        loop {
            let size = file.read(&mut buffer).unwrap();
            let file_sep = String::from_utf8_lossy(&buffer[..size]);
            let conn_option = CONNECTIONS.get(&target_uid);
            if conn_option.is_none() {
                log::error!("Connection break;");
                return;
            }
            let conn = conn_option.unwrap();
            for _i in 0..3 {
                let send_result = udp_client::send(Arc::clone(&udp_socket), conn.target_addr.clone(), format!("file#{}#{}", index, file_sep));
                if send_result.is_err() {
                    sleep(Duration::from_millis(100));
                    udp_client::send(Arc::clone(&udp_socket), conn.target_addr.clone(),
                        format!("file#{}#{}#{}", file_name.unwrap().to_string_lossy(), index, file_sep)).unwrap();
                } else {
                    sleep(Duration::from_millis(0));
                }
            }
            if size != BUFFER_LEN {
                break;
            }

            index += 1;
        }
        log::info!("File send successfully");
    });
}


pub fn receiver_file(_udp_socket: Arc<UdpSocket>, content: String) {
    let mut i = content.find("#");
    if i.is_none() { return; }
    let (file_name, file_content) = content.split_at(i.unwrap());

    i = content.find("#");
    if i.is_none() { return; }
    let (index_str, file_content) = file_content.split_at(i.unwrap());
    let path_str = format!("{}/tech.wjharry.chat2c/file_{}_{}", cache_dir().unwrap().as_path().to_string_lossy(), index_str, file_name);
    let path = Path::new(&path_str);
    let mut file = match fs::File::create(&path) {
        Err(why) => panic!("couldn't create {}: {:?}", path.display(), why),
        Ok(file) => file,
    };
    file.write_all(file_content.as_bytes()).unwrap();
}