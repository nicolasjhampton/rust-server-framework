use std::net::TcpStream;
use crate::network::stream::TcpStreamExt;

pub struct Router {
    stream: TcpStream,
    route: String
}

impl Router {
    pub fn new(stream: TcpStream) -> Router {
        let path = stream.read_line(1);
        Router {
            stream: stream,
            route: path
        }
    }

    pub fn get<H>(&mut self, route: &str, handler: H)
    where
        H: Fn(&mut TcpStream)
    {
        if self.route.starts_with("GET") {
            self.handle(route, handler);
        }
    }

    pub fn hole<H>(&mut self, handler: H) 
    where
        H: Fn(&mut TcpStream)
    {
        self.stream.print_request();

        handler(&mut self.stream);
    }

    pub fn handle<H>(&mut self, route: &str, handler: H) 
    where
        H: Fn(&mut TcpStream)
    {   
        let route_string = format!(" {} ", route);
        if self.route.contains(&route_string) {
            self.stream.print_request();

            handler(&mut self.stream);
        }
    }
}