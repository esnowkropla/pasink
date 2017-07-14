use std::net::{TcpStream, TcpListener};
use std::io::{Read, Write};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}

pub fn server(in_str: &str) {
    let listener = match TcpListener::bind(in_str) {
        Ok(l) => l,
        Err(e) => panic!("got error {}", e),
    };

    let (mut first_stream, first_addr) = listener.accept().unwrap();
    println!("First connection: {}", first_addr);
    let (mut second_stream, second_addr) = listener.accept().unwrap();
    println!("Second connection: {}", first_addr);

    match write!(first_stream, "you are first") {
        Ok(_) => (),
        Err(e) => println!("got error {}", e),
    };

    match write!(second_stream, "you are second") {
        Ok(_) => (),
        Err(e) => println!("got error {}", e),
    };
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
