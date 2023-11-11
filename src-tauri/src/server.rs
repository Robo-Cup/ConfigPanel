use serial2::SerialPort;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;

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
        tauri::async_runtime::spawn(async move {
            //let (tx, rx) = mpsc::channel();
            let serial_port = SerialPort::open(com_port.clone(), 115200);

            match serial_port {
                Ok(mut port) => {
                    println!("Successfully opened \"{}\"", com_port);
                    port.set_read_timeout(std::time::Duration::from_millis(10)).unwrap();
                    port.set_write_timeout(std::time::Duration::from_millis(10)).unwrap();
                    let mut buffer = [0; 256];
                    loop {
                        // Check if server should be running
                        if !*server_running.lock().unwrap() {
                            break;
                        }
                        // Attempt to read from the port
                        let read = match port.read(&mut buffer) {
                            Ok(read) => read,
                            Err(_err) => {
                                //eprintln!("Error reading from port: {}", err);
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
                        // Add '|' character to end of data_str
                        let data_str = format!("{}|", data_str);
                        if let Err(err) = port.write(data_str.as_bytes()) {
                            eprintln!("Error writing to port: {}", err);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to open \"{}\". Error: {}", com_port, e);
                }
            }
        });
    }
}