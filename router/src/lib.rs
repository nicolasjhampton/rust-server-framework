use http::{Request, Response, Method};
use std::net::TcpStream;


pub struct Router {
    request: Request<TcpStream>,
    response: Response<TcpStream>
}

impl Router {
    pub fn new(stream: TcpStream) -> Router {
        let stream_copy = stream.try_clone().unwrap();
        let request = Request::new(Box::new(stream));
        let response = Response::new(Box::new(stream_copy));
        Router {
            request,
            response
        }
    }

    pub fn get<H>(&mut self, route: &str, handler: H)
    where
        H: Fn(&mut Request<TcpStream>, &mut Response<TcpStream>)
    {
        if self.request.route.is_method(Method::GET) {
            self.handle(route, handler);
        }
    }

    pub fn hole<H>(&mut self, handler: H) 
    where
        H: Fn(&mut Request<TcpStream>, &mut Response<TcpStream>)
    {
        handler(&mut self.request, &mut self.response);
    }

    pub fn handle<H>(&mut self, route: &str, handler: H) 
    where
        H: Fn(&mut Request<TcpStream>, &mut Response<TcpStream>)
    {   
        if self.request.route.matches(route) {
            handler(&mut self.request, &mut self.response);
        }
    }
}
