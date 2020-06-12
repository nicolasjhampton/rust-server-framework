use std::net::TcpStream;
use std::io::{BufReader, BufRead, BufWriter};
use crate::network::stream::TcpWriter;

pub struct Request {
    pub reader: BufReader<TcpStream>,
    pub headers: Vec<String>,
    pub route: String
}

impl Request {
    pub fn new(stream: TcpStream) -> Request {
        let mut reader = BufReader::new(stream);
        let route = Router::get_route(&mut reader);
        let headers = Router::get_headers(&mut reader);
        Request {
            reader,
            route,
            headers,
        }
    }
}

pub struct Response {
    pub writer: BufWriter<TcpStream>
}

impl Response {
    pub fn new(stream: TcpStream) -> Response {
        let mut writer = BufWriter::new(stream);
        Response {
            writer
        }
    }
}

pub struct Router {
    request: Request,
    response: Response
}

impl Router {
    pub fn new(stream: TcpStream) -> Router {
        let stream_copy = stream.try_clone().unwrap();
        let request = Request::new(stream);
        let response = Response::new(stream_copy);
        Router {
            request,
            response
        }
    }
    
    fn get_route(reader: &mut BufReader<TcpStream>) -> String {
        let mut route = String::new();
        reader.read_line(&mut route).unwrap();
        route
    }

    fn get_headers(reader: &mut BufReader<TcpStream>) -> Vec<String> {
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

    pub fn get<H>(&mut self, route: &str, handler: H)
    where
        H: Fn(&mut Request, &mut Response)
    {
        if self.request.route.starts_with("GET") {
            self.handle(route, handler);
        }
    }

    pub fn hole<H>(&mut self, handler: H) 
    where
        H: Fn(&mut Request, &mut Response)
    {
        handler(&mut self.request, &mut self.response);
    }

    pub fn handle<H>(&mut self, route: &str, handler: H) 
    where
        H: Fn(&mut Request, &mut Response)
    {   
        let route_string = format!(" {} ", route);
        if self.request.route.contains(&route_string) {
            handler(&mut self.request, &mut self.response);
        }
    }
}