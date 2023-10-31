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
}

impl From<&TcpStream> for HTTPRequest {
    fn from(mut stream: &TcpStream) -> Self {
        // it's only print the debug when the request were closed or canceled. why?
        // possible because of it can't read the EOF from the request.
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

        dbg!(&req);
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

        Self {
            path: path.to_string(),
            method: chunks[0].to_string(),
            body: String::new(), //TODO: IMPLEMENT
            header: headers,
        }
    }
}
