// Uncomment this block to pass the first stage
use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    // println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                handle_connection(_stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let chunks: Vec<&str> = request_line.split_whitespace().collect();

    let path = chunks[1]; //ex: /echo/abc

    let pathwithdata: Vec<&str> = path.split("/").filter(|x| x.len() > 0).collect();

    // dbg!(path);
    // dbg!(pathwithdata);

    let containecho = path.contains("echo");

    match containecho {
        true => {
            if let Some(x) = pathwithdata.get(0) {
                if *x == "echo" {
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
            }
        }
        false => {
            match path {
                "/" => stream
                    .write("HTTP/1.1 200 OK\r\n\r\n".as_bytes())
                    .expect("Unable to write to stream"),
                _ => stream
                    .write("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes())
                    .expect("Unable to write to stream"),
            };
        }
    }
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
    use crate::handle_content;

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
