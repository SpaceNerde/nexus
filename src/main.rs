// basic echo server

use std::net::TcpListener;

fn server() {
    let listener = TcpListener::bind("127.0.0.1:80").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Connected to: {:?}", stream.peer_addr().unwrap());
            },
            Err(e) => {

            }
        }
    }
}

fn client() {

}

fn main() {
    
}
