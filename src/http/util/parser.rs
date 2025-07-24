use std::str::FromStr;

use crate::http::enums::{version, HttpMethod, HttpStatus, HttpVersion};
use crate::http::headers::{self, HttpHeaders};
use crate::http::request::HttpRequest;
use crate::http::response::HttpResponse;

pub fn parse_http_request(request: &str) -> Option<HttpRequest> {
    let (head, body) = match request.split_once("\r\n\r\n") {
        Some((head, body)) => (head, Some(body.as_bytes())),
        None => (request, None),
    };


    let mut lines = head.lines();
    let request_line = lines.next()?;

    let mut parts = request_line.split_whitespace();

    let method = HttpMethod::from_str(parts.next()?).ok()?;
    let path = parts.next()?.to_string();
    let version = HttpVersion::from_str(parts.next()?).ok()?;

    let mut headers = HttpHeaders::new();
    for line in lines {
        let line = line.trim();
        if line.is_empty(){ break; }
        if let Some((key, value)) = line.split_once(':') {
            headers.insert(key.trim().to_ascii_lowercase(), value.trim().to_string());
        }
    }

    let host = headers.get("host").map(|s| s.as_str()).unwrap_or("127.0.0.1");
    let origin = if let Some((h,p)) = host.split_once(':') {
        (h.to_string(), p.parse().unwrap_or(80))
    } else {
        (host.to_string(), 80)
    };

    // now build the body from that slice, if any
    let body = body
        .map(|b| b.to_vec());

    Some(HttpRequest {
        method, path, version, headers, origin, body
    })


}






