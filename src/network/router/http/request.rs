use std::net::TcpStream;
use std::io::{BufReader, BufRead};


pub struct Request {
    pub reader: BufReader<TcpStream>,
    pub headers: Vec<String>,
    pub route: String
}

impl Request {
    pub fn new(stream: TcpStream) -> Request {
        let mut reader = BufReader::new(stream);
        let route = Request::read_route(&mut reader);
        let headers = Request::read_headers(&mut reader);
        Request {
            reader,
            route,
            headers,
        }
    }

    fn read_route(reader: &mut BufReader<TcpStream>) -> String {
        let mut route = String::new();
        reader.read_line(&mut route).unwrap();
        route
    }

    fn read_headers(reader: &mut BufReader<TcpStream>) -> Vec<String> {
        let mut headers = vec![];
        loop {
            let mut header = String::new();
            reader.read_line(&mut header).unwrap();
            if header.as_str() != "\r\n" {
                headers.push(header);
            } else {
                break;
            }
        }
        headers
    }
}
