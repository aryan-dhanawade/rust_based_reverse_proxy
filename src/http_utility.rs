// src/http_request_parser.rs

/// Parses the HTTP request line into method, path, and version.
/// Example: "GET /index HTTP/1.1" => Some(HttpRequest)

use crate::enums::{HttpRequest, HttpResponse};
/// Parses the HTTP request into HttpRequest
pub fn parse_http_request(request: &str) -> Option<HttpRequest> {
    let mut lines = request.lines();
    let request_line = lines.next()?;
    let mut parts = request_line.split_whitespace();

    let method  = parts.next()?.to_string();
    let path    = parts.next()?.to_string();
    let version = parts.next()?.to_string();

    let mut headers = Vec::new();
    for line in lines {
        let line = line.trim();
        if line.is_empty() { break; }
        if let Some((k, v)) = line.split_once(':') {
            headers.push((k.trim().to_ascii_lowercase(), v.trim().to_string()));
        }
    }

    let host_header = headers.iter()
        .find(|(k, _)| k == "host")
        .map(|(_, v)| v.as_str())
        .unwrap_or("127.0.0.1");

    let origin = if let Some((h, p)) = host_header.split_once(':') {
        (h.to_string(), p.parse().unwrap_or(80))
    } else {
        (host_header.to_string(), 80) // default port 80
    };


    Some(HttpRequest { method, path, version, headers, origin })
}

/// Validate request and return either Ok(response) or Err(response)
pub fn verify_http_request(req: &HttpRequest) -> Result<(), HttpResponse> {
    if req.method.is_empty() || req.path.is_empty() || req.version.is_empty() {
        return Err(HttpResponse::new(400, "Malformed Request"));
    }
    let allowed = ["GET","POST","PUT","DELETE","HEAD","OPTIONS"];
    if !allowed.contains(&req.method.as_str()) {
        return Err(HttpResponse::new(405, format!("Method Not Allowed ({})", req.method)));
    }
    if req.version != "HTTP/1.1" {
        return Err(HttpResponse::new(505, format!("Version Not Supported ({})", req.version)));
    }
    if !req.headers.iter().any(|(k,_)| k == "host") {
        return Err(HttpResponse::new(400, "Bad Request: Missing Host Header"));
    }
    // all good
    Ok(())
}
