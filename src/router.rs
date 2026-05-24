use crate::request::Request;
use crate::response::Response;

pub fn route(request: &Request) -> Response {
    match (request.method.as_str(), request.path.as_str()) {
        ("GET", "/") => {
            let body = "Hello, World!";
            Response::ok(body, Some("text/plain"))
        }
        ("GET", "/about") => {
            let body = "This is a simple HTTP server written in Rust.";
            Response::ok(body, Some("text/plain"))
        }
        (_, _) => Response::not_found(),
    }
}
