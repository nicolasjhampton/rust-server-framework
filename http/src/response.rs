use std::net::TcpStream;
use std::io::{BufWriter, Write};


pub struct Response {
    pub writer: BufWriter<TcpStream>
}

impl Response {
    pub fn new(stream: TcpStream) -> Response {
        let writer = BufWriter::new(stream);
        Response {
            writer
        }
    }

    pub fn send_response(&mut self, response: String) {
        self.writer.write_all(response.as_bytes()).unwrap();
        self.writer.flush().unwrap();
    }

    pub fn send_200(&mut self, html: String) {
        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}\r\n", html);
        println!("Response:\r\n\r\n{}", response);
        self.send_response(response);
    }

    pub fn send_404(&mut self) {
        let response = String::from("HTTP/1.1 404 NOT FOUND\r\n\r\n");
        println!("Response:\r\n\r\n{}", response);
        self.send_response(response);
    }
}
