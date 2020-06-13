mod http;
use std::net::TcpStream;
use http::request::Request;
use http::response::Response;

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
