pub mod request;
pub use request::Request;
pub mod response;
pub use response::Response;
pub mod headers;
pub use headers::Headers;
use response::Status;
use request::Method;
use request::Route;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trailing_whitespace_not_included_in_headers() -> Result<(), std::io::Error> {
        let mut vec: &[u8] = b"GET / HTTP\r\nheader: my header\r\n";
        let request = Request::new(Box::new(vec));
        for (header, value) in request.headers.iter() {
            assert!(!header.contains("\r\n") && !value.contains("\r\n"));
        }
        Ok(())
    }

    #[test]
    fn header_can_be_retrieved_by_name() {
        let mut vec: &[u8] = b"GET / HTTP\r\nauth: noyabusiness\r\n";
        let request = Request::new(Box::new(vec));
        match request.headers.get("auth") {
            Some(value) => assert_eq!(value, "noyabusiness"),
            _ => panic!("auth header key not found!")
        };
    }

}