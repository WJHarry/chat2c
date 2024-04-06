// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::thread::{self, sleep};
use std::net::UdpSocket;
use std::string::ToString;
use std::sync::Arc;
use std::time::Duration;
use lazy_static::lazy_static;
use tauri::Window;
use uuid::Uuid;
use dashmap::DashMap;

mod network;
mod service;
mod connection;
mod message;

const SERVER_ADDR : &str = "wjharry.tech:19999";

lazy_static!{
    static ref MY_UID: String = Uuid::new_v4().to_string();
    static ref SOCKET: Arc<UdpSocket> = Arc::new(network::udp::bind_udp_socket());
    static ref CONNECTIONS : DashMap<String, connection::Connection> = Default::default();
    static ref MESSAGES : DashMap<message::Message, u8> = Default::default();
}


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn my_uid_get() -> String {
    (*MY_UID).clone()
}

#[tauri::command]
fn connect(target_uid: String) {
    service::connect::connect(Arc::clone(&SOCKET), String::from(target_uid.trim()));
}

static mut FLAG :u8= 0;

#[derive(Clone, serde::Serialize)]
struct Payload {
    target_uid: String
}

#[tauri::command]
fn init_process(window: Window) {
    unsafe {
        if FLAG > 0 {
            return;
        }

        thread::spawn(move || {
            loop {
                FLAG = 1;
                for item in CONNECTIONS.iter() {
                    if item.value().status == connection::ConnStatus::ESTABLISHED {
                        let target_uid = item.key().into();
                        window.emit("established_connect", Payload{ target_uid }).unwrap();
                        break;
                    }
                }
                window.emit("established_connect", Payload{target_uid: String::from("")}).unwrap();
                sleep(Duration::from_secs(1));
            }
        });

    }
}

#[tauri::command]
fn send_message(message: String) {
    for item in CONNECTIONS.iter() {
        service::message_service::do_send(Arc::clone(&SOCKET), item.value().target_addr.clone(), message);
        return;
    }
    log::error!("None active connection.")
}

#[tauri::command]
fn send_file(file_path: String) {
    let conn = find_unique_established_conn();
    if conn.is_none() {
        return;
    }
    log::info!("Sending file {}", file_path);
    service::file_service::do_send(Arc::clone(&SOCKET), file_path, conn.unwrap().target_uid);
}

#[tauri::command]
fn fetch_messages() -> String {
    let messages: Vec<message::Message> = MESSAGES.iter().map(|i|{i.key().clone()}).collect();
    return serde_json::to_string(&messages).unwrap();
}

fn main() {
    env_logger::init();

    network::udp::start_udp_daemon(&SOCKET);

    thread::spawn(||{
        loop {
            register(Arc::clone(&SOCKET));
            let conn = find_unique_established_conn();
            if conn.is_some() {
                service::connect::connect(Arc::clone(&SOCKET), conn.unwrap().target_uid.clone());
            }
            sleep(Duration::from_secs(10));
        }
    });

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![my_uid_get, connect, init_process, send_message, fetch_messages, send_file])
        .run(tauri::generate_context!())
        .expect("error occurred while running tauri application");
}

fn register(udp_socket: Arc<UdpSocket>) {
    log::info!("Registering to server. My UID: {}", *MY_UID);
    network::udp::udp_client::send(udp_socket, String::from(SERVER_ADDR), format!("register#{}", *MY_UID)).expect("unknown uid");
    log::info!("Registered");
}

fn find_unique_established_conn() -> Option<connection::Connection> {
    for item in CONNECTIONS.iter() {
        if item.value().status == connection::ConnStatus::ESTABLISHED {
            return Some(item.value().clone());
        }
    }
    None
}
