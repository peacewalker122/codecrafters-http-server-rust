use anyhow::Result;
use itertools::Itertools;
use request::{HTTPMethod, HTTPRequest};
use std::{
    env,
    io::Write,
    net::{TcpListener, TcpStream},
    sync::Arc,
    thread,
};

mod handler;
mod request;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:4221").unwrap();

    println!("[INFO] Starting Server AT :4221");

    let mut paramdir: Option<String> = None;

    if env::args().len() > 1 && env::args().contains(&String::from("--directory")) {
        paramdir = Some(
            env::args()
                .collect::<Vec<String>>()
                .get(2)
                .expect("can't find the argument value.")
                .clone(),
        )
    }

    let arcparamdir = Arc::new(paramdir);

    for stream in listener.incoming() {
        let dir = Arc::clone(&arcparamdir);
        let _thread = thread::spawn(|| match stream {
            Ok(mut _stream) => {
                let _ = handle_connection(_stream, dir);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        });
    }
}

fn handle_connection(mut stream: TcpStream, dir: Arc<Option<String>>) -> Result<()> {
    let mut req = HTTPRequest::from(&stream);

    let path: &str = &req.path;

    req.folder = match dir.as_ref().to_owned() {
        Some(val) => Some(val),
        _ => None,
    };

    match path {
        a if a.starts_with("/echo") => handler::echo_handler(&mut stream, &req),
        a if a.starts_with("/user-agent") => handler::user_agent_handler(&mut stream, &req),
        a if a.starts_with("/files") => match &req.method {
            HTTPMethod::GET => handler::get_file(&mut stream, &req),
            HTTPMethod::POST => handler::download_file(&mut stream, &req),
            _ => handler::handle_error(stream, None),
        },
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
