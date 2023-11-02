use std::{
    fs::{File, self},
    io::{Read, Write},
    net::TcpStream,
    path::Path,
};

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
    dbg!(&req);
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

pub fn get_file(stream: &mut TcpStream, req: &HTTPRequest) {
    let pathwithdata: Vec<&str> = req.path.splitn(3, "/").filter(|x| x.len() > 0).collect();

    if let Some(folderpath) = &req.folder {
        let filepath = Path::new(folderpath).join(pathwithdata[1]);

        match filepath.exists() {
            true => {
                let mut file = File::open(filepath).unwrap();
                let mut buf = vec![];
                Read::read_to_end(&mut file, &mut buf).unwrap();

                let contentlength = format!("Content-length: {}\r\n", buf.len());

                let msg = vec![
                    "HTTP/1.1 200 OK\r\n",
                    "Content-Type: application/octet-stream\r\n",
                    &contentlength,
                    "\r\n",
                ];

                stream
                    .write_all(handle_content(&msg).as_bytes())
                    .expect("Unable to write to stream");

                stream.write_all(&buf).expect("unable to write to stream");
            }
            _ => {
                stream
                    .write("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes())
                    .expect("Unable to write to stream");
            }
        }
    }
}

pub fn download_file(stream: &mut TcpStream, req: &HTTPRequest) {
    let pathwithdata: Vec<&str> = req.path.splitn(3, "/").filter(|x| x.len() > 0).collect();

    if let Some(folderpath) = &req.folder {
        let filepath = Path::new(folderpath).join(pathwithdata[1]);

        dbg!(&filepath);

        // let mut file = match File::create(&filepath) {
        //     Ok(file) => file,
        //     Err(why) => {
        //         eprintln!("couldn't create {}: {}", filepath.display(), why);
        //         return;
        //     }
        // };

        match fs::write(&filepath,req.body.as_bytes()) {
            Err(why) => eprintln!("couldn't create {}: {}", filepath.display(), why),
            _ => {}
        }

        let msg = vec!["HTTP/1.1 201 Created\r\n\r\n"];

        stream
            .write_all(handle_content(&msg).as_bytes())
            .expect("Unable to write to stream");
        return;
    }

    stream
        .write("HTTP/1.1 500 Internal Server Error\r\n\r\n".as_bytes())
        .expect("Unable to write to stream");

    return;
}

fn handle_content(val: &Vec<&str>) -> String {
    let mut result = String::new();

    for i in 0..val.len() {
        result.push_str(val[i]);
    }

    result
}

pub fn handle_error(mut stream: TcpStream, val: Option<String>) {
    let _val = match val {
        Some(x) => stream
            .write(format!("HTTP/1.1 {} \r\n\r\n", x).as_bytes())
            .expect("Unable to write to stream"),
        None => stream
            .write("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes())
            .expect("Unable to write to stream"),
    };
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
