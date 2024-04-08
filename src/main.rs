use std::fs;
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    let contents = fs::read_to_string("index.html").unwrap();

    stream.read(&mut buffer).unwrap();

    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    // HTTP_Version Status_Code Reason_Phrase
    // headers CRLF
    // message_body
    //
    // ex: HTTP/1.1 200 OK\r\n\r\n

    // HTTP response
    let response = "HTTP/1.1 200 OK\r\n\r\n";
    let response = format!("{}{}", response, contents);

    stream.write(response.as_bytes()).unwrap();

    stream.flush().unwrap();
}