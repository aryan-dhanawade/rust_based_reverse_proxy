use std::str::FromStr;
use crate::http::enums::{HttpMethod, HttpVersion};
use crate::http::headers::HttpHeaders;
use crate::http::request::HttpRequest;

pub fn parse_http_request(request: &str) -> Result<HttpRequest, String> {
    let (head, body) = match request.split_once("\r\n\r\n") {
        Some((head, body)) => (head, Some(body.as_bytes())),
        None => (request, None),
    };

    let mut lines = head.lines();

    let request_line = lines
        .next()
        .ok_or_else(|| "Malformed HTTP Request: missing request line".to_string())?;

    let mut parts = request_line.split_whitespace();

    let method = parts
                            .next()
                            .ok_or_else(|| "Missing HTTP method".to_string())
                            .and_then(|m| HttpMethod::from_str(m))?; 
                    
    let path = parts
        .next()
        .ok_or_else(|| "Missing HTTP path".to_string())?
        .to_string();

// fix version too
    let version =  parts
                                    .next()
                                    .ok_or_else(|| "Missing HTTP version".to_string())
                                    .and_then(|v| HttpVersion::from_str(v).map_err(|e| e.to_string()))?;


    let mut headers = HttpHeaders::new();
    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            break;
        }
        if let Some((key, value)) = line.split_once(':') {
            headers.insert(key.trim().to_ascii_lowercase(), value.trim().to_string());
        } else {
            return Err(format!("Malformed header line: '{}'", line));
        }
    }

    let host = headers.get("host").map(|s| s.as_str()).unwrap_or("127.0.0.1");
    let origin = if let Some((h, p)) = host.split_once(':') {
        (
            h.to_string(),
            p.parse().unwrap_or(80), // consider making this a Result too
        )
    } else {
        (host.to_string(), 80)
    };

    let body = body.map(|b| b.to_vec());

    Ok(HttpRequest {
        method,
        path,
        version,
        headers,
        origin,
        body,
    })
}
