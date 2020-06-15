use std::fmt;
use super::Method;
use crate::Protocol;


#[derive(Debug, Clone)]
pub struct Route {
    method: Method,
    uri: String,
    protocol: Protocol,
}

impl Route {
    pub fn matches(&self, uri: &str) -> bool {
        self.uri == uri
    }

    pub fn is_method(&self, method: Method) -> bool {
        self.method == method
    }
}

impl From<String> for Route {
    fn from(raw: String) -> Route {
        let parsed_route: Vec<&str> = raw.trim().split(' ').collect();
        if parsed_route.len() < 3 {
            panic!("Misformed route: {:?}", parsed_route);
        }
        Route {
            method: Method::new(parsed_route[0]),
            uri: parsed_route[1].to_string(),
            protocol: Protocol::new(parsed_route[2]),
        }
    }
}

impl fmt::Display for Route {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.method, self.uri, self.protocol)
    }
}
