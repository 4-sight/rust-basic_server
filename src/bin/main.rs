use std::fs;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};

use basic_server::ThreadPool;

const PORT: &str = "127.0.0.1:7878";

fn main() {
    let listener = TcpListener::bind(PORT).unwrap();
    println!("\nConnection Established\nListening on port: {}\n", PORT);

    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        })
    }

    println!("Shutting down.")
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = String::new();

    BufReader::new(&stream).read_line(&mut buffer).unwrap();

    let get = "GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.eq(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
