use std::fmt;
use super::Method;
use super::URI;
use crate::Protocol;


#[derive(Debug)]
pub struct Route {
    method: Method,
    uri: URI,
    protocol: Protocol,
}

impl Route {
    pub fn set_protocol(&mut self, protocol: Protocol) {
        self.protocol = protocol;
    }

    pub fn set_uri(&mut self, uri: URI) {
        self.uri = uri;
    }

    pub fn set_method(&mut self, method: Method) {
        self.method = method;
    }

    pub fn matches(&self, uri: &str) -> bool {
        format!("{}", self.uri) == uri
    }

    pub fn is_method(&self, method: Method) -> bool {
        self.method == method
    }
}

impl From<String> for Route {
    fn from(raw: String) -> Route {
        if raw.is_empty() {
            return Route {
                method: Method::GET,
                uri: URI::from(""),
                protocol: Protocol::HTTP1_1,
            }
        };
        let parsed_route: Vec<&str> = raw.trim().split(' ').collect();
        if parsed_route.len() < 3 {
            panic!("Misformed route: {:?}", parsed_route);
        }
        Route {
            method: Method::from(parsed_route[0]),
            uri: URI::from(parsed_route[1]),
            protocol: Protocol::from(parsed_route[2]),
        }
    }
}

impl From<&str> for Route {
    fn from(raw: &str) -> Route {
        let raw = raw.to_string();
        if raw.is_empty() { 
            return Route {
                method: Method::GET,
                uri: URI::from(""),
                protocol: Protocol::HTTP1_1,
            }
        };
        let parsed_route: Vec<&str> = raw.trim().split(' ').collect();
        if parsed_route.len() < 3 {
            panic!("Misformed route: {:?}", parsed_route);
        }
        Route {
            method: Method::from(parsed_route[0]),
            uri: URI::from(parsed_route[1]),
            protocol: Protocol::from(parsed_route[2]),
        }
    }
}

impl fmt::Display for Route {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        println!("route {:?}", self.protocol);
        println!("route {}", self.protocol);
        write!(f, "{} {} {}", self.method, self.uri, self.protocol)
    }
}
