use crate::http::enums::{HttpMethod, HttpVersion};
use crate::http::headers::HttpHeaders;
use crate::http::request::HttpRequest;
use crate::http::util::errors::HttpParseError;
use std::str::FromStr;

pub fn parse_http_request(request: &str) -> Result<HttpRequest, HttpParseError> {
    let (head, body) = request
        .split_once("\r\n\r\n")
        .ok_or_else(|| HttpParseError::MalformedRequest("".to_string()))?;

    let mut lines = head.lines();

    let request_line = lines
        .next()
        .ok_or_else(|| HttpParseError::MalformedRequest("Malformed Request Line".to_string()))?;

    let mut parts = request_line.split_whitespace();

    let method = HttpMethod::from_str(
        parts
            .next()
            .ok_or_else(|| HttpParseError::MalformedRequest("Missing HTTP method".to_string()))?,
    )
    .map_err(|_| HttpParseError::UnsupportedMethod("Invalid HTTP method".to_string()))?;

    let path = parts
        .next()
        .ok_or_else(|| HttpParseError::MalformedRequest("Missing Path".to_string()))?
        .to_string();

    let version = HttpVersion::from_str(
        parts
            .next()
            .ok_or_else(|| HttpParseError::MalformedRequest("Missing HTTP version".to_string()))?,
    )
    .map_err(|_| HttpParseError::UnsupportedHttpVersion("Invalid HTTP version".to_string()))?;

    let mut headers = HttpHeaders::new();
    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            break;
        }
        if let Some((key, value)) = line.split_once(':') {
            headers.insert(key.trim().to_ascii_lowercase(), value.trim().to_string());
        } else {
            return Err(HttpParseError::MalformedRequest(format!(
                "Invalid header line: {}",
                line
            )));
        }
    }

    let host = headers
        .get("host")
        .map(|s| s.as_str())
        .unwrap_or("127.0.0.1");
    let origin = if let Some((h, p)) = host.split_once(':') {
        (
            h.to_string(),
            p.parse().unwrap_or(80), //TODO: try result refactoring?
        )
    } else {
        (host.to_string(), 80)
    };

    let body_bytes = body.as_bytes().to_vec();

    Ok(HttpRequest {
        method,
        path,
        version,
        headers,
        origin,
        body: Some(body_bytes),
    })
}
