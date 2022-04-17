use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;
use std::thread;
use std::time::Duration;



fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for _stream in listener.incoming() {
        let _stream = _stream.unwrap();

        handle_connection(_stream);
    }
} 

fn handle_connection(mut stream: TcpStream) {

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "./src/test.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "./src/test.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

}