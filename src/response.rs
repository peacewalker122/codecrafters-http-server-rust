use std::{collections::BTreeMap, fmt};

#[derive(Debug)]
pub struct HTTPResponse<T> {
    pub status: ResponseStatus,
    pub headers: BTreeMap<String, String>,
    pub body: T,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum ResponseStatus {
    Ok,
    Created,
    NotFound,
    BadRequest,
    InternalServerError,
    Unknown,
}

impl fmt::Display for ResponseStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (code, message) = match self {
            ResponseStatus::Ok => (200, "OK"),
            ResponseStatus::Created => (201, "Created"),
            ResponseStatus::BadRequest => (400, "Bad Request"),
            ResponseStatus::NotFound => (404, "Not Found"),
            ResponseStatus::InternalServerError => (500, "Internal Server Error"),
            ResponseStatus::Unknown => (0, "Unknown"),
        };

        write!(f, "HTTP/1.1 {} {}\r\n", code, message)
    }
}

impl fmt::Display for HTTPResponse<String> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut headers = self
            .headers
            .iter()
            .map(|x| format!("{}:{}\r\n", x.0, x.1))
            .collect::<Vec<String>>()
            .join("");
        headers.push_str("\r\n");

        write!(f, "{}{}{}", self.status.to_string(), headers, self.body)
    }
}

impl fmt::Display for HTTPResponse<Vec<u8>> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut headers = self
            .headers
            .iter()
            .map(|x| format!("{}:{}\r\n", x.0, x.1))
            .collect::<Vec<String>>()
            .join("");
        headers.push_str("\r\n");

        dbg!(&headers);

        let body = &self.body;

        write!(
            f,
            "{}{}{}",
            self.status.to_string(),
            headers,
            String::from_utf8(body.clone()).unwrap()
        )
    }
}

// impl Into<String> for HTTPResponse<String> {
//     fn into(self) -> String {
//         let mut headers = self
//             .headers
//             .iter()
//             .map(|x| format!("{}:{}\r\n", x.0, x.1))
//             .collect::<Vec<String>>()
//             .join("");
//         headers.push_str("\r\n");
//         format!("{}{}{:?}", self.status.to_string(), headers, self.body)
//     }
// }

impl<T> HTTPResponse<T> {
    pub fn add_headers<X: ToString>(&mut self, key: X, val: X) {
        self.headers.insert(key.to_string(), val.to_string());
    }
}

impl HTTPResponse<String> {
    pub fn add_body(&mut self, val: String) {
        if val.len() > 0 {
            self.add_headers("Content-Length", &val.len().to_string())
        }

        self.body = val;
    }
}

impl HTTPResponse<Vec<u8>> {
    #[allow(dead_code)]
    pub fn add_body(&mut self, val: Vec<u8>) {
        if val.len() > 0 {
            self.add_headers("Content-Length", &val.len().to_string())
        }

        self.body = val;
    }
}

#[cfg(test)]
mod test {
    use crate::response::*;

    #[tokio::test]
    async fn test_new_response_string() {
        let mut resp = HTTPResponse {
            status: ResponseStatus::Ok,
            headers: BTreeMap::new(),
            body: String::new(),
        };

        resp.add_headers("Content-Type", "application/text");

        resp.add_body(String::from("value"));
        assert!(resp.headers.len() == 2);
        println!("{}", resp.to_string());
    }

    #[tokio::test]
    async fn test_new_response_u8() {
        let mut resp = HTTPResponse {
            status: ResponseStatus::Ok,
            headers: BTreeMap::new(),
            body: vec![],
        };

        resp.add_headers("Content-Type", "application/octet-stream");

        resp.add_body(String::from("value").into_bytes());
        assert!(resp.headers.len() == 2);
        println!("{}", resp.to_string());

        // let borresp = resp;

        // borresp.to_string();
    }
}
