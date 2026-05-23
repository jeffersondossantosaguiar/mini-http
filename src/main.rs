use std::{
    collections::HashMap,
    fmt,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

struct Request {
    method: String,
    path: String,
    version: String,
}

struct Response {
    status_line: String,
    headers: HashMap<String, String>,
    body: String,
}

impl Response {
    fn ok(body: &str, content_type: Option<&str>) -> Response {
        let content_type = content_type.unwrap_or("text/plain");
        let headers = Self::build_headers(content_type, body);

        Response {
            status_line: "HTTP/1.1 200 OK".to_string(),
            headers,
            body: body.to_string(),
        }
    }

    fn not_found() -> Response {
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

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server listening on port 7878");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(&stream);
    }
}

fn handle_connection(mut stream: &TcpStream) {
    let buf_reader = BufReader::new(stream);
    let http_request: Vec<String> = buf_reader
        .lines()
        .map(|line| line.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Received request: \n{:#?}", http_request);

    let request = parse_request(&http_request);

    let response = match request {
        Some(req) => match (req.method.as_str(), req.path.as_str()) {
            ("GET", "/") => {
                let body = "Hello, World!";
                Response::ok(body, Some("text/plain")).to_string()
            }
            ("GET", "/about") => {
                let body = "This is a simple HTTP server written in Rust.";
                Response::ok(body, Some("text/plain")).to_string()
            }
            (_, _) => Response::not_found().to_string(),
        },
        None => Response::not_found().to_string(),
    };

    stream.write_all(response.as_bytes()).unwrap();
}

fn parse_request(http_request: &Vec<String>) -> Option<Request> {
    if let Some(line) = http_request.first() {
        let parts = line.split_whitespace().collect::<Vec<&str>>();

        let request: Option<Request> = match parts.as_slice() {
            [method, path, version] => Some(Request {
                method: method.to_string(),
                path: path.to_string(),
                version: version.to_string(),
            }),
            _ => None,
        };
        request
    } else {
        None
    }
}
