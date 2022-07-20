use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    // bind listener to localhost (127.0.0.1) port 7878
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // iterate over connection attempts
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
        
    }
}


fn handle_connection(mut stream: TcpStream) {
    // parameter TcpStream needs to be mutable since it keeps track of data read
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    // for now we only respond if request is GET on root directory
    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}