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
        let route: Route = Route::from(Request::unwind(&mut buf_reader));
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

    fn unwind(reader: &mut BufReader<Box<R>>) -> String {
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        line
    }

    fn unwind_headers(mut reader: &mut BufReader<Box<R>>) -> String {
        let mut raw_headers = String::new();
        loop {
            let line = Request::unwind(&mut reader);
            if line.is_empty() || line == "\r\n" {
                break;
            } else {
                raw_headers.push_str(format!("{}\n", line).as_str());
            }
        }
        raw_headers
    }
}
