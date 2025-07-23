// src/main.rs
#![allow(unused_imports)]
#![allow(dead_code)]

mod forwader;
mod enums;

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
mod http_utility;

use crate::enums::{HttpRequest, HttpResponse};
use crate::http_utility::{parse_http_request, verify_http_request};
use crate::forwader::forward_to_upstream; 


fn handle_client(mut stream: TcpStream){
    let mut buffer: [u8; 4096] = [0; 4096];
    let bytes_read: usize = stream.read(&mut buffer).expect("Failed to read from stream");

    let request: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buffer[..bytes_read]);

    if let Some(req) = http_utility::parse_http_request(&request){
        println!("Parsed request: Method: {}\nPath: {}\nVersion: {}\nOrigin: {}\n", 
        req.method,
        req.path, 
        req.version, 
        req.origin.0);
        match http_utility::verify_http_request(&req){
            Ok(_) => {
                println!("Request is valid");
                println!("Forwarding request to upstream server at {}:{}", req.origin.0, req.origin.1);
                let req_str = req.to_string();
                println!("Request string: {}", req_str);
                match forwader::forward_to_upstream(&req) {
                    Ok(upstream_response) => {
                        println!("Received response from upstream server");
                        let response_str = upstream_response.to_string();
                        stream.write_all(response_str.as_bytes()).expect("Failed to write response to stream"); 
                    }
                    Err(response) => {
                        println!("Error forwarding request to upstream server: {}", response);
                        let error_str = response.to_string();
                        stream.write_all(error_str.as_bytes()).expect("Failed to write error response");
                    }
                }
            },
            Err(e) => {
                println!("Error verifying request!\nHttpResponse:\n\n{}", e);
                let error_str = e.to_string();
                stream.write_all(error_str.as_bytes()).expect("Failed to write error response");
            }
        }
    } else {
        println!("Failed to parse request");
        return; 
    }
  


}

fn main() {
    let addr: &'static str = "127.0.0.1:8080";
    let listener: TcpListener = TcpListener::bind(addr).expect("Failed to bind to address");

    println!("Server listening on {}", addr);   

    for stream in listener.incoming(){
        match stream{
            Ok(stream) => {
                println!("New connection established");
                handle_client(stream);
            },
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);
        }
    }
}
}
