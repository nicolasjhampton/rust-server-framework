use std::io::{BufReader, BufRead, Read};
use crate::headers::Headers;
use super::Route;


#[derive(Debug)]
pub struct Request<R> {
    pub reader: BufReader<Box<R>>,
    pub headers: Headers,
    pub route: Route
}

impl<R: Read> Request<R> {
    pub fn new(reader: Box<R>) -> Request<R> {
        let mut buf_reader = BufReader::new(reader);
        let route: Route = Route::from(Request::unwind_route(&mut buf_reader));
        let headers: Headers = Headers::from(Request::unwind_headers(&mut buf_reader));
        Request {
            reader: buf_reader,
            route,
            headers,
        }
    }

    pub fn body(&mut self) -> String {
        let mut body = String::new();
        self.reader.read_to_string(&mut body).unwrap();
        body
    }

    fn unwind_route(reader: &mut BufReader<Box<R>>) -> String {
        let mut route = String::new();
        reader.read_line(&mut route).unwrap();
        route
    }

    fn unwind_headers(reader: &mut BufReader<Box<R>>) -> Vec<String> {
        let mut raw_headers = vec![];
        loop {
            let mut header = String::new();
            let length = reader.read_line(&mut header).unwrap();
            if length != 0 && header.as_str() != "\r\n" {
                raw_headers.push(header);
            } else {
                break;
            }
        }
        raw_headers
    }
}
