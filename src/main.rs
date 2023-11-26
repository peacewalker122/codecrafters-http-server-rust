use anyhow::Result;
use log;
use request::{HTTPMethod, HTTPRequest};
use response::HTTPResponse;
use std::{
    env,
    io::Write,
    net::{TcpListener, TcpStream},
    sync::Arc,
    thread,
};

mod config;
mod handler;
mod request;
mod response;

fn main() {
    let cfg = config::parse(env::args()).expect("failed to parse config.");
    let listener = TcpListener::bind(&cfg.url).unwrap();

    let mut paramdir: Option<String> = None;
    paramdir = cfg.directory;

    let arcparamdir = Arc::new(paramdir);

    println!("[INFO] server run at: {}", &cfg.url);
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

fn handle_connection(stream: TcpStream, dir: Arc<Option<String>>) -> Result<()> {
    let mut req = HTTPRequest::from(stream);

    let path: &str = &req.path;

    req.folder = match dir.as_ref().to_owned() {
        Some(val) => Some(val),
        _ => None,
    };

    match path {
        a if a.starts_with("/echo") => path_handler(&mut req, handler::echo_handler),
        a if a.starts_with("/user-agent") => path_handler(&mut req, handler::user_agent_handler),
        a if a.starts_with("/files") => match &mut req.method {
            HTTPMethod::GET => path_handler(&mut req, handler::get_file::<Vec<u8>>),
            HTTPMethod::POST => path_handler(&mut req, handler::download_file),
            _ => path_handler(&mut req, handler::handle_notfound),
        },
        "/" => path_handler(&mut req, handler::success),
        _ => path_handler(&mut req, handler::handle_notfound),
    }

    Ok(())
}

fn path_handler<T>(req: &mut HTTPRequest, mut f: T)
where
    T: FnMut(&mut HTTPRequest) -> Result<HTTPResponse<String>>,
{
    log::trace!("{:?}", &req);
    match f(req) {
        Ok(x) => req
            .conn
            .write_all(x.to_string().as_bytes())
            .expect("failed to write to stream"),
        Err(_) => {}
    }
}
