pub mod method;
pub mod route;
pub mod request;
pub mod uri;
pub use uri::URI;
pub mod query;
pub use query::Query;
pub mod path;
pub use path::Path;
pub use method::Method;
pub use route::Route;
pub use request::Request;


#[cfg(test)]
mod tests {
    use super::*;
    use std::io::prelude::*;
    use std::net::{TcpStream, TcpListener};

    #[test]
    fn a_path_can_be_printed() {
        let mut path = Path::from(String::new());
        path.push("this".to_string());
        path.push("is".to_string());
        path.push("the".to_string());
        path.push("way".to_string());
        assert_eq!(format!("{}", path), "/this/is/the/way")
    }

    #[test]
    fn a_path_can_be_created_from_a_string() {
        let mut path = Path::from(String::from("/this/is/the/way"));
        assert_eq!(format!("{}", path), "/this/is/the/way")
    }

    #[test]
    fn a_query_can_be_printed() {
        let mut query = Query::from(String::new());
        query.insert("query".to_string(), "what".into());
        assert_eq!(format!("{}", query), "?query=what")
    }

    #[test]
    fn a_query_can_be_made_from_a_string() {
        let mut query_string = String::from("query=what&if=then");
        let query = Query::from(query_string);
        assert_eq!(*query.get("query").unwrap(), String::from("what"));
        assert_eq!(*query.get("if").unwrap(), String::from("then"));
    }

    #[test]
    fn a_uri_can_be_printed() {
        let mut uri_string = String::from("scheme://authority/path0/path1/path2?query=what#fragment");
        let uri = URI::from(uri_string);
        assert_eq!(format!("{}", uri), "scheme://authority/path0/path1/path2?query=what#fragment")
    }

    #[test]
    fn a_uri_can_be_made_from_a_string() {
        let mut uri_string = String::from("scheme://authority/path0/path1/path2?query=what#fragment");
        let uri = URI::from(uri_string);
        assert_eq!(uri.scheme(), "scheme");
        assert_eq!(uri.authority(), "authority");
        for (num, path) in uri.path.iter().enumerate() {
            assert_eq!(path, &(format!("path{}", num)));
        }
        assert_eq!(uri.query.get("query").unwrap(), "what");
        assert_eq!(uri.fragment(), "fragment");
    }

    #[test]
    fn a_request_can_be_made_with_any_reader() -> Result<(), std::io::Error> {
        let mut vec: &[u8] = b"GET / HTTP/1.1\r\n\r\n";
        let request = Request::new(Box::new(vec));
        Ok(())
    }

    #[test]
    #[should_panic(expected = r#"Invalid method used in request: JIVE"#)]
    fn a_request_panics_when_invalid_method_used() {
        let mut route_method_invalid: &[u8] = b"JIVE / HTTP/1.1\r\n\r\n";
        let request = Request::new(Box::new(route_method_invalid));
    }

    #[test]
    #[should_panic(expected = r#"Misformed route: ["GET", "HTTP/1.1"]"#)]
    fn a_request_panics_when_route_is_misformed() {
        let mut route_without_uri: &[u8] = b"GET HTTP/1.1\r\n\r\n";
        let request = Request::new(Box::new(route_without_uri));
    }

    #[test]
    fn trailing_whitespace_not_included_in_route() -> Result<(), std::io::Error> {
        let mut vec: &[u8] = b"GET / HTTP/1.1\r\nheader: my header\r\n";
        let request = Request::new(Box::new(vec));
        assert!(!request.route.to_string().contains("\r\n"));
        Ok(())
    }


    #[test]
    fn body_can_be_retrieved_from_request() {
        let mut vec: &[u8] = b"GET / HTTP/1.1\r\nauth: noyabusiness\r\n\r\nThis is the body\nandI don't have anything more\r\n";
        let mut request = Request::new(Box::new(vec));
        let body = request.body();
        assert_eq!(body, "This is the body\nandI don\'t have anything more\r\n");
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