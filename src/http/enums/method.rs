
/// This module defines the HTTP methods used in the Orion project.
/// src/http/enums/method.rs

use std::fmt;
#[derive(Debug, Clone, PartialEq)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    OPTIONS,
}

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
impl std::str::FromStr for HttpMethod {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "GET" => Ok(HttpMethod::GET),
            "POST" => Ok(HttpMethod::POST),
            "PUT" => Ok(HttpMethod::PUT),
            "DELETE" => Ok(HttpMethod::DELETE),
            "HEAD" => Ok(HttpMethod::HEAD),
            "OPTIONS" => Ok(HttpMethod::OPTIONS),
            _ => Err(format!("Unsupported HTTP method: {}", s)),
        }
    }
}