use std::net::{TcpStream, TcpListener};
use std::io::{Read, Write};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}

fn handle_client(mut stream: TcpStream) {
    let addr = stream.peer_addr().unwrap();
    println!("addr: {}", addr);
    write!(&mut stream, "addr: {}", addr);
}

pub fn server(in_str: &str) {
    let listener = match TcpListener::bind(in_str) {
        Ok(l) => l,
        Err(e) => panic!("got error {}", e),
    };

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_client(stream),
            Err(e) => println!("Connection failed: {}", e),
        }
    }
}

pub fn client(in_str: &str) {
    let mut stream = match TcpStream::connect(in_str) {
        Ok(conn) => conn,
        Err(e) => panic!("Got error {}", e),
    };

    let mut buf = String::new();
    stream.read_to_string(&mut buf);
    println!("received from server: {}", buf);
}
