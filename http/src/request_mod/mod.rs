pub mod request;
pub use request::Request;

pub mod route_mod;
pub use route_mod::{
    Route, 
    Method,
    uri_mod::{
        URI,
        Path,
        Query
    }
};


#[cfg(test)]
mod tests {
    use super::*;
    use std::io::prelude::*;
    use std::net::{TcpStream, TcpListener};

    #[test]
    fn a_request_can_be_made_with_any_reader() -> Result<(), std::io::Error> {
        let vec: &[u8] = b"GET / HTTP/1.1\r\n\r\n";
        let _request = Request::new(Box::new(vec));
        Ok(())
    }

    #[test]
    #[should_panic(expected = r#"Invalid method used in request: JIVE"#)]
    fn a_request_panics_when_invalid_method_used() {
        let route_method_invalid: &[u8] = b"JIVE / HTTP/1.1\r\n\r\n";
        let _request = Request::new(Box::new(route_method_invalid));
    }

    #[test]
    #[should_panic(expected = r#"Misformed route: ["GET", "HTTP/1.1"]"#)]
    fn a_request_panics_when_route_is_misformed() {
        let route_without_uri: &[u8] = b"GET HTTP/1.1\r\n\r\n";
        let _request = Request::new(Box::new(route_without_uri));
    }

    #[test]
    fn trailing_whitespace_not_included_in_route() -> Result<(), std::io::Error> {
        let vec: &[u8] = b"GET / HTTP/1.1\r\nheader: my header\r\n";
        let request = Request::new(Box::new(vec));
        assert!(!request.route.to_string().contains("\r\n"));
        Ok(())
    }

    #[test]
    fn body_can_be_retrieved_from_request() {
        let vec: &[u8] = b"GET / HTTP/1.1\r\nauth: noyabusiness\r\n\r\nThis is the body\nandI don't have anything more\r\n";
        let mut request = Request::new(Box::new(vec));
        let body = request.body();
        assert_eq!(body, "This is the body\nandI don\'t have anything more\r\n");
    }

    #[test]
    fn a_request_can_be_made_from_tcp_stream() -> Result<(), std::io::Error> {
        let listener = TcpListener::bind(&("127.0.0.1", 7879))?;
        let mut stream = TcpStream::connect(&("localhost", 7879)).unwrap();
        stream.write(b"GET / HTTP/1.1\r\n\r\n").unwrap();
        let instream = listener.accept()?.0;
        let _request = Request::new(Box::new(instream));
        Ok(())
    }
}
