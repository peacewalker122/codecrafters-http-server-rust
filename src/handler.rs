use anyhow::{Ok, Result};
use std::{
    collections::BTreeMap,
    fs::{self, File},
    io::Read,
    path::Path,
};

use crate::response::*;
use crate::{request::HTTPRequest, response};

pub fn echo_handler(req: &mut HTTPRequest) -> Result<HTTPResponse<String>> {
    let pathwithdata: Vec<&str> = req.path.splitn(3, "/").filter(|x| x.len() > 0).collect();

    let mut resp = response::HTTPResponse {
        status: ResponseStatus::Ok,
        headers: BTreeMap::new(),
        body: String::new(),
    };
    resp.add_headers("Content-Type", "text/plain");
    resp.add_body(pathwithdata[1].to_string());

    Ok(resp)
}

pub fn user_agent_handler(req: &mut HTTPRequest) -> Result<HTTPResponse<String>> {
    let mut resp = response::HTTPResponse {
        status: response::ResponseStatus::Ok,
        headers: BTreeMap::new(),
        body: String::new(),
    };
    resp.add_headers("Content-Type", "text/plain");
    resp.add_body(req.header["User-Agent"].clone());

    Ok(resp)
}

pub fn get_file<T>(req: &mut HTTPRequest) -> Result<HTTPResponse<String>> {
    let pathwithdata: Vec<&str> = req.path.splitn(3, "/").filter(|x| x.len() > 0).collect();

    if let Some(folderpath) = &req.folder {
        let filepath = Path::new(folderpath).join(pathwithdata[1]);

        match filepath.exists() {
            true => {
                let mut file = File::open(filepath).unwrap();
                let mut buf = vec![];
                Read::read_to_end(&mut file, &mut buf).unwrap();

                let mut resp = response::HTTPResponse {
                    status: ResponseStatus::Ok,
                    headers: BTreeMap::new(),
                    body: String::new(),
                };

                resp.add_headers("Content-Type", "application/octet-stream");
                resp.add_headers("Content-length", &buf.len().to_string());
                // resp.add_body(String::from_utf8(buf).unwrap());
                resp.add_body(String::from_utf8(buf).unwrap());
                return Ok(resp);
            }
            _ => {
                let errresponse = String::from("can't find the requested file");
                let resp = response::HTTPResponse {
                    status: ResponseStatus::NotFound,
                    headers: {
                        let mut header = BTreeMap::new();
                        header.insert("Content-Type".to_string(), "text/plain".to_string());

                        header
                    },
                    body: errresponse,
                };

                return Ok(resp);
            }
        }
    }

    Ok(HTTPResponse {
        status: ResponseStatus::BadRequest,
        headers: BTreeMap::new(),
        body: String::from("path doesn't initialized."),
    })
}

pub fn download_file(req: &mut HTTPRequest) -> Result<HTTPResponse<String>> {
    let pathwithdata: Vec<&str> = req.path.splitn(3, "/").filter(|x| x.len() > 0).collect();

    if let Some(folderpath) = &req.folder {
        let filepath = Path::new(folderpath).join(pathwithdata[1]);

        match fs::write(&filepath, req.body.as_bytes()) {
            Err(why) => eprintln!("couldn't create {}: {}", filepath.display(), why),
            _ => {}
        }

        let resp = response::HTTPResponse {
            status: ResponseStatus::Created,
            headers: BTreeMap::new(),
            body: String::new(),
        };

        return Ok(resp);
    }

    Ok(HTTPResponse {
        status: ResponseStatus::BadRequest,
        headers: BTreeMap::new(),
        body: String::from("path doesn't initialized."),
    })
}

pub fn handle_notfound(_req: &mut HTTPRequest) -> Result<HTTPResponse<String>> {
    Ok(HTTPResponse {
        status: ResponseStatus::NotFound,
        headers: BTreeMap::new(),
        body: String::new(),
    })
}

pub fn success(_req: &mut HTTPRequest) -> Result<HTTPResponse<String>> {
    Ok(HTTPResponse {
        status: ResponseStatus::Ok,
        headers: BTreeMap::new(),
        body: String::new(),
    })
}
