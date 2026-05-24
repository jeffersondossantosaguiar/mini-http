use std::{collections::HashMap, fmt};

pub struct Response {
    pub status_line: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl Response {
    pub fn ok(body: &str, content_type: Option<&str>) -> Response {
        let content_type = content_type.unwrap_or("text/plain");
        let headers = Self::build_headers(content_type, body);

        Response {
            status_line: "HTTP/1.1 200 OK".to_string(),
            headers,
            body: body.to_string(),
        }
    }

    pub fn not_found() -> Response {
        let body = "Página não encontrada";
        let headers = Self::build_headers("text/plain", body);

        Response {
            status_line: "HTTP/1.1 404 NOT FOUND".to_string(),
            headers,
            body: body.to_string(),
        }
    }

    fn build_headers(content_type: &str, body: &str) -> HashMap<String, String> {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), content_type.to_string());
        headers.insert("Content-Length".to_string(), body.len().to_string());
        headers
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let headers = self
            .headers
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v))
            .collect::<Vec<String>>()
            .join("\r\n");

        write!(
            f,
            "{}\r\n{}\r\n\r\n{}",
            self.status_line, headers, self.body
        )
    }
}
