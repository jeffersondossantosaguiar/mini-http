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
