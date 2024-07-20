use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread::spawn;

fn handle_client(mut stream: TcpStream) {
    // this buffer to read data from the client
    let mut buffer = [0; 1024];
    // reads data from stream and stores in buffer
    stream.read(&mut buffer).expect("failed to read from client!!!");
    // convert data to utf-8 encoded string
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("received request: {}", request);
    let response = "hello, RUST RUST Client!!!".as_bytes();
    stream.write(response).expect("failed to write response");
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("failed to bind address");
    println!("server listening at port no: 8080");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                spawn(move || handle_client(stream));
            }
            Err(e) => {
                eprintln!("failed to establish connection: {}", e);
            }
        }
    }
}
