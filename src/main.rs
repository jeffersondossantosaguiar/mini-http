mod request;
mod response;
mod router;
use request::parse_request;
use response::Response;
use router::route;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();

    println!("Server listening on port 7878");

    loop {
        let (stream, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            handle_connection(stream).await;
        });
    }
}

async fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let mut lines = buf_reader.lines();

    let mut http_request: Vec<String> = Vec::new();

    while let Some(line) = lines.next_line().await.unwrap() {
        if line.is_empty() {
            break;
        }
        http_request.push(line);
    }

    println!("Received request: \n{:#?}", http_request);

    let request = parse_request(&http_request);

    let response = match request {
        Some(req) => route(&req).to_string(),
        None => Response::not_found().to_string(),
    };

    let _ = stream.write_all(response.as_bytes()).await;
}
