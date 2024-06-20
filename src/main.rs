// Uncomment this block to pass the first stage
// use std::net::TcpListener;

use std::{
    borrow::Borrow,
    error::Error,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use itertools::Itertools;
use nom::Err;

#[derive(Debug)]
struct RequestLine {
    method: Option<String>,
    target: Option<String>,
    version: Option<String>,
}

#[derive(Debug)]
struct Header {
    host: Option<String>,
    user_agent: Option<String>,
    accept: Option<String>,
}

#[derive(Debug)]
struct Body {}

#[derive(Debug)]
struct HttpRequest {
    request_line: Option<RequestLine>,
    headers: Option<Header>,
    body: Option<Body>,
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    // println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage

    let mut request_line = Vec::<String>::new();
    let mut headers = Vec::<String>::new();
    let mut request_body = Vec::<String>::new();

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
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    let get = b"GET / HTTP/1.1\r\n";
    let response = if buffer.starts_with(get) {
        "HTTP/1.1 200 OK\r\n\r\n"
    } else {
        "HTTP/1.1 404 Not Found\r\n\r\n"
    };
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn parse_http_string(http_string: &str) -> Result<HttpRequest, String> {
    let vector_str: Vec<&str> = http_string.split("\r\n").collect();

    let request_line;

    // let body;

    if let Some(request_line_str) = vector_str.get(0) {
        let parts: Vec<&str> = request_line_str.split_whitespace().collect();
        request_line = RequestLine {
            method: Some(parts.get(0).unwrap_or(&"").to_string()),
            target: Some(parts.get(1).unwrap_or(&"").to_string()),
            version: Some(parts.get(2).unwrap_or(&"").to_string()),
        }
    } else {
        return Err("Do not have request_line!".to_string());
    }

    let mut headers = Header {
        host: None,
        user_agent: None,
        accept: None,
    };

    for header_str in vector_str.iter().skip(1) {
        // Skip the request line
        if header_str.starts_with("Host: ") {
            headers.host = Some(header_str.replace("Host: ", ""));
        } else if header_str.starts_with("User-Agent: ") {
            headers.user_agent = Some(header_str.replace("User-Agent: ", ""));
        } else if header_str.starts_with("Accept: ") {
            headers.accept = Some(header_str.replace("Accept: ", ""));
        }
    }

    if headers.host.is_none() || headers.user_agent.is_none() || headers.accept.is_none() {
        return Err("Missing required headers".to_string());
    }

    return Ok(HttpRequest {
        request_line: Some(request_line),
        headers: Some(headers),
        body: None,
    });
}
