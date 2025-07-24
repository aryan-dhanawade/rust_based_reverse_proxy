/// This module defines the HTTP request structure used in the Orion project.
/// src/http/request.rs
use crate::http::enums::{HttpMethod, HttpVersion};
use crate::http::headers::HttpHeaders;
use std::fmt;

#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub path: String,
    pub version: HttpVersion,
    pub headers: HttpHeaders,
    pub origin: (String, u16), // (host, port)
    pub body: Option<Vec<u8>>,
}

impl HttpRequest {
    pub fn new(method: HttpMethod, path: impl Into<String>) -> Self {
        Self {
            method,
            path: path.into(),
            version: HttpVersion::HTTP1_1,
            headers: HttpHeaders::new(),
            origin: ("localhost".to_string(), 80),
            body: None,
        }
    }

    pub fn with_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(key.into(), value.into());
        self
    }

    pub fn with_body(mut self, body: Vec<u8>) -> Self {
        let body_data: Vec<u8> = body.into();
        self.headers
            .insert("Content-Length".to_string(), body_data.len().to_string());
        self.body = Some(body_data);
        self
    }

    pub fn body_as_string(&self) -> Option<String> {
        self.body
            .as_ref()
            .and_then(|b| String::from_utf8(b.clone()).ok())
    }
}

impl fmt::Display for HttpRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} {} {}", self.method, self.path, self.version)?;

        for (key, value) in self.headers.iter() {
            writeln!(f, "{}: {}", key, value)?;
        }

        writeln!(f)?;

        if let Some(body) = &self.body {
            if let Ok(body_str) = String::from_utf8(body.clone()) {
                write!(f, "{}", body_str)?;
            }
        }

        Ok(())
    }
}