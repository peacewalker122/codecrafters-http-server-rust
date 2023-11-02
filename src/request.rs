use std::{
    collections::BTreeMap,
    io::{self, BufRead, BufReader},
    net::TcpStream,
};

#[derive(Debug)]
pub enum HTTPMethod {
    GET,
    POST,
    None,
}

impl HTTPMethod {
    fn try_from_string(value: &str) -> Option<Self> {
        match value {
            "GET" => Some(HTTPMethod::GET),
            "POST" => Some(HTTPMethod::POST),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct HTTPRequest {
    pub path: String,
    pub method: HTTPMethod,
    pub body: String,
    pub header: BTreeMap<String, String>,

    pub folder: Option<String>,
}

// need to update especially in how to parse the body.
// to read body we can't relay again in the newline.
// we need to make a custom parser for the http information such like METHOD,path,etc.
impl From<&TcpStream> for HTTPRequest {
    fn from(mut stream: &TcpStream) -> Self {
        let mut reader = BufReader::new(&mut stream);
        let mut received = vec![];

        loop {
            let mut buf = vec![];
            let bytes = reader.read_until(b'\n', &mut buf).unwrap();

            // endline
            if bytes == 0 {
                break;
            }

            let line = String::from_utf8(buf.clone()).unwrap();

            if line == "\r\n" {
                break;
            }

            received.push(buf);
        }

        let mut receipt: Vec<u8> = vec![];

        for i in 0..received.len() {
            receipt.append(&mut received[i])
        }

        let req = String::from_utf8(receipt)
            .map(|msg| msg)
            .map_err(|err| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Couldn't parse received string as utf8: {err}"),
                )
            })
            .unwrap();
        let req: Vec<&str> = req.split("\r\n").filter(|x| x.len() > 0).collect();

        dbg!(&req);

        let chunks: Vec<&str> = req[0].split_whitespace().collect();
        let path = chunks[1]; //ex: /echo/abc

        let headers: BTreeMap<_, _> = req
            .iter()
            .filter(|x| x.contains(":"))
            .map(|x| {
                let parts: Vec<&str> = x.splitn(2, ":").collect();
                (parts[0].trim().to_string(), parts[1].trim().to_string())
            })
            .collect();

        Self {
            path: path.to_string(),
            method: match HTTPMethod::try_from_string(&chunks[0].to_uppercase()) {
                Some(x) => x,
                _ => HTTPMethod::None,
            },
            body: match headers.get("Content-Length") {
                Some(_x) => {
                    let buf = reader.fill_buf().unwrap().to_vec();
                    dbg!(&buf);
                    
                    reader.consume(buf.len());
                    
                    String::from_utf8(buf).unwrap()
                }
                _ => String::new(),
            },
            header: headers,
            folder: None,
        }
    }
}

#[allow(dead_code)]
fn parse_folder(arg: std::env::Args) -> Result<String, String> {
    let arg: Vec<String> = arg.collect();

    if let Some(x) = arg.iter().position(|x| x == "--directory") {
        if x + 1 < arg.len() {
            return Ok(arg[x + 1].clone());
        }
    }

    Err("no --directory value provided".to_string())
}
