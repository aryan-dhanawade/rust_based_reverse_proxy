use crate::http::enums::{version, HttpMethod, HttpStatus, HttpVersion};
use crate::http::headers::{self, HttpHeaders};
use crate::http::request::HttpRequest;
use crate::http::response::HttpResponse;
use std::str::FromStr;

pub fn verify_http_request(req: &HttpRequest) -> Result<(), HttpResponse> {
    // Check if the path is valid 
    if req.path.is_empty() {
        return Err(HttpResponse::text(
            HttpStatus::BadRequest,
            "Path cannot be empty".to_string(),
        ));
    }

    if req.version != HttpVersion::HTTP1_1 {
        return Err(HttpResponse::text(
            HttpStatus::HttpVersionNotSupported,
            format!(
                "HTTP version ({}) is not supported",
                req.version.to_string()
            ),
        ));
    }

    if req.headers.get("host").is_none() {
        return Err(HttpResponse::text(
            HttpStatus::BadRequest,
            "Host header is required".to_string(),
        ));
    }

    Ok(()) // we wont be responding with an HttpResponse if everything is fine, this will go to the forwarding logic.

}



