use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::net::TcpStream;

pub mod lib {
    pub mod datastructures {
        pub mod btree;
    }
}

fn main() {
    let svr = Server::new();
    svr.setup();
    svr.start();
}

pub struct Server {
    pub port: u16,
}

impl Server {
    pub fn new() -> Self {
        Server { port: 7878 } // Default port
    }

    pub fn setup(&self) {
        println!("Setting up server...");

        let mut base_dir = dirs::data_local_dir().expect("Could not find local app data directory");
        base_dir = base_dir.join("ruql").join("testdb");
        let table_path = base_dir.join("testtable.db");

        if let Err(e) = fs::create_dir_all(&table_path) {
            eprintln!("Failed to create directory: {}", e);
        }

        if !table_path.exists() {
            match fs::File::create(&table_path) {
                Ok(_) => println!("File created: {:?}", table_path),
                Err(e) => eprintln!("Failed to create file: {}", e),
            }
        } else {
            println!("File already exists: {:?}", table_path);
        }
    }

    pub fn start(&self) {
        let port = 7878; // Default port
        println!("Starting server on port {}", port);

        let listener = TcpListener::bind("127.0.0.1:7878").expect("Could not bind");
        println!("Server listening on 127.0.0.1:7878");
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let mut buffer = [0; 512];
                    match stream.read(&mut buffer) {
                        Ok(size) => {
                            let msg = String::from_utf8_lossy(&buffer[..size]);

                            println!("Received: {}", msg);

                            if msg.trim() == "select" {
                                println!("Select command received");
                                let _ = stream.write_all(b"Ok\n");
                            } else {
                                println!("Unknown command");
                                let _ = stream.write_all(b"Error\n");
                            }
                        }
                        Err(e) => println!("Failed to read from client: {}", e),
                    }
                }
                Err(e) => println!("Connection failed: {}", e),
            }
        }
    }
}

pub fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    match stream.read(&mut buffer) {
        Ok(size) => {
            let msg = String::from_utf8_lossy(&buffer[..size]);

            println!("Received: {}", msg);

            if msg.trim() == "select" {
                println!("Select command received");
                let _ = stream.write_all(b"Ok\n");
            } else {
                println!("Unknown command");
                let _ = stream.write_all(b"Error\n");
            }
        }
        Err(e) => println!("Failed to read from client: {}", e),
    }
}
