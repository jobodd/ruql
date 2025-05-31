use std::env;
use std::io::Write;
use std::net::TcpStream;

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: client <message>");
        std::process::exit(1);
    }
    let message = &args[1];
    match TcpStream::connect("127.0.0.1:7878") {
        Ok(mut stream) => {
            stream.write_all(message.as_bytes()).expect("Failed to send message");
            println!("Sent: {}", message);
        }
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
        }
    }
}
