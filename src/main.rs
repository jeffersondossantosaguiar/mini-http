mod request;
mod response;
mod router;
use request::parse_request;
use response::Response;
use router::route;
use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

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
        Some(req) => route(&req).to_string(),
        None => Response::not_found().to_string(),
    };

    stream.write_all(response.as_bytes()).unwrap();
}
