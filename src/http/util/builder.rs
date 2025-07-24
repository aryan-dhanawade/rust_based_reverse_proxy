use crate::http::enums::HttpStatus;
use crate::http::response::HttpResponse;


pub fn create_error_response(status: HttpStatus, message: impl Into<String>) -> HttpResponse {
    HttpResponse::text(status, message)
        .with_header("Connection", "close")
}

/// Create a successful response with optional custom headers
pub fn create_success_response(body: impl Into<String>) -> HttpResponse {
    HttpResponse::text(HttpStatus::Ok, body)
        .with_header("Connection", "keep-alive")
}

/// Create an HTML response
pub fn create_html_response(html: impl Into<String>) -> HttpResponse {
    HttpResponse::html(HttpStatus::Ok, html)
        .with_header("Connection", "keep-alive")
}

/// Create a JSON response
pub fn create_json_response(json: impl Into<String>) -> HttpResponse {
    HttpResponse::json(HttpStatus::Ok, json)
        .with_header("Connection", "keep-alive")
}