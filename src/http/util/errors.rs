use std::fmt;


#[derive(Debug)]
pub enum HttpParseError{
    EmptyRequest,
    MalformedRequest(String),
    UnsupportedMethod(String),
    UnsupportedHttpVersion(String),

}


impl fmt::Display for HttpParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpParseError::EmptyRequest => write!(f, "HTTP request cannot be empty"),
            HttpParseError::MalformedRequest(http_error) => write!(f, "Malformed HTTP request {}", http_error),
            HttpParseError::UnsupportedMethod(method) => write!(f, "Unsupported HTTP method: {}", method),
            HttpParseError::UnsupportedHttpVersion(version) => write!(f, "Unsupported HTTP version: {}", version),
        }
    }
}
impl std::error::Error for HttpParseError {}