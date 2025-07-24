use crate::http::enums::{HttpStatus, HttpVersion};
use crate::http::request::HttpRequest;
use crate::http::response::HttpResponse;
use crate::http::util::constants::HttpLimits;
use crate::http::util::query_params::extract_query_params;


pub fn verify_http_request(req: &HttpRequest) -> Result<(), HttpResponse> {

    let (path, query) = extract_query_params(req.path.as_str());
    // Check if the path is valid 
    if path.is_empty() {
        return Err(HttpResponse::text(
            HttpStatus::BadRequest,
            "Path cannot be empty".to_string(),
        ));
    }

    if path.len() > HttpLimits::MAX_URL_LENGTH {
        return Err(HttpResponse::text(
            HttpStatus::RequestUriTooLong,
            format!("Path exceeds maximum length of {} characters", HttpLimits::MAX_URL_LENGTH),
        ));
    }

    if query.len() > HttpLimits::MAX_QUERY_PARAMS {
        return Err(HttpResponse::text(
            HttpStatus::BadRequest,
            format!("Too many query parameters, maximum is {}", HttpLimits::MAX_QUERY_PARAMS),
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

    let header_count = req.headers.iter().count();
    if header_count > HttpLimits::MAX_HEADERS {
        return Err(HttpResponse::text(
            HttpStatus::BadRequest,
            format!("Too many headers, maximum is {}", HttpLimits::MAX_HEADERS),
        ));
    }
    for (key, value) in req.headers.iter() {
        if key.len() > HttpLimits::MAX_HEADER_NAME_LEN {
            return Err(HttpResponse::text(
                HttpStatus::BadRequest,
                format!("Header name '{}' exceeds maximum length of {}", key, HttpLimits::MAX_HEADER_NAME_LEN),
            ));
        }
        if value.len() > HttpLimits::MAX_HEADER_VALUE_LEN {
            return Err(HttpResponse::text(
                HttpStatus::BadRequest,
                format!("Header value for '{}' exceeds maximum length of {}", key, HttpLimits::MAX_HEADER_VALUE_LEN),
            ));
        }
    }



    if req.headers.get("host").is_none() {
        return Err(HttpResponse::text(
            HttpStatus::BadRequest,
            "Host header is required".to_string(),
        ));
    }

    if let Some(body) = req.body.as_ref() {
        if body.len() > HttpLimits::MAX_BODY_SIZE {
            return Err(HttpResponse::text(
                HttpStatus::PayloadTooLarge,
                format!("Request body exceeds maximum size of {} bytes", HttpLimits::MAX_BODY_SIZE),
            ));
        }
    }

    Ok(()) // we wont be responding with an HttpResponse if everything is fine, this will go to the forwarding logic.

}



