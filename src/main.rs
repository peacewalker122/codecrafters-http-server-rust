use anyhow::Result;
use request::HTTPRequest;
use std::{
    io::Write,
    net::{TcpListener, TcpStream},
};

mod handler;
mod request;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    // println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                let _ = handle_connection(_stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) -> Result<()> {
    let req = HTTPRequest::from(&stream);

    let path: &str = &req.path;

    match path {
        a if a.starts_with("/echo") => handler::echo_handler(&mut stream, &req),
        a if a.starts_with("/user-agent") => handler::user_agent_handler(&mut stream, &req),
        "/" => {
            stream
                .write("HTTP/1.1 200 OK\r\n\r\n".as_bytes())
                .expect("Unable to write to stream");
        }
        _ => {
            stream
                .write("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes())
                .expect("Unable to write to stream");
        }
    }

    Ok(())
}
