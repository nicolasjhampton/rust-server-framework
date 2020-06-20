pub mod route;
pub use route::Route;

pub mod method;
pub use method::Method;

pub mod uri_mod;
pub use uri_mod::{
    URI, 
    Path, 
    Query
};


#[cfg(test)]
mod tests {
    use super::*;
    use crate::Protocol;

    #[test]
    fn a_route_can_be_created_from_a_string() {
        let route = Route::from("POST /log?format=json HTTP/2.0");
        println!("{:?}", route);
        assert_eq!(format!("{}", route), "POST /log?format=json HTTP/2.0");
    }

    #[test]
    fn a_route_can_be_printed() {
        let mut route = Route::from("");
        route.set_uri(URI::from("/log?format=json"));
        route.set_protocol(Protocol::HTTP1_1);
        route.set_method(Method::POST);
        assert_eq!(format!("{}", route), "POST /log?format=json HTTP/1.1");
    }

    #[test]
    fn a_method_can_be_created_from_a_string() {
        let method = Method::from("GET");
        assert_eq!(format!("{}", method), "GET");
    }

    #[test]
    fn a_method_can_be_printed() {
        assert_eq!(format!("{}", Method::POST), "POST");
    }
}
