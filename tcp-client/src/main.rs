use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();

    let data = "Hello, TCP server!".as_bytes();
    stream.write(data).unwrap();

    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let str = str::from_utf8(&buffer).unwrap();
    print!("Response from server: {:?}", str);
}