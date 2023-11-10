// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
//use serialport::{available_ports, SerialPortType};
//use serialport::SerialPort;
use std::io::Read;
use serial2::SerialPort;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use std::io::{self};

fn main() {
    // Connect to bluetooth classic device using windows::bluetooth::* crate
    tauri::async_runtime::spawn(async move {
        let port = SerialPort::open("COM4", 115200);
        let outport = SerialPort::open("COM5", 115200);
        match outport {
            Ok(outport) => {
                match port {
                    Ok(mut port) => {
                        println!("Connected to COM5");
                        port.set_read_timeout(std::time::Duration::from_millis(10)).unwrap();
                        port.set_write_timeout(std::time::Duration::from_millis(10)).unwrap();
                        let mut buffer = [0; 256];
                        loop {
                            let read = match port.read(&mut buffer) {
                                Ok(read) => read,
                                Err(err) => {
                                    eprintln!("Error reading from port: {}", err);
                                    continue;
                                }
                            };
                        
                            // Process the data in the buffer
                            let data_str = match std::str::from_utf8(&buffer[..read]) {
                                Ok(s) => s,
                                Err(_) => {
                                    eprintln!("Error: Invalid UTF-8 data in buffer");
                                    continue;
                                }
                            };
    
                            println!("{}", data_str);
                            // Add '|' character to end of data_str
                            let data_str = format!("{}|", data_str);
                            if let Err(err) = port.write(data_str.as_bytes()) {
                                eprintln!("Error writing to port: {}", err);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to open \"{}\". Error: {}", "COM5", e);
                    }
                }
            },
            Err(_) => todo!()
        }
        
    });

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
