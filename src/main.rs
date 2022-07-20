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

    let response = "HTTP/1.1 200 OK\r\n\r\n";
    
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}