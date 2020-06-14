use std::fmt;
use std::ops::Deref;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Method {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE
}

impl Method {
    pub fn new(method: &str) -> Method {
        match method {
            "GET" => Method::GET,
            "POST" => Method::POST,
            "PUT" => Method::PUT,
            "PATCH" => Method::PATCH,
            "DELETE" => Method::DELETE,
            _ => panic!("Invalid method used in request: {}", method)
        }
    }
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let method = match self {
            Method::GET => "GET",
            Method::POST => "POST",
            Method::PUT => "PUT",
            Method::PATCH => "PATCH",
            Method::DELETE => "DELETE",
        };
        write!(f, "{}", method)
    }
}
