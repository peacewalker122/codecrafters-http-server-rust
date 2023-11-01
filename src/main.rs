use anyhow::Result;
use request::HTTPRequest;
use std::{
    env,
    io::Write,
    net::{TcpListener, TcpStream},
    thread,
};

mod handler;
mod request;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    // println!("Logs from your program will appear here!");
    let listener = TcpListener::bind("0.0.0.0:4221").unwrap();

    println!("[INFO] Starting Server AT :4221");

    // let _unintended = check_flag("--directory").unwrap();

    for stream in listener.incoming() {
        let _thread = thread::spawn(|| match stream {
            Ok(mut _stream) => {
                let _ = handle_connection(_stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        });
    }
}

fn handle_connection(mut stream: TcpStream) -> Result<()> {
    let req = HTTPRequest::from(&stream);

    let path: &str = &req.path;

    match path {
        a if a.starts_with("/echo") => handler::echo_handler(&mut stream, &req),
        a if a.starts_with("/user-agent") => handler::user_agent_handler(&mut stream, &req),
        a if a.starts_with("/files") => handler::get_file(&mut stream, &req),
        "/" => {
            stream
                .write("HTTP/1.1 200 OK\r\n\r\n".as_bytes())
                .expect("Unable to write to stream");
        }
        _ => handler::handle_error(stream, None),
    }

    Ok(())
}

#[allow(dead_code)]
fn check_flag(flag: &str) -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.iter().any(|arg| arg == flag) {
        Ok(())
    } else {
        Err(format!(
            "Flag '{}' is not present in the command-line arguments.",
            flag
        ))
    }
}
