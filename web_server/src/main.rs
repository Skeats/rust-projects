use std::{
    fs,
    io::{
        BufRead,
        BufReader,
        Write
    },
    net::{
        TcpListener,
        TcpStream
    },
    thread,
    time::Duration
};
use web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        
        pool.execute(||  {
            handle_connection(stream);
        });
    }
}

fn handle_connection(stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line|  !line.is_empty())
        .collect();

    let request_line = &http_request[0];

    match &request_line[..] {
        "GET / HTTP/1.1" => send_file("200 OK", "index.html", &stream),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            send_file("200 OK", "index.html", &stream);
        },
        _ => send_file("404 NOT FOUND", "404.html", &stream),
    }
}

fn send_file(response_str: &str, file_path: &str, mut stream: &TcpStream) {
    let status_line = format!("HTTP/1.1 {response_str}");
    let contents = fs::read_to_string(file_path).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}