use crate::http::enums::{HttpMethod, HttpVersion};
use crate::http::headers::HttpHeaders;
use crate::http::request::HttpRequest;
use crate::http::util::errors::HttpParseError;
use std::str::FromStr;

pub fn parse_http_request(request: &str) -> Result<HttpRequest, HttpParseError> {
    let seperator = request
        .find("\r\n\r\n")
        .ok_or_else(|| HttpParseError::MalformedRequest("No headers found".to_string()))?;

    let head = &request[..seperator];

    let mut lines = head.lines();

    let request_line = lines
        .next()
        .ok_or_else(|| HttpParseError::MalformedRequest("Malformed Request Line".to_string()))?;

    let parts: Vec<&str> = request_line.split(' ').collect();

    // checking if we have all parts of the request line
    if parts.len() < 3 {

        if parts.is_empty() || parts[0].trim().is_empty() {
            return Err(HttpParseError::MalformedRequest("Missing HTTP method".to_string(),));
        } else if parts.len() == 1 || parts[1].trim().is_empty() {
            return Err(HttpParseError::MalformedRequest("Missing Path".to_string()));
        } else {
            return Err(HttpParseError::MalformedRequest("Missing HTTP version".to_string(),));
        }
    }

    let method_str = parts[0].trim();
    let path_str = parts[1].trim();
    let version_str = parts[2].trim();

    // Check for empty parts after trimming
    if method_str.is_empty() {
        return Err(HttpParseError::MalformedRequest("Missing HTTP method".to_string(),));
    }
    if path_str.is_empty() {
        return Err(HttpParseError::MalformedRequest("Missing Path".to_string()));
    }
    if version_str.is_empty() {
        return Err(HttpParseError::MalformedRequest("Missing HTTP version".to_string(),));
    }

    let method = HttpMethod::from_str(method_str)
        .map_err(|_| HttpParseError::UnsupportedMethod("Invalid HTTP method".to_string()))?;

    let path = path_str.to_string();

    let version = HttpVersion::from_str(version_str)
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

    let origin = parse_host_and_port(host);

    let body_start = seperator + 4; // Skip the "\r\n\r\n"
    let body = if body_start < request.len() {
        request[body_start ..].as_bytes().to_vec()
    } else {
        Vec::new()
    };


    Ok(HttpRequest {
        method,
        path,
        version,
        headers,
        origin,
        body: Some(body),
    })
}

fn parse_host_and_port(host: &str) -> (String, u16) {
    // Handle IPv6 addresses in brackets like [::1]:8080
    if host.starts_with('[') {
        if let Some(bracket_end) = host.find(']') {
            let ipv6_part = &host[..bracket_end + 1]; // Include the closing bracket

            // Check if there's a port after the closing bracket
            if bracket_end + 1 < host.len() && host.chars().nth(bracket_end + 1) == Some(':') {
                let port_str = &host[bracket_end + 2..];
                let port = port_str.parse().unwrap_or(80);
                return (ipv6_part.to_string(), port);
            } else {
                return (ipv6_part.to_string(), 80);
            }
        }
    }

    // Handle regular hostnames and IPv4 addresses
    if let Some((hostname, port_str)) = host.rsplit_once(':') {
        let port = port_str.parse().unwrap_or(80);
        (hostname.to_string(), port)
    } else {
        (host.to_string(), 80)
    }
}
