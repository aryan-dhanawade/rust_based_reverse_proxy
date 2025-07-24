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



#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::enums::{HttpMethod, HttpStatus, HttpVersion};
    use crate::http::request::HttpRequest;
    
    fn make_request(path: &str, headers: Vec<(&str, &str)>, body: Option<&str>) -> HttpRequest {

        let mut request =  HttpRequest::new(
            HttpMethod::GET,
            path.to_string(),
        );
        for (key, value) in headers {
            request = request.with_header(key, value);
        }   

        if let Some(b) = body {
            request = request.with_body(b.as_bytes().to_vec());
        }

        request
    }

    #[test]
    fn test_valid_request() {
        let req = make_request(
            "/valid?x=1",
            vec![("host", "example.com")],
            Some("body"),
        );
        let result = verify_http_request(&req);
        assert!(result.is_ok());
    }

    #[test]
    fn test_empty_path() {
        let req = make_request(
            "",
            vec![("host", "example.com")],
            None,
        );
        let result = verify_http_request(&req);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().status, HttpStatus::BadRequest);
    }

    #[test]
    fn test_missing_host_header() {
        let req = make_request(
            "/somepath",
            vec![],
            None,
        );
        let result = verify_http_request(&req);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().status, HttpStatus::BadRequest);
    }

    #[test]
    fn test_unsupported_http_version() {
        let mut req = make_request(
            "/somepath",
            vec![("host", "example.com")],
            None,
        );
        req.version = HttpVersion::HTTP1_0;
        let result = verify_http_request(&req);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().status, HttpStatus::HttpVersionNotSupported);
    }

    #[test]
    fn test_large_body() {
        let big_body = vec![b'a'; HttpLimits::MAX_BODY_SIZE + 1];
        let req = HttpRequest::new( 
            HttpMethod::POST,
            "/upload".to_string())
            .with_header("host", "example.com")
            .with_body(big_body);
        
        let result = verify_http_request(&req);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().status, HttpStatus::PayloadTooLarge);
    }
}
