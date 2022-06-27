use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use std::{fs, thread};

use simple_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // Serve response on Thread
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
            println!("Response sent!");
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024]; // to store http request
    stream.read(&mut buffer).unwrap();

    // println!("Request: {}!", String::from_utf8_lossy(&buffer[..]));

    // Request : METHOD PATH PROTOCOL
    // GET / HTTP/1.1

    // Response : HTTP-Version Status-code reason CRLF
    // Headers CRLFCRLF
    // Body CRLF
    // HTTP/1.1 200 OK\r\n\r\n

    let index = b"GET / HTTP/1.1\r\n";
    let status = b"GET /status HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(index) {
        ("HTTP/1.1 200 OK", "index.html")
    } else if buffer.starts_with(status) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "status.html")
    } else {
        ("HTTP/1.1 404 Not Found", "404.html")
    };

    let content = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}\r\n",
        status_line,
        content.len(),
        content
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
