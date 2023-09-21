#[warn(non_snake_case)]
pub mod thread_pool;
pub mod worker;

use std::{
    fs,
    io::prelude::*,
    net::{SocketAddr, TcpListener, TcpStream},
};

use thread_pool::ThreadPool;

fn main() {
    let address = "0.0.0.0:10000".parse::<SocketAddr>().unwrap();
    let server_socket = TcpListener::bind(&address).unwrap();

    let pool = ThreadPool::new(4);
    for stream in server_socket.incoming() {
        let stream = stream.unwrap();
        println!("{:?}", stream);

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
