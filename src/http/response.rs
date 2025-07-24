/// This module defines the `HttpResponse` struct and its associated methods for creating HTTP responses.
/// src/http/response.rs

use crate::http::enums::{HttpMethod, HttpVersion, HttpStatus};
use crate::http::headers::HttpHeaders;
use std::fmt;
pub struct HttpResponse {
    pub status: HttpStatus,
    pub headers: HttpHeaders,
    pub body: Vec<u8>,
}

impl HttpResponse {
    /// Constructor: sets default headers and computes Content-Length
    fn standard(status: HttpStatus, body: impl Into<Vec<u8>>) -> Self {
        let body = body.into();
        let mut headers = HttpHeaders::new();

        // default headers
        headers.insert("Content-Type".to_string(), "text/plain".to_string());
        headers.insert("Content-Length".to_string(), body.len().to_string());
        headers.insert("Server".to_string(), "Orion/1.0".to_string());

        Self {
            status,
            headers,
            body,
        }
    }
    /// Create a new response with a status code and body   
    pub fn text(status: HttpStatus, body: impl Into<String>) -> Self {
        let body = body.into();
        let mut response = Self::standard(status, body);
        response.headers.insert(
            "Content-Type".to_string(),
            "text/plain; charset=utf-8".to_string(),
        );
        response
    }
    /// Create a HTML response with the given status and body
    pub fn html(status: HttpStatus, body: impl Into<String>) -> Self {
        let body = body.into();
        let mut response = Self::standard(status, body);
        response.headers.insert(
            "Content-Type".to_string(),
            "text/html; charset=utf-8".to_string(),
        );
        response
    }
    /// Create a JSON response
    pub fn json(status: HttpStatus, body: impl Into<String>) -> Self {
        let body = body.into();
        let mut response = Self::standard(status, body);
        response
            .headers
            .insert("Content-Type".to_string(), "application/json".to_string());
        response
    }

    /// Append a custom header
    pub fn with_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(key.into(), value.into());
        self
    }

    pub fn body_as_string(&self) -> Option<String> {
        String::from_utf8(self.body.clone()).ok()
    }

    pub fn status_code(&self) -> u16 {
        self.status.code()
    }
}

impl fmt::Display for HttpResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Status line
        writeln!(
            f,
            "HTTP/1.1 {} {}",
            self.status.code(),
            self.status.reason_phrase()
        )?;

        // Headers
        for (key, value) in self.headers.iter() {
            writeln!(f, "{}: {}", key, value)?;
        }

        writeln!(f)?;

        // Body (attempt to display as string, fall back to raw bytes info)
        if let Ok(body_str) = String::from_utf8(self.body.clone()) {
            write!(f, "{}", body_str)
        } else {
            write!(f, "[Binary data: {} bytes]", self.body.len())
        }
    }
}