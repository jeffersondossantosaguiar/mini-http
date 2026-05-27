pub struct Request {
    pub method: String,
    pub path: String,
    pub version: String,
}

pub fn parse_request(http_request: &Vec<String>) -> Option<Request> {
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

#[cfg(test)]
mod tests {
    mod parse_request {
        use super::super::*;

        #[test]
        fn test_parse_request() {
            let http_request = vec![
                "GET /index.html HTTP/1.1".to_string(),
                "Host: example.com".to_string(),
                "User-Agent: TestAgent".to_string(),
            ];

            let request = parse_request(&http_request).unwrap();
            assert_eq!(request.method, "GET");
            assert_eq!(request.path, "/index.html");
            assert_eq!(request.version, "HTTP/1.1");
        }

        #[test]
        fn test_parse_request_invalid() {
            let http_request = vec![
                "INVALID REQUEST".to_string(),
                "Host: example.com".to_string(),
                "User-Agent: TestAgent".to_string(),
            ];

            let request = parse_request(&http_request);
            assert!(request.is_none());
        }
    }
}
