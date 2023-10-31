use std::{io::Write, net::TcpStream};

use crate::request::HTTPRequest;

pub fn echo_handler(stream: &mut TcpStream, req: &HTTPRequest) {
    let pathwithdata: Vec<&str> = req.path.splitn(3, "/").filter(|x| x.len() > 0).collect();

    let contentlength = format!("Content-length: {}\r\n", pathwithdata[1].len());
    let msg = vec![
        "HTTP/1.1 200 OK\r\n",
        "Content-Type: text/plain\r\n",
        &contentlength,
        "\r\n",
        pathwithdata[1],
    ];

    stream
        .write_all(handle_content(&msg).as_bytes())
        .expect("Unable to write to stream");
}

pub fn user_agent_handler(stream: &mut TcpStream, req: &HTTPRequest) {
    let contentlength = format!("Content-length: {}\r\n", req.header["User-Agent"].len());
    let msg = vec![
        "HTTP/1.1 200 OK\r\n",
        "Content-Type: text/plain\r\n",
        &contentlength,
        "\r\n",
        &req.header["User-Agent"],
    ];

    stream
        .write_all(handle_content(&msg).as_bytes())
        .expect("Unable to write to stream");
}

fn handle_content(val: &Vec<&str>) -> String {
    let mut result = String::new();

    for i in 0..val.len() {
        result.push_str(val[i]);
    }

    result
}

#[cfg(test)]
mod test {
    use crate::handler::handle_content;

    #[test]
    fn test_handle_content() {
        let contentlength = format!("Content-length: {}\r\n\r\n", 10);
        let res = handle_content(&vec![
            "HTTP/1.1 200 OK\r\n\r\n",
            "Content-Type: text/plain\r\n\r\n",
            &contentlength,
            "this-is-the-message\r\n\r\n",
        ]);

        dbg!(&res);

        assert!(res.len() > 0)
    }
}
