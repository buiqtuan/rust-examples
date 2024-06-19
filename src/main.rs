// Uncomment this block to pass the first stage
// use std::net::TcpListener;

use std::{borrow::Borrow, io::Write, net::TcpListener};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                match _stream.borrow().write("HTTP/1.1 200 OK\r\n\r\n".as_bytes()) {
                    Ok(written_byte) => {
                        println!("{} bytes written successfully!", written_byte);
                    },
                    Err(e) => {
                        panic!("Fail {}", e);
                    }
                };

                println!("accepted new connection");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
