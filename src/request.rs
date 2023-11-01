use std::{
    collections::BTreeMap,
    io::{self, BufRead, BufReader},
    net::TcpStream,
};

#[derive(Debug)]
pub struct HTTPRequest {
    pub path: String,
    pub method: String,
    pub body: String,
    pub header: BTreeMap<String, String>,

    pub folder: Option<String>,
}

impl From<&TcpStream> for HTTPRequest {
    fn from(mut stream: &TcpStream) -> Self {
        let mut reader = BufReader::new(&mut stream);
        let received = reader.fill_buf().unwrap().to_vec();

        reader.consume(received.len());

        let req = String::from_utf8(received)
            .map(|msg| msg)
            .map_err(|_| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Couldn't parse received string as utf8",
                )
            })
            .unwrap();
        let req: Vec<&str> = req.split("\r\n").filter(|x| x.len() > 0).collect();

        let chunks: Vec<&str> = req[0].split_whitespace().collect();
        let path = chunks[1]; //ex: /echo/abc

        let headers: BTreeMap<_, _> = req
            .iter()
            .filter(|x| x.contains(":"))
            .map(|x| {
                let parts: Vec<&str> = x.split(":").collect();
                (parts[0].trim().to_string(), parts[1].trim().to_string())
            })
            .collect();

        let dir = match parse_folder(std::env::args()) {
            Ok(x) => Some(x),
            Err(_x) => None,
        };

        Self {
            path: path.to_string(),
            method: chunks[0].to_string(),
            body: String::new(), //TODO: IMPLEMENT
            header: headers,
            folder: dir,
        }
    }
}

fn parse_folder(arg: std::env::Args) -> Result<String, String> {
    let arg: Vec<String> = arg.collect();

    if let Some(x) = arg.iter().position(|x| x == "--directory") {
        if x + 1 < arg.len() {
            return Ok(arg[x + 1].clone());
        }
    }

    Err("no --directory value provided".to_string())
}
