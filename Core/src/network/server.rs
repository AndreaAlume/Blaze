use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

pub struct Server {
    pub host: String,
    pub port: String,
    pub is_open: bool,
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer).unwrap();

    let msg = &buffer[..n];
    println!("Client says: {:?}", msg);

    stream.write_all(b"OK\n").unwrap();
}


pub fn start_server() -> Server {
    let host = "localhost".to_string();
    let port = "9000".to_string();

    let listener = TcpListener::bind(format!("{host}:{port}"))
        .expect("Failed to bind!");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream);
            }
            Err(e) => { println!("{}", e) }
        }
    }

    Server { host, port, is_open: true }
}
