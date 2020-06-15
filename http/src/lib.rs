pub mod request;
pub use request::Request;
pub mod response;
pub use response::Response;
pub mod headers;
pub use headers::Headers;
pub mod protocol;
pub use protocol::Protocol;
use response::Status;
use request::Method;
use request::Route;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = r#""#)]
    fn an_invalid_protocol_panics() {
        let request = Protocol::new("HTTPWHATWHAT");
    }

    #[test]
    fn trailing_whitespace_not_included_in_headers() -> Result<(), std::io::Error> {
        let mut vec: &[u8] = b"GET / HTTP/1.1\r\nheader: my header\r\n";
        let request = Request::new(Box::new(vec));
        for (header, value) in request.headers.iter() {
            assert!(!header.contains("\r\n") && !value.contains("\r\n"));
        }
        Ok(())
    }

    #[test]
    fn header_can_be_retrieved_by_name() {
        let mut vec: &[u8] = b"GET / HTTP/1.1\r\nauth: noyabusiness\r\n";
        let request = Request::new(Box::new(vec));
        match request.headers.get("auth") {
            Some(value) => assert_eq!(value, "noyabusiness"),
            _ => panic!("auth header key not found!")
        };
    }

}