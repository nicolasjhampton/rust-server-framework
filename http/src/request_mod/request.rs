use std::io::{BufReader, BufRead, Read};
use crate::headers::Headers;
use super::Route;
use std::fmt;


#[derive(Debug)]
pub struct Request<R> {
    pub reader: BufReader<Box<R>>,
    pub headers: Headers,
    pub route: Route,
    body: Option<String>
}

impl<R: Read> Request<R> {
    // change to from impl
    // new should be created from sum parts
    pub fn new(reader: Box<R>) -> Request<R> {
        let mut buf_reader = BufReader::new(reader);
        let route: Route = Route::from(Request::unwind(&mut buf_reader));
        let headers: Headers = Headers::from(Request::unwind_headers(&mut buf_reader));
        Request {
            reader: buf_reader,
            body: None,
            route,
            headers,
        }
    }

    fn get_body(&mut self) -> String {
        let mut body = String::new();
        self.reader.read_to_string(&mut body).unwrap();
        body
    }

    pub fn body(&mut self) -> String {
        if let None = self.body {
            self.body = Some(self.get_body());
        }
        self.body.as_ref().unwrap().to_string()
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

impl<R: Read> fmt::Display for Request<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{route}\n{headers}\r\n{body}",
            route = self.route,
            headers = self.headers,
            body = self.body.as_ref().unwrap()
        )
    }
}
