use retry::delay::Fixed;
use retry::retry_with_index;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use bytes::{BytesMut, Buf};
use std::str;
use std::thread;

pub struct Server {
    listener: TcpListener
}

impl Server {
    fn get_conn() -> Result<std::net::TcpListener, retry::Error<String>> {
        return retry_with_index(Fixed::from_millis(100), |index| {
            match TcpListener::bind("127.0.0.1:7878") {
                Ok(sock) => Ok(sock),
                Err(e) => Err(format!("Failed {} times: {}", index, e)),
            }
        });
    }

    pub fn new() -> Server {
        let listener = Server::get_conn().unwrap();
        Server {
            listener: listener
        }
    }

    pub fn run<S: 'static>(&mut self, server_logic: S) 
    where
        S: Fn(&mut TcpStream) + std::marker::Sync + std::marker::Send + Copy
    {
        for stream in self.listener.incoming() {
            
            thread::spawn(move || {
                let mut stream = stream.expect("Error binding");
                server_logic(&mut stream);
            });
        }
    }
}

pub trait TcpStreamExt {
    fn send_404(&mut self);
    fn send_response(&mut self, html: String);
    fn read_line(&mut self, line_number: usize) -> String;
    fn handle<H>(&mut self, route: &str, handler: H) 
        where H: Fn(&mut TcpStream);
    fn get<H>(&mut self, route: &str, handler: H) -> bool
        where H: Fn(&mut TcpStream);
    fn hole<H>(&mut self, handler: H) 
        where H: Fn(&mut TcpStream);
}

pub fn print_request(stream: &mut TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    println!("Request:\r\n\r\n{}\r\n", String::from_utf8_lossy(&buffer[..]));
}


impl TcpStreamExt for TcpStream {
    fn send_response(&mut self, html: String) {
        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}\r\n", html);
        println!("Response:\r\n\r\n{}", response);
        self.write(response.as_bytes()).unwrap();
    }

    fn send_404(&mut self) {
        let response = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        println!("Response:\r\n\r\n{}", response);
        self.write(response.as_bytes()).unwrap();
    }

    fn read_line(&mut self, line_number: usize) -> String {
        let step = 50;
        let carriage_return = "\r\n";
        let mut count = 0;
        loop {
            count = count + step;
            let mut space = BytesMut::new();
            space.resize(count, 0);

            self.peek(&mut space[..]).unwrap();
            let copy = space.bytes();
            let mut substring = str::from_utf8(copy).unwrap(); // we have 0..(offset + step)

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

    fn get<H>(&mut self, route: &str, handler: H) -> bool
    where
        H: Fn(&mut TcpStream) -> ()
    {
        let mut buf = [0; 4];
        self.peek(&mut buf).expect("peek failed");
        if buf.starts_with(b"GET") {
            self.handle(route, handler);
            return true
        }
        false
    }

    fn hole<H>(&mut self, handler: H) 
    where
        H: Fn(&mut TcpStream)
    {
        print_request(self);

        handler(self);

        self.flush().unwrap();
    }

    fn handle<H>(&mut self, route: &str, handler: H) 
    where
        H: Fn(&mut TcpStream)
    {   
        let path = self.read_line(1);
        let route_string = format!(" {} ", route);
        if path.contains(&route_string) {
            print_request(self);

            handler(self);

            self.flush().unwrap();
        }
    }
}
