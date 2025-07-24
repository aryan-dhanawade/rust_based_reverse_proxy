
/// This module defines the HTTP methods used in the Orion project.
/// src/http/enums/method.rs

use std::fmt;
use crate::http::util::errors::HttpParseError;
#[derive(Debug, Clone, PartialEq)]
// We can support more methods in the future, but for now we will just use these.
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    OPTIONS,
}
// Implement for string conversion and parsing
impl fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let method_str = match self {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST",
            HttpMethod::PUT => "PUT",
            HttpMethod::DELETE => "DELETE",
            HttpMethod::HEAD => "HEAD",
            HttpMethod::OPTIONS => "OPTIONS",
        };
        write!(f, "{}", method_str)
    }
}
// Implement FromStr to parse from string
impl std::str::FromStr for HttpMethod {
    type Err = HttpParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(HttpMethod::GET),
            "POST" => Ok(HttpMethod::POST),
            "PUT" => Ok(HttpMethod::PUT),
            "DELETE" => Ok(HttpMethod::DELETE),
            "HEAD" => Ok(HttpMethod::HEAD),
            "OPTIONS" => Ok(HttpMethod::OPTIONS),
            _ => Err(HttpParseError::UnsupportedMethod(s.to_string())),
        }
    }
}