use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    match stream.read(&mut buffer) {
        Ok(size) => {
            let msg = String::from_utf8_lossy(&buffer[..size]);
            println!("Received: {}", msg);
            let _ = stream.write_all(b"Message received\n");
        }
        Err(e) => println!("Failed to read from client: {}", e),
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Could not bind");
    println!("Server listening on 127.0.0.1:7878");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => println!("Connection failed: {}", e),
        }
    }
}
