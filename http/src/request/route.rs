use super::Method;

#[derive(Debug)]
pub struct Route {
    method: Method,
    path: String,
    pub protocol: String,
}

impl Route {
    pub fn matches(&self, path: &str) -> bool {
        let let_through = self.path == path;
        println!("let_through: {} starts with {} -> {:?}", self.path, path, let_through);
        let_through
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
            path: parsed_route[1].to_string(),
            protocol: parsed_route[2].to_string(),
        }
    }
}
