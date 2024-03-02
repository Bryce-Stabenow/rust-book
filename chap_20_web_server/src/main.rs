use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream}
};

use chap_20_web_server::ThreadPool;

fn main() {
    let server_address = "127.0.0.1:7878";
    let listener: TcpListener = TcpListener::bind(server_address)
        .expect(format!("Unable to listen at {server_address}").as_str());
    let pool = ThreadPool::new(10);

    for stream in listener.incoming() {
        let data = stream.unwrap();

        pool.execute(|| handle_request(data));
    }
}

fn handle_request(mut data: TcpStream) {
    let buf_reader: BufReader<&mut TcpStream> = BufReader::new(&mut data);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let req_head: &String = &http_request[0];

    let (res_status, filename) = match req_head.as_str() {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let res_body = std::fs::read_to_string(filename).unwrap();
    let res_len = res_body.len();

    let res = format!("{res_status}\r\nContent-Length: {res_len}\r\n\r\n{res_body}");
    data.write_all(res.as_bytes()).unwrap();
}
