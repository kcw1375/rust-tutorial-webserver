use std::net::TcpListener;

fn main() {
    // bind listener to localhost (127.0.0.1) port 7878
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // iterate over connection attempts
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
    }
}
