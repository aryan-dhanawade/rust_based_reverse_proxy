use std::fmt;

pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub version: String,
    pub headers: Vec<(String, String)>,
    pub origin: (String, u16), // (host, port)
}

impl HttpRequest {
    pub fn to_string(&self) -> String {
        let mut s = format!("{} {} {}\r\n", self.method, self.path, self.version);
        for (k, v) in &self.headers {
            s.push_str(&format!("{}: {}\r\n", k, v));
        }
        s.push_str("\r\n");
        return s
    }
}

pub struct HttpResponse {
    pub status_code: u16,
    pub reason_phrase: String,
    pub headers: Vec<(String, String)>,
    pub body: String,
}

impl HttpResponse {
    /// Constructor: sets default headers and computes Content-Length
    pub fn new(status_code: u16, body: impl Into<String>) -> Self {
        let body = body.into();
        let reason = match status_code {
            200 => "OK",
            400 => "Bad Request",
            405 => "Method Not Allowed",
            505 => "HTTP Version Not Supported",
            500 => "Internal Server Error",
            _ => "Unknown",
        }
        .to_string();

        // default headers
        let mut headers = vec![
            ("Content-Type".into(), "text/plain".into()),
            ("Server".into(), "Orion/1.0".into()),
        ];
        headers.push(("Content-Length".into(), body.len().to_string()));

        HttpResponse {
            status_code,
            reason_phrase: reason,
            headers,
            body,
        }
    }

    /// Append a custom header
    pub fn with_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push((key.into(), value.into()));
        self
    }
}

impl fmt::Display for HttpResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // status line uses stored reason_phrase
        writeln!(f, "HTTP/1.1 {} {}", self.status_code, self.reason_phrase)?;

        // headers
        for (key, value) in &self.headers {
            writeln!(f, "{}: {}", key, value)?;
        }

        writeln!(f)?;
        write!(f, "{}", self.body)
    }
}
