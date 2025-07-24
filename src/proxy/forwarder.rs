// src/proxy/forwader.rs

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

use crate::enums::{HttpRequest, HttpResponse};

pub fn forward_to_upstream(req: &HttpRequest) -> Result<HttpResponse, HttpResponse> {
    let (host, port) = ("127.0.0.1", 8081); // Default upstream server
    let mut upstream = TcpStream::connect((host, port))
    .map_err(|e| HttpResponse::new(500, format!("Bad Gateway {}", e)))?;

    upstream.write_all(req.to_string().as_bytes())
    .map_err(|e| HttpResponse::new(500, format!("Bad Gateway {}", e)))?;


    let mut response_buffer = Vec::new();
    upstream.read_to_end(&mut response_buffer)
        .map_err(|e| HttpResponse::new(502, format!("Bad Gateway {}", e)))?;

    let response_string = String::from_utf8_lossy(&response_buffer).to_string();
    print!("response_string: {}", response_string);


    Ok(HttpResponse::new(200, response_string).with_header("X-Forwarded-For", format!("{}:{}", host, port)) )
}
