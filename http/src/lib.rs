pub mod request;
pub use request::Request;
pub mod response;
pub use response::Response;
mod headers;
mod route;
mod method;
pub use method::Method;


#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::any::type_name;
    use std::net::{TcpStream, TcpListener};
    use std::io::prelude::*;
    use std::io::{BufReader, BufRead, Read};

    #[test]
    fn a_request_can_be_made_with_any_reader() -> Result<(), std::io::Error> {
        let mut vec: &[u8] = b"GET / HTTP\r\n\r\n";
        let request = Request::new(Box::new(vec));
        Ok(())
    }

    #[test]
    #[should_panic(expected = r#"Invalid method used in request: JIVE"#)]
    fn a_request_panics_when_invalid_method_used() {
        let mut route_without_path: &[u8] = b"JIVE / HTTP\r\n\r\n";
        let request = Request::new(Box::new(route_without_path));
    }

    #[test]
    #[should_panic(expected = r#"Misformed route: ["GET", "HTTP"]"#)]
    fn a_request_panics_when_route_is_misformed() {
        let mut route_without_path: &[u8] = b"GET HTTP\r\n\r\n";
        let request = Request::new(Box::new(route_without_path));
    }

    #[test]
    fn trailing_whitespace_not_included_in_route() -> Result<(), std::io::Error> {
        let mut vec: &[u8] = b"GET / HTTP\r\nheader: my header\r\n";
        let request = Request::new(Box::new(vec));
        assert!(!request.route.protocol.contains("\r\n"));
        Ok(())
    }

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

    #[test]
    fn a_request_can_be_made_from_tcp_stream() -> Result<(), std::io::Error> {
        let listener = TcpListener::bind(&("127.0.0.1", 7879))?;
        let mut stream = TcpStream::connect(&("localhost", 7879)).unwrap();
        stream.write(b"GET / HTTP/1.1\r\n\r\n").unwrap();
        let mut instream = listener.accept()?.0;
        let request = Request::new(Box::new(instream));
        Ok(())
    }
}