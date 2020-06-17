pub mod headers;
pub use headers::Headers;

pub mod protocol;
pub use protocol::Protocol;

pub mod request_mod;
pub use request_mod::{
    Request,
    route_mod::{
        Route, 
        Method,
        uri_mod::{
            URI,
            Path,
            Query
        }
    }
};

pub mod response_mod;
pub use response_mod::{
    Response,
    Status
};


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = r#""#)]
    fn an_invalid_protocol_panics() {
        let _request = Protocol::from("HTTPWHATWHAT");
    }

    #[test]
    fn trailing_whitespace_not_included_in_headers() -> Result<(), std::io::Error> {
        let vec: &[u8] = b"GET / HTTP/1.1\r\nheader: my header\r\n";
        let request = Request::new(Box::new(vec));
        for (header, value) in request.headers.iter() {
            assert!(!header.contains("\r\n") && !value.contains("\r\n"));
        }
        Ok(())
    }

    #[test]
    fn header_can_be_retrieved_by_name() {
        let vec: &[u8] = b"GET / HTTP/1.1\r\nauth: noyabusiness\r\n";
        let request = Request::new(Box::new(vec));
        match request.headers.get("auth") {
            Some(value) => assert_eq!(value, "noyabusiness"),
            _ => panic!("auth header key not found!")
        };
    }

}
