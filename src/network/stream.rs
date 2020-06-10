use std::io::prelude::*;
use std::net::TcpStream;
use bytes::{BytesMut, Buf};
use std::str;


pub trait TcpStreamExt {
    fn send_404(&mut self);
    fn print_request(&mut self);
    fn send_200(&mut self, html: String);
    fn send_response(&mut self, response: String);
    fn read_line(&self, line_number: usize) -> String;
}

impl TcpStreamExt for TcpStream {
    fn send_response(&mut self, response: String) {
        self.write_all(response.as_bytes()).unwrap();
        self.flush().unwrap();
    }

    fn send_200(&mut self, html: String) {
        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}\r\n", html);
        println!("Response:\r\n\r\n{}", response);
        self.send_response(response);
    }

    fn send_404(&mut self) {
        let response = String::from("HTTP/1.1 404 NOT FOUND\r\n\r\n");
        println!("Response:\r\n\r\n{}", response);
        self.send_response(response);
    }

    fn print_request(&mut self) {
        let mut buffer = [0; 512];
        self.read(&mut buffer).unwrap();
        println!("Request:\r\n\r\n{}\r\n", String::from_utf8_lossy(&buffer[..]));
    }

    fn read_line(&self, line_number: usize) -> String {
        let step = 50;
        let carriage_return = "\r\n";
        let mut count = 0;
        loop {
            count = count + step;
            let mut space = BytesMut::new();
            space.resize(count, 0);

            self.peek(&mut space[..]).unwrap();
            let copy = space.bytes();
            let mut substring = str::from_utf8(copy).unwrap();

            for line_count in 0..line_number {
                if let Some(index) = substring.find(carriage_return) {
                    if (line_count + 1) == line_number {
                        return String::from(&substring[0..index]);
                    } else {
                        let offset = index + carriage_return.len();
                        substring = substring.get(offset..).unwrap();
                    }
                }
            }
        }
    }
}
