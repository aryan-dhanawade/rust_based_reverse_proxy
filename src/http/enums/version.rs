/// This module defines the HTTP version enum used in the Orion project.
/// src/http/enums/version.rs
use std::fmt;
use crate::http::util::errors::HttpParseError;
#[derive(Debug, Clone, PartialEq)]
pub enum HttpVersion {
    HTTP1_0,
    HTTP1_1,
    HTTP2_0,
}

impl fmt::Display for HttpVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let version_str = match self {
            HttpVersion::HTTP1_0 => "HTTP/1.0",
            HttpVersion::HTTP1_1 => "HTTP/1.1",
            HttpVersion::HTTP2_0 => "HTTP/2.0",
        };
        write!(f, "{}", version_str)
    }
}

impl std::str::FromStr for HttpVersion {
    type Err = HttpParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HTTP/1.0" => Ok(HttpVersion::HTTP1_0),
            "HTTP/1.1" => Ok(HttpVersion::HTTP1_1),
            "HTTP/2.0" | "HTTP/2" => Ok(HttpVersion::HTTP2_0),
            _ => Err(HttpParseError::UnsupportedHttpVersion(s.to_string())),
        }
    }
}