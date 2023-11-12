use serial2::SerialPort;
use std::sync::{Arc, Mutex};
use serde_json::{Value, Map};
use crate::db;
use crate::put_full_json;

fn json_to_hashmap(json_str: &str) -> Option<Map<String, Value>> {
    // Parse the JSON string into a serde_json::Value
    let json_value: Value = serde_json::from_str(json_str).ok()?;

    // Check if the JSON value is an object (HashMap)
    let json_object = json_value.as_object()?;

    // Convert the JSON object to a HashMap
    let hashmap: Map<String, Value> = json_object.clone();

    Some(hashmap)
}

#[derive(Clone)]
pub struct Server {
    server_running: Arc<Mutex<bool>>,
}

impl Server {
    pub fn new() -> Server {
        Server {
            server_running: Arc::new(Mutex::new(false)),
        }
    }

    pub fn stop_server(&self) -> () {
        *self.server_running.lock().unwrap() = false;
        println!("Stopping the server");
    }

    pub fn start_server(&self, port: String) -> () {
        if *self.server_running.lock().unwrap() {
            return;
        }
        *self.server_running.lock().unwrap() = true;
        self.start_server_dont_call(port);
    }

    fn start_server_dont_call(&self, com_port: String) {
        let server_running = Arc::clone(&self.server_running);
        let serial_port = SerialPort::open(com_port.clone(), 115200);
        match serial_port {
            Ok(mut port) => {
                println!("Successfully opened \"{}\"", com_port);
                port.set_read_timeout(std::time::Duration::from_millis(100)).unwrap();
                port.set_write_timeout(std::time::Duration::from_millis(100)).unwrap();
                let mut is_reading: bool = false;
                let mut successfull_read: bool = true;
                // Spawn a thread to read from the port
                tauri::async_runtime::spawn(async move {
                    let mut buffer = [0; 256];
                    let mut sendmap: std::collections::HashMap<String, String> = std::collections::HashMap::new();
                    loop {
                        // Check if server should be running
                        if !*server_running.lock().unwrap() {
                            break;
                        }

                        if is_reading == false {
                                // Reset sendmap
                            sendmap.clear();
                            // Get data from database
                            let db = db::HASHMAP.lock().unwrap();
                            // Loop over key values of hashmap
                            for (key, value) in db.iter() {
                                // Check if key begins with "get"
                                if key.starts_with("get") {
                                    sendmap.insert(key.clone(), value.to_string());
                                }

                            }

                            if let Ok(send_str) = serde_json::to_string(&sendmap) {
                                // println!("Send JSON: {}", send_str);
                                let final_str = format!("{}|", send_str);
                                if let Err(err) = port.write(final_str.as_bytes()) {
                                    eprintln!("Error writing to port: {}", err);
                                };
                            } else {
                                println!("Failed to convert sendmap to JSON");
                            }
                            is_reading = true;
                            continue;
                        } else {
                            is_reading = false;
                        }

                        while successfull_read {
                            // Attempt to read from the port
                            let read = match port.read(&mut buffer) {
                                Ok(read) => { 
                                    successfull_read = true;
                                    read
                                },
                                Err(_err) => {
                                    //eprintln!("Error reading from port: {}", err);
                                    successfull_read = false;
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
                            println!("Read from port: {}", data_str);

                            if let Some(hashmap) = json_to_hashmap(data_str) {
                                println!("JSON: {:?}", hashmap);
                                // Send data to frontend + database
                                put_full_json(hashmap);

                            } else {
                                println!("Not JSON");
                            }
                        }
                        successfull_read = true;
                    }
                });
            }
            Err(e) => {
                eprintln!("Failed to open \"{}\". Error: {}", com_port, e);
            }    
        }
    }
}