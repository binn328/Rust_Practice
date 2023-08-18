// TCP 연결을 처리하는 모듈
use std::net::TcpListener;
use std::io::prelude::*;
use std::net::TcpStream;
fn main() {
    // 바인딩을 통해 127.0.0.1:7878로의 수신을 받아들임
    let listener = TcpListener::bind("localhost:20777").unwrap();

    // incoming은 스트림의 반복자를 반환
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
