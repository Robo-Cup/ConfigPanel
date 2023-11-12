// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
//use serialport::{available_ports, SerialPortType};
//use serialport::SerialPort; 
use std::sync::OnceLock;
use tauri::{Window, Manager, Wry};
use tauri::State;
// use tauri::async_runtime::Mutex;
use std::sync::Mutex;

use crate::db::{HASHMAP, Value};
mod db;
mod server;

struct BTServer(Mutex<server::Server>);

pub fn put_front_end(key: &str, value: Value) -> () {
    let window = WINDOW.get().expect("window is available");

    // check type of value
    match value {
        Value::Number(num) => {
            let ret_data = format!("{}:{}", key, num);
            window.emit("put-number", ret_data.clone()).expect("emit works");
        }
        Value::String(string) => {
            let ret_data = format!("{}:{}", key, string);
            window.emit("put-string", ret_data.clone()).expect("emit works");
        }
        Value::Boolean(boolean) => {
            let ret_data = format!("{}:{}", key, boolean);
            window.emit("put-boolean", ret_data.clone()).expect("emit works");
        }
    }
}

pub fn put_back_end(key: &str, value: Value) -> () {
    put_value(key, value);
    // print hte hashmap now
    let hashmap = HASHMAP.lock().unwrap();
    println!("hashmap: {:?}", hashmap);
}

pub fn put_full(key: &str, value: Value) -> () {
    put_front_end(key, value.clone());
    put_back_end(key, value);
}

pub fn put_full_json(map: serde_json::Map<String, serde_json::Value>) -> () {
    for (key, value) in map.iter() {
        let db_value = match value {
            serde_json::Value::Number(num) => db::Value::Number(num.as_f64().unwrap()),
            serde_json::Value::String(s) => db::Value::String(s.clone()),
            serde_json::Value::Bool(b) => db::Value::Boolean(*b),
            serde_json::Value::Object(_) | serde_json::Value::Array(_) | serde_json::Value::Null => {
                continue;
            }
        };

        // Check if key begins with "get"
        if key.starts_with("get") {
            // Check if key already exists in hashmap
            let hashmap = HASHMAP.lock().unwrap();
            if hashmap.contains_key(key) {
                // Already has key, don't overwrite
                continue;
            }
        }

        put_full(key, db_value);
    }
}

#[tauri::command]
fn put_value(key: &str, value: Value) -> () {
    HASHMAP.lock().unwrap().insert(key.to_string(), value);
}

#[tauri::command]
fn start_bt_server(port: String, state: State<BTServer>) -> () {
    let server = state.0.lock().unwrap();
    server.start_server(port);
}

#[tauri::command]
fn stop_bt_server(state: State<BTServer>) -> () {
    let server = state.0.lock().unwrap();
    server.stop_server();
}

pub static WINDOW: OnceLock<Window> = OnceLock::new();
fn main() {
    // Connect to bluetooth classic device using windows::bluetooth::* crate
    tauri::Builder::default()
        .setup(move |app| {
            // Get the window to send data to the front end
            let window: Window<Wry> = app.get_window("main").unwrap();
            _ = WINDOW.set(window).expect("Failed to set window");    
        
            Ok(())
        })
        .manage(BTServer(Mutex::new(server::Server::new())))
        .invoke_handler(tauri::generate_handler![put_value, start_bt_server, stop_bt_server])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
