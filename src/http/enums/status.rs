/// This module defines the HTTP status codes used in the Orion project.
/// src/http/enums/status.rs

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum HttpStatus {
    // Success Codes
    Ok = 200,
    Created = 201,
    NoContent = 204,

    // Redirection Codes
    MovedPermanently = 301,
    Found = 302,
    NotModified = 304,

    // Client Error Codes
    BadRequest = 400,
    Unauthorized = 401,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    PayloadTooLarge = 413,
    RequestUriTooLong = 414,

    // Server Error Codes
    InternalServerError = 500,
    NotImplemented = 501,
    BadGateway = 502,
    ServiceUnavailable = 503,
    HttpVersionNotSupported = 505,
}

impl HttpStatus {
    pub fn code(&self) -> u16 {
        *self as u16
    }
    pub fn reason_phrase(&self) -> &str {
        match self {
            HttpStatus::Ok => "OK",
            HttpStatus::Created => "Created",
            HttpStatus::NoContent => "No Content",
            HttpStatus::MovedPermanently => "Moved Permanently",
            HttpStatus::Found => "Found",
            HttpStatus::NotModified => "Not Modified",
            HttpStatus::BadRequest => "Bad Request",
            HttpStatus::Unauthorized => "Unauthorized",
            HttpStatus::Forbidden => "Forbidden",
            HttpStatus::NotFound => "Not Found",
            HttpStatus::MethodNotAllowed => "Method Not Allowed",
            HttpStatus::PayloadTooLarge => "Payload Too Large",
            HttpStatus::RequestUriTooLong => "Request URI Too Long",
            HttpStatus::InternalServerError => "Internal Server Error",
            HttpStatus::NotImplemented => "Not Implemented",
            HttpStatus::BadGateway => "Bad Gateway",
            HttpStatus::ServiceUnavailable => "Service Unavailable",
            HttpStatus::HttpVersionNotSupported => "HTTP Version Not Supported",

        }
    }
    
    pub fn from_code(code: u16) -> Option<HttpStatus> {
        match code {
            200 => Some(HttpStatus::Ok),
            201 => Some(HttpStatus::Created),
            204 => Some(HttpStatus::NoContent),
            301 => Some(HttpStatus::MovedPermanently),
            302 => Some(HttpStatus::Found),
            304 => Some(HttpStatus::NotModified),
            400 => Some(HttpStatus::BadRequest),
            401 => Some(HttpStatus::Unauthorized),
            403 => Some(HttpStatus::Forbidden),
            404 => Some(HttpStatus::NotFound),
            405 => Some(HttpStatus::MethodNotAllowed),
            413 => Some(HttpStatus::PayloadTooLarge),
            414 => Some(HttpStatus::RequestUriTooLong),
            500 => Some(HttpStatus::InternalServerError),
            501 => Some(HttpStatus::NotImplemented),
            502 => Some(HttpStatus::BadGateway),
            503 => Some(HttpStatus::ServiceUnavailable),
            505 => Some(HttpStatus::HttpVersionNotSupported),
            _ => None,
        }
    }
}