// Uncomment this block to pass the first stage
// use std::net::TcpListener;

use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

#[derive(Debug)]
struct RequestLine {
    method: String,
    uri: String,
    version: String,
}

#[derive(Debug)]
struct Header {
    key: String,
    value: String,
}

#[derive(Debug)]
struct Body {}

#[derive(Debug)]
struct HttpRequest {
    request_line: RequestLine,
    headers: Vec<Header>,
    body: Option<String>,
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    // println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => match handle_connection(&mut _stream) {
                Ok(request) => {
                    if request.request_line.uri.contains("echo/") {
                        let path = request
                            .request_line.uri.splitn(2, "echo/").collect::<Vec<&str>>()[1];

                        let content_length = path.len();
                        
                        let response_str = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {content_length}\r\n\r\n{path}");

                        //unwrap here to hire some warning on some errors might not be handled correctly.
                        _stream.write(response_str.as_bytes()).unwrap();
                        _stream.flush().unwrap();
                    }
                }
                Err(e) => {
                    println!("{e}");
                }
            },
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(stream: &mut TcpStream) -> Result<HttpRequest, String> {
    let mut buffer = Vec::new();

    match stream.read_to_end(&mut buffer) {
        Ok(_) => {
            let http_string = match String::from_utf8(buffer) {
                Ok(s) => s,
                Err(e) => {
                    return Err(e.to_string());
                }
            };

            return parse_http_string(&http_string);
        }
        Err(e) => {
            return Err(format!("Failed to read from stream: {e}"));
        }
    }

    // let mut buffer = [0; 1024];
    // stream.read(&mut buffer).unwrap();

    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    // let get = b"GET / HTTP/1.1\r\n";
    // let response = if buffer.starts_with(get) {
    //     "HTTP/1.1 200 OK\r\n\r\n"
    // } else {
    //     "HTTP/1.1 404 Not Found\r\n\r\n"
    // };
    // stream.write(response.as_bytes()).unwrap();
    // stream.flush().unwrap();
}

fn parse_http_string(http_string: &str) -> Result<HttpRequest, String> {
    let mut lines = http_string.split("\r\n").peekable();
    let request_line = lines.next().ok_or("Request line missing")?;

    let parts: Vec<&str> = request_line.split_whitespace().collect();
    if parts.len() != 3 {
        return Err("Invalid request line".to_string());
    }

    let request_line = RequestLine {
        method: parts[0].to_string(),
        uri: parts[1].to_string(),
        version: parts[2].to_string(),
    };

    let mut headers = Vec::new();
    while let Some(&line) = lines.peek() {
        if line.is_empty() {
            lines.next();
            break;
        }
        let header_parts: Vec<&str> = line.splitn(2, ':').collect();
        if header_parts.len() != 2 {
            return Err("Invalid Header".to_string());
        }
        headers.push(Header {
            key: header_parts[0].to_string(),
            value: header_parts[1].to_string(),
        });
        lines.next();
    }

    let body = if lines.peek().is_some() {
        Some(lines.collect::<Vec<&str>>().join("\r\n"))
    } else {
        None
    };

    Ok(HttpRequest {
        request_line,
        headers,
        body,
    })
}
